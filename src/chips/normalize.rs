use crate::types::NormalizeOutput;
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

/// Normalize a JSON value to canonical form
///
/// Algorithm:
/// 1. Parse input as JSON (if string, parse; otherwise treat as already parsed).
/// 2. Normalize Unicode – apply NFC (Normalization Form Canonical Composition) to all strings.
/// 3. Normalize numbers – reject any non‑integer numeric value (no floats, no exponential notation). Accept only i64 range.
/// 4. Normalize null/absent – drop object keys with value null.
/// 5. Sort object keys recursively (lexicographic, byte order).
/// 6. Serialize to canonical JSON (no whitespace).
/// 7. Output base64 of the canonical bytes and its blake3 CID.
pub fn normalize(value: Value) -> Result<NormalizeOutput> {
    // Normalize the value recursively
    let normalized = normalize_value(value)?;

    // Serialize to canonical JSON (no whitespace)
    let canonical_json = serde_json::to_string(&normalized)?;
    let canonical_bytes = canonical_json.as_bytes();

    // Compute blake3 hash
    let hash = blake3::hash(canonical_bytes);
    let cid = BASE64.encode(hash.as_bytes());

    // Encode bytes as base64
    let bytes_b64 = BASE64.encode(canonical_bytes);

    Ok(NormalizeOutput {
        bytes: bytes_b64,
        cid,
    })
}

/// Recursively normalize a JSON value
fn normalize_value(value: Value) -> Result<Value> {
    match value {
        Value::String(s) => {
            // Apply NFC normalization to strings
            let normalized: String = s.nfc().collect();
            Ok(Value::String(normalized))
        }
        Value::Number(n) => {
            // Only allow i64 integers
            if let Some(i) = n.as_i64() {
                Ok(Value::Number(serde_json::Number::from(i)))
            } else {
                Err(RhoError::Normalize(
                    "only i64 integers allowed, no floats or exponential notation".to_string(),
                ))
            }
        }
        Value::Object(map) => {
            // Remove null values and normalize recursively
            let mut normalized_map = serde_json::Map::new();

            for (k, v) in map {
                if !v.is_null() {
                    let normalized_key = k.nfc().collect::<String>();
                    let normalized_value = normalize_value(v)?;
                    normalized_map.insert(normalized_key, normalized_value);
                }
            }

            // Sort keys (serde_json::Map maintains insertion order, but we need sorted)
            let mut sorted: Vec<(String, Value)> = normalized_map.into_iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));

            let mut result_map = serde_json::Map::new();
            for (k, v) in sorted {
                result_map.insert(k, v);
            }

            Ok(Value::Object(result_map))
        }
        Value::Array(arr) => {
            // Normalize each element
            let normalized: Result<Vec<Value>> = arr.into_iter().map(normalize_value).collect();
            Ok(Value::Array(normalized?))
        }
        Value::Null => Ok(Value::Null),
        Value::Bool(b) => Ok(Value::Bool(b)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_normalize_simple_object() {
        let input = json!({"b": 2, "a": 1});
        let result = normalize(input).unwrap();

        // Check that the canonical form has sorted keys
        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, r#"{"a":1,"b":2}"#);
    }

    #[test]
    fn test_normalize_unicode_nfc() {
        // "café" can be represented in different ways
        let input = json!({"x": "café"});
        let result = normalize(input).unwrap();

        // Should be NFC normalized
        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert!(decoded_str.contains("café"));
    }

    #[test]
    fn test_normalize_rejects_float() {
        let input = json!(123.5);
        let result = normalize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("i64"));
    }

    #[test]
    fn test_normalize_integer() {
        let input = json!(123);
        let result = normalize(input).unwrap();

        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, "123");
    }

    #[test]
    fn test_normalize_removes_null() {
        let input = json!({"a": 1, "b": null, "c": 3});
        let result = normalize(input).unwrap();

        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, r#"{"a":1,"c":3}"#);
        assert!(!decoded_str.contains("\"b\""));
    }

    #[test]
    fn test_normalize_array_with_null() {
        let input = json!([null, 1, 2]);
        let result = normalize(input).unwrap();

        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, "[null,1,2]");
    }

    #[test]
    fn test_normalize_nested_object() {
        let input = json!({
            "z": {"nested_b": 2, "nested_a": 1},
            "a": {"x": 3}
        });
        let result = normalize(input).unwrap();

        let decoded = BASE64.decode(&result.bytes).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        // Both outer and inner keys should be sorted
        assert_eq!(
            decoded_str,
            r#"{"a":{"x":3},"z":{"nested_a":1,"nested_b":2}}"#
        );
    }

    #[test]
    fn test_normalize_deterministic() {
        // Same input should produce same CID
        let input1 = json!({"b": 2, "a": 1});
        let input2 = json!({"a": 1, "b": 2});

        let result1 = normalize(input1).unwrap();
        let result2 = normalize(input2).unwrap();

        assert_eq!(result1.cid, result2.cid);
        assert_eq!(result1.bytes, result2.bytes);
    }
}
