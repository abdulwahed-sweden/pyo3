use crate::longobject::PyLongObject;
use crate::object::*;
use std::ffi::{c_int, c_long};

#[inline]
pub unsafe fn PyBool_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PyBool_Type) as c_int
}

extern_libpython! {
    static mut _Py_FalseStruct: PyLongObject;
    static mut _Py_TrueStruct: PyLongObject;
}

#[inline]
pub unsafe fn Py_False() -> *mut PyObject {
    (&raw mut _Py_FalseStruct).cast()
}

#[inline]
pub unsafe fn Py_True() -> *mut PyObject {
    (&raw mut _Py_TrueStruct).cast()
}

#[inline]
pub unsafe fn Py_IsTrue(x: *mut PyObject) -> c_int {
    Py_Is(x, Py_True())
}

#[inline]
pub unsafe fn Py_IsFalse(x: *mut PyObject) -> c_int {
    Py_Is(x, Py_False())
}

// skipped Py_RETURN_TRUE
// skipped Py_RETURN_FALSE

extern_libpython! {
    pub fn PyBool_FromLong(arg1: c_long) -> *mut PyObject;
}
