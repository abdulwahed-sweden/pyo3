use crate::object::*;
use crate::PyCodeObject;
use crate::PyFrameObject;
use crate::PyThreadState;
use std::ffi::c_int;

// skipped _PyFrame_IsRunnable
// skipped _PyFrame_IsExecuting
// skipped _PyFrameHasCompleted

extern_libpython! {
    pub fn PyFrame_New(
        tstate: *mut PyThreadState,
        code: *mut PyCodeObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
    ) -> *mut PyFrameObject;
    // skipped _PyFrame_New_NoTrack

    pub fn PyFrame_BlockSetup(f: *mut PyFrameObject, _type: c_int, handler: c_int, level: c_int);
    pub fn PyFrame_LocalsToFast(f: *mut PyFrameObject, clear: c_int);
    pub fn PyFrame_FastToLocalsWithError(f: *mut PyFrameObject) -> c_int;
    pub fn PyFrame_FastToLocals(f: *mut PyFrameObject);

    // skipped _PyFrame_DebugMallocStats

}
