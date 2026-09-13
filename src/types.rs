//! Core domain types for horse-race odds and markets.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Decimal odds (European style). A price of 2.50 means $2.50 returned per $1 staked
/// (stake included). Must be > 1.0 for a valid betting price.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DecimalOdds(f64);

impl DecimalOdds {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if !value.is_finite() || value <= 1.0 {
            return Err("decimal odds must be finite and > 1.0");
        }
        Ok(Self(value))
    }

    /// Construct without validation (for fixtures / internal use after checks).
    pub fn from_unchecked(value: f64) -> Self {
        Self(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    /// Implied probability = 1 / odds.
    pub fn implied_prob(self) -> f64 {
        1.0 / self.0
    }
}

impl fmt::Display for DecimalOdds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

/// Market kind. Place/Show are stubbed for future work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketKind {
    /// Win: full field, one winner among all runners (mutually exclusive + exhaustive).
    Win,
    /// Two-way: named horse vs the field (or high vs low fixture).
    HorseVsField,
    /// Stub — not scanned yet.
    Place,
    /// Stub — not scanned yet.
    Show,
}

impl fmt::Display for MarketKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketKind::Win => write!(f, "win"),
            MarketKind::HorseVsField => write!(f, "horse_vs_field"),
            MarketKind::Place => write!(f, "place"),
            MarketKind::Show => write!(f, "show"),
        }
    }
}

/// Identity for a race card entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RaceKey {
    pub track: String,
    pub date: NaiveDate,
    pub race_number: u32,
}

impl RaceKey {
    pub fn new(track: impl Into<String>, date: NaiveDate, race_number: u32) -> Self {
        Self {
            track: track.into(),
            date,
            race_number,
        }
    }
}

impl fmt::Display for RaceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} R{}", self.track, self.date, self.race_number)
    }
}

/// A selectable outcome within a market (runner name, or "Field", etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Outcome {
    pub name: String,
}

impl Outcome {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// One book's price for one outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub book: String,
    pub race: RaceKey,
    pub market: MarketKind,
    pub outcome: Outcome,
    pub odds: DecimalOdds,
}

/// Stake recommendation for one leg of a surebet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakeLeg {
    pub book: String,
    pub outcome: String,
    pub odds: f64,
    pub stake: f64,
}

/// A detected surebet (arbitrage opportunity). Alerts + stakes only — never places bets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Surebet {
    pub race: RaceKey,
    pub market: MarketKind,
    /// Sum of 1/best_odds across outcomes. Arb iff this is < 1.0.
    pub implied_sum: f64,
    /// (1 - implied_sum) * 100 — underround percentage.
    pub edge_pct: f64,
    /// Guaranteed profit if total stake is $100.
    pub profit_100: f64,
    /// Guaranteed profit if total stake is $1000.
    pub profit_1000: f64,
    pub legs: Vec<StakeLeg>,
}

impl Surebet {
    pub fn format_alert(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "SUREBET {} | market={} | edge={:.2}% | Σ(1/odds)={:.4}\n",
            self.race, self.market, self.edge_pct, self.implied_sum
        ));
        out.push_str(&format!(
            "  Profit on $100: ${:.2} | on $1000: ${:.2}\n",
            self.profit_100, self.profit_1000
        ));
        out.push_str("  Stakes (for $100 total):\n");
        for leg in &self.legs {
            out.push_str(&format!(
                "    {:>8} @ {} odds {:.3} → stake ${:.2}\n",
                leg.outcome, leg.book, leg.odds, leg.stake
            ));
        }
        out.push_str("  ⚠ Alert only — do not auto-bet. Confirm legality & ToS.\n");
        out
    }
}
