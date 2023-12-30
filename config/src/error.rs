use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Yaml parse failed")]
    YamlParseFailed(#[from] serde_yaml::Error),

    #[error("Environment variable is not found")]
    KeyNotFound,

    #[error("Cipher error")]
    CipherError(#[from] cipher::error::CipherError),

    #[error("Illegal state")]
    IllegalState,

    #[error("IO error")]
    FileSaveError(#[from] std::io::Error),
}
