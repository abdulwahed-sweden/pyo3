use pyforge::prelude::*;

#[pymodule(some_arg)]
fn module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

#[pyforge::pymodule(gil_used = false, gil_used = true, name = "foo", name = "bar")]
fn module_fn_multiple_errors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

#[pyforge::pymodule(gil_used = false, gil_used = true, name = "foo", name = "bar")]
mod pyo3_module_multiple_errors {}

fn main() {}
