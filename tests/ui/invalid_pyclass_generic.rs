use pyforge::prelude::*;
use pyforge::types::PyType;

#[pyclass(generic)]
struct ClassRedefinesClassGetItem {}

#[pymethods]
impl ClassRedefinesClassGetItem {
    #[new]
    fn new() -> ClassRedefinesClassGetItem {
        Self {}
    }

    #[classmethod]
    pub fn __class_getitem__(
        cls: &Bound<'_, PyType>,
        key: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        pyforge::types::PyGenericAlias::new(cls.py(), cls.as_any(), key)
    }
}

fn main() {}
