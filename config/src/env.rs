use crate::error::ConfigError;

const PREFIX: &str = "ENV_KEY";


pub fn key_for(_env: &str, _key: &str) -> Result<String, ConfigError> {
    match std::env::var(PREFIX.to_string() + "__" + _env + "__" + _key) {
        Ok(value) => return Ok(value),
        Err(_) => (),
    }
    match std::env::var(PREFIX.to_string() + "__" + _env) {
        Ok(value) => return Ok(value),
        Err(_) => (),
    }
    match std::env::var(PREFIX.to_string()) {
        Ok(value) => Ok(value),
        Err(_) => Err(ConfigError::KeyNotFound),
    }
}

#[cfg(test)]
mod tests {
    use crate::env::key_for;
    use crate::with_env_vars::with_env_vars;

    #[test]
    fn with_root_password() {
        with_env_vars(vec![
            ("ENV_KEY", Some("password1"))
        ], || {
            let password = key_for("dev", "a").unwrap();
            assert_eq!(password, "password1");
        })
    }

    #[test]
    fn with_password_for_env() {
        with_env_vars(vec![
            ("ENV_KEY", Some("password1")),
            ("ENV_KEY__dev", Some("password2"))
        ], || {
            let password = key_for("dev", "a").unwrap();
            assert_eq!(password, "password2");
        })
    }

    #[test]
    fn with_password_for_key() {
        with_env_vars(vec![
            ("ENV_KEY", Some("password1")),
            ("ENV_KEY__dev", Some("password2")),
            ("ENV_KEY__dev__a", Some("password3"))
        ], || {
            let password = key_for("dev", "a").unwrap();
            assert_eq!(password, "password3");
        })
    }
}