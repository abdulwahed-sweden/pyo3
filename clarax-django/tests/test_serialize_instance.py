# Author: Abdulwahed Mansour
"""
Integration tests for serialize_instance with real Django model instances.

These tests verify end-to-end: Django model → Rust extraction → JSON dict.
They use in-memory SQLite to create real model instances with real data.
"""

import uuid
from datetime import date, datetime, timezone
from decimal import Decimal

import pytest
from django.test import TestCase

from tests.django_app.models import NullableFieldModel, RentalApplication


def _can_import_clarax():
    try:
        import clarax_django
        return True
    except ImportError:
        return False


pytestmark = pytest.mark.skipif(
    not _can_import_clarax(),
    reason="clarax_django native extension not installed",
)


class TestSerializeInstance(TestCase):
    """Tests for serialize_instance with real Django objects."""

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        from clarax_django import ModelSchema

        cls.schema = ModelSchema(RentalApplication)
        cls.nullable_schema = ModelSchema(NullableFieldModel)

    def _create_application(self, **overrides):
        defaults = {
            "applicant_name": "Abdulwahed Mansour",
            "national_id": "SE-19880515-001",
            "monthly_income": Decimal("45000.00"),
            "application_date": date(2025, 3, 15),
            "submitted_at": datetime(2025, 3, 15, 14, 30, 0, tzinfo=timezone.utc),
            "apartment_uuid": uuid.UUID("12345678-1234-5678-1234-567812345678"),
            "is_approved": False,
            "notes": "Priority applicant",
            "score": 750,
        }
        defaults.update(overrides)
        return RentalApplication.objects.create(**defaults)

    def test_serialize_basic_instance(self):
        """A standard model instance should serialize all fields correctly."""
        from clarax_django import serialize_instance

        obj = self._create_application()
        result = serialize_instance(obj, self.schema)

        assert result["applicant_name"] == "Abdulwahed Mansour"
        assert result["national_id"] == "SE-19880515-001"
        # Decimal preserved as string
        assert result["monthly_income"] == "45000.00"
        # Date as ISO string
        assert result["application_date"] == "2025-03-15"
        # DateTime as RFC3339
        assert "2025-03-15" in result["submitted_at"]
        # UUID as hyphenated string
        assert result["apartment_uuid"] == "12345678-1234-5678-1234-567812345678"
        # Boolean as True/False, not 1/0
        assert result["is_approved"] is False
        assert result["notes"] == "Priority applicant"
        assert result["score"] == 750

    def test_serialize_boolean_is_python_bool(self):
        """Booleans must serialize as Python True/False, not int 1/0."""
        from clarax_django import serialize_instance

        obj = self._create_application(is_approved=True)
        result = serialize_instance(obj, self.schema)
        assert result["is_approved"] is True
        assert type(result["is_approved"]) is bool

    def test_serialize_decimal_preserves_precision(self):
        """DecimalField must serialize as string without float conversion."""
        from clarax_django import serialize_instance

        obj = self._create_application(monthly_income=Decimal("99999999.99"))
        result = serialize_instance(obj, self.schema)
        assert result["monthly_income"] == "99999999.99"

    def test_serialize_nullable_fields_with_none(self):
        """None values on nullable fields should serialize as null."""
        from clarax_django import serialize_instance

        obj = NullableFieldModel.objects.create(
            optional_name=None,
            optional_score=None,
            optional_date=None,
        )
        result = serialize_instance(obj, self.nullable_schema)
        assert result["optional_name"] is None
        assert result["optional_score"] is None
        assert result["optional_date"] is None

    def test_serialize_batch(self):
        """serialize_batch should return a list of dicts for multiple instances."""
        from clarax_django import serialize_batch

        objs = [self._create_application(score=700 + i) for i in range(5)]
        results = serialize_batch(objs, self.schema)
        assert len(results) == 5
        for i, result in enumerate(results):
            assert result["score"] == 700 + i

    def test_serialize_multibyte_charfield(self):
        """Multi-byte UTF-8 characters should serialize correctly."""
        from clarax_django import serialize_instance

        arabic_name = "\u0639\u0628\u062f\u0627\u0644\u0648\u0627\u062d\u062f"
        obj = self._create_application(applicant_name=arabic_name)
        result = serialize_instance(obj, self.schema)
        assert result["applicant_name"] == arabic_name
