# Changelog

All notable changes to PyForge will be documented here.

Format: [keepachangelog.com](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-04-04

### Added

- `ModelSchema`: compiled field descriptor cache built once at Django startup,
  reused on every request with zero per-request parsing overhead
- `serialize_instance()`: single-call serialization — extracts all field values
  from a Django model instance in one Rust call, reducing boundary crossings from N+1 to 1
- `serialize_batch()`: queryset serialization processing a list of instances in one call
- `validate_instance()`: schema-based validation with structured error output and `is_valid` flag
- `RustSerializerMixin`: drop-in DRF mixin that automatically classifies fields into
  Rust-accelerated (simple model fields) and Python-delegated (SerializerMethodField,
  nested serializers, custom source attributes)
- Rayon parallel validation for batches above 64 fields with deterministic error ordering
- Full Django field type support: CharField, TextField, IntegerField, BigIntegerField,
  DecimalField, DateField, TimeField, DateTimeField, UUIDField, BooleanField,
  FloatField, JSONField, BinaryField, EmailField, URLField, SlugField
- DecimalField precision preservation — values serialized as strings, never converted to float
- CharField max_length validation counts characters, not bytes (correct for multi-byte UTF-8)
- ASGI-compatible async bridge with proper GIL release for Django async views
- Django 4.2 LTS and 5.x support
- Python 3.11, 3.12, 3.13 support
- CPython-only build (PyPy and GraalPy removed)
- Minimum Python version raised to 3.11 (removed 3.8/3.9/3.10 compatibility shims)
- Criterion micro-benchmarks for serializer, validator, and per-field-type cost
- Python comparison benchmarks against pure Python DRF-style serialization
- PEP 561 type stubs for IDE autocompletion
- 33 Rust unit tests + 13 Django integration tests + 898 core tests = 944 total

### Security

- All string inputs validated through PyForge extract() — no UTF-8 panic paths
- Memory exhaustion protection via Django's field size limit enforcement
- Bounds-checked indexing throughout — no buffer overflow vectors
- No `unwrap()` on user-controlled input — all fallible operations return `DjangoError`

[0.1.0]: https://github.com/abdulwahed-sweden/pyforge/releases/tag/v0.1.0
