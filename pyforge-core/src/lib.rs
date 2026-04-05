// Author: Abdulwahed Mansour
//! # pyforge-core
//!
//! Rust-accelerated serialization and validation for Python.
//! Framework-agnostic — works with any Python project.
//!
//! This crate provides the core engine that pyforge-django delegates to.
//! It can also be used standalone with Flask, FastAPI, scripts, or any
//! Python code that processes structured data.

pub mod engine_serialize;
pub mod engine_validate;
pub mod error;
pub mod types;

// Re-export public types for downstream crates (pyforge-django).
pub use engine_serialize::{serialize_fields, serialize_rows, SerializedRecord};
pub use engine_validate::{validate_batch, ValidationReport, PARALLEL_THRESHOLD};
pub use error::{CoreError, FieldValidationError};
pub use types::{FieldDescriptor, FieldType, FieldValue};

use pyforge::prelude::*;
use pyforge::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

// ─── Schema: compiled field descriptor cache ──────────────────────────────────

/// A compiled schema that caches field descriptors for repeated use.
///
/// Built once from a dict of field definitions, then reused on every
/// serialize/validate call with zero per-call parsing overhead.
///
/// Python usage:
/// ```python
/// from pyforge_core import Schema, Field
/// schema = Schema({"name": Field(str, max_length=100), "age": Field(int)})
/// ```
#[pyclass]
#[derive(Clone)]
pub struct Schema {
    descriptors: Vec<FieldDescriptor>,
    field_names: Vec<String>,
}

#[pymethods]
impl Schema {
    /// Compiles a schema from a dict of field definitions.
    ///
    /// Each key is a field name, each value is a `Field` instance
    /// describing the type and constraints.
    #[new]
    fn new(fields: &Bound<'_, PyDict>) -> PyResult<Self> {
        let mut descriptors = Vec::with_capacity(fields.len());
        let mut field_names = Vec::with_capacity(fields.len());

        for (key, value) in fields.iter() {
            let name: String = key.extract()?;
            let field: Field = value.extract()?;

            field_names.push(name.clone());
            descriptors.push(FieldDescriptor {
                name,
                field_type: field.field_type,
                nullable: field.nullable,
                has_default: field.has_default,
            });
        }

        Ok(Schema {
            descriptors,
            field_names,
        })
    }

    /// Returns the list of field names in declaration order.
    #[getter]
    fn field_names_list(&self) -> Vec<String> {
        self.field_names.clone()
    }

    /// Returns the number of fields in the schema.
    fn __len__(&self) -> usize {
        self.descriptors.len()
    }

    fn __repr__(&self) -> String {
        format!("Schema({} fields)", self.descriptors.len())
    }
}

// ─── Field: single field definition ───────────────────────────────────────────

/// Defines a single field's type and constraints.
///
/// Python usage:
/// ```python
/// from pyforge_core import Field
/// from decimal import Decimal
/// from datetime import datetime
///
/// Field(str, max_length=100)
/// Field(int, min_value=0, max_value=150)
/// Field(Decimal, max_digits=10, decimal_places=2)
/// Field(datetime)
/// Field(str, nullable=True)
/// ```
#[pyclass]
#[derive(Clone)]
pub struct Field {
    field_type: FieldType,
    nullable: bool,
    has_default: bool,
}

#[pymethods]
impl Field {
    /// Creates a new field definition.
    ///
    /// Args:
    ///     python_type: The Python type (str, int, float, bool, Decimal, datetime, date, time, UUID, list, dict, bytes).
    ///     max_length: Maximum string length or byte length.
    ///     min_length: Minimum string length.
    ///     min_value: Minimum numeric value (int or float).
    ///     max_value: Maximum numeric value (int or float).
    ///     max_digits: Maximum total digits for Decimal.
    ///     decimal_places: Maximum decimal places for Decimal.
    ///     nullable: Whether None is allowed (default False).
    ///     default: Whether the field has a default value (default False).
    #[new]
    #[pyo3(signature = (python_type, *, max_length=None, min_length=None, min_value=None, max_value=None, max_digits=None, decimal_places=None, nullable=false, default=false))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        python_type: &Bound<'_, PyAny>,
        max_length: Option<usize>,
        min_length: Option<usize>,
        min_value: Option<&Bound<'_, PyAny>>,
        max_value: Option<&Bound<'_, PyAny>>,
        max_digits: Option<u32>,
        decimal_places: Option<u32>,
        nullable: bool,
        default: bool,
    ) -> PyResult<Self> {
        let type_name = python_type
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_default();

        let field_type = match type_name.as_str() {
            "str" => FieldType::Str { max_length, min_length },
            "int" => FieldType::Int {
                min_value: extract_opt_i64(min_value)?,
                max_value: extract_opt_i64(max_value)?,
            },
            "float" => FieldType::Float {
                min_value: extract_opt_f64(min_value)?,
                max_value: extract_opt_f64(max_value)?,
            },
            "bool" => FieldType::Bool,
            "Decimal" => FieldType::Decimal { max_digits, decimal_places },
            "datetime" => FieldType::DateTime,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            "UUID" => FieldType::Uuid,
            "list" => FieldType::List,
            "dict" => FieldType::Dict,
            "bytes" => FieldType::Bytes { max_length },
            other => {
                return Err(CoreError::SchemaError {
                    message: format!("unsupported type: {other}"),
                }
                .into());
            }
        };

        Ok(Field {
            field_type,
            nullable,
            has_default: default,
        })
    }

    fn __repr__(&self) -> String {
        format!("Field({}, nullable={})", self.field_type.type_name(), self.nullable)
    }
}

fn extract_opt_i64(val: Option<&Bound<'_, PyAny>>) -> PyResult<Option<i64>> {
    match val {
        Some(v) => Ok(Some(v.extract::<i64>()?)),
        None => Ok(None),
    }
}

fn extract_opt_f64(val: Option<&Bound<'_, PyAny>>) -> PyResult<Option<f64>> {
    match val {
        Some(v) => Ok(Some(v.extract::<f64>()?)),
        None => Ok(None),
    }
}

// ─── Python-exposed functions ─────────────────────────────────────────────────

/// Serializes a Python dict or object using a precompiled schema.
///
/// Accepts either a dict (keys are field names) or any object with
/// attributes matching the schema field names.
///
/// Returns a dict of serialized field values (JSON-compatible).
#[pyfunction]
fn serialize<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    schema: &Schema,
) -> PyResult<Bound<'py, PyDict>> {
    let values = extract_values(py, data, &schema.descriptors)?;
    let record = serialize_fields(&schema.descriptors, &values)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;
    record_to_pydict(py, &record)
}

/// Serializes a list of dicts or objects using a precompiled schema.
///
/// Returns a list of dicts.
#[pyfunction]
fn serialize_many<'py>(
    py: Python<'py>,
    data_list: &Bound<'py, PyList>,
    schema: &Schema,
) -> PyResult<Bound<'py, PyList>> {
    let result = PyList::empty(py);
    for item in data_list.iter() {
        let dict = serialize(py, &item, schema)?;
        result.append(dict)?;
    }
    Ok(result)
}

/// Validates a Python dict or object against a schema.
///
/// Returns a dict with `is_valid`, `valid_count`, `error_count`, and `errors`.
#[pyfunction]
fn validate<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    schema: &Schema,
) -> PyResult<Bound<'py, PyDict>> {
    let mut batch = Vec::with_capacity(schema.descriptors.len());
    for desc in &schema.descriptors {
        let fv = extract_single_value(py, data, desc)?;
        batch.push((desc.clone(), fv));
    }
    let report = validate_batch(&batch);
    report_to_pydict(py, &report)
}

/// Validates a list of dicts or objects against a schema.
///
/// Returns a combined validation report.
#[pyfunction]
fn validate_many<'py>(
    py: Python<'py>,
    data_list: &Bound<'py, PyList>,
    schema: &Schema,
) -> PyResult<Bound<'py, PyDict>> {
    let mut all_entries = Vec::new();
    for item in data_list.iter() {
        for desc in &schema.descriptors {
            let fv = extract_single_value(py, &item, desc)?;
            all_entries.push((desc.clone(), fv));
        }
    }
    let report = validate_batch(&all_entries);
    report_to_pydict(py, &report)
}

/// Returns the pyforge-core version string.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The native extension module.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Schema>()?;
    m.add_class::<Field>()?;
    m.add_function(wrap_pyfunction!(serialize, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_many, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(validate_many, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

// ─── Value extraction from Python ─────────────────────────────────────────────

/// Extracts all field values from a Python dict or object.
fn extract_values<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    descriptors: &[FieldDescriptor],
) -> PyResult<Vec<FieldValue>> {
    let mut values = Vec::with_capacity(descriptors.len());
    for desc in descriptors {
        values.push(extract_single_value(py, data, desc)?);
    }
    Ok(values)
}

/// Extracts a single field value from a Python dict or object.
fn extract_single_value<'py>(
    _py: Python<'py>,
    data: &Bound<'py, PyAny>,
    desc: &FieldDescriptor,
) -> PyResult<FieldValue> {
    // Try dict access first, then attribute access
    let py_val = if let Ok(dict) = data.cast::<PyDict>() {
        dict.get_item(&desc.name)?
    } else {
        data.getattr(desc.name.as_str()).ok()
    };

    let py_val = match py_val {
        Some(v) if !v.is_none() => v,
        _ => {
            if desc.nullable || desc.has_default {
                return Ok(FieldValue::Null);
            }
            return Err(CoreError::NullField {
                field: desc.name.clone(),
            }
            .into());
        }
    };

    convert_py_to_field_value(&py_val, desc)
}

/// Converts a Python object to a FieldValue based on the field descriptor.
fn convert_py_to_field_value(
    val: &Bound<'_, PyAny>,
    desc: &FieldDescriptor,
) -> PyResult<FieldValue> {
    match &desc.field_type {
        FieldType::Str { .. } => {
            let s: String = val.extract()?;
            Ok(FieldValue::Text(s))
        }
        FieldType::Int { .. } => {
            let n: i64 = val.extract()?;
            Ok(FieldValue::Integer(n))
        }
        FieldType::Float { .. } => {
            let f: f64 = val.extract()?;
            Ok(FieldValue::Float(f))
        }
        FieldType::Bool => {
            let b: bool = val.extract()?;
            Ok(FieldValue::Boolean(b))
        }
        FieldType::Decimal { .. } => {
            // Python Decimal → string → rust_decimal::Decimal
            let s: String = val.str()?.extract()?;
            let d = Decimal::from_str_exact(&s).map_err(|e| CoreError::TypeError {
                field: desc.name.clone(),
                expected: "Decimal".into(),
                got: format!("invalid decimal string: {e}"),
            })?;
            Ok(FieldValue::Decimal(d))
        }
        FieldType::DateTime => {
            let iso: String = val.call_method0("isoformat")?.extract()?;
            let dt: DateTime<Utc> = iso
                .parse()
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(&iso, "%Y-%m-%dT%H:%M:%S%.f")
                        .or_else(|_| chrono::NaiveDateTime::parse_from_str(&iso, "%Y-%m-%dT%H:%M:%S"))
                        .map(|ndt| ndt.and_utc())
                })
                .map_err(|e| CoreError::TypeError {
                    field: desc.name.clone(),
                    expected: "datetime".into(),
                    got: format!("could not parse: {e}"),
                })?;
            Ok(FieldValue::DateTime(dt))
        }
        FieldType::Date => {
            let iso: String = val.call_method0("isoformat")?.extract()?;
            let d = NaiveDate::parse_from_str(&iso, "%Y-%m-%d").map_err(|e| {
                CoreError::TypeError {
                    field: desc.name.clone(),
                    expected: "date".into(),
                    got: format!("could not parse: {e}"),
                }
            })?;
            Ok(FieldValue::Date(d))
        }
        FieldType::Time => {
            let iso: String = val.call_method0("isoformat")?.extract()?;
            let t = NaiveTime::parse_from_str(&iso, "%H:%M:%S%.f")
                .or_else(|_| NaiveTime::parse_from_str(&iso, "%H:%M:%S"))
                .map_err(|e| CoreError::TypeError {
                    field: desc.name.clone(),
                    expected: "time".into(),
                    got: format!("could not parse: {e}"),
                })?;
            Ok(FieldValue::Time(t))
        }
        FieldType::Uuid => {
            let s: String = val.str()?.extract()?;
            let u = Uuid::parse_str(&s).map_err(|e| CoreError::TypeError {
                field: desc.name.clone(),
                expected: "UUID".into(),
                got: format!("invalid UUID: {e}"),
            })?;
            Ok(FieldValue::Uuid(u))
        }
        FieldType::List | FieldType::Dict => {
            // Convert Python list/dict → JSON via str(json.dumps())
            let json_str: String = {
                let json_mod = val.py().import("json")?;
                let dumped = json_mod.call_method1("dumps", (val,))?;
                dumped.extract()?
            };
            let json_val: serde_json::Value =
                serde_json::from_str(&json_str).map_err(|e| CoreError::TypeError {
                    field: desc.name.clone(),
                    expected: desc.field_type.type_name().into(),
                    got: format!("invalid JSON: {e}"),
                })?;
            Ok(FieldValue::Json(json_val))
        }
        FieldType::Bytes { .. } => {
            let b: Vec<u8> = val.extract()?;
            Ok(FieldValue::Binary(b))
        }
    }
}

// ─── Output conversion helpers ────────────────────────────────────────────────

/// Converts a serialized record to a Python dict.
fn record_to_pydict<'py>(
    py: Python<'py>,
    record: &serde_json::Map<String, serde_json::Value>,
) -> PyResult<Bound<'py, PyDict>> {
    let output = PyDict::new(py);
    for (key, val) in record {
        let py_val = json_to_py(py, val)?;
        output.set_item(key, py_val)?;
    }
    Ok(output)
}

/// Converts a ValidationReport to a Python dict.
fn report_to_pydict<'py>(
    py: Python<'py>,
    report: &ValidationReport,
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

/// Converts a serde_json::Value to a Python object.
fn json_to_py<'py>(py: Python<'py>, value: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
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
                list.append(json_to_py(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}
