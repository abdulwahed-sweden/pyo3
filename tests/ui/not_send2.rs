use pyforge::prelude::*;
use pyforge::types::PyString;

fn main() {
    Python::attach(|py| {
        let string = PyString::new(py, "foo");

        py.detach(|| {
            println!("{:?}", string);
        });
    });
}
