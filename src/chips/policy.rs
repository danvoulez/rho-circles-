use crate::types::{PolicyEvalOutput, Proof};
use crate::{Result, RhoError};

/// Policy AST node
#[derive(Debug, Clone, PartialEq)]
enum PolicyNode {
    True,
    False,
    Ed25519,
    MlDsa3,
    HybridAnd(Vec<PolicyNode>),
    HybridOr(Vec<PolicyNode>),
}

/// Evaluate a signature policy expression against provided proofs
///
/// Policy Grammar:
/// policy = hybrid-and "(" list ")"
///        | hybrid-or  "(" list ")"
///        | ed25519 | mldsa3 | "true" | "false"
/// list = policy ("," policy)*
pub fn policy_eval(policy_expr: String, proofs: Vec<Proof>) -> Result<PolicyEvalOutput> {
    // Parse the policy expression
    let policy = parse_policy(&policy_expr)?;

    // Evaluate the policy against proofs
    let result = evaluate_policy(&policy, &proofs);

    Ok(PolicyEvalOutput { result })
}

/// Parse policy expression into AST
fn parse_policy(expr: &str) -> Result<PolicyNode> {
    let expr = expr.trim();

    if expr == "true" {
        return Ok(PolicyNode::True);
    }
    if expr == "false" {
        return Ok(PolicyNode::False);
    }
    if expr == "ed25519" {
        return Ok(PolicyNode::Ed25519);
    }
    if expr == "mldsa3" {
        return Ok(PolicyNode::MlDsa3);
    }

    // Check for hybrid-and or hybrid-or
    if let Some(content) = expr.strip_prefix("hybrid-and(") {
        if !content.ends_with(')') {
            return Err(RhoError::Policy("Missing closing paren".to_string()));
        }
        let inner = &content[..content.len() - 1];
        let policies = parse_list(inner)?;
        return Ok(PolicyNode::HybridAnd(policies));
    }

    if let Some(content) = expr.strip_prefix("hybrid-or(") {
        if !content.ends_with(')') {
            return Err(RhoError::Policy("Missing closing paren".to_string()));
        }
        let inner = &content[..content.len() - 1];
        let policies = parse_list(inner)?;
        return Ok(PolicyNode::HybridOr(policies));
    }

    Err(RhoError::Policy(format!("Unknown policy: {}", expr)))
}

/// Parse comma-separated list of policies
fn parse_list(list: &str) -> Result<Vec<PolicyNode>> {
    if list.is_empty() {
        return Ok(vec![]);
    }

    let mut policies = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, ch) in list.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                policies.push(parse_policy(&list[start..i])?);
                start = i + 1;
            }
            _ => {}
        }
    }

    // Don't forget the last element
    if start < list.len() {
        policies.push(parse_policy(&list[start..])?);
    }

    Ok(policies)
}

/// Evaluate policy tree against proofs
fn evaluate_policy(policy: &PolicyNode, proofs: &[Proof]) -> bool {
    match policy {
        PolicyNode::True => true,
        PolicyNode::False => false,
        PolicyNode::Ed25519 => proofs
            .iter()
            .any(|p| p.algorithm.to_lowercase() == "ed25519"),
        PolicyNode::MlDsa3 => proofs
            .iter()
            .any(|p| p.algorithm.to_lowercase() == "mldsa3"),
        PolicyNode::HybridAnd(policies) => {
            // Short-circuit: all must be true
            policies.iter().all(|p| evaluate_policy(p, proofs))
        }
        PolicyNode::HybridOr(policies) => {
            // Short-circuit: at least one must be true
            policies.iter().any(|p| evaluate_policy(p, proofs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_proof(algorithm: &str) -> Proof {
        Proof {
            algorithm: algorithm.to_string(),
            public_key: "test_key".to_string(),
            signature: "test_sig".to_string(),
            message_cid: "test_cid".to_string(),
        }
    }

    #[test]
    fn test_policy_eval_true() {
        let result = policy_eval("true".to_string(), vec![]).unwrap();
        assert!(result.result);
    }

    #[test]
    fn test_policy_eval_false() {
        let result = policy_eval("false".to_string(), vec![]).unwrap();
        assert!(!result.result);
    }

    #[test]
    fn test_policy_eval_ed25519() {
        let proofs = vec![make_proof("ed25519")];
        let result = policy_eval("ed25519".to_string(), proofs).unwrap();
        assert!(result.result);
    }

    #[test]
    fn test_policy_eval_ed25519_missing() {
        let proofs = vec![make_proof("mldsa3")];
        let result = policy_eval("ed25519".to_string(), proofs).unwrap();
        assert!(!result.result);
    }

    #[test]
    fn test_policy_eval_hybrid_and_success() {
        let proofs = vec![make_proof("ed25519"), make_proof("mldsa3")];
        let result = policy_eval("hybrid-and(ed25519,mldsa3)".to_string(), proofs).unwrap();
        assert!(result.result);
    }

    #[test]
    fn test_policy_eval_hybrid_and_failure() {
        let proofs = vec![make_proof("ed25519")];
        let result = policy_eval("hybrid-and(ed25519,mldsa3)".to_string(), proofs).unwrap();
        assert!(!result.result);
    }

    #[test]
    fn test_policy_eval_hybrid_or_success() {
        let proofs = vec![make_proof("ed25519")];
        let result = policy_eval("hybrid-or(ed25519,mldsa3)".to_string(), proofs).unwrap();
        assert!(result.result);
    }

    #[test]
    fn test_policy_eval_hybrid_or_failure() {
        let proofs = vec![];
        let result = policy_eval("hybrid-or(ed25519,mldsa3)".to_string(), proofs).unwrap();
        assert!(!result.result);
    }

    #[test]
    fn test_policy_eval_nested() {
        let proofs = vec![make_proof("ed25519")];
        let result = policy_eval(
            "hybrid-or(hybrid-and(ed25519,mldsa3),ed25519)".to_string(),
            proofs,
        )
        .unwrap();
        assert!(result.result); // Second branch succeeds
    }
}
