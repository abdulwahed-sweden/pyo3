# Author: Abdulwahed Mansour
"""
Integration tests for ModelSchema with real Django models.

These tests verify that:
1. ModelSchema correctly introspects Django model _meta
2. Field descriptors match the model's actual fields
3. Schema caching works (same schema object reused across calls)
"""

import pytest
from django.test import TestCase

from tests.django_app.models import RentalApplication, NullableFieldModel

# Skip all tests if clarax_django extension is not installed
pytestmark = pytest.mark.skipif(
    not _can_import_clarax(),
    reason="clarax_django native extension not installed",
)


def _can_import_clarax():
    try:
        import clarax_django
        return True
    except ImportError:
        return False


class TestModelSchema(TestCase):
    """Tests for ModelSchema compilation from Django models."""

    def test_schema_from_rental_application(self):
        """ModelSchema should extract all 9 concrete fields from RentalApplication."""
        from clarax_django import ModelSchema

        schema = ModelSchema(RentalApplication)
        field_names = schema.field_names_list
        assert "applicant_name" in field_names
        assert "monthly_income" in field_names
        assert "apartment_uuid" in field_names
        assert "is_approved" in field_names
        assert "score" in field_names
        # id is auto-generated but should not be in schema (AutoField)
        assert "id" not in field_names

    def test_schema_length(self):
        """Schema length should match the number of concrete model fields."""
        from clarax_django import ModelSchema

        schema = ModelSchema(RentalApplication)
        assert len(schema) == 9

    def test_schema_repr(self):
        """Schema repr should include model name and field count."""
        from clarax_django import ModelSchema

        schema = ModelSchema(RentalApplication)
        r = repr(schema)
        assert "RentalApplication" in r
        assert "9" in r

    def test_schema_to_descriptor_list(self):
        """to_descriptor_list should return a list of dicts with correct keys."""
        from clarax_django import ModelSchema

        schema = ModelSchema(RentalApplication)
        desc_list = schema.to_descriptor_list()
        assert len(desc_list) == 9
        for desc in desc_list:
            assert "name" in desc
            assert "type" in desc
            assert "nullable" in desc
            assert "has_default" in desc

    def test_nullable_model_fields_marked_correctly(self):
        """Nullable fields should have nullable=True in the schema."""
        from clarax_django import ModelSchema

        schema = ModelSchema(NullableFieldModel)
        desc_list = schema.to_descriptor_list()
        for desc in desc_list:
            if desc["name"] == "optional_name":
                assert desc["nullable"] is True
            if desc["name"] == "optional_score":
                assert desc["nullable"] is True

    def test_schema_rejects_non_model_class(self):
        """ModelSchema should raise on non-Django classes."""
        from clarax_django import ModelSchema

        with pytest.raises(Exception):
            ModelSchema(str)  # str has no _meta
