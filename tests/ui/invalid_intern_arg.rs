use clarax::Python;

fn main() {
    let _foo = if true { "foo" } else { "bar" };
    Python::attach(|py| py.import(clarax::intern!(py, _foo)).unwrap());
}
