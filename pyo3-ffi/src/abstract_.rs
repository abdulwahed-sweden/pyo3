use crate::object::*;
use crate::pyport::Py_ssize_t;
#[cfg(any(Py_3_12, not(Py_LIMITED_API)))]
use libc::size_t;
use std::ffi::{c_char, c_int};

#[inline]
#[cfg(not(Py_3_13))] // CPython exposed as a function in 3.13, in object.h
pub unsafe fn PyObject_DelAttrString(o: *mut PyObject, attr_name: *const c_char) -> c_int {
    PyObject_SetAttrString(o, attr_name, std::ptr::null_mut())
}

#[inline]
#[cfg(not(Py_3_13))] // CPython exposed as a function in 3.13, in object.h
pub unsafe fn PyObject_DelAttr(o: *mut PyObject, attr_name: *mut PyObject) -> c_int {
    PyObject_SetAttr(o, attr_name, std::ptr::null_mut())
}

extern_libpython! {
    #[cfg(any(Py_3_10, all(not(Py_LIMITED_API), Py_3_9)))] // Added to python in 3.9 but to limited API in 3.10
    pub fn PyObject_CallNoArgs(func: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Call(
        callable_object: *mut PyObject,
        args: *mut PyObject,
        kw: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyObject_CallObject(
        callable_object: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyObject_CallFunction(
        callable_object: *mut PyObject,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    pub fn PyObject_CallMethod(
        o: *mut PyObject,
        method: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;

    #[cfg(not(Py_3_13))]
    pub fn _PyObject_CallFunction_SizeT(
        callable_object: *mut PyObject,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    #[cfg(not(Py_3_13))]
    pub fn _PyObject_CallMethod_SizeT(
        o: *mut PyObject,
        method: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;

    pub fn PyObject_CallFunctionObjArgs(callable: *mut PyObject, ...) -> *mut PyObject;
    pub fn PyObject_CallMethodObjArgs(
        o: *mut PyObject,
        method: *mut PyObject,
        ...
    ) -> *mut PyObject;
}
#[cfg(any(Py_3_12, not(Py_LIMITED_API)))]
pub const PY_VECTORCALL_ARGUMENTS_OFFSET: size_t =
    1 << (8 * std::mem::size_of::<size_t>() as size_t - 1);

extern_libpython! {
    #[cfg(any(Py_3_12, all(Py_3_11, not(Py_LIMITED_API))))]
    pub fn PyObject_Vectorcall(
        callable: *mut PyObject,
        args: *const *mut PyObject,
        nargsf: size_t,
        kwnames: *mut PyObject,
    ) -> *mut PyObject;

    #[cfg(any(Py_3_12, all(Py_3_9, not(Py_LIMITED_API))))]
    pub fn PyObject_VectorcallMethod(
        name: *mut PyObject,
        args: *const *mut PyObject,
        nargsf: size_t,
        kwnames: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyObject_Type(o: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
pub unsafe fn PyObject_Length(o: *mut PyObject) -> Py_ssize_t {
    PyObject_Size(o)
}

extern_libpython! {
    pub fn PyObject_GetItem(o: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_SetItem(o: *mut PyObject, key: *mut PyObject, v: *mut PyObject) -> c_int;
    pub fn PyObject_DelItemString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyObject_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int;
}

extern_libpython! {
    pub fn PyObject_Format(obj: *mut PyObject, format_spec: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_GetIter(arg1: *mut PyObject) -> *mut PyObject;
}

extern_libpython! {
    pub fn PyIter_Check(obj: *mut PyObject) -> c_int;

    #[cfg(Py_3_14)]
    pub fn PyIter_NextItem(iter: *mut PyObject, item: *mut *mut PyObject) -> c_int;
    pub fn PyIter_Next(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_10)]
    pub fn PyIter_Send(
        iter: *mut PyObject,
        arg: *mut PyObject,
        presult: *mut *mut PyObject,
    ) -> PySendResult;

    pub fn PyNumber_Check(o: *mut PyObject) -> c_int;
    pub fn PyNumber_Add(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Subtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Multiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_MatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_FloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_TrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Remainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Divmod(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Power(o1: *mut PyObject, o2: *mut PyObject, o3: *mut PyObject)
        -> *mut PyObject;
    pub fn PyNumber_Negative(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Positive(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Absolute(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Invert(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Lshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Rshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_And(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Xor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Or(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
}

// Defined as this macro in Python limited API, but relies on
// non-limited PyTypeObject. Don't expose this since it cannot be used.
#[cfg(not(Py_LIMITED_API))]
#[inline]
pub unsafe fn PyIndex_Check(o: *mut PyObject) -> c_int {
    let tp_as_number = (*Py_TYPE(o)).tp_as_number;
    (!tp_as_number.is_null() && (*tp_as_number).nb_index.is_some()) as c_int
}

extern_libpython! {
    #[cfg(Py_LIMITED_API)]
    pub fn PyIndex_Check(o: *mut PyObject) -> c_int;

    pub fn PyNumber_Index(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_AsSsize_t(o: *mut PyObject, exc: *mut PyObject) -> Py_ssize_t;
    pub fn PyNumber_Long(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Float(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceAdd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceSubtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceMatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceFloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceTrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceRemainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlacePower(
        o1: *mut PyObject,
        o2: *mut PyObject,
        o3: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyNumber_InPlaceLshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceRshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceAnd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceXor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceOr(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_ToBase(n: *mut PyObject, base: c_int) -> *mut PyObject;

    pub fn PySequence_Check(o: *mut PyObject) -> c_int;
    pub fn PySequence_Size(o: *mut PyObject) -> Py_ssize_t;

}

#[inline]
pub unsafe fn PySequence_Length(o: *mut PyObject) -> Py_ssize_t {
    PySequence_Size(o)
}

extern_libpython! {
    pub fn PySequence_Concat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_Repeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_GetItem(o: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_GetSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_SetItem(o: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) -> c_int;
    pub fn PySequence_DelItem(o: *mut PyObject, i: Py_ssize_t) -> c_int;
    pub fn PySequence_SetSlice(
        o: *mut PyObject,
        i1: Py_ssize_t,
        i2: Py_ssize_t,
        v: *mut PyObject,
    ) -> c_int;
    pub fn PySequence_DelSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> c_int;
    pub fn PySequence_Tuple(o: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_List(o: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_Fast(o: *mut PyObject, m: *const c_char) -> *mut PyObject;
    // skipped PySequence_Fast_GET_SIZE
    // skipped PySequence_Fast_GET_ITEM
    // skipped PySequence_Fast_GET_ITEMS
    pub fn PySequence_Count(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    pub fn PySequence_Contains(seq: *mut PyObject, ob: *mut PyObject) -> c_int;
}

#[inline]
pub unsafe fn PySequence_In(o: *mut PyObject, value: *mut PyObject) -> c_int {
    PySequence_Contains(o, value)
}

extern_libpython! {
    pub fn PySequence_Index(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    pub fn PySequence_InPlaceConcat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_InPlaceRepeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    pub fn PyMapping_Check(o: *mut PyObject) -> c_int;
    pub fn PyMapping_Size(o: *mut PyObject) -> Py_ssize_t;

}

#[inline]
pub unsafe fn PyMapping_Length(o: *mut PyObject) -> Py_ssize_t {
    PyMapping_Size(o)
}

#[inline]
pub unsafe fn PyMapping_DelItemString(o: *mut PyObject, key: *mut c_char) -> c_int {
    PyObject_DelItemString(o, key)
}

#[inline]
pub unsafe fn PyMapping_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int {
    PyObject_DelItem(o, key)
}

extern_libpython! {
    pub fn PyMapping_HasKeyString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyMapping_HasKey(o: *mut PyObject, key: *mut PyObject) -> c_int;
    pub fn PyMapping_Keys(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_Values(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_Items(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_GetItemString(o: *mut PyObject, key: *const c_char) -> *mut PyObject;
    pub fn PyMapping_SetItemString(
        o: *mut PyObject,
        key: *const c_char,
        value: *mut PyObject,
    ) -> c_int;
    pub fn PyObject_IsInstance(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
    pub fn PyObject_IsSubclass(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
}
