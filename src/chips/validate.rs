use crate::cas::Cas;
use crate::chips::normalize;
use crate::types::{Cid, ValidateOutput};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::Value;

/// Validate a JSON value against a JSON Schema stored in CAS
///
/// THE CANON:
/// 1. Input value → normalize() → canonical bytes → store in CAS by normalized.cid
/// 2. Schema → fetch from CAS by schema_cid → parse → compile
/// 3. Validate canonical value against schema
pub fn validate(value: Value, schema_cid: Cid, cas: &Cas) -> Result<ValidateOutput> {
    // Step 1: Normalize input to canonical form
    let normalized_value = normalize(value)?;

    // Step 2: Store normalized value in CAS (following the canon)
    let canonical_bytes = BASE64
        .decode(&normalized_value.bytes)
        .map_err(|e| RhoError::Validate(format!("Failed to decode normalized bytes: {}", e)))?;
    let value_cid = cas.put(canonical_bytes.clone())?;

    // Verify CID matches (canon check)
    if value_cid != normalized_value.cid {
        return Err(RhoError::Validate(format!(
            "CID mismatch: expected {}, got {}",
            normalized_value.cid, value_cid
        )));
    }

    // Step 3: Fetch schema from CAS by schema_cid (schema must be pre-normalized and stored)
    let schema_bytes = cas
        .get(&schema_cid)
        .map_err(|e| RhoError::Validate(format!("Schema not found in CAS: {}", e)))?;

    let schema_json: Value = serde_json::from_slice(&schema_bytes)
        .map_err(|e| RhoError::Validate(format!("Invalid schema JSON: {}", e)))?;

    // Step 4: Parse canonical value from CAS
    let canonical_value: Value = serde_json::from_slice(&canonical_bytes)
        .map_err(|e| RhoError::Validate(format!("Failed to parse canonical value: {}", e)))?;

    // Step 5: Compile schema and validate
    let compiled = jsonschema::JSONSchema::compile(&schema_json)
        .map_err(|e| RhoError::Validate(format!("Failed to compile schema: {}", e)))?;

    if compiled.is_valid(&canonical_value) {
        Ok(ValidateOutput {
            valid: true,
            errors: None,
        })
    } else {
        let validation_result = compiled.validate(&canonical_value);
        let error_messages: Vec<String> = match validation_result {
            Err(errors) => errors
                .map(|e| format!("{}: {}", e.instance_path, e))
                .collect(),
            Ok(_) => vec![],
        };

        Ok(ValidateOutput {
            valid: false,
            errors: Some(error_messages),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_follows_canon() {
        let cas = Cas::new();

        // Define schema and normalize it (THE CANON)
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });

        // Schema must be normalized and stored following the canon
        let normalized_schema = crate::chips::normalize(schema).unwrap();
        let schema_bytes = BASE64.decode(&normalized_schema.bytes).unwrap();
        let schema_cid = cas.put(schema_bytes).unwrap();

        // Verify canon: CID matches
        assert_eq!(schema_cid, normalized_schema.cid);

        // Now validate a value (it will be normalized internally)
        let value = json!({"name": "Alice", "extra": null});
        let result = validate(value, schema_cid, &cas).unwrap();

        assert!(result.valid);
    }

    #[test]
    fn test_validate_deterministic_key_order() {
        let cas = Cas::new();

        // Schema
        let schema = json!({
            "type": "object",
            "properties": {
                "a": {"type": "integer"},
                "b": {"type": "integer"}
            }
        });
        let norm_schema = crate::chips::normalize(schema).unwrap();
        let schema_cid = cas.put(BASE64.decode(&norm_schema.bytes).unwrap()).unwrap();

        // Same data, different key orders - should normalize to same canonical form
        let value1 = json!({"b": 2, "a": 1});
        let value2 = json!({"a": 1, "b": 2});

        let result1 = validate(value1, schema_cid.clone(), &cas).unwrap();
        let result2 = validate(value2, schema_cid, &cas).unwrap();

        assert!(result1.valid);
        assert!(result2.valid);
    }

    #[test]
    fn test_validate_rejects_invalid() {
        let cas = Cas::new();

        let schema = json!({
            "type": "object",
            "required": ["name"]
        });
        let norm_schema = crate::chips::normalize(schema).unwrap();
        let schema_cid = cas.put(BASE64.decode(&norm_schema.bytes).unwrap()).unwrap();

        let value = json!({"age": 30});
        let result = validate(value, schema_cid, &cas).unwrap();

        assert!(!result.valid);
        assert!(result.errors.is_some());
    }

    #[test]
    fn test_validate_rejects_float_at_normalization() {
        let cas = Cas::new();

        let schema = json!({"type": "object"});
        let norm_schema = crate::chips::normalize(schema).unwrap();
        let schema_cid = cas.put(BASE64.decode(&norm_schema.bytes).unwrap()).unwrap();

        // Float should be rejected during normalization (THE CANON: only i64)
        let value = json!({"count": 3.14});
        let result = validate(value, schema_cid, &cas);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("i64"));
    }
}
