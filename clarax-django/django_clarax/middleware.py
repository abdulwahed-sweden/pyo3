# Author: Abdulwahed Mansour
"""
ClaraX metrics middleware.

Tracks per-request serialization metrics and exposes them via
the X-ClaraX-Stats response header.

Usage in settings.py:
    MIDDLEWARE = [
        ...
        "django_clarax.middleware.ClaraXMetricsMiddleware",
    ]
    CLARAX_METRICS = True  # Enable metrics (disabled by default)

Response header format:
    X-ClaraX-Stats: rust=15, python=2, calls=3, ms=4.2
"""

import threading
import time

_thread_local = threading.local()


def get_metrics():
    """Get the current thread's metrics accumulator."""
    if not hasattr(_thread_local, "clarax_metrics"):
        _thread_local.clarax_metrics = {
            "rust_fields": 0,
            "python_fields": 0,
            "calls": 0,
            "total_ms": 0.0,
        }
    return _thread_local.clarax_metrics


def reset_metrics():
    """Reset metrics for a new request."""
    _thread_local.clarax_metrics = {
        "rust_fields": 0,
        "python_fields": 0,
        "calls": 0,
        "total_ms": 0.0,
    }


def record_serialization(rust_count, python_count, elapsed_ms):
    """Record a single serialization call's metrics."""
    m = get_metrics()
    m["rust_fields"] += rust_count
    m["python_fields"] += python_count
    m["calls"] += 1
    m["total_ms"] += elapsed_ms


class ClaraXMetricsMiddleware:
    """Django middleware that attaches ClaraX metrics to response headers.

    Only active when settings.CLARAX_METRICS is True.
    Zero overhead when disabled — the check happens once at init time.
    """

    def __init__(self, get_response):
        self.get_response = get_response
        from django.conf import settings
        self.enabled = getattr(settings, "CLARAX_METRICS", False)

    def __call__(self, request):
        if not self.enabled:
            return self.get_response(request)

        reset_metrics()
        t0 = time.perf_counter()

        response = self.get_response(request)

        elapsed = (time.perf_counter() - t0) * 1000
        m = get_metrics()

        if m["calls"] > 0:
            response["X-ClaraX-Stats"] = (
                f"rust={m['rust_fields']}, "
                f"python={m['python_fields']}, "
                f"calls={m['calls']}, "
                f"ms={m['total_ms']:.1f}"
            )

        return response
