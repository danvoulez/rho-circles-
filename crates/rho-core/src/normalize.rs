use crate::errors::{Result, RhoError};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

/// Normalize a JSON value to canonical form according to THE CANON
///
/// Algorithm:
/// 1. Parse input as JSON
/// 2. Normalize Unicode – apply NFC to all strings
/// 3. Normalize numbers – reject floats, only i64
/// 4. Normalize null/absent – drop object keys with null values
/// 5. Sort object keys recursively (lexicographic)
/// 6. Serialize to canonical JSON (no whitespace)
/// 7. Return normalized string and blake3 CID in base64url (no padding)
pub fn normalize(value: Value) -> Result<(String, String)> {
    // Normalize the value recursively
    let normalized = normalize_value(value)?;

    // Serialize to canonical JSON (no whitespace)
    let canonical_json = serde_json::to_string(&normalized)?;
    let canonical_bytes = canonical_json.as_bytes();

    // Compute blake3 hash and encode as base64url without padding
    let hash = blake3::hash(canonical_bytes);
    let cid = URL_SAFE_NO_PAD.encode(hash.as_bytes());

    Ok((canonical_json, cid))
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

            // Sort keys
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

/// Compute CID from raw bytes
pub fn compute_cid(bytes: &[u8]) -> String {
    let hash = blake3::hash(bytes);
    URL_SAFE_NO_PAD.encode(hash.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_normalize_simple_object() {
        let input = json!({"b": 2, "a": 1});
        let (normalized, _cid) = normalize(input).unwrap();
        assert_eq!(normalized, r#"{"a":1,"b":2}"#);
    }

    #[test]
    fn test_normalize_unicode_nfc() {
        let input = json!({"x": "café"});
        let (normalized, _cid) = normalize(input).unwrap();
        assert!(normalized.contains("café"));
    }

    #[test]
    fn test_normalize_rejects_float() {
        let input = json!(123.5);
        let result = normalize(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("i64"));
    }

    #[test]
    fn test_normalize_removes_null() {
        let input = json!({"a": 1, "b": null, "c": 3});
        let (normalized, _cid) = normalize(input).unwrap();
        assert_eq!(normalized, r#"{"a":1,"c":3}"#);
        assert!(!normalized.contains("\"b\""));
    }

    #[test]
    fn test_normalize_deterministic() {
        let input1 = json!({"b": 2, "a": 1});
        let input2 = json!({"a": 1, "b": 2});

        let (_norm1, cid1) = normalize(input1).unwrap();
        let (_norm2, cid2) = normalize(input2).unwrap();

        assert_eq!(cid1, cid2);
    }

    #[test]
    fn test_cid_base64url_no_padding() {
        let input = json!({"test": 123});
        let (_normalized, cid) = normalize(input).unwrap();
        
        // CID should not contain padding characters
        assert!(!cid.contains('='));
        // Should be base64url (no + or /)
        assert!(!cid.contains('+'));
        assert!(!cid.contains('/'));
    }

    #[test]
    fn test_normalize_idempotent() {
        let input = json!({"b": 2, "a": 1});
        let (norm1, cid1) = normalize(input).unwrap();
        
        // Parse and normalize again
        let parsed: Value = serde_json::from_str(&norm1).unwrap();
        let (norm2, cid2) = normalize(parsed).unwrap();
        
        assert_eq!(norm1, norm2);
        assert_eq!(cid1, cid2);
    }
}
