use clarax::create_exception;
use clarax::exceptions::PyValueError;
use clarax::prelude::*;

create_exception!(pytests.exception, MyValueError, PyValueError);

#[pymodule(gil_used = false)]
pub mod exception {
    use clarax::exceptions::PyValueError;
    use clarax::prelude::*;

    #[pymodule_export]
    use super::MyValueError;

    #[pyfunction]
    fn raise_my_value_error() -> PyResult<()> {
        Err(MyValueError::new_err("error"))
    }

    #[pyfunction]
    fn return_value_error<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyValueError>> {
        Ok(PyValueError::new_err("error")
            .into_pyobject(py)?
            .cast_into()?)
    }

    #[pyfunction]
    fn return_my_value_error<'py>(py: Python<'py>) -> PyResult<Bound<'py, MyValueError>> {
        Ok(MyValueError::new_err("error")
            .into_pyobject(py)?
            .cast_into()?)
    }

    #[pyfunction]
    fn return_pyerr() -> PyErr {
        MyValueError::new_err("error")
    }
}
