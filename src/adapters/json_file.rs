//! Generic JSON file odds adapter.
//!
//! Expected shape (array of quotes):
//! ```json
//! [
//!   {
//!     "book": "SomeBook",
//!     "track": "SA",
//!     "date": "2026-09-13",
//!     "race_number": 5,
//!     "market": "win",
//!     "outcome": "Thunder Bay",
//!     "odds": 3.5
//!   }
//! ]
//! ```

use super::{AdapterError, OddsAdapter};
use crate::types::{DecimalOdds, MarketKind, Outcome, Quote, RaceKey};
use chrono::NaiveDate;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RawQuote {
    book: String,
    track: String,
    date: NaiveDate,
    race_number: u32,
    market: MarketKind,
    outcome: String,
    odds: f64,
}

pub struct JsonFileAdapter {
    path: PathBuf,
    label: String,
}

impl JsonFileAdapter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let label = format!("JsonFile({})", path.display());
        Self { path, label }
    }

    pub fn load_quotes(path: impl AsRef<Path>) -> Result<Vec<Quote>, AdapterError> {
        let text = fs::read_to_string(path)?;
        let raw: Vec<RawQuote> = serde_json::from_str(&text)?;
        let mut out = Vec::with_capacity(raw.len());
        for r in raw {
            let odds = DecimalOdds::new(r.odds).map_err(|e| AdapterError::InvalidOdds {
                outcome: r.outcome.clone(),
                detail: e.to_string(),
            })?;
            out.push(Quote {
                book: r.book,
                race: RaceKey::new(r.track, r.date, r.race_number),
                market: r.market,
                outcome: Outcome::new(r.outcome),
                odds,
            });
        }
        Ok(out)
    }
}

impl OddsAdapter for JsonFileAdapter {
    fn name(&self) -> &str {
        &self.label
    }

    fn fetch_quotes(&self) -> Result<Vec<Quote>, AdapterError> {
        Self::load_quotes(&self.path)
    }
}
