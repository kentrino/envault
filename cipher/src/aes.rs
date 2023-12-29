use crate::error::CipherError;
use crate::pbkdf2::key_and_iv;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::engine::general_purpose::STANDARD as base64;
use base64::Engine;
use cbc::cipher::block_padding::Pkcs7;
use rand::distributions::Alphanumeric;
use rand::Rng;

const SALTED_MAGIC: &[u8] = b"Salted__";

#[allow(dead_code)]
pub fn decrypt(ciphertext: &str, key: &str) -> Result<String, CipherError> {
    base64
        .decode(ciphertext.as_bytes())
        .map_err(CipherError::InvalidBase64Encoding)
        .and_then(|v| decrypt_bytes(&v, key))
}

pub fn decrypt_bytes(ciphertext: &[u8], key: &str) -> Result<String, CipherError> {
    if ciphertext.len() < 16 {
        return Err(CipherError::CipherTextIsTooShort());
    }
    let (salted, rest) = ciphertext.split_at(8);
    if salted != SALTED_MAGIC {
        return Err(CipherError::InvalidCipherHeader());
    }
    let (salt, rest) = rest.split_at(8);
    let (key, iv) = key_and_iv(key.as_bytes(), salt)?;
    let cipher = cbc::Decryptor::<aes::Aes256>::new_from_slices(&key, &iv)?;
    let res = cipher
        .decrypt_padded_vec_mut::<Pkcs7>(rest)
        .map_err(CipherError::InvalidKey)?;
    String::from_utf8(res).map_err(CipherError::InvalidUtf8Encoding)
}

#[allow(dead_code)]
pub fn encrypt(plaintext: &str, key: &str) -> Result<String, CipherError> {
    let message = encrypt_bytes(plaintext, key)?;
    Ok(base64.encode(message))
}

pub fn encrypt_bytes(plaintext: &str, key: &str) -> Result<Vec<u8>, CipherError> {
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let (key, iv) = key_and_iv(key.as_bytes(), salt.as_bytes())?;
    let cipher = cbc::Encryptor::<aes::Aes256>::new_from_slices(&key, &iv)?;
    let plaintext = plaintext.as_bytes().to_vec();
    let ciphertext = cipher.encrypt_padded_vec_mut::<Pkcs7>(&plaintext);
    Ok([SALTED_MAGIC, salt.as_bytes(), &ciphertext].concat())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = "password";
        let plaintext = "Hello, world!";
        let ciphertext = encrypt(plaintext, key).unwrap();
        let decrypted = decrypt(&ciphertext, key).unwrap();
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_decrypt() {
        let password = "test";
        // generate:
        //   echo -n test | openssl aes-256-cbc -pbkdf2 -base64 -md sha256 -pass pass:test
        // decrypt:
        //   echo U2FsdGVkX19GqvuF+PkB8Pm7rCELUHdj/SxpgdBgwSU= | openssl aes-256-cbc -pbkdf2 -d -base64 -md sha256 -pass pass:test
        let encrypted = "U2FsdGVkX18OtYlc5sMYSNdZ8zUWhACPqYSSwVuPSPA=";
        let decrypted = decrypt(encrypted, password).unwrap();
        assert_eq!(decrypted, "test");
    }
}
