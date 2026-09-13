//! Xpressbet **stub** — reads a local JSON snapshot only.
//!
//! This does **not** scrape xpressbet.com or any live feed. Point it at a
//! file you exported yourself. Shape matches the generic JSON adapter, with
//! an optional `"source": "xpressbet"` field ignored on deserialize.

use super::json_file::JsonFileAdapter;
use super::{AdapterError, OddsAdapter};
use crate::types::Quote;
use std::path::{Path, PathBuf};

pub struct XpressbetStub {
    inner: JsonFileAdapter,
    label: String,
}

impl XpressbetStub {
    pub fn from_local_json(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let label = format!("XpressbetStub({})", path.display());
        Self {
            inner: JsonFileAdapter::new(path),
            label,
        }
    }

    /// Convenience for the bundled sample under `fixtures/`.
    pub fn sample(path: PathBuf) -> Self {
        Self::from_local_json(path)
    }
}

impl OddsAdapter for XpressbetStub {
    fn name(&self) -> &str {
        &self.label
    }

    fn fetch_quotes(&self) -> Result<Vec<Quote>, AdapterError> {
        // Local file only — no network, no scrape.
        self.inner.fetch_quotes()
    }
}
