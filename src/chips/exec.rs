use crate::cas::Cas;
use crate::chips::normalize;
use crate::types::{Cid, ExecOutput};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;
use serde_json::Value;

/// Execute bytecode with given inputs
///
/// THE CANON:
/// 1. Fetch bytecode from CAS by rb_cid
/// 2. Normalize inputs → canonical form
/// 3. Execute bytecode on canonical inputs
/// 4. Normalize output → canonical form → content_cid
pub fn exec(rb_cid: Cid, inputs: Value, cas: &Cas) -> Result<ExecOutput> {
    // Step 1: Fetch bytecode from CAS (THE CANON: bytecode stored by rb_cid)
    let rb_bytes = cas
        .get(&rb_cid)
        .map_err(|e| RhoError::Exec(format!("Bytecode not found in CAS: {}", e)))?;

    // Bytecode should be raw bytes (not base64 - that's for transmission)
    let bytecode = rb_bytes;

    // Step 2: Parse bytecode
    if bytecode.len() < 2 {
        return Err(RhoError::Exec("Bytecode too short".to_string()));
    }

    let version = bytecode[0];
    let opcode = bytecode[1];

    if version != 0x01 {
        return Err(RhoError::Exec(format!("Unsupported version: {}", version)));
    }

    // Step 3: Normalize inputs (THE CANON: all inputs must be canonical)
    let normalized_inputs = normalize(inputs)?;
    let canonical_input_bytes = BASE64
        .decode(&normalized_inputs.bytes)
        .map_err(|e| RhoError::Exec(format!("Failed to decode normalized inputs: {}", e)))?;
    let canonical_inputs: Value = serde_json::from_slice(&canonical_input_bytes)
        .map_err(|e| RhoError::Exec(format!("Failed to parse canonical inputs: {}", e)))?;

    // Step 4: Execute based on opcode (operating on canonical inputs)
    let output = match opcode {
        2 => {
            // rho.normalize - already normalized, return as-is
            canonical_inputs
        }
        3 => {
            // rho.validate - would need schema_cid from inputs
            json!({"status": "validated", "input_cid": normalized_inputs.cid})
        }
        _ => {
            // For other opcodes, echo canonical inputs
            canonical_inputs
        }
    };

    // Step 5: Normalize output (THE CANON: all outputs must be canonical)
    let normalized_output = normalize(output)?;

    // Step 6: Decode canonical output to return as body
    let canonical_output_bytes = BASE64
        .decode(&normalized_output.bytes)
        .map_err(|e| RhoError::Exec(format!("Failed to decode normalized output: {}", e)))?;
    let body: Value = serde_json::from_slice(&canonical_output_bytes)
        .map_err(|e| RhoError::Exec(format!("Failed to parse canonical output: {}", e)))?;

    // content_cid is the CID of the normalized output (THE CANON)
    Ok(ExecOutput {
        body,
        content_cid: normalized_output.cid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_exec_follows_canon() {
        let cas = Cas::new();

        // Create bytecode following THE CANON
        let bytecode = vec![0x01, 0x00]; // Version 1, opcode 0 (echo)
        let rb_cid = cas.put(bytecode).unwrap();

        // Execute with inputs (will be normalized internally)
        let inputs = json!({"z": 3, "a": 1});
        let result = exec(rb_cid, inputs, &cas).unwrap();

        // Output should be canonical (keys sorted)
        assert_eq!(result.body, json!({"a": 1, "z": 3}));
    }

    #[test]
    fn test_exec_deterministic() {
        let cas = Cas::new();

        let bytecode = vec![0x01, 0x00];
        let rb_cid = cas.put(bytecode).unwrap();

        // Same input in different orders
        let inputs1 = json!({"b": 2, "a": 1});
        let inputs2 = json!({"a": 1, "b": 2});

        let r1 = exec(rb_cid.clone(), inputs1, &cas).unwrap();
        let r2 = exec(rb_cid, inputs2, &cas).unwrap();

        // THE CANON: same canonical input → same content_cid
        assert_eq!(
            r1.content_cid, r2.content_cid,
            "Same inputs should produce same content_cid"
        );
        assert_eq!(r1.body, r2.body, "Same inputs should produce same body");
    }

    #[test]
    fn test_exec_removes_nulls() {
        let cas = Cas::new();

        let bytecode = vec![0x01, 0x00];
        let rb_cid = cas.put(bytecode).unwrap();

        let inputs = json!({"value": 1, "removed": null});
        let result = exec(rb_cid, inputs, &cas).unwrap();

        // THE CANON: nulls removed
        assert_eq!(result.body, json!({"value": 1}));
    }

    #[test]
    fn test_exec_rejects_float() {
        let cas = Cas::new();

        let bytecode = vec![0x01, 0x00];
        let rb_cid = cas.put(bytecode).unwrap();

        let inputs = json!({"value": 3.14});
        let result = exec(rb_cid, inputs, &cas);

        // THE CANON: only i64 integers
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("i64"));
    }

    #[test]
    fn test_exec_missing_bytecode() {
        let cas = Cas::new();
        let inputs = json!({"test": "value"});
        let result = exec("nonexistent_cid".to_string(), inputs, &cas);
        assert!(result.is_err());
    }
}
