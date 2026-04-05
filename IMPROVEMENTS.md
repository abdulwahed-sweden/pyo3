# ClaraX Improvement Roadmap

Author: Abdulwahed Mansour

Technical notes on future improvements with clear approach and blockers.
Only items with provable value are listed — no speculative features.

---

## 1. Compiled Validation Rule Set (High Impact)

**Current state:** Every call to `validate_field_batch` re-reads field constraints from `FieldDescriptor` structs dynamically.

**Proposed:** At Django startup (in `AppConfig.ready()`), compile a `CompiledValidator` struct that stores function pointers for each field's validation logic. On the hot path, iterate function pointers directly — no match/dispatch overhead.

**Approach:**
```rust
pub struct CompiledValidator {
    rules: Vec<Box<dyn Fn(&FieldValue) -> Vec<FieldValidationError> + Send + Sync>>,
}
```
Build once from `Vec<FieldDescriptor>`, cache on the Python side, pass a handle to Rust on each request.

**Expected speedup:** 15-30% on validation by eliminating per-call match dispatch.

**Blocker:** None — this is pure Rust work. Can be shipped as a `CompiledValidator` class exposed to Python.

---

## 2. Zero-Copy TextField Serialization (Medium Impact)

**Current state:** `FieldValue::Text(s)` → `JsonValue::String(s.clone())` clones every string.

**Proposed:** For the serializer, use a `Cow<'a, str>` or write directly to a `serde_json::Serializer` backed by a `Vec<u8>` buffer, avoiding the intermediate `serde_json::Value` tree entirely.

**Approach:**
```rust
pub fn serialize_to_bytes(descriptors: &[FieldDescriptor], values: &[FieldValue]) -> Result<Vec<u8>, DjangoError> {
    let mut buf = Vec::with_capacity(estimate_json_size(descriptors, values));
    let mut ser = serde_json::Serializer::new(&mut buf);
    // Write fields directly to serializer
}
```

**Expected speedup:** 20-40% for large TextFields (10KB+) by eliminating the clone.

**Blocker:** Requires changing the return type from `SerializedRecord` (Map) to `Vec<u8>`, which means the Python layer needs to parse the bytes or we return a Python `bytes` object. Trade-off between API simplicity and performance.

---

## 3. Django QuerySet Fast Path (High Impact, Requires Research)

**Current state:** Python code extracts field values from model instances one by one via `getattr()`, then passes them to Rust.

**Proposed:** Intercept at the `QuerySet.values_list()` level where Django already has a flat tuple of values. Pass the tuple directly to Rust without per-field `getattr()` calls.

**Approach:**
1. In Python: `qs.values_list()` returns tuples — pass them to a Rust `serialize_value_tuples()` function
2. In Rust: accept `Vec<Bound<'_, PyTuple>>` and extract by index (faster than by-name attr access)

**Expected speedup:** 2-3x on the Python→Rust bridge overhead, which dominates for small records.

**Blocker:** Requires Django ORM knowledge to intercept at the right level. The `values_list()` output order must match the descriptor order exactly.

---

## 4. Missing Django Field Types

**Currently missing:**

| Django Field | Use Frequency | Approach |
|-------------|--------------|----------|
| `ArrayField` (PostgreSQL) | High in PostgreSQL projects | Map to `Vec<FieldValue>` |
| `HStoreField` (PostgreSQL) | Medium | Map to `HashMap<String, String>` |
| `FileField` / `ImageField` | High | Map path only (String), not content |
| `IPAddressField` / `GenericIPAddressField` | Low-Medium | Map to `String` with format validation |
| `DurationField` | Low | Map to `chrono::Duration` |

**Blocker:** Each new type needs a Python extraction path, a Rust type, a serialization path, and validation rules. ArrayField is the highest priority because it's used in many PostgreSQL-backed Django projects.

---

## 5. Type Stub Generation (.pyi)

**Current state:** No IDE autocompletion for `clarax_django` functions.

**Proposed:** Generate `clarax_django.pyi` stub file during build, or commit a hand-written one.

**Approach:**
```python
# clarax_django.pyi
from typing import Any

def extract_model_fields(model_class: type) -> list[dict[str, Any]]: ...
def serialize_fields(field_descriptors: list[dict], values: dict) -> dict: ...
def validate_fields(descriptors: list[dict], values: list) -> dict: ...
def version() -> str: ...
```

**Blocker:** None — straightforward to ship.

---

## 6. Reduce ClaraX Boundary Crossings

**Current state:** `serialize_fields` makes N+1 Python↔Rust boundary crossings (1 per field + 1 for the result dict).

**Proposed:** Accept the entire model instance as a single `PyAny`, extract all fields in one Rust function call, serialize, and return a single dict.

**Approach:**
```rust
#[pyfunction]
fn serialize_instance(py: Python, instance: &Bound<PyAny>, descriptors: &Bound<PyList>) -> PyResult<Bound<PyDict>> {
    // Extract all values in one pass
    // Serialize in Rust
    // Return single dict
}
```

**Expected speedup:** 30-50% for small records (3-5 fields) where bridge overhead dominates.

**Blocker:** None — this is the natural next step after the current field-by-field API proves stable.
