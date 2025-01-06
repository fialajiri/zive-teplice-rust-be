use rand::RngCore;
use ring::pbkdf2;
use std::num::NonZeroU32;
use data_encoding::HEXLOWER;

const NUMBER_OF_ITERATIONS: u32 = 25000;
const KEYLEN: usize = 512;

pub struct Password;

impl Password {
    pub fn generate_password(password: &str) -> String {
        let mut salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        let salt_hex = HEXLOWER.encode(&salt);
        
        let mut hash = vec![0u8; KEYLEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(NUMBER_OF_ITERATIONS).unwrap(),
            &salt,
            password.as_bytes(),
            &mut hash,
        );
        
        let hash_hex = HEXLOWER.encode(&hash);
        format!("{}.{}", hash_hex, salt_hex)
    }

    pub fn compare(stored_password: &str, supplied_password: &str) -> bool {
        let parts: Vec<&str> = stored_password.split('.').collect();
        if parts.len() != 2 {
            return false;
        }

        let hash_hex = parts[0];
        let salt_hex = parts[1];
        
        let salt = match HEXLOWER.decode(salt_hex.as_bytes()) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let mut hash = vec![0u8; KEYLEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(NUMBER_OF_ITERATIONS).unwrap(),
            &salt,
            supplied_password.as_bytes(),
            &mut hash,
        );
        
        let verify_hex = HEXLOWER.encode(&hash);
        hash_hex == verify_hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_verification() {
        let password = "my_secure_password";
        let hash = Password::generate_password(password);
        assert!(Password::compare(&hash, password));
        assert!(!Password::compare(&hash, "wrong_password"));
    }
}