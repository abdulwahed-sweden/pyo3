#!/bin/bash
# Author: Abdulwahed Mansour
# Publishes all ClaraX crates to crates.io in dependency order.
# Usage: ./publish-crates.sh
set -e

REPO="$(cd "$(dirname "$0")" && pwd)"

# Load token from .env
if [ ! -f "$REPO/.env" ]; then
    echo "Error: .env file not found"
    echo "Create .env with CARGO_REGISTRY_TOKEN=your_crates_io_token_here"
    exit 1
fi
source "$REPO/.env"

if [ -z "$CARGO_REGISTRY_TOKEN" ] || [ "$CARGO_REGISTRY_TOKEN" = "your_crates_io_token_here" ]; then
    echo "Error: CARGO_REGISTRY_TOKEN not set in .env (or still has placeholder value)"
    exit 1
fi

export CARGO_REGISTRY_TOKEN

CRATES=(
    "clarax-build-config"
    "clarax-ffi"
    "clarax-macros-backend"
    "clarax-macros"
    "clarax"
    "clarax-core"
    "clarax-django"
)

echo "ClaraX — crates.io publish"
echo ""

# Verify workspace builds
cargo check --workspace
echo "Build OK."
echo ""

START_FROM="${1:-}"
skip=false
if [ -n "$START_FROM" ]; then
    skip=true
fi

for crate in "${CRATES[@]}"; do
    if $skip; then
        if [ "$crate" = "$START_FROM" ]; then
            skip=false
        else
            echo "Skipping $crate"
            continue
        fi
    fi

    echo "=== Publishing: $crate ==="

    if cargo publish -p "$crate" --no-verify 2>&1; then
        echo "$crate published."
    else
        echo ""
        echo "Failed on $crate. To resume: ./publish-crates.sh $crate"
        exit 1
    fi

    echo "Waiting 20s for crates.io index..."
    sleep 20
done

echo ""
echo "All crates published:"
for crate in "${CRATES[@]}"; do
    echo "  https://crates.io/crates/$crate"
done
