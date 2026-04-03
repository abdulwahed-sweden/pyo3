use crate::object::PyObject;
use crate::pytypedefs::PyThreadState;
use std::ffi::{c_char, c_int, c_void};

extern_libpython! {
    pub fn PyEval_EvalCode(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject;

    pub fn PyEval_EvalCodeEx(
        co: *mut PyObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
        args: *const *mut PyObject,
        argc: c_int,
        kwds: *const *mut PyObject,
        kwdc: c_int,
        defs: *const *mut PyObject,
        defc: c_int,
        kwdefs: *mut PyObject,
        closure: *mut PyObject,
    ) -> *mut PyObject;

    #[cfg(not(Py_3_13))]
    #[cfg_attr(Py_3_9, deprecated(note = "Python 3.9"))]
    pub fn PyEval_CallObjectWithKeywords(
        func: *mut PyObject,
        obj: *mut PyObject,
        kwargs: *mut PyObject,
    ) -> *mut PyObject;
}

#[cfg(not(Py_3_13))]
#[cfg_attr(Py_3_9, deprecated(note = "Python 3.9"))]
#[inline]
pub unsafe fn PyEval_CallObject(func: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    #[allow(deprecated)]
    PyEval_CallObjectWithKeywords(func, arg, std::ptr::null_mut())
}

extern_libpython! {
    #[cfg(not(Py_3_13))]
    #[cfg_attr(Py_3_9, deprecated(note = "Python 3.9"))]
    pub fn PyEval_CallFunction(obj: *mut PyObject, format: *const c_char, ...) -> *mut PyObject;
    #[cfg(not(Py_3_13))]
    #[cfg_attr(Py_3_9, deprecated(note = "Python 3.9"))]
    pub fn PyEval_CallMethod(
        obj: *mut PyObject,
        methodname: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    pub fn PyEval_GetBuiltins() -> *mut PyObject;
    pub fn PyEval_GetGlobals() -> *mut PyObject;
    pub fn PyEval_GetLocals() -> *mut PyObject;
    pub fn PyEval_GetFrame() -> *mut crate::PyFrameObject;

    #[cfg(Py_3_13)]
    pub fn PyEval_GetFrameBuiltins() -> *mut PyObject;
    #[cfg(Py_3_13)]
    pub fn PyEval_GetFrameGlobals() -> *mut PyObject;
    #[cfg(Py_3_13)]
    pub fn PyEval_GetFrameLocals() -> *mut PyObject;

    pub fn Py_AddPendingCall(
        func: Option<extern "C" fn(arg1: *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn Py_MakePendingCalls() -> c_int;

    pub fn Py_SetRecursionLimit(arg1: c_int);
    pub fn Py_GetRecursionLimit() -> c_int;

    #[cfg(Py_3_9)]
    pub fn Py_EnterRecursiveCall(arg1: *const c_char) -> c_int;
    #[cfg(Py_3_9)]
    pub fn Py_LeaveRecursiveCall();

    pub fn PyEval_GetFuncName(arg1: *mut PyObject) -> *const c_char;
    pub fn PyEval_GetFuncDesc(arg1: *mut PyObject) -> *const c_char;

    pub fn PyEval_EvalFrame(arg1: *mut crate::PyFrameObject) -> *mut PyObject;
    pub fn PyEval_EvalFrameEx(f: *mut crate::PyFrameObject, exc: c_int) -> *mut PyObject;

    pub fn PyEval_SaveThread() -> *mut PyThreadState;
    pub fn PyEval_RestoreThread(arg1: *mut PyThreadState);

    #[cfg(not(Py_3_13))]
    #[cfg_attr(
        Py_3_9,
        deprecated(
            note = "Deprecated in Python 3.9, this function always returns true in Python 3.7 or newer."
        )
    )]
    pub fn PyEval_ThreadsInitialized() -> c_int;
    #[cfg_attr(
        Py_3_9,
        deprecated(
            note = "Deprecated in Python 3.9, this function does nothing in Python 3.7 or newer."
        )
    )]
    pub fn PyEval_InitThreads();
    #[cfg(not(Py_3_13))]
    #[deprecated(note = "Deprecated in Python 3.2")]
    pub fn PyEval_AcquireLock();
    #[cfg(not(Py_3_13))]
    #[deprecated(note = "Deprecated in Python 3.2")]
    pub fn PyEval_ReleaseLock();
    pub fn PyEval_AcquireThread(tstate: *mut PyThreadState);
    pub fn PyEval_ReleaseThread(tstate: *mut PyThreadState);
}

// skipped Py_BEGIN_ALLOW_THREADS
// skipped Py_BLOCK_THREADS
// skipped Py_UNBLOCK_THREADS
// skipped Py_END_ALLOW_THREADS
// skipped FVC_MASK
// skipped FVC_NONE
// skipped FVC_STR
// skipped FVC_REPR
// skipped FVC_ASCII
// skipped FVS_MASK
// skipped FVS_HAVE_SPEC
