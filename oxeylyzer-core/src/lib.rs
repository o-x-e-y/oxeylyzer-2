use libdof::DofError;
use thiserror::Error;

pub mod analyze;
pub mod analyzer_data;
pub mod cached_layout;
pub mod char_mapping;
pub mod corpus_cleaner;
pub mod data;
pub mod depth_optimization;
pub mod layout;
pub mod optimization;
pub mod simulated_annealing;
pub mod stats;
pub mod weights;

pub mod prelude {
    pub use super::{
        analyze::Analyzer,
        cached_layout::CachedLayout,
        corpus_cleaner::*,
        data::Data,
        layout::{Layout, PosPair},
        weights::Weights,
        OxeylyzerError, REPEAT_KEY, REPLACEMENT_CHAR, SHIFT_CHAR,
    };
}

// pub use libdof;

pub const REPLACEMENT_CHAR: char = char::REPLACEMENT_CHARACTER;
pub const SHIFT_CHAR: char = '⇑';
pub const REPEAT_KEY: char = '@';

#[derive(Debug, Error)]
pub enum OxeylyzerError {
    #[error("Bigrams should contain 2 characters, bigram with length {0} encountered.")]
    InvalidBigramLength(usize),
    #[error("Trigrams should contain 3 characters, trigram with length {0} encountered.")]
    InvalidTrigramLength(usize),
    #[error("Failed to create a file chunker")]
    ChunkerInitError,
    #[error("Failed to create appropriate chunks")]
    ChunkerChunkError,
    #[error("Path must be either a directory or a file")]
    NotAFile,
    #[error("Specifying a name for the corpus is required")]
    MissingDataName,

    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    UTF8Error(#[from] std::str::Utf8Error),
    #[error("{0}")]
    DofError(#[from] DofError),

    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    GlooError(#[from] gloo_net::Error),
}

pub type Result<T> = std::result::Result<T, OxeylyzerError>;
