// Author: Abdulwahed Mansour
//! Serialization throughput benchmarks for pyforge-django.
//!
//! Measures end-to-end serialization of Django-like model data (RentalApplication)
//! at realistic batch sizes. All data shapes mirror production Django workloads.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use chrono::{NaiveDate, Utc};
use pyforge_django::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};
use pyforge_django::serializer::{serialize_model_fields, serialize_queryset_rows};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Builds the field descriptors for the RentalApplication model.
fn rental_application_descriptors() -> Vec<FieldDescriptor> {
    vec![
        FieldDescriptor {
            name: "applicant_name".into(),
            field_type: DjangoFieldType::CharField { max_length: 120 },
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "national_id".into(),
            field_type: DjangoFieldType::CharField { max_length: 20 },
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "monthly_income".into(),
            field_type: DjangoFieldType::DecimalField {
                max_digits: 10,
                decimal_places: 2,
            },
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "application_date".into(),
            field_type: DjangoFieldType::DateField,
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "submitted_at".into(),
            field_type: DjangoFieldType::DateTimeField,
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "apartment_uuid".into(),
            field_type: DjangoFieldType::UuidField,
            nullable: false,
            has_default: false,
        },
        FieldDescriptor {
            name: "is_approved".into(),
            field_type: DjangoFieldType::BooleanField,
            nullable: false,
            has_default: true,
        },
        FieldDescriptor {
            name: "notes".into(),
            field_type: DjangoFieldType::TextField,
            nullable: false,
            has_default: true,
        },
        FieldDescriptor {
            name: "score".into(),
            field_type: DjangoFieldType::IntegerField,
            nullable: false,
            has_default: false,
        },
    ]
}

/// Builds one row of realistic RentalApplication field values.
fn rental_application_row(index: usize) -> Vec<FieldValue> {
    vec![
        FieldValue::Text(format!("Applicant #{index}")),
        FieldValue::Text(format!("ID-{index:010}")),
        FieldValue::Decimal(Decimal::new(4500000 + (index as i64 * 1500), 2)),
        FieldValue::Date(NaiveDate::from_ymd_opt(2025, 3, (index % 28 + 1) as u32).unwrap()),
        FieldValue::DateTime(Utc::now()),
        FieldValue::Uuid(Uuid::new_v4()),
        FieldValue::Boolean(index % 3 == 0),
        FieldValue::Text(if index % 5 == 0 {
            "Priority applicant — fast-track review requested by property manager.".into()
        } else {
            String::new()
        }),
        FieldValue::Integer((700 + (index % 200)) as i32),
    ]
}

/// Builds N rows of rental application data for queryset benchmarks.
fn rental_application_queryset(count: usize) -> Vec<Vec<FieldValue>> {
    (0..count).map(rental_application_row).collect()
}

/// Measures single-record serialization throughput.
fn bench_serialize_single_record(c: &mut Criterion) {
    let descriptors = rental_application_descriptors();
    let values = rental_application_row(0);

    c.bench_function("serialize_single_rental_application", |b| {
        b.iter(|| serialize_model_fields(black_box(&descriptors), black_box(&values)))
    });
}

/// Measures queryset serialization at different batch sizes.
fn bench_serialize_queryset_batch(c: &mut Criterion) {
    let descriptors = rental_application_descriptors();
    let mut group = c.benchmark_group("serialize_queryset");

    for size in [10, 100, 1_000, 10_000] {
        let rows = rental_application_queryset(size);
        group.bench_with_input(
            BenchmarkId::new("rental_applications", size),
            &rows,
            |b, rows| {
                b.iter(|| serialize_queryset_rows(black_box(&descriptors), black_box(rows)))
            },
        );
    }
    group.finish();
}

/// Measures serialization with worst-case inputs: max-length strings and boundary decimals.
fn bench_serialize_worst_case(c: &mut Criterion) {
    let descriptors = rental_application_descriptors();
    let worst_case_row = vec![
        FieldValue::Text("A".repeat(120)),                         // max_length CharField
        FieldValue::Text("9".repeat(20)),                          // max_length national_id
        FieldValue::Decimal(Decimal::new(9_999_999_999, 2)),       // max digits DecimalField
        FieldValue::Date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        FieldValue::DateTime(Utc::now()),
        FieldValue::Uuid(Uuid::new_v4()),
        FieldValue::Boolean(true),
        FieldValue::Text("X".repeat(10_000)),                      // large TextField
        FieldValue::Integer(i32::MAX),
    ];

    c.bench_function("serialize_worst_case_rental_application", |b| {
        b.iter(|| serialize_model_fields(black_box(&descriptors), black_box(&worst_case_row)))
    });
}

/// Measures serialization with nullable fields set to Null.
fn bench_serialize_with_nulls(c: &mut Criterion) {
    let mut descriptors = rental_application_descriptors();
    // Make half the fields nullable
    for desc in descriptors.iter_mut().skip(3) {
        desc.nullable = true;
    }

    let values_with_nulls = vec![
        FieldValue::Text("Applicant".into()),
        FieldValue::Text("ID-001".into()),
        FieldValue::Decimal(Decimal::new(50000, 2)),
        FieldValue::Null,
        FieldValue::Null,
        FieldValue::Null,
        FieldValue::Null,
        FieldValue::Null,
        FieldValue::Null,
    ];

    c.bench_function("serialize_rental_application_with_nulls", |b| {
        b.iter(|| serialize_model_fields(black_box(&descriptors), black_box(&values_with_nulls)))
    });
}

criterion_group!(
    benches,
    bench_serialize_single_record,
    bench_serialize_queryset_batch,
    bench_serialize_worst_case,
    bench_serialize_with_nulls,
);
criterion_main!(benches);
