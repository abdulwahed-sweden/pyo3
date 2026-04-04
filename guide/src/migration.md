# Migrating from older PyForge versions

This guide can help you upgrade code through breaking changes from one PyForge version to the next.
For a detailed list of all changes, see the [CHANGELOG](changelog.md).

## from 0.28.* to 0.29

### Removed implementations of `From<str::Utf8Error>`, `From<string::FromUtf16Error>`, and `From<char::DecodeUtf16Error>` for `PyErr`

Previously the implementations of `From<string::FromUtf8Error>`, `From<ffi::IntoStringError>`, `From<str::Utf8Error>`, `From<string::FromUtf16Error>`, and `From<char::DecodeUtf16Error>` failed to construct the correct Python exception class, as reported in <https://github.com/abdulwahed-sweden/pyforge/issues/5651>.
The implementations for `string::FromUtf8Error` and `ffi::IntoStringError` were fixed in this release.

For `str::Utf8Error`, the Rust error does not contain the source bytes required to construct the Python exception.
Instead, `PyUnicodeDecodeError::new_err_from_utf8` can be used to convert the error to a `PyErr`.

Before:

```rust,ignore
fn bytes_to_str(bytes: &[u8]) -> PyResult<&str> {
    Ok(std::str::from_utf8(bytes)?)
}
```

After:

```rust
# use pyforge::prelude::*;
use pyforge::exceptions::PyUnicodeDecodeError;

# #[expect(dead_code)]
fn bytes_to_str<'a>(py: Python<'_>, bytes: &'a [u8]) -> PyResult<&'a str> {
    std::str::from_utf8(bytes).map_err(|e| PyUnicodeDecodeError::new_err_from_utf8(py, bytes, e))
}
```

For `string::FromUtf16Error` and `char::DecodeUtf16Error` the Rust error types do not contain any of the information required to construct a `UnicodeDecodeError`.
To raise a Python `UnicodeDecodeError` a new error should be manually constructed by calling `PyUnicodeDecodeError::new_err(...)`.

## from 0.27.* to 0.28

### Default to supporting free-threaded Python

<details open>
<summary><small>Click to expand</small></summary>

When PyForge 0.23 added support for free-threaded Python, this was as an opt-in feature for modules by annotating with `#[pymodule(gil_used = false)]`.

As the support has matured and PyForge's own API has evolved to remove reliance on the GIL, the time is right to switch the default.
Modules now automatically allow use on free-threaded Python, unless they directly state they require the GIL with `#[pymodule(gil_used = true)]`.
</details>

### Deprecation of automatic `FromPyObject` for `#[pyclass]` types which implement `Clone`

<details open>
<summary><small>Click to expand</small></summary>

`#[pyclass]` types which implement `Clone` used to also implement `FromPyObject` automatically.
This behavior is being phased out and replaced by an explicit opt-in, which will allow [better error messages and more user control](https://github.com/abdulwahed-sweden/pyforge/issues/5419).
Affected types will by marked by a deprecation message.

To migrate use either

- `#[pyclass(from_py_object)]` to keep the automatic derive, or
- `#[pyclass(skip_from_py_object)]` to accept the new behavior.

Before:

```rust
# #![allow(deprecated)]
# use pyforge::prelude::*;
#[pyclass]
#[derive(Clone)]
struct PyClass {}
```

After:

```rust
# use pyforge::prelude::*;
// If the automatic implementation of `FromPyObject` is desired, opt in:
#[pyclass(from_py_object)]
#[derive(Clone)]
struct PyClass {}

// or if the `FromPyObject` implementation is not needed:
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct PyClassWithoutFromPyObject {}
```

The `#[pyclass(skip_from_py_object)]` option will eventually be deprecated and removed as it becomes the default behavior.

</details>

### Deprecation of `Py<T>` constructors from raw pointer

<details open>
<summary><small>Click to expand</small></summary>

The constructors `Py::from_owned_ptr`, `Py::from_owned_ptr_or_opt`, and `Py::from_owned_ptr_or_err` (and similar "borrowed" variants) perform an unchecked cast to the `Py<T>` target type `T`.
This unchecked cast is a footgun on APIs where the primary concern is about constructing PyForge's safe smart pointer types correctly from the raw pointer value.

The equivalent constructors on `Bound` always produce a `Bound<PyAny>`, which encourages any subsequent cast to be done explicitly as either checked or unchecked.
These should be used instead.

Before:

```rust
# #![allow(deprecated)]
# use pyforge::prelude::*;
# use pyforge::types::PyNone;
# Python::attach(|py| {
let raw_ptr = py.None().into_ptr();

let _: Py<PyNone> = unsafe { Py::from_borrowed_ptr(py, raw_ptr) };
let _: Py<PyNone> = unsafe { Py::from_owned_ptr(py, raw_ptr) };
# })
```

After:

```rust
# use pyforge::prelude::*;
# use pyforge::types::PyNone;
# Python::attach(|py| {
let raw_ptr = py.None().into_ptr();

// Bound APIs require choice of doing unchecked or checked cast. Optionally `.unbind()` to
// produce `Py<T>` values.
let _: Bound<'_, PyNone> = unsafe { Bound::from_borrowed_ptr(py, raw_ptr).cast_into_unchecked() };
let _: Bound<'_, PyNone> = unsafe { Bound::from_owned_ptr(py, raw_ptr).cast_into_unchecked() };
# })
```

</details>

### Removal of `From<Bound<'_, T>` and `From<Py<T>> for PyClassInitializer<T>`

<details open>
<summary><small>Click to expand</small></summary>

As part of refactoring the initialization code these impls were removed and its functionality was moved into the generated code for `#[new]`.
As a small side side effect the following pattern will not be accepted anymore:

```rust,ignore
# use pyforge::prelude::*;
# Python::attach(|py| {
# let existing_py: Py<PyAny> = py.None();
let obj_1 = Py::new(py, existing_py);

# let existing_bound: Bound<'_, PyAny> = py.None().into_bound(py);
let obj_2 = Bound::new(py, existing_bound);
# })
```

To migrate use `clone` or `clone_ref`:

```rust
# use pyforge::prelude::*;
# Python::attach(|py| {
# let existing_py: Py<PyAny> = py.None();
let obj_1 = existing_py.clone_ref(py);

# let existing_bound: Bound<'_, PyAny> = py.None().into_bound(py);
let obj_2 = existing_bound.clone();
# })
```

</details>

### Untyped buffer API moved to PyUntypedBuffer

<details open>
<summary><small>Click to expand</small></summary>

`PyBuffer<T>` now is a typed wrapper around `PyUntypedBuffer`.
Many methods such as `PyBuffer::format` have been moved to `PyUntypedBuffer::format`.
`PyBuffer<T>` dereferences to `PyUntypedBuffer`, so method call syntax will continue to work as-is.
Users may need to update references to the moved functions.
</details>

### Internal change to use multi-phase initialization

<details open>
<summary><small>Click to expand</small></summary>

[PEP 489](https://peps.python.org/pep-0489/) introduced "multi-phase initialization" for extension modules which provides ways to allocate and clean up per-module state.
This is a necessary step towards supporting Python "subinterpreters" which run on their own copy of state.

Starting in PyForge 0.28, the `#[pymodule]` macro machinery has been reworked to use multi-phase initialization.
The possibility of creating and consuming per-module state (and supporting subinterpreters) is left for a future PyForge version.
This should not require migration, nor is there expected to be breakage caused by the change.

Nevertheless, this affects the order of initialization so seemed worth noting in this guide.
</details>

## from 0.26.* to 0.27

### `FromPyObject` reworked for flexibility and efficiency

<details>
<summary><small>Click to expand</small></summary>

With the removal of the `gil-ref` API in PyForge 0.23 it is now possible to fully split the Python lifetime `'py` and the input lifetime `'a`.
This allows borrowing from the input data without extending the lifetime of being attached to the interpreter.

`FromPyObject` now takes an additional lifetime `'a` describing the input lifetime.
The argument type of the `extract` method changed from `&Bound<'py, PyAny>` to `Borrowed<'a, 'py, PyAny>`.
This was done because `&'a Bound<'py, PyAny>` would have an implicit restriction `'py: 'a` due to the reference type.

This new form was partly implemented already in 0.22 using the internal `FromPyObjectBound` trait and is now extended to all types.

Most implementations can just add an elided lifetime to migrate.

Additionally `FromPyObject` gained an associated type `Error`.
This is the error type that can be used in case of a conversion error.
During migration using `PyErr` is a good default, later a custom error type can be introduced to prevent unnecessary creation of Python exception objects and improved type safety.

Before:

```rust,ignore
impl<'py> FromPyObject<'py> for IpAddr {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        ...
    }
}
```

After

```rust,ignore
impl<'py> FromPyObject<'_, 'py> for IpAddr {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        ...
        // since `Borrowed` derefs to `&Bound`, the body often
        // needs no changes, or adding an occasional `&`
    }
}
```

Occasionally, more steps are necessary.
For generic types, the bounds need to be adjusted.
The correct bound depends on how the type is used.

For simple wrapper types usually it's possible to just forward the bound.

Before:

```rust,ignore
struct MyWrapper<T>(T);

impl<'py, T> FromPyObject<'py> for MyWrapper<T>
where
    T: FromPyObject<'py>
{
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        ob.extract().map(MyWrapper)
    }
}
```

After:

```rust
# use pyforge::prelude::*;
# #[allow(dead_code)]
# pub struct MyWrapper<T>(T);
impl<'a, 'py, T> FromPyObject<'a, 'py> for MyWrapper<T>
where
    T: FromPyObject<'a, 'py>
{
    type Error = T::Error;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        obj.extract().map(MyWrapper)
    }
}
```

Container types that need to create temporary Python references during extraction, for example extracting from a `PyList`, requires a stronger bound.
For these the `FromPyObjectOwned` trait was introduced.
It is automatically implemented for any type that implements `FromPyObject` and does not borrow from the input.
It is intended to be used as a trait bound in these situations.

Before:

```rust,ignore
struct MyVec<T>(Vec<T>);
impl<'py, T> FromPyObject<'py> for Vec<T>
where
    T: FromPyObject<'py>,
{
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut v = MyVec(Vec::new());
        for item in obj.try_iter()? {
            v.0.push(item?.extract::<T>()?);
        }
        Ok(v)
    }
}
```

After:

```rust
# use pyforge::prelude::*;
# #[allow(dead_code)]
# pub struct MyVec<T>(Vec<T>);
impl<'py, T> FromPyObject<'_, 'py> for MyVec<T>
where
    T: FromPyObjectOwned<'py> // 👈 can only extract owned values, because each `item` below
                              //    is a temporary short lived owned reference
{
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let mut v = MyVec(Vec::new());
        for item in obj.try_iter()? {
            v.0.push(item?.extract::<T>().map_err(Into::into)?); // `map_err` is needed because `?` uses `From`, not `Into` 🙁
        }
        Ok(v)
    }
}
```

This is very similar to `serde`s [`Deserialize`] and [`DeserializeOwned`] traits, see [the `serde` docs](https://serde.rs/lifetimes.html).

[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[`DeserializeOwned`]: https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html
</details>

### `.downcast()` and `DowncastError` replaced with `.cast()` and `CastError`

<details>
<summary><small>Click to expand</small></summary>

The `.downcast()` family of functions were only available on `Bound<PyAny>`.
In corner cases (particularly related to `.downcast_into()`) this would require use of `.as_any().downcast()` or `.into_any().downcast_into()` chains.
Additionally, `DowncastError` produced Python exception messages which are not very Pythonic due to use of Rust type names in the error messages.

The `.cast()` family of functions are available on all `Bound` and `Borrowed` smart pointers, whatever the type, and have error messages derived from the actual type at runtime.
This produces a nicer experience for both PyForge module authors and consumers.

To migrate, replace `.downcast()` with `.cast()` and `DowncastError` with `CastError` (and similar with `.downcast_into()` / `DowncastIntoError` etc).

`CastError` requires a Python `type` object (or other "classinfo" object compatible with `isinstance()`) as the second object, so in the rare case where `DowncastError` was manually constructed, small adjustments to code may apply.
</details>

### `PyTypeCheck` is now an `unsafe trait`

<details>
<summary><small>Click to expand</small></summary>

Because `PyTypeCheck` is the trait used to guard the `.cast()` functions to treat Python objects as specific concrete types, the trait is `unsafe` to implement.

This should always have been the case, it was an unfortunate omission from its original implementation which is being corrected in this release.
</details>

## from 0.25.* to 0.26

### Rename of `Python::with_gil`, `Python::allow_threads`, and `pyforge::prepare_freethreaded_python`

<details>
<summary><small>Click to expand</small></summary>

The names for these APIs were created when the global interpreter lock (GIL) was mandatory.
With the introduction of free-threading in Python 3.13 this is no longer the case, and the naming has no universal meaning anymore.
For this reason, we chose to rename these to more modern terminology introduced in free-threading:

- `Python::with_gil` is now called `Python::attach`, it attaches a Python thread-state to the current thread.
  In GIL enabled builds there can only be 1 thread attached to the interpreter, in free-threading there can be more.
- `Python::allow_threads` is now called `Python::detach`, it detaches a previously attached thread-state.
- `pyforge::prepare_freethreaded_python` is now called `Python::initialize`.
</details>

### Deprecation of `PyObject` type alias

<details>
<summary><small>Click to expand</small></summary>

The type alias `PyObject` (aka `Py<PyAny>`) is often confused with the identically named FFI definition `pyforge::ffi::PyObject`.
For this reason we are deprecating its usage.
To migrate simply replace its usage by the target type `Py<PyAny>`.
</details>

### Replacement of `GILOnceCell` with `PyOnceLock`

<details>
<summary><small>Click to expand</small></summary>

Similar to the above renaming of `Python::with_gil` and related APIs, the `GILOnceCell` type was designed for a Python interpreter which was limited by the GIL.
Aside from its name, it allowed for the "once" initialization to race because the racing was mediated by the GIL and was extremely unlikely to manifest in practice.

With the introduction of free-threaded Python the racy initialization behavior is more likely to be problematic and so a new type `PyOnceLock` has been introduced which performs true single-initialization correctly while attached to the Python interpreter.
It exposes the same API as `GILOnceCell`, so should be a drop-in replacement with the notable exception that if the racy initialization of `GILOnceCell` was inadvertently relied on (e.g. due to circular references) then the stronger once-ever guarantee of `PyOnceLock` may lead to deadlocking which requires refactoring.

Before:

```rust,ignore
# use pyforge::prelude::*;
# use pyforge::sync::GILOnceCell;
# use pyforge::types::PyType;
# fn main() -> PyResult<()> {
# Python::attach(|py| {
static DECIMAL_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
DECIMAL_TYPE.import(py, "decimal", "Decimal")?;
# Ok(())
# })
# }
```

After:

```rust
# use pyforge::prelude::*;
# use pyforge::sync::PyOnceLock;
# use pyforge::types::PyType;
# fn main() -> PyResult<()> {
# Python::attach(|py| {
static DECIMAL_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
DECIMAL_TYPE.import(py, "decimal", "Decimal")?;
# Ok(())
# })
# }
```

</details>

### Deprecation of `GILProtected`

<details>
<summary><small>Click to expand</small></summary>

As another cleanup related to concurrency primitives designed for a Python constrained by the GIL, the `GILProtected` type is now deprecated.
Prefer to use concurrency primitives which are compatible with free-threaded Python, such as [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) (in combination with PyForge's [`MutexExt`]({{#PYO3_DOCS_URL}}/pyforge/sync/trait.MutexExt.html) trait).

Before:

```rust,ignore
# use pyforge::prelude::*;
# fn main() {
# #[cfg(not(Py_GIL_DISABLED))] {
use pyforge::sync::GILProtected;
use std::cell::RefCell;
# Python::attach(|py| {
static NUMBERS: GILProtected<RefCell<Vec<i32>>> = GILProtected::new(RefCell::new(Vec::new()));
Python::attach(|py| {
    NUMBERS.get(py).borrow_mut().push(42);
});
# })
# }
# }
```

After:

```rust
# use pyforge::prelude::*;
use pyforge::sync::MutexExt;
use std::sync::Mutex;
# fn main() {
# Python::attach(|py| {
static NUMBERS: Mutex<Vec<i32>> = Mutex::new(Vec::new());
Python::attach(|py| {
    NUMBERS.lock_py_attached(py).expect("no poisoning").push(42);
});
# })
# }
```

</details>

### `PyMemoryError` now maps to `io::ErrorKind::OutOfMemory` when converted to `io::Error`

<details>
<summary><small>Click to expand</small></summary>

Previously, converting a `PyMemoryError` into a Rust `io::Error` would result in an error with kind `Other`.
Now, it produces an error with kind `OutOfMemory`.
Similarly, converting an `io::Error` with kind `OutOfMemory` back into a Python error would previously yield a generic `PyOSError`.
Now, it yields a `PyMemoryError`.

This change makes error conversions more precise and matches the semantics of out-of-memory errors between Python and Rust.
</details>

## from 0.24.* to 0.25

### `AsPyPointer` removal

<details>
<summary><small>Click to expand</small></summary>

The `AsPyPointer` trait is mostly a leftover from the now removed gil-refs API.
The last remaining uses were the GC API, namely `PyVisit::call`, and identity comparison (`PyAnyMethods::is` and `Py::is`).

`PyVisit::call` has been updated to take `T: Into<Option<&Py<T>>>`, which allows for arguments of type `&Py<T>`, `&Option<Py<T>>` and `Option<&Py<T>>`.
It is unlikely any changes are needed here to migrate.

`PyAnyMethods::is`/ `Py::is` has been updated to take `T: AsRef<Py<PyAny>>>`.
Additionally `AsRef<Py<PyAny>>>` implementations were added for `Py`, `Bound` and `Borrowed`.
Because of the existing `AsRef<Bound<PyAny>> for Bound<T>` implementation this may cause inference issues in non-generic code.
This can be easily migrated by switching to `as_any` instead of `as_ref` for these calls.
</details>

## from 0.23.* to 0.24

<details>
<summary><small>Click to expand</small></summary>
There were no significant changes from 0.23 to 0.24 which required documenting in this guide.
</details>

## from 0.22.* to 0.23

<details>
<summary><small>Click to expand</small></summary>

PyForge 0.23 is a significant rework of PyForge's internals for two major improvements:

- Support of Python 3.13's new freethreaded build (aka "3.13t")
- Rework of to-Python conversions with a new `IntoPyObject` trait.

These changes are both substantial and reasonable efforts have been made to allow as much code as possible to continue to work as-is despite the changes.
The impacts are likely to be seen in three places when upgrading:

- PyForge's data structures [are now thread-safe](#free-threaded-python-support) instead of reliant on the GIL for synchronization.
  In particular, `#[pyclass]` types are [now required to be `Sync`](./class/thread-safety.md).
- The [`IntoPyObject` trait](#new-intopyobject-trait-unifies-to-python-conversions) may need to be implemented for types in your codebase.
  In most cases this can simply be done with [`#[derive(IntoPyObject)]`](#intopyobject-and-intopyobjectref-derive-macros).
  There will be many deprecation warnings from the replacement of `IntoPy` and `ToPyObject` traits.
- There will be many deprecation warnings from the [final removal of the `gil-refs` feature](#gil-refs-feature-removed), which opened up API space for a cleanup and simplification to PyForge's "Bound" API.

The sections below discuss the rationale and details of each change in more depth.
</details>

### Free-threaded Python Support

<details>
<summary><small>Click to expand</small></summary>

PyForge 0.23 introduces initial support for the new free-threaded build of CPython 3.13, aka "3.13t".

Because this build allows multiple Python threads to operate simultaneously on underlying Rust data, the `#[pyclass]` macro now requires that types it operates on implement `Sync`.

Aside from the change to `#[pyclass]`, most features of PyForge work unchanged, as the changes have been to the internal data structures to make them thread-safe.
An example of this is the `GILOnceCell` type, which used the GIL to synchronize single-initialization.
It now uses internal locks to guarantee that only one write ever succeeds, however it allows for multiple racing runs of the initialization closure.
It may be preferable to instead use `std::sync::OnceLock` in combination with the `pyforge::sync::OnceLockExt` trait which adds `OnceLock::get_or_init_py_attached` for single-initialization where the initialization closure is guaranteed only ever to run once and without deadlocking with the GIL.

Future PyForge versions will likely add more traits and data structures to make working with free-threaded Python easier.

Some features are inaccessible on the free-threaded build:

- The `GILProtected` type, which relied on the GIL to expose synchronized access to inner contents
- `PyList::get_item_unchecked`, which cannot soundly be used due to races between time-of-check and time-of-use

If you make use of these features then you will need to account for the unavailability of the API in the free-threaded build.
One way to handle it is via conditional compilation -- extension modules can use `pyforge-build-config` to get access to a `#[cfg(Py_GIL_DISABLED)]` guard.

See [the guide section on free-threaded Python](free-threading.md) for more details about supporting free-threaded Python in a PyForge extension module.
</details>

### New `IntoPyObject` trait unifies to-Python conversions

<details>
<summary><small>Click to expand</small></summary>

PyForge 0.23 introduces a new `IntoPyObject` trait to convert Rust types into Python objects which replaces both `IntoPy` and `ToPyObject`.
Notable features of this new trait include:

- conversions can now return an error
- it is designed to work efficiently for both `T` owned types and `&T` references
- compared to `IntoPy<T>` the generic `T` moved into an associated type, so
  - there is now only one way to convert a given type
  - the output type is stronger typed and may return any Python type instead of just `PyAny`
- byte collections are specialized to convert into `PyBytes` now, see [below](#to-python-conversions-changed-for-byte-collections-vecu8-u8-n-and-smallvecu8-n)
- `()` (unit) is now only specialized in return position of `#[pyfunction]` and `#[pymethods]` to return `None`, in normal usage it converts into an empty `PyTuple`

All PyForge provided types as well as `#[pyclass]`es already implement `IntoPyObject`.
Other types will need to adapt an implementation of `IntoPyObject` to stay compatible with the Python APIs.
In many cases the new [`#[derive(IntoPyObject)]`](#intopyobject-and-intopyobjectref-derive-macros) macro can be used instead of [manual implementations](#intopyobject-manual-implementation).

Since `IntoPyObject::into_pyobject` may return either a `Bound` or `Borrowed`, you may find the [`BoundObject`](conversions/traits.md#boundobject-for-conversions-that-may-be-bound-or-borrowed) trait to be useful to write code that generically handles either type of smart pointer.

Together with the introduction of `IntoPyObject` the old conversion traits `ToPyObject` and `IntoPy` are deprecated and will be removed in a future PyForge version.

#### `IntoPyObject` and `IntoPyObjectRef` derive macros

To implement the new trait you may use the new `IntoPyObject` and `IntoPyObjectRef` derive macros as below.

```rust,no_run
# use pyforge::prelude::*;
#[derive(IntoPyObject, IntoPyObjectRef)]
struct Struct {
    count: usize,
    obj: Py<PyAny>,
}
```

The `IntoPyObjectRef` derive macro derives implementations for references (e.g. for `&Struct` in the example above), which is a replacement for the `ToPyObject` trait.

#### `IntoPyObject` manual implementation

Before:

```rust,ignore
# use pyforge::prelude::*;
# #[allow(dead_code)]
struct MyPyObjectWrapper(PyObject);

impl IntoPy<PyObject> for MyPyObjectWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0
    }
}

impl ToPyObject for MyPyObjectWrapper {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.0.clone_ref(py)
    }
}
```

After:

```rust,ignore
# use pyforge::prelude::*;
# #[allow(dead_code)]
# struct MyPyObjectWrapper(PyObject);

impl<'py> IntoPyObject<'py> for MyPyObjectWrapper {
    type Target = PyAny; // the Python type
    type Output = Bound<'py, Self::Target>; // in most cases this will be `Bound`
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.into_bound(py))
    }
}

// `ToPyObject` implementations should be converted to implementations on reference types
impl<'a, 'py> IntoPyObject<'py> for &'a MyPyObjectWrapper {
    type Target = PyAny;
    type Output = Borrowed<'a, 'py, Self::Target>; // `Borrowed` can be used to optimized reference counting
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.bind_borrowed(py))
    }
}
```

</details>

### To-Python conversions changed for byte collections (`Vec<u8>`, `[u8; N]` and `SmallVec<[u8; N]>`)

<details>
<summary><small>Click to expand</small></summary>

With the introduction of the `IntoPyObject` trait, PyForge's macros now prefer `IntoPyObject` implementations over `IntoPy<PyObject>` when producing Python values.
This applies to `#[pyfunction]` and `#[pymethods]` return values and also fields accessed via `#[pyforge(get)]`.

This change has an effect on functions and methods returning _byte_ collections like

- `Vec<u8>`
- `[u8; N]`
- `SmallVec<[u8; N]>`

In their new `IntoPyObject` implementation these will now turn into `PyBytes` rather than a `PyList`.
All other `T`s are unaffected and still convert into a `PyList`.

```rust,no_run
# #![allow(dead_code)]
# use pyforge::prelude::*;
#[pyfunction]
fn foo() -> Vec<u8> { // would previously turn into a `PyList`, now `PyBytes`
    vec![0, 1, 2, 3]
}

#[pyfunction]
fn bar() -> Vec<u16> { // unaffected, returns `PyList`
    vec![0, 1, 2, 3]
}
```

If this conversion is _not_ desired, consider building a list manually using `PyList::new`.

The following types were previously _only_ implemented for `u8` and now allow other `T`s turn into `PyList`:

- `&[T]`
- `Cow<[T]>`

This is purely additional and should just extend the possible return types.

</details>

### `gil-refs` feature removed

<details>
<summary><small>Click to expand</small></summary>

PyForge 0.23 completes the removal of the "GIL Refs" API in favour of the new "Bound" API introduced in PyForge 0.21.

With the removal of the old API, many "Bound" API functions which had been introduced with `_bound` suffixes no longer need the suffixes as these names have been freed up.
For example, `PyTuple::new_bound` is now just `PyTuple::new` (the existing name remains but is deprecated).

Before:

```rust,ignore
# #![allow(deprecated)]
# use pyforge::prelude::*;
# use pyforge::types::PyTuple;
# fn main() {
# Python::attach(|py| {
// For example, for PyTuple. Many such APIs have been changed.
let tup = PyTuple::new_bound(py, [1, 2, 3]);
# })
# }
```

After:

```rust
# use pyforge::prelude::*;
# use pyforge::types::PyTuple;
# fn main() {
# Python::attach(|py| {
// For example, for PyTuple. Many such APIs have been changed.
let tup = PyTuple::new(py, [1, 2, 3]);
# })
# }
```

#### `IntoPyDict` trait adjusted for removal of `gil-refs`

As part of this API simplification, the `IntoPyDict` trait has had a small breaking change: `IntoPyDict::into_py_dict_bound` method has been renamed to `IntoPyDict::into_py_dict`.
It is also now fallible as part of the `IntoPyObject` trait addition.

If you implemented `IntoPyDict` for your type, you should implement `into_py_dict` instead of `into_py_dict_bound`.
The old name is still available for calling but deprecated.

Before:

```rust,ignore
# use pyforge::prelude::*;
# use pyforge::types::{PyDict, IntoPyDict};
# use std::collections::HashMap;

struct MyMap<K, V>(HashMap<K, V>);

impl<K, V> IntoPyDict for MyMap<K, V>
where
    K: ToPyObject,
    V: ToPyObject,
{
    fn into_py_dict_bound(self, py: Python<'_>) -> Bound<'_, PyDict> {
        let dict = PyDict::new_bound(py);
        for (key, value) in self.0 {
            dict.set_item(key, value)
                .expect("Failed to set_item on dict");
        }
        dict
    }
}
```

After:

```rust,no_run
# use pyforge::prelude::*;
# use pyforge::types::{PyDict, IntoPyDict};
# use std::collections::HashMap;

# #[allow(dead_code)]
struct MyMap<K, V>(HashMap<K, V>);

impl<'py, K, V> IntoPyDict<'py> for MyMap<K, V>
where
    K: IntoPyObject<'py>,
    V: IntoPyObject<'py>,
{
    fn into_py_dict(self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in self.0 {
            dict.set_item(key, value)?;
        }
        Ok(dict)
    }
}
```

</details>

## from 0.21.* to 0.22

### Deprecation of `gil-refs` feature continues

<details>
<summary><small>Click to expand</small></summary>

Following the introduction of the "Bound" API in PyForge 0.21 and the planned removal of the "GIL Refs" API, all functionality related to GIL Refs is now gated behind the `gil-refs` feature and emits a deprecation warning on use.

See <a href="#from-021-to-022">the 0.21 migration entry</a> for help upgrading.
</details>

### Deprecation of implicit default for trailing optional arguments

<details>
<summary><small>Click to expand</small></summary>

With `pyforge` 0.22 the implicit `None` default for trailing `Option<T>` type argument is deprecated.
To migrate, place a `#[pyforge(signature = (...))]` attribute on affected functions or methods and specify the desired behavior.
The migration warning specifies the corresponding signature to keep the current behavior.
With 0.23 the signature will be required for any function containing `Option<T>` type parameters to prevent accidental and unnoticed changes in behavior.
With 0.24 this restriction will be lifted again and `Option<T>` type arguments will be treated as any other argument _without_ special handling.

Before:

```rust,no_run
# #![allow(deprecated, dead_code)]
# use pyforge::prelude::*;
#[pyfunction]
fn increment(x: u64, amount: Option<u64>) -> u64 {
    x + amount.unwrap_or(1)
}
```

After:

```rust,no_run
# #![allow(dead_code)]
# use pyforge::prelude::*;
#[pyfunction]
#[pyforge(signature = (x, amount=None))]
fn increment(x: u64, amount: Option<u64>) -> u64 {
    x + amount.unwrap_or(1)
}
```

</details>

### `Py::clone` is now gated behind the `py-clone` feature

<details>
<summary><small>Click to expand</small></summary>

If you rely on `impl<T> Clone for Py<T>` to fulfil trait requirements imposed by existing Rust code written without PyForge-based code in mind, the newly introduced feature `py-clone` must be enabled.

However, take care to note that the behaviour is different from previous versions.
If `Clone` was called without the GIL being held, we tried to delay the application of these reference count increments until PyForge-based code would re-acquire it.
This turned out to be impossible to implement in a sound manner and hence was removed.
Now, if `Clone` is called without the GIL being held, we panic instead for which calling code might not be prepared.

It is advised to migrate off the `py-clone` feature.
The simplest way to remove dependency on `impl<T> Clone for Py<T>` is to wrap `Py<T>` as `Arc<Py<T>>` and use cloning of the arc.

Related to this, we also added a `pyo3_disable_reference_pool` conditional compilation flag which removes the infrastructure necessary to apply delayed reference count decrements implied by `impl<T> Drop for Py<T>`.
They do not appear to be a soundness hazard as they should lead to memory leaks in the worst case.
However, the global synchronization adds significant overhead to cross the Python-Rust boundary.
Enabling this feature will remove these costs and make the `Drop` implementation abort the process if called without the GIL being held instead.
</details>

### Require explicit opt-in for comparison for simple enums

<details>
<summary><small>Click to expand</small></summary>

With `pyforge` 0.22 the new `#[pyforge(eq)]` options allows automatic implementation of Python equality using Rust's `PartialEq`.
Previously simple enums automatically implemented equality in terms of their discriminants.
To make PyForge more consistent, this automatic equality implementation is deprecated in favour of having opt-ins for all `#[pyclass]` types.
Similarly, simple enums supported comparison with integers, which is not covered by Rust's `PartialEq` derive, so has been split out into the `#[pyforge(eq_int)]` attribute.

To migrate, place a `#[pyforge(eq, eq_int)]` attribute on simple enum classes.

Before:

```rust,no_run
# #![allow(deprecated, dead_code)]
# use pyforge::prelude::*;
#[pyclass]
enum SimpleEnum {
    VariantA,
    VariantB = 42,
}
```

After:

```rust,no_run
# #![allow(dead_code)]
# use pyforge::prelude::*;
#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
enum SimpleEnum {
    VariantA,
    VariantB = 42,
}
```

</details>

### `PyType::name` reworked to better match Python `__name__`

<details>
<summary><small>Click to expand</small></summary>

This function previously would try to read directly from Python type objects' C API field (`tp_name`), in which case it would return a `Cow::Borrowed`.
However the contents of `tp_name` don't have well-defined semantics.

Instead `PyType::name()` now returns the equivalent of Python `__name__` and returns `PyResult<Bound<'py, PyString>>`.

The closest equivalent to PyForge 0.21's version of `PyType::name()` has been introduced as a new function `PyType::fully_qualified_name()`, which is equivalent to `__module__` and `__qualname__` joined as `module.qualname`.

Before:

```rust,ignore
# #![allow(deprecated, dead_code)]
# use pyforge::prelude::*;
# use pyforge::types::{PyBool};
# fn main() -> PyResult<()> {
Python::with_gil(|py| {
    let bool_type = py.get_type_bound::<PyBool>();
    let name = bool_type.name()?.into_owned();
    println!("Hello, {}", name);

    let mut name_upper = bool_type.name()?;
    name_upper.to_mut().make_ascii_uppercase();
    println!("Hello, {}", name_upper);

    Ok(())
})
# }
```

After:

```rust,ignore
# #![allow(dead_code)]
# use pyforge::prelude::*;
# use pyforge::types::{PyBool};
# fn main() -> PyResult<()> {
Python::with_gil(|py| {
    let bool_type = py.get_type_bound::<PyBool>();
    let name = bool_type.name()?;
    println!("Hello, {}", name);

    // (if the full dotted path was desired, switch from `name()` to `fully_qualified_name()`)
    let mut name_upper = bool_type.fully_qualified_name()?.to_string();
    name_upper.make_ascii_uppercase();
    println!("Hello, {}", name_upper);

    Ok(())
})
# }
```

</details>


---

_Migration guides for versions 0.20 and older have been removed. For historical migration guides, see the [PyForge upstream documentation](https://pyforge.rs/latest/migration.html)._
