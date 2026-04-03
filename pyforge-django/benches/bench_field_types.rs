// Author: Abdulwahed Mansour
//! Per-field-type conversion cost benchmarks.
//!
//! Isolates the serialization cost of each Django field type to identify
//! which types benefit most from Rust acceleration and which are bound
//! by data copying (e.g., large TextFields).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chrono::{NaiveDate, NaiveTime, Utc};
use pyforge_django::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};
use pyforge_django::serializer::serialize_model_fields;
use rust_decimal::Decimal;
use uuid::Uuid;

fn make_single(name: &str, ft: DjangoFieldType, val: FieldValue) -> (Vec<FieldDescriptor>, Vec<FieldValue>) {
    (
        vec![FieldDescriptor {
            name: name.into(),
            field_type: ft,
            nullable: false,
            has_default: false,
        }],
        vec![val],
    )
}

/// Measures CharField serialization cost (short string, 15 chars).
fn bench_charfield(c: &mut Criterion) {
    let (d, v) = make_single(
        "name",
        DjangoFieldType::CharField { max_length: 120 },
        FieldValue::Text("Abdulwahed M.".into()),
    );
    c.bench_function("field_type/CharField_short", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures TextField serialization cost (1KB body).
fn bench_textfield(c: &mut Criterion) {
    let body = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(18);
    let (d, v) = make_single("notes", DjangoFieldType::TextField, FieldValue::Text(body));
    c.bench_function("field_type/TextField_1kb", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures IntegerField serialization cost.
fn bench_integerfield(c: &mut Criterion) {
    let (d, v) = make_single(
        "score",
        DjangoFieldType::IntegerField,
        FieldValue::Integer(750),
    );
    c.bench_function("field_type/IntegerField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures DecimalField serialization cost — Decimal→string path.
fn bench_decimalfield(c: &mut Criterion) {
    let (d, v) = make_single(
        "income",
        DjangoFieldType::DecimalField {
            max_digits: 10,
            decimal_places: 2,
        },
        FieldValue::Decimal(Decimal::new(4500000, 2)),
    );
    c.bench_function("field_type/DecimalField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures DateField serialization cost (NaiveDate→ISO string).
fn bench_datefield(c: &mut Criterion) {
    let (d, v) = make_single(
        "application_date",
        DjangoFieldType::DateField,
        FieldValue::Date(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()),
    );
    c.bench_function("field_type/DateField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures TimeField serialization cost.
fn bench_timefield(c: &mut Criterion) {
    let (d, v) = make_single(
        "check_in_time",
        DjangoFieldType::TimeField,
        FieldValue::Time(NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
    );
    c.bench_function("field_type/TimeField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures DateTimeField serialization cost (DateTime<Utc>→RFC3339).
fn bench_datetimefield(c: &mut Criterion) {
    let (d, v) = make_single(
        "submitted_at",
        DjangoFieldType::DateTimeField,
        FieldValue::DateTime(Utc::now()),
    );
    c.bench_function("field_type/DateTimeField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures UUIDField serialization cost.
fn bench_uuidfield(c: &mut Criterion) {
    let (d, v) = make_single(
        "apartment_uuid",
        DjangoFieldType::UuidField,
        FieldValue::Uuid(Uuid::new_v4()),
    );
    c.bench_function("field_type/UUIDField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures BooleanField serialization cost.
fn bench_booleanfield(c: &mut Criterion) {
    let (d, v) = make_single(
        "is_approved",
        DjangoFieldType::BooleanField,
        FieldValue::Boolean(true),
    );
    c.bench_function("field_type/BooleanField", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures JSONField serialization cost (nested object).
fn bench_jsonfield(c: &mut Criterion) {
    let json_val = serde_json::json!({
        "preferences": {
            "floor": 3,
            "balcony": true,
            "pet_friendly": false
        },
        "previous_addresses": ["Stockholm", "Malmo", "Gothenburg"]
    });
    let (d, v) = make_single(
        "metadata",
        DjangoFieldType::JsonField,
        FieldValue::Json(json_val),
    );
    c.bench_function("field_type/JSONField_nested", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

/// Measures Null value serialization cost (baseline — minimal work).
fn bench_null_value(c: &mut Criterion) {
    let (d, v) = make_single("optional_field", DjangoFieldType::TextField, FieldValue::Null);
    c.bench_function("field_type/Null", |b| {
        b.iter(|| serialize_model_fields(black_box(&d), black_box(&v)))
    });
}

criterion_group!(
    benches,
    bench_charfield,
    bench_textfield,
    bench_integerfield,
    bench_decimalfield,
    bench_datefield,
    bench_timefield,
    bench_datetimefield,
    bench_uuidfield,
    bench_booleanfield,
    bench_jsonfield,
    bench_null_value,
);
criterion_main!(benches);
