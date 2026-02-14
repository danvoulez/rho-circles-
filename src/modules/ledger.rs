use crate::cas::Cas;
use crate::chips::normalize;
use crate::types::ReciboCard;
use crate::Result;
use base64::Engine;

/// mod.ledger.append: Append-only ledger for audit
///
/// Appends a Recibo Card to the ledger
///
/// Inputs:
/// - rc: ReciboCard to append
///
/// Output: Success boolean
pub fn append(rc: ReciboCard, cas: &Cas) -> Result<bool> {
    // Normalize the RC for storage
    let rc_value = serde_json::to_value(&rc)?;
    let normalized = normalize(rc_value)?;

    // Store in CAS
    let rc_bytes = base64::engine::general_purpose::STANDARD.decode(&normalized.bytes)?;
    let stored_cid = cas.put(rc_bytes)?;

    // In a real implementation, this would:
    // 1. Append to a Merkle tree or blockchain
    // 2. Update the ledger head pointer
    // 3. Emit a ledger event
    //
    // For now, we just verify storage succeeded
    Ok(stored_cid == normalized.cid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc;
    use serde_json::json;

    #[test]
    fn test_append_success() {
        let cas = Cas::new();
        let body = json!({"test": "data"});
        let rc = rc::emit(body).unwrap();
        let result = append(rc, &cas);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_append_deterministic() {
        let cas = Cas::new();
        let body = json!({"test": "data"});
        let rc1 = rc::emit(body.clone()).unwrap();
        let rc2 = rc::emit(body).unwrap();
        
        let result1 = append(rc1, &cas).unwrap();
        let result2 = append(rc2, &cas).unwrap();
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }
}
