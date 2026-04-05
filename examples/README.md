# ClaraX Examples

These example crates are a collection of toy extension modules built with ClaraX. They are all tested using `nox` in ClaraX's CI.

Below is a brief description of each of these:

| Example | Description |
| ------- | ----------- |
| `decorator` | A project showcasing the example from the [Emulating callable objects](https://github.com/abdulwahed-sweden/clarax/latest/class/call.html) chapter of the guide. |
| `maturin-starter` | A template project which is configured to use [`maturin`](https://github.com/PyO3/maturin) for development. |
| `setuptools-rust-starter` | A template project which is configured to use [`setuptools_rust`](https://github.com/PyO3/setuptools-rust/) for development. |
| `plugin` | Illustrates how to use Python as a scripting language within a Rust application |

Note that there are also other examples in the `clarax-ffi/examples`
directory that illustrate how to create rust extensions using raw FFI calls into
the CPython C API instead of using ClaraX's abstractions.

## Creating new projects from these examples

To copy an example, use [`cargo-generate`](https://crates.io/crates/cargo-generate). Follow the commands below, replacing `<example>` with the example to start from:

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/abdulwahed-sweden/clarax examples/<example>
```

(`cargo generate` will take a little while to clone the ClaraX repo first; be patient when waiting for the command to run.)
