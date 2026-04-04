# Author: Abdulwahed Mansour
"""Type stubs for django_pyforge.validators."""

from typing import Any

from django_pyforge import ModelSchema

def validate_model_instance(
    instance: Any,
    schema: ModelSchema | None = None,
) -> dict[str, Any]:
    """Validate all fields of a Django model instance.

    Args:
        instance: A Django model instance.
        schema: Optional pre-compiled ModelSchema. Compiled on the fly if None.

    Returns:
        A dict with is_valid, valid_count, error_count, and errors.
    """
    ...

def validate_field_batch(
    descriptors: list[dict[str, Any]],
    values: list[Any],
) -> dict[str, Any]:
    """Validate a batch of field values using Rust.

    For batches above 64 entries, runs in parallel via Rayon.
    """
    ...
