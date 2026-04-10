use sha2::{Sha256, Digest};
use hex;

pub struct SHA256Hasher;

impl SHA256Hasher {
    pub fn hash_bytes(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn hash_string(data: &str) -> String {
        Self::hash_bytes(data.as_bytes())
    }

    pub fn hash_double(data: &[u8]) -> String {
        let first = Sha256::digest(data);
        let second = Sha256::digest(&first);
        hex::encode(second)
    }

    pub fn verify_hash(data: &[u8], target_hash: &str) -> bool {
        let computed = Self::hash_bytes(data);
        computed == target_hash
    }
}
