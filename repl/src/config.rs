use oxeylyzer_core::prelude::Weights;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, OneOrMany};
use std::path::{Path, PathBuf};

use crate::Result;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub weights: Weights,
    pub corpus: String,
    #[serde_as(as = "OneOrMany<_>")]
    pub layouts: Vec<PathBuf>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;

        toml::from_str(&s).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[serde_as]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Layouts {
        #[serde_as(as = "OneOrMany<_>")]
        layouts: Vec<PathBuf>,
    }

    #[test]
    fn one_or_many() {
        let s1 = r#"layouts = "./layouts""#;
        let s2 = r#"layouts = ["./vec", "./p2"]"#;
        let s3 = "layouts = []";

        let p1 = toml::from_str::<Layouts>(&s1);
        let p2 = toml::from_str::<Layouts>(&s2);
        let p3 = toml::from_str::<Layouts>(&s3);

        println!("{:?}", p1);
        println!("{:?}", p2);
        println!("{:?}", p3);

        assert!(p1.is_ok());
        assert!(p2.is_ok());
        assert!(p3.is_ok());
    }
}
