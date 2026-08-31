use crate::context::ConversionContext;
use crate::constants::FCL_SCROLL_REVERSE;
use crate::utils::*;
use serde_json::{json, Value};

pub fn fcl_event_has_payload(event: &Value) -> bool {
    let kcl = fcl_keycode_list(&get_or(event, "outputKeycodes", Value::Null));
    if kcl.as_array().map_or(false, |a| !a.is_empty()) {
        return true;
    }
    if to_bool(&get_or(event, "input", json!(false))) {
        return true;
    }
    if to_bool(&get_or(event, "openMenu", json!(false))) {
        return true;
    }
    if !to_string_v(&get_or(event, "outputText", json!(""))).is_empty() {
        return true;
    }
    if !get_or_list(event, "bindViewGroup").is_empty() {
        return true;
    }
    if to_bool(&get_or(event, "switchTouchMode", json!(false))) {
        return true;
    }
    if to_bool(&get_or(event, "switchMouseMode", json!(false))) {
        return true;
    }
    if to_bool(&get_or(event, "quickInput", json!(false))) {
        return true;
    }
    false
}

pub fn fcl_button_has_payload(button: &Value) -> bool {
    let event_root = button
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    for event_name in ["pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"] {
        let event = event_root
            .get(event_name)
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        if fcl_event_has_payload(&event) {
            return true;
        }
    }
    false
}

pub fn fcl_button_is_decorative(button: &Value) -> bool {
    if fcl_button_has_payload(button) {
        return false;
    }
    let event_root = button
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    if to_bool(&get_or(&event_root, "pointerFollow", json!(false)))
        || to_bool(&get_or(&event_root, "Movable", json!(false)))
    {
        return false;
    }
    true
}

pub fn fcl_event_to_zl_events(
    ctx: &mut ConversionContext,
    event: &Value,
    strict: bool,
    label: &str,
    event_name: &str,
    group_ids_by_name: &GroupIdsByName,
    substitutions: &mut Vec<Value>,
) -> Vec<Value> {
    let mut result: Vec<Value> = Vec::new();
    let auto_click = to_bool(&get_or(event, "autoClick", json!(false)));
    let mut keycodes: Vec<i64> = Vec::new();
    for kc in fcl_keycode_list(&get_or(event, "outputKeycodes", Value::Null))
        .as_array()
        .cloned()
        .unwrap_or_default()
    {
        keycodes.push(clamp_int(&kc, 0));
    }

    if (event_name == "clickEvent" || event_name == "doubleClickEvent") && fcl_event_has_payload(event) {
        let reason = format!(
            "FCL {} has no exact ZL timing equivalent; converted to a normal ZL press/release event",
            event_name
        );
        let message = format!("{} on button {}", reason, strconv_quote(label));
        ctx.warn(&message, strict, true);
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_event", "event": event_name }),
            &json!({ "type": "zl_click_events" }),
            &reason,
            "events",
        ));
    }
    if event_name == "longPressEvent" && fcl_event_has_payload(event) {
        let reason = "FCL longPressEvent has no exact ZL timing equivalent; converted to a normal event";
        let message = format!("{} on button {}", reason, strconv_quote(label));
        ctx.warn(&message, strict, true);
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_event", "event": event_name }),
            &json!({ "type": "zl_click_events" }),
            reason,
            "events",
        ));
    }
    if auto_click {
        let has_non_scroll = keycodes.iter().any(|kc| !FCL_SCROLL_REVERSE.contains_key(kc));
        if has_non_scroll {
            let reason = "FCL autoClick only has a ZL equivalent for scroll events; non-scroll keys are converted as normal press events";
            ctx.warn(reason, strict, true);
            substitutions.push(substitution(
                ctx,
                &json!({ "type": "fcl_auto_click", "event": event_name }),
                &json!({ "type": "zl_normal_press" }),
                reason,
                "events",
            ));
        }
    }

    for keycode in &keycodes {
        if let Some(converted) =
            convert_key_to_zl(ctx, *keycode, strict, auto_click, label, Some(substitutions))
        {
            result.push(json!({
                "type": converted.event_type,
                "key": converted.key,
            }));
        }
    }
    if to_bool(&get_or(event, "input", json!(false))) {
        result.push(json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_ime",
        }));
    }
    if to_bool(&get_or(event, "openMenu", json!(false))) {
        result.push(json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_menu",
        }));
    }
    if to_bool(&get_or(event, "switchTouchMode", json!(false))) {
        let reason = "FCL switchTouchMode has no ZL equivalent; substituted with launcher menu toggle";
        ctx.warn(reason, strict, true);
        result.push(json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_menu",
        }));
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_event", "key": "switchTouchMode" }),
            &json!({ "type": "launcher_event", "key": "launcher.event.switch_menu" }),
            reason,
            "events",
        ));
    }
    if to_bool(&get_or(event, "switchMouseMode", json!(false))) {
        let reason = "FCL switchMouseMode has no ZL equivalent; substituted with launcher menu toggle";
        ctx.warn(reason, strict, true);
        result.push(json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_menu",
        }));
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_event", "key": "switchMouseMode" }),
            &json!({ "type": "launcher_event", "key": "launcher.event.switch_menu" }),
            reason,
            "events",
        ));
    }
    if to_bool(&get_or(event, "quickInput", json!(false))) {
        let reason = "FCL quickInput has no ZL equivalent; substituted with input method toggle";
        ctx.warn(reason, strict, true);
        result.push(json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_ime",
        }));
        substitutions.push(substitution(
            ctx,
            &json!({ "type": "fcl_event", "key": "quickInput" }),
            &json!({ "type": "launcher_event", "key": "launcher.event.switch_ime" }),
            reason,
            "events",
        ));
    }
    let output_text = to_string_v(&get_or(event, "outputText", json!("")));
    if !output_text.is_empty() {
        result.push(json!({
            "type": "send_text",
            "key": output_text,
        }));
    }

    let mut bind_groups: Vec<String> = Vec::new();
    for group_id in get_or_list(event, "bindViewGroup") {
        bind_groups.push(to_string_v(group_id));
    }

    let mut suppress_chat_layer = false;
    let chat_id = group_ids_by_name
        .get("聊天")
        .cloned()
        .unwrap_or_default();
    if !chat_id.is_empty() {
        let has_key_t = result.iter().any(|item| {
            to_string_v(&get_or(item, "type", json!(""))) == "key"
                && to_string_v(&get_or(item, "key", json!(""))) == "GLFW_KEY_T"
        });
        if has_key_t && bind_groups.iter().any(|gid| *gid == chat_id) {
            suppress_chat_layer = true;
        }
    }

    for group_id in bind_groups {
        if suppress_chat_layer && group_id == chat_id {
            continue;
        }
        result.push(json!({
            "type": "switch_layer",
            "key": group_id,
        }));
    }
    result
}

pub fn strconv_quote(s: &str) -> String {
    let mut b = String::new();
    b.push('"');
    for r in s.chars() {
        match r {
            '"' => b.push_str("\\\""),
            '\\' => b.push_str("\\\\"),
            '\n' => b.push_str("\\n"),
            '\r' => b.push_str("\\r"),
            '\t' => b.push_str("\\t"),
            _ => b.push(r),
        }
    }
    b.push('"');
    b
}

pub fn normalize_zl_click_events(events: Vec<Value>) -> Vec<Value> {
    let deduped = dedupe_events(events);
    let mut send_text_events: Vec<Value> = Vec::new();
    let mut other_events: Vec<Value> = Vec::new();
    for event in deduped {
        if to_string_v(&get_or(&event, "type", json!(""))) == "send_text"
            && !to_string_v(&get_or(&event, "key", json!(""))).is_empty()
        {
            send_text_events.push(event);
        } else {
            other_events.push(event);
        }
    }
    if let Some(first) = send_text_events.into_iter().next() {
        other_events.push(first);
    }
    other_events
}
