#[pyforge::pymodule]
mod mymodule {
	#[pyforge::pymodule(submodule)]
	mod submod {}
}

fn main() {}
