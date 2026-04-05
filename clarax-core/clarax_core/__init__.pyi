# Author: Abdulwahed Mansour
"""Type stubs for clarax_core."""

from typing import Any, Optional, Union
from decimal import Decimal
from datetime import datetime, date, time
from uuid import UUID

__version__: str
__author__: str

class Schema:
    """A compiled schema that caches field descriptors for repeated use."""

    def __init__(self, fields: dict[str, "Field"]) -> None: ...
    @property
    def field_names_list(self) -> list[str]: ...
    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...

class Field:
    """Defines a single field's type and constraints."""

    def __init__(
        self,
        python_type: type,
        *,
        max_length: Optional[int] = None,
        min_length: Optional[int] = None,
        min_value: Optional[Union[int, float]] = None,
        max_value: Optional[Union[int, float]] = None,
        max_digits: Optional[int] = None,
        decimal_places: Optional[int] = None,
        nullable: bool = False,
        default: bool = False,
    ) -> None: ...
    def __repr__(self) -> str: ...

def serialize(data: Union[dict[str, Any], Any], schema: Schema) -> dict[str, Any]:
    """Serialize a dict or object using a precompiled schema."""
    ...

def serialize_many(data_list: list[Union[dict[str, Any], Any]], schema: Schema) -> list[dict[str, Any]]:
    """Serialize a list of dicts or objects."""
    ...

def validate(data: Union[dict[str, Any], Any], schema: Schema) -> dict[str, Any]:
    """Validate a dict or object against a schema. Returns {is_valid, errors, ...}."""
    ...

def validate_many(data_list: list[Union[dict[str, Any], Any]], schema: Schema) -> dict[str, Any]:
    """Validate a list of dicts or objects. Returns combined report."""
    ...

def version() -> str:
    """Return the clarax-core version string."""
    ...
