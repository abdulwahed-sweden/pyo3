use clarax::prelude::*;

#[pymodule]
mod module {
    use clarax::prelude::*;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        Ok(())
    }

    #[pymodule_init]
    fn init2(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        Ok(())
    }
}

fn main() {}
