use libc::size_t;
use std::ffi::c_void;

extern_libpython! {
    pub fn PyMem_Malloc(size: size_t) -> *mut c_void;
    pub fn PyMem_Calloc(nelem: size_t, elsize: size_t) -> *mut c_void;
    pub fn PyMem_Realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void;
    pub fn PyMem_Free(ptr: *mut c_void);
}
