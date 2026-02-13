use crate::chips::normalize;
use crate::types::{ChipSpec, ChipType, CompileOutput};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Compile a chip_spec into deterministic TLV bytecode
///
/// THE CANON:
/// 1. chip_spec → normalize() → canonical bytes
/// 2. Canonical spec → TLV bytecode
/// 3. bytecode → blake3 → rb_cid
pub fn compile(
    chip_spec: ChipSpec,
    _dependencies: Option<serde_json::Value>,
) -> Result<CompileOutput> {
    // Step 1: Serialize and normalize chip_spec (THE CANON)
    let chip_json = serde_json::to_value(&chip_spec)
        .map_err(|e| RhoError::Compile(format!("Failed to serialize chip_spec: {}", e)))?;

    let normalized = normalize(chip_json)?;

    // Step 2: Decode canonical bytes
    let canonical_bytes = BASE64
        .decode(&normalized.bytes)
        .map_err(|e| RhoError::Compile(format!("Failed to decode normalized bytes: {}", e)))?;

    // Step 3: Parse canonical chip_spec
    let canonical_spec: ChipSpec = serde_json::from_slice(&canonical_bytes)
        .map_err(|e| RhoError::Compile(format!("Failed to parse normalized chip_spec: {}", e)))?;

    // Step 4: Validate canonical spec
    validate_chip_spec(&canonical_spec)?;

    // Step 5: Compile canonical spec to TLV bytecode
    let bytecode = compile_to_tlv(&canonical_spec, &normalized.cid)?;

    // Step 6: Generate rb_cid from bytecode (THE CANON)
    let rb_cid = BASE64.encode(blake3::hash(&bytecode).as_bytes());
    let rb_bytes = BASE64.encode(&bytecode);

    Ok(CompileOutput { rb_bytes, rb_cid })
}

fn validate_chip_spec(spec: &ChipSpec) -> Result<()> {
    if spec.chip.is_empty() {
        return Err(RhoError::Compile("Chip name cannot be empty".to_string()));
    }
    if spec.chip_type == ChipType::Base && spec.opcode.is_none() {
        return Err(RhoError::Compile(
            "Base chips must have an opcode".to_string(),
        ));
    }
    Ok(())
}

fn compile_to_tlv(spec: &ChipSpec, spec_cid: &str) -> Result<Vec<u8>> {
    let mut bytecode = Vec::new();

    // Version
    bytecode.push(0x01);

    // Opcode
    bytecode.push(spec.opcode.unwrap_or(0));

    // Spec CID (for traceability - embed the canonical spec CID)
    let spec_cid_bytes = BASE64
        .decode(spec_cid)
        .map_err(|e| RhoError::Compile(format!("Invalid spec CID: {}", e)))?;
    bytecode.push(spec_cid_bytes.len() as u8);
    bytecode.extend_from_slice(&spec_cid_bytes);

    // Input count
    bytecode.push(match &spec.inputs {
        serde_json::Value::Object(m) => m.len() as u8,
        _ => 1,
    });

    // Output count
    bytecode.push(0x01);

    // Wiring (for modules)
    if let Some(wiring) = &spec.wiring {
        bytecode.push(wiring.len() as u8);
        for op in wiring {
            // Each wiring op should also be normalized
            let op_normalized = normalize(op.clone())?;
            let op_cid_bytes = BASE64
                .decode(&op_normalized.cid)
                .map_err(|e| RhoError::Compile(format!("Invalid op CID: {}", e)))?;
            bytecode.extend_from_slice(&op_cid_bytes);
        }
    } else {
        bytecode.push(0x00);
    }

    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_follows_canon() {
        // Same spec with different key orders should produce same bytecode
        let spec1 = ChipSpec {
            chip: "test".to_string(),
            version: "1.0.0".to_string(),
            chip_type: ChipType::Base,
            inputs: json!({"z": {"type": "string"}, "a": {"type": "integer"}}),
            outputs: json!({"result": {}}),
            determinism: Some("spec→rb".to_string()),
            opcode: Some(2),
            wiring: None,
        };

        let spec2 = ChipSpec {
            chip: "test".to_string(),
            version: "1.0.0".to_string(),
            chip_type: ChipType::Base,
            inputs: json!({"a": {"type": "integer"}, "z": {"type": "string"}}),
            outputs: json!({"result": {}}),
            determinism: Some("spec→rb".to_string()),
            opcode: Some(2),
            wiring: None,
        };

        let r1 = compile(spec1, None).unwrap();
        let r2 = compile(spec2, None).unwrap();

        // THE CANON: normalization ensures same bytecode
        assert_eq!(
            r1.rb_cid, r2.rb_cid,
            "Different key orders should produce same rb_cid"
        );
        assert_eq!(
            r1.rb_bytes, r2.rb_bytes,
            "Different key orders should produce same bytecode"
        );
    }

    #[test]
    fn test_compile_removes_nulls() {
        let spec = ChipSpec {
            chip: "test".to_string(),
            version: "1.0.0".to_string(),
            chip_type: ChipType::Base,
            inputs: json!({"value": {}, "removed": null}),
            outputs: json!({}),
            determinism: Some("spec→rb".to_string()),
            opcode: Some(2),
            wiring: None,
        };

        let result = compile(spec, None).unwrap();
        let decoded = BASE64.decode(&result.rb_bytes).unwrap();

        // Should have version and opcode
        assert_eq!(decoded[0], 0x01);
        assert_eq!(decoded[1], 0x02);
    }

    #[test]
    fn test_compile_deterministic() {
        let spec = ChipSpec {
            chip: "test".to_string(),
            version: "1.0.0".to_string(),
            chip_type: ChipType::Base,
            inputs: json!({"value": {}}),
            outputs: json!({}),
            determinism: Some("spec→rb".to_string()),
            opcode: Some(2),
            wiring: None,
        };

        let r1 = compile(spec.clone(), None).unwrap();
        let r2 = compile(spec, None).unwrap();

        // THE CANON: same input → same output
        assert_eq!(r1.rb_cid, r2.rb_cid);
        assert_eq!(r1.rb_bytes, r2.rb_bytes);
    }

    #[test]
    fn test_compile_rejects_float() {
        let spec = ChipSpec {
            chip: "test".to_string(),
            version: "1.0.0".to_string(),
            chip_type: ChipType::Base,
            inputs: json!({"value": 3.14}),
            outputs: json!({}),
            determinism: Some("spec→rb".to_string()),
            opcode: Some(2),
            wiring: None,
        };

        let result = compile(spec, None);

        // THE CANON: only i64 integers allowed
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("i64"));
    }
}
