#[pyforge::pymodule]
mod pyo3_scratch {
    use pyforge::prelude::*;

    #[pyclass]
    struct Foo {}

    #[pymethods]
    impl Foo {
        #[pyfunction]
        fn bug() {}
    }
}

fn main() {}
