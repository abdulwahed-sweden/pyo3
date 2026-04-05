# Author: Abdulwahed Mansour
"""
Constraint markers for use with typing.Annotated.

Usage:
    from typing import Annotated
    from clarax_core.constraints import MaxLength, MinValue

    @dataclass
    class User:
        name: Annotated[str, MaxLength(100)]
        age: Annotated[int, MinValue(0), MaxValue(150)]
"""


class MaxLength:
    """Maximum string or bytes length."""
    def __init__(self, value: int):
        self.max_length = value


class MinLength:
    """Minimum string length."""
    def __init__(self, value: int):
        self.min_length = value


class MinValue:
    """Minimum numeric value (int or float)."""
    def __init__(self, value):
        self.min_value = value


class MaxValue:
    """Maximum numeric value (int or float)."""
    def __init__(self, value):
        self.max_value = value


class MaxDigits:
    """Maximum total digits for Decimal."""
    def __init__(self, value: int):
        self.max_digits = value


class DecimalPlaces:
    """Maximum decimal places for Decimal."""
    def __init__(self, value: int):
        self.decimal_places = value
