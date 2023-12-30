use crate::error::ConfigError;

const PREFIX: &str = "ENV_KEY__";

pub fn key_for(_env: &str, _key: &str) -> Result<String, ConfigError> {
    match std::env::var(PREFIX.to_string() + _env + "__" + _key) {
        Ok(value) => Ok(value),
        Err(_) => match std::env::var(PREFIX.to_string() + _env) {
            Ok(value) => Ok(value),
            Err(_) => Err(ConfigError::KeyNotFound),
        },
    }
}
