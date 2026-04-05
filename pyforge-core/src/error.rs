// Author: Abdulwahed Mansour
//! Error types for pyforge-core.
//!
//! Provides structured errors that map cleanly to Python exceptions
//! and carry enough detail for useful error messages.

use std::collections::HashMap;

/// A validation error for a single field.
///
/// Carries the field name, a human-readable message, an error code
/// for programmatic handling, and optional parameters for message
/// interpolation (e.g., `max_length` in a length violation).
#[derive(Debug, Clone)]
pub struct FieldValidationError {
    /// The field that failed validation.
    pub field_name: String,
    /// Human-readable error message.
    pub message: String,
    /// Machine-readable error code (e.g., "required", "max_length", "invalid").
    pub code: String,
    /// Additional parameters for the error (e.g., {"max_length": "100"}).
    pub params: HashMap<String, String>,
}

/// Top-level error type for pyforge-core operations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// A field value could not be serialized to JSON.
    #[error("serialization error on field '{field}': {message}")]
    Serialization { field: String, message: String },

    /// A required field was null or missing.
    #[error("field '{field}' is required but was null")]
    NullField { field: String },

    /// A field value had the wrong type.
    #[error("type error on field '{field}': expected {expected}, got {got}")]
    TypeError {
        field: String,
        expected: String,
        got: String,
    },

    /// Schema compilation failed.
    #[error("schema error: {message}")]
    SchemaError { message: String },
}

impl From<CoreError> for pyforge::PyErr {
    fn from(err: CoreError) -> pyforge::PyErr {
        match &err {
            CoreError::NullField { .. } => {
                pyforge::exceptions::PyValueError::new_err(err.to_string())
            }
            CoreError::TypeError { .. } => {
                pyforge::exceptions::PyTypeError::new_err(err.to_string())
            }
            CoreError::Serialization { .. } => {
                pyforge::exceptions::PyValueError::new_err(err.to_string())
            }
            CoreError::SchemaError { .. } => {
                pyforge::exceptions::PyValueError::new_err(err.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_includes_field_name() {
        let err = CoreError::NullField {
            field: "email".into(),
        };
        assert!(err.to_string().contains("email"));
    }

    #[test]
    fn type_error_display() {
        let err = CoreError::TypeError {
            field: "age".into(),
            expected: "int".into(),
            got: "str".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("age"));
        assert!(msg.contains("int"));
        assert!(msg.contains("str"));
    }

    #[test]
    fn field_validation_error_holds_params() {
        let err = FieldValidationError {
            field_name: "name".into(),
            message: "too long".into(),
            code: "max_length".into(),
            params: HashMap::from([("max_length".into(), "100".into())]),
        };
        assert_eq!(err.params["max_length"], "100");
    }
}
