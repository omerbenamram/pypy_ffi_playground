#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod cpyext_ffi;

#[allow(dead_code)]
mod pypy_ffi;

use std::ffi::CString;
use pypy_ffi::{rpython_startup_code, pypy_setup_home};
use cpyext_ffi::{PyRun_SimpleString, PyGILState_Ensure};
use cpyext_ffi::PyImport_ImportModule;
use pypy_ffi::pypy_carefully_make_gil;
use std::ptr;


fn main() {
    let script = CString::new("import this; import sys; print(sys.version)").unwrap();
    let this = CString::new("this").unwrap();
    unsafe {
        assert_eq!(pypy_carefully_make_gil(ptr::null_mut()), 0);
        // Crashes
        PyImport_ImportModule(this.as_ptr());
    }
}