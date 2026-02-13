use crate::types::Cid;
use crate::{Result, RhoError};
use std::collections::HashMap;
use std::sync::Mutex;

/// Content Addressable Storage
///
/// Stores content by its blake3 hash (CID)
pub struct Cas {
    storage: Mutex<HashMap<Cid, Vec<u8>>>,
}

impl Cas {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(HashMap::new()),
        }
    }

    /// Store bytes and return the CID
    pub fn put(&self, bytes: Vec<u8>) -> Result<Cid> {
        let hash = blake3::hash(&bytes);
        let cid =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash.as_bytes());

        let mut storage = self.storage.lock().unwrap();
        storage.insert(cid.clone(), bytes);

        Ok(cid)
    }

    /// Retrieve bytes by CID
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>> {
        let storage = self.storage.lock().unwrap();
        storage
            .get(cid)
            .cloned()
            .ok_or_else(|| RhoError::Cas(format!("CID not found: {}", cid)))
    }
}

impl Default for Cas {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cas_put_get() {
        let cas = Cas::new();
        let data = b"test data".to_vec();

        let cid = cas.put(data.clone()).unwrap();
        let retrieved = cas.get(&cid).unwrap();

        assert_eq!(data, retrieved);
    }

    #[test]
    fn test_cas_deterministic() {
        let cas = Cas::new();
        let data = b"test data".to_vec();

        let cid1 = cas.put(data.clone()).unwrap();
        let cid2 = cas.put(data.clone()).unwrap();

        assert_eq!(cid1, cid2);
    }
}
