use crate::object::*;
use crate::pyport::Py_ssize_t;
use std::ffi::{c_char, c_int};

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
pub struct PyByteArrayObject {
    pub ob_base: PyVarObject,
    pub ob_alloc: Py_ssize_t,
    pub ob_bytes: *mut c_char,
    pub ob_start: *mut c_char,
    pub ob_exports: Py_ssize_t,
    #[cfg(Py_3_15)]
    pub ob_bytes_object: *mut PyObject,
}

#[cfg(Py_LIMITED_API)]
opaque_struct!(pub PyByteArrayObject);

extern_libpython! {
    pub static mut PyByteArray_Type: PyTypeObject;

    pub static mut PyByteArrayIter_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyByteArray_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut PyByteArray_Type)
}

#[inline]
pub unsafe fn PyByteArray_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PyByteArray_Type) as c_int
}

extern_libpython! {
    pub fn PyByteArray_FromObject(o: *mut PyObject) -> *mut PyObject;
    pub fn PyByteArray_Concat(a: *mut PyObject, b: *mut PyObject) -> *mut PyObject;
    pub fn PyByteArray_FromStringAndSize(string: *const c_char, len: Py_ssize_t) -> *mut PyObject;
    pub fn PyByteArray_Size(bytearray: *mut PyObject) -> Py_ssize_t;
    pub fn PyByteArray_AsString(bytearray: *mut PyObject) -> *mut c_char;
    pub fn PyByteArray_Resize(bytearray: *mut PyObject, len: Py_ssize_t) -> c_int;
}
