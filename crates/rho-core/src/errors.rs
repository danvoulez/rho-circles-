use thiserror::Error;

#[derive(Error, Debug)]
pub enum RhoError {
    #[error("Normalization error: {0}")]
    Normalize(String),
    
    #[error("Validation error: {0}")]
    Validate(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RhoError>;
