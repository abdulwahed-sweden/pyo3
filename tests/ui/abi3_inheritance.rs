use clarax::exceptions::PyException;
use clarax::prelude::*;

#[pyclass(extends=PyException)]
#[derive(Clone)]
struct MyException {
    code: u32,
}

fn main() {}
