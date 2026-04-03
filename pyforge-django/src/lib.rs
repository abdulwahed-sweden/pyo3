// Author: Abdulwahed Mansour
//! # pyforge-django
//!
//! High-performance Django integration layer built on PyForge.
//!
//! Provides Rust-accelerated serialization, validation, and field mapping
//! for Django 5.x+ projects. Designed for drop-in use with Django REST Framework.

pub mod async_bridge;
pub mod error;
pub mod field_types;
pub mod model;
pub mod serializer;
pub mod validator;

use pyforge::prelude::*;
use pyforge::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};

use crate::error::DjangoError;
use crate::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};

// ─── ModelSchema: compiled descriptor cache ─────────────────────────────────

/// A compiled model schema that caches field descriptors for repeated use.
///
/// Built once at Django startup from a model class, then reused on every
/// request. Eliminates per-request descriptor parsing overhead entirely.
///
/// Usage from Python:
/// ```python
/// schema = pyforge_django.ModelSchema(MyModel)
/// result = pyforge_django.serialize_instance(instance, schema)
/// ```
#[pyclass]
#[derive(Clone)]
pub struct ModelSchema {
    /// Cached field descriptors extracted from the Django model.
    descriptors: Vec<FieldDescriptor>,
    /// The model class name, for error messages.
    model_name: String,
    /// Field names in declaration order, for fast lookup.
    field_names: Vec<String>,
}

#[pymethods]
impl ModelSchema {
    /// Compiles a schema from a Django model class.
    ///
    /// Introspects the model's `_meta` API once and caches the result.
    /// Subsequent serialization/validation calls use the cached descriptors
    /// with zero per-request parsing overhead.
    ///
    /// Args:
    ///     model_class: A Django model class (e.g., `MyModel`).
    ///
    /// Raises:
    ///     ValueError: If the class lacks a `_meta` attribute.
    #[new]
    fn new(py: Python<'_>, model_class: &Bound<'_, PyAny>) -> PyResult<Self> {
        let model_name: String = model_class
            .getattr("__name__")
            .and_then(|n| n.extract())
            .unwrap_or_else(|_| "UnknownModel".into());

        let descriptors = model::extract_field_descriptors(py, model_class)
            .map_err(|e| -> pyforge::PyErr { e.into() })?;

        let field_names = descriptors.iter().map(|d| d.name.clone()).collect();

        Ok(ModelSchema {
            descriptors,
            model_name,
            field_names,
        })
    }

    /// Returns the list of field names in this schema.
    #[getter]
    fn field_names_list(&self) -> Vec<String> {
        self.field_names.clone()
    }

    /// Returns the model class name.
    #[getter]
    fn model_name_str(&self) -> &str {
        &self.model_name
    }

    /// Returns the number of fields in the schema.
    fn __len__(&self) -> usize {
        self.descriptors.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "ModelSchema({}, {} fields)",
            self.model_name,
            self.descriptors.len()
        )
    }

    /// Returns the schema as a list of dicts (for compatibility with existing APIs).
    fn to_descriptor_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        descriptors_to_pylist(py, &self.descriptors)
    }
}

// ─── Python-exposed functions ───────────────────────────────────────────────

/// Serializes a single Django model instance using a precompiled schema.
///
/// This is the primary fast path: extracts all field values from the instance
/// in a single Rust call via `getattr`, serializes them, and returns a dict.
/// Eliminates multiple Python↔Rust boundary crossings.
///
/// Args:
///     instance: A Django model instance (e.g., `my_obj`).
///     schema: A `ModelSchema` compiled from the model class.
///
/// Returns:
///     A dict of serialized field values (JSON-compatible).
///
/// Raises:
///     ValueError: On serialization failure.
///     TypeError: On type conversion failure.
#[pyfunction]
fn serialize_instance<'py>(
    py: Python<'py>,
    instance: &Bound<'py, PyAny>,
    schema: &ModelSchema,
) -> PyResult<Bound<'py, PyDict>> {
    let mut field_values = Vec::with_capacity(schema.descriptors.len());

    // Single-pass field extraction — all getattr calls happen here, in Rust
    for desc in &schema.descriptors {
        let py_val = instance.getattr(desc.name.as_str());
        match py_val {
            Ok(val) => {
                let fv = model::convert_python_value_to_field(&val, desc)
                    .map_err(|e| -> pyforge::PyErr { e.into() })?;
                field_values.push(fv);
            }
            Err(_) => {
                if desc.nullable || desc.has_default {
                    field_values.push(FieldValue::Null);
                } else {
                    return Err(DjangoError::NullField {
                        field: desc.name.clone(),
                    }
                    .into());
                }
            }
        }
    }

    let record = serializer::serialize_model_fields(&schema.descriptors, &field_values)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;

    record_to_pydict(py, &record)
}

/// Serializes a batch of Django model instances using a precompiled schema.
///
/// Processes all instances in a single call, returning a list of dicts.
/// For batches above 64 records, consider using `async_serialize_batch`
/// from an ASGI view to release the GIL during computation.
///
/// Args:
///     instances: A list/queryset of Django model instances.
///     schema: A `ModelSchema` compiled from the model class.
///
/// Returns:
///     A list of dicts, one per instance.
#[pyfunction]
fn serialize_batch<'py>(
    py: Python<'py>,
    instances: &Bound<'py, PyList>,
    schema: &ModelSchema,
) -> PyResult<Bound<'py, PyList>> {
    let result = PyList::empty(py);

    for instance in instances.iter() {
        let dict = serialize_instance(py, &instance, schema)?;
        result.append(dict)?;
    }

    Ok(result)
}

/// Validates a Django model instance against its schema.
///
/// Extracts field values and runs them through the Rust validator.
/// For batches above 64 fields, validation runs in parallel via Rayon.
///
/// Args:
///     instance: A Django model instance.
///     schema: A `ModelSchema` compiled from the model class.
///
/// Returns:
///     A dict with `valid_count`, `error_count`, `errors`, and `is_valid`.
#[pyfunction]
fn validate_instance<'py>(
    py: Python<'py>,
    instance: &Bound<'py, PyAny>,
    schema: &ModelSchema,
) -> PyResult<Bound<'py, PyDict>> {
    let mut batch = Vec::with_capacity(schema.descriptors.len());

    for desc in &schema.descriptors {
        let py_val = instance.getattr(desc.name.as_str());
        let fv = match py_val {
            Ok(val) if !val.is_none() => model::convert_python_value_to_field(&val, desc)
                .map_err(|e| -> pyforge::PyErr { e.into() })?,
            _ => FieldValue::Null,
        };
        batch.push((desc.clone(), fv));
    }

    let report = validator::validate_field_batch(&batch);
    validation_report_to_pydict(py, &report)
}

/// Extracts field descriptors from a Django model class.
///
/// Returns a list of dicts. For repeated use, prefer `ModelSchema(model_class)`
/// which caches the result.
#[pyfunction]
fn extract_model_fields<'py>(
    py: Python<'py>,
    model_class: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyList>> {
    let descriptors = model::extract_field_descriptors(py, model_class)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;
    descriptors_to_pylist(py, &descriptors)
}

/// Serializes field values from a dict (legacy API — prefer `serialize_instance`).
#[pyfunction]
fn serialize_fields<'py>(
    py: Python<'py>,
    field_descriptors: &Bound<'py, PyList>,
    values: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyDict>> {
    let descriptors = extract_descriptor_list(field_descriptors)?;

    let mut field_values = Vec::with_capacity(descriptors.len());
    for desc in &descriptors {
        let py_val = values.get_item(&desc.name)?;
        match py_val {
            Some(val) => {
                let fv = model::convert_python_value_to_field(&val, desc)
                    .map_err(|e| -> pyforge::PyErr { e.into() })?;
                field_values.push(fv);
            }
            None => {
                if desc.nullable || desc.has_default {
                    field_values.push(FieldValue::Null);
                } else {
                    return Err(DjangoError::NullField {
                        field: desc.name.clone(),
                    }
                    .into());
                }
            }
        }
    }

    let record = serializer::serialize_model_fields(&descriptors, &field_values)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;
    record_to_pydict(py, &record)
}

/// Validates field values from a list (legacy API — prefer `validate_instance`).
#[pyfunction]
fn validate_fields<'py>(
    py: Python<'py>,
    descriptors: &Bound<'py, PyList>,
    values: &Bound<'py, PyList>,
) -> PyResult<Bound<'py, PyDict>> {
    let descs = extract_descriptor_list(descriptors)?;

    let mut batch = Vec::with_capacity(descs.len());
    for (i, desc) in descs.into_iter().enumerate() {
        let py_val = values.get_item(i)?;
        let fv = if py_val.is_none() {
            FieldValue::Null
        } else {
            model::convert_python_value_to_field(&py_val, &desc)
                .map_err(|e| -> pyforge::PyErr { e.into() })?
        };
        batch.push((desc, fv));
    }

    let report = validator::validate_field_batch(&batch);
    validation_report_to_pydict(py, &report)
}

/// Returns the pyforge-django version string.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The `pyforge_django` Python module.
#[pymodule]
fn pyforge_django(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ModelSchema>()?;
    m.add_function(wrap_pyfunction!(serialize_instance, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_batch, m)?)?;
    m.add_function(wrap_pyfunction!(validate_instance, m)?)?;
    m.add_function(wrap_pyfunction!(extract_model_fields, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_fields, m)?)?;
    m.add_function(wrap_pyfunction!(validate_fields, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

// ─── Internal helpers ───────────────────────────────────────────────────────

/// Converts a serialized record to a Python dict.
fn record_to_pydict<'py>(
    py: Python<'py>,
    record: &serde_json::Map<String, serde_json::Value>,
) -> PyResult<Bound<'py, PyDict>> {
    let output = PyDict::new(py);
    for (key, val) in record {
        let py_val = json_value_to_pyobject(py, val)?;
        output.set_item(key, py_val)?;
    }
    Ok(output)
}

/// Converts a ValidationReport to a Python dict.
fn validation_report_to_pydict<'py>(
    py: Python<'py>,
    report: &validator::ValidationReport,
) -> PyResult<Bound<'py, PyDict>> {
    let result = PyDict::new(py);
    result.set_item("valid_count", report.valid_count)?;
    result.set_item("error_count", report.error_count)?;
    result.set_item("is_valid", report.is_valid())?;

    let errors = PyList::empty(py);
    for err in &report.field_errors {
        let err_dict = PyDict::new(py);
        err_dict.set_item("field", &err.field_name)?;
        err_dict.set_item("message", &err.message)?;
        err_dict.set_item("code", &err.code)?;
        let params = PyDict::new(py);
        for (k, v) in &err.params {
            params.set_item(k, v)?;
        }
        err_dict.set_item("params", params)?;
        errors.append(err_dict)?;
    }
    result.set_item("errors", errors)?;

    Ok(result)
}

/// Converts descriptors to a Python list of dicts.
fn descriptors_to_pylist<'py>(
    py: Python<'py>,
    descriptors: &[FieldDescriptor],
) -> PyResult<Bound<'py, PyList>> {
    let list = PyList::empty(py);
    for desc in descriptors {
        let dict = PyDict::new(py);
        dict.set_item("name", &desc.name)?;
        dict.set_item("type", desc.field_type.django_type_name())?;
        dict.set_item("nullable", desc.nullable)?;
        dict.set_item("has_default", desc.has_default)?;
        match &desc.field_type {
            DjangoFieldType::CharField { max_length }
            | DjangoFieldType::EmailField { max_length }
            | DjangoFieldType::UrlField { max_length }
            | DjangoFieldType::SlugField { max_length } => {
                dict.set_item("max_length", *max_length)?;
            }
            DjangoFieldType::DecimalField {
                max_digits,
                decimal_places,
            } => {
                dict.set_item("max_digits", *max_digits)?;
                dict.set_item("decimal_places", *decimal_places)?;
            }
            DjangoFieldType::BinaryField {
                max_length: Some(ml),
            } => {
                dict.set_item("max_length", *ml)?;
            }
            _ => {}
        }
        list.append(dict)?;
    }
    Ok(list)
}

/// Extracts a list of `FieldDescriptor` from a Python list of dicts.
fn extract_descriptor_list(py_list: &Bound<'_, PyList>) -> PyResult<Vec<FieldDescriptor>> {
    let mut descriptors = Vec::with_capacity(py_list.len());

    for item in py_list.iter() {
        let name: String = item.get_item("name")?.extract()?;
        let field_type_str: String = item.get_item("type")?.extract()?;
        let nullable: bool = item.get_item("nullable")?.extract()?;
        let has_default: bool = item.get_item("has_default")?.extract()?;

        let max_length: Option<usize> = item
            .get_item("max_length")
            .ok()
            .and_then(|v| v.extract().ok());
        let max_digits: Option<u32> = item
            .get_item("max_digits")
            .ok()
            .and_then(|v| v.extract().ok());
        let decimal_places: Option<u32> = item
            .get_item("decimal_places")
            .ok()
            .and_then(|v| v.extract().ok());

        let field_type = parse_field_type_str(&field_type_str, max_length, max_digits, decimal_places);

        descriptors.push(FieldDescriptor {
            name,
            field_type,
            nullable,
            has_default,
        });
    }

    Ok(descriptors)
}

/// Parses a Django field type name string into a `DjangoFieldType`.
fn parse_field_type_str(
    s: &str,
    max_length: Option<usize>,
    max_digits: Option<u32>,
    decimal_places: Option<u32>,
) -> DjangoFieldType {
    match s {
        "CharField" => DjangoFieldType::CharField {
            max_length: max_length.unwrap_or(255),
        },
        "TextField" => DjangoFieldType::TextField,
        "IntegerField" => DjangoFieldType::IntegerField,
        "BigIntegerField" => DjangoFieldType::BigIntegerField,
        "FloatField" => DjangoFieldType::FloatField,
        "DecimalField" => DjangoFieldType::DecimalField {
            max_digits: max_digits.unwrap_or(10),
            decimal_places: decimal_places.unwrap_or(2),
        },
        "BooleanField" => DjangoFieldType::BooleanField,
        "DateField" => DjangoFieldType::DateField,
        "TimeField" => DjangoFieldType::TimeField,
        "DateTimeField" => DjangoFieldType::DateTimeField,
        "UUIDField" => DjangoFieldType::UuidField,
        "JSONField" => DjangoFieldType::JsonField,
        "BinaryField" => DjangoFieldType::BinaryField { max_length },
        "EmailField" => DjangoFieldType::EmailField {
            max_length: max_length.unwrap_or(254),
        },
        "URLField" => DjangoFieldType::UrlField {
            max_length: max_length.unwrap_or(200),
        },
        "SlugField" => DjangoFieldType::SlugField {
            max_length: max_length.unwrap_or(50),
        },
        _ => DjangoFieldType::TextField,
    }
}

/// Converts a `serde_json::Value` into a Python object.
fn json_value_to_pyobject<'py>(
    py: Python<'py>,
    value: &serde_json::Value,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => Ok(PyBool::new(py, *b).to_owned().into_any()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(PyInt::new(py, i).into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into_any())
            } else {
                Ok(PyString::new(py, &n.to_string()).into_any())
            }
        }
        serde_json::Value::String(s) => Ok(PyString::new(py, s).into_any()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_pyobject(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_pyobject(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}
