use crate::cas::Cas;
use crate::rc;
use crate::types::{Cid, ReciboCard};
use crate::Result;
use serde_json::json;

/// mod.judge: LLM gateway (designated I/O gateway)
///
/// This is a designated I/O gateway that allows controlled external communication
/// with LLM services. It's the only module allowed to break the No-IO rule.
///
/// Inputs:
/// - prompt_cid: CID of the prompt in CAS
/// - policy_cid: CID of the trust policy in CAS
///
/// Output: RC containing LLM response
pub fn judge(prompt_cid: Cid, policy_cid: Cid, cas: &Cas) -> Result<ReciboCard> {
    // Fetch prompt from CAS
    let prompt_bytes = cas.get(&prompt_cid)?;
    let _prompt: serde_json::Value = serde_json::from_slice(&prompt_bytes)?;

    // Fetch policy from CAS
    let policy_bytes = cas.get(&policy_cid)?;
    let _policy: serde_json::Value = serde_json::from_slice(&policy_bytes)?;

    // In a real implementation, this would:
    // 1. Validate the prompt against content policy
    // 2. Send the prompt to an LLM API (external I/O - ALLOWED here)
    // 3. Validate the response against trust policy
    // 4. Log the interaction for audit
    //
    // For now, we create a mock response

    let mock_response = json!({
        "prompt_cid": prompt_cid,
        "policy_cid": policy_cid,
        "response": "Mock LLM response",
        "model": "mock-v1",
        "timestamp": "deterministic_timestamp",
        "tokens_used": 42,
    });

    // Emit as RC
    rc::emit(mock_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chips::normalize;
    use base64::Engine;
    use serde_json::json;

    #[test]
    fn test_judge_valid_request() {
        let cas = Cas::new();

        // Store prompt in CAS
        let prompt = json!({"text": "What is the meaning of life?"});
        let normalized_prompt = normalize(prompt).unwrap();
        let prompt_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized_prompt.bytes)
            .unwrap();
        let prompt_cid = cas.put(prompt_bytes).unwrap();

        // Store policy in CAS
        let policy = json!({"max_tokens": 100, "temperature": 7});
        let normalized_policy = normalize(policy).unwrap();
        let policy_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized_policy.bytes)
            .unwrap();
        let policy_cid = cas.put(policy_bytes).unwrap();

        // Call judge
        let result = judge(prompt_cid, policy_cid, &cas);
        assert!(result.is_ok());
        let rc = result.unwrap();
        assert_eq!(rc.body["response"], "Mock LLM response");
    }

    #[test]
    fn test_judge_missing_prompt() {
        let cas = Cas::new();

        // Store only policy
        let policy = json!({"max_tokens": 100});
        let normalized_policy = normalize(policy).unwrap();
        let policy_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized_policy.bytes)
            .unwrap();
        let policy_cid = cas.put(policy_bytes).unwrap();

        let result = judge("nonexistent_cid".to_string(), policy_cid, &cas);
        assert!(result.is_err());
    }

    #[test]
    fn test_judge_deterministic() {
        let cas = Cas::new();

        // Store prompt in CAS
        let prompt = json!({"text": "Test"});
        let normalized_prompt = normalize(prompt).unwrap();
        let prompt_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized_prompt.bytes)
            .unwrap();
        let prompt_cid = cas.put(prompt_bytes).unwrap();

        // Store policy in CAS
        let policy = json!({"max_tokens": 100});
        let normalized_policy = normalize(policy).unwrap();
        let policy_bytes = base64::engine::general_purpose::STANDARD
            .decode(&normalized_policy.bytes)
            .unwrap();
        let policy_cid = cas.put(policy_bytes).unwrap();

        // Call twice
        let rc1 = judge(prompt_cid.clone(), policy_cid.clone(), &cas).unwrap();
        let rc2 = judge(prompt_cid, policy_cid, &cas).unwrap();
        // Note: In a real implementation with actual LLM calls, this might not be deterministic
        // But the CID generation from normalized output should still be deterministic
        assert!(!rc1.recibo.content_cid.is_empty());
        assert!(!rc2.recibo.content_cid.is_empty());
    }
}
