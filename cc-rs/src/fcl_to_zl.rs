use crate::buttons::{click_events_to_interface, fcl_button_to_zl, fcl_button_to_zl_textbox};
use crate::context::ConversionContext;
use crate::direction::direction_to_zl_joystick;
use crate::events::{fcl_button_has_payload, strconv_quote};
use crate::geometry::{
    infer_builtin_menu_events, infer_events_from_group_names, infer_reciprocal_layer_openers,
    inferable_grid_indices, match_fcl_overlay_buttons,
};
use crate::styles::{
    default_fcl_direction_style, default_fcl_style, direction_style_map, fcl_button_style_to_zl_joystick,
    fcl_rocker_style_to_zl_joystick, fcl_styles_to_zl, resolve_direction_style,
};
use crate::utils::*;
use serde_json::{json, Map, Value};

pub fn zl_button_area_ratio(button: &Value, aspect: f64) -> f64 {
    let pos = button
        .get("position")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let button_size = button
        .get("buttonSize")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let temp_button = json!({
        "baseInfo": {
            "xPosition": gof_num(clamp_int(&get_or(&pos, "x", inum(0)), 0) as f64 / 10.0),
            "yPosition": gof_num(clamp_int(&get_or(&pos, "y", inum(0)), 0) as f64 / 10.0),
            "sizeType": "PERCENTAGE",
            "percentageWidth": {
                "reference": "SCREEN_WIDTH",
                "size": gof_num(clamp_int(&get_or(&button_size, "widthPercentage", inum(0)), 0) as f64 / 10.0),
            },
            "percentageHeight": {
                "reference": "SCREEN_WIDTH",
                "size": gof_num(clamp_int(&get_or(&button_size, "heightPercentage", inum(0)), 0) as f64 / 10.0),
            },
        },
    });
    let debug = std::env::var_os("CC_DEBUG_RATIO").is_some();
    if debug {
        let base = get_or(&temp_button, "baseInfo", obj());
        let r = crate::geometry::fcl_button_rect(&temp_button, aspect);
        eprintln!(
            "RECT xPosition={} yPosition={} sizeW={} x={} y={} w={} h={} x2x1={:.20e} y2y1={:.20e}",
            to_string_v(&get_or(&base, "xPosition", Value::Null)),
            to_string_v(&get_or(&base, "yPosition", Value::Null)),
            to_string_v(&get_or(&get_or(&base, "percentageWidth", obj()), "size", Value::Null)),
            r.x1,
            r.y1,
            r.x2 - r.x1,
            r.y2 - r.y1,
            r.x2 - r.x1,
            r.y2 - r.y1,
        );
    }
    crate::geometry::fcl_button_area_ratio(&temp_button, aspect)
}

pub fn fcl_to_zl(
    ctx: &mut ConversionContext,
    data: &Value,
    include_directions: bool,
    strict: bool,
    aspect: f64,
    lossless: bool,
    absolute_as_percentage: bool,
) -> Value {
    let include_directions = include_directions || lossless;
    let root_original = meta_original(data, "zl", Some("layout"));

    let default_styles;
    let styles_list: &[Value] = {
        let bl = get_or_list(data, "buttonStyles");
        if !bl.is_empty() {
            bl
        } else {
            default_styles = vec![default_fcl_style(None)];
            &default_styles
        }
    };
    let (styles, style_map) = fcl_styles_to_zl(styles_list);

    let default_dir_styles;
    let dir_styles_input: &[Value] = {
        let ds = get_or_list(data, "directionStyles");
        if !ds.is_empty() {
            ds
        } else {
            default_dir_styles = vec![default_fcl_direction_style()];
            &default_dir_styles
        }
    };
    let direction_styles = direction_style_map(dir_styles_input);

    let mut joystick_styles: Vec<Value> = Vec::new();
    if let Some(ro) = &root_original {
        if let Some(jsl) = ro.get("joystickStyles").and_then(|v| v.as_array()) {
            joystick_styles = jsl.clone();
        }
    }
    let mut joystick_style_uuids: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for item in &joystick_styles {
        if item.is_object() {
            let name = to_string_v(&get_or(item, "name", json!("")));
            if !name.is_empty() && !joystick_style_uuids.contains_key(&name) {
                joystick_style_uuids.insert(name, to_string_v(&get_or(item, "uuid", json!(""))));
            }
        }
    }
    let mut warned_joystick_settings = false;

    let mut group_ids_by_name = GroupIdsByName::new();
    for item in get_or_list(data, "viewGroups") {
        if item.is_object() {
            let id = to_string_v(&get_or(item, "id", json!("")));
            if !id.is_empty() {
                let name = to_string_v(&get_or(item, "name", json!("Layer")));
                group_ids_by_name.insert(name, id);
            }
        }
    }

    let reciprocal_openers = infer_reciprocal_layer_openers(data, aspect);

    let mut layers: Vec<Value> = Vec::new();
    let view_groups = get_or_list(data, "viewGroups");
    for i in (0..view_groups.len()).rev() {
        let group = &view_groups[i];
        if !group.is_object() {
            continue;
        }
        let layer_original = meta_original(group, "zl", Some("layer"));
        let view_data = group
            .get("viewData")
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        let mut group_name = to_string_v(&get_or(group, "name", json!("Layer")));
        if group_name.is_empty() {
            group_name = "Layer".to_string();
        }

        let mut buttons: Vec<Value> = Vec::new();
        let mut text_boxes: Vec<Value> = Vec::new();
        let mut fcl_buttons: Vec<&Value> = Vec::new();
        if let Some(items) = view_data.get("buttonList").and_then(|v| v.as_array()) {
            for item in items {
                if item.is_object() {
                    fcl_buttons.push(item);
                }
            }
        }

        let (overlay_matches, consumed_display_indices) =
            match_fcl_overlay_buttons(&fcl_buttons, aspect);
        let grid_indices = inferable_grid_indices(&fcl_buttons);

        for (index, button) in fcl_buttons.iter().enumerate() {
            if consumed_display_indices.contains(&index) {
                continue;
            }
            let has_payload = fcl_button_has_payload(button);
            if has_payload {
                let visual_button = overlay_matches.get(&index).map(|&mi| fcl_buttons[mi]);
                let converted_button = fcl_button_to_zl(
                    ctx,
                    button,
                    &style_map,
                    strict,
                    &group_name,
                    &group_ids_by_name,
                    visual_button,
                    absolute_as_percentage,
                    aspect,
                );
                if !get_or_list(&converted_button, "clickEvents").is_empty() {
                    buttons.push(converted_button);
                } else {
                    let vb = visual_button.unwrap_or(button);
                    text_boxes.push(fcl_button_to_zl_textbox(
                        vb,
                        &style_map,
                        absolute_as_percentage,
                        aspect,
                    ));
                }
            } else {
                let button_id = to_string_v(&get_or(button, "id", json!("")));
                let opener_target = reciprocal_openers.get(&button_id).cloned().unwrap_or_default();
                let mut inferred_events: Vec<Value> = Vec::new();
                if !opener_target.is_empty() {
                    inferred_events = vec![json!({
                        "type": "switch_layer",
                        "key": opener_target,
                    })];
                }
                if inferred_events.is_empty() && grid_indices.contains(&index) {
                    inferred_events =
                        infer_events_from_group_names(button, &group_ids_by_name, &group_name);
                }
                if inferred_events.is_empty() && grid_indices.contains(&index) {
                    inferred_events = infer_builtin_menu_events(button);
                }
                if !inferred_events.is_empty() {
                    let mut inferred_button = fcl_button_to_zl(
                        ctx,
                        button,
                        &style_map,
                        strict,
                        &group_name,
                        &group_ids_by_name,
                        None,
                        absolute_as_percentage,
                        aspect,
                    );
                    if let Value::Object(m) = &mut inferred_button {
                        m.insert("clickEvents".to_string(), click_events_to_interface(inferred_events));
                        m.insert("isSwipple".to_string(), Value::Bool(false));
                        m.insert("isPenetrable".to_string(), Value::Bool(false));
                    }
                    buttons.push(inferred_button);
                } else {
                    buttons.push(fcl_button_to_zl(
                        ctx,
                        button,
                        &style_map,
                        strict,
                        &group_name,
                        &group_ids_by_name,
                        None,
                        absolute_as_percentage,
                        aspect,
                    ));
                }
            }
        }

        let directions = get_or_list(&view_data, "directionList");
        let mut joystick_buttons: Vec<Value> = Vec::new();
        if !directions.is_empty() && !include_directions {
            let message = format!(
                "skipped {} FCL direction control(s) in group {}; use --include-directions to convert them",
                directions.len(),
                strconv_quote(&to_string_v(&get_or(group, "name", json!("")))),
            );
            ctx.warn(&message, strict, false);
        }
        if include_directions {
            for dir_item in directions {
                if !dir_item.is_object() {
                    continue;
                }
                let direction = dir_item;
                let direction_style = resolve_direction_style(direction, &direction_styles);
                let is_rocker =
                    to_string_v(&get_or(&direction_style, "styleType", json!(""))) == "ROCKER";
                let style_name = to_string_v(&get_or(&direction_style, "name", json!("Default")));
                let mut style_uuid = joystick_style_uuids
                    .get(&style_name)
                    .cloned()
                    .unwrap_or_default();
                if style_uuid.is_empty() {
                    let mut joystick_style = if is_rocker {
                        fcl_rocker_style_to_zl_joystick(Some(&direction_style))
                    } else {
                        fcl_button_style_to_zl_joystick(Some(&direction_style))
                    };
                    style_uuid = to_string_v(&get_or(&joystick_style, "uuid", json!("")));
                    joystick_style_uuids.insert(style_name.clone(), style_uuid.clone());
                    let meta = make_meta("fcl", "directionStyle", &style_name, &direction_style, None);
                    joystick_style = set_meta(joystick_style, Some(&meta));
                    joystick_styles.push(joystick_style);
                }
                if !warned_joystick_settings {
                    ctx.warn(
                        "converted FCL direction controls (ROCKER and BUTTON styles) to ZL joystickButtons and joystickStyles (ZL editor v12)",
                        strict,
                        false,
                    );
                    warned_joystick_settings = true;
                }
                joystick_buttons.push(direction_to_zl_joystick(
                    ctx,
                    direction,
                    &direction_style,
                    &style_uuid,
                    strict,
                    aspect,
                ));
                ctx.bump("directions");
            }
        }

        buttons.sort_by(|a, b| {
            zl_button_area_ratio(b, aspect)
                .partial_cmp(&zl_button_area_ratio(a, aspect))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut layer_obj = match &layer_original {
            Some(lo) if lo.is_object() => lo.clone(),
            _ => obj(),
        };
        let hide_when_mouse = to_bool(&get_or(&layer_obj, "hideWhenMouse", json!(false)));
        let hide_when_gamepad = to_bool(&get_or(&layer_obj, "hideWhenGamepad", json!(false)));
        let hide_when_joystick = to_bool(&get_or(&layer_obj, "hideWhenJoystick", json!(false)));
        let visibility_type = to_string_v(&get_or(&layer_obj, "visibilityType", json!("always")));
        let layer_uuid_default = layer_obj.get("uuid").cloned().unwrap_or_else(|| json!(short_id()));
        if let Value::Object(m) = &mut layer_obj {
            m.insert("name".to_string(), json!(group_name));
            m.insert(
                "uuid".to_string(),
                json!(to_string_v(&get_or(group, "id", layer_uuid_default))),
            );
            m.insert(
                "hide".to_string(),
                Value::Bool(to_string_v(&get_or(group, "visibility", json!(""))) == "INVISIBLE"),
            );
            insert_if_absent(m, "hideWhenMouse", Value::Bool(hide_when_mouse));
            insert_if_absent(m, "hideWhenGamepad", Value::Bool(hide_when_gamepad));
            insert_if_absent(m, "hideWhenJoystick", Value::Bool(hide_when_joystick));
            insert_if_absent(m, "visibilityType", json!(visibility_type));
            if m.get("visibilityType").map(to_string_v).unwrap_or_default().is_empty() {
                m.insert("visibilityType".to_string(), json!("always"));
            }
            m.insert("normalButtons".to_string(), Value::Array(buttons));
            m.insert("textBoxes".to_string(), Value::Array(text_boxes));
            m.insert("joystickButtons".to_string(), Value::Array(joystick_buttons));
        }

        let origin_id = to_string_v(&get_or(
            group,
            "id",
            json!(to_string_v(&get_or(&layer_obj, "uuid", json!("")))),
        ));
        let meta = make_meta("fcl", "viewGroup", &origin_id, group, None);
        layer_obj = set_meta(layer_obj, Some(&meta));
        layers.push(layer_obj);
    }

    let mut layer_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    for layer in &layers {
        layer_ids.insert(to_string_v(&get_or(layer, "uuid", json!(""))));
    }
    for layer in &mut layers {
        if let Some(normal_buttons) = layer.get_mut("normalButtons").and_then(|v| v.as_array_mut()) {
            for item in normal_buttons.iter_mut() {
                if !item.is_object() {
                    continue;
                }
                let click_events: Vec<Value> = item
                    .get("clickEvents")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut filtered: Vec<Value> = Vec::new();
                for event_item in click_events {
                    if !event_item.is_object() {
                        filtered.push(event_item);
                        continue;
                    }
                    let event_type = to_string_v(&get_or(&event_item, "type", json!("")));
                    let event_key = to_string_v(&get_or(&event_item, "key", json!("")));
                    if event_type == "switch_layer" || event_type == "show_layer" || event_type == "hide_layer" {
                        if !layer_ids.contains(&event_key) {
                            continue;
                        }
                    }
                    filtered.push(event_item);
                }
                let empty = filtered.is_empty();
                if let Value::Object(m) = item {
                    m.insert("clickEvents".to_string(), Value::Array(filtered));
                    if empty {
                        m.insert("isSwipple".to_string(), Value::Bool(true));
                        m.insert("isPenetrable".to_string(), Value::Bool(true));
                        m.insert("isToggleable".to_string(), Value::Bool(false));
                    }
                }
            }
        }
    }

    let mut result = match &root_original {
        Some(ro) if ro.is_object() => ro.clone(),
        _ => obj(),
    };

    let result_info = result
        .get("info")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let version_code_default = clamp_int(&get_or(&result_info, "versionCode", inum(1)), 1);
    let editor_version =
        clamp_int(&get_or(&result, "editorVersion", inum(crate::constants::ZL_EDITOR_VERSION)), crate::constants::ZL_EDITOR_VERSION);
    let info = json!({
        "name": translatable(&to_string_v(&get_or(data, "name", json!("Converted from FCL"))), result_info.get("name")),
        "author": translatable(&to_string_v(&get_or(data, "author", json!(""))), result_info.get("author")),
        "description": translatable(&to_string_v(&get_or(data, "description", json!(""))), result_info.get("description")),
        "versionCode": inum(0.max(clamp_int(&get_or(data, "versionCode", inum(version_code_default)), 0))),
        "versionName": to_string_v(&get_or(data, "version", get_or(&result_info, "versionName", json!("1.0")))),
    });
    if let Value::Object(m) = &mut result {
        m.insert("info".to_string(), info);
        m.insert("layers".to_string(), Value::Array(layers));

        let styles_value = match m.get("styles") {
            Some(sl) if sl.as_array().map_or(false, |a| !a.is_empty()) => sl.clone(),
            _ => Value::Array(styles.clone()),
        };
        m.insert("styles".to_string(), styles_value);
        m.insert("joystickStyles".to_string(), Value::Array(joystick_styles));
        m.insert("editorVersion".to_string(), inum(editor_version));
    }

    let mut result_id = to_string_v(&get_or(data, "id", json!("")));
    if result_id.is_empty() {
        result_id = to_string_v(&get_or(&get_or(&result, "info", obj()), "name", json!("")));
    }
    if result_id.is_empty() {
        result_id = short_id();
    }
    let meta = make_meta("fcl", "controller", &result_id, data, None);
    set_meta(result, Some(&meta))
}

fn insert_if_absent(m: &mut Map<String, Value>, key: &str, value: Value) {
    use serde_json::map::Entry;
    if let Entry::Vacant(e) = m.entry(key.to_string()) {
        e.insert(value);
    }
}

pub fn normalize_zl_layout(layout: Value) -> Value {
    let mut result = layout;
    if let Value::Object(m) = &mut result {
        insert_if_absent(m, "joystickStyles", Value::Array(Vec::new()));
    }
    if let Some(layers) = result.get_mut("layers").and_then(|v| v.as_array_mut()) {
        for layer in layers.iter_mut() {
            if !layer.is_object() {
                continue;
            }
            if let Value::Object(m) = layer {
                insert_if_absent(m, "hideWhenMouse", Value::Bool(true));
                insert_if_absent(m, "hideWhenGamepad", Value::Bool(true));
                insert_if_absent(m, "hideWhenJoystick", Value::Bool(false));
                insert_if_absent(m, "normalButtons", Value::Array(Vec::new()));
                insert_if_absent(m, "textBoxes", Value::Array(Vec::new()));
                insert_if_absent(m, "joystickButtons", Value::Array(Vec::new()));
            }
        }
    }
    result
}

pub fn convert_fcl_to_zl(
    ctx: &mut ConversionContext,
    data: &Value,
    include_directions: bool,
    strict: bool,
    aspect: f64,
    lossless: bool,
    absolute_as_percentage: bool,
) -> Value {
    normalize_zl_layout(fcl_to_zl(
        ctx,
        data,
        include_directions,
        strict,
        aspect,
        lossless,
        absolute_as_percentage,
    ))
}
