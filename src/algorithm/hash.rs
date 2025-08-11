use sha2::{Digest, Sha256};

pub fn hash_sha(payload: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_str());
    let result = hasher.finalize();
    hex::encode(result)
}
