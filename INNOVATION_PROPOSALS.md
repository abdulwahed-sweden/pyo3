# ClaraX Innovation Proposals

**Author:** Abdulwahed Mansour
**Date:** 2026-04-05
**Version:** v0.2.0

Proposals ranked by impact. Each one is independently implementable.

---

## 1. Queryset-to-Rust batch path (break the 2.5x ceiling)

**What:** Pass an entire Django queryset's `.values_list()` output to Rust in a single call, eliminating the per-instance Python↔Rust bridge crossing.

**Why:** The current 2.3-2.6x speedup ceiling exists because `serialize_instance()` is called once per object, and each call crosses the Python↔Rust boundary (~10us). For 3,000 objects, that's 30ms of pure bridge overhead. If we pass all 3,000 objects' field values in a single `PyList` of `PyTuple`s, the bridge is crossed exactly once.

**How:** Add `serialize_queryset(qs.values_list(*schema.field_names), schema)` that takes a list-of-tuples (the output of `QuerySet.values_list()`). In Rust, iterate the tuples without individual `getattr` calls. The values_list output is already in Python tuple form — extraction to Rust types is cheaper than `getattr` on model instances. Estimated speedup: 5-10x over DRF.

**Effort:** Medium
**Impact:** Game-changing

---

## 2. `clarax doctor` CLI command

**What:** A management command that audits a Django project and reports which serializers would benefit from ClaraX, which wouldn't, and why.

**Why:** The #1 friction point for adoption is "which serializers should I add the mixin to?" We learned from Hyra that `ListingListSerializer` was slower with the mixin due to computed fields. A doctor command would have caught this before the developer wasted time benchmarking.

**How:** `python manage.py clarax_doctor` scans all ModelSerializer subclasses in INSTALLED_APPS, classifies their fields (model vs computed vs nested), and prints a report like:
```
QueueEntrySerializer: 15/17 fields Rust-compatible → RECOMMENDED (est. 2.5x)
ListingListSerializer: 17/21 fields Rust-compatible → NOT RECOMMENDED (4 computed fields)
ApplicationSerializer: 9/11 fields Rust-compatible → RECOMMENDED (est. 2.5x)
```

**Effort:** Low
**Impact:** High

---

## 3. Auto-schema from dataclass / TypedDict

**What:** `Schema.from_dataclass(UserData)` that introspects a Python dataclass or TypedDict and generates the Schema automatically.

**Why:** clarax-core requires manual `Schema({"name": Field(str, max_length=100), ...})` definitions. If a developer already has a `@dataclass` or `TypedDict`, they shouldn't have to repeat the type information.

**How:** In Python, inspect `__dataclass_fields__` or `__annotations__`. Map `str` → `Field(str)`, `int` → `Field(int)`, `Optional[str]` → `Field(str, nullable=True)`, `Annotated[str, MaxLength(100)]` → `Field(str, max_length=100)`. The Schema is built in Python and compiled to Rust — no Rust changes needed.

**Effort:** Low
**Impact:** High

---

## 4. Request-level metrics middleware

**What:** Django middleware that tracks per-request ClaraX metrics: fields_accelerated, fields_delegated, time_saved_ms, and exposes them via a `X-ClaraX-Stats` response header.

**Why:** Observability. In production, you want to know "is ClaraX actually doing anything on this endpoint?" Without metrics, developers add the mixin and hope. With metrics, they can see exactly which endpoints benefit and by how much.

**How:** Add `ClaraXMetricsMiddleware` that reads from a thread-local counter incremented by `RustSerializerMixin.to_representation()`. After the view returns, attach `X-ClaraX-Stats: rust=15,python=2,saved=4.2ms` to the response. Optional — only active when `CLARAX_METRICS = True` in settings.

**Effort:** Low
**Impact:** Medium

---

## 5. FastAPI / Pydantic response model

**What:** `clarax_core.FastAPIModel` that wraps a `Schema` and can be used as a FastAPI response model, replacing Pydantic's serialization with Rust.

**Why:** clarax-core is framework-agnostic but has no FastAPI integration yet. FastAPI is the #2 Python web framework. Pydantic v2 already uses Rust (pydantic-core), but clarax-core could offer a simpler, faster alternative for serialization-only use cases.

**How:** Create a `clarax-fastapi` Python package (no Rust needed) that wraps `Schema` in a class compatible with FastAPI's `response_model` parameter. When FastAPI calls `.model_dump()` or `.__get_validators__()`, delegate to `clarax_core.serialize()`. This is a Python-only package that depends on `clarax-core`.

**Effort:** Medium
**Impact:** High

---

## 6. N+1 query detector in serializer

**What:** Detect when a serializer causes N+1 database queries and emit a warning with the fix.

**Why:** The #1 Django performance issue is N+1 queries from nested serializers. ClaraX already classifies fields as "model" vs "nested/computed". If a nested serializer triggers a query per instance, ClaraX could detect the missing `select_related` / `prefetch_related` and warn.

**How:** In `RustSerializerMixin.to_representation()`, before delegating a Python field, check if the field accesses a ForeignKey attribute by inspecting `field.source`. If the FK isn't in the queryset's `select_related` set (accessible via `instance._state.fields_cache`), log a warning: "Field 'landlord_name' accesses ForeignKey 'landlord' which is not select_related — add .select_related('landlord') to your queryset."

**Effort:** Medium
**Impact:** High

---

## 7. Schema pre-warming via Django signal

**What:** Automatically compile `ModelSchema` for all registered serializers at Django startup via `AppConfig.ready()`.

**Why:** Currently, `ModelSchema` is compiled on first use — the first request to each endpoint pays the compilation cost (~1ms). Pre-warming at startup eliminates this cold-start latency.

**How:** In `django_clarax/apps.py`, during `ready()`, scan all imported `RustSerializerMixin` subclasses and call `_init_clarax_schema()` on each. This uses Django's app registry which is fully loaded at `ready()` time.

**Effort:** Low
**Impact:** Low

---

## 8. Compile-time schema validation

**What:** Validate that a `Schema` definition is internally consistent at compilation time, not at first use.

**Why:** Currently, if a developer passes `Field(int, max_length=100)` (max_length on an int), it silently ignores the constraint. A strict mode could catch these mistakes early.

**How:** In `Field.__init__()`, validate that constraints match the declared type. `max_length` only valid for `str` and `bytes`. `min_value`/`max_value` only valid for `int` and `float`. `max_digits`/`decimal_places` only valid for `Decimal`. Raise `SchemaError` immediately.

**Effort:** Low
**Impact:** Medium

---

## 9. Streaming serialization for large exports

**What:** `serialize_stream(queryset, schema)` that yields serialized chunks instead of building the entire list in memory.

**Why:** For CSV/JSON export endpoints with 100K+ records, building the full list in memory before responding causes memory spikes. A streaming approach uses constant memory.

**How:** Use Django's `StreamingHttpResponse` with a generator that calls `serialize_instance()` per chunk of 1,000 records. Each chunk is serialized in Rust, yielded as JSON bytes, then discarded. Memory stays flat regardless of total record count.

**Effort:** Medium
**Impact:** Medium

---

## 10. Benchmark regression CI

**What:** GitHub Actions job that runs benchmarks on every PR and comments with a comparison table (before/after).

**Why:** Performance regressions can creep in silently. The Phase 3 refactor introduced a small regression (2.6x → 2.3x) that was caught manually. An automated check would catch these before merge.

**How:** Use `criterion`'s `--save-baseline` and `--baseline` flags. On PR, run benchmarks against `main` baseline. If any benchmark regresses by >10%, post a comment with the comparison. Use `github-action-benchmark` or `codspeed`.

**Effort:** Medium
**Impact:** Medium

---

## Priority Ranking

| # | Proposal | Effort | Impact |
|---|----------|--------|--------|
| 1 | Queryset-to-Rust batch path | Medium | Game-changing |
| 2 | `clarax doctor` command | Low | High |
| 3 | Auto-schema from dataclass | Low | High |
| 5 | FastAPI response model | Medium | High |
| 6 | N+1 query detector | Medium | High |
| 4 | Request-level metrics | Low | Medium |
| 8 | Compile-time schema validation | Low | Medium |
| 9 | Streaming serialization | Medium | Medium |
| 10 | Benchmark regression CI | Medium | Medium |
| 7 | Schema pre-warming | Low | Low |
