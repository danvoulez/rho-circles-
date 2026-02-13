use crate::cas::Cas;
use crate::types::{Cid, ValidateOutput};
use crate::{Result, RhoError};
use serde_json::Value;

/// Validate a JSON value against a JSON Schema stored in CAS
pub fn validate(value: Value, schema_cid: Cid, cas: &Cas) -> Result<ValidateOutput> {
    // Fetch schema from CAS
    let schema_bytes = cas
        .get(&schema_cid)
        .map_err(|e| RhoError::Validate(format!("Failed to fetch schema: {}", e)))?;

    let schema_json: Value = serde_json::from_slice(&schema_bytes)
        .map_err(|e| RhoError::Validate(format!("Invalid schema JSON: {}", e)))?;

    // Compile the schema
    let compiled = jsonschema::JSONSchema::compile(&schema_json)
        .map_err(|e| RhoError::Validate(format!("Failed to compile schema: {}", e)))?;

    // Validate the value
    if compiled.is_valid(&value) {
        Ok(ValidateOutput {
            valid: true,
            errors: None,
        })
    } else {
        // Get validation errors
        let validation_result = compiled.validate(&value);
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
    fn test_validate_simple_schema() {
        let cas = Cas::new();

        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });
        let schema_cid = cas.put(serde_json::to_vec(&schema).unwrap()).unwrap();

        let valid_value = json!({"name": "Alice", "age": 30});
        let result = validate(valid_value, schema_cid.clone(), &cas).unwrap();
        assert!(result.valid);

        let invalid_value = json!({"age": 30});
        let result = validate(invalid_value, schema_cid, &cas).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn test_validate_type_mismatch() {
        let cas = Cas::new();

        let schema = json!({
            "type": "object",
            "properties": {
                "count": {"type": "integer"}
            }
        });
        let schema_cid = cas.put(serde_json::to_vec(&schema).unwrap()).unwrap();

        let value = json!({"count": "not a number"});
        let result = validate(value, schema_cid, &cas).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn test_validate_missing_schema() {
        let cas = Cas::new();
        let value = json!({"test": "value"});
        let result = validate(value, "nonexistent_cid".to_string(), &cas);
        assert!(result.is_err());
    }
}
