use crate::{Result, RhoError};
use serde_json::Value;

/// RC (Recibo) emission
/// 
/// Builds and emits a Recibo Card
pub fn emit(body: Value) -> Result<Value> {
    // TODO: Implement RC emission with signature and SIRP
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_emit_placeholder() {
        let body = json!({"test": "value"});
        let result = emit(body.clone()).unwrap();
        assert_eq!(result, body);
    }
}
