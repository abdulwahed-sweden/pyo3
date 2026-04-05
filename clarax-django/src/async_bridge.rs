// Author: Abdulwahed Mansour
//! ASGI-compatible async context handling for Django.
//!
//! Provides utilities for running Rust-accelerated operations within Django's
//! async view layer (ASGI) without blocking the event loop.

use clarax::prelude::*;

use crate::error::DjangoError;
use crate::field_types::FieldDescriptor;
use crate::serializer::{serialize_model_fields, SerializedRecord};
use crate::validator::{validate_field_batch, ValidationReport};
use clarax_core::types::FieldValue;

/// Serializes model fields while releasing the Python GIL during computation.
pub fn serialize_fields_release_gil(
    py: Python<'_>,
    descriptors: Vec<FieldDescriptor>,
    values: Vec<FieldValue>,
) -> Result<SerializedRecord, DjangoError> {
    py.detach(|| serialize_model_fields(&descriptors, &values))
}

/// Validates a field batch while releasing the Python GIL.
pub fn validate_batch_release_gil(
    py: Python<'_>,
    entries: Vec<(FieldDescriptor, FieldValue)>,
) -> ValidationReport {
    py.detach(|| validate_field_batch(&entries))
}
