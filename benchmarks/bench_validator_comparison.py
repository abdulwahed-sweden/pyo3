# Author: Abdulwahed Mansour
"""
Validation benchmark: ClaraX Rust backend vs pure Python field validation.

Compares wall-clock time of validating Django-like field values through:
  1. Pure Python validation (simulates Django's field.clean() path)
  2. ClaraX's Rust-accelerated validate_field_batch

Run:
    python benchmarks/bench_validator_comparison.py
"""

import re
import timeit
from decimal import Decimal

# ─── Pure Python validators (simulates Django's field validation) ─────────

def python_validate_charfield(value: str, max_length: int) -> list[str]:
    """Validate a CharField value — max_length check."""
    errors = []
    if value is None:
        errors.append("This field is required.")
    elif len(value) > max_length:
        errors.append(f"Ensure this value has at most {max_length} characters (it has {len(value)}).")
    return errors


def python_validate_decimalfield(value: Decimal, max_digits: int, decimal_places: int) -> list[str]:
    """Validate a DecimalField value — digit count and precision."""
    errors = []
    if value is None:
        errors.append("This field is required.")
        return errors
    sign, digits, exponent = value.as_tuple()
    total_digits = len(digits)
    if total_digits > max_digits:
        errors.append(f"Ensure that there are no more than {max_digits} digits in total.")
    scale = abs(exponent) if exponent < 0 else 0
    if scale > decimal_places:
        errors.append(f"Ensure that there are no more than {decimal_places} decimal places.")
    return errors


def python_validate_slugfield(value: str, max_length: int) -> list[str]:
    """Validate a SlugField — alphanumeric, hyphens, underscores."""
    errors = python_validate_charfield(value, max_length)
    if value and not re.match(r'^[a-zA-Z0-9_-]+$', value):
        errors.append("Enter a valid 'slug' consisting of letters, numbers, underscores or hyphens.")
    return errors


def python_validate_batch(entries: list[dict]) -> dict:
    """Validate a batch of field entries using pure Python."""
    field_errors = []
    for entry in entries:
        name = entry["name"]
        value = entry["value"]
        field_type = entry["type"]
        nullable = entry.get("nullable", False)

        if value is None:
            if not nullable:
                field_errors.append({"field": name, "message": "This field is required.", "code": "required"})
            continue

        if field_type == "CharField":
            errs = python_validate_charfield(value, entry.get("max_length", 255))
        elif field_type == "DecimalField":
            errs = python_validate_decimalfield(value, entry.get("max_digits", 10), entry.get("decimal_places", 2))
        elif field_type == "SlugField":
            errs = python_validate_slugfield(value, entry.get("max_length", 50))
        else:
            errs = []

        for err in errs:
            field_errors.append({"field": name, "message": err, "code": "invalid"})

    return {
        "valid_count": len(entries) - len(field_errors),
        "error_count": len(field_errors),
        "errors": field_errors,
    }


# ─── Test data generation ────────────────────────────────────────────────

def generate_validation_entries(count: int, error_rate: float = 0.0) -> list[dict]:
    """Generate realistic validation entries with a configurable error rate."""
    entries = []
    for i in range(count):
        inject_error = (i / count) < error_rate
        entries.append({
            "name": f"applicant_name_{i}",
            "type": "CharField",
            "nullable": False,
            "has_default": False,
            "max_length": 120,
            "value": "A" * 200 if inject_error else f"Applicant {i}",
        })
    return entries


# ─── Benchmark runner ────────────────────────────────────────────────────

def run_benchmark(func, iterations: int = 5, repeats: int = 3) -> float:
    """Run a benchmark and return the median per-iteration time in seconds."""
    timer = timeit.Timer(func)
    results = timer.repeat(repeat=repeats, number=iterations)
    per_iter = sorted([r / iterations for r in results])
    return per_iter[len(per_iter) // 2]


def format_time(seconds: float) -> str:
    """Format a time value with appropriate units."""
    if seconds < 1e-6:
        return f"{seconds * 1e9:.1f}ns"
    elif seconds < 1e-3:
        return f"{seconds * 1e6:.1f}\u00b5s"
    elif seconds < 1:
        return f"{seconds * 1e3:.1f}ms"
    else:
        return f"{seconds:.2f}s"


def main():
    print("\n" + "=" * 78)
    print("ClaraX Validation Benchmark — CharField / DecimalField / SlugField")
    print("=" * 78)
    print()
    print(f"{'Benchmark':<42} | {'Pure Python':<12} | {'ClaraX':<12} | {'Speedup':<8}")
    print("-" * 42 + "-|-" + "-" * 12 + "-|-" + "-" * 12 + "-|-" + "-" * 8)

    scenarios = [
        ("validate_10_fields_all_valid", 10, 0.0),
        ("validate_100_fields_all_valid", 100, 0.0),
        ("validate_1000_fields_all_valid", 1_000, 0.0),
        ("validate_1000_fields_50pct_invalid", 1_000, 0.5),
        ("validate_10000_fields_parallel", 10_000, 0.0),
    ]

    for label, count, error_rate in scenarios:
        entries = generate_validation_entries(count, error_rate)
        iters = max(1, 500 // count)

        python_time = run_benchmark(
            lambda: python_validate_batch(entries),
            iterations=iters,
        )

        padded_label = f"{label:<42}"

        try:
            from clarax_django import validate_fields as _rust_validate

            descriptors = [{"name": e["name"], "type": e["type"], "nullable": e["nullable"],
                           "has_default": e.get("has_default", False), "max_length": e.get("max_length")}
                          for e in entries]
            values = [e["value"] for e in entries]

            rust_time = run_benchmark(
                lambda: _rust_validate(descriptors, values),
                iterations=iters,
            )

            speedup = python_time / rust_time if rust_time > 0 else float("inf")
            print(f"{padded_label} | {format_time(python_time):<12} | {format_time(rust_time):<12} | {speedup:.1f}x")
        except ImportError:
            print(f"{padded_label} | {format_time(python_time):<12} | {'N/A':<12} | (not installed)")

    print()
    print("Note: For batches >= 64 entries, ClaraX uses Rayon parallel validation.")
    print("Small batches may show less speedup due to Python→Rust bridge overhead.")
    print()


if __name__ == "__main__":
    main()
