use std::ptr;
use std::ffi::CString;

mod ffi;

use ffi::{Py_Initialize, Py_Finalize, PyRun_SimpleStringFlags};

fn main() {
    let script = CString::new("import this").unwrap();

    unsafe {
        Py_Initialize();
        PyRun_SimpleStringFlags(
            script.as_ptr(),
            ptr::null_mut()
        );
        Py_Finalize();
    }
}