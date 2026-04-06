# Your Django API is slow. Here's why — and a fix that takes 2 minutes.

I built ClaraX because I had a problem I couldn't solve with Python alone.

## The problem

A rental queue system in Stockholm. Thousands of applicants ranked in real time. The API worked fine with 50 users. At 2,000 users, serialization alone consumed 80% of CPU. The endpoint went from 80ms to 3 seconds.

We tried everything. `select_related`. Caching. Celery. Even looked at rewriting in Go. Nothing worked because the bottleneck wasn't the database. It was Django REST Framework itself.

DRF processes every field in Python. One by one. For every record. A 17-field serializer on 1,000 records means 17,000 Python function calls just to turn data into JSON.

## The fix

ClaraX moves that work to Rust. You don't learn Rust. You don't install Rust. You add one line to your existing serializer:

```python
from django_clarax.serializers import RustSerializerMixin

class ApplicationSerializer(RustSerializerMixin, serializers.ModelSerializer):
    class Meta:
        model = Application
        fields = "__all__"
```

Same views. Same tests. Same team. The heavy lifting happens in Rust underneath.

## Real numbers

| What changed | Before | After |
|---|---|---|
| Serialize 1,000 records | 475 ms | 14 ms (33x faster) |
| Validate 1,000 records | 506 ms | 10 ms (50x faster) |
| Name validation at scale | 2.8 sec | 339 ms (8x faster) |

These are measured numbers, not projections. On a real Django project (Hyra, a rental queue system), the improvement was 2.2-2.7x on production-like data.

Honest caveat: simple dict processing shows only 2.2x improvement. ClaraX replaces DRF overhead, not Python itself.

## Who this is for

- Django projects with 1,000+ users where API response time matters
- Bulk operations: exports, imports, reports that process thousands of records
- Teams that know Django and don't want to rewrite in another framework
- Companies where scaling servers is more expensive than optimizing code

## Who this is NOT for

- Small projects under 1,000 users. DRF is fast enough.
- Database-bound APIs. Fix your queries first.
- Teams already migrating away from Django.

## How to check if it helps YOUR project

```bash
pip install clarax-django
python manage.py clarax_doctor
```

`clarax_doctor` scans every serializer in your project and tells you which ones benefit. No code changes needed for the audit.

## The technical details (for CTOs)

- Built on PyO3 0.28.3 (Rust-Python bridge)
- Pre-built wheels for macOS, Linux, Windows. No compilation.
- Python 3.11+ supported. Python 3.14 free-threading ready.
- MIT licensed. Open source.
- Published on PyPI and crates.io.

ClaraX was born from a real production problem. It's now open source because every Django team hitting this wall deserves a solution that doesn't require rewriting their entire stack.

GitHub: github.com/abdulwahed-sweden/clarax

---

*Abdulwahed Mansour — Stockholm, Sweden*

#Django #Python #Rust #Performance #OpenSource #DRF #BackendEngineering
