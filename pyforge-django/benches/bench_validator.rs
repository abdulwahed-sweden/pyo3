// Author: Abdulwahed Mansour
//! Validation throughput benchmarks for pyforge-django.
//!
//! Measures field validation at batch sizes that cross the serial→parallel
//! threshold (64 entries), demonstrating Rayon's scaling behavior.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use chrono::{NaiveDate, Utc};
use pyforge_django::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};
use pyforge_django::validator::validate_field_batch;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Builds a valid (descriptor, value) pair for each RentalApplication field.
fn rental_application_entry_set() -> Vec<(FieldDescriptor, FieldValue)> {
    vec![
        (
            FieldDescriptor {
                name: "applicant_name".into(),
                field_type: DjangoFieldType::CharField { max_length: 120 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text("Abdulwahed Mansour".into()),
        ),
        (
            FieldDescriptor {
                name: "national_id".into(),
                field_type: DjangoFieldType::CharField { max_length: 20 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text("SE-19880515-001".into()),
        ),
        (
            FieldDescriptor {
                name: "monthly_income".into(),
                field_type: DjangoFieldType::DecimalField {
                    max_digits: 10,
                    decimal_places: 2,
                },
                nullable: false,
                has_default: false,
            },
            FieldValue::Decimal(Decimal::new(4500000, 2)),
        ),
        (
            FieldDescriptor {
                name: "application_date".into(),
                field_type: DjangoFieldType::DateField,
                nullable: false,
                has_default: false,
            },
            FieldValue::Date(NaiveDate::from_ymd_opt(2025, 3, 15).unwrap()),
        ),
        (
            FieldDescriptor {
                name: "submitted_at".into(),
                field_type: DjangoFieldType::DateTimeField,
                nullable: false,
                has_default: false,
            },
            FieldValue::DateTime(Utc::now()),
        ),
        (
            FieldDescriptor {
                name: "apartment_uuid".into(),
                field_type: DjangoFieldType::UuidField,
                nullable: false,
                has_default: false,
            },
            FieldValue::Uuid(Uuid::new_v4()),
        ),
        (
            FieldDescriptor {
                name: "is_approved".into(),
                field_type: DjangoFieldType::BooleanField,
                nullable: false,
                has_default: true,
            },
            FieldValue::Boolean(false),
        ),
        (
            FieldDescriptor {
                name: "notes".into(),
                field_type: DjangoFieldType::TextField,
                nullable: false,
                has_default: true,
            },
            FieldValue::Text(String::new()),
        ),
        (
            FieldDescriptor {
                name: "score".into(),
                field_type: DjangoFieldType::IntegerField,
                nullable: false,
                has_default: false,
            },
            FieldValue::Integer(750i64),
        ),
    ]
}

/// Generates N entries by repeating the rental application field set.
fn build_validation_batch(record_count: usize) -> Vec<(FieldDescriptor, FieldValue)> {
    let template = rental_application_entry_set();
    let fields_per_record = template.len();
    let mut batch = Vec::with_capacity(record_count * fields_per_record);
    for i in 0..record_count {
        for (desc, val) in &template {
            let adjusted_val = match val {
                FieldValue::Text(s) => FieldValue::Text(format!("{s}_{i}")),
                FieldValue::Integer(n) => FieldValue::Integer(*n + i as i64),
                other => other.clone(),
            };
            batch.push((desc.clone(), adjusted_val));
        }
    }
    batch
}

/// Measures validation of a single 9-field record (serial path).
fn bench_validate_single_record(c: &mut Criterion) {
    let entries = rental_application_entry_set();

    c.bench_function("validate_single_rental_application", |b| {
        b.iter(|| validate_field_batch(black_box(&entries)))
    });
}

/// Measures validation at different batch sizes, crossing the serial→parallel threshold.
fn bench_validate_batch_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate_batch");

    for record_count in [1, 10, 100, 1_000] {
        let batch = build_validation_batch(record_count);
        group.bench_with_input(
            BenchmarkId::new("rental_records", record_count),
            &batch,
            |b, batch| b.iter(|| validate_field_batch(black_box(batch))),
        );
    }
    group.finish();
}

/// Measures validation with mixed valid and invalid inputs (50% failure rate).
fn bench_validate_mixed_valid_invalid(c: &mut Criterion) {
    let mut entries = Vec::with_capacity(200);
    for i in 0..100 {
        // Valid entry
        entries.push((
            FieldDescriptor {
                name: format!("name_{i}"),
                field_type: DjangoFieldType::CharField { max_length: 120 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text(format!("Valid Name {i}")),
        ));
        // Invalid entry — exceeds max_length
        entries.push((
            FieldDescriptor {
                name: format!("bad_name_{i}"),
                field_type: DjangoFieldType::CharField { max_length: 5 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text(format!("This string is definitely longer than five characters {i}")),
        ));
    }

    c.bench_function("validate_200_entries_50pct_invalid", |b| {
        b.iter(|| validate_field_batch(black_box(&entries)))
    });
}

/// Measures validation throughput when all fields are null on non-nullable descriptors.
fn bench_validate_all_null_required(c: &mut Criterion) {
    let entries: Vec<_> = (0..100)
        .map(|i| {
            (
                FieldDescriptor {
                    name: format!("required_field_{i}"),
                    field_type: DjangoFieldType::CharField { max_length: 255 },
                    nullable: false,
                    has_default: false,
                },
                FieldValue::Null,
            )
        })
        .collect();

    c.bench_function("validate_100_null_on_required_fields", |b| {
        b.iter(|| validate_field_batch(black_box(&entries)))
    });
}

criterion_group!(
    benches,
    bench_validate_single_record,
    bench_validate_batch_scaling,
    bench_validate_mixed_valid_invalid,
    bench_validate_all_null_required,
);
criterion_main!(benches);
