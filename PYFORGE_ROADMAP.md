# ClaraX — Modern Rust-Python Bindings for Production

**Author & Sole Maintainer:** Abdulwahed Mansour  
**Repository:** github.com/abdulwahed-sweden/clarax  
**License:** MIT  
**Version:** 0.1.0  
**Status:** Pre-Publication — Audit Complete

---

## What Is ClaraX

ClaraX is a modernized, production-focused fork of PyO3 (upstream), the Rust-Python binding library.

ClaraX strips away legacy compatibility layers and delivers a clean, high-performance bridge between Rust and modern Python. It targets CPython 3.11+ exclusively, promotes async as first-class, and removes support for alternative interpreters (PyPy, GraalPy) that add complexity without serving modern production stacks.

### Key Differences from PyO3 (Upstream)

| Area | PyO3 0.28.x | ClaraX 0.1.0 |
|------|-------------|----------------|
| Python minimum | 3.8 | **3.11** |
| Async support | `experimental-async` feature flag | **Always enabled, first-class** |
| CPython only | No (PyPy, GraalPy supported) | **Yes — CPython only** |
| Deprecated APIs | Accumulated (GILOnceCell public, old casts) | **Removed** |
| abi3 minimum | py38 | **py311** |
| `extension-module` flag | Deprecated but present | **Removed** |
| `generate-import-lib` flag | Deprecated but present | **Removed** |
| `num-complex` / `num-rational` | Included | **Removed from full feature set** |
| Published crate names | `pyo3-*` | `clarax-*` |

---

## Changes Completed (Phase 1)

### 1. Crate Identity

All 6 workspace crates renamed:

| Old Name | New Name |
|----------|----------|
| `pyo3` | `clarax` |
| `pyo3-ffi` | `clarax-ffi` |
| `pyo3-macros` | `clarax-macros` |
| `pyo3-macros-backend` | `clarax-macros-backend` |
| `pyo3-build-config` | `clarax-build-config` |
| `pyo3-introspection` | `clarax-introspection` |

Author set to **Abdulwahed Mansour** across all crates.

### 2. Python 3.11+ Minimum

- `MINIMUM_SUPPORTED_VERSION` raised from `3.8` to `3.11`
- abi3 feature chain starts at `abi3-py311` (removed `abi3-py38`, `abi3-py39`, `abi3-py310`)
- Build-time assertion enforces CPython 3.11+ and rejects PyPy/GraalPy
- FFI package metadata updated: `min-version = "3.11"`
- PyPy metadata section removed from `clarax-ffi`

### 3. Async First-Class

- Removed `experimental-async` compile-time gates from `pyfunction.rs` and `pymethod.rs`
- Removed `#[cfg(feature = "experimental-async")]` from `coroutine` module
- `experimental-async` is now included in default features
- `async fn` works out of the box with `#[pyfunction]` and `#[pymethods]`

### 4. Deprecated API Cleanup

- Removed deprecated `with_critical_section()` and `with_critical_section2()` public aliases
- Cleaned `GILOnceCell` — removed deprecated annotation, kept as internal-only (`pub(crate)`)
- Removed all `#[allow(deprecated)]` noise from `sync.rs` and `lazy_type_object.rs`
- Removed deprecated `FromPyObject` automatic derivation warning
- Removed deprecated `extension-module` feature flag
- Removed deprecated `generate-import-lib` feature flag
- Removed `num-complex` and `num-rational` from `full` feature set

---

## Development Roadmap

### Phase 1: Foundation (COMPLETE)

- [x] Rename all crates to `clarax-*`
- [x] Set sole authorship to Abdulwahed Mansour
- [x] Raise minimum Python to 3.11
- [x] Make async first-class
- [x] Remove deprecated APIs
- [x] Enforce CPython-only at build time
- [x] Full source rename (`pyo3` -> `clarax` in all Rust code)
- [x] Remove ALL `#[cfg(PyPy)]` and `#[cfg(GraalPy)]` code paths (~925 blocks)
- [x] Remove Python 3.8/3.9/3.10 compatibility shims from FFI
- [x] 925 tests passing, zero compilation errors
- [x] Simplify CI matrix to CPython 3.11+ (removed 3.8/3.9/3.10, PyPy, GraalPy)
- [x] Repository cleanup: remove ClaraX branding, irrelevant examples, old docs, Netlify config
- [ ] Update UI test snapshots for renamed error messages

### Phase 2: Django Integration Layer (COMPLETE)

- [x] Create `clarax-django` crate with workspace integration
- [x] Field type system: 16 Django field types mapped to Rust native types
- [x] Model introspection: extract field descriptors from Django `_meta` API
- [x] Serializer: JSON-compatible output with Decimal precision preservation
- [x] Validator: Rayon-parallel batch validation (threshold: 64 entries)
- [x] Async bridge: GIL-releasing wrappers for ASGI compatibility
- [x] Error system: structured errors mapping to Django's ValidationError
- [x] Python integration layer: DRF mixin, validators, Django AppConfig
- [x] 25 unit tests, zero clippy warnings, zero compilation errors
- [ ] Promote `experimental-inspect` to stable
- [ ] Add `tokio` runtime integration for `#[pyfunction] async fn`

### Phase 3: Performance Benchmarks (COMPLETE)

- [x] Criterion micro-benchmarks: serializer, validator, per-field-type (3 bench files)
- [x] Python comparison benchmarks: serializer and validator vs pure Python
- [x] Results: 288K records/sec serialization, 3-10x speedup over Python
- [x] Rayon parallel validation: 3.6x speedup at 100+ records (900+ entries)
- [x] Per-field-type profiling: BooleanField 252ns → JSONField 1.94µs
- [x] Honest reporting: small batches show 1.5-2x (bridge overhead), large batches 4-8x
- [x] BENCHMARKS.md with full results and methodology

### Pre-Publication Audit (COMPLETE)

- [x] Deep bug hunt: 6 bugs found and fixed with regression tests
  - Bool serialized as int(1)/int(0) instead of True/False
  - valid_count arithmetic underflow when one entry produces multiple errors
  - CharField max_length counted bytes instead of characters (broke multi-byte UTF-8)
  - Rayon parallel validation produced non-deterministic error ordering
  - extract_model_fields emitted Debug format instead of type name string
  - Decimal digit counting algorithm fundamentally wrong (100 counted as 1 digit)
- [x] 33 tests passing (8 new regression tests)
- [x] Zero clippy warnings
- [x] Competitive analysis: vs orjson, pydantic-core, django-ninja
- [x] IMPROVEMENTS.md: 6 future improvements with technical approach documented
- [x] COMPETITIVE_ANALYSIS.md: honest positioning document

### Phase 4: Publish (COMPLETE)

- [x] Publish all 6 crates to crates.io (clarax, clarax-django, clarax-ffi, clarax-macros, clarax-macros-backend, clarax-build-config)
- [x] Publish clarax-django to PyPI (macOS x86_64 wheel)
- [x] CI/CD pipeline: GitHub Actions with multi-platform wheel builds (Linux x86_64/ARM64, macOS x86_64/ARM64, Windows x86_64) for CPython 3.11/3.12/3.13
- [x] Repository renamed from pyo3 to clarax, all internal directory names consistent
- [ ] Full documentation site
- [ ] GitHub release with multi-platform wheels (v0.1.1)

---

## Contribution Policy

**Sole Author and Maintainer:** Abdulwahed Mansour  
**GitHub:** github.com/abdulwahed-sweden

### Commit Convention

```
<type>(<scope>): <description>

Signed-off-by: Abdulwahed Mansour
```

Types: `feat`, `fix`, `refactor`, `docs`, `ci`, `perf`, `test`, `chore`  
Scopes: `core`, `ffi`, `macros`, `build`, `async`

---

## Technology Stack

| Layer | Technology | Constraint |
|-------|-----------|------------|
| Language | Rust 2021 edition | Stable toolchain, MSRV 1.83 |
| Python | CPython | 3.11+ only |
| Build | maturin | Latest |
| Serialization | serde + serde_json | Latest |
| Date/Time | chrono, jiff | Latest |
| UUID | uuid | Latest |
| Decimal | rust_decimal, bigdecimal | Latest |

---

*This document is the authoritative roadmap for the ClaraX project.*  
*All work is performed by Abdulwahed Mansour.*
