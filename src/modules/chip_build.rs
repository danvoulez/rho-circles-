use crate::cas::Cas;
use crate::chips::compile;
use crate::rc;
use crate::types::{ChipSpec, Cid, ReciboCard};
use crate::Result;
use base64::Engine;
use serde_json::json;

/// mod.chip.build: Compile chip to bytecode
///
/// Pipeline: cas.get(spec_cid) → compile → cas.put(rb_cid) → rc.emit
///
/// Inputs:
/// - spec_cid: CID of the chip spec in CAS
///
/// Output: RC containing the compiled bytecode CID
pub fn build(spec_cid: Cid, cas: &Cas) -> Result<ReciboCard> {
    // Fetch chip spec from CAS
    let spec_bytes = cas.get(&spec_cid)?;
    let chip_spec: ChipSpec = serde_json::from_slice(&spec_bytes)?;

    // Compile the chip spec to bytecode
    let compiled = compile(chip_spec, None)?;

    // Store the bytecode in CAS
    let rb_bytes = base64::engine::general_purpose::STANDARD.decode(&compiled.rb_bytes)?;
    let rb_cid = cas.put(rb_bytes)?;

    // Verify CID consistency
    if rb_cid != compiled.rb_cid {
        return Err(crate::RhoError::CidMismatch {
            expected: compiled.rb_cid,
            actual: rb_cid,
        });
    }

    // Build result
    let result = json!({
        "spec_cid": spec_cid,
        "rb_cid": rb_cid,
        "compiled_at": "deterministic_timestamp",
    });

    // Emit as RC
    rc::emit(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use serde_json::json;

    #[test]
    fn test_build_valid_chip() {
        let cas = Cas::new();

        // Create and store a chip spec
        let chip_spec = ChipSpec {
            chip: "test.chip".to_string(),
            version: "1.0.0".to_string(),
            chip_type: crate::types::ChipType::Module,
            inputs: json!({"value": {"type": "string"}}),
            outputs: json!({"result": {"type": "string"}}),
            determinism: None,
            opcode: Some(10),
            wiring: None,
        };

        let normalized =
            crate::chips::normalize(serde_json::to_value(&chip_spec).unwrap()).unwrap();
        let spec_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized.bytes)
            .unwrap();
        let spec_cid = cas.put(spec_bytes).unwrap();

        // Build the chip
        let result = build(spec_cid, &cas);
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert!(rc.body["rb_cid"].is_string());
        assert_eq!(rc.body["spec_cid"], normalized.cid);
    }

    #[test]
    fn test_build_missing_spec() {
        let cas = Cas::new();
        let result = build("nonexistent_cid".to_string(), &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_deterministic() {
        let cas = Cas::new();

        // Create and store a chip spec
        let chip_spec = ChipSpec {
            chip: "test.chip".to_string(),
            version: "1.0.0".to_string(),
            chip_type: crate::types::ChipType::Module,
            inputs: json!({}),
            outputs: json!({}),
            determinism: None,
            opcode: Some(10),
            wiring: None,
        };

        let normalized =
            crate::chips::normalize(serde_json::to_value(&chip_spec).unwrap()).unwrap();
        let spec_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized.bytes)
            .unwrap();
        let spec_cid = cas.put(spec_bytes).unwrap();

        // Build twice
        let rc1 = build(spec_cid.clone(), &cas).unwrap();
        let rc2 = build(spec_cid, &cas).unwrap();
        assert_eq!(rc1.body["rb_cid"], rc2.body["rb_cid"]);
    }
}
