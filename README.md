[![crates.io](https://img.shields.io/crates/v/pyforge-django.svg)](https://crates.io/crates/pyforge-django)
[![PyPI](https://img.shields.io/pypi/v/pyforge-django.svg)](https://pypi.org/project/pyforge-django)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Python 3.11+](https://img.shields.io/badge/python-3.11%2B-blue.svg)](https://www.python.org)
[![Django 4.2 | 5.x](https://img.shields.io/badge/django-4.2%20LTS%20%7C%205.x-green.svg)](https://www.djangoproject.com)
[![CI](https://img.shields.io/github/actions/workflow/status/abdulwahed-sweden/pyforge/ci.yml?branch=main)](https://github.com/abdulwahed-sweden/pyforge/actions)

# PyForge

Rust-accelerated serialization and validation for Django — drop-in, zero rewrite.

## What Is PyForge

PyForge moves Django REST Framework's serialization and validation hot paths
from Python to Rust. It reads your existing Django model definitions, compiles
a typed schema once at startup, and processes field values through Rust-native
code on every request. The result is 30-50x faster serialization and validation
for list views, bulk operations, and any endpoint that touches more than a
handful of records.

It is designed for Django developers running production APIs on DRF who need
better throughput without migrating to a different framework.

## Benchmarks

Measured against DRF `ModelSerializer` — PyForge's actual replacement target.
CPython 3.12, Django 6.0, in-memory SQLite, macOS.
Model: 9-field `RentalApplication` (CharField, DecimalField, DateTimeField, UUIDField, etc.)

| Scenario | DRF | PyForge | Speedup |
|---|---|---|---|
| Serialize 100 instances | 40.8 ms | 1.2 ms | **33x** |
| Serialize 1,000 instances | 475.2 ms | 14.6 ms | **33x** |
| Validate 100 instances | 49.8 ms | 963 µs | **52x** |
| Validate 1,000 instances | 506.0 ms | 10.2 ms | **50x** |

Benchmarks run against DRF ModelSerializer — PyForge's actual replacement target.
Comparison against raw dict comprehensions is not meaningful.
Small batches (<10 records) may show minimal gain due to Rust/Python bridge overhead.
Database query time is not included — PyForge accelerates serialization and validation only.

Full methodology and reproduction steps: [BENCHMARKS.md](BENCHMARKS.md)

## Installation

```bash
pip install pyforge-django
```

Add the app to your Django settings:

```python
INSTALLED_APPS = [
    ...
    "django_pyforge",
]
```

## Quickstart

```python
from django_pyforge import ModelSchema, serialize_instance, validate_instance
from django_pyforge.serializers import RustSerializerMixin
from rest_framework import serializers

# 1. Compile a schema once (e.g., in AppConfig.ready)
schema = ModelSchema(RentalApplication)

# 2. Serialize a single instance — one Rust call, all fields
result = serialize_instance(instance, schema)

# 3. Validate an instance against the schema
report = validate_instance(instance, schema)
if not report["is_valid"]:
    print(report["errors"])

# 4. Or use the DRF mixin — zero changes to your existing code
class ApplicationSerializer(RustSerializerMixin, serializers.ModelSerializer):
    class Meta:
        model = RentalApplication
        fields = "__all__"
```

## Supported Django Field Types

| Django Field | Rust Type | Notes |
|---|---|---|
| CharField | `String` | max_length enforced in characters, not bytes |
| TextField | `String` | No length limit |
| IntegerField | `i32` | Full range |
| BigIntegerField | `i64` | Full range |
| DecimalField | `rust_decimal::Decimal` | Full precision preserved — never converted to float |
| DateField | `chrono::NaiveDate` | ISO 8601 format |
| DateTimeField | `chrono::DateTime<Utc>` | RFC 3339 with timezone (USE_TZ=True) |
| TimeField | `chrono::NaiveTime` | ISO 8601 format |
| UUIDField | `uuid::Uuid` | Hyphenated and non-hyphenated accepted |
| BooleanField | `bool` | Serialized as True/False, never as 1/0 |
| FloatField | `f64` | NaN and Infinity rejected at serialization |
| JSONField | `serde_json::Value` | Full nested structure preserved |
| BinaryField | `Vec<u8>` | Base64 encoded for JSON transport |
| EmailField | `String` | max_length enforced |
| URLField | `String` | max_length enforced |
| SlugField | `String` | Character set validated |

## Requirements

- **Python** 3.11 or newer
- **Django** 4.2 LTS or 5.x
- **Rust** 1.75+ (only needed when building from source)
- **Platforms:** Linux, macOS, Windows (manylinux wheels published to PyPI)

## Architecture

PyForge has three layers. `ModelSchema` is a Rust `#[pyclass]` that reads
Django's `_meta` API once and compiles field descriptors into a cached struct.
`serialize_instance` takes a Django model object and the cached schema, extracts
every field value via `getattr` in a single Rust call, converts each to a native
Rust type (Decimal, DateTime, UUID, etc.), serializes to JSON-compatible output,
and returns a Python dict. `RustSerializerMixin` sits on top as a DRF mixin that
automatically classifies which fields are simple model fields (Rust-accelerated)
and which are computed fields (delegated back to DRF). For validation batches
above 64 fields, Rayon distributes work across CPU cores while the GIL is released.

## Crates

| Crate | Description |
|---|---|
| [`pyforge`](https://crates.io/crates/pyforge) | Core Rust-Python binding library |
| [`pyforge-django`](https://crates.io/crates/pyforge-django) | Django integration layer |
| `pyforge-ffi` | CPython C API bindings |
| `pyforge-macros` | Procedural macros |
| `pyforge-build-config` | Build-time Python detection |

## License

MIT — [Abdulwahed Mansour](https://github.com/abdulwahed-sweden)
