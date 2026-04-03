# Author: Abdulwahed Mansour
"""
Drop-in mixin for Django REST Framework serializers.

Accelerates the serialization phase using Rust-native field processing.
Works with existing DRF ModelSerializer — just add the mixin.

Usage:
    from django_pyforge.serializers import RustSerializerMixin
    from rest_framework import serializers

    class UserSerializer(RustSerializerMixin, serializers.ModelSerializer):
        class Meta:
            model = User
            fields = '__all__'

The mixin automatically detects which fields are simple model fields
(accelerated by Rust) vs computed fields (delegated to DRF).
"""

import logging

from django_pyforge import ModelSchema, serialize_instance

logger = logging.getLogger("django_pyforge")

# Sentinel for uninitialized schema cache
_UNINITIALIZED = object()


class RustSerializerMixin:
    """
    Mixin that accelerates DRF serializer .to_representation() using Rust.

    Add this mixin BEFORE the base serializer class in the MRO:

        class MySerializer(RustSerializerMixin, serializers.ModelSerializer):
            ...

    Behavior:
    - On first use, compiles a ModelSchema from the model class and caches it.
    - For each instance, extracts simple model fields via Rust (single call).
    - Delegates computed fields (SerializerMethodField, source overrides,
      nested serializers) to DRF's standard path.
    - Reports which fields were Rust-accelerated in debug mode.

    Set `PYFORGE_DEBUG = True` in Django settings to enable debug logging.
    """

    _pyforge_schema = _UNINITIALIZED
    _pyforge_rust_fields = _UNINITIALIZED
    _pyforge_python_fields = _UNINITIALIZED

    @classmethod
    def _init_pyforge_schema(cls):
        """Build the schema and classify fields on first use."""
        if cls._pyforge_schema is not _UNINITIALIZED:
            return

        try:
            model_class = cls.Meta.model
        except AttributeError:
            cls._pyforge_schema = None
            cls._pyforge_rust_fields = set()
            cls._pyforge_python_fields = set()
            return

        try:
            schema = ModelSchema(model_class)
            rust_field_names = set(schema.field_names_list)
            cls._pyforge_schema = schema
        except Exception as exc:
            logger.debug("PyForge: could not compile schema for %s: %s", model_class.__name__, exc)
            cls._pyforge_schema = None
            cls._pyforge_rust_fields = set()
            cls._pyforge_python_fields = set()
            return

        cls._pyforge_rust_fields = rust_field_names
        cls._pyforge_python_fields = set()

    def _classify_fields(self):
        """
        Classify serializer fields into Rust-accelerated and Python-delegated.

        A field is Rust-accelerated if:
        1. Its field_name matches a model field in the schema
        2. It has no custom `source` attribute (or source == field_name)
        3. It is NOT a SerializerMethodField, nested serializer, or property field
        """
        cls = type(self)
        cls._init_pyforge_schema()

        if cls._pyforge_schema is None:
            return set(), set(self.fields.keys())

        rust_fields = set()
        python_fields = set()

        for field_name, field_obj in self.fields.items():
            field_class_name = type(field_obj).__name__

            # These field types require Python processing — cannot be accelerated
            is_computed = field_class_name in (
                "SerializerMethodField",
                "HiddenField",
                "ReadOnlyField",
            )

            # Nested serializers require their own to_representation
            is_nested = hasattr(field_obj, "Meta") and hasattr(field_obj, "fields")

            # Custom source means the value doesn't come from a simple getattr
            source = getattr(field_obj, "source", field_name)
            has_custom_source = source != field_name and source != "*"

            # Check if this field name exists in the Rust schema
            in_rust_schema = field_name in cls._pyforge_rust_fields

            if in_rust_schema and not is_computed and not is_nested and not has_custom_source:
                rust_fields.add(field_name)
            else:
                python_fields.add(field_name)

        return rust_fields, python_fields

    def to_representation(self, instance):
        """
        Serialize a model instance using Rust for simple fields, DRF for complex ones.

        This is the hot path. For a 9-field model with no computed fields, this
        makes a single Rust call instead of 9 individual Python field.to_representation() calls.
        """
        cls = type(self)
        cls._init_pyforge_schema()

        if cls._pyforge_schema is None:
            return super().to_representation(instance)

        rust_fields, python_fields = self._classify_fields()

        if not rust_fields:
            return super().to_representation(instance)

        try:
            # Rust path: single call for all model fields
            rust_result = serialize_instance(instance, cls._pyforge_schema)

            if not python_fields:
                # All fields handled by Rust — fastest path
                return {k: v for k, v in rust_result.items() if k in rust_fields}

            # Hybrid path: Rust for model fields, DRF for computed fields
            result = {}

            # Take Rust-serialized fields
            for field_name in rust_fields:
                if field_name in rust_result:
                    result[field_name] = rust_result[field_name]

            # Delegate computed fields to DRF
            for field_name in python_fields:
                field_obj = self.fields[field_name]
                try:
                    attr = field_obj.get_attribute(instance)
                    result[field_name] = field_obj.to_representation(attr)
                except Exception:
                    result[field_name] = None

            # Preserve field declaration order
            ordered = {}
            for field_name in self.fields:
                if field_name in result:
                    ordered[field_name] = result[field_name]
            return ordered

        except Exception as exc:
            logger.debug("PyForge: Rust serialization failed, falling back to DRF: %s", exc)
            return super().to_representation(instance)
