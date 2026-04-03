use pyforge::{IntoPyObject, IntoPyObjectRef};

#[derive(IntoPyObject, IntoPyObjectRef)]
struct InvalidIntoPyWithFn {
    #[pyo3(into_py_with = into)]
    inner: String,
}

fn into(_a: String, _py: pyforge::Python<'_>) -> pyforge::PyResult<pyforge::Bound<'_, pyforge::PyAny>> {
    todo!()
}

fn main() {}
