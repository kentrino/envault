const PREFIX: &str = "CRYENV_";

pub fn key_for(_env: &str, _key: &str) -> Option<String> {
    #[cfg(not(test))]
    return match std::env::var(PREFIX.to_string() + _env + "_" + _key) {
        Ok(value) => Some(value),
        Err(_) => match std::env::var(PREFIX.to_string() + _env) {
            Ok(value) => Some(value),
            Err(_) => None,
        },
    };

    #[cfg(test)]
    return Some("password".to_string());
}
