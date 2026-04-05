// Author: Abdulwahed Mansour
//! Django-specific field type mapping.
//!
//! Provides `DjangoFieldType` which maps Django's model field classes to
//! pyforge-core's `FieldType`. The actual serialization and validation
//! logic lives in pyforge-core — this module only handles the mapping.

use pyforge_core::types::{FieldDescriptor as CoreDescriptor, FieldType as CoreFieldType};
use serde::{Deserialize, Serialize};

/// Describes a single Django model field, including its type and constraints.
///
/// This is the Django-specific wrapper. It stores the Django field type name
/// for error messages and round-tripping, and converts to pyforge-core's
/// `FieldDescriptor` for actual processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub name: String,
    pub field_type: DjangoFieldType,
    pub nullable: bool,
    pub has_default: bool,
}

impl FieldDescriptor {
    /// Converts this Django descriptor to a pyforge-core descriptor.
    pub fn to_core(&self) -> CoreDescriptor {
        CoreDescriptor {
            name: self.name.clone(),
            field_type: self.field_type.to_core_type(),
            nullable: self.nullable,
            has_default: self.has_default,
        }
    }
}

/// Enumeration of Django field types with their associated validation constraints.
///
/// Each variant maps to a pyforge-core `FieldType` via `to_core_type()`.
/// This layer exists so that Django-specific type names (CharField vs TextField,
/// EmailField, SlugField, etc.) are preserved for error messages and
/// descriptor round-tripping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DjangoFieldType {
    CharField { max_length: usize },
    TextField,
    IntegerField,
    BigIntegerField,
    FloatField,
    DecimalField { max_digits: u32, decimal_places: u32 },
    BooleanField,
    DateField,
    TimeField,
    DateTimeField,
    UuidField,
    JsonField,
    BinaryField { max_length: Option<usize> },
    EmailField { max_length: usize },
    UrlField { max_length: usize },
    SlugField { max_length: usize },
}

impl DjangoFieldType {
    /// Returns the Django `get_internal_type()` name for this field type.
    pub fn django_type_name(&self) -> &'static str {
        match self {
            DjangoFieldType::CharField { .. } => "CharField",
            DjangoFieldType::TextField => "TextField",
            DjangoFieldType::IntegerField => "IntegerField",
            DjangoFieldType::BigIntegerField => "BigIntegerField",
            DjangoFieldType::FloatField => "FloatField",
            DjangoFieldType::DecimalField { .. } => "DecimalField",
            DjangoFieldType::BooleanField => "BooleanField",
            DjangoFieldType::DateField => "DateField",
            DjangoFieldType::TimeField => "TimeField",
            DjangoFieldType::DateTimeField => "DateTimeField",
            DjangoFieldType::UuidField => "UUIDField",
            DjangoFieldType::JsonField => "JSONField",
            DjangoFieldType::BinaryField { .. } => "BinaryField",
            DjangoFieldType::EmailField { .. } => "EmailField",
            DjangoFieldType::UrlField { .. } => "URLField",
            DjangoFieldType::SlugField { .. } => "SlugField",
        }
    }

    /// Converts this Django field type to a pyforge-core `FieldType`.
    pub fn to_core_type(&self) -> CoreFieldType {
        match self {
            DjangoFieldType::CharField { max_length } => CoreFieldType::Str {
                max_length: Some(*max_length),
                min_length: None,
            },
            DjangoFieldType::TextField => CoreFieldType::Str {
                max_length: None,
                min_length: None,
            },
            DjangoFieldType::IntegerField => CoreFieldType::Int {
                min_value: Some(i32::MIN as i64),
                max_value: Some(i32::MAX as i64),
            },
            DjangoFieldType::BigIntegerField => CoreFieldType::Int {
                min_value: None,
                max_value: None,
            },
            DjangoFieldType::FloatField => CoreFieldType::Float {
                min_value: None,
                max_value: None,
            },
            DjangoFieldType::DecimalField { max_digits, decimal_places } => CoreFieldType::Decimal {
                max_digits: Some(*max_digits),
                decimal_places: Some(*decimal_places),
            },
            DjangoFieldType::BooleanField => CoreFieldType::Bool,
            DjangoFieldType::DateField => CoreFieldType::Date,
            DjangoFieldType::TimeField => CoreFieldType::Time,
            DjangoFieldType::DateTimeField => CoreFieldType::DateTime,
            DjangoFieldType::UuidField => CoreFieldType::Uuid,
            DjangoFieldType::JsonField => CoreFieldType::Dict,
            DjangoFieldType::BinaryField { max_length } => CoreFieldType::Bytes {
                max_length: *max_length,
            },
            DjangoFieldType::EmailField { max_length } => CoreFieldType::Str {
                max_length: Some(*max_length),
                min_length: None,
            },
            DjangoFieldType::UrlField { max_length } => CoreFieldType::Str {
                max_length: Some(*max_length),
                min_length: None,
            },
            DjangoFieldType::SlugField { max_length } => CoreFieldType::Str {
                max_length: Some(*max_length),
                min_length: None,
            },
        }
    }
}

// Re-export FieldValue from pyforge-core — no Django-specific wrapper needed.
pub use pyforge_core::types::FieldValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_descriptor_converts_to_core() {
        let desc = FieldDescriptor {
            name: "price".into(),
            field_type: DjangoFieldType::DecimalField {
                max_digits: 10,
                decimal_places: 2,
            },
            nullable: false,
            has_default: false,
        };
        let core = desc.to_core();
        assert_eq!(core.name, "price");
        assert!(matches!(core.field_type, CoreFieldType::Decimal { .. }));
    }

    #[test]
    fn django_type_names_round_trip() {
        assert_eq!(DjangoFieldType::CharField { max_length: 100 }.django_type_name(), "CharField");
        assert_eq!(DjangoFieldType::UuidField.django_type_name(), "UUIDField");
        assert_eq!(DjangoFieldType::JsonField.django_type_name(), "JSONField");
    }

    #[test]
    fn charfield_maps_to_str_with_max_length() {
        let ct = DjangoFieldType::CharField { max_length: 50 }.to_core_type();
        match ct {
            CoreFieldType::Str { max_length, .. } => assert_eq!(max_length, Some(50)),
            _ => panic!("expected Str"),
        }
    }
}
