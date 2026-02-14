use crate::cas::Cas;
use crate::chips::{normalize, validate};
use crate::rc;
use crate::types::{Cid, ReciboCard};
use crate::{Result, RhoError};
use base64::Engine;
use serde_json::{json, Value};

/// mod.chip.publish: Publish chip definition to registry
///
/// Pipeline: normalize(chip_spec) → validate → cas.put → rc.emit
///
/// Inputs:
/// - chip_spec: ChipSpec object
/// - owner_cid: CID of the owner's public key
///
/// Output: RC containing the published chip's CID
pub fn publish(chip_spec: Value, owner_cid: Cid, cas: &Cas) -> Result<ReciboCard> {
    // Normalize the chip spec
    let normalized = normalize(chip_spec.clone())?;

    // Define basic chip spec schema
    let schema = json!({
        "type": "object",
        "properties": {
            "chip": {"type": "string"},
            "version": {"type": "string"},
            "type": {"type": "string", "enum": ["base", "module", "product"]},
            "inputs": {"type": "object"},
            "outputs": {"type": "object"}
        },
        "required": ["chip", "version", "type", "inputs", "outputs"]
    });

    // Store schema in CAS
    let schema_normalized = normalize(schema)?;
    let schema_bytes = base64::engine::general_purpose::STANDARD.decode(&schema_normalized.bytes)?;
    let schema_cid = cas.put(schema_bytes)?;

    // Validate the chip spec
    let validation = validate(chip_spec.clone(), schema_cid, cas)?;
    if !validation.valid {
        return Err(RhoError::ValidationFailed(format!(
            "Chip spec validation failed: {:?}",
            validation.errors
        )));
    }

    // Store the chip spec in CAS
    let chip_bytes = base64::engine::general_purpose::STANDARD.decode(&normalized.bytes)?;
    let chip_cid = cas.put(chip_bytes)?;

    // Build result
    let result = json!({
        "chip_cid": chip_cid,
        "owner_cid": owner_cid,
        "published_at": "deterministic_timestamp",  // In real impl, this would be from ledger
    });

    // Emit as RC
    rc::emit(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_valid_chip() {
        let cas = Cas::new();
        let chip_spec = json!({
            "chip": "test.chip",
            "version": "1.0.0",
            "type": "module",
            "inputs": {"value": {"type": "string"}},
            "outputs": {"result": {"type": "string"}}
        });
        let owner_cid = "owner_123".to_string();
        let result = publish(chip_spec, owner_cid, &cas);
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert!(rc.body["chip_cid"].is_string());
    }

    #[test]
    fn test_publish_invalid_chip() {
        let cas = Cas::new();
        let chip_spec = json!({
            "chip": "test.chip",
            // Missing required fields
        });
        let owner_cid = "owner_123".to_string();
        let result = publish(chip_spec, owner_cid, &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_publish_deterministic() {
        let cas = Cas::new();
        let chip_spec = json!({
            "chip": "test.chip",
            "version": "1.0.0",
            "type": "module",
            "inputs": {"value": {"type": "string"}},
            "outputs": {"result": {"type": "string"}}
        });
        let owner_cid = "owner_123".to_string();
        let rc1 = publish(chip_spec.clone(), owner_cid.clone(), &cas).unwrap();
        let rc2 = publish(chip_spec, owner_cid, &cas).unwrap();
        assert_eq!(rc1.body["chip_cid"], rc2.body["chip_cid"]);
    }
}
