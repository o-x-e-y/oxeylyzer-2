use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub heatmap: i64,
    pub sfbs: i64,
    pub sfs: i64,
}
