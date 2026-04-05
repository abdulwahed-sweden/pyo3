use clarax::prelude::*;
use clarax::types::PyString;

fn main() {
    Python::attach(|py| {
        let string = PyString::new(py, "foo");

        py.detach(|| {
            println!("{:?}", string);
        });
    });
}
