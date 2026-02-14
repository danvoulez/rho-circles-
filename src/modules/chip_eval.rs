use crate::cas::Cas;
use crate::chips::{exec, normalize};
use crate::rc;
use crate::types::{Cid, ReciboCard};
use crate::Result;
use serde_json::{json, Value};

/// mod.chip.eval: Execute chip
///
/// Pipeline: cas.get(rb_cid) → normalize(inputs) → exec → rc.emit
///
/// Inputs:
/// - rb_cid: CID of the bytecode in CAS
/// - chip_inputs: Object with input values
///
/// Output: RC containing execution results
pub fn eval(rb_cid: Cid, chip_inputs: Value, cas: &Cas) -> Result<ReciboCard> {
    // Normalize inputs
    let _normalized_inputs = normalize(chip_inputs.clone())?;

    // Execute the chip
    let exec_output = exec(rb_cid.clone(), chip_inputs, cas)?;

    // Build result
    let result = json!({
        "rb_cid": rb_cid,
        "body": exec_output.body,
        "content_cid": exec_output.content_cid,
    });

    // Emit as RC
    rc::emit(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chips::compile;
    use crate::types::ChipSpec;
    use base64::Engine;
    use serde_json::json;

    #[test]
    fn test_eval_valid_chip() {
        let cas = Cas::new();

        // Create, compile, and store a chip
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

        let compiled = compile(chip_spec, None).unwrap();
        let rb_bytes = base64::engine::general_purpose::STANDARD
            .decode(&compiled.rb_bytes)
            .unwrap();
        let rb_cid = cas.put(rb_bytes).unwrap();

        // Evaluate the chip
        let inputs = json!({});
        let result = eval(rb_cid, inputs, &cas);
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert!(rc.body["content_cid"].is_string());
    }

    #[test]
    fn test_eval_missing_bytecode() {
        let cas = Cas::new();
        let result = eval("nonexistent_cid".to_string(), json!({}), &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_deterministic() {
        let cas = Cas::new();

        // Create, compile, and store a chip
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

        let compiled = compile(chip_spec, None).unwrap();
        let rb_bytes = base64::engine::general_purpose::STANDARD
            .decode(&compiled.rb_bytes)
            .unwrap();
        let rb_cid = cas.put(rb_bytes).unwrap();

        // Evaluate twice
        let inputs = json!({});
        let rc1 = eval(rb_cid.clone(), inputs.clone(), &cas).unwrap();
        let rc2 = eval(rb_cid, inputs, &cas).unwrap();
        assert_eq!(rc1.body["content_cid"], rc2.body["content_cid"]);
    }
}
