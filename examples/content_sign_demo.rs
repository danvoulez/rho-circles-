// Example: Content Sign Product
// 
// Demonstrates how to use product.content-sign to sign content
// and generate verifiable receipts (anti-fake news).

use rho_circles::cas::Cas;
use rho_circles::products::{sign_content, sign_json, verify_content, verify_json};
use rho_circles::types::Signature;
use serde_json::json;

fn main() {
    println!("=== Product: Content Sign Demo ===\n");

    let cas = Cas::new();

    // Example 1: Sign a news article (JSON)
    println!("Example 1: Signing a news article\n");

    let article = json!({
        "headline": "Revolutionary Rho Circles Platform Launches",
        "author": "Jane Reporter",
        "publication": "Tech Daily",
        "date": "2024-01-15",
        "content": "A new deterministic computation platform called Rho Circles has been unveiled today, promising to revolutionize API security, content verification, and AI governance...",
        "tags": ["technology", "security", "blockchain-alternative"]
    });

    let publisher_signature = Signature {
        algorithm: "ed25519".to_string(),
        public_key: "tech_daily_publisher_key".to_string(),
        signature: "tech_daily_signature".to_string(),
    };

    match sign_json(
        article.clone(),
        "Tech Daily - Verified Publisher".to_string(),
        "2024-01-15T10:00:00Z".to_string(),
        vec![publisher_signature],
    ) {
        Ok(receipt) => {
            println!("✓ Article signed successfully!");
            println!("\nReceipt details:");
            println!("  Content CID: {}", receipt.recibo.content_cid);
            println!("  Author: {}", receipt.body["author"]);
            println!("  Headline: {}", receipt.body["content"]["headline"]);

            // Verify the signed article
            match verify_json(&receipt) {
                Ok(true) => println!("\n✓ Content verification: PASSED"),
                Ok(false) => println!("\n✗ Content verification: FAILED"),
                Err(e) => println!("\n✗ Verification error: {}", e),
            }

            println!("\nReaders can now:");
            println!("  • Verify the article came from Tech Daily");
            println!("  • Detect any tampering (CID will change)");
            println!("  • Check authenticity offline");
        }
        Err(e) => {
            println!("✗ Signing failed: {}", e);
        }
    }

    // Example 2: Sign binary content (like an image or PDF)
    println!("\n\nExample 2: Signing an image file\n");

    let image_content = b"[Binary image data would be here...]".to_vec();
    
    let photographer_signature = Signature {
        algorithm: "ed25519".to_string(),
        public_key: "photographer_public_key".to_string(),
        signature: "photographer_signature".to_string(),
    };

    match sign_content(
        "image".to_string(),
        "Press Conference Photo".to_string(),
        "Alice Photographer".to_string(),
        "2024-01-15T14:30:00Z".to_string(),
        image_content.clone(),
        vec![photographer_signature],
        Some(json!({
            "camera": "Canon EOS R5",
            "location": "Tech Conference 2024",
            "license": "CC BY-NC 4.0"
        })),
        &cas,
    ) {
        Ok(receipt) => {
            println!("✓ Image signed successfully!");
            println!("\nReceipt details:");
            println!("  Content CID: {}", receipt.signed_content.content_cid);
            println!("  Author: {}", receipt.signed_content.author);
            println!("  Title: {}", receipt.signed_content.title);
            println!("  License: {}", receipt.signed_content.metadata.as_ref()
                .and_then(|m| m.get("license"))
                .unwrap_or(&json!("N/A")));

            // Verify the signed content
            match verify_content(&receipt, image_content) {
                Ok(true) => println!("\n✓ Image verification: PASSED"),
                Ok(false) => println!("\n✗ Image verification: FAILED"),
                Err(e) => println!("\n✗ Verification error: {}", e),
            }

            println!("\nBenefits:");
            println!("  • Proves photographer ownership");
            println!("  • Detects AI-generated deepfakes");
            println!("  • Immutable attribution and licensing");
            println!("  • Combat misinformation with verifiable sources");
        }
        Err(e) => {
            println!("✗ Signing failed: {}", e);
        }
    }

    println!("\n=== Demo Complete ===");
}
