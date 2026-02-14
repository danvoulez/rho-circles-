use crate::normalize;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct NormalizeOutput {
    pub normalized: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateOutput {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Initialize the WASM module (can be called for any setup if needed)
#[wasm_bindgen]
pub fn init() {
    // Set panic hook for better error messages in the future
    // Currently no additional setup needed
}

/// Normalize a JSON value and return canonical form with CID
/// 
/// Input: any JS value (object, array, string, number, etc.)
/// Output: { normalized: string, cid: string }
#[wasm_bindgen]
pub fn wasm_normalize(input: JsValue) -> Result<JsValue, JsValue> {
    // Convert JS value to serde_json::Value
    let value: serde_json::Value = serde_wasm_bindgen::from_value(input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;

    // Normalize
    let (normalized, cid) = normalize::normalize(value)
        .map_err(|e| JsValue::from_str(&format!("Normalization error: {}", e)))?;

    // Return as JS object
    let output = NormalizeOutput { normalized, cid };
    serde_wasm_bindgen::to_value(&output)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize output: {}", e)))
}

/// Validate is a simplified version for now - in full implementation,
/// it would fetch schema from CAS and validate against it
#[wasm_bindgen]
pub fn wasm_validate(_schema_cid: String, input: JsValue) -> Result<JsValue, JsValue> {
    // For now, we'll just normalize the input to ensure it's valid JSON
    // Full implementation would fetch schema from CAS
    let value: serde_json::Value = serde_wasm_bindgen::from_value(input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;

    // Attempt to normalize (this validates the input follows CANON rules)
    match normalize::normalize(value) {
        Ok(_) => {
            let output = ValidateOutput {
                ok: true,
                errors: None,
            };
            serde_wasm_bindgen::to_value(&output)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize output: {}", e)))
        }
        Err(e) => {
            let output = ValidateOutput {
                ok: false,
                errors: Some(vec![format!("Validation error: {}", e)]),
            };
            serde_wasm_bindgen::to_value(&output)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize output: {}", e)))
        }
    }
}

/// Compute CID from raw bytes (base64url, no padding)
#[wasm_bindgen]
pub fn wasm_cid(bytes: &[u8]) -> String {
    normalize::compute_cid(bytes)
}
