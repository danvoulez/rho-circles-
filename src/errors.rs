use thiserror::Error;

#[derive(Error, Debug)]
pub enum RhoError {
    #[error("Normalization error: {0}")]
    Normalize(String),

    #[error("Validation error: {0}")]
    Validate(String),

    #[error("Policy evaluation error: {0}")]
    Policy(String),

    #[error("Compilation error: {0}")]
    Compile(String),

    #[error("Execution error: {0}")]
    Exec(String),

    #[error("CAS error: {0}")]
    Cas(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("CID not found: {0}")]
    CidNotFound(String),

    #[error("CID mismatch: expected {expected}, got {actual}")]
    CidMismatch { expected: String, actual: String },
}

pub type Result<T> = std::result::Result<T, RhoError>;
