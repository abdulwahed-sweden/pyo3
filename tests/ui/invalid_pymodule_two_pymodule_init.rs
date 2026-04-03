use pyforge::prelude::*;

#[pymodule]
mod module {
    use pyforge::prelude::*;

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
