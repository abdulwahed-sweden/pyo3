// Author: Abdulwahed Mansour
//! Django serialization — delegates to pyforge-core.
//!
//! This module is a thin wrapper that converts Django-specific types
//! to pyforge-core types and calls the core serialization engine.

use crate::error::DjangoError;
use crate::field_types::FieldDescriptor;

// Re-export the core types for downstream use.
pub use pyforge_core::engine_serialize::SerializedRecord;
pub use pyforge_core::types::FieldValue;

/// Serializes a set of Django field values into a JSON-compatible record.
///
/// Delegates to `pyforge_core::serialize_fields()` after converting
/// Django descriptors to core descriptors.
pub fn serialize_model_fields(
    descriptors: &[FieldDescriptor],
    values: &[FieldValue],
) -> Result<SerializedRecord, DjangoError> {
    let core_descs: Vec<_> = descriptors.iter().map(|d| d.to_core()).collect();
    pyforge_core::serialize_fields(&core_descs, values).map_err(|e| DjangoError::Serialization {
        field: "<batch>".into(),
        message: e.to_string(),
    })
}

/// Serializes multiple rows of Django field values.
///
/// Delegates to `pyforge_core::serialize_rows()`.
pub fn serialize_queryset_rows(
    descriptors: &[FieldDescriptor],
    rows: &[Vec<FieldValue>],
) -> Result<Vec<SerializedRecord>, DjangoError> {
    let core_descs: Vec<_> = descriptors.iter().map(|d| d.to_core()).collect();
    pyforge_core::serialize_rows(&core_descs, rows).map_err(|e| DjangoError::Serialization {
        field: "<batch>".into(),
        message: e.to_string(),
    })
}
