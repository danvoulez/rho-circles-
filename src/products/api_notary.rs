/// Product: API Notary
///
/// HTTP wrapper around rho.normalize + rc.emit that generates signed receipts
/// for B2B API data exchanges. Eliminates disputes with cryptographic proof.
///
/// Use case: Sidecar for APIs that need to prove "I sent this" or "I received this"
use crate::chips::normalize;
use crate::rc;
use crate::types::{ReciboCard, Signature};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// API Request/Response pair for notarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTransaction {
    pub method: String,
    pub path: String,
    pub timestamp: String, // ISO 8601 format
    pub request_body: Option<Value>,
    pub response_body: Option<Value>,
    pub status_code: u16,
}

/// Notarization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaryReceipt {
    pub transaction: ApiTransaction,
    pub receipt_card: ReciboCard,
}

/// Notarize an API transaction
///
/// Creates a cryptographic receipt for an API request/response pair.
/// Both parties can sign to create proof of agreement on what was exchanged.
pub fn notarize(transaction: ApiTransaction, signatures: Vec<Signature>) -> Result<NotaryReceipt> {
    // Convert transaction to value and emit receipt card
    // Note: emit_with_signatures will normalize internally
    let transaction_value = serde_json::to_value(&transaction)?;
    let receipt_card = rc::emit_with_signatures(transaction_value, signatures)?;

    Ok(NotaryReceipt {
        transaction,
        receipt_card,
    })
}

/// Verify a notary receipt
///
/// Verifies that the receipt's CID matches the transaction content.
/// In a real implementation, this would also verify signatures.
pub fn verify(receipt: &NotaryReceipt) -> Result<bool> {
    // Re-normalize the transaction
    let transaction_value = serde_json::to_value(&receipt.transaction)?;
    let normalized = normalize(transaction_value)?;

    // Check if CID matches
    Ok(normalized.cid == receipt.receipt_card.recibo.content_cid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_notarize_api_transaction() {
        let transaction = ApiTransaction {
            method: "POST".to_string(),
            path: "/api/v1/orders".to_string(),
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            request_body: Some(json!({"item": "widget", "quantity": 5})),
            response_body: Some(json!({"order_id": "12345", "status": "confirmed"})),
            status_code: 200,
        };

        let result = notarize(transaction.clone(), vec![]);
        assert!(result.is_ok());

        let receipt = result.unwrap();
        assert_eq!(receipt.transaction.method, "POST");
        assert!(!receipt.receipt_card.recibo.content_cid.is_empty());
    }

    #[test]
    fn test_notarize_with_signatures() {
        let transaction = ApiTransaction {
            method: "GET".to_string(),
            path: "/api/v1/data".to_string(),
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            request_body: None,
            response_body: Some(json!({"data": "sensitive_info"})),
            status_code: 200,
        };

        let sig1 = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "party_a_key".to_string(),
            signature: "party_a_sig".to_string(),
        };

        let sig2 = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "party_b_key".to_string(),
            signature: "party_b_sig".to_string(),
        };

        let result = notarize(transaction, vec![sig1, sig2]);
        assert!(result.is_ok());

        let receipt = result.unwrap();
        assert_eq!(receipt.receipt_card.recibo.signatures.len(), 2);
    }

    #[test]
    fn test_verify_receipt() {
        let transaction = ApiTransaction {
            method: "PUT".to_string(),
            path: "/api/v1/update".to_string(),
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            request_body: Some(json!({"key": "value"})),
            response_body: Some(json!({"success": true})),
            status_code: 200,
        };

        let receipt = notarize(transaction, vec![]).unwrap();
        let is_valid = verify(&receipt).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_notarize_deterministic() {
        let transaction = ApiTransaction {
            method: "POST".to_string(),
            path: "/api/test".to_string(),
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            request_body: Some(json!({"b": 2, "a": 1})),
            response_body: Some(json!({"status": "ok"})),
            status_code: 200,
        };

        let receipt1 = notarize(transaction.clone(), vec![]).unwrap();
        let receipt2 = notarize(transaction, vec![]).unwrap();

        assert_eq!(
            receipt1.receipt_card.recibo.content_cid,
            receipt2.receipt_card.recibo.content_cid
        );
    }
}
