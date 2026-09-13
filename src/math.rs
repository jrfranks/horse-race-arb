//! Surebet / arbitrage math on decimal odds.
//!
//! For a mutually exclusive and exhaustive set of outcomes, take the **best**
//! decimal odds available for each outcome across books. An arbitrage (surebet)
//! exists iff:
//!
//! ```text
//! sum_i (1 / best_odds_i) < 1
//! ```
//!
//! Stakes proportional to `1/odds` equalize return so every outcome pays the
//! same amount. This module never places bets — it only sizes stakes.

use crate::types::{DecimalOdds, MarketKind, Outcome, RaceKey, StakeLeg, Surebet};

/// One outcome with its best available book price.
#[derive(Debug, Clone)]
pub struct BestPrice {
    pub outcome: Outcome,
    pub book: String,
    pub odds: DecimalOdds,
}

/// Result of evaluating whether a market is an arb.
#[derive(Debug, Clone)]
pub enum ArbCheck {
    /// Not an arb (fair or overround market).
    NoArb { implied_sum: f64 },
    /// Surebet found.
    Surebet(Surebet),
}

/// Compute implied-probability sum Σ(1/odds). Arb iff result < 1.0.
pub fn implied_sum(odds: &[DecimalOdds]) -> f64 {
    odds.iter().map(|o| o.implied_prob()).sum()
}

/// True iff Σ(1/odds) < 1 for the given prices (one per exhaustive outcome).
pub fn is_arbitrage(odds: &[DecimalOdds]) -> bool {
    odds.len() >= 2 && implied_sum(odds) < 1.0
}

/// Stake for each outcome given a total bankroll, proportional to 1/odds.
///
/// `stake_i = (1/odds_i) / S * total` where `S = Σ(1/odds)`.
/// Guaranteed return = `total / S`; profit = `total * (1/S - 1)`.
pub fn equalized_stakes(odds: &[DecimalOdds], total: f64) -> Vec<f64> {
    let s = implied_sum(odds);
    if s <= 0.0 || !s.is_finite() {
        return vec![0.0; odds.len()];
    }
    odds.iter()
        .map(|o| (o.implied_prob() / s) * total)
        .collect()
}

/// Guaranteed profit for a total stake when an arb exists.
pub fn guaranteed_profit(odds: &[DecimalOdds], total: f64) -> f64 {
    let s = implied_sum(odds);
    if s <= 0.0 || !s.is_finite() {
        return 0.0;
    }
    total * (1.0 / s - 1.0)
}

/// Edge as underround percentage: `(1 - S) * 100`.
pub fn edge_pct(odds: &[DecimalOdds]) -> f64 {
    (1.0 - implied_sum(odds)) * 100.0
}

/// Build a [`Surebet`] from best prices across books for one market.
///
/// `prices` must cover a mutually exclusive exhaustive outcome set (e.g. full
/// win field, or horse + field for a two-way).
pub fn evaluate_market(
    race: RaceKey,
    market: MarketKind,
    prices: &[BestPrice],
) -> ArbCheck {
    if prices.len() < 2 {
        return ArbCheck::NoArb { implied_sum: f64::NAN };
    }

    let odds: Vec<DecimalOdds> = prices.iter().map(|p| p.odds).collect();
    let s = implied_sum(&odds);
    if !(s < 1.0) {
        return ArbCheck::NoArb { implied_sum: s };
    }

    let stakes_100 = equalized_stakes(&odds, 100.0);
    let legs: Vec<StakeLeg> = prices
        .iter()
        .zip(stakes_100.iter())
        .map(|(p, &stake)| StakeLeg {
            book: p.book.clone(),
            outcome: p.outcome.name.clone(),
            odds: p.odds.value(),
            stake,
        })
        .collect();

    ArbCheck::Surebet(Surebet {
        race,
        market,
        implied_sum: s,
        edge_pct: (1.0 - s) * 100.0,
        profit_100: guaranteed_profit(&odds, 100.0),
        profit_1000: guaranteed_profit(&odds, 1000.0),
        legs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use chrono::NaiveDate;

    fn o(v: f64) -> DecimalOdds {
        DecimalOdds::new(v).unwrap()
    }

    #[test]
    fn classic_two_way_arb() {
        // 2.10 and 2.10 → S = 1/2.1 + 1/2.1 ≈ 0.9524 < 1
        let odds = [o(2.10), o(2.10)];
        assert!(is_arbitrage(&odds));
        assert_relative_eq!(implied_sum(&odds), 2.0 / 2.10, epsilon = 1e-9);
        let profit = guaranteed_profit(&odds, 100.0);
        assert!(profit > 4.0 && profit < 6.0);
        let stakes = equalized_stakes(&odds, 100.0);
        assert_relative_eq!(stakes[0], stakes[1], epsilon = 1e-9);
        assert_relative_eq!(stakes[0] + stakes[1], 100.0, epsilon = 1e-9);
    }

    #[test]
    fn no_arb_when_overround() {
        // Typical book: 1.90 / 1.90 → S ≈ 1.0526
        let odds = [o(1.90), o(1.90)];
        assert!(!is_arbitrage(&odds));
        assert!(implied_sum(&odds) > 1.0);
        assert!(edge_pct(&odds) < 0.0);
    }

    #[test]
    fn three_way_win_field_arb() {
        // Best prices across books intentionally mispriced
        let odds = [o(3.5), o(3.5), o(3.5)];
        // S = 3/3.5 ≈ 0.857
        assert!(is_arbitrage(&odds));
        let s = implied_sum(&odds);
        assert_relative_eq!(s, 3.0 / 3.5, epsilon = 1e-9);
        let stakes = equalized_stakes(&odds, 100.0);
        assert_relative_eq!(stakes.iter().sum::<f64>(), 100.0, epsilon = 1e-9);
        // Equal odds → equal stakes
        assert_relative_eq!(stakes[0], stakes[1], epsilon = 1e-9);
        assert_relative_eq!(stakes[1], stakes[2], epsilon = 1e-9);
        // Each leg returns the same
        let ret0 = stakes[0] * odds[0].value();
        let ret1 = stakes[1] * odds[1].value();
        assert_relative_eq!(ret0, ret1, epsilon = 1e-6);
        assert_relative_eq!(ret0, 100.0 / s, epsilon = 1e-6);
    }

    #[test]
    fn evaluate_market_builds_surebet() {
        let race = RaceKey::new(
            "SA",
            NaiveDate::from_ymd_opt(2026, 9, 13).unwrap(),
            5,
        );
        let prices = vec![
            BestPrice {
                outcome: Outcome::new("Thunder Bay"),
                book: "MockA".into(),
                odds: o(2.20),
            },
            BestPrice {
                outcome: Outcome::new("Field"),
                book: "MockB".into(),
                odds: o(2.15),
            },
        ];
        match evaluate_market(race, MarketKind::HorseVsField, &prices) {
            ArbCheck::Surebet(sb) => {
                assert!(sb.implied_sum < 1.0);
                assert!(sb.edge_pct > 0.0);
                assert!(sb.profit_100 > 0.0);
                assert_relative_eq!(sb.profit_1000, sb.profit_100 * 10.0, epsilon = 1e-6);
                assert_eq!(sb.legs.len(), 2);
                assert_relative_eq!(
                    sb.legs.iter().map(|l| l.stake).sum::<f64>(),
                    100.0,
                    epsilon = 1e-9
                );
            }
            ArbCheck::NoArb { .. } => panic!("expected surebet"),
        }
    }

    #[test]
    fn reject_invalid_odds() {
        assert!(DecimalOdds::new(1.0).is_err());
        assert!(DecimalOdds::new(0.5).is_err());
        assert!(DecimalOdds::new(f64::NAN).is_err());
    }
}
