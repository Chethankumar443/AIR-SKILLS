use sha2::{Digest, Sha256};

/// Calculate hex-encoded SHA-256 hash of a byte slice
pub fn sha256_digest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
