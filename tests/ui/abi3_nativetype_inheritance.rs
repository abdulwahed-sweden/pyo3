//! With abi3, we cannot inherit native types.
use pyforge::prelude::*;
use pyforge::types::PyDict;

#[pyclass(extends=PyDict)]
struct TestClass {}

fn main() {}
