use crate::chips::normalize;
use crate::types::{Recibo, ReciboCard, Signature};
use crate::Result;
use serde_json::Value;

/// RC (Recibo) emission
///
/// Builds and emits a Recibo Card with normalized body and computed CID
pub fn emit(body: Value) -> Result<ReciboCard> {
    emit_with_signatures(body, vec![])
}

/// Emit RC with signatures
///
/// Creates a Recibo Card with the given body and optional signatures.
/// The body is normalized and its CID is computed.
pub fn emit_with_signatures(body: Value, signatures: Vec<Signature>) -> Result<ReciboCard> {
    // Normalize the body to get the content CID
    let normalized = normalize(body.clone())?;

    let recibo = Recibo {
        content_cid: normalized.cid,
        signatures,
    };

    Ok(ReciboCard { body, recibo })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_emit_placeholder() {
        let body = json!({"test": "value"});
        let result = emit(body.clone()).unwrap();
        assert_eq!(result.body, body);
        assert!(!result.recibo.content_cid.is_empty());
        assert!(result.recibo.signatures.is_empty());
    }

    #[test]
    fn test_emit_with_signatures() {
        let body = json!({"test": "value"});
        let sig = Signature {
            algorithm: "ed25519".to_string(),
            public_key: "test_key".to_string(),
            signature: "test_sig".to_string(),
        };
        let result = emit_with_signatures(body.clone(), vec![sig.clone()]).unwrap();
        assert_eq!(result.body, body);
        assert_eq!(result.recibo.signatures.len(), 1);
        assert_eq!(result.recibo.signatures[0].algorithm, "ed25519");
    }

    #[test]
    fn test_emit_deterministic() {
        let body = json!({"b": 2, "a": 1});
        let rc1 = emit(body.clone()).unwrap();
        let rc2 = emit(body.clone()).unwrap();
        assert_eq!(rc1.recibo.content_cid, rc2.recibo.content_cid);
    }
}
