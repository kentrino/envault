use aes::cipher::block_padding::UnpadError;
use hmac::digest::InvalidLength;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Invalid key length")]
    InvalidLength(#[from] InvalidLength),

    #[error("Invalid cipher header, header byte should be `Salted__`")]
    InvalidCipherHeader(),

    #[error("Cipher text is too short")]
    CipherTextIsTooShort(),

    #[error("Invalid key")]
    InvalidKey(#[from] UnpadError),

    #[error("Invalid Base64 encoding")]
    InvalidBase64Encoding(#[from] base64::DecodeError),

    #[error("Invalid UTF-8 encoding")]
    InvalidUtf8Encoding(#[from] FromUtf8Error),
}
