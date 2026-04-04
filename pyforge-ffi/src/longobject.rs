use crate::object::*;
use crate::pyport::Py_ssize_t;
use libc::size_t;
use std::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_ulong, c_ulonglong, c_void};

opaque_struct!(pub PyLongObject);

#[inline]
pub unsafe fn PyLong_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_LONG_SUBCLASS)
}

#[inline]
pub unsafe fn PyLong_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PyLong_Type) as c_int
}

extern_libpython! {
    pub fn PyLong_FromLong(arg1: c_long) -> *mut PyObject;
    pub fn PyLong_FromUnsignedLong(arg1: c_ulong) -> *mut PyObject;
    pub fn PyLong_FromSize_t(arg1: size_t) -> *mut PyObject;
    pub fn PyLong_FromSsize_t(arg1: Py_ssize_t) -> *mut PyObject;
    pub fn PyLong_FromDouble(arg1: c_double) -> *mut PyObject;
    pub fn PyLong_AsLong(arg1: *mut PyObject) -> c_long;
    pub fn PyLong_AsLongAndOverflow(arg1: *mut PyObject, arg2: *mut c_int) -> c_long;
    pub fn PyLong_AsSsize_t(arg1: *mut PyObject) -> Py_ssize_t;
    pub fn PyLong_AsSize_t(arg1: *mut PyObject) -> size_t;
    pub fn PyLong_AsUnsignedLong(arg1: *mut PyObject) -> c_ulong;
    pub fn PyLong_AsUnsignedLongMask(arg1: *mut PyObject) -> c_ulong;
    // skipped non-limited _PyLong_AsInt
    pub fn PyLong_GetInfo() -> *mut PyObject;
    // skipped PyLong_AS_LONG

    // skipped PyLong_FromPid
    // skipped PyLong_AsPid
    // skipped _Py_PARSE_INTPTR
    // skipped _Py_PARSE_UINTPTR

    // skipped non-limited _PyLong_UnsignedShort_Converter
    // skipped non-limited _PyLong_UnsignedInt_Converter
    // skipped non-limited _PyLong_UnsignedLong_Converter
    // skipped non-limited _PyLong_UnsignedLongLong_Converter
    // skipped non-limited _PyLong_Size_t_Converter

    // skipped non-limited _PyLong_DigitValue
    // skipped non-limited _PyLong_Frexp

    pub fn PyLong_AsDouble(arg1: *mut PyObject) -> c_double;
    pub fn PyLong_FromVoidPtr(arg1: *mut c_void) -> *mut PyObject;
    pub fn PyLong_AsVoidPtr(arg1: *mut PyObject) -> *mut c_void;
    pub fn PyLong_FromLongLong(arg1: c_longlong) -> *mut PyObject;
    pub fn PyLong_FromUnsignedLongLong(arg1: c_ulonglong) -> *mut PyObject;
    pub fn PyLong_AsLongLong(arg1: *mut PyObject) -> c_longlong;
    pub fn PyLong_AsUnsignedLongLong(arg1: *mut PyObject) -> c_ulonglong;
    pub fn PyLong_AsUnsignedLongLongMask(arg1: *mut PyObject) -> c_ulonglong;
    pub fn PyLong_AsLongLongAndOverflow(arg1: *mut PyObject, arg2: *mut c_int) -> c_longlong;
    pub fn PyLong_FromString(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> *mut PyObject;
}

#[cfg(not(Py_LIMITED_API))]
extern_libpython! {
    pub fn _PyLong_NumBits(obj: *mut PyObject) -> size_t;
}

// skipped non-limited _PyLong_Format
// skipped non-limited _PyLong_FormatWriter
// skipped non-limited _PyLong_FormatBytesWriter
// skipped non-limited _PyLong_FormatAdvancedWriter

extern_libpython! {
    pub fn PyOS_strtoul(arg1: *const c_char, arg2: *mut *mut c_char, arg3: c_int) -> c_ulong;
    pub fn PyOS_strtol(arg1: *const c_char, arg2: *mut *mut c_char, arg3: c_int) -> c_long;
}

// skipped non-limited _PyLong_Rshift
// skipped non-limited _PyLong_Lshift
