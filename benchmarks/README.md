# PyForge Benchmarks

Performance comparison between PyForge's Rust backend and pure Python Django serialization/validation.

## Running Benchmarks

### Rust Micro-benchmarks (criterion)

```bash
# All benchmarks
cargo bench -p pyforge-django

# Individual benchmark suites
cargo bench -p pyforge-django --bench bench_serializer
cargo bench -p pyforge-django --bench bench_validator
cargo bench -p pyforge-django --bench bench_field_types
```

Results are saved to `target/criterion/` with HTML reports.

### Python Comparison Benchmarks

```bash
# Requires pyforge_django native extension installed
pip install -e pyforge-django/

# Serialization comparison
python benchmarks/bench_serializer_comparison.py

# Validation comparison
python benchmarks/bench_validator_comparison.py
```

## What Each Benchmark Measures

### `bench_serializer.rs`
- **Single record**: One 9-field RentalApplication → JSON
- **Queryset batch**: 10, 100, 1K, 10K records → JSON array
- **Worst case**: Max-length strings, boundary decimals, large TextField
- **Null handling**: Records with half the fields set to NULL

### `bench_validator.rs`
- **Single record**: 9-field validation (serial path)
- **Batch scaling**: 1, 10, 100, 1K records — crosses Rayon threshold at 64
- **Mixed valid/invalid**: 50% error rate (error construction overhead)
- **All-null required**: 100 required fields all NULL (fast-path rejection)

### `bench_field_types.rs`
- **Per-type cost**: Isolates serialization cost for each Django field type
- CharField, TextField, IntegerField, DecimalField, DateField, TimeField, DateTimeField, UUIDField, BooleanField, JSONField, Null

### `bench_serializer_comparison.py`
- Side-by-side: PyForge vs pure Python dict construction at different batch sizes
- Includes Python→Rust bridge overhead (realistic end-to-end measurement)

### `bench_validator_comparison.py`
- Side-by-side: PyForge vs pure Python field validation
- Tests different error rates and batch sizes

## Interpreting Results

- **Pure Rust numbers** (criterion): Represent the maximum achievable speedup when data is already in Rust. This is the ceiling.
- **Python comparison numbers**: Include the Python→Rust conversion overhead via PyForge's GIL bridge. This is what users actually experience.
- **Small batches** (< 10 records): May show modest speedup because bridge overhead dominates.
- **Large batches** (100+ records): Show significant speedup as Rust's computational advantage outweighs bridge overhead.
- **Rayon parallelism**: Kicks in at 64+ entries for validation, adding another speedup factor on multi-core systems.
