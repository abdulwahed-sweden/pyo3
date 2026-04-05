// Author: Abdulwahed Mansour
//! Framework-agnostic field types for serialization and validation.
//!
//! These types are the core data model of pyforge-core. They describe
//! what a field is (its type and constraints) and what a field holds
//! (its runtime value). No Django, no Flask — pure Python types only.

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Describes a single field in a schema, including its type and constraints.
///
/// Constructed once when a `Schema` is compiled, then reused on every
/// serialize/validate call with zero per-call parsing overhead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor {
    /// The field name — used as the key in serialized output.
    pub name: String,
    /// The field's type and type-specific constraints.
    pub field_type: FieldType,
    /// Whether `None` is an acceptable value for this field.
    pub nullable: bool,
    /// Whether the field has a default and can be omitted from input.
    pub has_default: bool,
}

/// Enumeration of supported field types with embedded constraints.
///
/// Each variant carries the validation metadata needed to check values
/// without additional lookups. Constraints are set once at schema
/// compilation time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    /// Python `str` — bounded or unbounded text.
    Str {
        max_length: Option<usize>,
        min_length: Option<usize>,
    },
    /// Python `int` — 64-bit signed integer with optional bounds.
    Int {
        min_value: Option<i64>,
        max_value: Option<i64>,
    },
    /// Python `float` — IEEE 754 double-precision with optional bounds.
    Float {
        min_value: Option<f64>,
        max_value: Option<f64>,
    },
    /// Python `bool`.
    Bool,
    /// Python `decimal.Decimal` — arbitrary-precision decimal.
    Decimal {
        max_digits: Option<u32>,
        decimal_places: Option<u32>,
    },
    /// Python `datetime.datetime` — timezone-aware datetime.
    DateTime,
    /// Python `datetime.date` — date without time.
    Date,
    /// Python `datetime.time` — time without date.
    Time,
    /// Python `uuid.UUID` — RFC 4122 UUID.
    Uuid,
    /// Python `list` — stored as JSON array.
    List,
    /// Python `dict` — stored as JSON object.
    Dict,
    /// Python `bytes` — raw byte data with optional length limit.
    Bytes { max_length: Option<usize> },
}

impl FieldType {
    /// Returns a human-readable type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            FieldType::Str { .. } => "str",
            FieldType::Int { .. } => "int",
            FieldType::Float { .. } => "float",
            FieldType::Bool => "bool",
            FieldType::Decimal { .. } => "Decimal",
            FieldType::DateTime => "datetime",
            FieldType::Date => "date",
            FieldType::Time => "time",
            FieldType::Uuid => "UUID",
            FieldType::List => "list",
            FieldType::Dict => "dict",
            FieldType::Bytes { .. } => "bytes",
        }
    }
}

/// A concrete Rust value extracted from a Python object or dict.
///
/// `Null` is a first-class variant rather than wrapping everything in
/// `Option` — this matches Python semantics where `None` and missing
/// are distinct concepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Text(String),
    Integer(i64),
    Float(f64),
    Decimal(Decimal),
    Boolean(bool),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(DateTime<Utc>),
    Uuid(Uuid),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Null,
}

impl FieldValue {
    /// Returns a human-readable type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            FieldValue::Text(_) => "str",
            FieldValue::Integer(_) => "int",
            FieldValue::Float(_) => "float",
            FieldValue::Decimal(_) => "Decimal",
            FieldValue::Boolean(_) => "bool",
            FieldValue::Date(_) => "date",
            FieldValue::Time(_) => "time",
            FieldValue::DateTime(_) => "datetime",
            FieldValue::Uuid(_) => "UUID",
            FieldValue::Json(_) => "JSON",
            FieldValue::Binary(_) => "bytes",
            FieldValue::Null => "None",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_descriptor_roundtrips_through_json() {
        let field = FieldDescriptor {
            name: "price".into(),
            field_type: FieldType::Decimal {
                max_digits: Some(10),
                decimal_places: Some(2),
            },
            nullable: false,
            has_default: false,
        };
        let json = serde_json::to_string(&field).unwrap();
        let back: FieldDescriptor = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "price");
    }

    #[test]
    fn field_value_type_names_match_python() {
        assert_eq!(FieldValue::Text("hi".into()).type_name(), "str");
        assert_eq!(FieldValue::Integer(42).type_name(), "int");
        assert_eq!(FieldValue::Null.type_name(), "None");
        assert_eq!(FieldValue::Uuid(Uuid::new_v4()).type_name(), "UUID");
    }

    #[test]
    fn field_type_names_match_python() {
        assert_eq!(
            FieldType::Str { max_length: None, min_length: None }.type_name(),
            "str"
        );
        assert_eq!(
            FieldType::Int { min_value: None, max_value: None }.type_name(),
            "int"
        );
        assert_eq!(FieldType::Bool.type_name(), "bool");
    }

    #[test]
    fn null_is_distinct_variant() {
        assert!(matches!(FieldValue::Null, FieldValue::Null));
        assert!(!matches!(FieldValue::Null, FieldValue::Text(_)));
    }

    #[test]
    fn str_field_with_constraints() {
        let ft = FieldType::Str {
            max_length: Some(100),
            min_length: Some(1),
        };
        assert_eq!(ft.type_name(), "str");
    }
}
