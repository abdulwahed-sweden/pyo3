use pyforge::Python;

fn main() {
    let _foo = if true { "foo" } else { "bar" };
    Python::attach(|py| py.import(pyforge::intern!(py, _foo)).unwrap());
}
