use crate::object::*;
use std::ffi::{c_double, c_int};

extern_libpython! {
    pub static mut PyComplex_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyComplex_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut PyComplex_Type)
}

#[inline]
pub unsafe fn PyComplex_CheckExact(op: *mut PyObject) -> c_int {
    Py_IS_TYPE(op, &raw mut PyComplex_Type)
}

extern_libpython! {
    // skipped non-limited PyComplex_FromCComplex
    pub fn PyComplex_FromDoubles(real: c_double, imag: c_double) -> *mut PyObject;

    pub fn PyComplex_RealAsDouble(op: *mut PyObject) -> c_double;
    pub fn PyComplex_ImagAsDouble(op: *mut PyObject) -> c_double;
}
