#[clarax::pymodule]
mod pyo3_scratch {
    use clarax::prelude::*;

    #[pyclass]
    struct Foo {}

    #[pymethods]
    impl Foo {
        #[pyfunction]
        fn bug() {}
    }
}

fn main() {}
