use crate::types::{ChipSpec, ChipType, CompileOutput};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Compile a chip_spec DAG into deterministic TLV bytecode
pub fn compile(
    chip_spec: ChipSpec,
    _dependencies: Option<serde_json::Value>,
) -> Result<CompileOutput> {
    validate_chip_spec(&chip_spec)?;
    let bytecode = compile_to_tlv(&chip_spec)?;
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

fn compile_to_tlv(spec: &ChipSpec) -> Result<Vec<u8>> {
    let mut bytecode = Vec::new();
    bytecode.push(0x01); // Version
    bytecode.push(spec.opcode.unwrap_or(0)); // Opcode
    bytecode.push(match &spec.inputs {
        serde_json::Value::Object(m) => m.len() as u8,
        _ => 1,
    });
    let input_hash = blake3::hash(spec.inputs.to_string().as_bytes());
    bytecode.extend_from_slice(input_hash.as_bytes());
    bytecode.push(0x01); // OutputCount
    bytecode.push(0x01); // OutputType
    if let Some(wiring) = &spec.wiring {
        bytecode.push(wiring.len() as u8);
        for op in wiring {
            let op_hash = blake3::hash(op.to_string().as_bytes());
            bytecode.extend_from_slice(op_hash.as_bytes());
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
    fn test_compile_base_chip() {
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
        let result = compile(spec, None).unwrap();
        let decoded = BASE64.decode(&result.rb_bytes).unwrap();
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
        assert_eq!(r1.rb_cid, r2.rb_cid);
    }
}
