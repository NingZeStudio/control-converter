use crate::context::ConversionContext;
use crate::styles::{
    default_fcl_direction_style, default_zl_fallback_fcl_style, style_name_for_zl_style,
    zl_shape_to_fcl_radius,
};
use crate::utils::*;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

fn py_str(v: &Value) -> String {
    match v {
        Value::Null => "None".to_string(),
        _ => to_string_v(v),
    }
}

fn truthy_str(v: &Value) -> Option<String> {
    let s = to_string_v(v);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn zl_joystick_style_to_fcl_rocker(style: &Value) -> Value {
    let empty = obj();
    let light = style
        .get("lightStyle")
        .filter(|v| !v.is_null())
        .unwrap_or(&empty);
    let joystick_size = clamp_range(&get_or_empty(light, "joystickSize"), 0.0, 1.0, 0.5);
    json!({
        "rockerSize": inum(100.max(1000.min(py_round(joystick_size * 1000.0)))),
        "bgCornerRadius": inum(0.max(500.min(clamp_int(&get_or_empty(light, "backgroundShape"), 50) * 10))),
        "bgStrokeWidth": inum(0.max(500.min(clamp_int(&get_or_empty(light, "borderWidthRatio"), 0) * 10))),
        "bgStrokeColor": inum(zl_color_to_fcl(&get_or_empty(light, "borderColor"), -12303292, None)),
        "bgFillColor": inum(zl_color_to_fcl(&get_or_empty(light, "backgroundColor"), 0, Some(&get_or_empty(light, "alpha")))),
        "rockerCornerRadius": inum(0.max(500.min(clamp_int(&get_or_empty(light, "joystickShape"), 50) * 10))),
        "rockerStrokeWidth": 10,
        "rockerStrokeColor": inum(zl_color_to_fcl(&get_or_empty(light, "joystickColor"), -12303292, None)),
        "rockerFillColor": inum(zl_color_to_fcl(&get_or_empty(light, "joystickColor"), -7829368, Some(&get_or_empty(light, "alpha")))),
    })
}

pub fn zl_styles_to_fcl(styles: &[Value]) -> (Vec<Value>, HashMap<String, String>) {
    let mut result: Vec<Value> = Vec::new();
    let mut mapping: HashMap<String, String> = HashMap::new();
    let mut used: HashSet<String> = HashSet::new();

    for style in styles {
        let uuid_value = py_str(&get_or(style, "uuid", json!("")));
        let name_raw = to_string_v(&get_or(style, "name", json!("")));
        let base_name = if !name_raw.is_empty() {
            name_raw
        } else if !uuid_value.is_empty() {
            uuid_value.clone()
        } else {
            "Style".to_string()
        };
        let mut name = style_name_for_zl_style(&base_name, &uuid_value);
        let mut suffix = 2;
        while used.contains(&name) {
            name = format!("{}_{}", style_name_for_zl_style(&base_name, &uuid_value), suffix);
            suffix += 1;
        }
        used.insert(name.clone());
        if !uuid_value.is_empty() {
            mapping.insert(uuid_value.clone(), name.clone());
        }

        let light = style.get("lightStyle").cloned().unwrap_or_else(obj);
        result.push(json!({
            "name": name,
            "textColor": inum(zl_color_to_fcl(&get_or_empty(&light, "contentColor"), -1, None)),
            "textSize": inum(clamp_int(&get_or_empty(&light, "fontSize"), 12)),
            "strokeColor": inum(zl_color_to_fcl(&get_or_empty(&light, "borderColor"), -12303292, None)),
            "strokeWidth": inum(clamp_int(&get_or_empty(&light, "borderWidth"), 1) * 10),
            "cornerRadius": inum(zl_shape_to_fcl_radius(&get_or_empty(&light, "borderRadius"))),
            "fillColor": inum(zl_color_to_fcl(&get_or_empty(&light, "backgroundColor"), 0, Some(&get_or_empty(&light, "alpha")))),
            "textColorPressed": inum(zl_color_to_fcl(&get_or_empty(&light, "pressedContentColor"), -1, None)),
            "textSizePressed": inum(clamp_int(&get_or_empty(&light, "pressedFontSize"), clamp_int(&get_or_empty(&light, "fontSize"), 12))),
            "strokeColorPressed": inum(zl_color_to_fcl(&get_or_empty(&light, "pressedBorderColor"), -12303292, None)),
            "strokeWidthPressed": inum(clamp_int(&get_or_empty(&light, "pressedBorderWidth"), clamp_int(&get_or_empty(&light, "borderWidth"), 1)) * 10),
            "cornerRadiusPressed": inum(zl_shape_to_fcl_radius(&get_or_empty(&light, "pressedBorderRadius"))),
            "fillColorPressed": inum(zl_color_to_fcl(&get_or_empty(&light, "pressedBackgroundColor"), -3355444, Some(&get_or_empty(&light, "pressedAlpha")))),
        }));
    }

    if result.is_empty() {
        result.push(default_zl_fallback_fcl_style(None));
    } else {
        let has_native_default = result
            .iter()
            .any(|s| to_string_v(&get_or(s, "name", json!(""))) == "ZL Native Default");
        if !has_native_default {
            result.insert(0, default_zl_fallback_fcl_style(None));
        }
    }
    (result, mapping)
}

pub fn make_base_info_from_zl(
    ctx: &mut ConversionContext,
    button: &Value,
    layer_visibility: &str,
    strict: bool,
    label: &str,
    style_name: Option<&str>,
    fcl_styles: &[Value],
) -> Value {
    let empty_size = obj();
    let size = button.get("buttonSize").filter(|v| !v.is_null()).unwrap_or(&empty_size);
    let size_kind = size.get("type").map(to_string_v).unwrap_or_default();
    let size_type: &str;
    let absolute_width: i64;
    let absolute_height: i64;
    if size_kind == "absolute" || size_kind == "dp" {
        size_type = "ABSOLUTE";
        absolute_width = clamp_int(&get_or_empty(size, "widthDp"), 50);
        absolute_height = clamp_int(&get_or_empty(size, "heightDp"), 50);
    } else if size_kind == "wrap_content" {
        size_type = "ABSOLUTE";
        let (w, h) = estimate_wrap_content_dp(button, style_name.unwrap_or(""), &Value::Array(fcl_styles.to_vec()));
        absolute_width = w;
        absolute_height = h;
        let widget_label = {
            let l = label.to_string();
            if !l.is_empty() {
                l
            } else {
                let t = text_default(&get_or_empty(button, "text"));
                if !t.is_empty() {
                    t
                } else {
                    let u = to_string_v(&get_or(button, "uuid", json!("")));
                    if u.is_empty() {
                        "<unnamed>".to_string()
                    } else {
                        u
                    }
                }
            }
        };
        ctx.warn(
            &format!(
                "ZL wrap_content size on widget {:?} has no exact FCL equivalent; estimated dp size",
                widget_label
            ),
            strict,
            true,
        );
    } else {
        size_type = "PERCENTAGE";
        absolute_width = clamp_int(&get_or_empty(size, "widthDp"), 50);
        absolute_height = clamp_int(&get_or_empty(size, "heightDp"), 50);
    }
    let visibility_src = button
        .get("visibilityType")
        .map(to_string_v)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| layer_visibility.to_string());
    let empty_pos = obj();
    let position = button.get("position").filter(|v| !v.is_null()).unwrap_or(&empty_pos);
    json!({
        "visibilityType": visibility_zl_to_fcl(&visibility_src),
        "xPosition": inum(scale_position_to_fcl(&get_or(position, "x", inum(0)))),
        "yPosition": inum(scale_position_to_fcl(&get_or(position, "y", inum(0)))),
        "sizeType": size_type,
        "absoluteWidth": inum(absolute_width),
        "absoluteHeight": inum(absolute_height),
        "percentageWidth": {
            "reference": zl_ref_to_fcl(&to_string_v(&get_or(size, "widthReference", Value::Null))),
            "size": inum(scale_position_to_fcl(&get_or(size, "widthPercentage", inum(500)))),
        },
        "percentageHeight": {
            "reference": zl_ref_to_fcl(&to_string_v(&get_or(size, "heightReference", Value::Null))),
            "size": inum(scale_position_to_fcl(&get_or(size, "heightPercentage", inum(500)))),
        },
    })
}

fn warn_unmapped_layer_flags(ctx: &mut ConversionContext, layer: &Value, strict: bool) {
    let layer_name = {
        let n = to_string_v(&get_or(layer, "name", json!("")));
        if !n.is_empty() {
            n
        } else {
            let u = to_string_v(&get_or(layer, "uuid", json!("")));
            if u.is_empty() {
                "Layer".to_string()
            } else {
                u
            }
        }
    };
    if to_bool(&get_or(layer, "hideWhenMouse", json!(false))) {
        ctx.warn(
            &format!("ZL layer {:?} hideWhenMouse has no FCL equivalent; skipped", layer_name),
            strict,
            true,
        );
    }
    if to_bool(&get_or(layer, "hideWhenGamepad", json!(false))) {
        ctx.warn(
            &format!("ZL layer {:?} hideWhenGamepad has no FCL equivalent; skipped", layer_name),
            strict,
            true,
        );
    }
    if to_bool(&get_or(layer, "hideWhenJoystick", json!(false))) {
        ctx.warn(
            &format!("ZL layer {:?} hideWhenJoystick has no FCL equivalent; skipped", layer_name),
            strict,
            true,
        );
    }
}

fn event_keycode_list_mut<'a>(fcl_event: &'a mut Value, event_name: &str) -> &'a mut Vec<Value> {
    fcl_event
        .get_mut(event_name)
        .and_then(|e| e.get_mut("outputKeycodes"))
        .and_then(|k| k.as_array_mut())
        .expect("fcl event structure")
}

fn set_click_flag(fcl_event: &mut Value, flag: &str, value: Value) {
    if let Some(click) = fcl_event.get_mut("clickEvent").and_then(|c| c.as_object_mut()) {
        click.insert(flag.to_string(), value);
    }
}

fn push_keycode(fcl_event: &mut Value, event_name: &str, keycode: i64) {
    event_keycode_list_mut(fcl_event, event_name).push(inum(keycode));
}

fn apply_zl_event_to_fcl(
    ctx: &mut ConversionContext,
    event: &Value,
    fcl_event: &mut Value,
    strict: bool,
    mut substitutions: Option<&mut Vec<Value>>,
    _layer_id_map: &HashMap<String, String>,
) {
    let etype = event.get("type").map(to_string_v);
    let raw_key = py_str(&get_or(event, "key", json!("")));
    let key = normalize_zl_key(&raw_key);

    match etype.as_deref() {
        Some("key") => {
            let keycode = convert_key_to_fcl(ctx, &key, strict, substitutions.as_deref_mut());
            push_keycode(fcl_event, "pressEvent", keycode);
        }
        Some("launcher_event") => {
            if let Some(mouse) = crate::constants::FCL_MOUSE.get(key.as_str()) {
                push_keycode(fcl_event, "pressEvent", *mouse);
            } else if key == "launcher.event.switch_ime" {
                set_click_flag(fcl_event, "input", Value::Bool(true));
            } else if key == "launcher.event.switch_menu" {
                set_click_flag(fcl_event, "openMenu", Value::Bool(true));
            } else if key == "launcher.event.scroll_up.single" {
                push_keycode(fcl_event, "clickEvent", 1003);
            } else if key == "launcher.event.scroll_down.single" {
                push_keycode(fcl_event, "clickEvent", 1004);
            } else if key == "launcher.event.scroll_up" || key == "launcher.event.scroll_down" {
                let code = if key.ends_with("scroll_up") { 1003 } else { 1004 };
                if let Some(press) = fcl_event.get_mut("pressEvent").and_then(|p| p.as_object_mut()) {
                    press.insert("autoClick".to_string(), Value::Bool(true));
                }
                push_keycode(fcl_event, "pressEvent", code);
            } else {
                let keycode = convert_key_to_fcl(ctx, &key, strict, substitutions.as_deref_mut());
                push_keycode(fcl_event, "pressEvent", keycode);
            }
        }
        Some("switch_layer") | Some("show_layer") | Some("hide_layer") => {
            return;
        }
        Some("send_text") => {
            set_click_flag(fcl_event, "outputText", json!(raw_key));
        }
        None => {
            return;
        }
        Some(other) => {
            let reason = format!("unsupported ZL event type {:?}; substituted with no-op text event", other);
            ctx.warn(&reason, strict, false);
            if let Some(subs) = substitutions.as_deref_mut() {
                subs.push(substitution(
                    ctx,
                    &json!({ "type": other, "key": raw_key }),
                    &json!({ "type": "send_text", "key": "" }),
                    &reason,
                    "events",
                ));
            }
            let existing = fcl_event
                .get("clickEvent")
                .and_then(|c| c.get("outputText"))
                .cloned()
                .unwrap_or(Value::Null);
            let _ = existing;
        }
    }
}

fn apply_zl_layer_events_to_fcl(
    ctx: &mut ConversionContext,
    events: &[Value],
    fcl_event: &mut Value,
    _strict: bool,
    initial_layer_state: &HashMap<String, bool>,
    layer_id_map: &HashMap<String, String>,
    mut substitutions: Option<&mut Vec<Value>>,
) {
    let mut local_state: HashMap<String, bool> = initial_layer_state.clone();
    let mut toggles: Vec<String> = Vec::new();

    for event in events {
        let etype = event.get("type").map(to_string_v).unwrap_or_default();
        if etype != "switch_layer" && etype != "show_layer" && etype != "hide_layer" {
            continue;
        }
        let raw_key = py_str(&get_or(event, "key", json!("")));
        let target_id = layer_id_map
            .get(&raw_key)
            .cloned()
            .unwrap_or_else(|| raw_key.clone());
        if target_id.is_empty() {
            continue;
        }

        let current = local_state.get(&target_id).copied().unwrap_or(false);
        let mut should_toggle = false;
        if etype == "switch_layer" {
            should_toggle = true;
            local_state.insert(target_id.clone(), !current);
        } else if etype == "show_layer" {
            if !current {
                should_toggle = true;
                local_state.insert(target_id.clone(), true);
            }
        } else if etype == "hide_layer" {
            if current {
                should_toggle = true;
                local_state.insert(target_id.clone(), false);
            }
        }

        if should_toggle {
            toggles.push(target_id);
        } else if etype != "switch_layer" {
            if let Some(subs) = substitutions.as_deref_mut() {
                subs.push(substitution(
                    ctx,
                    &json!({ "type": etype, "key": raw_key }),
                    &json!({ "type": "no_op", "key": target_id }),
                    &format!(
                        "Layer already {} in the simulated ZL state; skipped FCL toggle",
                        if etype == "show_layer" { "visible" } else { "hidden" }
                    ),
                    "layers",
                ));
            }
        }
    }

    let mut counts: HashMap<String, usize> = HashMap::new();
    for t in &toggles {
        *counts.entry(t.clone()).or_insert(0) += 1;
    }
    let mut ordered_unique: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for target_id in &toggles {
        if seen.contains(target_id) || counts[target_id] % 2 == 0 {
            continue;
        }
        seen.insert(target_id.clone());
        ordered_unique.push(target_id.clone());
    }

    if let Some(bind) = fcl_event
        .get_mut("clickEvent")
        .and_then(|c| c.get_mut("bindViewGroup"))
        .and_then(|b| b.as_array_mut())
    {
        for id in &ordered_unique {
            bind.push(json!(id));
        }
    }

    if let Some(subs) = substitutions.as_deref_mut() {
        let has_state_events = events.iter().any(|e| {
            let t = e.get("type").map(to_string_v).unwrap_or_default();
            t == "show_layer" || t == "hide_layer"
        });
        if has_state_events {
            let state_events: Vec<Value> = events
                .iter()
                .filter(|e| {
                    let t = e.get("type").map(to_string_v).unwrap_or_default();
                    t == "switch_layer" || t == "show_layer" || t == "hide_layer"
                })
                .cloned()
                .collect();
            subs.push(substitution(
                ctx,
                &json!({ "type": "zl_layer_state_events", "events": state_events }),
                &json!({ "type": "fcl_bindViewGroup", "keys": ordered_unique }),
                "Converted ZL show/hide/switch layer events by simulating initial layer visibility and emitting only necessary FCL toggles",
                "layers",
            ));
        }
    }
}

fn resolve_zl_button_style_name(
    style_uuid: &Value,
    style_map: &HashMap<String, String>,
    fallback: &str,
) -> String {
    if style_uuid.is_null() {
        return fallback.to_string();
    }
    style_map
        .get(&py_str(style_uuid))
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

fn overlay_shared_fields_fcl(
    ctx: &mut ConversionContext,
    original: &Value,
    current: &Value,
    layer_visibility: &str,
    style_map: &HashMap<String, String>,
    strict: bool,
    fcl_styles: &[Value],
) -> Value {
    let mut restored = original.clone();
    let style_uuid = get_or_empty(current, "buttonStyle");
    let default_style = to_string_v(&get_or(&restored, "style", json!("ZL Native Default")));
    let fallback = if default_style.is_empty() {
        "ZL Native Default".to_string()
    } else {
        default_style
    };
    let style_name = resolve_zl_button_style_name(&style_uuid, style_map, &fallback);
    let restored_id = {
        let cu = to_string_v(&get_or(current, "uuid", Value::Null));
        if !cu.is_empty() {
            cu
        } else {
            let ri = to_string_v(&get_or(&restored, "id", Value::Null));
            if !ri.is_empty() {
                ri
            } else {
                fcl_id()
            }
        }
    };
    let text = text_default(&get_or_empty(current, "text"));
    let base_info = make_base_info_from_zl(
        ctx,
        current,
        layer_visibility,
        strict,
        &text,
        Some(&style_name),
        fcl_styles,
    );
    if let Value::Object(m) = &mut restored {
        m.insert("id".to_string(), json!(restored_id));
        m.insert("text".to_string(), json!(text));
        m.insert("style".to_string(), json!(style_name));
        m.insert("baseInfo".to_string(), base_info);
    }
    restored
}

fn zl_button_to_fcl(
    ctx: &mut ConversionContext,
    button: &Value,
    layer_visibility: &str,
    style_map: &HashMap<String, String>,
    strict: bool,
    layer_id_map: &HashMap<String, String>,
    fcl_styles: &[Value],
    initial_layer_state: &HashMap<String, bool>,
    current_layer_id: &str,
) -> Option<Value> {
    let original = meta_original(button, "fcl", Some("button"));
    if let Some(orig) = original {
        let restored = overlay_shared_fields_fcl(ctx, &orig, button, layer_visibility, style_map, strict, fcl_styles);
        let origin_id = {
            let u = to_string_v(&get_or(button, "uuid", Value::Null));
            if !u.is_empty() {
                u
            } else {
                let ri = to_string_v(&get_or(&restored, "id", Value::Null));
                if !ri.is_empty() {
                    ri
                } else {
                    fcl_id()
                }
            }
        };
        let meta = make_meta("zl", "button", &origin_id, button, None);
        return Some(set_meta(restored, Some(&meta)));
    }

    if meta_original(button, "fcl", Some("direction")).is_some() {
        return None;
    }

    let mut event = crate::styles::fcl_button_event();
    let mut substitutions: Vec<Value> = Vec::new();
    let click_events = button.get("clickEvents").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for click_event in &click_events {
        apply_zl_event_to_fcl(
            ctx,
            click_event,
            &mut event,
            strict,
            Some(&mut substitutions),
            layer_id_map,
        );
    }
    let mut simulated_state = initial_layer_state.clone();
    if !current_layer_id.is_empty() {
        simulated_state.insert(current_layer_id.to_string(), true);
    }
    apply_zl_layer_events_to_fcl(
        ctx,
        &click_events,
        &mut event,
        strict,
        &simulated_state,
        layer_id_map,
        Some(&mut substitutions),
    );

    if to_bool(&get_or(button, "isToggleable", json!(false))) {
        if let Some(press) = event.get_mut("pressEvent").and_then(|p| p.as_object_mut()) {
            press.insert("autoKeep".to_string(), Value::Bool(true));
        }
    }

    let style_uuid = get_or_empty(button, "buttonStyle");
    let style_name = resolve_zl_button_style_name(&style_uuid, style_map, "ZL Native Default");
    let text = text_default(&get_or_empty(button, "text"));
    let base_info = make_base_info_from_zl(ctx, button, layer_visibility, strict, &text, Some(&style_name), fcl_styles);
    let id = {
        let u = to_string_v(&get_or(button, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            fcl_id()
        }
    };
    let result = json!({
        "id": id,
        "text": text,
        "style": style_name,
        "baseInfo": base_info,
        "event": event,
    });
    let origin_id = {
        let u = to_string_v(&get_or(button, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            to_string_v(&get_or(&result, "id", Value::Null))
        }
    };
    let mapping = append_substitutions(None, &substitutions);
    let meta = make_meta("zl", "button", &origin_id, button, mapping.as_ref());
    Some(set_meta(result, Some(&meta)))
}

fn zl_textbox_to_fcl(
    ctx: &mut ConversionContext,
    textbox: &Value,
    layer_visibility: &str,
    style_map: &HashMap<String, String>,
    strict: bool,
    fcl_styles: &[Value],
) -> Option<Value> {
    let original = meta_original(textbox, "fcl", Some("button"));
    if let Some(orig) = original {
        let restored = overlay_shared_fields_fcl(ctx, &orig, textbox, layer_visibility, style_map, strict, fcl_styles);
        let origin_id = {
            let u = to_string_v(&get_or(textbox, "uuid", Value::Null));
            if !u.is_empty() {
                u
            } else {
                let ri = to_string_v(&get_or(&restored, "id", Value::Null));
                if !ri.is_empty() {
                    ri
                } else {
                    fcl_id()
                }
            }
        };
        let meta = make_meta("zl", "textbox", &origin_id, textbox, None);
        return Some(set_meta(restored, Some(&meta)));
    }

    if meta_original(textbox, "fcl", Some("direction")).is_some() {
        return None;
    }

    let style_uuid = get_or_empty(textbox, "buttonStyle");
    let style_name = resolve_zl_button_style_name(&style_uuid, style_map, "ZL Native Default");
    let text = text_default(&get_or_empty(textbox, "text"));
    let base_info = make_base_info_from_zl(ctx, textbox, layer_visibility, strict, &text, Some(&style_name), fcl_styles);
    let id = {
        let u = to_string_v(&get_or(textbox, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            fcl_id()
        }
    };
    let result = json!({
        "id": id,
        "text": text,
        "style": style_name,
        "baseInfo": base_info,
        "event": crate::styles::fcl_button_event(),
    });
    let origin_id = {
        let u = to_string_v(&get_or(textbox, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            to_string_v(&get_or(&result, "id", Value::Null))
        }
    };
    let meta = make_meta("zl", "textbox", &origin_id, textbox, None);
    Some(set_meta(result, Some(&meta)))
}

fn order_fcl_buttons_for_layer(buttons: Vec<Value>) -> Vec<Value> {
    let mut decorated: Vec<(usize, Value)> = buttons.into_iter().enumerate().collect();
    decorated.sort_by_key(|(index, button)| {
        (if crate::events::fcl_button_is_decorative(button) { 0 } else { 1 }, *index)
    });
    decorated.into_iter().map(|(_, b)| b).collect()
}

fn layer_is_background_like(group: &Value) -> bool {
    let buttons = group
        .get("viewData")
        .and_then(|v| v.get("buttonList"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if buttons.is_empty() {
        return false;
    }
    let decorative = buttons
        .iter()
        .filter(|b| crate::events::fcl_button_is_decorative(b))
        .count();
    decorative == buttons.len()
}

fn order_fcl_view_groups(groups: Vec<Value>) -> Vec<Value> {
    let mut decorated: Vec<(usize, Value)> = groups.into_iter().enumerate().collect();
    decorated.sort_by_key(|(index, group)| {
        (if layer_is_background_like(group) { 0 } else { 1 }, *index)
    });
    decorated.into_iter().map(|(_, g)| g).collect()
}

fn infer_visible_companion_layers(
    data: &Value,
    layer_id_map: &HashMap<String, String>,
) -> HashMap<String, HashSet<String>> {
    let mut companions: HashMap<String, HashSet<String>> = HashMap::new();
    let mut layer_ids: HashSet<String> = HashSet::new();
    let mut hidden_layers: HashSet<String> = HashSet::new();
    if let Some(layers) = data.get("layers").and_then(|v| v.as_array()) {
        for layer in layers {
            let u = to_string_v(&get_or(layer, "uuid", json!("")));
            if u.is_empty() {
                continue;
            }
            layer_ids.insert(u.clone());
            if to_bool(&get_or(layer, "hide", json!(false))) {
                hidden_layers.insert(u);
            }
        }
    }
    let mut opener_targets: HashMap<String, HashSet<String>> = HashMap::new();

    if let Some(layers) = data.get("layers").and_then(|v| v.as_array()) {
        for layer in layers {
            let source_id = to_string_v(&get_or(layer, "uuid", json!("")));
            let buttons = layer
                .get("normalButtons")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for button in &buttons {
                let events = button
                    .get("clickEvents")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut visible_targets: Vec<String> = Vec::new();
                for event in &events {
                    let etype = event.get("type").map(to_string_v).unwrap_or_default();
                    let raw_key = py_str(&get_or(event, "key", json!("")));
                    let target_id = layer_id_map
                        .get(&raw_key)
                        .cloned()
                        .unwrap_or_else(|| raw_key.clone());
                    if (etype != "show_layer" && etype != "switch_layer")
                        || !layer_ids.contains(&target_id)
                    {
                        continue;
                    }
                    visible_targets.push(target_id.clone());
                    if !source_id.is_empty() {
                        opener_targets
                            .entry(target_id)
                            .or_default()
                            .insert(source_id.clone());
                    }
                }
                if visible_targets.len() < 2 {
                    continue;
                }
                let group: HashSet<String> = visible_targets.into_iter().collect();
                for target_id in &group {
                    companions
                        .entry(target_id.clone())
                        .or_default()
                        .extend(group.clone());
                }
            }
        }
    }

    let opener_entries: Vec<(String, HashSet<String>)> = opener_targets
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    for (target_id, source_ids) in opener_entries {
        if !hidden_layers.contains(&target_id) {
            continue;
        }
        let empty: HashSet<String> = HashSet::new();
        let companion_ids = companions.get(&target_id).unwrap_or(&empty).clone();
        let co_opened = companion_ids.iter().any(|c| opener_targets.contains_key(c));
        if !co_opened {
            continue;
        }
        for source_id in &source_ids {
            if !source_id.is_empty()
                && !companion_ids.contains(source_id)
                && hidden_layers.contains(source_id)
            {
                companions
                    .entry(target_id.clone())
                    .or_default()
                    .insert(source_id.clone());
            }
        }
    }

    companions
}

fn make_direction_base_info_from_zl(joystick: &Value, layer_visibility: &str) -> Value {
    let size_type_raw = to_string_v(&get_or(joystick, "sizeType", json!("Percentage")))
        .to_lowercase();
    let fcl_size_type: &str;
    let absolute: i64;
    let percentage: i64;
    if size_type_raw == "dp" || size_type_raw == "dip" || size_type_raw == "absolute" {
        fcl_size_type = "ABSOLUTE";
        absolute = 5.max(clamp_int(&get_or(joystick, "sizeDp", inum(50)), 50));
        percentage = 300;
    } else {
        fcl_size_type = "PERCENTAGE";
        absolute = 5.max(clamp_int(&get_or(joystick, "sizeDp", inum(50)), 50));
        percentage = 100.max(1000.min(clamp_int(&get_or(joystick, "sizePercentage", inum(2500)), 0) / 10));
    }
    let visibility_src = joystick
        .get("visibilityType")
        .map(to_string_v)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| layer_visibility.to_string());
    let empty_pos = obj();
    let position = joystick.get("position").filter(|v| !v.is_null()).unwrap_or(&empty_pos);
    json!({
        "visibilityType": visibility_zl_to_fcl(&visibility_src),
        "xPosition": inum(scale_position_to_fcl(&get_or(position, "x", inum(0)))),
        "yPosition": inum(scale_position_to_fcl(&get_or(position, "y", inum(0)))),
        "sizeType": fcl_size_type,
        "absoluteWidth": inum(absolute),
        "absoluteHeight": inum(absolute),
        "percentageWidth": { "reference": "SCREEN_HEIGHT", "size": inum(percentage) },
        "percentageHeight": { "reference": "SCREEN_HEIGHT", "size": inum(percentage) },
    })
}

fn overlay_shared_fields_fcl_direction(
    original: &Value,
    joystick: &Value,
    layer_visibility: &str,
) -> Value {
    let mut restored = original.clone();
    let restored_id = {
        let u = to_string_v(&get_or(joystick, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            let ri = to_string_v(&get_or(&restored, "id", Value::Null));
            if !ri.is_empty() {
                ri
            } else {
                fcl_id()
            }
        }
    };
    if let Value::Object(m) = &mut restored {
        m.insert("id".to_string(), json!(restored_id));
        m.insert(
            "baseInfo".to_string(),
            make_direction_base_info_from_zl(joystick, layer_visibility),
        );
    }
    restored
}

fn zl_joystick_to_fcl_direction(
    ctx: &mut ConversionContext,
    joystick: &Value,
    layer_visibility: &str,
    strict: bool,
    style_name: &str,
) -> Option<Value> {
    let original = meta_original(joystick, "fcl", Some("direction"));
    if let Some(orig) = original {
        let restored = overlay_shared_fields_fcl_direction(&orig, joystick, layer_visibility);
        let origin_id = {
            let u = to_string_v(&get_or(joystick, "uuid", Value::Null));
            if !u.is_empty() {
                u
            } else {
                let ri = to_string_v(&get_or(&restored, "id", Value::Null));
                if !ri.is_empty() {
                    ri
                } else {
                    fcl_id()
                }
            }
        };
        let meta = make_meta("zl", "joystick", &origin_id, joystick, None);
        return Some(set_meta(restored, Some(&meta)));
    }

    let direction_events = joystick
        .get("directionEvents")
        .cloned()
        .unwrap_or_else(obj);

    let mut keycodes_for = |name: &str| -> Vec<i64> {
        let mut keycodes = Vec::new();
        let events = direction_events
            .get(name)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for event in events {
            if !event.is_object() {
                continue;
            }
            let etype = event.get("type").map(to_string_v).unwrap_or_default();
            if etype != "key" {
                continue;
            }
            let key = py_str(&get_or(&event, "key", json!("")));
            let keycode = convert_key_to_fcl(ctx, &key, strict, None);
            keycodes.push(keycode);
        }
        keycodes
    };

    let direction_id = {
        let u = to_string_v(&get_or(joystick, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            fcl_id()
        }
    };
    let direction_obj = json!({
        "id": direction_id,
        "baseInfo": make_direction_base_info_from_zl(joystick, layer_visibility),
        "event": {
            "upKeycode": keycodes_for("north").into_iter().map(inum).collect::<Vec<_>>(),
            "downKeycode": keycodes_for("south").into_iter().map(inum).collect::<Vec<_>>(),
            "leftKeycode": keycodes_for("west").into_iter().map(inum).collect::<Vec<_>>(),
            "rightKeycode": keycodes_for("east").into_iter().map(inum).collect::<Vec<_>>(),
        },
        "style": style_name,
    });
    let origin_id = {
        let u = to_string_v(&get_or(joystick, "uuid", Value::Null));
        if !u.is_empty() {
            u
        } else {
            to_string_v(&get_or(&direction_obj, "id", Value::Null))
        }
    };
    let meta = make_meta("zl", "joystick", &origin_id, joystick, None);
    Some(set_meta(direction_obj, Some(&meta)))
}

fn fcl_rocker_style_matches(rocker_a: &Value, rocker_b: &Value) -> bool {
    let comparable = [
        "rockerSize",
        "bgCornerRadius",
        "bgStrokeWidth",
        "bgStrokeColor",
        "bgFillColor",
        "rockerCornerRadius",
        "rockerFillColor",
    ];
    for key in comparable {
        let a = rocker_a.get(key).cloned().unwrap_or(Value::Null);
        let b = rocker_b.get(key).cloned().unwrap_or(Value::Null);
        if a != b {
            return false;
        }
    }
    true
}

fn zl_joystick_styles_to_fcl_direction_styles(
    ctx: &mut ConversionContext,
    joystick_styles: &[Value],
    existing_styles: &[Value],
) -> (Vec<Value>, HashMap<String, String>) {
    let mut result: Vec<Value> = Vec::new();
    let mut mapping: HashMap<String, String> = HashMap::new();
    let mut used_names: HashSet<String> = HashSet::new();
    for style in existing_styles {
        if !style.is_object() {
            continue;
        }
        used_names.insert(py_str(&get_or(style, "name", Value::Null)));
    }
    // cc.py builds a {name: style} dict (insertion order, last value wins per
    // name) and iterates its values in first-insertion order; mirror that so
    // tie-breaking between identical rocker styles stays deterministic.
    let mut existing_rockers: Vec<(String, &Value)> = Vec::new();
    let mut existing_by_name: HashMap<String, &Value> = HashMap::new();
    for style in existing_styles {
        if !style.is_object() {
            continue;
        }
        let name = py_str(&get_or(style, "name", Value::Null));
        if to_string_v(&get_or(style, "styleType", json!(""))) == "ROCKER" {
            if existing_by_name.contains_key(&name) {
                if let Some(slot) = existing_rockers.iter_mut().find(|(n, _)| *n == name) {
                    slot.1 = style;
                }
            } else {
                existing_rockers.push((name.clone(), style));
            }
            existing_by_name.insert(name, style);
        }
    }
    let default_direction = default_fcl_direction_style();
    let default_button_style = default_direction
        .get("buttonStyle")
        .cloned()
        .unwrap_or_else(obj);
    for style in joystick_styles {
        if !style.is_object() {
            continue;
        }
        let uuid_value = py_str(&get_or(style, "uuid", json!("")));
        let base_name = {
            let n = to_string_v(&get_or(style, "name", json!("")));
            if !n.is_empty() {
                n
            } else if !uuid_value.is_empty() {
                uuid_value.clone()
            } else {
                "Joystick".to_string()
            }
        };
        let converted_rocker = zl_joystick_style_to_fcl_rocker(style);
        let mut matched_name = String::new();
        let original_style = meta_original(style, "fcl", Some("directionStyle"));
        if let Some(orig) = original_style {
            let candidate = py_str(&get_or(&orig, "name", Value::Null));
            if existing_by_name.contains_key(&candidate) {
                matched_name = candidate;
            }
        }
        if matched_name.is_empty() {
            for (existing_name, existing) in &existing_rockers {
                let rocker = existing.get("rockerStyle").cloned().unwrap_or(Value::Null);
                if fcl_rocker_style_matches(&converted_rocker, &rocker) {
                    matched_name = existing_name.clone();
                    break;
                }
            }
        }
        if !matched_name.is_empty() {
            mapping.insert(uuid_value, matched_name);
            continue;
        }
        let mut name = style_name_for_zl_style(&base_name, &uuid_value);
        let mut suffix = 2;
        while used_names.contains(&name) {
            name = format!("{}_{}", style_name_for_zl_style(&base_name, &uuid_value), suffix);
            suffix += 1;
        }
        used_names.insert(name.clone());
        if !uuid_value.is_empty() {
            mapping.insert(uuid_value.clone(), name.clone());
        }
        result.push(json!({
            "name": name,
            "styleType": "ROCKER",
            "buttonStyle": default_button_style.clone(),
            "rockerStyle": converted_rocker,
        }));
    }
    let _ = ctx;
    (result, mapping)
}

pub fn zl_to_fcl(ctx: &mut ConversionContext, data: &Value, strict: bool) -> Value {
    let root_original = meta_original(data, "fcl", Some("controller"));
    let info = data.get("info").cloned().unwrap_or_else(obj);
    let empty_styles: Vec<Value> = Vec::new();
    let styles_in = data
        .get("styles")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let (styles, style_map) = zl_styles_to_fcl(if styles_in.is_empty() {
        &empty_styles
    } else {
        &styles_in
    });
    let mut existing_direction_styles = root_original
        .as_ref()
        .and_then(|ro| ro.get("directionStyles"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if existing_direction_styles.is_empty() {
        existing_direction_styles = vec![default_fcl_direction_style()];
    }
    let joystick_styles_in = data
        .get("joystickStyles")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let (joystick_style_styles, joystick_style_names) = zl_joystick_styles_to_fcl_direction_styles(
        ctx,
        &joystick_styles_in,
        &existing_direction_styles,
    );
    let mut view_groups: Vec<Value> = Vec::new();

    let mut layer_id_map: HashMap<String, String> = HashMap::new();
    let mut initial_layer_state: HashMap<String, bool> = HashMap::new();
    if let Some(layers) = data.get("layers").and_then(|v| v.as_array()) {
        for layer in layers {
            let layer_original = meta_original(layer, "fcl", Some("viewGroup"));
            let layer_uuid = {
                let u = to_string_v(&get_or(layer, "uuid", Value::Null));
                if !u.is_empty() {
                    u
                } else {
                    let rid = layer_original
                        .as_ref()
                        .and_then(|r| truthy_str(&get_or(r, "id", Value::Null)));
                    rid.unwrap_or_else(fcl_id)
                }
            };
            let key = {
                let u = to_string_v(&get_or(layer, "uuid", Value::Null));
                if !u.is_empty() {
                    u
                } else {
                    layer_uuid.clone()
                }
            };
            layer_id_map.insert(key, layer_uuid.clone());
            initial_layer_state.insert(layer_uuid, !to_bool(&get_or(layer, "hide", json!(false))));
        }
    }
    let companion_layers = infer_visible_companion_layers(data, &layer_id_map);

    if let Some(layers) = data.get("layers").and_then(|v| v.as_array()) {
        for layer in layers {
            let layer_original = meta_original(layer, "fcl", Some("viewGroup"));
            warn_unmapped_layer_flags(ctx, layer, strict);
            let layer_visibility = to_string_v(&get_or(layer, "visibilityType", json!("always")));
            let layer_uuid = to_string_v(&get_or(layer, "uuid", json!("")));
            let current_layer_id = layer_id_map
                .get(&layer_uuid)
                .cloned()
                .unwrap_or_else(|| layer_uuid.clone());
            let mut layer_state_for_buttons = initial_layer_state.clone();
            match companion_layers.get(&current_layer_id) {
                Some(companions) => {
                    for companion_id in companions {
                        layer_state_for_buttons.insert(companion_id.clone(), true);
                    }
                }
                None => {
                    layer_state_for_buttons.insert(current_layer_id.clone(), true);
                }
            }
            let mut buttons: Vec<Value> = Vec::new();
            let normal_buttons = layer
                .get("normalButtons")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for button in &normal_buttons {
                if let Some(converted) = zl_button_to_fcl(
                    ctx,
                    button,
                    &layer_visibility,
                    &style_map,
                    strict,
                    &layer_id_map,
                    &styles,
                    &layer_state_for_buttons,
                    &current_layer_id,
                ) {
                    buttons.push(converted);
                }
            }
            let text_boxes = layer
                .get("textBoxes")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for textbox in &text_boxes {
                if let Some(converted) = zl_textbox_to_fcl(
                    ctx,
                    textbox,
                    &layer_visibility,
                    &style_map,
                    strict,
                    &styles,
                ) {
                    buttons.push(converted);
                }
            }
            let mut restored_group = match &layer_original {
                Some(lo) => lo.clone(),
                None => obj(),
            };
            let mut direction_list: Vec<Value> = restored_group
                .get("viewData")
                .and_then(|v| v.get("directionList"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let restored_direction_ids: HashSet<String> = direction_list
                .iter()
                .filter_map(|item| truthy_str(&get_or(item, "id", Value::Null)))
                .collect();
            let joystick_buttons = layer
                .get("joystickButtons")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for joystick in &joystick_buttons {
                if !joystick.is_object() {
                    continue;
                }
                let joystick_original = meta_original(joystick, "fcl", Some("direction"));
                if let Some(orig) = &joystick_original {
                    let orig_id = to_string_v(&get_or(orig, "id", Value::Null));
                    if restored_direction_ids.contains(&orig_id) {
                        continue;
                    }
                }
                let style_id = py_str(&get_or(joystick, "joystickStyleId", json!("")));
                let mut style_name = joystick_style_names.get(&style_id).cloned();
                if style_name.is_none() {
                    let layer_name = py_str(&get_or(layer, "name", Value::Null));
                    ctx.warn(
                        &format!(
                            "ZL joystick on layer {:?} references unknown joystickStyleId; using ROCKER style from its style definition",
                            layer_name
                        ),
                        strict,
                        true,
                    );
                    style_name = Some("ZL Joystick".to_string());
                }
                let converted_direction = zl_joystick_to_fcl_direction(
                    ctx,
                    joystick,
                    &layer_visibility,
                    strict,
                    style_name.as_deref().unwrap_or("ZL Joystick"),
                );
                if let Some(d) = converted_direction {
                    direction_list.push(d);
                }
            }
            let group_id = {
                let u = to_string_v(&get_or(layer, "uuid", Value::Null));
                if !u.is_empty() {
                    u
                } else {
                    let rid = truthy_str(&get_or(&restored_group, "id", Value::Null));
                    rid.unwrap_or_else(fcl_id)
                }
            };
            let group_name = {
                let n = to_string_v(&get_or(layer, "name", Value::Null));
                if !n.is_empty() {
                    n
                } else {
                    let rn = truthy_str(&get_or(&restored_group, "name", Value::Null));
                    rn.unwrap_or_else(|| "Layer".to_string())
                }
            };
            let result_group = json!({
                "id": group_id,
                "name": group_name,
                "visibility": if to_bool(&get_or(layer, "hide", json!(false))) { "INVISIBLE" } else { "VISIBLE" },
                "viewData": {
                    "buttonList": order_fcl_buttons_for_layer(buttons),
                    "directionList": direction_list.clone(),
                },
            });
            let origin_id = {
                let u = to_string_v(&get_or(layer, "uuid", Value::Null));
                if !u.is_empty() {
                    u
                } else {
                    to_string_v(&get_or(&result_group, "id", Value::Null))
                }
            };
            let mut meta = make_meta("zl", "layer", &origin_id, layer, None);
            if !direction_list.is_empty() {
                if let Some(original) = meta.get_mut("original").and_then(|o| o.as_object_mut()) {
                    original.insert(
                        "directionList".to_string(),
                        Value::Array(direction_list.clone()),
                    );
                }
            }
            let result_group = set_meta(result_group, Some(&meta));
            view_groups.push(result_group);
        }
    }

    let mut result = match &root_original {
        Some(ro) if ro.is_object() => ro.clone(),
        _ => obj(),
    };
    let (new_id, new_name, new_version, new_version_code, new_author, new_description, new_controller_version) = {
        let id = {
            let di = to_string_v(&get_or(data, "id", Value::Null));
            if !di.is_empty() {
                di
            } else {
                let ri = truthy_str(&get_or(&result, "id", Value::Null));
                ri.unwrap_or_else(|| short_id()[..8].to_string())
            }
        };
        let name = {
            let n = text_default(&get_or(&info, "name", Value::Null));
            if !n.is_empty() {
                n
            } else {
                let rn = truthy_str(&get_or(&result, "name", Value::Null));
                rn.unwrap_or_else(|| "Converted from Zalith".to_string())
            }
        };
        let version = {
            let v = text_default(&get_or(&info, "versionName", Value::Null));
            if !v.is_empty() {
                v
            } else {
                let rv = truthy_str(&get_or(&result, "version", Value::Null));
                rv.unwrap_or_else(|| "1.0".to_string())
            }
        };
        let version_code_default = clamp_int(&get_or(&result, "versionCode", inum(1)), 1);
        let version_code = clamp_int(&get_or(&info, "versionCode", inum(version_code_default)), 1);
        let author = {
            let a = text_default(&get_or(&info, "author", Value::Null));
            if !a.is_empty() {
                a
            } else {
                let ra = truthy_str(&get_or(&result, "author", Value::Null));
                ra.unwrap_or_default()
            }
        };
        let description = {
            let d = text_default(&get_or(&info, "description", Value::Null));
            if !d.is_empty() {
                d
            } else {
                let rd = truthy_str(&get_or(&result, "description", Value::Null));
                rd.unwrap_or_default()
            }
        };
        let controller_version = clamp_int(
            &result
                .get("controllerVersion")
                .cloned()
                .unwrap_or(Value::Null),
            crate::constants::FCL_CONTROLLER_VERSION,
        );
        (
            id,
            name,
            version,
            version_code,
            author,
            description,
            controller_version,
        )
    };
    if let Value::Object(m) = &mut result {
        m.insert("id".to_string(), json!(new_id));
        m.insert("name".to_string(), json!(new_name));
        m.insert("version".to_string(), json!(new_version));
        m.insert("versionCode".to_string(), inum(new_version_code));
        m.insert("author".to_string(), json!(new_author));
        m.insert("description".to_string(), json!(new_description));
        m.insert("controllerVersion".to_string(), inum(new_controller_version));
        m.insert("buttonStyles".to_string(), Value::Array(styles));
        let mut all_direction_styles = existing_direction_styles.clone();
        all_direction_styles.extend(joystick_style_styles);
        m.insert("directionStyles".to_string(), Value::Array(all_direction_styles));
        m.insert("viewGroups".to_string(), Value::Array(order_fcl_view_groups(view_groups)));
    }
    let origin_id = {
        let di = to_string_v(&get_or(data, "id", Value::Null));
        if !di.is_empty() {
            di
        } else {
            to_string_v(&get_or(&result, "id", Value::Null))
        }
    };
    let meta = make_meta("zl", "layout", &origin_id, data, None);
    set_meta(result, Some(&meta))
}

pub fn convert_zl_to_fcl(ctx: &mut ConversionContext, data: &Value, strict: bool) -> Value {
    zl_to_fcl(ctx, data, strict)
}
