#![cfg(feature = "macros")]

use pyforge::exceptions::PyValueError;
use pyforge::prelude::*;
use pyforge::sync::PyOnceLock;
use pyforge::types::IntoPyDict;

#[pyclass]
struct EmptyClassWithNew {}

#[pymethods]
impl EmptyClassWithNew {
    #[new]
    fn new() -> EmptyClassWithNew {
        EmptyClassWithNew {}
    }
}

#[test]
fn empty_class_with_new() {
    Python::attach(|py| {
        let typeobj = py.get_type::<EmptyClassWithNew>();
        assert!(typeobj
            .call((), None)
            .unwrap()
            .cast::<EmptyClassWithNew>()
            .is_ok());

        // Calling with arbitrary args or kwargs is not ok
        assert!(typeobj.call(("some", "args"), None).is_err());
        assert!(typeobj
            .call((), Some(&[("some", "kwarg")].into_py_dict(py).unwrap()))
            .is_err());
    });
}

#[pyclass]
struct UnitClassWithNew;

#[pymethods]
impl UnitClassWithNew {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[test]
fn unit_class_with_new() {
    Python::attach(|py| {
        let typeobj = py.get_type::<UnitClassWithNew>();
        assert!(typeobj
            .call((), None)
            .unwrap()
            .cast::<UnitClassWithNew>()
            .is_ok());
    });
}

#[pyclass]
struct TupleClassWithNew(i32);

#[pymethods]
impl TupleClassWithNew {
    #[new]
    fn new(arg: i32) -> Self {
        Self(arg)
    }
}

#[test]
fn tuple_class_with_new() {
    Python::attach(|py| {
        let typeobj = py.get_type::<TupleClassWithNew>();
        let wrp = typeobj.call((42,), None).unwrap();
        let obj = wrp.cast::<TupleClassWithNew>().unwrap();
        let obj_ref = obj.borrow();
        assert_eq!(obj_ref.0, 42);
    });
}

#[pyclass]
#[derive(Debug)]
struct NewWithOneArg {
    data: i32,
}

#[pymethods]
impl NewWithOneArg {
    #[new]
    fn new(arg: i32) -> NewWithOneArg {
        NewWithOneArg { data: arg }
    }
}

#[test]
fn new_with_one_arg() {
    Python::attach(|py| {
        let typeobj = py.get_type::<NewWithOneArg>();
        let wrp = typeobj.call((42,), None).unwrap();
        let obj = wrp.cast::<NewWithOneArg>().unwrap();
        let obj_ref = obj.borrow();
        assert_eq!(obj_ref.data, 42);
    });
}

#[pyclass]
struct NewWithTwoArgs {
    data1: i32,
    data2: i32,
}

#[pymethods]
impl NewWithTwoArgs {
    #[new]
    fn new(arg1: i32, arg2: i32) -> Self {
        NewWithTwoArgs {
            data1: arg1,
            data2: arg2,
        }
    }
}

#[test]
fn new_with_two_args() {
    Python::attach(|py| {
        let typeobj = py.get_type::<NewWithTwoArgs>();
        let wrp = typeobj
            .call((10, 20), None)
            .map_err(|e| e.display(py))
            .unwrap();
        let obj = wrp.cast::<NewWithTwoArgs>().unwrap();
        let obj_ref = obj.borrow();
        assert_eq!(obj_ref.data1, 10);
        assert_eq!(obj_ref.data2, 20);
    });
}

#[pyclass(subclass)]
struct SuperClass {
    #[pyo3(get)]
    from_rust: bool,
}

#[pymethods]
impl SuperClass {
    #[new]
    fn new() -> Self {
        SuperClass { from_rust: true }
    }
}

/// Checks that `subclass.__new__` works correctly.
/// See https://github.com/PyO3/pyo3/issues/947 for the corresponding bug.
#[test]
fn subclass_new() {
    Python::attach(|py| {
        let super_cls = py.get_type::<SuperClass>();
        let source = pyforge_ffi::c_str!(
            r#"
class Class(SuperClass):
    def __new__(cls):
        return super().__new__(cls)  # This should return an instance of Class

    @property
    def from_rust(self):
        return False
c = Class()
assert c.from_rust is False
"#
        );
        let globals = PyModule::import(py, "__main__").unwrap().dict();
        globals.set_item("SuperClass", super_cls).unwrap();
        py.run(source, Some(&globals), None)
            .map_err(|e| e.display(py))
            .unwrap();
    });
}

#[pyclass]
#[derive(Debug)]
struct NewWithCustomError {}

struct CustomError;

impl From<CustomError> for PyErr {
    fn from(_error: CustomError) -> PyErr {
        PyValueError::new_err("custom error")
    }
}

#[pymethods]
impl NewWithCustomError {
    #[new]
    fn new() -> Result<NewWithCustomError, CustomError> {
        Err(CustomError)
    }
}

#[test]
fn new_with_custom_error() {
    Python::attach(|py| {
        let typeobj = py.get_type::<NewWithCustomError>();
        let err = typeobj.call0().unwrap_err();
        assert_eq!(err.to_string(), "ValueError: custom error");
    });
}

#[pyclass]
struct NewExisting {
    #[pyo3(get)]
    num: usize,
}

#[pymethods]
impl NewExisting {
    #[new]
    fn new(py: pyforge::Python<'_>, val: usize) -> pyforge::Py<NewExisting> {
        static PRE_BUILT: PyOnceLock<[pyforge::Py<NewExisting>; 2]> = PyOnceLock::new();
        let existing = PRE_BUILT.get_or_init(py, || {
            [
                pyforge::Py::new(py, NewExisting { num: 0 }).unwrap(),
                pyforge::Py::new(py, NewExisting { num: 1 }).unwrap(),
            ]
        });

        if val < existing.len() {
            return existing[val].clone_ref(py);
        }

        pyforge::Py::new(py, NewExisting { num: val }).unwrap()
    }
}

#[test]
fn test_new_existing() {
    Python::attach(|py| {
        let typeobj = py.get_type::<NewExisting>();

        let obj1 = typeobj.call1((0,)).unwrap();
        let obj2 = typeobj.call1((0,)).unwrap();
        let obj3 = typeobj.call1((1,)).unwrap();
        let obj4 = typeobj.call1((1,)).unwrap();
        let obj5 = typeobj.call1((2,)).unwrap();
        let obj6 = typeobj.call1((2,)).unwrap();

        assert_eq!(obj1.getattr("num").unwrap().extract::<u32>().unwrap(), 0);
        assert_eq!(obj2.getattr("num").unwrap().extract::<u32>().unwrap(), 0);
        assert_eq!(obj3.getattr("num").unwrap().extract::<u32>().unwrap(), 1);
        assert_eq!(obj4.getattr("num").unwrap().extract::<u32>().unwrap(), 1);
        assert_eq!(obj5.getattr("num").unwrap().extract::<u32>().unwrap(), 2);
        assert_eq!(obj6.getattr("num").unwrap().extract::<u32>().unwrap(), 2);

        assert!(obj1.is(&obj2));
        assert!(obj3.is(&obj4));
        assert!(!obj1.is(&obj3));
        assert!(!obj1.is(&obj5));
        assert!(!obj5.is(&obj6));
    });
}

#[pyclass]
struct NewReturnsPy;

#[pymethods]
impl NewReturnsPy {
    #[new]
    fn new(py: Python<'_>) -> PyResult<Py<NewReturnsPy>> {
        Py::new(py, NewReturnsPy)
    }
}

#[test]
fn test_new_returns_py() {
    Python::attach(|py| {
        let type_ = py.get_type::<NewReturnsPy>();
        let obj = type_.call0().unwrap();
        assert!(obj.is_exact_instance_of::<NewReturnsPy>());
    })
}

#[pyclass]
struct NewReturnsBound;

#[pymethods]
impl NewReturnsBound {
    #[new]
    fn new(py: Python<'_>) -> PyResult<Bound<'_, NewReturnsBound>> {
        Bound::new(py, NewReturnsBound)
    }
}

#[test]
fn test_new_returns_bound() {
    Python::attach(|py| {
        let type_ = py.get_type::<NewReturnsBound>();
        let obj = type_.call0().unwrap();
        assert!(obj.is_exact_instance_of::<NewReturnsBound>());
    })
}

#[pyforge::pyclass]
struct NewClassMethod {
    #[pyo3(get)]
    cls: pyforge::Py<PyAny>,
}

#[pyforge::pymethods]
impl NewClassMethod {
    #[new]
    #[classmethod]
    fn new(cls: &pyforge::Bound<'_, pyforge::types::PyType>) -> Self {
        Self {
            cls: cls.clone().into_any().unbind(),
        }
    }
}

#[test]
fn test_new_class_method() {
    pyforge::Python::attach(|py| {
        let cls = py.get_type::<NewClassMethod>();
        pyforge::py_run!(py, cls, "assert cls().cls is cls");
    });
}
