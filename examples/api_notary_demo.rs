// Example: API Notary Product
//
// Demonstrates how to use product.api-notary to create cryptographic
// receipts for B2B API transactions.

use rho_circles::products::{notarize, verify_notary, ApiTransaction};
use rho_circles::types::Signature;
use serde_json::json;

fn main() {
    println!("=== Product: API Notary Demo ===\n");

    // Scenario: A payment API call between two companies
    println!("Scenario: Company A calls Company B's payment API\n");

    // Create the transaction
    let transaction = ApiTransaction {
        method: "POST".to_string(),
        path: "/api/v1/orders/create".to_string(),
        timestamp: "2024-01-15T14:30:00Z".to_string(),
        request_body: Some(json!({
            "order_id": "ORD-12345",
            "items": [
                {"product": "Widget A", "quantity": 10, "price": 25},
                {"product": "Widget B", "quantity": 5, "price": 50}
            ],
            "total": 500
        })),
        response_body: Some(json!({
            "order_id": "ORD-12345",
            "status": "confirmed",
            "tracking_number": "TRK-98765",
            "estimated_delivery": "2024-01-20"
        })),
        status_code: 201,
    };

    println!("Transaction details:");
    println!("  Method: {}", transaction.method);
    println!("  Path: {}", transaction.path);
    println!("  Status: {}", transaction.status_code);

    // In a real implementation, both parties would sign
    let company_a_signature = Signature {
        algorithm: "ed25519".to_string(),
        public_key: "company_a_public_key_base64".to_string(),
        signature: "company_a_signature_base64".to_string(),
    };

    let company_b_signature = Signature {
        algorithm: "ed25519".to_string(),
        public_key: "company_b_public_key_base64".to_string(),
        signature: "company_b_signature_base64".to_string(),
    };

    let signatures = vec![company_a_signature, company_b_signature];

    // Notarize the transaction
    match notarize(transaction.clone(), signatures) {
        Ok(receipt) => {
            println!("\n✓ Transaction notarized successfully!");
            println!("\nReceipt details:");
            println!("  Content CID: {}", receipt.receipt_card.recibo.content_cid);
            println!(
                "  Signatures: {}",
                receipt.receipt_card.recibo.signatures.len()
            );
            println!(
                "  Order ID: {}",
                receipt
                    .transaction
                    .request_body
                    .as_ref()
                    .and_then(|b| b.get("order_id"))
                    .unwrap_or(&json!("N/A"))
            );

            // Verify the receipt
            match verify_notary(&receipt) {
                Ok(true) => println!("\n✓ Receipt verification: PASSED"),
                Ok(false) => println!("\n✗ Receipt verification: FAILED"),
                Err(e) => println!("\n✗ Verification error: {}", e),
            }

            println!("\nBenefits:");
            println!("  • Both parties have cryptographic proof of the transaction");
            println!("  • Receipt can be verified offline at any time");
            println!("  • Eliminates \"I didn't send that\" or \"I didn't receive that\" disputes");
            println!("  • Content-addressable: same transaction = same CID");
        }
        Err(e) => {
            println!("✗ Notarization failed: {}", e);
        }
    }

    println!("\n=== Demo Complete ===");
}
