#[clarax::pymodule]
mod mymodule {
	#[clarax::pymodule(submodule)]
	mod submod {}
}

fn main() {}
