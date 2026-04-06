# clarax-django

**Rust-accelerated serialization for Django REST Framework.**

One mixin. Same serializer. 33x faster.

```bash
pip install clarax-django
```

```python
# settings.py
INSTALLED_APPS = [..., "django_clarax"]
```

```python
# your serializer
from django_clarax.serializers import RustSerializerMixin

class MySerializer(RustSerializerMixin, serializers.ModelSerializer):
    class Meta:
        model = MyModel
        fields = "__all__"
```

Not sure which serializers benefit? Run the audit:

```bash
python manage.py clarax_doctor
```

Serialize 1,000 records: 475ms with DRF, 14ms with ClaraX. Validate 1,000 records: 506ms with DRF, 10ms with ClaraX.

Requires Python 3.11+ and Django 4.2+. Supports Python 3.14t (free-threading).

Read the [full story](https://github.com/abdulwahed-sweden/clarax#readme) for benchmarks, architecture, and the decision guide.

MIT -- [Abdulwahed Mansour](https://github.com/abdulwahed-sweden)
