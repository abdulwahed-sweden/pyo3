use crate::object::*;
use crate::PyFrameObject;
#[cfg(not(Py_3_14))]
use std::ffi::c_char;
use std::ffi::c_int;

#[cfg(not(Py_3_14))]
#[repr(C)]
pub struct PyGenObject {
    pub ob_base: PyObject,
    #[cfg(not(Py_3_12))]
    pub gi_code: *mut PyObject,
    pub gi_weakreflist: *mut PyObject,
    pub gi_name: *mut PyObject,
    pub gi_qualname: *mut PyObject,
    #[allow(
        private_interfaces,
        reason = "PyGenObject layout was public until 3.14"
    )]
    pub gi_exc_state: crate::cpython::pystate::_PyErr_StackItem,
    pub gi_origin_or_finalizer: *mut PyObject,
    pub gi_hooks_inited: c_char,
    pub gi_closed: c_char,
    pub gi_running_async: c_char,
    pub gi_frame_state: i8,
    pub gi_iframe: [*mut PyObject; 1],
}

#[cfg(Py_3_14)]
opaque_struct!(pub PyGenObject);

extern_libpython! {
    pub static mut PyGen_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyGen_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut PyGen_Type)
}

#[inline]
pub unsafe fn PyGen_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut PyGen_Type) as c_int
}

extern_libpython! {
    pub fn PyGen_New(frame: *mut PyFrameObject) -> *mut PyObject;
    // skipped PyGen_NewWithQualName
    // skipped _PyGen_SetStopIterationValue
    // skipped _PyGen_FetchStopIterationValue
    // skipped _PyGen_yf
    // skipped _PyGen_Finalize
}

// skipped PyCoroObject

extern_libpython! {
    pub static mut PyCoro_Type: PyTypeObject;
}

// skipped _PyCoroWrapper_Type

#[inline]
pub unsafe fn PyCoro_CheckExact(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut PyCoro_Type)
}

// skipped _PyCoro_GetAwaitableIter
// skipped PyCoro_New

// skipped PyAsyncGenObject

extern_libpython! {
    pub static mut PyAsyncGen_Type: PyTypeObject;
    // skipped _PyAsyncGenASend_Type
    // skipped _PyAsyncGenWrappedValue_Type
    // skipped _PyAsyncGenAThrow_Type
}

// skipped PyAsyncGen_New

#[inline]
pub unsafe fn PyAsyncGen_CheckExact(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut PyAsyncGen_Type)
}

// skipped _PyAsyncGenValueWrapperNew
