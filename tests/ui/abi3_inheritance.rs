use pyforge::exceptions::PyException;
use pyforge::prelude::*;

#[pyclass(extends=PyException)]
#[derive(Clone)]
struct MyException {
    code: u32,
}

fn main() {}
