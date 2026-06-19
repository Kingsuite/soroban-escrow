use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};

/// Returns SHA-256 hash as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Returns SHA-256 hash as bytes
pub fn sha256_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Returns SHA-512 hash as hex string
pub fn sha512_hex(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Returns BLAKE3 hash as hex string
pub fn blake3_hex(data: &[u8]) -> String {
    hex::encode(blake3::hash(data).as_bytes())
}

/// Returns HMAC-SHA256 signature as hex string
///
/// # Examples
///
/// ```
/// use soroban_toolkit::hash::hmac_sha256;
///
/// let key = b"secret key";
/// let message = b"message";
/// let result = hmac_sha256(key, message);
/// assert_eq!(result, "8b5f48702995c15988c307eee256e215a380174e328b313d16d41b0b23a0e60e");
/// ```
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take any key size");
    mac.update(message);
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Returns double SHA-256 hash as hex string (used in blockchain contexts like Bitcoin)
///
/// # Examples
///
/// ```
/// use soroban_toolkit::hash::double_sha256;
///
/// let result = double_sha256(b"hello");
/// assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
/// ```
pub fn double_sha256(data: &[u8]) -> String {
    let first = sha256_bytes(data);
    sha256_hex(&first)
}

/// Timing-safe comparison of two byte slices
pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_value() {
        let result = sha256_hex(b"hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hmac_sha256_known_value_1() {
        let key = b"secret key";
        let message = b"message";
        let result = hmac_sha256(key, message);
        assert_eq!(result, "8b5f48702995c15988c307eee256e215a380174e328b313d16d41b0b23a0e60e");
    }

    #[test]
    fn test_hmac_sha256_rfc4231_test_case_1() {
        let key = [0x0b; 20];
        let message = b"Hi There";
        let expected = "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7";
        let result = hmac_sha256(&key, message);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hmac_sha256_rfc4231_test_case_2() {
        let key = b"Jefe";
        let message = b"what do ya want for nothing?";
        let expected = "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";
        let result = hmac_sha256(key, message);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sha512_not_empty() {
        let result = sha512_hex(b"hello");
        assert!(!result.is_empty());
        assert_eq!(result.len(), 128);
    }

    #[test]
    fn test_double_sha256() {
        let result = double_sha256(b"hello");
        assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_secure_compare_equal() {
        assert!(secure_compare(b"hello", b"hello"));
    }

    #[test]
    fn test_secure_compare_not_equal() {
        assert!(!secure_compare(b"hello", b"world"));
    }
}
