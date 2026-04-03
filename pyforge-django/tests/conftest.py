# Author: Abdulwahed Mansour
"""
pytest configuration for pyforge-django integration tests.

Sets up Django with in-memory SQLite before any tests run.
"""

import django
from django.conf import settings


def pytest_configure():
    """Configure Django settings for test suite."""
    if not settings.configured:
        settings.configure(
            SECRET_KEY="pyforge-django-test-key-not-for-production",
            DATABASES={
                "default": {
                    "ENGINE": "django.db.backends.sqlite3",
                    "NAME": ":memory:",
                }
            },
            INSTALLED_APPS=[
                "django.contrib.contenttypes",
                "django.contrib.auth",
                "tests.django_app",
            ],
            DEFAULT_AUTO_FIELD="django.db.models.BigAutoField",
            USE_TZ=True,
        )
        django.setup()
