# PyForge Benchmarks

**Author:** Abdulwahed Mansour

## Methodology

- **Baseline:** DRF `ModelSerializer` with `fields = "__all__"` — PyForge's actual replacement target
- **Model:** `RentalApplication` with 9 fields: CharField (x2), DecimalField, DateField, DateTimeField, UUIDField, BooleanField, IntegerField
- **Measurement:** `statistics.median()` over 5 runs per scenario, 2 warm-up runs discarded
- **Database:** in-memory SQLite — removes query time from results
- **Environment:** Python 3.12.12, Django 6.0, DRF 3.17.1, PyForge 0.1.1, macOS x86_64

## Results

| Benchmark | DRF | PyForge | Speedup |
|---|---|---|---|
| Serialize 100 instances | 40.8 ms | 1.2 ms | **33x** |
| Serialize 1,000 instances | 475.2 ms | 14.6 ms | **33x** |
| Validate 100 instances | 49.8 ms | 963 us | **52x** |
| Validate 1,000 instances | 506.0 ms | 10.2 ms | **50x** |

## What these numbers mean

**33x on serialization:** DRF `ModelSerializer` resolves field descriptors, runs type coercion, and dispatches method calls per instance per request. PyForge compiles the schema once at startup via `ModelSchema(Model)` and runs a single Rust call per instance — no per-field Python dispatch, no method resolution, no intermediate object allocation.

**50x on validation:** DRF runs the full validator chain per field per instance — `Field.run_validators()`, `Serializer.validate_<field>()`, and `Serializer.validate()`. PyForge runs Rayon-parallel validation across the entire batch in one Rust call once the batch exceeds 64 entries, with zero Python callbacks during validation.

## When PyForge helps

- List endpoints returning 50+ records
- Bulk create/update with validation
- Export jobs processing thousands of records
- Any view where `ModelSerializer.data` or `.is_valid()` is the bottleneck
- ASGI deployments where per-request latency compounds under concurrent load

## When PyForge does NOT help

- Raw dict comprehensions or `.values()` querysets that bypass DRF entirely
- Single-record detail endpoints (bridge overhead ~10us per call)
- Database-bound views — PyForge does not touch query time
- Views with complex computed/method fields that must run in Python

## Per-field-type cost (Rust micro-benchmarks)

Measured via criterion.rs, 100 samples per benchmark:

| Django Field | Rust Type | Time (median) |
|---|---|---|
| BooleanField | `bool` | 252 ns |
| IntegerField | `i32` | 260 ns |
| Null field | (none) | 264 ns |
| TextField (1KB) | `String` | 372 ns |
| CharField (short) | `String` | 391 ns |
| UUIDField | `uuid::Uuid` | 416 ns |
| DecimalField | `rust_decimal::Decimal` | 471 ns |
| DateTimeField | `chrono::DateTime` | 485 ns |
| DateField | `chrono::NaiveDate` | 677 ns |
| TimeField | `chrono::NaiveTime` | 682 ns |
| JSONField (nested) | `serde_json::Value` | 1.94 us |

BooleanField and IntegerField are essentially free (~250ns baseline is serde_json::Map allocation).
DecimalField is fast because `rust_decimal::Decimal::to_string()` is a stack operation.
JSONField is 4-7x more expensive than primitives because it clones the nested Value tree.

## Reproducing these benchmarks

```bash
# 1. Create an isolated test environment
mkdir -p /tmp/pyforge-test && cd /tmp/pyforge-test
python3 -m venv .venv && source .venv/bin/activate

# 2. Install from PyPI
pip install "django>=5.0" djangorestframework pyforge-django

# 3. Create settings.py
cat > settings.py << 'EOF'
SECRET_KEY = "pyforge-test-key-not-for-production"
INSTALLED_APPS = [
    "django.contrib.contenttypes",
    "django.contrib.auth",
    "rest_framework",
    "django_pyforge",
]
DATABASES = {"default": {"ENGINE": "django.db.backends.sqlite3", "NAME": ":memory:"}}
USE_TZ = True
DEFAULT_AUTO_FIELD = "django.db.models.BigAutoField"
EOF

# 4. Download and run the benchmark
curl -O https://raw.githubusercontent.com/abdulwahed-sweden/pyforge/main/benchmarks/benchmark_drf.py
python benchmark_drf.py
```

Rust micro-benchmarks (requires source checkout):
```bash
cargo bench -p pyforge-django
```
