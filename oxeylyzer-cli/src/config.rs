use std::path::Path;

use serde::{Deserialize, Serialize};

use oxeylyzer_core::prelude::Weights;

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // pub analyzer: AnalyzerConfig,
    pub weights: Weights,
    pub corpus: String,
    pub layouts: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;

        toml::from_str(&s).map_err(Into::into)
    }
}
