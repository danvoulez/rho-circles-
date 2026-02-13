use crate::cas::Cas;
use crate::types::{Cid, ExecOutput};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::Value;

/// Execute a TLV bytecode program with given inputs
pub fn exec(rb_cid: Cid, inputs: Value, cas: &Cas) -> Result<ExecOutput> {
    // Fetch bytecode from CAS
    let rb_bytes = cas
        .get(&rb_cid)
        .map_err(|e| RhoError::Exec(format!("Failed to fetch bytecode: {}", e)))?;

    // Decode if base64
    let bytecode = if let Ok(decoded) = BASE64.decode(&rb_bytes) {
        decoded
    } else {
        rb_bytes
    };

    // Parse bytecode
    if bytecode.len() < 2 {
        return Err(RhoError::Exec("Bytecode too short".to_string()));
    }

    let version = bytecode[0];
    let opcode = bytecode[1];

    if version != 0x01 {
        return Err(RhoError::Exec(format!("Unsupported version: {}", version)));
    }

    // Execute based on opcode
    let body = match opcode {
        2 => {
            // rho.normalize
            crate::chips::normalize(inputs)?;
            serde_json::json!({"status": "normalized"})
        }
        3 => {
            // rho.validate - simplified
            serde_json::json!({"status": "validated"})
        }
        _ => {
            // For other opcodes or modules, return input as output
            inputs
        }
    };

    // Hash the body
    let content_cid = BASE64.encode(blake3::hash(body.to_string().as_bytes()).as_bytes());

    Ok(ExecOutput { body, content_cid })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_exec_simple() {
        let cas = Cas::new();
        let bytecode = vec![0x01, 0x00]; // Version 1, opcode 0
        let rb_cid = cas.put(bytecode).unwrap();

        let inputs = json!({"test": "value"});
        let result = exec(rb_cid, inputs.clone(), &cas).unwrap();
        assert_eq!(result.body, inputs);
    }

    #[test]
    fn test_exec_missing_bytecode() {
        let cas = Cas::new();
        let inputs = json!({"test": "value"});
        let result = exec("nonexistent".to_string(), inputs, &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_exec_deterministic() {
        let cas = Cas::new();
        let bytecode = vec![0x01, 0x00];
        let rb_cid = cas.put(bytecode).unwrap();

        let inputs = json!({"a": 1, "b": 2});
        let r1 = exec(rb_cid.clone(), inputs.clone(), &cas).unwrap();
        let r2 = exec(rb_cid, inputs, &cas).unwrap();
        assert_eq!(r1.content_cid, r2.content_cid);
    }
}
