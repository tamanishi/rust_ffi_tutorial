extern crate libc;
use libc::{c_char, size_t};
use std::ffi::CStr;

#[link(name = "print", kind = "static")]
extern "C" {
    fn print_str(str: *const c_char, len: size_t) -> *const c_char;
}

fn main() {
    let str: &str = "Hello world!";
    let ret_ptr: *const c_char = unsafe { print_str(str.as_ptr() as *const c_char, str.len()) };
    let ret_str: &CStr = unsafe { CStr::from_ptr(ret_ptr) };
    println!("{}", ret_str.to_str().unwrap());
}
