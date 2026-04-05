# pyforge-core

Rust-accelerated serialization and validation for Python. Framework-agnostic.

## Installation

```bash
pip install pyforge-core
```

## Quick Start

```python
from pyforge_core import Schema, Field, serialize, validate
from decimal import Decimal
from datetime import datetime
from uuid import UUID

schema = Schema({
    "name": Field(str, max_length=100),
    "age": Field(int, min_value=0, max_value=150),
    "salary": Field(Decimal, max_digits=10, decimal_places=2),
    "joined": Field(datetime),
    "id": Field(UUID),
    "active": Field(bool),
})

data = {
    "name": "Erik",
    "age": 30,
    "salary": Decimal("52000.00"),
    "joined": datetime(2025, 1, 15, 10, 30),
    "id": UUID("12345678-1234-5678-1234-567812345678"),
    "active": True,
}

result = serialize(data, schema)
report = validate(data, schema)
```

## Requirements

- Python 3.11+
- No framework dependencies

## License

MIT — Abdulwahed Mansour
