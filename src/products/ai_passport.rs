/// Product: AI Passport (Lite)
///
/// Simple registry where developers upload AI model hash + compliance PDF,
/// generating a unified RC for EU AI Act compliance and model certification.
///
/// Use case: AI model passport, governance, regulatory compliance

use crate::cas::Cas;
use crate::chips::normalize;
use crate::rc;
use crate::types::{ReciboCard, Signature};
use crate::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// AI Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_name: String,
    pub version: String,
    pub architecture: String, // "transformer", "cnn", "rnn", etc.
    pub parameters: u64, // number of parameters
    pub training_data_description: String,
}

/// Compliance documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDoc {
    pub framework: String, // "EU AI Act", "NIST AI RMF", etc.
    pub risk_level: String, // "minimal", "limited", "high", "unacceptable"
    pub certification_date: String,
    pub auditor: String,
    pub document_cid: String, // CID of the compliance PDF
}

/// Bias and fairness metrics
/// All metrics are represented as integers (0-10000) to maintain determinism
/// Divide by 10000 to get the actual decimal value (e.g., 1500 = 0.15)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasMetrics {
    pub demographic_parity: i64, // 0-10000 (e.g., 1500 = 0.15 = 15%)
    pub equal_opportunity: i64, // 0-10000 (e.g., 8500 = 0.85 = 85%)
    pub fairness_score: i64, // 0-10000 (e.g., 8200 = 0.82 = 82%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity_score: Option<i64>, // 0-10000 (e.g., 1200 = 0.12 = 12%)
}

/// AI Model Passport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPassport {
    pub model_info: ModelInfo,
    pub model_weights_cid: String, // CID of model weights
    pub compliance: ComplianceDoc,
    pub bias_metrics: BiasMetrics,
    pub registration_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<Value>,
}

/// Passport registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportReceipt {
    pub passport: AiPassport,
    pub receipt_card: ReciboCard,
}

/// Register an AI model and generate a passport
///
/// Creates an immutable record proving the model's compliance status,
/// bias metrics, and training data characteristics.
pub fn register_model(
    model_info: ModelInfo,
    model_weights: Vec<u8>,
    compliance_framework: String,
    compliance_risk_level: String,
    compliance_auditor: String,
    compliance_pdf: Vec<u8>,
    bias_metrics: BiasMetrics,
    registration_timestamp: String,
    additional_metadata: Option<Value>,
    signatures: Vec<Signature>,
    cas: &Cas,
) -> Result<PassportReceipt> {
    // Hash and store model weights
    let weights_hash = blake3::hash(&model_weights);
    let model_weights_cid = BASE64.encode(weights_hash.as_bytes());
    cas.put(model_weights.clone())?;
    
    // Hash and store compliance PDF
    let pdf_hash = blake3::hash(&compliance_pdf);
    let document_cid = BASE64.encode(pdf_hash.as_bytes());
    cas.put(compliance_pdf.clone())?;
    
    // Create compliance doc with provided parameters
    let compliance = ComplianceDoc {
        framework: compliance_framework,
        risk_level: compliance_risk_level,
        certification_date: registration_timestamp.clone(),
        auditor: compliance_auditor,
        document_cid,
    };
    
    // Create passport
    let passport = AiPassport {
        model_info,
        model_weights_cid,
        compliance,
        bias_metrics,
        registration_timestamp,
        additional_metadata,
    };
    
    // Emit receipt card (normalization happens inside emit_with_signatures)
    let passport_value = serde_json::to_value(&passport)?;
    let receipt_card = rc::emit_with_signatures(passport_value, signatures)?;
    
    Ok(PassportReceipt {
        passport,
        receipt_card,
    })
}

/// Quick registration for existing models (hash-only)
///
/// Simplified version where you already have the hash of model weights
/// and don't need to upload the full weights.
pub fn register_with_hash(
    model_info: ModelInfo,
    model_weights_cid: String,
    compliance: ComplianceDoc,
    bias_metrics: BiasMetrics,
    registration_timestamp: String,
    signatures: Vec<Signature>,
) -> Result<PassportReceipt> {
    let passport = AiPassport {
        model_info,
        model_weights_cid,
        compliance,
        bias_metrics,
        registration_timestamp,
        additional_metadata: None,
    };
    
    // Emit receipt card (normalization happens inside emit_with_signatures)
    let passport_value = serde_json::to_value(&passport)?;
    let receipt_card = rc::emit_with_signatures(passport_value, signatures)?;
    
    Ok(PassportReceipt {
        passport,
        receipt_card,
    })
}

/// Verify a passport's integrity
///
/// Checks if the receipt's CID matches the passport data.
/// In production, would also verify cryptographic signatures and audit trail.
pub fn verify_passport(receipt: &PassportReceipt) -> Result<bool> {
    // Re-normalize the passport
    let passport_value = serde_json::to_value(&receipt.passport)?;
    let normalized = normalize(passport_value)?;
    
    // Check if CID matches
    Ok(normalized.cid == receipt.receipt_card.recibo.content_cid)
}

/// Check if model passes compliance requirements
///
/// Validates bias metrics against thresholds and compliance framework.
/// Metrics are represented as integers (0-10000) where 10000 = 1.0 = 100%
pub fn validate_compliance(passport: &AiPassport) -> Result<bool> {
    // Check bias metrics thresholds
    // fairness_score >= 0.7 (7000/10000)
    // demographic_parity <= 0.2 (2000/10000)
    // equal_opportunity >= 0.8 (8000/10000)
    let bias_ok = passport.bias_metrics.fairness_score >= 7000
        && passport.bias_metrics.demographic_parity <= 2000
        && passport.bias_metrics.equal_opportunity >= 8000;
    
    // Check toxicity if present (should be <= 0.3 = 3000/10000)
    let toxicity_ok = match passport.bias_metrics.toxicity_score {
        Some(score) => score <= 3000,
        None => true, // Optional metric
    };
    
    // Verify compliance framework is recognized
    let framework_ok = matches!(
        passport.compliance.framework.as_str(),
        "EU AI Act" | "NIST AI RMF" | "ISO 42001"
    );
    
    // Check risk level is acceptable
    let risk_ok = passport.compliance.risk_level != "unacceptable";
    
    Ok(bias_ok && toxicity_ok && framework_ok && risk_ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_register_model() {
        let cas = Cas::new();
        
        let model_info = ModelInfo {
            model_name: "GPT-Mini".to_string(),
            version: "1.0.0".to_string(),
            architecture: "transformer".to_string(),
            parameters: 125_000_000,
            training_data_description: "Public domain text corpus".to_string(),
        };
        
        let model_weights = b"mock_model_weights_data".to_vec();
        let compliance_pdf = b"mock_compliance_pdf_content".to_vec();
        
        let bias_metrics = BiasMetrics {
            demographic_parity: 1500, // 0.15
            equal_opportunity: 8500, // 0.85
            fairness_score: 8200, // 0.82
            toxicity_score: Some(1200), // 0.12
        };
        
        let sig = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "auditor_key".to_string(),
            signature: "auditor_sig".to_string(),
        };
        
        let result = register_model(
            model_info,
            model_weights,
            "EU AI Act".to_string(),
            "limited".to_string(),
            "Independent Auditor".to_string(),
            compliance_pdf,
            bias_metrics,
            "2024-01-01T12:00:00Z".to_string(),
            Some(json!({"purpose": "chatbot", "domain": "customer_service"})),
            vec![sig],
            &cas,
        );
        
        assert!(result.is_ok());
        let receipt = result.unwrap();
        assert_eq!(receipt.passport.model_info.model_name, "GPT-Mini");
        assert!(!receipt.passport.model_weights_cid.is_empty());
        assert_eq!(receipt.receipt_card.recibo.signatures.len(), 1);
    }

    #[test]
    fn test_register_with_hash() {
        let model_info = ModelInfo {
            model_name: "TestModel".to_string(),
            version: "2.0.0".to_string(),
            architecture: "cnn".to_string(),
            parameters: 50_000_000,
            training_data_description: "ImageNet subset".to_string(),
        };
        
        let compliance = ComplianceDoc {
            framework: "EU AI Act".to_string(),
            risk_level: "limited".to_string(),
            certification_date: "2024-01-01T12:00:00Z".to_string(),
            auditor: "AI Safety Lab".to_string(),
            document_cid: "mock_pdf_cid".to_string(),
        };
        
        let bias_metrics = BiasMetrics {
            demographic_parity: 1000, // 0.10
            equal_opportunity: 9000, // 0.90
            fairness_score: 8800, // 0.88
            toxicity_score: None,
        };
        
        let result = register_with_hash(
            model_info,
            "mock_weights_cid".to_string(),
            compliance,
            bias_metrics,
            "2024-01-01T12:00:00Z".to_string(),
            vec![],
        );
        
        assert!(result.is_ok());
        let receipt = result.unwrap();
        assert_eq!(receipt.passport.model_weights_cid, "mock_weights_cid");
    }

    #[test]
    fn test_verify_passport() {
        let model_info = ModelInfo {
            model_name: "VerifyTest".to_string(),
            version: "1.0.0".to_string(),
            architecture: "transformer".to_string(),
            parameters: 1_000_000,
            training_data_description: "Test data".to_string(),
        };
        
        let compliance = ComplianceDoc {
            framework: "EU AI Act".to_string(),
            risk_level: "minimal".to_string(),
            certification_date: "2024-01-01T12:00:00Z".to_string(),
            auditor: "Test Auditor".to_string(),
            document_cid: "test_cid".to_string(),
        };
        
        let bias_metrics = BiasMetrics {
            demographic_parity: 500, // 0.05
            equal_opportunity: 9500, // 0.95
            fairness_score: 9200, // 0.92
            toxicity_score: Some(800), // 0.08
        };
        
        let receipt = register_with_hash(
            model_info,
            "test_weights_cid".to_string(),
            compliance,
            bias_metrics,
            "2024-01-01T12:00:00Z".to_string(),
            vec![],
        ).unwrap();
        
        let is_valid = verify_passport(&receipt).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_validate_compliance() {
        // Test passing compliance
        let good_passport = AiPassport {
            model_info: ModelInfo {
                model_name: "GoodModel".to_string(),
                version: "1.0.0".to_string(),
                architecture: "transformer".to_string(),
                parameters: 1_000_000,
                training_data_description: "Test".to_string(),
            },
            model_weights_cid: "cid".to_string(),
            compliance: ComplianceDoc {
                framework: "EU AI Act".to_string(),
                risk_level: "minimal".to_string(),
                certification_date: "2024-01-01T12:00:00Z".to_string(),
                auditor: "Auditor".to_string(),
                document_cid: "doc_cid".to_string(),
            },
            bias_metrics: BiasMetrics {
                demographic_parity: 1500, // 0.15
                equal_opportunity: 8500, // 0.85
                fairness_score: 8000, // 0.80
                toxicity_score: Some(2000), // 0.20
            },
            registration_timestamp: "2024-01-01T12:00:00Z".to_string(),
            additional_metadata: None,
        };
        
        assert!(validate_compliance(&good_passport).unwrap());
        
        // Test failing compliance (high toxicity)
        let bad_passport = AiPassport {
            model_info: ModelInfo {
                model_name: "BadModel".to_string(),
                version: "1.0.0".to_string(),
                architecture: "transformer".to_string(),
                parameters: 1_000_000,
                training_data_description: "Test".to_string(),
            },
            model_weights_cid: "cid".to_string(),
            compliance: ComplianceDoc {
                framework: "EU AI Act".to_string(),
                risk_level: "high".to_string(),
                certification_date: "2024-01-01T12:00:00Z".to_string(),
                auditor: "Auditor".to_string(),
                document_cid: "doc_cid".to_string(),
            },
            bias_metrics: BiasMetrics {
                demographic_parity: 1500, // 0.15
                equal_opportunity: 8500, // 0.85
                fairness_score: 8000, // 0.80
                toxicity_score: Some(8000), // 0.80 - Too high!
            },
            registration_timestamp: "2024-01-01T12:00:00Z".to_string(),
            additional_metadata: None,
        };
        
        assert!(!validate_compliance(&bad_passport).unwrap());
    }

    #[test]
    fn test_register_deterministic() {
        let model_info = ModelInfo {
            model_name: "DeterministicTest".to_string(),
            version: "1.0.0".to_string(),
            architecture: "transformer".to_string(),
            parameters: 1_000_000,
            training_data_description: "Test".to_string(),
        };
        
        let compliance = ComplianceDoc {
            framework: "EU AI Act".to_string(),
            risk_level: "minimal".to_string(),
            certification_date: "2024-01-01T12:00:00Z".to_string(),
            auditor: "Test Auditor".to_string(),
            document_cid: "test_cid".to_string(),
        };
        
        let bias_metrics = BiasMetrics {
            demographic_parity: 1000, // 0.10
            equal_opportunity: 9000, // 0.90
            fairness_score: 8500, // 0.85
            toxicity_score: Some(1500), // 0.15
        };
        
        let receipt1 = register_with_hash(
            model_info.clone(),
            "test_cid".to_string(),
            compliance.clone(),
            bias_metrics.clone(),
            "2024-01-01T12:00:00Z".to_string(),
            vec![],
        ).unwrap();
        
        let receipt2 = register_with_hash(
            model_info,
            "test_cid".to_string(),
            compliance,
            bias_metrics,
            "2024-01-01T12:00:00Z".to_string(),
            vec![],
        ).unwrap();
        
        assert_eq!(
            receipt1.receipt_card.recibo.content_cid,
            receipt2.receipt_card.recibo.content_cid
        );
    }
}
