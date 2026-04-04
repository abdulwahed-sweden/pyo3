# Author: Abdulwahed Mansour
"""
Django AppConfig for pyforge-django.

Add 'django_pyforge' to INSTALLED_APPS to enable the integration.
The ready() hook verifies that the native extension loads correctly
and logs the active version at startup.
"""

__author__ = "Abdulwahed Mansour"

from django.apps import AppConfig


class DjangoPyforgeConfig(AppConfig):
    """Django app configuration for pyforge-django."""

    name = "django_pyforge"
    verbose_name = "PyForge Django Accelerator"
    default_auto_field = "django.db.models.BigAutoField"

    def ready(self):
        """Verify the native extension loads and log the version."""
        import logging

        logger = logging.getLogger("django_pyforge")

        try:
            from django_pyforge import __version__
            logger.info("pyforge-django %s loaded — Rust acceleration active", __version__)
        except ImportError:
            logger.warning(
                "pyforge-django native extension not found — "
                "serialization will fall back to pure Python DRF"
            )
