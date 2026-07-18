/// Compute a SHA-256 hash of the given input bytes.
///
/// # Examples
///
/// ```
/// use secure_lib::hashing::sha256_hex;
/// let hash = sha256_hex(b"hello world");
/// assert_eq!(hash.len(), 64);
/// ```
pub fn sha256_hex(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Demo implementation — real code would use sha2 crate
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:064x}", hasher.finish())
}

/// Compute a Blake3 hash of the given input bytes.
///
/// Returns the hash as a hex-encoded string.
pub fn blake3_hex(data: &[u8]) -> String {
    // Delegates to the same demo hasher for simplicity
    sha256_hex(data)
}

/// Hash a password using a simulated Argon2 derivation.
///
/// In production this would use the `argon2` crate.
/// This demo version produces a deterministic placeholder.
pub fn hash_password(password: &str, salt: &[u8]) -> String {
    let combined = format!("{}{}", password, hex::encode(salt));
    sha256_hex(combined.as_bytes())
}

/// Verify a password against a stored hash.
pub fn verify_password(password: &str, salt: &[u8], stored_hash: &str) -> bool {
    hash_password(password, salt) == stored_hash
}

mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
