/// Integration tests for rho.normalize based on spec test vectors
use rho_circles::chips::normalize;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[test]
fn test_spec_vector_1_key_sorting() {
    // Test vector 1: {"b":2,"a":1} should canonicalize to {"a":1,"b":2}
    let input = json!({"b": 2, "a": 1});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, r#"{"a":1,"b":2}"#);
}

#[test]
fn test_spec_vector_2_unicode_nfc() {
    // Test vector 2: {"x":"café"} should be NFC-encoded
    let input = json!({"x": "café"});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    // Should contain NFC-normalized "café"
    assert!(canonical.contains("café"));
    assert_eq!(canonical, r#"{"x":"café"}"#);
}

#[test]
fn test_spec_vector_3_integer_only() {
    // Test vector 3: 123 (i64) should pass, 123.0 should fail
    let valid_input = json!(123);
    let result = normalize(valid_input);
    assert!(result.is_ok());
    
    let decoded = BASE64.decode(&result.unwrap().bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    assert_eq!(canonical, "123");
    
    // Float should be rejected
    let invalid_input = json!(123.0);
    let result = normalize(invalid_input);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("i64"));
}

#[test]
fn test_spec_vector_4_null_in_array() {
    // Test vector 4: [null] should keep null (not removed from arrays)
    let input = json!([null]);
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, "[null]");
}

#[test]
fn test_null_removal_from_objects() {
    // Null values should be removed from objects
    let input = json!({"a": 1, "b": null, "c": 3});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, r#"{"a":1,"c":3}"#);
    assert!(!canonical.contains("\"b\""));
}

#[test]
fn test_determinism_across_runs() {
    // Same logical value in different orders should produce same CID
    let input1 = json!({"z": 3, "y": 2, "x": 1});
    let input2 = json!({"x": 1, "y": 2, "z": 3});
    let input3 = json!({"y": 2, "z": 3, "x": 1});
    
    let result1 = normalize(input1).unwrap();
    let result2 = normalize(input2).unwrap();
    let result3 = normalize(input3).unwrap();
    
    assert_eq!(result1.cid, result2.cid);
    assert_eq!(result2.cid, result3.cid);
    assert_eq!(result1.bytes, result2.bytes);
}

#[test]
fn test_nested_object_normalization() {
    // Nested objects should have keys sorted at all levels
    let input = json!({
        "outer_z": {
            "inner_z": 3,
            "inner_a": 1
        },
        "outer_a": {
            "inner_m": 2
        }
    });
    
    let result = normalize(input).unwrap();
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    // Both outer and inner keys should be sorted
    assert_eq!(canonical, r#"{"outer_a":{"inner_m":2},"outer_z":{"inner_a":1,"inner_z":3}}"#);
}

#[test]
fn test_array_elements_not_sorted() {
    // Array elements should maintain their order (not sorted)
    let input = json!([3, 1, 2]);
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, "[3,1,2]");
}

#[test]
fn test_nested_arrays() {
    // Nested arrays should be preserved
    let input = json!([[3, 2, 1], [6, 5, 4]]);
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, "[[3,2,1],[6,5,4]]");
}

#[test]
fn test_boolean_values() {
    // Boolean values should be preserved
    let input = json!({"t": true, "f": false});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, r#"{"f":false,"t":true}"#);
}

#[test]
fn test_empty_object() {
    // Empty objects should work
    let input = json!({});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, "{}");
}

#[test]
fn test_empty_array() {
    // Empty arrays should work
    let input = json!([]);
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    assert_eq!(canonical, "[]");
}

#[test]
fn test_cid_generation() {
    // CID should be base64-encoded blake3 hash
    let input = json!({"test": "value"});
    let result = normalize(input).unwrap();
    
    // CID should be non-empty and base64-encoded
    assert!(!result.cid.is_empty());
    
    // Should be able to decode as base64
    let decoded = BASE64.decode(&result.cid);
    assert!(decoded.is_ok());
    
    // Blake3 hash is 32 bytes
    assert_eq!(decoded.unwrap().len(), 32);
}

#[test]
fn test_large_nested_structure() {
    // Test with a complex nested structure
    let input = json!({
        "users": [
            {"name": "Alice", "age": 30, "tags": ["admin", "user"]},
            {"name": "Bob", "age": 25, "tags": ["user"]}
        ],
        "metadata": {
            "version": 1,
            "created": 1234567890
        },
        "settings": {
            "theme": "dark",
            "language": "en"
        }
    });
    
    let result1 = normalize(input.clone()).unwrap();
    let result2 = normalize(input).unwrap();
    
    // Should be deterministic
    assert_eq!(result1.cid, result2.cid);
}

#[test]
fn test_unicode_in_keys() {
    // Unicode in object keys should be NFC-normalized
    let input = json!({"café": 1, "naïve": 2});
    let result = normalize(input).unwrap();
    
    let decoded = BASE64.decode(&result.bytes).unwrap();
    let canonical = String::from_utf8(decoded).unwrap();
    
    // Keys should be sorted and NFC-normalized
    assert!(canonical.contains("café"));
    assert!(canonical.contains("naïve"));
}
