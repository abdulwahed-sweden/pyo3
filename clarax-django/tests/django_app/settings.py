# Author: Abdulwahed Mansour
"""
Minimal Django settings for clarax-django integration tests.
Uses in-memory SQLite — no persistent state, no external dependencies.
"""

SECRET_KEY = "clarax-django-test-key-not-for-production"

DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.sqlite3",
        "NAME": ":memory:",
    }
}

INSTALLED_APPS = [
    "django.contrib.contenttypes",
    "django.contrib.auth",
    "tests.django_app",
]

DEFAULT_AUTO_FIELD = "django.db.models.BigAutoField"

USE_TZ = True
