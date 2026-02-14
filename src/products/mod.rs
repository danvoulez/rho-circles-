// Outer Ring: Products
//
// Complete applications composed of middle ring modules

pub mod ai_passport;
pub mod api_notary;
pub mod content_sign;

// Re-export for convenience
pub use ai_passport::{
    register_model, register_with_hash, validate_compliance, verify_passport, AiPassport,
    PassportReceipt,
};
pub use api_notary::{notarize, verify as verify_notary, ApiTransaction, NotaryReceipt};
pub use content_sign::{
    sign_content, sign_json, verify as verify_content, verify_json, SignedContent, SignedReceipt,
};
