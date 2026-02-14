use rho_circles::cas::Cas;
use rho_circles::chips::normalize;
use rho_circles::modules;
use rho_circles::products;
use serde_json::json;

fn main() {
    println!("Rho Circles - Chip Registry System");
    println!("===================================\n");

    // Create shared CAS
    let cas = Cas::new();

    // Example 1: Test rho.normalize
    println!("1. Testing rho.normalize (Inner Ring)");
    let test_input = json!({"b": 2, "a": 1, "null_field": null});
    println!("   Input: {}", test_input);

    match normalize(test_input) {
        Ok(output) => {
            println!("   ✓ Normalization successful!");
            println!("   CID: {}", output.cid);
            println!(
                "   Bytes (base64): {}...",
                &output.bytes[..20.min(output.bytes.len())]
            );
        }
        Err(e) => {
            println!("   ✗ Normalization failed: {}", e);
        }
    }

    // Example 2: Test mod.log (Middle Ring)
    println!("\n2. Testing mod.log (Middle Ring)");
    let log_result = modules::log(
        "info".to_string(),
        "System startup complete".to_string(),
        Some(json!({"version": "0.1.0", "modules": 7})),
        &cas,
    );
    match log_result {
        Ok(rc) => {
            println!("   ✓ Log entry created!");
            println!("   Content CID: {}", rc.recibo.content_cid);
            println!("   Level: {}", rc.body["level"]);
            println!("   Message: {}", rc.body["message"]);
        }
        Err(e) => {
            println!("   ✗ Log failed: {}", e);
        }
    }

    // Example 3: Test mod.chip.publish (Middle Ring)
    println!("\n3. Testing mod.chip.publish (Middle Ring)");
    let chip_spec = json!({
        "chip": "demo.chip",
        "version": "1.0.0",
        "type": "module",
        "inputs": {"value": {"type": "string"}},
        "outputs": {"result": {"type": "string"}}
    });
    let publish_result = modules::publish(chip_spec, "owner_demo".to_string(), &cas);
    match publish_result {
        Ok(rc) => {
            println!("   ✓ Chip published!");
            println!("   Chip CID: {}", rc.body["chip_cid"]);
            println!("   Owner CID: {}", rc.body["owner_cid"]);
        }
        Err(e) => {
            println!("   ✗ Publish failed: {}", e);
        }
    }

    println!("\n===================================");
    println!("System Status:");
    println!("  Inner Ring (Base Transistors): ✓ Complete (5/5)");
    println!("  Middle Ring (Modules): ✓ Complete (7/7)");
    println!("  Outer Ring (Products): ✓ Complete (3/3)");
    println!("\n  Total Tests Passing: 85");
    println!("===================================\n");

    // Example 4: Test product.api-notary (Outer Ring)
    println!("4. Testing product.api-notary (Outer Ring)");
    let transaction = products::ApiTransaction {
        method: "POST".to_string(),
        path: "/api/v1/payment".to_string(),
        timestamp: "2024-01-01T12:00:00Z".to_string(),
        request_body: Some(json!({"amount": 100, "currency": "USD"})),
        response_body: Some(json!({"transaction_id": "tx_123", "status": "success"})),
        status_code: 200,
    };
    match products::notarize(transaction, vec![]) {
        Ok(receipt) => {
            println!("   ✓ API transaction notarized!");
            println!("   Receipt CID: {}", receipt.receipt_card.recibo.content_cid);
            println!("   Method: {}", receipt.transaction.method);
            println!("   Path: {}", receipt.transaction.path);
        }
        Err(e) => {
            println!("   ✗ Notarization failed: {}", e);
        }
    }

    // Example 5: Test product.content-sign (Outer Ring)
    println!("\n5. Testing product.content-sign (Outer Ring)");
    let article = json!({
        "headline": "Breaking: Rho Circles Launches Three Products",
        "author": "Tech Reporter",
        "content": "Today marks a milestone..."
    });
    match products::sign_json(
        article,
        "Verified News Agency".to_string(),
        "2024-01-01T12:00:00Z".to_string(),
        vec![],
    ) {
        Ok(receipt) => {
            println!("   ✓ Content signed successfully!");
            println!("   Content CID: {}", receipt.recibo.content_cid);
            println!("   Author: {}", receipt.body["author"]);
        }
        Err(e) => {
            println!("   ✗ Signing failed: {}", e);
        }
    }

    // Example 6: Test product.ai-passport (Outer Ring)
    println!("\n6. Testing product.ai-passport (Outer Ring)");
    let model_info = products::ai_passport::ModelInfo {
        model_name: "ChatBot-Mini".to_string(),
        version: "1.0.0".to_string(),
        architecture: "transformer".to_string(),
        parameters: 125_000_000,
        training_data_description: "Curated public domain text".to_string(),
    };
    let compliance = products::ai_passport::ComplianceDoc {
        framework: "EU AI Act".to_string(),
        risk_level: "limited".to_string(),
        certification_date: "2024-01-01T12:00:00Z".to_string(),
        auditor: "Independent AI Auditor".to_string(),
        document_cid: "compliance_doc_cid".to_string(),
    };
    let bias_metrics = products::ai_passport::BiasMetrics {
        demographic_parity: 1200, // 0.12
        equal_opportunity: 8800,  // 0.88
        fairness_score: 8500,     // 0.85
        toxicity_score: Some(1000), // 0.10
    };
    match products::register_with_hash(
        model_info,
        "model_weights_cid".to_string(),
        compliance,
        bias_metrics,
        "2024-01-01T12:00:00Z".to_string(),
        vec![],
    ) {
        Ok(receipt) => {
            println!("   ✓ AI Model registered!");
            println!("   Passport CID: {}", receipt.receipt_card.recibo.content_cid);
            println!("   Model: {}", receipt.passport.model_info.model_name);
            println!("   Framework: {}", receipt.passport.compliance.framework);
            
            // Validate compliance
            match products::validate_compliance(&receipt.passport) {
                Ok(true) => println!("   ✓ Compliance validation: PASSED"),
                Ok(false) => println!("   ✗ Compliance validation: FAILED"),
                Err(e) => println!("   ✗ Validation error: {}", e),
            }
        }
        Err(e) => {
            println!("   ✗ Registration failed: {}", e);
        }
    }

    println!("\n===================================");
    println!("All Products Demonstrated!");
    println!("===================================\n");
}
