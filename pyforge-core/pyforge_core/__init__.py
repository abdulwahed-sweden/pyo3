# Author: Abdulwahed Mansour
"""
pyforge-core — Rust-accelerated serialization and validation for Python.

Framework-agnostic. Works with Flask, FastAPI, scripts, ETL pipelines,
or any Python code that processes structured data.

Usage:
    from pyforge_core import Schema, Field, serialize, serialize_many, validate, validate_many
"""

__author__ = "Abdulwahed Mansour"
__version__ = "0.2.0"

from pyforge_core._native import (
    Schema,
    Field,
    serialize,
    serialize_many,
    validate,
    validate_many,
    version,
)

__all__ = [
    "Schema",
    "Field",
    "serialize",
    "serialize_many",
    "validate",
    "validate_many",
    "version",
]
