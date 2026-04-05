# Upgrading ClaraX

## From v0.2.x to v0.3.0

**Breaking changes:** None. All existing APIs work identically.

**New features available immediately:**

- `serialize_values_list()` — fastest path for bulk serialization
- `clarax_doctor` — find which serializers benefit from ClaraX
- `from_dataclass()` — auto-generate Schema from dataclasses
- `ClaraXMetricsMiddleware` — per-request performance metrics
- `serialize_stream()` — constant-memory exports

**Migration checklist:**

```
[ ] pip install --upgrade clarax-django
[ ] python manage.py clarax_doctor
[ ] (Optional) Add ClaraXMetricsMiddleware to MIDDLEWARE
[ ] (Optional) Switch bulk endpoints to serialize_values_list()
[ ] (Optional) Add CLARAX_METRICS = True to settings for observability
```

**For clarax-core users:**

```
[ ] pip install --upgrade clarax-core
[ ] Try from_dataclass() instead of manual Schema({...})
[ ] Field(int, max_length=100) now raises SchemaError (was silently ignored)
```
