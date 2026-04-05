// Author: Abdulwahed Mansour
//! Django model field extraction and type mapping.
//!
//! This module provides the bridge between Django's Python model `_meta` API
//! and the Rust-native `FieldDescriptor` system. It extracts field definitions
//! from a live Django model class and converts Python field values into
//! `FieldValue` variants for processing by the serializer and validator.

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use pyforge::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::error::DjangoError;
use crate::field_types::{DjangoFieldType, FieldDescriptor};
use pyforge_core::types::FieldValue;

/// Extracts field descriptors from a Django model class via its `_meta` API.
///
/// Introspects the model's `_meta.get_fields()` result and maps each concrete
/// field to a `FieldDescriptor`. Relation fields (ForeignKey, ManyToMany) are
/// skipped — they require separate handling via Django's ORM query layer.
///
/// # Arguments
/// * `model_class` - A Python reference to a Django model class (not an instance).
///
/// # Returns
/// A `Vec<FieldDescriptor>` for all concrete (non-relational) fields on the model,
/// or `DjangoError` if the class lacks a `_meta` attribute or introspection fails.
pub fn extract_field_descriptors(
    py: Python<'_>,
    model_class: &Bound<'_, PyAny>,
) -> Result<Vec<FieldDescriptor>, DjangoError> {
    let meta = model_class
        .getattr("_meta")
        .map_err(|_| DjangoError::Python("model class has no _meta attribute".into()))?;

    let fields = meta
        .call_method0("get_fields")
        .map_err(|e| DjangoError::Python(format!("_meta.get_fields() failed: {e}")))?;

    let mut descriptors = Vec::new();

    let fields_list: Vec<Bound<'_, PyAny>> = fields
        .extract()
        .map_err(|e| DjangoError::Python(format!("cannot iterate _meta.get_fields(): {e}")))?;

    for field in &fields_list {

        // Skip relation fields — they don't have a direct column representation
        let is_relation = field
            .getattr("is_relation")
            .and_then(|v| v.extract::<bool>())
            .unwrap_or(true);
        if is_relation {
            continue;
        }

        let name: String = field
            .getattr("name")
            .and_then(|v| v.extract())
            .map_err(|e| DjangoError::Python(format!("field missing 'name': {e}")))?;

        let internal_type: String = field
            .call_method0("get_internal_type")
            .and_then(|v| v.extract())
            .unwrap_or_default();

        let nullable: bool = field
            .getattr("null")
            .and_then(|v| v.extract())
            .unwrap_or(false);

        let has_default: bool = field
            .call_method0("has_default")
            .and_then(|v| v.extract())
            .unwrap_or(false);

        if let Some(field_type) = map_django_internal_type(py, field, &internal_type) {
            descriptors.push(FieldDescriptor {
                name,
                field_type,
                nullable,
                has_default,
            });
        }
    }

    Ok(descriptors)
}

/// Maps a Django field's `get_internal_type()` string to a `DjangoFieldType`.
///
/// Reads constraint attributes (max_length, max_digits, decimal_places) directly
/// from the Python field object.
fn map_django_internal_type(
    _py: Python<'_>,
    field: &Bound<'_, PyAny>,
    internal_type: &str,
) -> Option<DjangoFieldType> {
    match internal_type {
        "CharField" => {
            let max_length = extract_usize_attr(field, "max_length").unwrap_or(255);
            Some(DjangoFieldType::CharField { max_length })
        }
        "TextField" => Some(DjangoFieldType::TextField),
        "IntegerField" | "SmallIntegerField" | "PositiveIntegerField"
        | "PositiveSmallIntegerField" => Some(DjangoFieldType::IntegerField),
        "BigIntegerField" | "PositiveBigIntegerField" => Some(DjangoFieldType::BigIntegerField),
        "FloatField" => Some(DjangoFieldType::FloatField),
        "DecimalField" => {
            let max_digits = extract_u32_attr(field, "max_digits").unwrap_or(10);
            let decimal_places = extract_u32_attr(field, "decimal_places").unwrap_or(2);
            Some(DjangoFieldType::DecimalField {
                max_digits,
                decimal_places,
            })
        }
        "BooleanField" | "NullBooleanField" => Some(DjangoFieldType::BooleanField),
        "DateField" => Some(DjangoFieldType::DateField),
        "TimeField" => Some(DjangoFieldType::TimeField),
        "DateTimeField" => Some(DjangoFieldType::DateTimeField),
        "UUIDField" => Some(DjangoFieldType::UuidField),
        "JSONField" => Some(DjangoFieldType::JsonField),
        "BinaryField" => {
            let max_length = extract_usize_attr(field, "max_length");
            Some(DjangoFieldType::BinaryField { max_length })
        }
        "EmailField" => {
            let max_length = extract_usize_attr(field, "max_length").unwrap_or(254);
            Some(DjangoFieldType::EmailField { max_length })
        }
        "URLField" => {
            let max_length = extract_usize_attr(field, "max_length").unwrap_or(200);
            Some(DjangoFieldType::UrlField { max_length })
        }
        "SlugField" => {
            let max_length = extract_usize_attr(field, "max_length").unwrap_or(50);
            Some(DjangoFieldType::SlugField { max_length })
        }
        // AutoField, BigAutoField, and relation fields are handled elsewhere
        _ => None,
    }
}

fn extract_usize_attr(obj: &Bound<'_, PyAny>, attr: &str) -> Option<usize> {
    obj.getattr(attr).ok()?.extract::<usize>().ok()
}

fn extract_u32_attr(obj: &Bound<'_, PyAny>, attr: &str) -> Option<u32> {
    obj.getattr(attr).ok()?.extract::<u32>().ok()
}

/// Converts a Python object from a Django model instance into a `FieldValue`.
///
/// Inspects the `FieldDescriptor` to determine the expected type, then attempts
/// extraction. Returns `FieldValue::Null` for Python `None`, and `DjangoError`
/// for type mismatches that cannot be resolved.
///
/// # Arguments
/// * `py_value` - The raw Python object from `getattr(instance, field_name)`.
/// * `descriptor` - The field descriptor specifying the expected type.
pub fn convert_python_value_to_field(
    py_value: &Bound<'_, PyAny>,
    descriptor: &FieldDescriptor,
) -> Result<FieldValue, DjangoError> {
    if py_value.is_none() {
        return Ok(FieldValue::Null);
    }

    match &descriptor.field_type {
        DjangoFieldType::CharField { .. }
        | DjangoFieldType::TextField
        | DjangoFieldType::EmailField { .. }
        | DjangoFieldType::UrlField { .. }
        | DjangoFieldType::SlugField { .. } => {
            let val: String = py_value.extract().map_err(|_| DjangoError::TypeConversion {
                expected: "str".into(),
                actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
            })?;
            Ok(FieldValue::Text(val))
        }

        DjangoFieldType::IntegerField => {
            let val: i64 = py_value.extract().map_err(|_| DjangoError::TypeConversion {
                expected: "int".into(),
                actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
            })?;
            Ok(FieldValue::Integer(val))
        }

        DjangoFieldType::BigIntegerField => {
            let val: i64 = py_value.extract().map_err(|_| DjangoError::TypeConversion {
                expected: "int (i64)".into(),
                actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
            })?;
            Ok(FieldValue::Integer(val))
        }

        DjangoFieldType::FloatField => {
            let val: f64 = py_value.extract().map_err(|_| DjangoError::TypeConversion {
                expected: "float".into(),
                actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
            })?;
            Ok(FieldValue::Float(val))
        }

        DjangoFieldType::DecimalField { .. } => {
            // Django's Decimal comes as Python's decimal.Decimal — extract via str to preserve precision
            let str_val: String = py_value
                .call_method0("__str__")
                .and_then(|s| s.extract())
                .map_err(|_| DjangoError::TypeConversion {
                    expected: "Decimal (via str)".into(),
                    actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                })?;
            let decimal = Decimal::from_str(&str_val).map_err(|_| DjangoError::Serialization {
                field: descriptor.name.clone(),
                message: format!("cannot parse '{str_val}' as Decimal"),
            })?;
            Ok(FieldValue::Decimal(decimal))
        }

        DjangoFieldType::BooleanField => {
            let val: bool = py_value.extract().map_err(|_| DjangoError::TypeConversion {
                expected: "bool".into(),
                actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
            })?;
            Ok(FieldValue::Boolean(val))
        }

        DjangoFieldType::DateField => {
            let iso: String = py_value
                .call_method0("isoformat")
                .and_then(|s| s.extract())
                .map_err(|_| DjangoError::TypeConversion {
                    expected: "date".into(),
                    actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                })?;
            let date = NaiveDate::parse_from_str(&iso, "%Y-%m-%d").map_err(|_| {
                DjangoError::Serialization {
                    field: descriptor.name.clone(),
                    message: format!("cannot parse '{iso}' as date"),
                }
            })?;
            Ok(FieldValue::Date(date))
        }

        DjangoFieldType::TimeField => {
            let iso: String = py_value
                .call_method0("isoformat")
                .and_then(|s| s.extract())
                .map_err(|_| DjangoError::TypeConversion {
                    expected: "time".into(),
                    actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                })?;
            let time = NaiveTime::parse_from_str(&iso, "%H:%M:%S%.f").map_err(|_| {
                DjangoError::Serialization {
                    field: descriptor.name.clone(),
                    message: format!("cannot parse '{iso}' as time"),
                }
            })?;
            Ok(FieldValue::Time(time))
        }

        DjangoFieldType::DateTimeField => {
            let iso: String = py_value
                .call_method0("isoformat")
                .and_then(|s| s.extract())
                .map_err(|_| DjangoError::TypeConversion {
                    expected: "datetime".into(),
                    actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                })?;
            let dt = iso
                .parse::<DateTime<Utc>>()
                .or_else(|_| {
                    // Django may emit naive datetimes without timezone suffix
                    chrono::NaiveDateTime::parse_from_str(&iso, "%Y-%m-%dT%H:%M:%S%.f")
                        .map(|naive| naive.and_utc())
                })
                .map_err(|_| DjangoError::Serialization {
                    field: descriptor.name.clone(),
                    message: format!("cannot parse '{iso}' as datetime"),
                })?;
            Ok(FieldValue::DateTime(dt))
        }

        DjangoFieldType::UuidField => {
            let str_val: String =
                py_value
                    .call_method0("__str__")
                    .and_then(|s| s.extract())
                    .map_err(|_| DjangoError::TypeConversion {
                        expected: "UUID".into(),
                        actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                    })?;
            let uuid = Uuid::parse_str(&str_val).map_err(|_| DjangoError::Serialization {
                field: descriptor.name.clone(),
                message: format!("cannot parse '{str_val}' as UUID"),
            })?;
            Ok(FieldValue::Uuid(uuid))
        }

        DjangoFieldType::JsonField => {
            // GIL is already held — serialize through Python's json.dumps, then parse in Rust
            let py = py_value.py();
            let json_mod = py.import("json").map_err(|e| {
                DjangoError::Python(format!("cannot import json module: {e}"))
            })?;
            let dumped = json_mod
                .call_method1("dumps", (py_value,))
                .map_err(|e| DjangoError::Serialization {
                    field: descriptor.name.clone(),
                    message: format!("json.dumps failed: {e}"),
                })?;
            let json_str: String = dumped.extract::<String>().map_err(|e| {
                DjangoError::Python(format!("json.dumps returned non-string: {e}"))
            })?;
            let value: serde_json::Value =
                serde_json::from_str(&json_str).map_err(|e| DjangoError::Serialization {
                    field: descriptor.name.clone(),
                    message: format!("invalid JSON: {e}"),
                })?;
            Ok(FieldValue::Json(value))
        }

        DjangoFieldType::BinaryField { .. } => {
            let bytes: Vec<u8> =
                py_value
                    .extract()
                    .map_err(|_| DjangoError::TypeConversion {
                        expected: "bytes".into(),
                        actual: py_value.get_type().qualname().map(|n| n.to_string()).unwrap_or_else(|_| "unknown".into()),
                    })?;
            Ok(FieldValue::Binary(bytes))
        }
    }
}
