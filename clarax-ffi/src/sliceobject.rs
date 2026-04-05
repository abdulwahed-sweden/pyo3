use crate::object::*;
use crate::pyport::Py_ssize_t;
use std::ffi::c_int;

extern_libpython! {
    static mut _Py_EllipsisObject: PyObject;
}

#[inline]
pub unsafe fn Py_Ellipsis() -> *mut PyObject {
    &raw mut _Py_EllipsisObject
}

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
pub struct PySliceObject {
    pub ob_base: PyObject,
    pub start: *mut PyObject,
    pub stop: *mut PyObject,
    pub step: *mut PyObject,
}

extern_libpython! {
    pub static mut PySlice_Type: PyTypeObject;
    pub static mut PyEllipsis_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PySlice_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PySlice_Type) as c_int
}

extern_libpython! {
    pub fn PySlice_New(
        start: *mut PyObject,
        stop: *mut PyObject,
        step: *mut PyObject,
    ) -> *mut PyObject;

    // skipped non-limited _PySlice_FromIndices
    // skipped non-limited _PySlice_GetLongIndices

    pub fn PySlice_GetIndices(
        r: *mut PyObject,
        length: Py_ssize_t,
        start: *mut Py_ssize_t,
        stop: *mut Py_ssize_t,
        step: *mut Py_ssize_t,
    ) -> c_int;
}

#[inline]
pub unsafe fn PySlice_GetIndicesEx(
    slice: *mut PyObject,
    length: Py_ssize_t,
    start: *mut Py_ssize_t,
    stop: *mut Py_ssize_t,
    step: *mut Py_ssize_t,
    slicelength: *mut Py_ssize_t,
) -> c_int {
    if PySlice_Unpack(slice, start, stop, step) < 0 {
        *slicelength = 0;
        -1
    } else {
        *slicelength = PySlice_AdjustIndices(length, start, stop, *step);
        0
    }
}

extern_libpython! {
    pub fn PySlice_Unpack(
        slice: *mut PyObject,
        start: *mut Py_ssize_t,
        stop: *mut Py_ssize_t,
        step: *mut Py_ssize_t,
    ) -> c_int;

    pub fn PySlice_AdjustIndices(
        length: Py_ssize_t,
        start: *mut Py_ssize_t,
        stop: *mut Py_ssize_t,
        step: Py_ssize_t,
    ) -> Py_ssize_t;
}
