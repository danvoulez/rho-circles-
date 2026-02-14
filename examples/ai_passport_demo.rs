// Example: AI Passport Product
// 
// Demonstrates how to use product.ai-passport to register AI models
// with compliance certification and bias metrics.

use rho_circles::cas::Cas;
use rho_circles::products::ai_passport::{
    register_model, register_with_hash, validate_compliance, verify_passport,
    BiasMetrics, ComplianceDoc, ModelInfo,
};
use rho_circles::types::Signature;
use serde_json::json;

fn main() {
    println!("=== Product: AI Passport Demo ===\n");

    let cas = Cas::new();

    // Example 1: Register a new AI model with full compliance
    println!("Example 1: Registering a chatbot AI model\n");

    let model_info = ModelInfo {
        model_name: "CustomerServiceBot-v2".to_string(),
        version: "2.1.0".to_string(),
        architecture: "transformer".to_string(),
        parameters: 350_000_000,
        training_data_description: "Curated customer service conversations (2020-2023), filtered for PII and toxicity".to_string(),
    };

    // Bias metrics use integer scale 0-10000 (divide by 10000 for decimal)
    // Example: 1500 = 0.15 = 15%
    let bias_metrics = BiasMetrics {
        demographic_parity: 800,     // 0.08 (8%) - Low bias across demographics
        equal_opportunity: 9200,     // 0.92 (92%) - High equal opportunity
        fairness_score: 8900,        // 0.89 (89%) - Strong fairness
        toxicity_score: Some(500),   // 0.05 (5%) - Very low toxicity
    };

    // Mock model weights and compliance PDF
    let model_weights = b"[Model weights binary data...]".to_vec();
    let compliance_pdf = b"[EU AI Act compliance documentation PDF...]".to_vec();

    let auditor_signature = Signature {
        algorithm: "mldsa3".to_string(), // Post-quantum signature
        public_key: "independent_auditor_pq_key".to_string(),
        signature: "auditor_pq_signature".to_string(),
    };

    match register_model(
        model_info.clone(),
        model_weights,
        "EU AI Act".to_string(),
        "limited".to_string(),
        "Independent Auditor".to_string(),
        compliance_pdf,
        bias_metrics.clone(),
        "2024-01-15T09:00:00Z".to_string(),
        Some(json!({
            "intended_use": "Customer service automation",
            "deployment_region": "EU",
            "risk_category": "limited",
            "monitoring_enabled": true
        })),
        vec![auditor_signature],
        &cas,
    ) {
        Ok(passport) => {
            println!("✓ AI Model registered successfully!");
            println!("\nPassport details:");
            println!("  Model: {} v{}", passport.passport.model_info.model_name, passport.passport.model_info.version);
            println!("  Parameters: {}", passport.passport.model_info.parameters);
            println!("  Architecture: {}", passport.passport.model_info.architecture);
            println!("\nCompliance:");
            println!("  Framework: {}", passport.passport.compliance.framework);
            println!("  Risk Level: {}", passport.passport.compliance.risk_level);
            println!("  Auditor: {}", passport.passport.compliance.auditor);
            println!("\nBias Metrics:");
            println!("  Demographic Parity: {} ({}%)", passport.passport.bias_metrics.demographic_parity, 
                passport.passport.bias_metrics.demographic_parity as f64 / 100.0);
            println!("  Equal Opportunity: {} ({}%)", passport.passport.bias_metrics.equal_opportunity,
                passport.passport.bias_metrics.equal_opportunity as f64 / 100.0);
            println!("  Fairness Score: {} ({}%)", passport.passport.bias_metrics.fairness_score,
                passport.passport.bias_metrics.fairness_score as f64 / 100.0);
            println!("  Toxicity Score: {} ({}%)", 
                passport.passport.bias_metrics.toxicity_score.unwrap_or(0),
                passport.passport.bias_metrics.toxicity_score.unwrap_or(0) as f64 / 100.0);

            // Verify passport integrity
            match verify_passport(&passport) {
                Ok(true) => println!("\n✓ Passport verification: PASSED"),
                Ok(false) => println!("\n✗ Passport verification: FAILED"),
                Err(e) => println!("\n✗ Verification error: {}", e),
            }

            // Validate compliance
            match validate_compliance(&passport.passport) {
                Ok(true) => println!("✓ Compliance validation: PASSED"),
                Ok(false) => println!("✗ Compliance validation: FAILED"),
                Err(e) => println!("✗ Validation error: {}", e),
            }

            println!("\nPassport CID: {}", passport.receipt_card.recibo.content_cid);
        }
        Err(e) => {
            println!("✗ Registration failed: {}", e);
        }
    }

    // Example 2: Register with pre-computed hash (more common)
    println!("\n\nExample 2: Quick registration with existing hash\n");

    let model_info_2 = ModelInfo {
        model_name: "ContentModerator-ML".to_string(),
        version: "1.5.2".to_string(),
        architecture: "cnn".to_string(),
        parameters: 75_000_000,
        training_data_description: "Labeled content moderation dataset (public sources)".to_string(),
    };

    let compliance_2 = ComplianceDoc {
        framework: "EU AI Act".to_string(),
        risk_level: "high".to_string(), // Content moderation is high-risk per EU AI Act Annex III
        certification_date: "2024-01-10T00:00:00Z".to_string(),
        auditor: "AI Safety Institute Europe".to_string(),
        document_cid: "Q2FzaGVkX19jb21wbGlhbmNlX2RvY19oYXNo".to_string(), // Pre-computed
    };

    let bias_metrics_2 = BiasMetrics {
        demographic_parity: 1800,    // 0.18 (18%)
        equal_opportunity: 8500,     // 0.85 (85%)
        fairness_score: 7500,        // 0.75 (75%)
        toxicity_score: Some(2500),  // 0.25 (25%) - Higher for content moderation
    };

    match register_with_hash(
        model_info_2,
        "Q2FzaGVkX19tb2RlbF93ZWlnaHRzX2hhc2g=".to_string(), // Pre-computed hash
        compliance_2,
        bias_metrics_2,
        "2024-01-15T10:00:00Z".to_string(),
        vec![],
    ) {
        Ok(passport) => {
            println!("✓ Model registered with hash!");
            println!("  Model: {}", passport.passport.model_info.model_name);
            println!("  Risk Level: {}", passport.passport.compliance.risk_level);
            
            match validate_compliance(&passport.passport) {
                Ok(true) => println!("  ✓ Compliance: PASSED"),
                Ok(false) => println!("  ✗ Compliance: FAILED (metrics out of threshold)"),
                Err(e) => println!("  ✗ Validation error: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Registration failed: {}", e);
        }
    }

    println!("\n\nBenefits of AI Passport:");
    println!("  • Satisfies EU AI Act requirements");
    println!("  • Immutable proof of model characteristics");
    println!("  • Transparent bias and fairness metrics");
    println!("  • Auditable training data lineage");
    println!("  • Post-quantum cryptographic signatures");
    println!("  • Deterministic compliance validation");

    println!("\n=== Demo Complete ===");
}
