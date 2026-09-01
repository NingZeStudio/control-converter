use crate::utils::*;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

pub fn default_fcl_style(name: Option<&str>) -> Value {
    let style_name = name.unwrap_or("Default");
    json!({
        "name": style_name,
        "textColor": -1,
        "textSize": 12,
        "strokeColor": -12303292,
        "strokeWidth": 10,
        "cornerRadius": 100,
        "fillColor": 0,
        "textColorPressed": -1,
        "textSizePressed": 12,
        "strokeColorPressed": -12303292,
        "strokeWidthPressed": 10,
        "cornerRadiusPressed": 100,
        "fillColorPressed": -3355444,
    })
}

pub fn default_zl_fallback_fcl_style(name: Option<&str>) -> Value {
    let style_name = name.unwrap_or("ZL Native Default");
    json!({
        "name": style_name,
        "textColor": -1,
        "textSize": 14,
        "strokeColor": -1,
        "strokeWidth": 0,
        "cornerRadius": 0,
        "fillColor": -2147483648,
        "textColorPressed": -1,
        "textSizePressed": 14,
        "strokeColorPressed": -1,
        "strokeWidthPressed": 0,
        "cornerRadiusPressed": 0,
        "fillColorPressed": -1282897784,
    })
}

pub fn default_fcl_direction_style() -> Value {
    json!({
        "name": "Default",
        "styleType": "BUTTON",
        "buttonStyle": {
            "interval": 50,
            "textColor": -1,
            "textSize": 12,
            "strokeColor": -12303292,
            "strokeWidth": 10,
            "cornerRadius": 100,
            "fillColor": 0,
            "textColorPressed": -1,
            "textSizePressed": 12,
            "strokeColorPressed": -12303292,
            "strokeWidthPressed": 10,
            "cornerRadiusPressed": 100,
            "fillColorPressed": -3355444,
        },
        "rockerStyle": {
            "rockerSize": 400,
            "bgCornerRadius": 500,
            "bgStrokeWidth": 20,
            "bgStrokeColor": -12303292,
            "bgFillColor": 0,
            "rockerCornerRadius": 500,
            "rockerStrokeWidth": 10,
            "rockerStrokeColor": -12303292,
            "rockerFillColor": -7829368,
        },
    })
}

pub fn empty_fcl_event() -> Value {
    json!({
        "autoKeep": false,
        "autoClick": false,
        "openMenu": false,
        "switchTouchMode": false,
        "switchMouseMode": false,
        "input": false,
        "quickInput": false,
        "outputText": "",
        "outputKeycodes": [],
        "bindViewGroup": [],
    })
}

pub fn fcl_button_event() -> Value {
    json!({
        "pointerFollow": false,
        "Movable": false,
        "pressEvent": empty_fcl_event(),
        "longPressEvent": empty_fcl_event(),
        "clickEvent": empty_fcl_event(),
        "doubleClickEvent": empty_fcl_event(),
    })
}

pub fn zl_shape_to_fcl_radius(shape: &Value) -> i64 {
    let Some(s) = as_object(shape) else {
        return 100;
    };
    let keys = ["topStart", "topEnd", "bottomEnd", "bottomStart"];
    let mut sum = 0.0;
    for k in keys {
        let v = s.get(k).cloned().unwrap_or(Value::from(0));
        sum += clamp_zl_shape(&v, 0.0);
    }
    0.max(500.min(py_round(sum / keys.len() as f64 * 10.0)))
}

pub fn default_zl_joystick_style_config() -> Value {
    json!({
        "alpha": py_num(1.0),
        "backgroundColor": fcl_argb_to_zl_color_i(0x80000000),
        "joystickColor": fcl_argb_to_zl_color_i(0x80FFFFFF),
        "joystickCanLockColor": fcl_argb_to_zl_color_i(0x80FFFF00),
        "joystickLockedColor": fcl_argb_to_zl_color_i(0x8000FF00),
        "lockMarkColor": fcl_argb_to_zl_color_i(0xFFFFFFFF),
        "borderWidthRatio": 0,
        "borderColor": fcl_argb_to_zl_color_i(0xFFFFFFFF),
        "backgroundShape": 50,
        "joystickShape": 50,
        "joystickSize": py_num(0.5),
    })
}

fn set_config_from(
    config: &mut Value,
    src: &Value,
    fill_key: &str,
    fill_def: i64,
    text_key: &str,
    text_def: i64,
    stroke_key: &str,
    stroke_def: i64,
    stroke_width_key: &str,
    stroke_width_def: i64,
    corner_key: &str,
    corner_def: i64,
    joystick_corner_key: &str,
    joystick_corner_def: i64,
    joystick_size: f64,
) {
    if let Value::Object(c) = config {
        c.insert(
            "backgroundColor".to_string(),
            inum(fcl_argb_to_zl_color(&get_or(src, fill_key, inum(fill_def)), 0)),
        );
        c.insert(
            "joystickColor".to_string(),
            inum(fcl_argb_to_zl_color(&get_or(src, text_key, inum(text_def)), 0)),
        );
        c.insert(
            "borderColor".to_string(),
            inum(fcl_argb_to_zl_color(&get_or(src, stroke_key, inum(stroke_def)), 0)),
        );
        let w = clamp_int(&get_or(src, stroke_width_key, inum(stroke_width_def)), 0);
        c.insert("borderWidthRatio".to_string(), inum(0.max(50.min(w / 10))));
        c.insert(
            "backgroundShape".to_string(),
            inum(fcl_radius_to_zl_percent(&get_or(src, corner_key, inum(corner_def)), 500)),
        );
        c.insert(
            "joystickShape".to_string(),
            inum(fcl_radius_to_zl_percent(&get_or(src, joystick_corner_key, inum(joystick_corner_def)), 500)),
        );
        c.insert("joystickSize".to_string(), py_num(joystick_size));
    }
}

pub fn fcl_rocker_style_to_zl_joystick(style: Option<&Value>) -> Value {
    let mut rocker = obj();
    let mut name = "Default".to_string();
    if let Some(s) = style {
        name = to_string_v(&get_or(s, "name", json!("Default")));
        if let Some(r) = s.get("rockerStyle") {
            if let Value::Object(rm) = r {
                if !rm.is_empty() {
                    rocker = r.clone();
                }
            }
        }
    }
    let mut config = default_zl_joystick_style_config();
    set_config_from(
        &mut config,
        &rocker,
        "bgFillColor",
        0x80000000,
        "rockerFillColor",
        0x80FFFFFF,
        "bgStrokeColor",
        0xFFFFFFFF,
        "bgStrokeWidth",
        0,
        "bgCornerRadius",
        500,
        "rockerCornerRadius",
        500,
        fcl_ratio_to_zl(&get_or(&rocker, "rockerSize", inum(500)), 500),
    );
    json!({
        "name": name,
        "uuid": short_id(),
        "commonStyle": true,
        "lightStyle": config,
        "darkStyle": config.clone(),
    })
}

pub fn fcl_button_style_to_zl_joystick(style: Option<&Value>) -> Value {
    let mut btn = obj();
    let mut name = "Default".to_string();
    if let Some(s) = style {
        name = to_string_v(&get_or(s, "name", json!("Default")));
        if let Some(b) = s.get("buttonStyle") {
            if let Value::Object(bm) = b {
                if !bm.is_empty() {
                    btn = b.clone();
                }
            }
        }
    }
    let mut config = default_zl_joystick_style_config();
    set_config_from(
        &mut config,
        &btn,
        "fillColor",
        0x80000000,
        "textColor",
        0x80FFFFFF,
        "strokeColor",
        0xFFFFFFFF,
        "strokeWidth",
        10,
        "cornerRadius",
        100,
        "cornerRadius",
        100,
        0.5,
    );
    json!({
        "name": name,
        "uuid": short_id(),
        "commonStyle": true,
        "lightStyle": config,
        "darkStyle": config.clone(),
    })
}

pub fn direction_style_map<'a>(styles: &'a [Value]) -> HashMap<String, &'a Value> {
    let mut result = HashMap::new();
    for item in styles {
        if !item.is_object() {
            continue;
        }
        let name = to_string_v(&get_or(item, "name", json!("")));
        result.insert(name, item);
    }
    result
}

pub fn resolve_direction_style(
    direction: &Value,
    styles: &HashMap<String, &Value>,
) -> Value {
    match direction.get("style") {
        Some(style) => {
            if style.is_object() {
                return style.clone();
            }
            let name = to_string_v(style);
            styles.get(&name).map(|s| (*s).clone()).unwrap_or_else(obj)
        }
        None => styles
            .get("")
            .map(|s| (*s).clone())
            .unwrap_or_else(obj),
    }
}

pub fn style_name_for_zl_style(base_name: &str, uuid_value: &str) -> String {
    let suffix = if !uuid_value.is_empty() {
        if uuid_value.len() >= 6 {
            uuid_value[..6].to_string()
        } else {
            uuid_value.to_string()
        }
    } else {
        let s = short_id();
        if s.len() >= 6 {
            s[..6].to_string()
        } else {
            s
        }
    };
    format!("ZL {} {}", base_name, suffix)
}

pub fn fcl_styles_to_zl(styles: &[Value]) -> (Vec<Value>, HashMap<String, String>) {
    let default_style;
    let styles: &[Value] = if styles.is_empty() {
        default_style = vec![default_fcl_style(None)];
        &default_style
    } else {
        styles
    };
    let mut result = Vec::new();
    let mut mapping = HashMap::new();
    for item in styles {
        if !item.is_object() {
            continue;
        }
        let style = item;
        let mut name = to_string_v(&get_or(style, "name", json!("Default")));
        if name.is_empty() {
            name = "Default".to_string();
        }
        let sid = short_id();
        mapping.insert(name.clone(), sid.clone());
        let radius =
            clamp_zl_shape(&gof_num(clamp_float(&get_or(style, "cornerRadius", inum(0)), 0.0) / 10.0), 0.0);
        let pressed_src = get_or(style, "cornerRadiusPressed", get_or(style, "cornerRadius", inum(0)));
        let pressed_radius = clamp_zl_shape(&gof_num(clamp_float(&pressed_src, 0.0) / 10.0), 0.0);
        let stroke_width = clamp_int(&get_or(style, "strokeWidth", inum(10)), 0);
        let stroke_width_pressed = clamp_int(&get_or(style, "strokeWidthPressed", inum(10)), 0);
        let text_size = get_or(style, "textSize", inum(12));
        let radius_obj = {
            let mut m = Map::new();
            for k in ["topStart", "topEnd", "bottomEnd", "bottomStart"] {
                m.insert(k.to_string(), py_num(radius as f64));
            }
            Value::Object(m)
        };
        let pressed_radius_obj = {
            let mut m = Map::new();
            for k in ["topStart", "topEnd", "bottomEnd", "bottomStart"] {
                m.insert(k.to_string(), py_num(pressed_radius as f64));
            }
            Value::Object(m)
        };
        let light = json!({
            "alpha": py_num(1.0),
            "pressedAlpha": py_num(1.0),
            "backgroundColor": inum(fcl_argb_to_zl_color(&get_or(style, "fillColor", inum(0)), 0)),
            "pressedBackgroundColor": inum(fcl_argb_to_zl_color(&get_or(style, "fillColorPressed", inum(-3355444)), 0)),
            "contentColor": inum(fcl_argb_to_zl_color(&get_or(style, "textColor", inum(-1)), -1)),
            "pressedContentColor": inum(fcl_argb_to_zl_color(&get_or(style, "textColorPressed", inum(-1)), -1)),
            "fontSize": inum(fcl_font_to_zl(&text_size, 12)),
            "pressedFontSize": inum(fcl_font_to_zl(&get_or(style, "textSizePressed", text_size.clone()), 12)),
            "borderWidth": inum(clamp_zl_border_width(&inum(stroke_width / 10), 0)),
            "pressedBorderWidth": inum(clamp_zl_border_width(&inum(stroke_width_pressed / 10), 0)),
            "borderColor": inum(fcl_argb_to_zl_color(&get_or(style, "strokeColor", inum(-12303292)), -12303292)),
            "pressedBorderColor": inum(fcl_argb_to_zl_color(&get_or(style, "strokeColorPressed", inum(-12303292)), -12303292)),
            "borderRadius": radius_obj,
            "pressedBorderRadius": pressed_radius_obj,
        });
        result.push(json!({
            "name": name,
            "uuid": sid,
            "animateSwap": false,
            "commonStyle": true,
            "lightStyle": light,
            "darkStyle": light.clone(),
        }));
    }
    (result, mapping)
}

pub fn make_zl_button_size(base_info: &Value, absolute_as_percentage: bool, aspect: f64) -> Value {
    if to_string_v(&get_or(base_info, "sizeType", json!(""))) == "ABSOLUTE" && absolute_as_percentage {
        let screen_height_dp = 411.0f64;
        let screen_width_dp = screen_height_dp * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0));
        let width_dp = clamp_zl_dp(&get_or(base_info, "absoluteWidth", inum(50)), 50.0);
        let height_dp = clamp_zl_dp(&get_or(base_info, "absoluteHeight", inum(50)), 50.0);
        let width_percentage =
            100.max(10000.min(py_round(width_dp / screen_width_dp * 10000.0)));
        let height_percentage =
            100.max(10000.min(py_round(height_dp / screen_height_dp * 10000.0)));
        return json!({
            "type": "percentage",
            "widthDp": py_num(width_dp),
            "heightDp": py_num(height_dp),
            "widthPercentage": inum(width_percentage),
            "heightPercentage": inum(height_percentage),
            "widthReference": "screen_width",
            "heightReference": "screen_height",
        });
    }
    let mut size_type = "percentage";
    if to_string_v(&get_or(base_info, "sizeType", json!(""))) == "ABSOLUTE" {
        size_type = "dp";
    }
    let pw = get_or_obj_ref(base_info, "percentageWidth");
    let ph = get_or_obj_ref(base_info, "percentageHeight");
    let empty = obj();
    let pw = pw.unwrap_or(&empty);
    let ph = ph.unwrap_or(&empty);
    json!({
        "type": size_type,
        "widthDp": py_num(clamp_zl_dp(&get_or(base_info, "absoluteWidth", inum(50)), 50.0)),
        "heightDp": py_num(clamp_zl_dp(&get_or(base_info, "absoluteHeight", inum(50)), 50.0)),
        "widthPercentage": inum(fcl_size_to_zl(&get_or(pw, "size", inum(50)))),
        "heightPercentage": inum(fcl_size_to_zl(&get_or(ph, "size", inum(50)))),
        "widthReference": fcl_ref_to_zl(&to_string_v(&get_or(pw, "reference", json!("")))),
        "heightReference": fcl_ref_to_zl(&to_string_v(&get_or(ph, "reference", json!("")))),
    })
}
