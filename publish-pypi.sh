#!/bin/bash
# Author: Abdulwahed Mansour
# Builds and publishes pyforge-django to PyPI.
set -e

BUILD_ONLY=""
if [ "$1" = "--build" ]; then BUILD_ONLY=1; fi

REPO="$(cd "$(dirname "$0")" && pwd)"
WHEELS="$REPO/target/wheels"
mkdir -p "$WHEELS"

echo "PyForge Django v0.1.0 — Building wheel"
echo ""

# Step 1: Compile the Rust extension
echo "Step 1: Compiling native extension..."
cargo build -p pyforge-django --release
DYLIB=$(find "$REPO/target/release" -maxdepth 1 \( -name 'libpyforge_django.dylib' -o -name 'libpyforge_django.so' \) | head -1)
if [ -z "$DYLIB" ]; then echo "ERROR: compiled lib not found"; exit 1; fi
echo "Built: $DYLIB ($(du -h "$DYLIB" | cut -f1))"

# Step 2: Assemble the wheel
echo ""
echo "Step 2: Assembling wheel..."

# Detect platform
PYTAG="cp312"
MACHINE=$(python3 -c "import platform; print(platform.machine().lower())")
SYSTEM=$(python3 -c "import platform; print(platform.system().lower())")
if [ "$SYSTEM" = "darwin" ]; then
    PLAT="macosx_11_0_${MACHINE}"
elif [ "$SYSTEM" = "linux" ]; then
    PLAT="manylinux_2_17_${MACHINE}"
else
    PLAT="win_${MACHINE}"
fi

WHEEL_NAME="pyforge_django-0.1.0-${PYTAG}-${PYTAG}-${PLAT}.whl"
WHEEL_PATH="$WHEELS/$WHEEL_NAME"

STAGING=$(mktemp -d)
PKG="$STAGING/django_pyforge"
DIST="$STAGING/pyforge_django-0.1.0.dist-info"
mkdir -p "$PKG" "$DIST"

# Copy Python sources
cp "$REPO/pyforge-django/django_pyforge/__init__.py" "$PKG/"
cp "$REPO/pyforge-django/django_pyforge/__init__.pyi" "$PKG/" 2>/dev/null || true
cp "$REPO/pyforge-django/django_pyforge/apps.py" "$PKG/"
cp "$REPO/pyforge-django/django_pyforge/serializers.py" "$PKG/"
cp "$REPO/pyforge-django/django_pyforge/serializers.pyi" "$PKG/" 2>/dev/null || true
cp "$REPO/pyforge-django/django_pyforge/validators.py" "$PKG/"
cp "$REPO/pyforge-django/django_pyforge/validators.pyi" "$PKG/" 2>/dev/null || true
cp "$REPO/pyforge-django/django_pyforge/py.typed" "$PKG/" 2>/dev/null || true

# Copy the native extension as pyforge_django.so (at the root, not inside django_pyforge)
# This is what `from pyforge_django import ModelSchema` resolves to
cp "$DYLIB" "$STAGING/pyforge_django.so"

# Ensure __init__.py imports from pyforge_django (the .so at root level)
cat > "$PKG/__init__.py" << 'PYEOF'
# Author: Abdulwahed Mansour
"""django_pyforge — High-performance Django integration powered by Rust."""
__author__ = "Abdulwahed Mansour"
__version__ = "0.1.0"
try:
    from pyforge_django import (
        ModelSchema, extract_model_fields, serialize_batch, serialize_fields,
        serialize_instance, validate_fields, validate_instance, version,
    )
except ImportError as exc:
    raise ImportError("pyforge_django native extension not found. pip install pyforge-django") from exc
__all__ = ["ModelSchema", "extract_model_fields", "serialize_instance", "serialize_batch",
           "serialize_fields", "validate_instance", "validate_fields"]
PYEOF

# Write METADATA
cat > "$DIST/METADATA" << EOF
Metadata-Version: 2.1
Name: pyforge-django
Version: 0.1.0
Summary: Rust-accelerated Django serialization, validation, and field mapping
Author: Abdulwahed Mansour
License: MIT
Requires-Python: >=3.11
Requires-Dist: django>=4.2
Classifier: Framework :: Django
Classifier: Programming Language :: Rust
Classifier: Programming Language :: Python :: 3.11
Classifier: Programming Language :: Python :: 3.12
Classifier: Programming Language :: Python :: 3.13
EOF

# Write WHEEL
cat > "$DIST/WHEEL" << EOF
Wheel-Version: 1.0
Generator: pyforge-publish
Root-Is-Purelib: false
Tag: ${PYTAG}-${PYTAG}-${PLAT}
EOF

# Write top_level.txt
echo "django_pyforge" > "$DIST/top_level.txt"
echo "pyforge_django" >> "$DIST/top_level.txt"

# Create the zip
rm -f "$WHEEL_PATH"
cd "$STAGING"
python3 -c "
import zipfile, os, hashlib, base64, csv, io
whl = '$WHEEL_PATH'
with zipfile.ZipFile(whl, 'w', zipfile.ZIP_DEFLATED) as zf:
    records = []
    for root, dirs, files in os.walk('.'):
        for f in files:
            path = os.path.join(root, f)
            arc = os.path.relpath(path, '.')
            data = open(path, 'rb').read()
            zf.writestr(arc, data)
            h = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b'=').decode()
            records.append((arc, f'sha256={h}', str(len(data))))
    # RECORD
    buf = io.StringIO()
    w = csv.writer(buf)
    for r in records: w.writerow(r)
    rec_path = 'pyforge_django-0.1.0.dist-info/RECORD'
    w.writerow((rec_path, '', ''))
    zf.writestr(rec_path, buf.getvalue())
"

rm -rf "$STAGING"
cd "$REPO"

echo ""
echo "Wheel built: $WHEEL_PATH"
python3 -m zipfile -l "$WHEEL_PATH" | grep -E "\.so|\.py$|METADATA"
echo ""
ls -lh "$WHEEL_PATH"

if [ -z "$BUILD_ONLY" ]; then
    echo ""
    echo "Step 3: Uploading to PyPI..."
    twine upload "$WHEEL_PATH"
    echo "Published: pip install pyforge-django"
fi
