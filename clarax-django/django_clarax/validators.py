# Author: Abdulwahed Mansour
"""
Rust-accelerated validators for Django model instances.

Usage:
    from django_clarax.validators import validate_model_instance

    report = validate_model_instance(instance)
    if not report["is_valid"]:
        for error in report["errors"]:
            print(f"{error['field']}: {error['message']}")
"""

__author__ = "Abdulwahed Mansour"
__version__ = "0.1.0"

from django_clarax import ModelSchema, validate_fields as _rust_validate
from django_clarax import validate_instance as _rust_validate_instance


def validate_model_instance(instance, schema=None):
    """
    Validate all fields of a Django model instance.

    Extracts field values and runs them through Rust validation.
    For batches above 64 fields, validation runs in parallel.

    Args:
        instance: A Django model instance.
        schema: Optional pre-compiled ModelSchema. If None, compiled on the fly.

    Returns:
        A dict with:
            is_valid (bool): True if all fields pass validation.
            valid_count (int): Number of valid fields.
            error_count (int): Number of failed fields.
            errors (list[dict]): Per-field error details.
    """
    if schema is None:
        schema = ModelSchema(type(instance))
    return _rust_validate_instance(instance, schema)


def validate_field_batch(descriptors, values):
    """
    Validate a batch of field values using the Rust validation engine.

    For batches above 64 entries, validation runs in parallel.

    Args:
        descriptors: List of field descriptor dicts.
        values: List of Python values aligned by index with descriptors.

    Returns:
        A dict with is_valid, valid_count, error_count, errors.
    """
    return _rust_validate(descriptors, values)
