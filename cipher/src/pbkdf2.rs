use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha256;
use crate::error::CipherError;

pub fn key_and_iv(password: &[u8], salt: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CipherError> {
    let mut out = [0u8; 48];
    pbkdf2::<Hmac<Sha256>>(password, salt, 10000, &mut out).map_err(CipherError::InvalidLength)?;
    Ok((out[0..32].to_vec(), out[32..48].to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_pair() {
        let password = "password";
        let (key, iv) = key_and_iv(password.as_bytes(), b"nicesalt").unwrap();
        assert_eq!(to_hex_string(&key), "4e9bf2f86bbad0b354f9ff3f15a1770148ab0c69f9fb66fc22086aebb6b92c8b");
        assert_eq!(to_hex_string(&iv), "a009c2a2a14a1bd7886cdda5bc5d22be");
    }

    fn to_hex_string(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("")
    }
}
