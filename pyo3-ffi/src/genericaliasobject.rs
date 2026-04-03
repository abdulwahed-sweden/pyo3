#[cfg(Py_3_9)]
use crate::object::{PyObject, PyTypeObject};

extern_libpython! {
    #[cfg(Py_3_9)]
    pub fn Py_GenericAlias(origin: *mut PyObject, args: *mut PyObject) -> *mut PyObject;

    #[cfg(Py_3_9)]
    pub static mut Py_GenericAliasType: PyTypeObject;
}
