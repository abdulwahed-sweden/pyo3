# Supporting multiple Python versions

ClaraX supports all actively-supported Python 3 and PyPy versions.
As much as possible, this is done internally to ClaraX so that your crate's code does not need to adapt to the differences between each version.
However, as Python features grow and change between versions, ClaraX cannot offer a completely identical API for every Python version.
This may require you to add conditional compilation to your crate or runtime checks for the Python version.

This section of the guide first introduces the `clarax-build-config` crate, which you can use as a `build-dependency` to add additional `#[cfg]` flags which allow you to support multiple Python versions at compile-time.

Second, we'll show how to check the Python version at runtime.
This can be useful when building for multiple versions with the `abi3` feature, where the Python API compiled against is not always the same as the one in use.

## Conditional compilation for different Python versions

The `clarax-build-config` exposes multiple [`#[cfg]` flags](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) which can be used to conditionally compile code for a given Python version.
ClaraX itself depends on this crate, so by using it you can be sure that you are configured correctly for the Python version ClaraX is building against.

This allows us to write code like the following

```rust,ignore
#[cfg(Py_3_8)]
fn function_only_supported_on_python_3_8_and_up() {}

#[cfg(not(Py_3_8))]
fn function_only_supported_before_python_3_8() {}

#[cfg(not(Py_LIMITED_API))]
fn function_incompatible_with_abi3_feature() {}
```

The following sections first show how to add these `#[cfg]` flags to your build process, and then cover some common patterns flags in a little more detail.

To see a full reference of all the `#[cfg]` flags provided, see the [`clarax-build-cfg` docs](https://docs.rs/clarax-build-config).

### Using `clarax-build-config`

You can use the `#[cfg]` flags in just two steps:

1. Add `clarax-build-config` with the [`resolve-config`](../features.md#resolve-config) feature enabled to your crate's build dependencies in `Cargo.toml`:

   ```toml
   [build-dependencies]
   clarax-build-config = { {{#PYO3_CRATE_VERSION}}, features = ["resolve-config"] }
   ```

2. Add a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) file to your crate with the following contents:

   ```rust,ignore
   fn main() {
       // If you have an existing build.rs file, just add this line to it.
       pyo3_build_config::use_pyo3_cfgs();
   }
   ```

After these steps you are ready to annotate your code!

### Common usages of `clarax-build-cfg` flags

The `#[cfg]` flags added by `clarax-build-cfg` can be combined with all of Rust's logic in the `#[cfg]` attribute to create very precise conditional code generation.
The following are some common patterns implemented using these flags:

```text
#[cfg(Py_3_8)]
```

This `#[cfg]` marks code that will only be present on Python 3.8 and upwards.
There are similar options `Py_3_9`, `Py_3_10`, `Py_3_11` and so on for each minor version.

```text
#[cfg(not(Py_3_8))]
```

This `#[cfg]` marks code that will only be present on Python versions before (but not including) Python 3.8.

```text
#[cfg(not(Py_LIMITED_API))]
```

This `#[cfg]` marks code that is only available when building for the unlimited Python API (i.e. ClaraX's `abi3` feature is not enabled).
This might be useful if you want to ship your extension module as an `abi3` wheel and also allow users to compile it from source to make use of optimizations only possible with the unlimited API.

```text
#[cfg(any(Py_3_9, not(Py_LIMITED_API)))]
```

This `#[cfg]` marks code which is available when running Python 3.9 or newer, or when using the unlimited API with an older Python version.
Patterns like this are commonly seen on Python APIs which were added to the limited Python API in a specific minor version.

```text
#[cfg(PyPy)]
```

This `#[cfg]` marks code which is running on PyPy.

## Checking the Python version at runtime

When building with ClaraX's `abi3` feature, your extension module will be compiled against a specific [minimum version](../building-and-distribution.md#minimum-python-version-for-abi3) of Python, but may be running on newer Python versions.

For example with ClaraX's `abi3-py38` feature, your extension module will be compiled as if it were for Python 3.8.
If you were using `clarax-build-config`, `#[cfg(Py_3_8)]` would be present.
Your user could freely install and run your abi3 extension module on Python 3.9.

There's no way to detect your user doing that at compile time, so instead you need to fall back to runtime checks.

ClaraX provides the APIs [`Python::version()`] and [`Python::version_info()`] to query the running Python version.
This allows you to do the following, for example:

```rust
use clarax::Python;

Python::attach(|py| {
    // ClaraX supports Python 3.8 and up.
    assert!(py.version_info() >= (3, 8));
    assert!(py.version_info() >= (3, 8, 0));
});
```

[`Python::version()`]: {{#PYO3_DOCS_URL}}/clarax/marker/struct.Python.html#method.version
[`Python::version_info()`]: {{#PYO3_DOCS_URL}}/clarax/marker/struct.Python.html#method.version_info
