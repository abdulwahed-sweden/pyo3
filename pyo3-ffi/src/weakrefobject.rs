use crate::object::*;
use std::ffi::c_int;

#[cfg(Py_LIMITED_API)]
opaque_struct!(pub PyWeakReference);

#[cfg(not(Py_LIMITED_API))]
pub use crate::_PyWeakReference as PyWeakReference;

extern_libpython! {
    // TODO: PyO3 is depending on this symbol in `reference.rs`, we should change this and
    // remove the export as this is a private symbol.
    pub static mut _PyWeakref_RefType: PyTypeObject;
    static mut _PyWeakref_ProxyType: PyTypeObject;
    static mut _PyWeakref_CallableProxyType: PyTypeObject;
}

#[inline]
pub unsafe fn PyWeakref_CheckRef(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &raw mut _PyWeakref_RefType)
}

#[inline]
pub unsafe fn PyWeakref_CheckRefExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &raw mut _PyWeakref_RefType) as c_int
}

#[inline]
pub unsafe fn PyWeakref_CheckProxy(op: *mut PyObject) -> c_int {
    ((Py_TYPE(op) == &raw mut _PyWeakref_ProxyType)
        || (Py_TYPE(op) == &raw mut _PyWeakref_CallableProxyType)) as c_int
}

#[inline]
pub unsafe fn PyWeakref_Check(op: *mut PyObject) -> c_int {
    (PyWeakref_CheckRef(op) != 0 || PyWeakref_CheckProxy(op) != 0) as c_int
}

extern_libpython! {
    pub fn PyWeakref_NewRef(ob: *mut PyObject, callback: *mut PyObject) -> *mut PyObject;
    pub fn PyWeakref_NewProxy(ob: *mut PyObject, callback: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(
        Py_3_13,
        deprecated(note = "deprecated since Python 3.13. Use `PyWeakref_GetRef` instead.")
    )]
    pub fn PyWeakref_GetObject(reference: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_13)]
    pub fn PyWeakref_GetRef(reference: *mut PyObject, pobj: *mut *mut PyObject) -> c_int;
}
