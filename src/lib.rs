extern crate libc;

mod parser;

use libc::c_char;
use parser::parser;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;

#[no_mangle]
pub extern "C" fn require(js_module: *const c_char) -> *const c_char {
    let s1 = unsafe { CStr::from_ptr(js_module) };
    let module_path = s1.to_str().unwrap();
    let contents =
        fs::read_to_string(module_path).expect("Should have been able to read the file at {}");

    let updated_contents = parser(contents);

    let c_str = CString::new(updated_contents).unwrap();

    return c_str.into_raw();
}
