# sequential

A project built using only `pyforge_ffi`, without any of PyForge's safe api. It supports both subinterpreters and free-threaded Python.

## Building and Testing

To build this package, first install `maturin`:

```shell
pip install maturin
```

To build and test use `maturin develop`:

```shell
pip install -r requirements-dev.txt
maturin develop
pytest
```

Alternatively, install nox and run the tests inside an isolated environment:

```shell
nox
```

## Copying this example

Use [`cargo-generate`](https://crates.io/crates/cargo-generate):

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/abdulwahed-sweden/pyforge examples/sequential
```

(`cargo generate` will take a little while to clone the PyForge repo first; be patient when waiting for the command to run.)
