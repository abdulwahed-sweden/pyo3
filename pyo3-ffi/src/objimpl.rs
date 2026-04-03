use libc::size_t;
use std::ffi::{c_int, c_void};

use crate::object::*;
use crate::pyport::Py_ssize_t;

extern_libpython! {
    pub fn PyObject_Malloc(size: size_t) -> *mut c_void;
    pub fn PyObject_Calloc(nelem: size_t, elsize: size_t) -> *mut c_void;
    pub fn PyObject_Realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void;
    pub fn PyObject_Free(ptr: *mut c_void);

    // skipped PyObject_MALLOC
    // skipped PyObject_REALLOC
    // skipped PyObject_FREE
    // skipped PyObject_Del
    // skipped PyObject_DEL

    pub fn PyObject_Init(arg1: *mut PyObject, arg2: *mut PyTypeObject) -> *mut PyObject;
    pub fn PyObject_InitVar(
        arg1: *mut PyVarObject,
        arg2: *mut PyTypeObject,
        arg3: Py_ssize_t,
    ) -> *mut PyVarObject;

    // skipped PyObject_INIT
    // skipped PyObject_INIT_VAR

    fn _PyObject_New(typeobj: *mut PyTypeObject) -> *mut PyObject;
    fn _PyObject_NewVar(typeobj: *mut PyTypeObject, n: Py_ssize_t) -> *mut PyVarObject;
}

#[inline]
pub unsafe fn PyObject_New<T>(typeobj: *mut PyTypeObject) -> *mut T {
    _PyObject_New(typeobj).cast()
}

// skipped PyObject_NEW

#[inline]
pub unsafe fn PyObject_NewVar<T>(typeobj: *mut PyTypeObject, n: Py_ssize_t) -> *mut T {
    _PyObject_NewVar(typeobj, n).cast()
}

// skipped PyObject_NEW_VAR

extern_libpython! {
    pub fn PyGC_Collect() -> Py_ssize_t;

    #[cfg(Py_3_10)]
    pub fn PyGC_Enable() -> c_int;

    #[cfg(Py_3_10)]
    pub fn PyGC_Disable() -> c_int;

    #[cfg(Py_3_10)]
    pub fn PyGC_IsEnabled() -> c_int;
}

#[inline]
pub unsafe fn PyType_IS_GC(t: *mut PyTypeObject) -> c_int {
    PyType_HasFeature(t, Py_TPFLAGS_HAVE_GC)
}

extern_libpython! {
    fn _PyObject_GC_Resize(op: *mut PyVarObject, n: Py_ssize_t) -> *mut PyVarObject;
}

#[inline]
pub unsafe fn PyObject_GC_Resize<T>(op: *mut PyObject, n: Py_ssize_t) -> *mut T {
    _PyObject_GC_Resize(op.cast(), n).cast()
}

extern_libpython! {
    fn _PyObject_GC_New(typeobj: *mut PyTypeObject) -> *mut PyObject;
    fn _PyObject_GC_NewVar(typeobj: *mut PyTypeObject, n: Py_ssize_t) -> *mut PyVarObject;

    pub fn PyObject_GC_Track(arg1: *mut c_void);

    pub fn PyObject_GC_UnTrack(arg1: *mut c_void);

    pub fn PyObject_GC_Del(arg1: *mut c_void);
}

#[inline]
pub unsafe fn PyObject_GC_New<T>(typeobj: *mut PyTypeObject) -> *mut T {
    _PyObject_GC_New(typeobj).cast()
}

#[inline]
pub unsafe fn PyObject_GC_NewVar<T>(typeobj: *mut PyTypeObject, n: Py_ssize_t) -> *mut T {
    _PyObject_GC_NewVar(typeobj, n).cast()
}

extern_libpython! {
    #[cfg(Py_3_9)]
    pub fn PyObject_GC_IsTracked(arg1: *mut PyObject) -> c_int;
    #[cfg(Py_3_9)]
    pub fn PyObject_GC_IsFinalized(arg1: *mut PyObject) -> c_int;
}

// skipped Py_VISIT
