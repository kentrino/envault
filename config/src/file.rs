use crate::error::ConfigError;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    path: String,
    data: HashMap<String, HashMap<String, String>>,
}

#[allow(dead_code)]
impl File {
    pub fn new(data: HashMap<String, HashMap<String, String>>) -> Self {
        File {
            path: "".to_string(),
            data,
        }
    }

    pub fn for_each(&self, f: impl Fn(&str, &str, &str)) {
        for (env, keys) in self.data.iter() {
            for (key, value) in keys.iter() {
                f(env, key, value);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.data.iter().flat_map(|(env, keys)| {
            keys.iter()
                .map(move |(key, value)| (env.as_str(), key.as_str(), value.as_str()))
        })
    }

    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let file = std::fs::File::open(path).unwrap();
        let data: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_reader(file).map_err(ConfigError::YamlParseFailed)?;
        Ok(File {
            path: path.to_string(),
            data,
        })
    }

    fn save(&self) {
        let file = std::fs::File::create(&self.path).unwrap();
        serde_yaml::to_writer(file, &self.data).unwrap();
    }

    pub fn set(&mut self, env: &str, key: &str, value: &str) {
        let hm: &mut HashMap<String, String> = self.data.get_mut(env).unwrap();
        hm.insert(key.to_string(), value.to_string());
    }
}
