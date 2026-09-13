//! Odds source adapters. All adapters produce [`Quote`] values.
//!
//! **No live scraping.** Mock books are synthetic; JSON and Xpressbet stubs
//! read local files only.

mod json_file;
mod mock;
mod xpressbet;

pub use json_file::JsonFileAdapter;
pub use mock::{MockBookA, MockBookB};
pub use xpressbet::XpressbetStub;

use crate::types::Quote;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid odds for {outcome}: {detail}")]
    InvalidOdds { outcome: String, detail: String },
    #[error("{0}")]
    Other(String),
}

/// Trait for any odds source.
pub trait OddsAdapter {
    fn name(&self) -> &str;
    fn fetch_quotes(&self) -> Result<Vec<Quote>, AdapterError>;
}
