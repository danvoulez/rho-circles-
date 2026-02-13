use serde::{Deserialize, Serialize};

/// Content Identifier (CID) - a blake3 hash encoded as base64
pub type Cid = String;

/// Opcode for ISA operations
pub type Opcode = u8;

/// Chip specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChipSpec {
    pub chip: String,
    pub version: String,
    #[serde(rename = "type")]
    pub chip_type: ChipType,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub determinism: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opcode: Option<Opcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiring: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChipType {
    Base,
    Module,
    Product,
}

/// Normalized value output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeOutput {
    pub bytes: String, // base64 encoded
    pub cid: Cid,
}

/// Validation output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Policy evaluation output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvalOutput {
    pub result: bool,
}

/// Compilation output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOutput {
    pub rb_bytes: String, // base64 encoded
    pub rb_cid: Cid,
}

/// Execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecOutput {
    pub body: serde_json::Value,
    pub content_cid: Cid,
}

/// Signature proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub algorithm: String,
    pub public_key: String,
    pub signature: String,
    pub message_cid: Cid,
}
