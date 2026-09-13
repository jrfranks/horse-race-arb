//! Mock bookmakers with intentional mispricing so `race-arb demo` finds surebets.

use super::{AdapterError, OddsAdapter};
use crate::types::{DecimalOdds, MarketKind, Outcome, Quote, RaceKey};
use chrono::NaiveDate;

fn demo_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 9, 13).unwrap()
}

fn race() -> RaceKey {
    RaceKey::new("SA", demo_date(), 5)
}

fn q(book: &str, market: MarketKind, outcome: &str, odds: f64) -> Quote {
    Quote {
        book: book.to_string(),
        race: race(),
        market,
        outcome: Outcome::new(outcome),
        odds: DecimalOdds::from_unchecked(odds),
    }
}

/// Mock Book A — soft on favorites / named horses.
pub struct MockBookA;

impl OddsAdapter for MockBookA {
    fn name(&self) -> &str {
        "MockBookA"
    }

    fn fetch_quotes(&self) -> Result<Vec<Quote>, AdapterError> {
        // Win field (3 runners): A is long on Thunder Bay and Meadow Lark,
        // short on Coastal Fog → combined with B yields arb.
        // Horse-vs-field: Thunder Bay @ 2.25 (generous).
        Ok(vec![
            q("MockBookA", MarketKind::Win, "Thunder Bay", 3.80),
            q("MockBookA", MarketKind::Win, "Coastal Fog", 2.40),
            q("MockBookA", MarketKind::Win, "Meadow Lark", 4.20),
            q("MockBookA", MarketKind::HorseVsField, "Thunder Bay", 2.25),
            q("MockBookA", MarketKind::HorseVsField, "Field", 1.85),
        ])
    }
}

/// Mock Book B — soft on the complementary outcomes (creates cross-book arb).
pub struct MockBookB;

impl OddsAdapter for MockBookB {
    fn name(&self) -> &str {
        "MockBookB"
    }

    fn fetch_quotes(&self) -> Result<Vec<Quote>, AdapterError> {
        // Win: B is long on Coastal Fog; A was long on the other two.
        // Best across books: TB 3.80 (A), CF 3.60 (B), ML 4.20 (A)
        // S = 1/3.8 + 1/3.6 + 1/4.2 ≈ 0.263 + 0.278 + 0.238 = 0.779 < 1 ✓
        // Horse-vs-field: Field @ 2.20 (generous) vs A's Thunder Bay 2.25
        // S = 1/2.25 + 1/2.20 ≈ 0.444 + 0.455 = 0.899 < 1 ✓
        Ok(vec![
            q("MockBookB", MarketKind::Win, "Thunder Bay", 2.90),
            q("MockBookB", MarketKind::Win, "Coastal Fog", 3.60),
            q("MockBookB", MarketKind::Win, "Meadow Lark", 3.10),
            q("MockBookB", MarketKind::HorseVsField, "Thunder Bay", 1.80),
            q("MockBookB", MarketKind::HorseVsField, "Field", 2.20),
        ])
    }
}
