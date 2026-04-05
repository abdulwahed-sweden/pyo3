use crate::object::*;
use std::ffi::c_int;

extern_libpython! {
    pub fn PyTraceBack_Here(arg1: *mut crate::PyFrameObject) -> c_int;
    pub fn PyTraceBack_Print(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
}

extern_libpython! {
    pub static mut PyTraceBack_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyTraceBack_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PyTraceBack_Type) as c_int
}
