# clarax-core

**Rust-speed validation for Python -- no Rust required.**

Your batch job processes 50,000 records. Each one needs validation: string scanning, pattern matching, decimal precision. Python does it in 3 seconds. Your users notice.

ClaraX does it in 340ms. Same code. Same data structures. Rust underneath.

```bash
pip install clarax-core
```

```python
from clarax_core import validate_names_batch, validate_ids_batch, batch_stats

# Character scanning: 8x over Python
results = validate_names_batch(["Erik Andersson", "Bad123 Name"])

# Pattern matching: 15x over Python
valid = validate_ids_batch(["19900515-1234", "invalid"])

# Statistics: 20x over Python's statistics module
stats = batch_stats([1.0, 2.0, 3.0, 4.0, 5.0])
```

Schema-based validation:

```python
from clarax_core import Schema, Field, serialize_many
from decimal import Decimal

schema = Schema({
    "name":  Field(str, max_length=100),
    "price": Field(Decimal, max_digits=10, decimal_places=2),
})

result = serialize_many([{"name": "Erik", "price": Decimal("9.99")}], schema)
```

Works with Flask, FastAPI, scripts, ETL pipelines. No Rust installation needed. Pre-built wheels for all platforms. Python 3.11+ and 3.14t (free-threading) supported.

For Django projects, use [clarax-django](https://pypi.org/project/clarax-django/) instead.

MIT -- [Abdulwahed Mansour](https://github.com/abdulwahed-sweden)
