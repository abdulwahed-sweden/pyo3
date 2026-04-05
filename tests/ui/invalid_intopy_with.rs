use clarax::{IntoPyObject, IntoPyObjectRef};

#[derive(IntoPyObject, IntoPyObjectRef)]
struct InvalidIntoPyWithFn {
    #[pyo3(into_py_with = into)]
    inner: String,
}

fn into(_a: String, _py: clarax::Python<'_>) -> clarax::PyResult<clarax::Bound<'_, clarax::PyAny>> {
    todo!()
}

fn main() {}
