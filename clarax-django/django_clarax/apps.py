# Author: Abdulwahed Mansour
"""
Django AppConfig for clarax-django.

Add 'django_clarax' to INSTALLED_APPS to enable the integration.
The ready() hook verifies that the native extension loads correctly
and logs the active version at startup.
"""

__author__ = "Abdulwahed Mansour"

from django.apps import AppConfig


class DjangoPyforgeConfig(AppConfig):
    """Django app configuration for clarax-django."""

    name = "django_clarax"
    verbose_name = "ClaraX Django Accelerator"
    default_auto_field = "django.db.models.BigAutoField"

    def ready(self):
        """Verify the native extension loads and log the version."""
        import logging

        logger = logging.getLogger("django_clarax")

        try:
            from django_clarax import __version__
            logger.info("clarax-django %s loaded — Rust acceleration active", __version__)
        except ImportError:
            logger.warning(
                "clarax-django native extension not found — "
                "serialization will fall back to pure Python DRF"
            )
