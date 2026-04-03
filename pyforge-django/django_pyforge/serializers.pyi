# Author: Abdulwahed Mansour
"""Type stubs for django_pyforge.serializers."""

from typing import Any

class RustSerializerMixin:
    """DRF mixin that accelerates serialization using Rust.

    Add before the base serializer in MRO:
        class MySerializer(RustSerializerMixin, serializers.ModelSerializer): ...

    Automatically classifies fields into Rust-accelerated (simple model fields)
    and Python-delegated (SerializerMethodField, nested serializers, custom source).
    Falls back to DRF on any error.
    """

    def to_representation(self, instance: Any) -> dict[str, Any]:
        """Serialize using Rust for model fields, DRF for computed fields."""
        ...
