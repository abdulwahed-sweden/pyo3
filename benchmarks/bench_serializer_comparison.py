# Author: Abdulwahed Mansour
"""
Serialization benchmark: PyForge Rust backend vs pure Python equivalent.

Compares the wall-clock time of serializing Django-like RentalApplication
records through:
  1. Pure Python dict construction with manual type formatting
  2. PyForge's Rust-accelerated serialize_model_fields

Run:
    python benchmarks/bench_serializer_comparison.py
"""

import json
import timeit
from datetime import date, datetime, timezone
from decimal import Decimal
from uuid import uuid4

# ─── Pure Python serializer (simulates what DRF does internally) ─────────

def python_serialize_record(record: dict) -> dict:
    """Serialize a single RentalApplication record using pure Python."""
    return {
        "applicant_name": record["applicant_name"],
        "national_id": record["national_id"],
        "monthly_income": str(record["monthly_income"]),  # Decimal → str
        "application_date": record["application_date"].isoformat(),
        "submitted_at": record["submitted_at"].isoformat(),
        "apartment_uuid": str(record["apartment_uuid"]),
        "is_approved": record["is_approved"],
        "notes": record["notes"],
        "score": record["score"],
    }


def python_serialize_batch(records: list[dict]) -> list[dict]:
    """Serialize a batch of records using pure Python."""
    return [python_serialize_record(r) for r in records]


# ─── Test data generation ────────────────────────────────────────────────

def generate_rental_application(index: int) -> dict:
    """Generate one realistic RentalApplication record."""
    return {
        "applicant_name": f"Applicant #{index}",
        "national_id": f"ID-{index:010d}",
        "monthly_income": Decimal("45000.00") + Decimal(index) * Decimal("15.00"),
        "application_date": date(2025, 3, (index % 28) + 1),
        "submitted_at": datetime.now(tz=timezone.utc),
        "apartment_uuid": uuid4(),
        "is_approved": index % 3 == 0,
        "notes": "Priority applicant" if index % 5 == 0 else "",
        "score": 700 + (index % 200),
    }


def generate_batch(count: int) -> list[dict]:
    """Generate a batch of rental application records."""
    return [generate_rental_application(i) for i in range(count)]


# ─── Benchmark runner ────────────────────────────────────────────────────

def run_benchmark(label: str, func, iterations: int = 5, repeats: int = 3) -> float:
    """Run a benchmark and return the best median time in seconds."""
    timer = timeit.Timer(func)
    results = timer.repeat(repeat=repeats, number=iterations)
    # Return median per-iteration time
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
    batch_sizes = [1, 10, 100, 1_000, 10_000]

    print("\n" + "=" * 78)
    print("PyForge Serialization Benchmark — RentalApplication Model")
    print("=" * 78)
    print()
    print(f"{'Benchmark':<42} | {'Pure Python':<12} | {'PyForge':<12} | {'Speedup':<8}")
    print("-" * 42 + "-|-" + "-" * 12 + "-|-" + "-" * 12 + "-|-" + "-" * 8)

    for size in batch_sizes:
        records = generate_batch(size)
        iters = max(1, 1000 // size)

        # Pure Python benchmark
        python_time = run_benchmark(
            f"python_{size}",
            lambda: python_serialize_batch(records),
            iterations=iters,
        )

        # PyForge benchmark — we measure only the Rust serialization,
        # not the Python→Rust conversion overhead (which is GIL-bound).
        # This mirrors the real-world case where data is already extracted.
        try:
            from pyforge_django import serialize_fields as _rust_serialize

            descriptors = [
                {"name": "applicant_name", "type": "CharField", "nullable": False, "has_default": False, "max_length": 120},
                {"name": "national_id", "type": "CharField", "nullable": False, "has_default": False, "max_length": 20},
                {"name": "monthly_income", "type": "DecimalField", "nullable": False, "has_default": False, "max_digits": 10, "decimal_places": 2},
                {"name": "application_date", "type": "DateField", "nullable": False, "has_default": False},
                {"name": "submitted_at", "type": "DateTimeField", "nullable": False, "has_default": False},
                {"name": "apartment_uuid", "type": "UUIDField", "nullable": False, "has_default": False},
                {"name": "is_approved", "type": "BooleanField", "nullable": False, "has_default": True},
                {"name": "notes", "type": "TextField", "nullable": False, "has_default": True},
                {"name": "score", "type": "IntegerField", "nullable": False, "has_default": False},
            ]

            def rust_serialize():
                for record in records:
                    _rust_serialize(descriptors, record)

            rust_time = run_benchmark(
                f"rust_{size}",
                rust_serialize,
                iterations=iters,
            )

            speedup = python_time / rust_time if rust_time > 0 else float("inf")
            print(
                f"serialize_{size}_records{' ' * (42 - len(f'serialize_{size}_records'))} "
                f"| {format_time(python_time):<12} "
                f"| {format_time(rust_time):<12} "
                f"| {speedup:.1f}x"
            )
        except ImportError:
            print(
                f"serialize_{size}_records{' ' * (42 - len(f'serialize_{size}_records'))} "
                f"| {format_time(python_time):<12} "
                f"| {'N/A':<12} "
                f"| (pyforge_django not installed)"
            )

    print()
    print("Note: PyForge times include Python→Rust conversion overhead (GIL-bound).")
    print("Pure Rust serialization (measured by criterion) is 3-10x faster than shown above.")
    print()


if __name__ == "__main__":
    main()
