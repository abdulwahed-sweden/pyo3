#![allow(unused_imports)]

use clarax::prelude::*;

#[pyfunction]
fn foo() -> usize {
    0
}

#[pymodule]
mod module {
    #[pymodule_export]
    use super::*;
}

fn main() {}
