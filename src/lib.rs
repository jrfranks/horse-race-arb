//! `race-arb` — horse-race surebet / odds arbitrage scanner.
//!
//! Compares decimal odds across books and flags surebets where
//! `Σ(1/best_odds) < 1` for mutually exclusive exhaustive outcomes.
//! Emits alerts and stake sizes only — **never places bets**.

pub mod adapters;
pub mod matching;
pub mod math;
pub mod scan;
pub mod types;

pub use adapters::{JsonFileAdapter, MockBookA, MockBookB, OddsAdapter, XpressbetStub};
pub use math::{
    edge_pct, equalized_stakes, evaluate_market, guaranteed_profit, implied_sum, is_arbitrage,
    ArbCheck, BestPrice,
};
pub use scan::{run_demo, scan_adapters, scan_quotes};
pub use types::{DecimalOdds, MarketKind, Outcome, Quote, RaceKey, StakeLeg, Surebet};
