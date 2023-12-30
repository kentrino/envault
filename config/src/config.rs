use crate::env::key_for;
use crate::error::ConfigError;
use crate::file::File;
use cipher::aes::{decrypt, encrypt};
use rand::Rng;
use std::collections::HashMap;

struct Config {
    pub encoded: Option<File>,
    pub decoded: Option<File>,
}
#[allow(dead_code)]
impl Config {
    fn new(encoded: Option<File>, decoded: Option<File>) -> Self {
        Config { encoded, decoded }
    }

    fn load(encoded_path: Option<&str>, decoded_path: Option<&str>) -> Result<Self, ConfigError> {
        Ok(Config {
            encoded: match encoded_path {
                Some(path) => Some(File::load(path)?),
                None => None,
            },
            decoded: match decoded_path {
                Some(path) => Some(File::load(path)?),
                None => None,
            },
        })
    }

    fn save(&self, encoded_path: &str) -> Result<(), ConfigError> {
        if let Some(encoded) = &self.encoded {
            encoded.save(encoded_path)?;
            Ok(())
        } else {
            Err(ConfigError::IllegalState)
        }
    }

    fn export(&self, env: &str) -> Result<String, ConfigError> {
        let mut result = String::new();
        if let Some(decoded) = &self.decoded {
            for (_env, key, value) in decoded.iter() {
                if env == _env {
                    let password = key_for(_env, key)?;
                    result.push_str(&format!("export {}={};", key, decrypt(value, &password)?));
                }
            }
        }
        Ok(result)
    }

    fn apply<R: Rng>(&mut self, rng: &mut R) -> Result<(), ConfigError> {
        if let Some(decoded) = &self.decoded {
            for (env, key, value) in decoded.iter() {
                let password = key_for(env, key)?;
                let encrypted = encrypt(value, &password, rng)?;
                match &mut self.encoded {
                    Some(ref mut encoded) => {
                        encoded.set(env, key, &encrypted);
                    }
                    None => {
                        let mut root = HashMap::new();
                        let mut child = HashMap::new();
                        child.insert(key.to_string(), encrypted);
                        root.insert(env.to_string(), child);
                        self.encoded = Some(File::new(root));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::rand_core::SeedableRng;

    #[test]
    fn test_config() {
        std::env::set_var("ENV_KEY__prd__a", "password");
        let old = vec![("prd", "a", "old_value")];
        let new = vec![("prd", "a", "new_value")];
        let mut config = Config::new(
            Some(File::new(to_hash_map(old))),
            Some(File::new(to_hash_map(new))),
        );
        config
            .encoded
            .clone()
            .unwrap()
            .iter()
            .for_each(|(env, key, value)| {
                assert_eq!(env, "prd");
                assert_eq!(key, "a");
                assert_eq!(value, "old_value");
            });
        config
            .apply(&mut rand_chacha::ChaCha8Rng::seed_from_u64(10))
            .unwrap();
        config
            .encoded
            .unwrap()
            .iter()
            .for_each(|(env, key, value)| {
                assert_eq!(env, "prd");
                assert_eq!(key, "a");
                assert_eq!(value, "U2FsdGVkX19Wak5BUmlqM6K9BRN7rxlC2+NUsA+Qo4k=");
            });
    }

    #[test]
    fn test_export() {
        std::env::set_var("ENV_KEY__prd__a", "password");
        let new = vec![("prd", "a", "U2FsdGVkX19Wak5BUmlqM6K9BRN7rxlC2+NUsA+Qo4k=")];
        let mut config = Config::new(
            None,
            Some(File::new(to_hash_map(new))),
        );
        let res = config
            .export("prd").unwrap();
        assert_eq!(res, "export a=new_value;");
    }

    fn to_hash_map(v: Vec<(&str, &str, &str)>) -> HashMap<String, HashMap<String, String>> {
        let mut hm = HashMap::new();
        for (env, key, value) in v {
            hm.entry(env.to_string())
                .or_insert(HashMap::new())
                .insert(key.to_string(), value.to_string());
        }
        hm
    }
}
