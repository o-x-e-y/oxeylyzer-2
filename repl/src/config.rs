use serde::{Deserialize, Serialize};
use oxeylyzer_core::prelude::Weights;
use std::path::{Path, PathBuf};

use crate::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum OneOrManyString {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub weights: Weights,
    pub corpus: String,
    pub layouts: Vec<PathBuf>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;

        toml::from_str(&s).map_err(Into::into)
    }
}
