# string_sum

A project built using only `clarax_ffi`, without any of ClaraX's safe api.

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
$ cargo generate --git https://github.com/abdulwahed-sweden/clarax examples/string_sum
```

(`cargo generate` will take a little while to clone the ClaraX repo first; be patient when waiting for the command to run.)
