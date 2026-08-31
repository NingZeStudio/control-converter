use crate::context::ConversionContext;
use crate::events::{fcl_event_has_payload, fcl_event_to_zl_events, normalize_zl_click_events, strconv_quote};
use crate::styles::make_zl_button_size;
use crate::utils::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::ptr;

pub fn overlay_shared_fields_zl(
    original: &Value,
    current: &Value,
    style_map: &HashMap<String, String>,
    absolute_as_percentage: bool,
    aspect: f64,
) -> Value {
    let mut restored = original.clone();
    let base_info = current
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let source_text = get_or_empty(current, "text");
    let text_value = if source_text.is_object() {
        translatable(&text_default(&source_text), Some(&source_text))
    } else {
        let prev = restored.get("text").cloned();
        translatable(&to_string_v(&source_text), prev.as_ref())
    };
    let uuid = to_string_v(&get_or(
        current,
        "id",
        restored
            .get("uuid")
            .cloned()
            .unwrap_or_else(|| json!(format!("{}{}", short_id(), &short_id()[..6]))),
    ));
    let button_style = get_style_from_map(
        style_map,
        &to_string_v(&get_or(current, "style", json!("Default"))),
        restored.get("buttonStyle").cloned(),
    );
    if let Value::Object(m) = &mut restored {
        m.insert("text".to_string(), text_value);
        m.insert("uuid".to_string(), json!(uuid));
        m.insert(
            "position".to_string(),
            json!({
                "x": inum(scale_position_to_zl(&get_or(&base_info, "xPosition", inum(0)))),
                "y": inum(scale_position_to_zl(&get_or(&base_info, "yPosition", inum(0)))),
            }),
        );
        m.insert(
            "buttonSize".to_string(),
            make_zl_button_size(&base_info, absolute_as_percentage, aspect),
        );
        m.insert("buttonStyle".to_string(), button_style);
        m.insert(
            "visibilityType".to_string(),
            json!(visibility_fcl_to_zl(&to_string_v(&get_or(
                &base_info, "visibilityType", json!("")
            )))),
        );
    }
    restored
}

pub fn get_style_from_map(
    style_map: &HashMap<String, String>,
    name: &str,
    default_val: Option<Value>,
) -> Value {
    match style_map.get(name) {
        Some(uuid) => json!(uuid),
        None => default_val.unwrap_or(Value::Null),
    }
}

pub fn fcl_button_to_zl_textbox(

    button: &Value,
    style_map: &HashMap<String, String>,
    absolute_as_percentage: bool,
    aspect: f64,
) -> Value {
    let original = meta_original(button, "zl", None);
    if let Some(orig_map) = original {
        if orig_map.is_object() && orig_map.get("clickEvents").is_none() {
            let restored = overlay_shared_fields_zl(&orig_map, button, style_map, absolute_as_percentage, aspect);
            let uuid_default = restored
                .get("uuid")
                .cloned()
                .unwrap_or_else(|| json!(short_id()));
            let origin_id = to_string_v(&get_or(button, "id", uuid_default));
            let mapping = json!({ "synthetic": true, "generatedFrom": "decorative-textbox" });
            return set_meta(restored, Some(&make_meta("fcl", "button", &origin_id, button, Some(&mapping))));
        }
    }

    let base_info = button
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let text = to_string_v(&get_or(button, "text", json!("")));
    let style_name = to_string_v(&get_or(button, "style", json!("Default")));
    let button_style = match style_map.get(&style_name) {
        Some(uuid) => json!(uuid),
        None => json!(""),
    };
    let uuid = to_string_v(&get_or(
        button,
        "id",
        json!(format!("{}{}", short_id(), &short_id()[..6])),
    ));
    let result = json!({
        "text": translatable(&text, None),
        "uuid": uuid,
        "position": {
            "x": inum(scale_position_to_zl(&get_or(&base_info, "xPosition", inum(0)))),
            "y": inum(scale_position_to_zl(&get_or(&base_info, "yPosition", inum(0)))),
        },
        "buttonSize": make_zl_button_size(&base_info, absolute_as_percentage, aspect),
        "buttonStyle": button_style,
        "textAlignment": "Center",
        "textBold": false,
        "textItalic": false,
        "textUnderline": false,
        "visibilityType": visibility_fcl_to_zl(&to_string_v(&get_or(&base_info, "visibilityType", json!("")))),
    });
    let mapping = json!({ "synthetic": true, "generatedFrom": "decorative-textbox" });
    let origin_id = to_string_v(&get_or(
        button,
        "id",
        result.get("uuid").cloned().unwrap_or_else(|| json!(short_id())),
    ));
    set_meta(result, Some(&make_meta("fcl", "button", &origin_id, button, Some(&mapping))))
}

pub fn fcl_button_to_zl(
    ctx: &mut ConversionContext,
    button: &Value,
    style_map: &HashMap<String, String>,
    strict: bool,
    _group_name: &str,
    group_ids_by_name: &GroupIdsByName,
    visual_button: Option<&Value>,
    absolute_as_percentage: bool,
    aspect: f64,
) -> Value {
    let original = meta_original(button, "zl", None);
    if let Some(orig_map) = original {
        if orig_map.is_object() && orig_map.get("clickEvents").is_some() {
            let vb = visual_button.unwrap_or(button);
            let restored = overlay_shared_fields_zl(&orig_map, vb, style_map, absolute_as_percentage, aspect);
            let uuid_default = restored
                .get("uuid")
                .cloned()
                .unwrap_or_else(|| json!(short_id()));
            let origin_id = to_string_v(&get_or(button, "id", uuid_default));
            return set_meta(restored, Some(&make_meta("fcl", "button", &origin_id, button, None)));
        }
    }

    let visual_button = visual_button.unwrap_or(button);
    let base_info = visual_button
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let event_root = button
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let text = to_string_v(&get_or(
        visual_button,
        "text",
        json!(to_string_v(&get_or(button, "text", json!("")))),
    ));

    let mut click_events: Vec<Value> = Vec::new();
    let mut substitutions: Vec<Value> = Vec::new();

    let mut meaningful_events: Vec<&str> = Vec::new();
    for event_name in ["pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"] {
        let event = event_root
            .get(event_name)
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        if fcl_event_has_payload(&event) {
            meaningful_events.push(event_name);
        }
    }

    for event_name in ["pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"] {
        let event = event_root
            .get(event_name)
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        click_events.extend(fcl_event_to_zl_events(
            ctx,
            &event,
            strict,
            &text,
            event_name,
            group_ids_by_name,
            &mut substitutions,
        ));
    }
    let click_events = normalize_zl_click_events(click_events);

    let press_event = event_root
        .get("pressEvent")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let press_keycodes = fcl_keycode_list(&get_or(&press_event, "outputKeycodes", Value::Null));
    let press_keycode_arr = press_keycodes.as_array().cloned().unwrap_or_default();
    let can_toggle = to_bool(&get_or(&press_event, "autoKeep", json!(false)))
        && !press_keycode_arr.is_empty()
        && meaningful_events.len() == 1
        && meaningful_events[0] == "pressEvent";

    if to_bool(&get_or(&event_root, "Movable", json!(false))) {
        let reason = "FCL movable button cannot be represented in ZL layout JSON; preserved in metadata";
        let message = format!("{} on button {}", reason, strconv_quote(&text));
        ctx.warn(&message, strict, true);
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_button_flag", "key": "Movable" }),
            &json!({ "type": "metadata_only" }),
            reason,
            "events",
        ));
    }
    if to_bool(&get_or(&event_root, "pointerFollow", json!(false))) {
        let has_mouse_keycode = press_keycode_arr
            .iter()
            .any(|k| crate::constants::FCL_MOUSE_REVERSE.contains_key(&clamp_int(k, 0)));
        if !has_mouse_keycode {
            let reason = "FCL pointerFollow cannot be represented exactly in ZL; preserved in metadata";
            let message = format!("{} on button {}", reason, strconv_quote(&text));
            ctx.warn(&message, strict, true);
            substitutions.push(substitution(
                ctx,
                &json!({ "type": "fcl_button_flag", "key": "pointerFollow" }),
                &json!({ "type": "metadata_only" }),
                reason,
                "events",
            ));
        }
    }

    let is_decorative = click_events.is_empty();
    let style_name = to_string_v(&get_or(
        visual_button,
        "style",
        json!(to_string_v(&get_or(button, "style", json!("Default")))),
    ));
    let button_style = get_style_from_map(style_map, &style_name, None);
    let uuid = to_string_v(&get_or(
        button,
        "id",
        json!(format!("{}{}", short_id(), &short_id()[..6])),
    ));
    let result = json!({
        "text": translatable(&text, None),
        "uuid": uuid,
        "position": {
            "x": inum(scale_position_to_zl(&get_or(&base_info, "xPosition", inum(0)))),
            "y": inum(scale_position_to_zl(&get_or(&base_info, "yPosition", inum(0)))),
        },
        "buttonSize": make_zl_button_size(&base_info, absolute_as_percentage, aspect),
        "buttonStyle": button_style,
        "textAlignment": "Center",
        "textBold": false,
        "textItalic": false,
        "textUnderline": false,
        "visibilityType": visibility_fcl_to_zl(&to_string_v(&get_or(&base_info, "visibilityType", json!("")))),
        "clickEvents": click_events,
        "isSwipple": is_decorative,
        "isPenetrable": is_decorative,
        "isToggleable": can_toggle,
    });

    let mut mapping: Option<Value> = None;
    if !ptr::eq(visual_button, button) {
        mapping = Some(json!({
            "synthetic": true,
            "generatedFrom": "overlay-merge",
            "pairedVisualId": to_string_v(&get_or(visual_button, "id", json!(""))),
            "pairedEventId": to_string_v(&get_or(button, "id", json!(""))),
        }));
    }
    let mapping = append_substitutions(mapping.as_ref(), &substitutions);
    let origin_id = to_string_v(&get_or(
        button,
        "id",
        result.get("uuid").cloned().unwrap_or_else(|| json!(short_id())),
    ));
    let meta = make_meta("fcl", "button", &origin_id, button, mapping.as_ref());
    set_meta(result, Some(&meta))
}

pub fn click_events_to_interface(events: Vec<Value>) -> Value {
    Value::Array(events)
}
