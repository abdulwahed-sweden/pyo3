//! With abi3, weakref not supported until python 3.9 or greater
use clarax::prelude::*;

#[pyclass(weakref)]
struct TestClass {}

fn main() {}
