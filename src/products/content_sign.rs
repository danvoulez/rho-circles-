/// Product: Content Sign
///
/// CLI tool to sign files/JSONs and generate verifiable RC (Recibo Cards).
/// Provides "blue checkmark" for content, proving it came from a legitimate source.
///
/// Use case: Newsrooms, content creators, anti-fake news verification

use crate::cas::Cas;
use crate::chips::normalize;
use crate::rc;
use crate::types::{ReciboCard, Signature};
use crate::{Result, RhoError};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Signed content metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedContent {
    pub content_type: String, // "article", "image", "video", etc.
    pub title: String,
    pub author: String,
    pub timestamp: String, // ISO 8601 format
    pub content_cid: String, // CID of the actual content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Content signing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedReceipt {
    pub signed_content: SignedContent,
    pub receipt_card: ReciboCard,
}

/// Sign content and generate a verifiable receipt
///
/// Creates a Recibo Card that proves the content came from a specific author/organization.
/// The receipt can be verified offline by checking signatures and CID.
pub fn sign_content(
    content_type: String,
    title: String,
    author: String,
    timestamp: String,
    content: Vec<u8>,
    signatures: Vec<Signature>,
    metadata: Option<Value>,
    cas: &Cas,
) -> Result<SignedReceipt> {
    // Hash the actual content to get its CID
    let content_hash = blake3::hash(&content);
    let content_cid = BASE64.encode(content_hash.as_bytes());
    
    // Store content in CAS
    cas.put(content.clone())?;
    
    // Create signed content metadata
    let signed_content = SignedContent {
        content_type,
        title,
        author,
        timestamp,
        content_cid,
        metadata,
    };
    
    // Normalize and emit receipt
    let content_value = serde_json::to_value(&signed_content)?;
    normalize(content_value.clone())?; // Verify it's normalizable
    
    let receipt_card = rc::emit_with_signatures(content_value, signatures)?;
    
    Ok(SignedReceipt {
        signed_content,
        receipt_card,
    })
}

/// Sign a JSON document directly
///
/// Simpler version for pure JSON content (not binary files).
/// Useful for APIs, structured data, configuration files.
pub fn sign_json(
    content: Value,
    author: String,
    timestamp: String,
    signatures: Vec<Signature>,
) -> Result<ReciboCard> {
    // Normalize the content first to ensure it's valid
    let normalized = normalize(content.clone())?;
    
    // Create a wrapper with metadata
    let signed_doc = serde_json::json!({
        "author": author,
        "timestamp": timestamp,
        "content_cid": normalized.cid,
        "content": content,
    });
    
    // Emit receipt card
    rc::emit_with_signatures(signed_doc, signatures)
}

/// Verify a signed receipt
///
/// Checks if the content CID in the receipt matches the actual content.
/// In a real implementation, this would also verify cryptographic signatures.
pub fn verify(receipt: &SignedReceipt, content: Vec<u8>) -> Result<bool> {
    // Hash the content
    let content_hash = blake3::hash(&content);
    let content_cid = BASE64.encode(content_hash.as_bytes());
    
    // Check if CID matches
    Ok(content_cid == receipt.signed_content.content_cid)
}

/// Verify a signed JSON document
pub fn verify_json(receipt: &ReciboCard) -> Result<bool> {
    // Extract content from receipt
    let content = receipt.body.get("content")
        .ok_or_else(|| RhoError::Validate("Missing content field".to_string()))?;
    
    // Re-normalize the content
    let normalized = normalize(content.clone())?;
    
    // Get the CID from receipt
    let stored_cid = receipt.body.get("content_cid")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RhoError::Validate("Missing content_cid field".to_string()))?;
    
    // Compare CIDs
    Ok(normalized.cid == stored_cid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sign_content() {
        let cas = Cas::new();
        let content = b"Breaking News: Rho Circles launches three new products!";
        
        let sig = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "newsroom_key".to_string(),
            signature: "newsroom_sig".to_string(),
        };
        
        let result = sign_content(
            "article".to_string(),
            "New Products Launch".to_string(),
            "Tech Reporter".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            content.to_vec(),
            vec![sig],
            Some(json!({"category": "technology", "language": "en"})),
            &cas,
        );
        
        assert!(result.is_ok());
        let receipt = result.unwrap();
        assert_eq!(receipt.signed_content.content_type, "article");
        assert_eq!(receipt.signed_content.author, "Tech Reporter");
        assert_eq!(receipt.receipt_card.recibo.signatures.len(), 1);
    }

    #[test]
    fn test_sign_json() {
        let content = json!({
            "headline": "Important Announcement",
            "body": "This is verified content"
        });
        
        let sig = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "publisher_key".to_string(),
            signature: "publisher_sig".to_string(),
        };
        
        let result = sign_json(
            content,
            "Publisher Inc".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            vec![sig],
        );
        
        assert!(result.is_ok());
        let receipt = result.unwrap();
        assert_eq!(receipt.body["author"], "Publisher Inc");
        assert!(!receipt.recibo.content_cid.is_empty());
    }

    #[test]
    fn test_verify_content() {
        let cas = Cas::new();
        let content = b"Test content for verification";
        
        let receipt = sign_content(
            "document".to_string(),
            "Test Doc".to_string(),
            "Author".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            content.to_vec(),
            vec![],
            None,
            &cas,
        ).unwrap();
        
        let is_valid = verify(&receipt, content.to_vec()).unwrap();
        assert!(is_valid);
        
        // Test with tampered content
        let tampered = b"Tampered content";
        let is_valid_tampered = verify(&receipt, tampered.to_vec()).unwrap();
        assert!(!is_valid_tampered);
    }

    #[test]
    fn test_verify_json() {
        let content = json!({"data": "test"});
        let receipt = sign_json(
            content,
            "Author".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            vec![],
        ).unwrap();
        
        let is_valid = verify_json(&receipt).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_sign_content_deterministic() {
        let cas = Cas::new();
        let content = b"Deterministic test content";
        
        let receipt1 = sign_content(
            "test".to_string(),
            "Title".to_string(),
            "Author".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            content.to_vec(),
            vec![],
            None,
            &cas,
        ).unwrap();
        
        let receipt2 = sign_content(
            "test".to_string(),
            "Title".to_string(),
            "Author".to_string(),
            "2024-01-01T12:00:00Z".to_string(),
            content.to_vec(),
            vec![],
            None,
            &cas,
        ).unwrap();
        
        assert_eq!(
            receipt1.receipt_card.recibo.content_cid,
            receipt2.receipt_card.recibo.content_cid
        );
    }
}
