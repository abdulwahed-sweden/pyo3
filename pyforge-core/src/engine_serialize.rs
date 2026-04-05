// Author: Abdulwahed Mansour
//! Serialization engine — converts FieldValue collections into JSON-compatible output.
//!
//! Handles type-specific conversions:
//! - Decimal → string (preserves precision, matches Python convention)
//! - DateTime → ISO 8601 / RFC 3339
//! - UUID → hyphenated string
//! - Binary → base64 string
//! - Null → JSON null

use serde_json::{Map, Value as JsonValue};

use crate::error::CoreError;
use crate::types::{FieldDescriptor, FieldValue};

/// A single serialized record — a map of field names to JSON values.
pub type SerializedRecord = Map<String, JsonValue>;

/// Serializes a set of field values into a JSON-compatible record.
///
/// Descriptors and values must be aligned by index. Returns a JSON object
/// (map of field names to values).
pub fn serialize_fields(
    descriptors: &[FieldDescriptor],
    values: &[FieldValue],
) -> Result<SerializedRecord, CoreError> {
    if descriptors.len() != values.len() {
        return Err(CoreError::Serialization {
            field: "<batch>".into(),
            message: format!(
                "descriptor count ({}) does not match value count ({})",
                descriptors.len(),
                values.len()
            ),
        });
    }

    let mut record = Map::with_capacity(descriptors.len());

    for (desc, val) in descriptors.iter().zip(values.iter()) {
        let json_val = field_value_to_json(val, &desc.name)?;
        record.insert(desc.name.clone(), json_val);
    }

    Ok(record)
}

/// Serializes multiple rows into a vector of records.
///
/// Each row is a slice of FieldValue aligned with the shared descriptors.
pub fn serialize_rows(
    descriptors: &[FieldDescriptor],
    rows: &[Vec<FieldValue>],
) -> Result<Vec<SerializedRecord>, CoreError> {
    rows.iter()
        .enumerate()
        .map(|(idx, values)| {
            serialize_fields(descriptors, values).map_err(|e| match e {
                CoreError::Serialization { field, message } => CoreError::Serialization {
                    field: format!("row[{idx}].{field}"),
                    message,
                },
                other => other,
            })
        })
        .collect()
}

/// Converts a single FieldValue into a serde_json::Value.
fn field_value_to_json(value: &FieldValue, field_name: &str) -> Result<JsonValue, CoreError> {
    match value {
        FieldValue::Text(s) => Ok(JsonValue::String(s.clone())),
        FieldValue::Integer(n) => Ok(JsonValue::Number((*n).into())),
        FieldValue::Float(f) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .ok_or_else(|| CoreError::Serialization {
                field: field_name.into(),
                message: format!("float value {f} is not representable in JSON (NaN/Infinity)"),
            }),
        FieldValue::Decimal(d) => Ok(JsonValue::String(d.to_string())),
        FieldValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
        FieldValue::Date(d) => Ok(JsonValue::String(d.format("%Y-%m-%d").to_string())),
        FieldValue::Time(t) => Ok(JsonValue::String(t.format("%H:%M:%S%.f").to_string())),
        FieldValue::DateTime(dt) => Ok(JsonValue::String(dt.to_rfc3339())),
        FieldValue::Uuid(u) => Ok(JsonValue::String(u.to_string())),
        FieldValue::Json(v) => Ok(v.clone()),
        FieldValue::Binary(bytes) => Ok(JsonValue::String(base64_encode(bytes))),
        FieldValue::Null => Ok(JsonValue::Null),
    }
}

/// Minimal base64 encoder — avoids pulling in a full base64 crate.
fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        output.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        output.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            output.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            output.push('=');
        }
        if chunk.len() > 2 {
            output.push(ALPHABET[(triple & 0x3F) as usize] as char);
        } else {
            output.push('=');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FieldType;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn desc(name: &str, ft: FieldType) -> FieldDescriptor {
        FieldDescriptor {
            name: name.into(),
            field_type: ft,
            nullable: false,
            has_default: false,
        }
    }

    #[test]
    fn serialize_str_field() {
        let descs = vec![desc("name", FieldType::Str { max_length: None, min_length: None })];
        let vals = vec![FieldValue::Text("Alice".into())];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["name"], JsonValue::String("Alice".into()));
    }

    #[test]
    fn serialize_int_field() {
        let descs = vec![desc("age", FieldType::Int { min_value: None, max_value: None })];
        let vals = vec![FieldValue::Integer(30)];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["age"], JsonValue::Number(30.into()));
    }

    #[test]
    fn serialize_decimal_as_string() {
        let descs = vec![desc("price", FieldType::Decimal { max_digits: Some(10), decimal_places: Some(2) })];
        let vals = vec![FieldValue::Decimal(Decimal::new(19999, 2))];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["price"], JsonValue::String("199.99".into()));
    }

    #[test]
    fn serialize_null_becomes_json_null() {
        let descs = vec![desc("bio", FieldType::Str { max_length: None, min_length: None })];
        let vals = vec![FieldValue::Null];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["bio"], JsonValue::Null);
    }

    #[test]
    fn serialize_datetime_uses_rfc3339() {
        let descs = vec![desc("created", FieldType::DateTime)];
        let dt = NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(12, 30, 0)
            .unwrap()
            .and_utc();
        let vals = vec![FieldValue::DateTime(dt)];
        let record = serialize_fields(&descs, &vals).unwrap();
        let s = record["created"].as_str().unwrap();
        assert!(s.contains("2025-01-15"));
    }

    #[test]
    fn serialize_uuid_as_hyphenated() {
        let descs = vec![desc("id", FieldType::Uuid)];
        let id = Uuid::new_v4();
        let vals = vec![FieldValue::Uuid(id)];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["id"], JsonValue::String(id.to_string()));
    }

    #[test]
    fn serialize_bool_field() {
        let descs = vec![desc("active", FieldType::Bool)];
        let vals = vec![FieldValue::Boolean(true)];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["active"], JsonValue::Bool(true));
    }

    #[test]
    fn serialize_binary_as_base64() {
        let descs = vec![desc("data", FieldType::Bytes { max_length: None })];
        let vals = vec![FieldValue::Binary(b"hello".to_vec())];
        let record = serialize_fields(&descs, &vals).unwrap();
        assert_eq!(record["data"], JsonValue::String("aGVsbG8=".into()));
    }

    #[test]
    fn serialize_multiple_rows() {
        let descs = vec![
            desc("name", FieldType::Str { max_length: None, min_length: None }),
            desc("active", FieldType::Bool),
        ];
        let rows = vec![
            vec![FieldValue::Text("Alice".into()), FieldValue::Boolean(true)],
            vec![FieldValue::Text("Bob".into()), FieldValue::Boolean(false)],
        ];
        let results = serialize_rows(&descs, &rows).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], JsonValue::String("Alice".into()));
        assert_eq!(results[1]["active"], JsonValue::Bool(false));
    }

    #[test]
    fn mismatched_lengths_returns_error() {
        let descs = vec![desc("name", FieldType::Str { max_length: None, min_length: None })];
        let vals: Vec<FieldValue> = vec![];
        assert!(serialize_fields(&descs, &vals).is_err());
    }

    #[test]
    fn nan_float_returns_error() {
        let descs = vec![desc("score", FieldType::Float { min_value: None, max_value: None })];
        let vals = vec![FieldValue::Float(f64::NAN)];
        assert!(serialize_fields(&descs, &vals).is_err());
    }
}
