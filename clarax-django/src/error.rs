// Author: Abdulwahed Mansour
//! Django-specific error types with mapping to Django's exception hierarchy.
//!
//! Uses clarax-core's `FieldValidationError` for field-level errors.
//! `DjangoError` wraps these with Django-specific variants.

use clarax::exceptions::{PyTypeError, PyValueError};
use clarax::PyErr;

// Re-export from clarax-core — single source of truth for field validation errors.
pub use clarax_core::error::FieldValidationError;

/// Top-level error type for all clarax-django operations.
///
/// Each variant maps directly to a Django exception class.
#[derive(Debug, thiserror::Error)]
pub enum DjangoError {
    #[error("validation error on field '{field}': {message}")]
    FieldValidation { field: String, message: String },

    #[error("batch validation failed: {} field errors", .0.len())]
    BatchValidation(Vec<FieldValidationError>),

    #[error("serialization error on field '{field}': {message}")]
    Serialization { field: String, message: String },

    #[error("type conversion error: expected {expected}, got {actual}")]
    TypeConversion { expected: String, actual: String },

    #[error("field '{field}' is required but received null")]
    NullField { field: String },

    #[error("python error: {0}")]
    Python(String),
}

impl From<DjangoError> for PyErr {
    fn from(err: DjangoError) -> PyErr {
        match &err {
            DjangoError::FieldValidation { .. }
            | DjangoError::BatchValidation(_)
            | DjangoError::NullField { .. } => PyValueError::new_err(err.to_string()),
            DjangoError::TypeConversion { .. } => PyTypeError::new_err(err.to_string()),
            DjangoError::Serialization { .. } => PyValueError::new_err(err.to_string()),
            DjangoError::Python(msg) => PyValueError::new_err(msg.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn field_validation_error_formats_correctly() {
        let err = DjangoError::FieldValidation {
            field: "email".into(),
            message: "Enter a valid email address".into(),
        };
        assert!(err.to_string().contains("email"));
    }

    #[test]
    fn null_field_error_formats_correctly() {
        let err = DjangoError::NullField {
            field: "username".into(),
        };
        assert!(err.to_string().contains("username"));
    }

    #[test]
    fn type_conversion_error_formats_correctly() {
        let err = DjangoError::TypeConversion {
            expected: "i64".into(),
            actual: "str".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("i64"));
        assert!(msg.contains("str"));
    }

    #[test]
    fn batch_validation_error_reports_count() {
        let errors = vec![
            FieldValidationError {
                field_name: "age".into(),
                message: "Must be positive".into(),
                code: "min_value".into(),
                params: HashMap::new(),
            },
            FieldValidationError {
                field_name: "name".into(),
                message: "Too long".into(),
                code: "max_length".into(),
                params: HashMap::from([("max_length".into(), "255".into())]),
            },
        ];
        let err = DjangoError::BatchValidation(errors);
        assert!(err.to_string().contains("2 field errors"));
    }
}
