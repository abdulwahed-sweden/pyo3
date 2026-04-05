# ClaraX Health Report — v0.2.0

**Date:** 2026-04-05
**Author:** Abdulwahed Mansour

## Critical Issues (fixed)

### 1. Broken benchmarks after clarax-core refactor
**File:** `clarax-django/benches/bench_serializer.rs`, `bench_validator.rs`, `bench_field_types.rs`
**Problem:** `FieldValue::Integer` changed from `i32` to `i64` and `FieldValue::BigInteger` was removed during the refactor to clarax-core. The three benchmark files used the old types.
**Fix:** Updated all `Integer(i32)` to `Integer(i64)`, removed `BigInteger` references.
**Status:** Fixed. `cargo check -p clarax-django --benches` compiles clean.

## High Issues (fixed)

### 2. No VS Code workspace configuration
**Problem:** No `.vscode/` directory existed. Developers cloning the repo had no IDE guidance for a Rust+Python polyglot workspace.
**Fix:** Created `settings.json`, `extensions.json`, `tasks.json`, `launch.json` with rust-analyzer, ruff, clippy, and debugger configurations.
**Status:** Fixed.

## Medium Issues (documented)

### 3. No `cargo-audit` or `cargo-outdated` installed
**Problem:** Cannot verify dependency security or staleness.
**Proposed fix:** Add `cargo install cargo-audit cargo-outdated` to CI and developer setup docs.

### 4. `__pycache__` directories committed or present in source
**Problem:** `.gitignore` has the rule but `clarax-django/django_clarax/__pycache__/` exists locally.
**Proposed fix:** Already in `.gitignore` — no action needed. Local-only.

### 5. clarax-core's `Bound::cast` may panic on non-dict input
**File:** `clarax-core/src/lib.rs:319`
**Problem:** `data.cast::<PyDict>()` can fail if the input is an object, but the error path falls through to `getattr`. This is correct behavior but the error message on failure is opaque ("could not extract...").
**Proposed fix:** Add a clearer error when neither dict access nor getattr succeeds.

### 6. Maturin cannot detect pyo3 bindings (renamed fork)
**Problem:** Maturin auto-detects binding type by searching for a crate named `pyo3`. Since ClaraX renamed the crate to `clarax`, maturin cannot detect it. This blocks `maturin develop` and `maturin publish`.
**Workaround:** Manual wheel assembly via `publish-pypi.sh`.
**Long-term fix:** Contribute a PR to maturin to support renamed pyo3 forks, or maintain the manual build path.

### 7. Stale venv path cached in build artifacts
**Problem:** After the `pyo3` → `clarax` directory rename, `.venv/bin/pip` and some build caches point to the old path `/Users/mansour/Documents/GitHub/pyo3/.venv/`.
**Fix:** Recreate venv when moving the project directory. Document in CONTRIBUTING.md.

## Low Issues (documented)

### 8. Pedantic clippy warnings in inherited PyO3 code
**Count:** ~40 warnings with `-W clippy::pedantic`
**Categories:** `unnecessary_structure_name_repetition`, `item_in_documentation_is_missing_backticks`, `too_many_lines`
**Decision:** Not fixing — these are in the inherited PyO3 codebase (3000+ files), not in our code. Our crates (clarax-core, clarax-django) have zero clippy warnings.

### 9. No Python linting tools installed in project venv
**Problem:** `ruff`, `mypy`, `pylint` are not in the project venv.
**Proposed fix:** Add `ruff` and `mypy` to dev dependencies. Not critical because the Python code is thin (< 200 lines).

### 10. clarax-core and clarax-django have slightly different FieldValue enums
**Problem:** clarax-core uses `Integer(i64)` only. clarax-django's old API had `Integer(i32)` and `BigInteger(i64)` as separate variants. The refactor unified them, but code in benches and documentation may still reference the old variants.
**Status:** Fixed in benches. Docs updated.

## Cosmetic Issues

### 11. CLARAX_CORE_PLAN.md has markdown lint warnings
**Problem:** 30+ markdownlint warnings (table column spacing, fenced code language, trailing punctuation).
**Proposed fix:** This is a planning document, not user-facing. Can be cleaned up but not urgent.

## Summary

| Severity | Count | Fixed | Documented |
|----------|-------|-------|------------|
| Critical | 1 | 1 | 0 |
| High | 1 | 1 | 0 |
| Medium | 5 | 0 | 5 |
| Low | 3 | 1 | 2 |
| Cosmetic | 1 | 0 | 1 |
