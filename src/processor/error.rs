use thiserror::Error;
use crate::cipher::error::CipherError;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Yaml conversion failed")]
    YamlConversionFailed(#[from] serde_yaml::Error),

    #[error("Environment variable is not found")]
    KeyNotFound,

    #[error("Cipher error")]
    CipherError(#[from] CipherError),

    #[error("Illegal state")]
    IllegalState,

    #[error("File save/open error")]
    FileError(#[from] std::io::Error),
}
