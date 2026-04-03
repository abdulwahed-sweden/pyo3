struct Blah;

#[pyforge::pyfunction]
fn blah() -> Blah {
    Blah
}

fn main() {}
