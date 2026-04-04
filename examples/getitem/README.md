# getitem

A project showcasing how to create a `__getitem__` override that also showcases how to deal with multiple incoming types

## Relevant Documentation

Some of the relevant documentation links for this example:

* Converting Slices to Indices: https://docs.rs/pyforge/latest/pyforge/types/struct.PySlice.html#method.indices
* GetItem Docs: https://github.com/abdulwahed-sweden/pyforge/latest/class/protocols.html?highlight=__getitem__#mapping--sequence-types
* Extract: https://github.com/abdulwahed-sweden/pyforge/latest/conversions/traits.html?highlight=extract#extract-and-the-frompyobject-trait
* Downcast and getattr: https://github.com/abdulwahed-sweden/pyforge/v0.19.0/types.html?highlight=getattr#pyany


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
$ cargo generate --git https://github.com/abdulwahed-sweden/pyforge examples/decorator
```

(`cargo generate` will take a little while to clone the PyForge repo first; be patient when waiting for the command to run.)
