//! With abi3, we cannot inherit native types.
use clarax::prelude::*;
use clarax::types::PyDict;

#[pyclass(extends=PyDict)]
struct TestClass {}

fn main() {}
