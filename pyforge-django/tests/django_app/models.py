# Author: Abdulwahed Mansour
"""
Test models for pyforge-django integration tests.

RentalApplication is the canonical test model — it covers every common
Django field type and matches the model used in benchmarks.
"""

import uuid

from django.db import models


class RentalApplication(models.Model):
    """A rental application with all common Django field types."""

    applicant_name = models.CharField(max_length=120)
    national_id = models.CharField(max_length=20)
    monthly_income = models.DecimalField(max_digits=10, decimal_places=2)
    application_date = models.DateField()
    submitted_at = models.DateTimeField()
    apartment_uuid = models.UUIDField(default=uuid.uuid4)
    is_approved = models.BooleanField(default=False)
    notes = models.TextField(blank=True, default="")
    score = models.IntegerField()

    class Meta:
        app_label = "django_app"

    def __str__(self):
        return f"RentalApplication({self.applicant_name})"


class NullableFieldModel(models.Model):
    """Model with nullable fields for null-handling tests."""

    optional_name = models.CharField(max_length=100, null=True, blank=True)
    optional_score = models.IntegerField(null=True, blank=True)
    optional_date = models.DateField(null=True, blank=True)

    class Meta:
        app_label = "django_app"
