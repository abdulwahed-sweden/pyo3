use crate::{PyObject, Py_ssize_t};
use std::ffi::c_char;
use std::ffi::c_int;

use crate::{
    vectorcallfunc, PyCallable_Check, PyThreadState, PyThreadState_GET, PyTuple_Check,
    PyType_HasFeature, Py_TPFLAGS_HAVE_VECTORCALL,
};
use libc::size_t;

extern_libpython! {
    pub fn _PyStack_AsDict(values: *const *mut PyObject, kwnames: *mut PyObject) -> *mut PyObject;
}

const _PY_FASTCALL_SMALL_STACK: size_t = 5;

extern_libpython! {
    pub fn _Py_CheckFunctionResult(
        tstate: *mut PyThreadState,
        callable: *mut PyObject,
        result: *mut PyObject,
        where_: *const c_char,
    ) -> *mut PyObject;

    pub fn _PyObject_MakeTpCall(
        tstate: *mut PyThreadState,
        callable: *mut PyObject,
        args: *const *mut PyObject,
        nargs: Py_ssize_t,
        keywords: *mut PyObject,
    ) -> *mut PyObject;
}

const PY_VECTORCALL_ARGUMENTS_OFFSET: size_t =
    1 << (8 * std::mem::size_of::<size_t>() as size_t - 1);

#[inline(always)]
pub unsafe fn PyVectorcall_NARGS(n: size_t) -> Py_ssize_t {
    let n = n & !PY_VECTORCALL_ARGUMENTS_OFFSET;
    n.try_into().expect("cannot fail due to mask")
}

#[inline(always)]
pub unsafe fn PyVectorcall_Function(callable: *mut PyObject) -> Option<vectorcallfunc> {
    assert!(!callable.is_null());
    let tp = crate::Py_TYPE(callable);
    if PyType_HasFeature(tp, Py_TPFLAGS_HAVE_VECTORCALL) == 0 {
        return None;
    }
    assert!(PyCallable_Check(callable) > 0);
    let offset = (*tp).tp_vectorcall_offset;
    assert!(offset > 0);
    let ptr = callable.cast::<c_char>().offset(offset).cast();
    *ptr
}

#[inline(always)]
pub unsafe fn _PyObject_VectorcallTstate(
    tstate: *mut PyThreadState,
    callable: *mut PyObject,
    args: *const *mut PyObject,
    nargsf: size_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    assert!(kwnames.is_null() || PyTuple_Check(kwnames) > 0);
    assert!(!args.is_null() || PyVectorcall_NARGS(nargsf) == 0);

    match PyVectorcall_Function(callable) {
        None => {
            let nargs = PyVectorcall_NARGS(nargsf);
            _PyObject_MakeTpCall(tstate, callable, args, nargs, kwnames)
        }
        Some(func) => {
            let res = func(callable, args, nargsf, kwnames);
            _Py_CheckFunctionResult(tstate, callable, res, std::ptr::null_mut())
        }
    }
}

extern_libpython! {
    pub fn PyObject_VectorcallDict(
        callable: *mut PyObject,
        args: *const *mut PyObject,
        nargsf: size_t,
        kwdict: *mut PyObject,
    ) -> *mut PyObject;

    pub fn PyVectorcall_Call(
        callable: *mut PyObject,
        tuple: *mut PyObject,
        dict: *mut PyObject,
    ) -> *mut PyObject;
}

#[inline(always)]
pub unsafe fn _PyObject_FastCallTstate(
    tstate: *mut PyThreadState,
    func: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
) -> *mut PyObject {
    _PyObject_VectorcallTstate(tstate, func, args, nargs as size_t, std::ptr::null_mut())
}

#[inline(always)]
pub unsafe fn _PyObject_FastCall(
    func: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
) -> *mut PyObject {
    _PyObject_FastCallTstate(PyThreadState_GET(), func, args, nargs)
}

#[inline(always)]
pub unsafe fn _PyObject_CallNoArg(func: *mut PyObject) -> *mut PyObject {
    _PyObject_VectorcallTstate(
        PyThreadState_GET(),
        func,
        std::ptr::null_mut(),
        0,
        std::ptr::null_mut(),
    )
}

#[inline(always)]
pub unsafe fn PyObject_CallOneArg(func: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    assert!(!arg.is_null());
    let args_array = [std::ptr::null_mut(), arg];
    let args = args_array.as_ptr().offset(1); // For PY_VECTORCALL_ARGUMENTS_OFFSET
    let tstate = PyThreadState_GET();
    let nargsf = 1 | PY_VECTORCALL_ARGUMENTS_OFFSET;
    _PyObject_VectorcallTstate(tstate, func, args, nargsf, std::ptr::null_mut())
}

#[inline(always)]
pub unsafe fn PyObject_CallMethodNoArgs(
    self_: *mut PyObject,
    name: *mut PyObject,
) -> *mut PyObject {
    crate::PyObject_VectorcallMethod(
        name,
        &self_,
        1 | PY_VECTORCALL_ARGUMENTS_OFFSET,
        std::ptr::null_mut(),
    )
}

#[inline(always)]
pub unsafe fn PyObject_CallMethodOneArg(
    self_: *mut PyObject,
    name: *mut PyObject,
    arg: *mut PyObject,
) -> *mut PyObject {
    let args = [self_, arg];
    assert!(!arg.is_null());
    crate::PyObject_VectorcallMethod(
        name,
        args.as_ptr(),
        2 | PY_VECTORCALL_ARGUMENTS_OFFSET,
        std::ptr::null_mut(),
    )
}

// skipped _PyObject_VectorcallMethodId
// skipped _PyObject_CallMethodIdNoArgs
// skipped _PyObject_CallMethodIdOneArg

// skipped _PyObject_HasLen

extern_libpython! {
    pub fn PyObject_LengthHint(o: *mut PyObject, arg1: Py_ssize_t) -> Py_ssize_t;

}

// skipped PySequence_ITEM

pub const PY_ITERSEARCH_COUNT: c_int = 1;
pub const PY_ITERSEARCH_INDEX: c_int = 2;
pub const PY_ITERSEARCH_CONTAINS: c_int = 3;

extern_libpython! {
    pub fn _PySequence_IterSearch(
        seq: *mut PyObject,
        obj: *mut PyObject,
        operation: c_int,
    ) -> Py_ssize_t;
}

// skipped _PyObject_RealIsInstance
// skipped _PyObject_RealIsSubclass

// skipped _PySequence_BytesToCharpArray

// skipped _Py_FreeCharPArray

// skipped _Py_add_one_to_index_F
// skipped _Py_add_one_to_index_C

// skipped _Py_convert_optional_to_ssize_t

// skipped _PyNumber_Index(*mut PyObject o)
