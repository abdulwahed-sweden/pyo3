# Contributing to ClaraX

Thank you for your interest in contributing. ClaraX is a Rust-accelerated
Django integration library, so contributions can touch Rust, Python, or both.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/abdulwahed-sweden/clarax.git
cd clarax

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python dependencies
python -m pip install maturin django djangorestframework pytest pytest-django

# Build the native extension in development mode
cd clarax-django
maturin develop

# Run Rust tests
cargo test --workspace

# Run Django integration tests
cd clarax-django
pytest tests/ -v
```

## Running Benchmarks

```bash
# Rust micro-benchmarks
cargo bench -p clarax-django

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
clarax-django/
├── src/                    # Rust core
│   ├── lib.rs              # #[pymodule] entry + ModelSchema #[pyclass]
│   ├── model.rs            # Django _meta extraction + Python→Rust conversion
│   ├── serializer.rs       # Rust serialization engine
│   ├── validator.rs        # Rayon-parallel validation
│   ├── field_types.rs      # Django field type system
│   ├── async_bridge.rs     # GIL-releasing wrappers
│   └── error.rs            # Error types → Python exceptions
├── django_clarax/         # Python integration
│   ├── __init__.py         # Re-exports from native extension
│   ├── serializers.py      # RustSerializerMixin for DRF
│   ├── validators.py       # Validation utilities
│   └── apps.py             # Django AppConfig
├── benches/                # Criterion micro-benchmarks
└── tests/                  # Django integration tests
```

## Publishing

For maintainers with publish access:

```bash
# 1. Copy the example env file and fill in your tokens
cp .env.example .env
# Edit .env — add your PyPI token and crates.io token

# 2. Build wheels without publishing (to verify)
./publish-pypi.sh --build

# 3. Publish to PyPI
./publish-pypi.sh all          # both clarax-core and clarax-django
./publish-pypi.sh core         # clarax-core only
./publish-pypi.sh django       # clarax-django only

# 4. Publish to crates.io (all 7 crates in dependency order)
./publish-crates.sh
```

Tokens are loaded from `.env` which is gitignored. Never commit tokens.

## License

By contributing, you agree that your contributions will be licensed under MIT.
