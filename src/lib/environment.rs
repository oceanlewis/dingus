use crate::lib::error::Error;
use std::{collections::HashMap, env};

type Variables = HashMap<String, String>;

const LEVEL_VAR_NAME: &str = "DINGUS_LEVEL";

#[derive(Debug)]
pub struct Environment {
    pub variables: Variables,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            variables: Variables::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        for (key, value) in other.variables.into_iter() {
            self.variables.insert(key, value);
        }
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, Error> {
        Ok(Self {
            variables: serde_yaml::from_str(&yaml)?,
        })
    }

    pub fn current_level(&self) -> Option<u32> {
        env::var(LEVEL_VAR_NAME)
            .ok()
            .and_then(|lvl| str::parse::<u32>(&lvl).ok())
    }

    pub fn increment_level(&mut self) {
        let previous_level = self.current_level().unwrap_or_default();
        let new_level = previous_level + 1;

        self.variables
            .insert(LEVEL_VAR_NAME.to_string(), new_level.to_string());
    }
}
