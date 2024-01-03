use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Yaml conversion failed")]
    YamlConversionFailed(#[from] serde_yaml::Error),

    #[error("Environment variable is not found")]
    KeyNotFound,

    #[error("Cipher error")]
    CipherError(#[from] envault_cipher::error::CipherError),

    #[error("Illegal state")]
    IllegalState,

    #[error("File save/open error")]
    FileError(#[from] std::io::Error),
}
