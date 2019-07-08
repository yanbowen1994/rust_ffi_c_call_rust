#![crate_type = "staticlib"]

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern fn status() -> CString {
    let s = CString::new("o1ooo").expect("CString::new failed");
    s
}

#[no_mangle]
pub extern fn backint() -> c_int {
    let a = 1;
    a
}