//! Cross-book scan: match races, pick best odds per outcome, evaluate arbs.

use crate::adapters::OddsAdapter;
use crate::matching::{canonical_race_key, normalize_runner};
use crate::math::{evaluate_market, ArbCheck, BestPrice};
use crate::types::{MarketKind, Outcome, Quote, RaceKey, Surebet};
use std::collections::HashMap;

/// Markets we currently scan. Place/Show are stubbed (skipped).
fn scannable(m: MarketKind) -> bool {
    matches!(m, MarketKind::Win | MarketKind::HorseVsField)
}

/// Collect quotes from all adapters, then find surebets.
pub fn scan_adapters(adapters: &[&dyn OddsAdapter]) -> Result<Vec<Surebet>, String> {
    let mut all = Vec::new();
    for a in adapters {
        let quotes = a.fetch_quotes().map_err(|e| format!("{}: {e}", a.name()))?;
        all.extend(quotes);
    }
    Ok(scan_quotes(&all))
}

/// Scan a flat quote list for surebets.
pub fn scan_quotes(quotes: &[Quote]) -> Vec<Surebet> {
    // Group: race_key → market → outcome_norm → list of (book, odds, display name, race)
    let mut by_race: HashMap<String, Vec<&Quote>> = HashMap::new();
    for q in quotes {
        if !scannable(q.market) {
            continue;
        }
        by_race
            .entry(canonical_race_key(&q.race))
            .or_default()
            .push(q);
    }

    let mut surebets = Vec::new();

    for (_rk, race_quotes) in by_race {
        let race = race_quotes[0].race.clone();
        let mut by_market: HashMap<MarketKind, Vec<&Quote>> = HashMap::new();
        for q in race_quotes {
            by_market.entry(q.market).or_default().push(q);
        }

        for (market, mquotes) in by_market {
            if let Some(sb) = scan_one_market(&race, market, &mquotes) {
                surebets.push(sb);
            }
        }
    }

    surebets.sort_by(|a, b| {
        b.edge_pct
            .partial_cmp(&a.edge_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    surebets
}

fn scan_one_market(race: &RaceKey, market: MarketKind, quotes: &[&Quote]) -> Option<Surebet> {
    // Best odds per normalized outcome name.
    let mut best: HashMap<String, BestPrice> = HashMap::new();
    for q in quotes {
        let key = normalize_runner(&q.outcome.name);
        let entry = best.entry(key).or_insert_with(|| BestPrice {
            outcome: Outcome::new(&q.outcome.name),
            book: q.book.clone(),
            odds: q.odds,
        });
        if q.odds.value() > entry.odds.value() {
            entry.outcome = Outcome::new(&q.outcome.name);
            entry.book = q.book.clone();
            entry.odds = q.odds;
        }
    }

    let prices: Vec<BestPrice> = best.into_values().collect();
    match evaluate_market(race.clone(), market, &prices) {
        ArbCheck::Surebet(sb) => Some(sb),
        ArbCheck::NoArb { .. } => None,
    }
}

/// Demo scan using MockBookA + MockBookB.
pub fn run_demo() -> Result<Vec<Surebet>, String> {
    use crate::adapters::{MockBookA, MockBookB};
    let a = MockBookA;
    let b = MockBookB;
    scan_adapters(&[&a, &b])
}
