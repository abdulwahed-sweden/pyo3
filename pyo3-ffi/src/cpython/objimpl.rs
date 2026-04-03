
use libc::size_t;
use std::ffi::c_int;


use std::ffi::c_void;

use crate::object::*;

// skipped _PyObject_SIZE
// skipped _PyObject_VAR_SIZE


#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyObjectArenaAllocator {
    pub ctx: *mut c_void,
    pub alloc: Option<extern "C" fn(ctx: *mut c_void, size: size_t) -> *mut c_void>,
    pub free: Option<extern "C" fn(ctx: *mut c_void, ptr: *mut c_void, size: size_t)>,
}


impl Default for PyObjectArenaAllocator {
    #[inline]
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

extern_libpython! {
    
    pub fn PyObject_GetArenaAllocator(allocator: *mut PyObjectArenaAllocator);
    
    pub fn PyObject_SetArenaAllocator(allocator: *mut PyObjectArenaAllocator);

    pub fn PyObject_IS_GC(o: *mut PyObject) -> c_int;
}

#[inline]
pub unsafe fn PyType_SUPPORTS_WEAKREFS(t: *mut PyTypeObject) -> c_int {
    ((*t).tp_weaklistoffset > 0) as c_int
}

#[inline]
pub unsafe fn PyObject_GET_WEAKREFS_LISTPTR(o: *mut PyObject) -> *mut *mut PyObject {
    let weaklistoffset = (*Py_TYPE(o)).tp_weaklistoffset;
    o.offset(weaklistoffset) as *mut *mut PyObject
}

// skipped PyUnstable_Object_GC_NewWithExtraData
