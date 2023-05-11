use serde_json::{Map, Value};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn parse_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(val) => val.to_string(),
        Value::Number(val) => val.to_string(),
        Value::String(val) => format!("\"{}\"", val),
        Value::Array(val) => {
            let mut result = String::new();
            result.push('[');
            for item in val {
                result += &parse_value(item);
                result.push(',');
            }
            if !val.is_empty() {
                result.pop();
            }
            result.push(']');
            result
        }
        Value::Object(val) => {
            let mut result = String::new();
            result.push('{');
            for (key, value) in val {
                result.push_str(&format!("\"{}\":", key));
                result += &parse_value(value);
                result.push(',');
            }
            if !val.is_empty() {
                result.pop();
            }
            result.push('}');
            result
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_json(input: *const c_char) -> *mut c_char {
    let input_str = match CStr::from_ptr(input).to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let result = match serde_json::from_str::<Value>(input_str) {
        Ok(data) => parse_value(&data),
        Err(err) => format!("Failed to parse JSON: {}", err),
    };

    CString::new(result)
        .map(|s| s.into_raw())
        .unwrap_or_else(|_| CString::new("").unwrap().into_raw())
}
