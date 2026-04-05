# Rename Plan: ClaraX → ClaraX

**Author:** Abdulwahed Mansour
**Scope:** ~3,100 references across ~200 files + 10 directories + GitHub repo

---

## Phase 1 — Name Mapping

### Crate/Package Names

| Old | New | Type |
|---|---|---|
| `clarax` | `clarax` | Root binding crate |
| `clarax-core` | `clarax-core` | Framework-agnostic engine |
| `clarax-django` | `clarax-django` | Django integration |
| `clarax-ffi` | `clarax-ffi` | CPython C API bindings |
| `clarax-macros` | `clarax-macros` | Proc macros |
| `clarax-macros-backend` | `clarax-macros-backend` | Macro code gen |
| `clarax-build-config` | `clarax-build-config` | Build detection |
| `clarax-introspection` | `clarax-introspection` | Stubs introspection |
| `clarax-benches` | `clarax-benches` | Benchmarks |
| `clarax-ffi-check` | `clarax-ffi-check` | FFI verification |
| `clarax-pytests` | `clarax-pytests` | Python integration tests |

### Python Import Names

| Old | New |
|---|---|
| `import clarax_core` | `import clarax_core` |
| `from clarax_core import ...` | `from clarax_core import ...` |
| `import django_clarax` | `import django_clarax` |
| `from django_clarax import ...` | `from django_clarax import ...` |
| `import clarax_django` (native) | `import clarax_django` (native) |

### Directory Names

| Old | New |
|---|---|
| `clarax-core/` | `clarax-core/` |
| `clarax-core/clarax_core/` | `clarax-core/clarax_core/` |
| `clarax-django/` | `clarax-django/` |
| `clarax-django/django_clarax/` | `clarax-django/django_clarax/` |
| `clarax-ffi/` | `clarax-ffi/` |
| `clarax-macros/` | `clarax-macros/` |
| `clarax-macros-backend/` | `clarax-macros-backend/` |
| `clarax-build-config/` | `clarax-build-config/` |
| `clarax-introspection/` | `clarax-introspection/` |
| `clarax-benches/` | `clarax-benches/` |
| `clarax-ffi-check/` | `clarax-ffi-check/` |

### Branding

| Old | New |
|---|---|
| `ClaraX` | `ClaraX` |
| `clarax` | `clarax` |
| `CLARAX` | `CLARAX` |

### URLs

| Old | New |
|---|---|
| `github.com/abdulwahed-sweden/clarax` | `github.com/abdulwahed-sweden/clarax` |
| `crates.io/crates/clarax*` | `crates.io/crates/clarax*` |
| `pypi.org/project/clarax-*` | `pypi.org/project/clarax-*` |
| `docs.rs/clarax` | `docs.rs/clarax` |

### Environment Variables (keep PYO3_ prefix — these are PyO3 standard)

| Variable | Action |
|---|---|
| `PYO3_PYTHON` | KEEP — this is PyO3's env var, not ours |
| `PYO3_CONFIG_FILE` | KEEP |
| `PYO3_NO_PYTHON` | KEEP |
| `PYO3_CROSS_*` | KEEP |
| `CLARAX_METRICS` | → `CLARAX_METRICS` |
| `CLARAX_DEBUG` | → `CLARAX_DEBUG` |

---

## Phase 2 — Execution Order

1. Rename all directories (mv)
2. Bulk text replace in all source files
3. Update Cargo.toml workspace members
4. Update all pyproject.toml files
5. Update all Python __init__.py and import paths
6. Update all Rust use/extern crate statements
7. Update all documentation
8. Update GitHub workflows
9. Update publish scripts
10. Verify: cargo check --workspace
11. Verify: cargo test --workspace
12. Rename GitHub repo
13. Update git remote URL
14. Publish new crates to crates.io
15. Publish new packages to PyPI

---

## Phase 3 — What NOT to change

- `PYO3_*` environment variables (these are PyO3 standard, not ours)
- `pyo3_build_config` function names in build.rs (these reference the upstream crate API)
- External URLs to PyO3/maturin, PyO3/setuptools-rust (real upstream projects)
- `rules_pyo3` references (real third-party project)
- Git history (no rebase — clean rename commit)

---

## Phase 4 — Verification

- `cargo check --workspace` — zero errors
- `cargo clippy --workspace -- -D warnings` — zero warnings on clarax crates
- `cargo test --workspace` — all tests pass
- Python imports work: `from clarax_core import Schema`
- Python imports work: `from django_clarax import ModelSchema`
- Hyra integration: update and run 59 tests
