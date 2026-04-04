# Python exceptions

## Defining a new exception

Use the [`create_exception!`] macro:

```rust
use pyforge::create_exception;

create_exception!(module, MyError, pyforge::exceptions::PyException);
```

- `module` is the name of the containing module.
- `MyError` is the name of the new exception type.

For example:

```rust
use pyforge::prelude::*;
use pyforge::create_exception;
use pyforge::types::IntoPyDict;
use pyforge::exceptions::PyException;

create_exception!(mymodule, CustomError, PyException);

# fn main() -> PyResult<()> {
Python::attach(|py| {
    let ctx = [("CustomError", py.get_type::<CustomError>())].into_py_dict(py)?;
    pyforge::py_run!(
        py,
        *ctx,
        "assert str(CustomError) == \"<class 'mymodule.CustomError'>\""
    );
    pyforge::py_run!(py, *ctx, "assert CustomError('oops').args == ('oops',)");
#   Ok(())
})
# }
```

When using PyForge to create an extension module, you can add the new exception to the module like this, so that it is importable from Python:

```rust,no_run
# fn main() {}
use pyforge::prelude::*;
use pyforge::exceptions::PyException;

pyforge::create_exception!(mymodule, CustomError, PyException);

#[pymodule]
mod mymodule {
    #[pymodule_export]
    use super::CustomError;

    // ... other elements added to module ...
}
```

## Raising an exception

As described in the [function error handling](./function/error-handling.md) chapter, to raise an exception from a `#[pyfunction]` or `#[pymethods]`, return an `Err(PyErr)`.
PyForge will automatically raise this exception for you when returning the result to Python.

You can also manually write and fetch errors in the Python interpreter's global state:

```rust
use pyforge::{Python, PyErr};
use pyforge::exceptions::PyTypeError;

Python::attach(|py| {
    PyTypeError::new_err("Error").restore(py);
    assert!(PyErr::occurred(py));
    drop(PyErr::fetch(py));
});
```

## Checking exception types

Python has an [`isinstance`](https://docs.python.org/3/library/functions.html#isinstance) method to check an object's type.
In PyForge every object has the [`PyAny::is_instance`] and [`PyAny::is_instance_of`] methods which do the same thing.

```rust,no_run
use pyforge::prelude::*;
use pyforge::types::{PyBool, PyList};

# fn main() -> PyResult<()> {
Python::attach(|py| {
    assert!(PyBool::new(py, true).is_instance_of::<PyBool>());
    let list = PyList::new(py, &[1, 2, 3, 4])?;
    assert!(!list.is_instance_of::<PyBool>());
    assert!(list.is_instance_of::<PyList>());
# Ok(())
})
# }
```

To check the type of an exception, you can similarly do:

```rust,no_run
# use pyforge::exceptions::PyTypeError;
# use pyforge::prelude::*;
# Python::attach(|py| {
# let err = PyTypeError::new_err(());
err.is_instance_of::<PyTypeError>(py);
# });
```

## Using exceptions defined in Python code

It is possible to use an exception defined in Python code as a native Rust type.
The [`import_exception!`] macro allows importing a specific exception class and defines a Rust type for that exception.

```rust,no_run
#![allow(dead_code)]
use pyforge::prelude::*;

mod io {
    pyforge::import_exception!(io, UnsupportedOperation);
}

fn tell(file: &Bound<'_, PyAny>) -> PyResult<u64> {
    match file.call_method0("tell") {
        Err(_) => Err(io::UnsupportedOperation::new_err("not supported: tell")),
        Ok(x) => x.extract::<u64>(),
    }
}
```

[`pyforge::exceptions`]({{#PYO3_DOCS_URL}}/pyforge/exceptions/index.html) defines exceptions for several standard library modules.

## Creating more complex exceptions

If you need to create an exception with more complex behavior, you can also manually create a subclass of `PyException`:

```rust
#![allow(dead_code)]
# #[cfg(any(not(Py_LIMITED_API), Py_3_12))] {
use pyforge::prelude::*;
use pyforge::types::IntoPyDict;
use pyforge::exceptions::PyException;

#[pyclass(extends=PyException)]
struct CustomError {
    #[pyforge(get)]
    url: String,

    #[pyforge(get)]
    message: String,
}

#[pymethods]
impl CustomError {
    #[new]
    fn new(url: String, message: String) -> Self {
        Self { url, message }
    }
}

# fn main() -> PyResult<()> {
Python::attach(|py| {
    let ctx = [("CustomError", py.get_type::<CustomError>())].into_py_dict(py)?;
    pyforge::py_run!(
        py,
        *ctx,
        "assert str(CustomError) == \"<class 'builtins.CustomError'>\", repr(CustomError)"
    );
    pyforge::py_run!(py, *ctx, "assert CustomError('https://example.com', 'something went bad').args == ('https://example.com', 'something went bad')");
    pyforge::py_run!(py, *ctx, "assert CustomError('https://example.com', 'something went bad').url == 'https://example.com'");
#   Ok(())
})
# }
# }

```

Note that when the `abi3` feature is enabled, subclassing `PyException` is only possible on Python 3.12 or greater.

[`create_exception!`]: {{#PYO3_DOCS_URL}}/pyforge/macro.create_exception.html
[`import_exception!`]: {{#PYO3_DOCS_URL}}/pyforge/macro.import_exception.html
[`PyAny::is_instance`]: {{#PYO3_DOCS_URL}}/pyforge/types/trait.PyAnyMethods.html#tymethod.is_instance
[`PyAny::is_instance_of`]: {{#PYO3_DOCS_URL}}/pyforge/types/trait.PyAnyMethods.html#tymethod.is_instance_of
