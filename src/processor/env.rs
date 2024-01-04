use crate::processor::error::ConfigError;

const PREFIX: &str = "ENV_KEY";

pub fn key_for(_env: &str, _key: &str) -> Result<(String, String), ConfigError> {
    let mut name = PREFIX.to_string() + "__" + _env + "__" + _key;
    if let Ok(value) = std::env::var(&name) {
        return Ok((name.to_string(), value));
    }
    name = PREFIX.to_string() + "__" + _env;
    if let Ok(value) = std::env::var(&name) {
        return Ok((name.to_string(), value));
    }
    name = PREFIX.to_string();
    match std::env::var(PREFIX) {
        Ok(value) => Ok((name, value)),
        Err(_) => Err(ConfigError::KeyNotFound),
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::env::key_for;
    use crate::processor::with_env_vars::with_env_vars;

    #[test]
    fn with_root_password() {
        with_env_vars(vec![("ENV_KEY", Some("password1"))], || {
            let (_, password) = key_for("dev", "a").unwrap();
            assert_eq!(password, "password1");
        })
    }

    #[test]
    fn with_password_for_env() {
        with_env_vars(
            vec![
                ("ENV_KEY", Some("password1")),
                ("ENV_KEY__dev", Some("password2")),
            ],
            || {
                let (_, password) = key_for("dev", "a").unwrap();
                assert_eq!(password, "password2");
            },
        )
    }

    #[test]
    fn with_password_for_key() {
        with_env_vars(
            vec![
                ("ENV_KEY", Some("password1")),
                ("ENV_KEY__dev", Some("password2")),
                ("ENV_KEY__dev__a", Some("password3")),
            ],
            || {
                let (_, password) = key_for("dev", "a").unwrap();
                assert_eq!(password, "password3");
            },
        )
    }
}
