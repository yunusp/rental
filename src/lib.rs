use sha2::{self, Sha256, Digest};

pub fn sha256sum(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:X}", hasher.finalize()) //:X = hexadecimal uppercase
}