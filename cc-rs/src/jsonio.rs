use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn strip_json_comments(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut in_string = false;
    let mut escape = false;
    let mut index = 0usize;
    while index < bytes.len() {
        let ch = bytes[index];
        let next_ch = if index + 1 < bytes.len() { bytes[index + 1] } else { 0u8 };
        if in_string {
            result.push(ch);
            if escape {
                escape = false;
            } else if ch == b'\\' {
                escape = true;
            } else if ch == b'"' {
                in_string = false;
            }
            index += 1;
        } else if ch == b'"' {
            in_string = true;
            result.push(ch);
            index += 1;
        } else if ch == b'/' && next_ch == b'/' {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\r' && bytes[index] != b'\n' {
                index += 1;
            }
        } else if ch == b'/' && next_ch == b'*' {
            index += 2;
            while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                index += 1;
            }
            index += 2;
        } else {
            result.push(ch);
            index += 1;
        }
    }
    String::from_utf8_lossy(&result).into_owned()
}

pub fn decode_json(data: &[u8]) -> Result<Value, String> {
    serde_json::from_slice::<Value>(data).map_err(|e| e.to_string())
}

pub fn load_json_bytes(data: &[u8]) -> Result<Value, String> {
    match decode_json(data) {
        Ok(v) => Ok(v),
        Err(e1) => {
            let stripped = strip_json_comments(&String::from_utf8_lossy(data));
            match decode_json(stripped.as_bytes()) {
                Ok(v) => Ok(v),
                Err(_) => Err(e1),
            }
        }
    }
}

pub fn load_json_file(path: &str) -> Result<Value, String> {
    let data = fs::read(path).map_err(|e| format!("{}", e))?;
    let value = load_json_bytes(&data)?;
    if value.is_object() {
        Ok(value)
    } else {
        Err("expected JSON object at root".to_string())
    }
}

fn go_html_escape_json(data: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(data.len() + 16);
    let mut in_string = false;
    let mut i = 0usize;
    while i < data.len() {
        let b = data[i];
        if !in_string {
            if b == b'"' {
                in_string = true;
            }
            out.push(b);
            i += 1;
            continue;
        }
        match b {
            b'\\' => {
                out.push(b);
                if i + 1 < data.len() {
                    out.push(data[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            b'"' => {
                in_string = false;
                out.push(b);
                i += 1;
            }
            b'<' => {
                out.extend_from_slice(b"\\u003c");
                i += 1;
            }
            b'>' => {
                out.extend_from_slice(b"\\u003e");
                i += 1;
            }
            b'&' => {
                out.extend_from_slice(b"\\u0026");
                i += 1;
            }
            0xE2 if i + 2 < data.len() && data[i + 1] == 0x80 && (data[i + 2] == 0xA8 || data[i + 2] == 0xA9) => {
                if data[i + 2] == 0xA8 {
                    out.extend_from_slice(b"\\u2028");
                } else {
                    out.extend_from_slice(b"\\u2029");
                }
                i += 3;
            }
            _ => {
                out.push(b);
                i += 1;
            }
        }
    }
    out
}

pub fn encode_json(value: &Value, indent: &str) -> Result<Vec<u8>, String> {
    encode_json_opts(value, indent, true)
}

/// escape_html=false matches Python json.dumps (used for ZL2->FCL output);
/// true matches Go encoding/json via OrderedMap.MarshalJSON (FCL->ZL2 output).
pub fn encode_json_opts(value: &Value, indent: &str, escape_html: bool) -> Result<Vec<u8>, String> {
    let mut s = if indent.is_empty() {
        serde_json::to_string(value).map_err(|e| e.to_string())?
    } else {
        serde_json::to_string_pretty(value).map_err(|e| e.to_string())?
    };
    if !indent.is_empty() {
        s.push('\n');
    }
    if escape_html {
        Ok(go_html_escape_json(s.as_bytes()))
    } else {
        Ok(s.into_bytes())
    }
}

pub fn write_json_file(path: &str, value: &Value, compact: bool) -> Result<(), String> {
    write_json_file_opts(path, value, compact, true)
}

pub fn write_json_file_opts(
    path: &str,
    value: &Value,
    compact: bool,
    escape_html: bool,
) -> Result<(), String> {
    let data = encode_json_opts(value, if compact { "" } else { "  " }, escape_html)?;
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("{}", e))?;
        }
    }
    let mut f = fs::File::create(path).map_err(|e| format!("{}", e))?;
    f.write_all(&data).map_err(|e| format!("{}", e))
}
