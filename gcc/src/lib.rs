#![crate_type = "staticlib"]

use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern fn status(c_input: *mut c_char) -> CString {
    if c_input.is_null() {
    }

    unsafe {
        let input = CStr::from_ptr(c_input);
        if let Ok(x) = input.to_str() {
            println!("{:?}", x);
        }
    }
    let s = CString::new("o1ooo").expect("CString::new failed");
    s
}

#[no_mangle]
pub extern fn backint() -> c_int {
    let a = 1;
    a
}