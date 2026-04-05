use clarax::prelude::*;

#[pyfunction]
#[pyo3(warn)]
fn no_parenthesis_deprecated() {}

#[pyfunction]
#[pyo3(warn())]
fn no_message_deprecated() {}

#[pyfunction]
#[pyo3(warn(category = clarax::exceptions::PyDeprecationWarning))]
fn no_message_deprecated_with_category() {}

#[pyfunction]
#[pyo3(warn(category = clarax::exceptions::PyDeprecationWarning, message = ,))]
fn empty_message_deprecated_with_category() {}

#[pyfunction]
#[pyo3(warn(message = "deprecated function", category = ,))]
fn empty_category_deprecated_with_message() {}

#[pyfunction]
#[pyo3(warn(message = "deprecated function", random_key))]
fn random_key_deprecated() {}

#[pyclass]
struct DeprecatedMethodContainer {}

#[pymethods]
impl DeprecatedMethodContainer {
    #[classattr]
    #[pyo3(warn(message = "deprecated class attr"))]
    fn deprecated_class_attr() -> i32 {
        5
    }
}

#[pymethods]
impl DeprecatedMethodContainer {
    #[pyo3(warn(message = "deprecated __traverse__"))]
    fn __traverse__(&self, _visit: clarax::gc::PyVisit<'_>) -> Result<(), clarax::PyTraverseError> {
        Ok(())
    }
}

fn main() {}
