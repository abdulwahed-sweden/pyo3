# Contributing to PyForge

Thank you for your interest in contributing. PyForge is a Rust-accelerated
Django integration library, so contributions can touch Rust, Python, or both.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/abdulwahed-sweden/pyforge.git
cd pyforge

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python dependencies
python -m pip install maturin django djangorestframework pytest pytest-django

# Build the native extension in development mode
cd pyforge-django
maturin develop

# Run Rust tests
cargo test --workspace

# Run Django integration tests
cd pyforge-django
pytest tests/ -v
```

## Running Benchmarks

```bash
# Rust micro-benchmarks
cargo bench -p pyforge-django

# Python comparison benchmarks
python benchmarks/bench_serializer_comparison.py
python benchmarks/bench_validator_comparison.py
```

## What Makes a Good PR

- **One concern per PR.** A bug fix and a feature should be separate PRs.
- **Tests required.** Every behavior change needs a test that would have caught
  a regression. Rust changes need Rust tests. Python changes need pytest tests.
- **Benchmark if relevant.** If your change touches the serializer or validator
  hot path, include before/after benchmark numbers.
- **No performance regressions.** Run `cargo bench` before and after your change.

## Commit Messages

```
<type>(<scope>): <description>

Types: feat, fix, perf, docs, test, chore
Scopes: django, core, ffi, macros, build
```

Examples:
- `feat(django): add ArrayField support for PostgreSQL`
- `fix(django): correct DecimalField digit counting for values < 1`
- `perf(django): reduce boundary crossings in serialize_batch`

## Project Structure

```
pyforge-django/
├── src/                    # Rust core
│   ├── lib.rs              # #[pymodule] entry + ModelSchema #[pyclass]
│   ├── model.rs            # Django _meta extraction + Python→Rust conversion
│   ├── serializer.rs       # Rust serialization engine
│   ├── validator.rs        # Rayon-parallel validation
│   ├── field_types.rs      # Django field type system
│   ├── async_bridge.rs     # GIL-releasing wrappers
│   └── error.rs            # Error types → Python exceptions
├── django_pyforge/         # Python integration
│   ├── __init__.py         # Re-exports from native extension
│   ├── serializers.py      # RustSerializerMixin for DRF
│   ├── validators.py       # Validation utilities
│   └── apps.py             # Django AppConfig
├── benches/                # Criterion micro-benchmarks
└── tests/                  # Django integration tests
```

## License

By contributing, you agree that your contributions will be licensed under MIT.
