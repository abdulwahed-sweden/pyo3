# Author: Abdulwahed Mansour
"""Type stubs for pyforge_django native extension."""

from typing import Any

__version__: str
__author__: str

class ModelSchema:
    """Compiled field descriptor cache for a Django model.

    Compile once at application startup, reuse on every request.
    Eliminates per-request descriptor parsing overhead.
    """

    def __init__(self, model_class: type) -> None:
        """Compile a schema from a Django model class.

        Args:
            model_class: A Django model class (e.g., MyModel).

        Raises:
            ValueError: If the class has no _meta attribute.
        """
        ...

    @property
    def field_names_list(self) -> list[str]:
        """Field names in declaration order."""
        ...

    @property
    def model_name_str(self) -> str:
        """The Django model class name."""
        ...

    def __len__(self) -> int:
        """Number of fields in the schema."""
        ...

    def __repr__(self) -> str: ...

    def to_descriptor_list(self) -> list[dict[str, Any]]:
        """Return the schema as a list of dicts for compatibility."""
        ...

def serialize_instance(
    instance: Any,
    schema: ModelSchema,
) -> dict[str, Any]:
    """Serialize a single Django model instance using the compiled schema.

    Extracts all field values in a single Rust call via getattr,
    converts to native types, and returns a JSON-compatible dict.

    Args:
        instance: A Django model instance.
        schema: A ModelSchema compiled from the model class.

    Returns:
        A dict mapping field names to serialized values.

    Raises:
        ValueError: If a required field is missing or serialization fails.
        TypeError: If a field value cannot be converted to the expected type.
    """
    ...

def serialize_batch(
    instances: list[Any],
    schema: ModelSchema,
) -> list[dict[str, Any]]:
    """Serialize a list of Django model instances.

    Args:
        instances: A list or queryset of model instances.
        schema: A ModelSchema compiled from the model class.

    Returns:
        A list of dicts, one per instance.
    """
    ...

def validate_instance(
    instance: Any,
    schema: ModelSchema,
) -> dict[str, Any]:
    """Validate a model instance against its compiled schema.

    Returns a dict with:
        is_valid (bool): True if all fields pass.
        valid_count (int): Number of valid fields.
        error_count (int): Number of failed fields.
        errors (list[dict]): Per-field error details.

    Args:
        instance: A Django model instance.
        schema: A ModelSchema compiled from the model class.
    """
    ...

def extract_model_fields(model_class: type) -> list[dict[str, Any]]:
    """Extract field descriptors from a Django model class.

    For repeated use, prefer ModelSchema which caches the result.
    """
    ...

def serialize_fields(
    field_descriptors: list[dict[str, Any]],
    values: dict[str, Any],
) -> dict[str, Any]:
    """Serialize field values from a dict (legacy API — prefer serialize_instance)."""
    ...

def validate_fields(
    descriptors: list[dict[str, Any]],
    values: list[Any],
) -> dict[str, Any]:
    """Validate field values from lists (legacy API — prefer validate_instance)."""
    ...

def version() -> str:
    """Return the pyforge-django version string."""
    ...
