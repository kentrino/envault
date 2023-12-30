const PREFIX: &str = "ENV_KEY__";

pub fn key_for(_env: &str, _key: &str) -> Option<String> {
    return match std::env::var(PREFIX.to_string() + _env + "__" + _key) {
        Ok(value) => Some(value),
        Err(_) => match std::env::var(PREFIX.to_string() + _env) {
            Ok(value) => Some(value),
            Err(_) => None,
        },
    };
}
