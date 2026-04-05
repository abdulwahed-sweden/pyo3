# Author: Abdulwahed Mansour
"""
Auto-generate Schema from Python dataclasses, TypedDict, or NamedTuple.

Usage:
    from dataclasses import dataclass
    from clarax_core import Schema
    from clarax_core.auto_schema import from_dataclass, from_typeddict

    @dataclass
    class User:
        name: str
        age: int
        email: str

    schema = from_dataclass(User)
    # Equivalent to: Schema({"name": Field(str), "age": Field(int), "email": Field(str)})
"""

import dataclasses
import typing
from datetime import date, datetime, time
from decimal import Decimal
from uuid import UUID

from clarax_core._native import Field, Schema

# Types that map directly to Field types
_TYPE_MAP = {
    str: str,
    int: int,
    float: float,
    bool: bool,
    Decimal: Decimal,
    datetime: datetime,
    date: date,
    time: time,
    UUID: UUID,
    list: list,
    dict: dict,
    bytes: bytes,
}


def _resolve_type(annotation):
    """Resolve a type annotation to a (python_type, nullable) pair."""
    origin = typing.get_origin(annotation)

    # Optional[X] = Union[X, None]
    if origin is typing.Union:
        args = typing.get_args(annotation)
        non_none = [a for a in args if a is not type(None)]
        if len(non_none) == 1:
            return non_none[0], True
        return str, True  # complex union → treat as str

    # Annotated[X, ...] — extract the base type
    if origin is typing.Annotated:
        args = typing.get_args(annotation)
        return args[0], False

    # Direct type
    if annotation in _TYPE_MAP:
        return annotation, False

    # Unknown → str fallback
    return str, False


def _extract_constraints(annotation):
    """Extract constraints from Annotated metadata if present."""
    origin = typing.get_origin(annotation)
    constraints = {}

    if origin is typing.Annotated:
        args = typing.get_args(annotation)
        for meta in args[1:]:
            if hasattr(meta, "max_length"):
                constraints["max_length"] = meta.max_length
            if hasattr(meta, "min_length"):
                constraints["min_length"] = meta.min_length
            if hasattr(meta, "min_value"):
                constraints["min_value"] = meta.min_value
            if hasattr(meta, "max_value"):
                constraints["max_value"] = meta.max_value
            if hasattr(meta, "max_digits"):
                constraints["max_digits"] = meta.max_digits
            if hasattr(meta, "decimal_places"):
                constraints["decimal_places"] = meta.decimal_places

    return constraints


def from_dataclass(cls):
    """Generate a Schema from a dataclass.

    Args:
        cls: A dataclass class (not an instance).

    Returns:
        A compiled Schema.

    Raises:
        TypeError: If cls is not a dataclass.
    """
    if not dataclasses.is_dataclass(cls):
        raise TypeError(f"{cls.__name__} is not a dataclass")

    fields = {}
    for dc_field in dataclasses.fields(cls):
        python_type, nullable = _resolve_type(dc_field.type)
        constraints = _extract_constraints(dc_field.type)

        has_default = (
            dc_field.default is not dataclasses.MISSING
            or dc_field.default_factory is not dataclasses.MISSING
        )

        fields[dc_field.name] = Field(
            _TYPE_MAP.get(python_type, str),
            nullable=nullable,
            default=has_default,
            **constraints,
        )

    return Schema(fields)


def from_typeddict(cls):
    """Generate a Schema from a TypedDict.

    Args:
        cls: A TypedDict class.

    Returns:
        A compiled Schema.
    """
    hints = typing.get_type_hints(cls, include_extras=True)
    required_keys = getattr(cls, "__required_keys__", set(hints.keys()))

    fields = {}
    for name, annotation in hints.items():
        python_type, nullable = _resolve_type(annotation)
        constraints = _extract_constraints(annotation)

        fields[name] = Field(
            _TYPE_MAP.get(python_type, str),
            nullable=nullable,
            default=name not in required_keys,
            **constraints,
        )

    return Schema(fields)
