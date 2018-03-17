#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod cpyext_ffi;

#[allow(dead_code)]
mod pypy_ffi;

use std::ffi::CString;
use pypy_ffi::{rpython_startup_code, pypy_setup_home};
use cpyext_ffi::PyRun_SimpleString;


fn main() {
    let script = CString::new("import this; import sys; print(sys.version)").unwrap();

    unsafe {
        rpython_startup_code();
        pypy_setup_home(0, 1);

        // cpyext code!
        PyRun_SimpleString(script.as_ptr());
    }
}