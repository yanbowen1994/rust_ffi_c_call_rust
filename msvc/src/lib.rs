extern crate libc;

use libc::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::str;

#[no_mangle]
pub extern fn add(a : u32, b : u32) -> u32
{
    a + b
}

#[no_mangle]
pub extern fn get_string() -> *mut c_char
{
    let x = CString::new("Hello!").unwrap();
    let p = x.as_ptr();
    mem::forget(x);
    p as *mut _
}

#[no_mangle]
pub extern fn free_string(c : *mut c_char)
{
    let x = unsafe { CStr::from_ptr(c) };
    let buf: &[u8] = x.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    println!("str_slice is {}", str_slice);
}