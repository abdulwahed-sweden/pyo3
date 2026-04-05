#!/bin/bash
# Author: Abdulwahed Mansour
# Builds and publishes ClaraX packages to PyPI.
# Usage: ./publish-pypi.sh [core|django|all]
set -e

REPO="$(cd "$(dirname "$0")" && pwd)"
WHEELS="$REPO/target/wheels"
mkdir -p "$WHEELS"

# Load tokens from .env
if [ ! -f "$REPO/.env" ]; then
    echo "Error: .env file not found"
    echo "Create .env with PYPI_TOKEN=your_token_here"
    echo "See .env.example for the required format."
    exit 1
fi
source "$REPO/.env"

if [ -z "$PYPI_TOKEN" ] || [ "$PYPI_TOKEN" = "your_token_here" ]; then
    echo "Error: PYPI_TOKEN not set in .env (or still has placeholder value)"
    exit 1
fi

PACKAGE="${1:-all}"

# ─── Build helpers ─────────────────────────────────────────────────────────────

detect_platform() {
    local PYTAG="cp312"
    local MACHINE
    MACHINE=$(python3 -c "import platform; print(platform.machine().lower())")
    local SYSTEM
    SYSTEM=$(python3 -c "import platform; print(platform.system().lower())")
    if [ "$SYSTEM" = "darwin" ]; then
        PLAT="macosx_11_0_${MACHINE}"
    elif [ "$SYSTEM" = "linux" ]; then
        PLAT="manylinux_2_17_${MACHINE}"
    else
        PLAT="win_${MACHINE}"
    fi
    echo "${PYTAG}-${PYTAG}-${PLAT}"
}

build_wheel() {
    local CRATE_NAME=$1      # e.g., clarax-core
    local PKG_NAME=$2        # e.g., clarax_core
    local PYPI_NAME=$3       # e.g., clarax-core
    local SRC_DIR=$4         # e.g., clarax-core/clarax_core
    local LIB_NAME=$5        # e.g., libclarax_core

    local VERSION
    VERSION=$(python3 -c "import tomllib; print(tomllib.load(open('$REPO/$CRATE_NAME/pyproject.toml','rb'))['project']['version'])")
    local TAG
    TAG=$(detect_platform)
    local WHEEL_NAME="${PKG_NAME}-${VERSION}-${TAG}.whl"
    local WHEEL_PATH="$WHEELS/$WHEEL_NAME"

    echo ""
    echo "=== Building ${PYPI_NAME} v${VERSION} ==="
    echo ""

    # Compile
    cargo build -p "$CRATE_NAME" --release
    local DYLIB
    DYLIB=$(find "$REPO/target/release" -maxdepth 1 \( -name "${LIB_NAME}.dylib" -o -name "${LIB_NAME}.so" -o -name "${PKG_NAME}.dll" \) | head -1)
    if [ -z "$DYLIB" ]; then echo "ERROR: compiled lib not found for $CRATE_NAME"; exit 1; fi
    echo "Built: $DYLIB ($(du -h "$DYLIB" | cut -f1))"

    # Assemble wheel
    local STAGING
    STAGING=$(mktemp -d)
    local PKG_DIR="$STAGING/$PKG_NAME"
    local DIST="$STAGING/${PKG_NAME}-${VERSION}.dist-info"
    mkdir -p "$PKG_DIR" "$DIST"

    # Copy Python sources (entire tree, excluding __pycache__)
    find "$REPO/$SRC_DIR" -type d -name __pycache__ -prune -o -type f -print | while read f; do
        local rel="${f#$REPO/$SRC_DIR/}"
        local dest="$PKG_DIR/$rel"
        mkdir -p "$(dirname "$dest")"
        cp "$f" "$dest"
    done

    # Copy native extension
    if [ "$CRATE_NAME" = "clarax-django" ]; then
        # Django: .so at root level (matches Rust lib name clarax_django)
        cp "$DYLIB" "$STAGING/${PKG_NAME}.so"
    else
        # Core: .so inside the package as _native.so
        cp "$DYLIB" "$PKG_DIR/_native.so"
    fi

    # README
    local README_CONTENT
    README_CONTENT=$(cat "$REPO/$CRATE_NAME/README.md")

    # METADATA
    local SUMMARY
    SUMMARY=$(python3 -c "import tomllib; print(tomllib.load(open('$REPO/$CRATE_NAME/pyproject.toml','rb'))['project']['description'])")

    cat > "$DIST/METADATA" << METAEOF
Metadata-Version: 2.1
Name: ${PYPI_NAME}
Version: ${VERSION}
Summary: ${SUMMARY}
Author: Abdulwahed Mansour
License: MIT
Requires-Python: >=3.11
Description-Content-Type: text/markdown
Project-URL: Homepage, https://github.com/abdulwahed-sweden/clarax
Project-URL: Repository, https://github.com/abdulwahed-sweden/clarax

${README_CONTENT}
METAEOF

    # Add django dependency for clarax-django
    if [ "$CRATE_NAME" = "clarax-django" ]; then
        sed -i '' '6a\
Requires-Dist: django>=4.2
' "$DIST/METADATA"
    fi

    cat > "$DIST/WHEEL" << WHEELEOF
Wheel-Version: 1.0
Generator: clarax-publish
Root-Is-Purelib: false
Tag: ${TAG}
WHEELEOF

    echo "$PKG_NAME" > "$DIST/top_level.txt"

    # Build zip
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
    buf = io.StringIO()
    w = csv.writer(buf)
    for r in records: w.writerow(r)
    rec_path = '${PKG_NAME}-${VERSION}.dist-info/RECORD'
    w.writerow((rec_path, '', ''))
    zf.writestr(rec_path, buf.getvalue())
"
    rm -rf "$STAGING"
    cd "$REPO"

    echo "Wheel: $WHEEL_PATH ($(du -h "$WHEEL_PATH" | cut -f1))"
}

publish_wheel() {
    local PKG_NAME=$1
    echo ""
    echo "Publishing ${PKG_NAME}..."
    TWINE_USERNAME=__token__ TWINE_PASSWORD="$PYPI_TOKEN" \
        "$REPO/.venv/bin/twine" upload "$WHEELS/${PKG_NAME}"-*.whl
    echo "${PKG_NAME} published."
}

# ─── Main ──────────────────────────────────────────────────────────────────────

case $PACKAGE in
    "core")
        build_wheel "clarax-core" "clarax_core" "clarax-core" "clarax-core/clarax_core" "libclarax_core"
        publish_wheel "clarax_core"
        ;;
    "django")
        build_wheel "clarax-django" "clarax_django" "clarax-django" "clarax-django/django_clarax" "libclarax_django"
        publish_wheel "clarax_django"
        ;;
    "all")
        build_wheel "clarax-core" "clarax_core" "clarax-core" "clarax-core/clarax_core" "libclarax_core"
        build_wheel "clarax-django" "clarax_django" "clarax-django" "clarax-django/django_clarax" "libclarax_django"
        publish_wheel "clarax_core"
        publish_wheel "clarax_django"
        ;;
    "--build")
        build_wheel "clarax-core" "clarax_core" "clarax-core" "clarax-core/clarax_core" "libclarax_core"
        build_wheel "clarax-django" "clarax_django" "clarax-django" "clarax-django/django_clarax" "libclarax_django"
        echo ""
        echo "Wheels built (not published). Run ./publish-pypi.sh all to publish."
        ;;
    *)
        echo "Usage: ./publish-pypi.sh [core|django|all|--build]"
        exit 1
        ;;
esac

echo ""
echo "https://pypi.org/project/clarax-core/"
echo "https://pypi.org/project/clarax-django/"
