use crate::constants::*;
use crate::context::ConversionContext;
use serde_json::{json, Map, Value};

pub fn inum(i: i64) -> Value {
    Value::Number(i.into())
}

fn number_from_str(s: String) -> serde_json::Number {
    serde_json::from_str::<Value>(&s)
        .ok()
        .and_then(|v| v.as_number().cloned())
        .unwrap_or_else(|| 0.into())
}

pub fn py_num(f: f64) -> Value {
    Value::Number(number_from_str(py_float_format(f)))
}

pub fn gof_num(f: f64) -> Value {
    Value::Number(number_from_str(go_json_float_format(f)))
}

pub fn obj() -> Value {
    Value::Object(Map::new())
}

fn ryu_parts(v: f64) -> (String, i32) {
    let mut buf = ryu::Buffer::new();
    let s = buf.format_finite(v);
    let (mant, exp_part) = match s.find('e') {
        Some(p) => (&s[..p], s[p + 1..].parse::<i32>().unwrap()),
        None => (&s[..], 0),
    };
    let dot = mant.find('.');
    let p = dot.unwrap_or(mant.len()) as i32;
    let digits_all: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let leading_zeros = digits_all.len() - digits_all.trim_start_matches('0').len();
    let e10 = p - 1 + exp_part - leading_zeros as i32;
    let digits = digits_all
        .trim_start_matches('0')
        .trim_end_matches('0')
        .to_string();
    let digits = if digits.is_empty() { "0".to_string() } else { digits };
    (digits, e10 + 1)
}

fn fmt_f_mode(out: &mut String, digits: &str, dp: i32) {
    if dp <= 0 {
        out.push_str("0.");
        for _ in 0..(-dp) {
            out.push('0');
        }
        out.push_str(digits);
    } else if dp as usize >= digits.len() {
        out.push_str(digits);
        for _ in 0..(dp as usize - digits.len()) {
            out.push('0');
        }
    } else {
        let dpi = dp as usize;
        out.push_str(&digits[..dpi]);
        out.push('.');
        out.push_str(&digits[dpi..]);
    }
}

fn fmt_e_mode(out: &mut String, digits: &str, exp: i32, pad_always: bool) {
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push('e');
    if exp < 0 {
        out.push('-');
    } else {
        out.push('+');
    }
    let ae = exp.abs();
    if ae >= 10 {
        out.push_str(&ae.to_string());
    } else if pad_always {
        out.push('0');
        out.push_str(&ae.to_string());
    } else if exp < 0 {
        out.push_str(&ae.to_string());
    } else {
        out.push('0');
        out.push_str(&ae.to_string());
    }
}

pub fn format_g_shortest(v: f64) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    let neg = v.is_sign_negative();
    let a = v.abs();
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if a == 0.0 {
        out.push('0');
        return out;
    }
    let (digits, dp) = ryu_parts(a);
    let exp = dp - 1;
    if exp < -4 || exp >= 6 {
        fmt_e_mode(&mut out, &digits, exp, true);
    } else {
        fmt_f_mode(&mut out, &digits, dp);
    }
    out
}

pub fn py_float_format(v: f64) -> String {
    if v.is_nan() || v.is_infinite() {
        return "null".to_string();
    }
    if v == v.trunc() && v.abs() < 1e16 {
        return format!("{:.1}", v);
    }
    format_g_shortest(v)
}

pub fn go_json_float_format(v: f64) -> String {
    if !v.is_finite() {
        return "0".to_string();
    }
    let mut out = String::new();
    if v.is_sign_negative() {
        out.push('-');
    }
    let abs = v.abs();
    if abs == 0.0 {
        out.push('0');
        return out;
    }
    if abs < 1e-6 || abs >= 1e21 {
        let (digits, dp) = ryu_parts(abs);
        let exp = dp - 1;
        fmt_e_mode(&mut out, &digits, exp, false);
        out
    } else {
        let (digits, dp) = ryu_parts(abs);
        fmt_f_mode(&mut out, &digits, dp);
        out
    }
}

pub fn to_float(v: &Value) -> Option<f64> {
    match v {
        Value::Null => None,
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.parse::<f64>().ok(),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}

pub fn to_int(v: &Value) -> Option<i64> {
    match v {
        Value::Null => None,
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.parse::<i64>().ok(),
        Value::Bool(b) => Some(if *b { 1 } else { 0 }),
        _ => None,
    }
}

pub fn to_int64(v: &Value) -> Option<i64> {
    to_int(v)
}

pub fn to_string_v(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}

pub fn to_bool(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => {
            let s = n.to_string();
            s != "0" && s != "0.0" && !s.is_empty()
        }
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

pub fn py_round(f: f64) -> i64 {
    if f.is_nan() || f.is_infinite() {
        return 0;
    }
    let t = f.trunc();
    if (f - t).abs() == 0.5 {
        if t % 2.0 == 0.0 {
            return t as i64;
        }
        if f >= 0.0 {
            return t as i64 + 1;
        }
        return t as i64 - 1;
    }
    f.round() as i64
}

pub fn clamp_int(value: &Value, default_val: i64) -> i64 {
    match to_float(value) {
        Some(f) => py_round(f),
        None => default_val,
    }
}

pub fn clamp_float(value: &Value, default_val: f64) -> f64 {
    match to_float(value) {
        Some(f) if f.is_finite() => f,
        _ => default_val,
    }
}

pub fn clamp_range(value: &Value, minimum: f64, maximum: f64, default_val: f64) -> f64 {
    minimum.max(maximum.min(clamp_float(value, default_val)))
}

pub fn clamp_zl_dp(value: &Value, default_val: f64) -> f64 {
    5.0f64.max(clamp_float(value, default_val))
}

pub fn clamp_zl_shape(value: &Value, default_val: f64) -> f64 {
    clamp_range(value, 0.0, 100.0, default_val)
}

pub fn clamp_zl_border_width(value: &Value, default_val: i64) -> i64 {
    0.max(50.min(clamp_int(value, default_val)))
}

pub fn scale_position_to_fcl(value: &Value) -> i64 {
    let c = clamp_int(value, 0);
    0.max(1000.min(clamp_int(&inum(c / 10), 0)))
}

pub fn scale_position_to_zl(value: &Value) -> i64 {
    let c = clamp_int(value, 0);
    0.max(10000.min(clamp_int(&inum(c * 10), 0)))
}

pub fn zl_ref_to_fcl(r: &str) -> &'static str {
    if r == "screen_height" {
        "SCREEN_HEIGHT"
    } else {
        "SCREEN_WIDTH"
    }
}

pub fn fcl_ref_to_zl(r: &str) -> &'static str {
    if r == "SCREEN_HEIGHT" {
        "screen_height"
    } else {
        "screen_width"
    }
}

pub fn visibility_zl_to_fcl(value: &str) -> &'static str {
    let v = if value.is_empty() { "always" } else { value };
    match v {
        "always" => "ALWAYS",
        "in_game" => "IN_GAME",
        "menu" | "in_menu" => "MENU",
        _ => "ALWAYS",
    }
}

pub fn visibility_fcl_to_zl(value: &str) -> &'static str {
    let v = if value.is_empty() { "ALWAYS" } else { value };
    match v {
        "ALWAYS" => "always",
        "IN_GAME" => "in_game",
        "MENU" => "in_menu",
        _ => "always",
    }
}

pub fn text_default(value: &Value) -> String {
    if let Value::Object(m) = value {
        if let Some(d) = m.get("default") {
            return to_string_v(d);
        }
        return String::new();
    }
    if value.is_null() {
        return String::new();
    }
    to_string_v(value)
}

pub fn translatable(text: &str, source: Option<&Value>) -> Value {
    if let Some(Value::Object(m)) = source {
        let default_val = m.get("default").map(|d| to_string_v(d)).unwrap_or_default();
        let d = if default_val.is_empty() { text.to_string() } else { default_val };
        if let Some(mq) = m.get("matchQueue") {
            if mq.is_array() {
                return json!({ "default": d, "matchQueue": mq.clone() });
            }
        }
        return json!({ "default": d, "matchQueue": [] });
    }
    json!({ "default": text, "matchQueue": [] })
}

pub fn signed_int32(value: i64) -> i64 {
    let value = value & 0xFFFFFFFF;
    if value >= 0x80000000 {
        value - 0x100000000
    } else {
        value
    }
}

pub fn apply_argb_alpha(color: i64, alpha_value: f64) -> i64 {
    if alpha_value >= 0.999 {
        return color;
    }
    let argb = color & 0xFFFFFFFF;
    let a = (argb >> 24) & 0xFF;
    let a = 0i64.max(255.min(py_round(a as f64 * alpha_value)));
    signed_int32((a << 24) | (argb & 0x00FFFFFF))
}

pub fn zl_color_to_fcl(color: &Value, fallback: i64, alpha: Option<&Value>) -> i64 {
    let alpha_val = match alpha {
        Some(a) => clamp_float(a, 1.0),
        None => 1.0,
    };
    if let Some(c) = to_int64(color) {
        let packed = c as u64;
        let argb = ((packed >> 32) & 0xFFFFFFFF) as i64;
        if argb != 0 || c == 0 {
            return apply_argb_alpha(signed_int32(argb), alpha_val);
        }
        if (-2147483648..=2147483647).contains(&c) {
            return apply_argb_alpha(c, alpha_val);
        }
    }
    apply_argb_alpha(fallback, alpha_val)
}

pub fn fcl_argb_to_zl_color(color: &Value, fallback: i64) -> i64 {
    let value = (clamp_int(color, fallback) as u64) & 0xFFFFFFFF;
    (value << 32) as i64
}

pub fn fcl_argb_to_zl_color_i(value: i64) -> i64 {
    ((value as u64 & 0xFFFFFFFF) << 32) as i64
}

pub fn fcl_font_to_zl(value: &Value, default_val: i64) -> i64 {
    2.max(30.min(clamp_int(value, default_val)))
}

pub fn fcl_radius_to_zl_percent(value: &Value, default_val: i64) -> i64 {
    0.max(50.min(clamp_int(value, default_val) / 10))
}

pub fn fcl_ratio_to_zl(value: &Value, default_val: i64) -> f64 {
    0.0f64.max(1.0f64.min(clamp_int(value, default_val) as f64 / 1000.0))
}

pub fn fcl_size_to_zl(value: &Value) -> i64 {
    let inner = clamp_int(value, 50);
    100.max(10000.min(clamp_int(&inum(inner * 10), 0)))
}

pub fn fcl_keycode_list(value: &Value) -> Value {
    match value {
        Value::Array(_) => value.clone(),
        Value::Null => Value::Array(Vec::new()),
        v => Value::Array(vec![v.clone()]),
    }
}

pub fn strip_converter_meta(value: &Value) -> Value {
    match value {
        Value::Object(m) => {
            let mut result = Map::new();
            for (key, v) in m {
                if key == META_KEY {
                    continue;
                }
                result.insert(key.clone(), strip_converter_meta(v));
            }
            Value::Object(result)
        }
        Value::Array(a) => Value::Array(a.iter().map(strip_converter_meta).collect()),
        _ => value.clone(),
    }
}

pub fn normalize_zl_key(event_key: &str) -> String {
    let mut key = event_key.trim().to_string();
    let upper_key = key.to_uppercase();
    if upper_key.starts_with("GLFW_") || upper_key.starts_with("MOUSE_") {
        key = upper_key;
    }
    if let Some(alias) = ZL_KEY_ALIASES.get(key.as_str()) {
        return alias.to_string();
    }
    key
}

pub struct ZlEvent {
    pub event_type: String,
    pub key: String,
}

pub fn convert_key_to_fcl(
    ctx: &mut ConversionContext,
    event_key: &str,
    strict: bool,
    mut substitutions: Option<&mut Vec<Value>>,
) -> i64 {
    let event_key = normalize_zl_key(event_key);
    if let Some(kc) = FCL_MOUSE.get(event_key.as_str()) {
        return *kc;
    }
    if let Some(kc) = GLFW_TO_FCL.get(event_key.as_str()) {
        return *kc;
    }
    if let Some(fb) = ZL_TO_FCL_FALLBACKS.get(event_key.as_str()) {
        ctx.warn(
            &format!(
                "ZL key event {:?} has no exact FCL equivalent; {}",
                event_key, fb.reason
            ),
            strict,
            false,
        );
        if let Some(subs) = substitutions.as_deref_mut() {
            subs.push(substitution(
                ctx,
                &json!({ "type": "key", "key": event_key }),
                &json!({ "type": "fcl_keycode", "keycode": fb.keycode }),
                fb.reason,
                "keys",
            ));
        }
        return fb.keycode;
    }
    if let Some(reason) = UNSUPPORTED_ZL_KEY_REASONS.get(event_key.as_str()) {
        ctx.warn(
            &format!(
                "ZL key event {:?} has no FCL control keycode equivalent: {}; substituted with UNKNOWN",
                event_key, reason
            ),
            strict,
            false,
        );
    } else {
        ctx.warn(
            &format!(
                "unsupported ZL key event {:?}; substituted with UNKNOWN",
                event_key
            ),
            strict,
            false,
        );
    }
    let fallback = GLFW_TO_FCL["GLFW_KEY_UNKNOWN"];
    if let Some(subs) = substitutions.as_deref_mut() {
        subs.push(substitution(
            ctx,
            &json!({ "type": "key", "key": event_key }),
            &json!({ "type": "fcl_keycode", "keycode": fallback }),
            "No known FCL equivalent; substituted with UNKNOWN",
            "keys",
        ));
    }
    fallback
}

pub fn convert_key_to_zl(
    ctx: &mut ConversionContext,
    keycode: i64,
    strict: bool,
    auto_click: bool,
    label: &str,
    mut substitutions: Option<&mut Vec<Value>>,
) -> Option<ZlEvent> {
    if keycode == -1 && label.trim() == "*" {
        return Some(ZlEvent {
            event_type: "key".to_string(),
            key: "GLFW_KEY_KP_MULTIPLY".to_string(),
        });
    }
    if let Some(key) = FCL_MOUSE_REVERSE.get(&keycode) {
        return Some(ZlEvent {
            event_type: "key".to_string(),
            key: key.to_string(),
        });
    }
    if let Some((single_event, long_event)) = FCL_SCROLL_REVERSE.get(&keycode) {
        let ev = if auto_click { long_event } else { single_event };
        return Some(ZlEvent {
            event_type: "launcher_event".to_string(),
            key: ev.to_string(),
        });
    }
    if let Some(key) = FCL_TO_GLFW.get(&keycode) {
        return Some(ZlEvent {
            event_type: "key".to_string(),
            key: key.to_string(),
        });
    }
    if let Some(fb) = FCL_TO_ZL_FALLBACKS.get(&keycode) {
        ctx.warn(
            &format!(
                "FCL keycode {} has no exact ZL equivalent; {}",
                keycode, fb.reason
            ),
            strict,
            false,
        );
        if let Some(subs) = substitutions.as_deref_mut() {
            subs.push(substitution(
                ctx,
                &json!({ "type": "fcl_keycode", "keycode": keycode }),
                &json!({ "type": fb.event_type, "key": fb.key }),
                fb.reason,
                "keys",
            ));
        }
        return Some(ZlEvent {
            event_type: fb.event_type.to_string(),
            key: fb.key.to_string(),
        });
    }
    if let Some(reason) = UNSUPPORTED_FCL_KEY_REASONS.get(&keycode) {
        ctx.warn(
            &format!(
                "FCL keycode {} has no ZL control event equivalent: {}; substituted with GLFW_KEY_UNKNOWN",
                keycode, reason
            ),
            strict,
            false,
        );
    } else {
        ctx.warn(
            &format!(
                "unsupported FCL keycode {}; substituted with GLFW_KEY_UNKNOWN",
                keycode
            ),
            strict,
            false,
        );
    }
    if let Some(subs) = substitutions.as_deref_mut() {
        subs.push(substitution(
            ctx,
            &json!({ "type": "fcl_keycode", "keycode": keycode }),
            &json!({ "type": "key", "key": "GLFW_KEY_UNKNOWN" }),
            "No known ZL equivalent; substituted with GLFW_KEY_UNKNOWN",
            "keys",
        ));
    }
    Some(ZlEvent {
        event_type: "key".to_string(),
        key: "GLFW_KEY_UNKNOWN".to_string(),
    })
}

pub fn substitution(
    ctx: &mut ConversionContext,
    source: &Value,
    target: &Value,
    reason: &str,
    category: &str,
) -> Value {
    ctx.bump(category);
    json!({
        "source": source.clone(),
        "target": target.clone(),
        "reason": reason,
    })
}

pub fn append_substitutions(mapping: Option<&Value>, substitutions: &[Value]) -> Option<Value> {
    if substitutions.is_empty() {
        return mapping.cloned();
    }
    let mut result = mapping.cloned().unwrap_or_else(obj);
    if !result.is_object() {
        result = obj();
    }
    let mut existing: Vec<Value> = result
        .get("substitutions")
        .and_then(|e| e.as_array().cloned())
        .unwrap_or_default();
    for s in substitutions {
        existing.push(s.clone());
    }
    if let Value::Object(m) = &mut result {
        m.insert("substitutions".to_string(), Value::Array(existing));
    }
    Some(result)
}

pub fn make_meta(
    origin_format: &str,
    origin_kind: &str,
    origin_id: &str,
    original: &Value,
    mapping: Option<&Value>,
) -> Value {
    let original_copy = strip_converter_meta(original);
    let mut meta = json!({
        "schema": META_SCHEMA_VERSION,
        "originFormat": origin_format,
        "originKind": origin_kind,
        "originId": origin_id,
        "original": original_copy,
    });
    if let Some(mp) = mapping {
        if let Value::Object(m) = &mut meta {
            m.insert("mapping".to_string(), mp.clone());
        }
    }
    meta
}

pub fn set_meta(mut target: Value, meta: Option<&Value>) -> Value {
    if let Some(m) = meta {
        if m.as_object().map_or(false, |o| !o.is_empty()) {
            if let Value::Object(t) = &mut target {
                t.insert(META_KEY.to_string(), m.clone());
            }
        }
    }
    target
}

pub fn get_meta(v: &Value) -> Option<&Value> {
    v.get(META_KEY).filter(|m| m.is_object())
}

pub fn meta_original(v: &Value, expected_format: &str, expected_kind: Option<&str>) -> Option<Value> {
    let meta = get_meta(v)?;
    let of = meta.get("originFormat")?;
    if to_string_v(of) != expected_format {
        return None;
    }
    if let Some(kind) = expected_kind {
        if !kind.is_empty() {
            let ok_val = meta.get("originKind")?;
            if to_string_v(ok_val) != kind {
                return None;
            }
        }
    }
    let original = meta.get("original")?;
    if !original.is_object() {
        return None;
    }
    Some(original.clone())
}

pub fn meta_kind(v: &Value) -> String {
    match get_meta(v).and_then(|m| m.get("originKind")) {
        Some(k) => to_string_v(k),
        None => String::new(),
    }
}

fn random_bytes() -> [u8; 16] {
    if std::env::var_os("CC_DETERMINISTIC").is_some() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let c = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        let mut b = [0u8; 16];
        b[..8].copy_from_slice(&c.to_be_bytes());
        for x in b[8..].iter_mut() {
            *x = 0xAB;
        }
        return b;
    }
    let mut b = [0u8; 16];
    getrandom::fill(&mut b).expect("getrandom failed");
    b
}

pub fn short_id() -> String {
    let b = random_bytes();
    b.iter().map(|x| format!("{:02x}", x)).collect::<String>()[..12].to_string()
}

pub fn fcl_id() -> String {
    let mut b = random_bytes();
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let hex: Vec<String> = b.iter().map(|x| format!("{:02x}", x)).collect();
    format!(
        "{}-{}-{}-{}-{}",
        hex[0..4].join(""),
        hex[4..6].join(""),
        hex[6..8].join(""),
        hex[8..10].join(""),
        hex[10..16].join("")
    )
}

pub fn estimate_wrap_content_dp(widget: &Value, style_name: &str, fcl_styles: &Value) -> (i64, i64) {
    let text = text_default(&get_or_empty(widget, "text"));
    let mut style: Option<&Value> = None;
    if let Some(items) = fcl_styles.as_array() {
        for item in items {
            if let Value::Object(_) = item {
                let name = item.get("name").map(to_string_v).unwrap_or_default();
                if name == style_name {
                    style = Some(item);
                    break;
                }
            }
        }
    }
    let fallback;
    let style = match style {
        Some(s) => s,
        None => {
            fallback = crate::styles::default_zl_fallback_fcl_style(None);
            &fallback
        }
    };
    let font_size = 2.max(clamp_int(&get_or(style, "textSize", inum(14)), 14));
    let lines: Vec<&str> = text.split('\n').collect();
    let mut longest = 0usize;
    for line in &lines {
        let n = line.chars().count();
        if n > longest {
            longest = n;
        }
    }
    let width = 5.max(480.min(py_round(longest as f64 * font_size as f64 * 0.62 + 8.0)));
    let height = 5.max(240.min(py_round(lines.len() as f64 * font_size as f64 * 1.25 + 6.0)));
    (width, height)
}

pub fn get_or(m: &Value, key: &str, default_val: Value) -> Value {
    match m.get(key) {
        Some(v) => v.clone(),
        None => default_val,
    }
}

pub fn get_or_empty(m: &Value, key: &str) -> Value {
    m.get(key).cloned().unwrap_or(Value::Null)
}

pub fn get_or_obj_ref<'a>(m: &'a Value, key: &str) -> Option<&'a Value> {
    m.get(key).filter(|v| v.is_object())
}

pub fn get_or_list<'a>(m: &'a Value, key: &str) -> &'a [Value] {
    match m.get(key) {
        Some(Value::Array(a)) => a,
        _ => &[],
    }
}

pub fn get_or_string(m: &Value, key: &str, default_val: &str) -> String {
    match m.get(key) {
        Some(Value::Null) | None => default_val.to_string(),
        Some(v) => to_string_v(v),
    }
}

pub fn as_object(v: &Value) -> Option<&Map<String, Value>> {
    v.as_object()
}

pub fn as_list(v: &Value) -> Option<&Vec<Value>> {
    v.as_array()
}

fn is_alnum_ascii(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_cjk(c: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&c)
}

pub fn normalized_control_text(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if is_alnum_ascii(c) || is_cjk(c) {
            out.extend(c.to_lowercase());
        }
    }
    out
}

fn control_text_runs(text: &str) -> Vec<String> {
    let mut runs = Vec::new();
    let mut current = String::new();
    let mut current_class: Option<bool> = None;
    for c in text.chars() {
        let class = if is_alnum_ascii(c) { Some(false) } else if is_cjk(c) { Some(true) } else { None };
        match (class, current_class) {
            (Some(cls), Some(prev)) if cls == prev => {
                current.extend(c.to_lowercase());
            }
            (Some(cls), _) => {
                if !current.is_empty() {
                    runs.push(std::mem::take(&mut current));
                }
                current_class = Some(cls);
                current.extend(c.to_lowercase());
            }
            (None, _) => {
                if !current.is_empty() {
                    runs.push(std::mem::take(&mut current));
                }
                current_class = None;
            }
        }
    }
    if !current.is_empty() {
        runs.push(current);
    }
    runs
}

pub fn normalized_control_words(text: &str) -> std::collections::HashSet<String> {
    let mut words = std::collections::HashSet::new();
    for raw in control_text_runs(text) {
        let word = raw;
        if word.chars().count() < 2 {
            continue;
        }
        words.insert(word.clone());
        let cjk_full = !word.is_empty() && word.chars().all(is_cjk);
        if cjk_full {
            let runes: Vec<char> = word.chars().collect();
            let max_size = 5.min(runes.len());
            for size in 2..=max_size {
                for start in 0..=(runes.len() - size) {
                    words.insert(runes[start..start + size].iter().collect());
                }
            }
        }
    }
    words
}

pub fn dedupe_events(events: Vec<Value>) -> Vec<Value> {
    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for event in events {
        let event_type = to_string_v(&event.get("type").cloned().unwrap_or(Value::Null));
        let key = to_string_v(&event.get("key").cloned().unwrap_or(Value::Null));
        let k = format!("{}\x00{}", event_type, key);
        if seen.insert(k) {
            result.push(event);
        }
    }
    result
}

pub struct GroupIdsByName {
    order: Vec<(String, String)>,
    map: std::collections::HashMap<String, String>,
}

impl GroupIdsByName {
    pub fn new() -> Self {
        GroupIdsByName {
            order: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, id: String) {
        if !self.map.contains_key(&name) {
            self.order.push((name.clone(), id.clone()));
        }
        self.map.insert(name, id);
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.order.iter().map(|(n, i)| (n, i))
    }
}

impl Default for GroupIdsByName {
    fn default() -> Self {
        Self::new()
    }
}
