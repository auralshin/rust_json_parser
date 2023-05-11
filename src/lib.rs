use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn parse_json(input: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!input.is_null());

        CStr::from_ptr(input)
    };

    let input_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let result = serde_json::from_str::<Value>(input_str);

    match result {
        Ok(data) => CString::new(format!("{:?}", data)).unwrap().into_raw(),
        Err(err) => CString::new(format!("Failed to parse JSON: {}", err))
            .unwrap()
            .into_raw(),
    }
}
