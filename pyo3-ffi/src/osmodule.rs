use crate::object::PyObject;

extern_libpython! {
    pub fn PyOS_FSPath(path: *mut PyObject) -> *mut PyObject;
}
