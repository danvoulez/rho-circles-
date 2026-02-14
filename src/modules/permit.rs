use crate::cas::Cas;
use crate::chips::policy_eval;
use crate::types::{Cid, Proof};
use crate::Result;

/// mod.permit: Access control policy evaluation
///
/// Evaluates whether a principal is allowed to perform an action on a resource
///
/// Inputs:
/// - principal: Principal identifier (e.g., user ID, public key)
/// - action: Action to perform (e.g., "read", "write", "delete")
/// - resource: Resource identifier
/// - policy_cid: CID of the policy in CAS
///
/// Output: Boolean indicating if access is allowed
pub fn permit(
    _principal: String,
    _action: String,
    _resource: String,
    _policy_cid: Cid,
    proofs: Vec<Proof>,
    _cas: &Cas,
) -> Result<bool> {
    // Fetch policy from CAS
    // In a real implementation, this would:
    // 1. Fetch the policy document from CAS
    // 2. Parse it into a policy expression
    // 3. Substitute principal/action/resource into the policy
    // 4. Evaluate the resulting policy expression
    //
    // For now, we use a simple policy expression based on the proofs

    // Build a simple policy: require at least one valid signature
    let policy_expr = if proofs.is_empty() {
        "false".to_string()
    } else {
        // For demonstration, we accept if any ed25519 or mldsa3 proof is present
        "hybrid-or(ed25519,mldsa3)".to_string()
    };

    // Evaluate the policy
    let result = policy_eval(policy_expr, proofs)?;

    // In a real system, we would also check:
    // - Principal matches the proof's public key
    // - Action is in the allowed set for this principal
    // - Resource is accessible by this principal
    //
    // For now, we just return the policy evaluation result
    Ok(result.result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permit_with_proof() {
        let cas = Cas::new();
        let proof = Proof {
            algorithm: "ed25519".to_string(),
            public_key: "test_key".to_string(),
            signature: "test_sig".to_string(),
            message_cid: "test_msg".to_string(),
        };
        let result = permit(
            "user123".to_string(),
            "read".to_string(),
            "resource456".to_string(),
            "policy_cid".to_string(),
            vec![proof],
            &cas,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_permit_without_proof() {
        let cas = Cas::new();
        let result = permit(
            "user123".to_string(),
            "read".to_string(),
            "resource456".to_string(),
            "policy_cid".to_string(),
            vec![],
            &cas,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_permit_deterministic() {
        let cas = Cas::new();
        let proof = Proof {
            algorithm: "ed25519".to_string(),
            public_key: "test_key".to_string(),
            signature: "test_sig".to_string(),
            message_cid: "test_msg".to_string(),
        };
        let result1 = permit(
            "user123".to_string(),
            "read".to_string(),
            "resource456".to_string(),
            "policy_cid".to_string(),
            vec![proof.clone()],
            &cas,
        )
        .unwrap();
        let result2 = permit(
            "user123".to_string(),
            "read".to_string(),
            "resource456".to_string(),
            "policy_cid".to_string(),
            vec![proof],
            &cas,
        )
        .unwrap();
        assert_eq!(result1, result2);
    }
}
