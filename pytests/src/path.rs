use pyforge::prelude::*;

#[pymodule]
pub mod path {
    use pyforge::prelude::*;
    use std::path::{Path, PathBuf};

    #[pyfunction]
    fn make_path() -> PathBuf {
        Path::new("/root").to_owned()
    }

    #[pyfunction]
    fn take_pathbuf(path: PathBuf) -> PathBuf {
        path
    }
}
