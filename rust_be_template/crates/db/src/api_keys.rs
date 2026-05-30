use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand_core::OsRng;
use rand_core::TryRngCore;

const SECRET_BYTES: usize = 32;
const PREFIX_SUFFIX_BYTES: usize = 4;

pub fn generate_api_key(base_prefix: &str) -> Result<(String, String)> {
    let key_prefix = generate_unique_prefix(base_prefix)?;
    let secret = generate_secret()?;
    let full_key = format!("{key_prefix}.{secret}");
    Ok((full_key, key_prefix))
}

pub fn hash_api_key(full_key: &str) -> Result<String> {
    Ok(bcrypt::hash(full_key, bcrypt::DEFAULT_COST)?)
}

pub fn verify_api_key(full_key: &str, stored_hash: &str) -> Result<bool> {
    Ok(bcrypt::verify(full_key, stored_hash)?)
}

fn generate_unique_prefix(base_prefix: &str) -> Result<String> {
    let mut suffix_bytes = [0u8; PREFIX_SUFFIX_BYTES];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut suffix_bytes)?;
    let suffix: String = suffix_bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(format!("{base_prefix}_{suffix}"))
}

fn generate_secret() -> Result<String> {
    let mut bytes = [0u8; SECRET_BYTES];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut bytes)?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_key_splits_into_unique_prefix_and_secret() {
        let (full_key, key_prefix) = generate_api_key("sk_live").expect("generate");
        let (prefix, secret) = full_key.split_once('.').expect("dot separator");
        assert_eq!(prefix, key_prefix);
        assert!(key_prefix.starts_with("sk_live_"));
        assert!(!secret.is_empty());
    }

    #[test]
    fn hash_and_verify_round_trip() {
        let (full_key, _) = generate_api_key("sk_test").expect("generate");
        let hash = hash_api_key(&full_key).expect("hash");
        assert!(verify_api_key(&full_key, &hash).expect("verify"));
        assert!(!verify_api_key("sk_test_deadbeef.wrong", &hash).expect("verify"));
    }
}
