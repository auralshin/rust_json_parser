use serde_json::{from_str, to_string, Value};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn parse_json_string(json_str: &str) -> Result<String, String> {
    let parsed_json = from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let result = to_string(&parsed_json).map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    Ok(result)
}

#[no_mangle]
pub unsafe extern "C" fn parse_json(input: *const c_char) -> *mut c_char {
    let input_str = match CStr::from_ptr(input).to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    match parse_json_string(input_str) {
        Ok(result) => CString::new(result)
            .map(|s| s.into_raw())
            .unwrap_or_else(|_| CString::new("").unwrap().into_raw()),
        Err(err) => CString::new(err)
            .map(|s| s.into_raw())
            .unwrap_or_else(|_| CString::new("").unwrap().into_raw()),
    }
}
