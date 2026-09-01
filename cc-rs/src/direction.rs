use crate::constants::GLFW_TO_FCL;
use crate::context::ConversionContext;
use crate::utils::*;
use serde_json::{json, Value};

pub struct DirectionGrid {
    pub widget_x: i64,
    pub widget_y: i64,
    pub size: i64,
    pub p0: i64,
    pub p1: i64,
    pub p2: i64,
    pub screen_w: f64,
    pub screen_h: f64,
    pub reference: String,
    pub button_size: Value,
    pub child_px: f64,
}

pub fn fcl_direction_rect_to_zl_grid(
    direction: &Value,
    style: &Value,
    aspect: f64,
    joined: bool,
) -> DirectionGrid {
    let base = direction
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let button_style = style
        .get("buttonStyle")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let absolute = to_string_v(&get_or(&base, "sizeType", json!(""))) == "ABSOLUTE";

    let screen_h: f64;
    let screen_w: f64;
    let mut reference_size: f64;
    let mut reference: String;
    let view_size: i64;

    if absolute {
        screen_h = 411.0;
        screen_w = screen_h * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0));
        reference = "SCREEN_HEIGHT".to_string();
        reference_size = screen_h;
        view_size = 1.max(clamp_int(&get_or(&base, "absoluteWidth", inum(50)), 50));
    } else {
        screen_h = 10000.0;
        screen_w = screen_h * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0));
        let pw = base
            .get("percentageWidth")
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        reference = to_string_v(&get_or(&pw, "reference", json!("SCREEN_WIDTH")));
        if reference.is_empty() {
            reference = "SCREEN_WIDTH".to_string();
        }
        if reference == "SCREEN_HEIGHT" {
            reference_size = screen_h;
        } else {
            reference_size = screen_w;
        }
        view_size = 1.max((reference_size * clamp_int(&get_or(&pw, "size", inum(100)), 0) as f64 / 1000.0) as i64);
    }

    let widget_x = ((screen_w - view_size as f64)
        * clamp_int(&get_or(&base, "xPosition", inum(0)), 0) as f64
        / 1000.0) as i64;
    let widget_y = ((screen_h - view_size as f64)
        * clamp_int(&get_or(&base, "yPosition", inum(0)), 0) as f64
        / 1000.0) as i64;
    let interval = 0.max(499.min(clamp_int(&get_or(&button_style, "interval", inum(50)), 50)));
    let mut child_size = 1.max((view_size as f64 * (1000 - (2 * interval)) as f64 / 3000.0) as i64);

    let p0: i64;
    let p1: i64;
    let p2: i64;
    if joined {
        if !absolute {
            reference = "SCREEN_HEIGHT".to_string();
            reference_size = screen_h;
            child_size = child_size.max((screen_h * 1350.0 / 10000.0) as i64);
        }
        let gap = 0.max((child_size as f64 * 3.0 * interval as f64 / 1.0f64.max((1000 - (2 * interval)) as f64)) as i64);
        p0 = 0;
        p1 = child_size + gap;
        p2 = (child_size + gap) * 2;
    } else {
        p0 = 0;
        p1 = child_size + (view_size as f64 * interval as f64 / 1000.0) as i64;
        p2 = view_size - child_size;
    }

    let child_percentage =
        100.max(10000.min(py_round(child_size as f64 / reference_size * 10000.0)));
    let button_size: Value;
    if absolute {
        button_size = json!({
            "type": "dp",
            "widthDp": py_num(clamp_zl_dp(&inum(child_size), 50.0)),
            "heightDp": py_num(clamp_zl_dp(&inum(child_size), 50.0)),
            "widthPercentage": inum(child_percentage),
            "heightPercentage": inum(child_percentage),
            "widthReference": "screen_height",
            "heightReference": "screen_height",
        });
    } else {
        button_size = json!({
            "type": "percentage",
            "widthDp": py_num(50.0),
            "heightDp": py_num(50.0),
            "widthPercentage": inum(child_percentage),
            "heightPercentage": inum(child_percentage),
            "widthReference": fcl_ref_name_to_zl(&reference),
            "heightReference": fcl_ref_name_to_zl(&reference),
        });
    }
    DirectionGrid {
        widget_x,
        widget_y,
        size: child_percentage,
        p0,
        p1,
        p2,
        screen_w,
        screen_h,
        reference,
        button_size,
        child_px: child_size as f64,
    }
}

pub fn fcl_ref_name_to_zl(reference: &str) -> &'static str {
    if reference == "SCREEN_HEIGHT" {
        "screen_height"
    } else {
        "screen_width"
    }
}

pub fn pixel_to_zl_position(pixel: i64, screen: f64, child: f64) -> i64 {
    let available = 1.0f64.max(screen - child);
    0.max(10000.min(py_round(pixel as f64 / available * 10000.0)))
}

pub fn direction_event_keycodes(event: &Value, name: &str, default_keycode: i64) -> Vec<Value> {
    match event.get(name) {
        None | Some(Value::Null) => vec![inum(default_keycode)],
        Some(value) => {
            let keycodes = fcl_keycode_list(value);
            let arr = keycodes.as_array().cloned().unwrap_or_default();
            if arr.is_empty() {
                vec![inum(default_keycode)]
            } else {
                arr
            }
        }
    }
}

struct DirectionEntry {
    text: &'static str,
    dx: i64,
    dy: i64,
    keycodes: Vec<Value>,
    is_center: bool,
}

pub fn direction_to_zl_buttons(
    ctx: &mut ConversionContext,
    direction: &Value,
    style: &Value,
    style_uuid: &str,
    strict: bool,
    aspect: f64,
    joined: bool,
) -> Vec<Value> {
    let base = direction
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let event = direction
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let grid = fcl_direction_rect_to_zl_grid(direction, style, aspect, joined);

    let up_keys = direction_event_keycodes(&event, "upKeycode", GLFW_TO_FCL["GLFW_KEY_W"]);
    let down_keys = direction_event_keycodes(&event, "downKeycode", GLFW_TO_FCL["GLFW_KEY_S"]);
    let left_keys = direction_event_keycodes(&event, "leftKeycode", GLFW_TO_FCL["GLFW_KEY_A"]);
    let right_keys = direction_event_keycodes(&event, "rightKeycode", GLFW_TO_FCL["GLFW_KEY_D"]);

    let concat = |a: &Vec<Value>, b: &Vec<Value>| -> Vec<Value> {
        a.iter().chain(b.iter()).cloned().collect()
    };
    let entries = vec![
        DirectionEntry { text: "◤", dx: grid.p0, dy: grid.p0, keycodes: concat(&up_keys, &left_keys), is_center: false },
        DirectionEntry { text: "▲", dx: grid.p1, dy: grid.p0, keycodes: up_keys.clone(), is_center: false },
        DirectionEntry { text: "◥", dx: grid.p2, dy: grid.p0, keycodes: concat(&up_keys, &right_keys), is_center: false },
        DirectionEntry { text: "◀", dx: grid.p0, dy: grid.p1, keycodes: left_keys.clone(), is_center: false },
        DirectionEntry { text: "", dx: grid.p1, dy: grid.p1, keycodes: Vec::new(), is_center: true },
        DirectionEntry { text: "▶", dx: grid.p2, dy: grid.p1, keycodes: right_keys.clone(), is_center: false },
        DirectionEntry { text: "◣", dx: grid.p0, dy: grid.p2, keycodes: concat(&down_keys, &left_keys), is_center: false },
        DirectionEntry { text: "▼", dx: grid.p1, dy: grid.p2, keycodes: down_keys.clone(), is_center: false },
        DirectionEntry { text: "◢", dx: grid.p2, dy: grid.p2, keycodes: concat(&down_keys, &right_keys), is_center: false },
    ];

    let mut buttons: Vec<Value> = Vec::new();
    for e in &entries {
        let mut click_events: Vec<Value> = Vec::new();
        let mut substitutions: Vec<Value> = Vec::new();
        for kc in &e.keycodes {
            if let Some(converted) = convert_key_to_zl(
                ctx,
                clamp_int(kc, 0),
                strict,
                false,
                e.text,
                Some(&mut substitutions),
            ) {
                click_events.push(json!({
                    "type": converted.event_type,
                    "key": converted.key,
                }));
            }
        }
        if e.is_center {
            continue;
        }
        let button_obj = json!({
            "text": translatable(e.text, None),
            "uuid": format!("{}{}", short_id(), &short_id()[..6]),
            "position": {
                "x": inum(pixel_to_zl_position(grid.widget_x + e.dx, grid.screen_w, grid.child_px)),
                "y": inum(pixel_to_zl_position(grid.widget_y + e.dy, grid.screen_h, grid.child_px)),
            },
            "buttonSize": grid.button_size.clone(),
            "buttonStyle": style_uuid,
            "textAlignment": "Center",
            "textBold": false,
            "textItalic": false,
            "textUnderline": false,
            "visibilityType": visibility_fcl_to_zl(&to_string_v(&get_or(&base, "visibilityType", json!("")))),
            "clickEvents": click_events,
            "isSwipple": true,
            "isPenetrable": false,
            "isToggleable": false,
        });
        let mapping = json!({ "synthetic": true, "generatedFrom": "direction-grid" });
        let mapping = append_substitutions(Some(&mapping), &substitutions);
        let meta = make_meta(
            "fcl",
            "direction",
            &to_string_v(&get_or(direction, "id", json!(""))),
            direction,
            mapping.as_ref(),
        );
        buttons.push(set_meta(button_obj, Some(&meta)));
    }
    buttons
}

pub fn direction_view_size(base: &Value, aspect: f64) -> i64 {
    if to_string_v(&get_or(base, "sizeType", json!(""))) == "ABSOLUTE" {
        return 1.max(clamp_int(&get_or(base, "absoluteWidth", inum(50)), 50));
    }
    let screen_h = 10000.0f64;
    let screen_w = screen_h * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0));
    let pw = base
        .get("percentageWidth")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let reference = to_string_v(&get_or(&pw, "reference", json!("SCREEN_WIDTH")));
    let reference_size = if reference == "SCREEN_HEIGHT" { screen_h } else { screen_w };
    1.max((reference_size * clamp_int(&get_or(&pw, "size", inum(100)), 0) as f64 / 1000.0) as i64)
}

pub fn zl_key_events_from_keycodes(
    ctx: &mut ConversionContext,
    keycodes: &[Value],
    strict: bool,
) -> Vec<Value> {
    let mut events = Vec::new();
    for kc in keycodes {
        if let Some(converted) = convert_key_to_zl(ctx, clamp_int(kc, 0), strict, false, "", None) {
            events.push(json!({
                "type": converted.event_type,
                "key": converted.key,
            }));
        }
    }
    events
}

pub fn direction_to_zl_joystick(
    ctx: &mut ConversionContext,
    direction: &Value,
    style: &Value,
    joystick_style_uuid: &str,
    strict: bool,
    aspect: f64,
) -> Value {
    let base = direction
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let event = direction
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let grid = fcl_direction_rect_to_zl_grid(direction, style, aspect, true);
    let absolute = to_string_v(&get_or(&base, "sizeType", json!(""))) == "ABSOLUTE";
    let view_size = direction_view_size(&base, aspect);

    let up_keys = direction_event_keycodes(&event, "upKeycode", GLFW_TO_FCL["GLFW_KEY_W"]);
    let down_keys = direction_event_keycodes(&event, "downKeycode", GLFW_TO_FCL["GLFW_KEY_S"]);
    let left_keys = direction_event_keycodes(&event, "leftKeycode", GLFW_TO_FCL["GLFW_KEY_A"]);
    let right_keys = direction_event_keycodes(&event, "rightKeycode", GLFW_TO_FCL["GLFW_KEY_D"]);
    let up = zl_key_events_from_keycodes(ctx, &up_keys, strict);
    let down = zl_key_events_from_keycodes(ctx, &down_keys, strict);
    let left = zl_key_events_from_keycodes(ctx, &left_keys, strict);
    let right = zl_key_events_from_keycodes(ctx, &right_keys, strict);

    let size_type: &str;
    let size_dp: f64;
    let size_percentage: i64;
    if absolute {
        size_type = "dp";
        size_dp = clamp_zl_dp(&inum(view_size), 50.0);
        size_percentage = 2500;
    } else {
        size_type = "percentage";
        size_percentage =
            2000.max(10000.min((view_size as f64 / grid.screen_h * 10000.0).round() as i64));
        size_dp = 200.0;
    }

    let append_clone = |a: &Vec<Value>, b: &Vec<Value>| -> Vec<Value> {
        a.iter().chain(b.iter()).cloned().collect()
    };
    let joystick_obj = json!({
        "uuid": format!("{}{}", short_id(), &short_id()[..6]),
        "position": {
            "x": inum(pixel_to_zl_position(grid.widget_x, grid.screen_w, view_size as f64)),
            "y": inum(pixel_to_zl_position(grid.widget_y, grid.screen_h, view_size as f64)),
        },
        "sizeType": size_type,
        "sizeDp": py_num(size_dp),
        "sizePercentage": inum(size_percentage),
        "visibilityType": visibility_fcl_to_zl(&to_string_v(&get_or(&base, "visibilityType", json!("")))),
        "joystickStyleId": joystick_style_uuid,
        "deadZoneRatio": py_num(0.5),
        "lockThreshold": py_num(0.3),
        "canLock": true,
        "triggerMode": "drag",
        "directionEvents": {
            "north": Value::Array(up.clone()),
            "north_east": Value::Array(append_clone(&up, &right)),
            "north_west": Value::Array(append_clone(&up, &left)),
            "south": Value::Array(down.clone()),
            "south_east": Value::Array(append_clone(&down, &right)),
            "south_west": Value::Array(append_clone(&down, &left)),
            "east": Value::Array(right.clone()),
            "west": Value::Array(left.clone()),
        },
        "lockEvents": [],
    });

    let origin_id = to_string_v(&get_or(
        direction,
        "id",
        json!(to_string_v(&get_or(&joystick_obj, "uuid", json!("")))),
    ));
    let mapping = json!({ "synthetic": true, "generatedFrom": "direction-joystick" });
    let meta = make_meta("fcl", "direction", &origin_id, direction, Some(&mapping));
    set_meta(joystick_obj, Some(&meta))
}

pub fn events_to_interface(events: Vec<Value>) -> Value {
    Value::Array(events)
}
