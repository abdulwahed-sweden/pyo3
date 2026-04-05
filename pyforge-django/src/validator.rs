// Author: Abdulwahed Mansour
//! Django validation — delegates to pyforge-core.
//!
//! This module is a thin wrapper that converts Django-specific types
//! to pyforge-core types and calls the core validation engine.

use crate::field_types::FieldDescriptor;

// Re-export core validation types for downstream use.
pub use pyforge_core::engine_validate::ValidationReport;
pub use pyforge_core::types::FieldValue;

/// Validates a batch of Django field values against their descriptors.
///
/// Delegates to `pyforge_core::validate_batch()` after converting
/// Django descriptors to core descriptors. Rayon parallelism kicks in
/// at the core's threshold (128 entries).
pub fn validate_field_batch(
    entries: &[(FieldDescriptor, FieldValue)],
) -> ValidationReport {
    let core_entries: Vec<_> = entries
        .iter()
        .map(|(desc, val)| (desc.to_core(), val.clone()))
        .collect();
    pyforge_core::validate_batch(&core_entries)
}
