# Python functions

The `#[pyfunction]` attribute is used to define a Python function from a Rust function.
Once defined, the function needs to be added to a [module](./module.md).

The following example defines a function called `double` in a Python module called `my_extension`:

```rust,no_run
#[pyforge::pymodule]
mod my_extension {
    use pyforge::prelude::*;

    #[pyfunction]
    fn double(x: usize) -> usize {
        x * 2
    }
}
```

This chapter of the guide explains full usage of the `#[pyfunction]` attribute.
In this first section, the following topics are covered:

- [Function options](#function-options)
  - [`#[pyforge(name = "...")]`](#name)
  - [`#[pyforge(signature = (...))]`](#signature)
  - [`#[pyforge(text_signature = "...")]`](#text_signature)
  - [`#[pyforge(pass_module)]`](#pass_module)
  - [`#[pyforge(warn(message = "...", category = ...))]`](#warn)
- [Per-argument options](#per-argument-options)
- [Advanced function patterns](#advanced-function-patterns)

There are also additional sections on the following topics:

- [Function Signatures](./function/signature.md)
- [Error Handling](./function/error-handling.md)

## Function options

The `#[pyforge]` attribute can be used to modify properties of the generated Python function.
It can take any combination of the following options:

- <a id="name"></a> `#[pyforge(name = "...")]`

    Overrides the name exposed to Python.

    In the following example, the Rust function `no_args_py` will be added to the Python module `module_with_functions` as the Python function `no_args`:

    ```rust
    # use pyforge::prelude::*;
    #[pyforge::pymodule]
    mod module_with_functions {
        use pyforge::prelude::*;

        #[pyfunction]
        #[pyforge(name = "no_args")]
        fn no_args_py() -> usize {
            42
        }
    }

    # Python::attach(|py| {
    #     let m = pyforge::wrap_pymodule!(module_with_functions)(py);
    #     assert!(m.getattr(py, "no_args").is_ok());
    #     assert!(m.getattr(py, "no_args_py").is_err());
    # });
    ```

- <a id="signature"></a> `#[pyforge(signature = (...))]`

    Defines the function signature in Python.
    See [Function Signatures](./function/signature.md).

- <a id="text_signature"></a> `#[pyforge(text_signature = "...")]`

    Overrides the PyForge-generated function signature visible in Python tooling (such as via [`inspect.signature`]).
    See the [corresponding topic in the Function Signatures subchapter](./function/signature.md#making-the-function-signature-available-to-python).

- <a id="pass_module" ></a> `#[pyforge(pass_module)]`

    Set this option to make PyForge pass the containing module as the first argument to the function.
    It is then possible to use the module in the function body.
    The first argument **must** be of type `&Bound<'_, PyModule>`, `Bound<'_, PyModule>`, or `Py<PyModule>`.

    The following example creates a function `pyfunction_with_module` which returns the containing module's name (i.e. `module_with_fn`):

    ```rust,no_run
    #[pyforge::pymodule]
    mod module_with_fn {
        use pyforge::prelude::*;
        use pyforge::types::PyString;

        #[pyfunction]
        #[pyforge(pass_module)]
        fn pyfunction_with_module<'py>(
            module: &Bound<'py, PyModule>,
        ) -> PyResult<Bound<'py, PyString>> {
            module.name()
        }
    }
    ```

- <a id="warn"></a> `#[pyforge(warn(message = "...", category = ...))]`

    This option is used to display a warning when the function is used in Python.
    It is equivalent to [`warnings.warn(message, category)`](https://docs.python.org/3/library/warnings.html#warnings.warn).
    The `message` parameter is a string that will be displayed when the function is called, and the `category` parameter is optional and has to be a subclass of [`Warning`](https://docs.python.org/3/library/exceptions.html#Warning).
    When the `category` parameter is not provided, the warning will be defaulted to [`UserWarning`](https://docs.python.org/3/library/exceptions.html#UserWarning).

    > Note: when used with `#[pymethods]`, this attribute does not work with `#[classattr]` nor `__traverse__` magic method.

    The following are examples of using the `#[pyforge(warn)]` attribute:

    ```rust
    use pyforge::prelude::*;

    #[pymodule]
    mod raising_warning_fn {
        use pyforge::prelude::pyfunction;
        use pyforge::exceptions::PyFutureWarning;

        #[pyfunction]
        #[pyforge(warn(message = "This is a warning message"))]
        fn function_with_warning() -> usize {
            42
        }

        #[pyfunction]
        #[pyforge(warn(message = "This function is warning with FutureWarning", category = PyFutureWarning))]
        fn function_with_warning_and_custom_category() -> usize {
            42
        }
    }

    # use pyforge::exceptions::{PyFutureWarning, PyUserWarning};
    # use pyforge::types::{IntoPyDict, PyList};
    # use pyforge::PyTypeInfo;
    #
    # fn catch_warning(py: Python<'_>, f: impl FnOnce(&Bound<'_, PyList>) -> ()) -> PyResult<()> {
    #     let warnings = py.import("warnings")?;
    #     let kwargs = [("record", true)].into_py_dict(py)?;
    #     let catch_warnings = warnings
    #         .getattr("catch_warnings")?
    #         .call((), Some(&kwargs))?;
    #     let list = catch_warnings.call_method0("__enter__")?.cast_into()?;
    #     warnings.getattr("simplefilter")?.call1(("always",))?;  // show all warnings
    #     f(&list);
    #     catch_warnings
    #         .call_method1("__exit__", (py.None(), py.None(), py.None()))
    #         .unwrap();
    #     Ok(())
    # }
    #
    # macro_rules! assert_warnings {
    #     ($py:expr, $body:expr, [$(($category:ty, $message:literal)),+] $(,)? ) => {
    #         catch_warning($py, |list| {
    #             $body;
    #             let expected_warnings = [$((<$category as PyTypeInfo>::type_object($py), $message)),+];
    #             assert_eq!(list.len(), expected_warnings.len());
    #             for (warning, (category, message)) in list.iter().zip(expected_warnings) {
    #                 assert!(warning.getattr("category").unwrap().is(&category));
    #                 assert_eq!(
    #                     warning.getattr("message").unwrap().str().unwrap().to_string_lossy(),
    #                     message
    #                 );
    #             }
    #         }).unwrap();
    #     };
    # }
    #
    # Python::attach(|py| {
    #     assert_warnings!(
    #         py,
    #         {
    #             let m = pyforge::wrap_pymodule!(raising_warning_fn)(py);
    #             let f1 = m.getattr(py, "function_with_warning").unwrap();
    #             let f2 = m.getattr(py, "function_with_warning_and_custom_category").unwrap();
    #             f1.call0(py).unwrap();
    #             f2.call0(py).unwrap();
    #         },
    #         [
    #             (PyUserWarning, "This is a warning message"),
    #             (
    #                 PyFutureWarning,
    #                 "This function is warning with FutureWarning"
    #             )
    #         ]
    #     );
    # });
    ```

    When the functions are called as the following, warnings will be displayed.

    ```python
    import warnings
    from raising_warning_fn import function_with_warning, function_with_warning_and_custom_category

    function_with_warning()
    function_with_warning_and_custom_category()
    ```

    The warning output will be:

    ```plaintext
    UserWarning: This is a warning message
    FutureWarning: This function is warning with FutureWarning
    ```

## Per-argument options

The `#[pyforge]` attribute can be used on individual arguments to modify properties of them in the generated function.
It can take any combination of the following options:

- <a id="from_py_with"></a> `#[pyforge(from_py_with = ...)]`

    Set this on an option to specify a custom function to convert the function argument from Python to the desired Rust type, instead of using the default `FromPyObject` extraction.
    The function signature must be `fn(&Bound<'_, PyAny>) -> PyResult<T>` where `T` is the Rust type of the argument.

    The following example uses `from_py_with` to convert the input Python object to its length:

    ```rust
    use pyforge::prelude::*;

    fn get_length(obj: &Bound<'_, PyAny>) -> PyResult<usize> {
        obj.len()
    }

    #[pyfunction]
    fn object_length(#[pyforge(from_py_with = get_length)] argument: usize) -> usize {
        argument
    }

    # Python::attach(|py| {
    #     let f = pyforge::wrap_pyfunction!(object_length)(py).unwrap();
    #     assert_eq!(f.call1((vec![1, 2, 3],)).unwrap().extract::<usize>().unwrap(), 3);
    # });
    ```

## Advanced function patterns

### Calling Python functions in Rust

You can pass Python `def`'d functions and built-in functions to Rust functions [`PyFunction`] corresponds to regular Python functions while [`PyCFunction`] describes built-ins such as `repr()`.

You can also use [`Bound<'_, PyAny>::is_callable`] to check if you have a callable object.
`is_callable` will return `true` for functions (including lambdas), methods and objects with a `__call__` method.
You can call the object with [`Bound<'_, PyAny>::call`] with the args as first parameter and the kwargs (or `None`) as second parameter.
There are also [`Bound<'_, PyAny>::call0`] with no args and [`Bound<'_, PyAny>::call1`] with only positional args.

### Calling Rust functions in Python

The ways to convert a Rust function into a Python object vary depending on the function:

- Named functions, e.g. `fn foo()`: add `#[pyfunction]` and then use [`wrap_pyfunction!`] to get the corresponding [`PyCFunction`].
- Anonymous functions (or closures), e.g. `foo: fn()` either:
  - use a `#[pyclass]` struct which stores the function as a field and implement `__call__` to call the stored function.
  - use `PyCFunction::new_closure` to create an object directly from the function.

### Accessing the FFI functions

In order to make Rust functions callable from Python, PyForge generates an `extern "C"` function whose exact signature depends on the Rust signature. (PyForge chooses the optimal Python argument passing convention.) It then embeds the call to the Rust function inside this FFI-wrapper function.
This wrapper handles extraction of the regular arguments and the keyword arguments from the input `PyObject`s.

The `wrap_pyfunction` macro can be used to directly get a `Bound<PyCFunction>` given a `#[pyfunction]` and a `Bound<PyModule>`: `wrap_pyfunction!(rust_fun, module)`.

[`Bound<'_, PyAny>::is_callable`]: {{#PYO3_DOCS_URL}}/pyforge/prelude/trait.PyAnyMethods.html#tymethod.is_callable
[`Bound<'_, PyAny>::call`]: {{#PYO3_DOCS_URL}}/pyforge/prelude/trait.PyAnyMethods.html#tymethod.call
[`Bound<'_, PyAny>::call0`]: {{#PYO3_DOCS_URL}}/pyforge/prelude/trait.PyAnyMethods.html#tymethod.call0
[`Bound<'_, PyAny>::call1`]: {{#PYO3_DOCS_URL}}/pyforge/prelude/trait.PyAnyMethods.html#tymethod.call1
[`wrap_pyfunction!`]: {{#PYO3_DOCS_URL}}/pyforge/macro.wrap_pyfunction.html
[`PyFunction`]: {{#PYO3_DOCS_URL}}/pyforge/types/struct.PyFunction.html
[`PyCFunction`]: {{#PYO3_DOCS_URL}}/pyforge/types/struct.PyCFunction.html
[`inspect.signature`]: https://docs.python.org/3/library/inspect.html#inspect.signature
