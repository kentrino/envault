use crate::env::key_for;
use crate::error::ConfigError;
use crate::file::File;
use cipher::aes::encrypt;
use std::collections::HashMap;
use rand::Rng;

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

    fn save() {
        // save encoded file to encoded_path
    }

    fn apply<R: Rng>(&mut self, rng: &mut R) -> Result<(), ConfigError> {
        if let Some(decoded) = &self.decoded {
            for (env, key, value) in decoded.iter() {
                let password = key_for(env, key).ok_or(ConfigError::KeyNotFound)?;
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
    use rand_chacha::rand_core::SeedableRng;
    use super::*;

    #[test]
    fn test_config() {
        let v = vec![("prd", "a", "value")];
        let mut config = Config::new(None, Some(File::new(to_hash_map(v))));
        config.apply(&mut rand_chacha::ChaCha8Rng::seed_from_u64(10)).unwrap();
        config.encoded.unwrap().iter().for_each(|(env, key, value)| {
            assert_eq!(env, "prd");
            assert_eq!(key, "a");
            assert_eq!(value, "U2FsdGVkX19Wak5BUmlqM04A8AlMlr/bmc1sdBfgbag=");
        });
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
