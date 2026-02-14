use crate::cas::Cas;
use crate::chips::{normalize, validate};
use crate::rc;
use crate::types::ReciboCard;
use crate::{Result, RhoError};
use base64::Engine;
use serde_json::{json, Value};

/// mod.log: Structured logging with validation
///
/// Pipeline: normalize → validate(schema) → rc.emit
///
/// Inputs:
/// - level: "info" | "warn" | "error"
/// - message: string
/// - fields: object (optional)
///
/// Output: RC (Recibo Card)
pub fn log(level: String, message: String, fields: Option<Value>, cas: &Cas) -> Result<ReciboCard> {
    // Validate level
    if !["info", "warn", "error"].contains(&level.as_str()) {
        return Err(RhoError::InvalidInput(format!(
            "Invalid log level: {}. Must be one of: info, warn, error",
            level
        )));
    }

    // Build log entry
    let mut log_entry = json!({
        "level": level,
        "message": message,
    });

    if let Some(f) = fields {
        log_entry["fields"] = f;
    }

    // Normalize the log entry
    let _normalized = normalize(log_entry.clone())?;

    // Define schema for log entry validation
    let schema = json!({
        "type": "object",
        "properties": {
            "level": {
                "type": "string",
                "enum": ["info", "warn", "error"]
            },
            "message": {
                "type": "string"
            },
            "fields": {
                "type": "object"
            }
        },
        "required": ["level", "message"],
        "additionalProperties": false
    });

    // Store schema in CAS
    let schema_normalized = normalize(schema)?;
    let schema_bytes =
        base64::engine::general_purpose::STANDARD.decode(&schema_normalized.bytes)?;
    let schema_cid = cas.put(schema_bytes)?;

    // Validate the log entry
    let validation = validate(log_entry.clone(), schema_cid, cas)?;
    if !validation.valid {
        return Err(RhoError::ValidationFailed(format!(
            "Log entry validation failed: {:?}",
            validation.errors
        )));
    }

    // Emit as RC
    rc::emit(log_entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_info() {
        let cas = Cas::new();
        let result = log("info".to_string(), "Test message".to_string(), None, &cas);
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert_eq!(rc.body["level"], "info");
        assert_eq!(rc.body["message"], "Test message");
    }

    #[test]
    fn test_log_with_fields() {
        let cas = Cas::new();
        let fields = json!({"user_id": "123", "action": "login"});
        let result = log(
            "warn".to_string(),
            "User login attempt".to_string(),
            Some(fields.clone()),
            &cas,
        );
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert_eq!(rc.body["fields"], fields);
    }

    #[test]
    fn test_log_invalid_level() {
        let cas = Cas::new();
        let result = log("debug".to_string(), "Test message".to_string(), None, &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_deterministic() {
        let cas = Cas::new();
        let rc1 = log("info".to_string(), "Test".to_string(), None, &cas).unwrap();
        let rc2 = log("info".to_string(), "Test".to_string(), None, &cas).unwrap();
        assert_eq!(rc1.recibo.content_cid, rc2.recibo.content_cid);
    }
}
