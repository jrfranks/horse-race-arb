//! Race and runner matching across books.
//!
//! Matching key: normalized track + date + race number.
//! Runner names are normalized (case, punctuation, whitespace) before compare.

use crate::types::RaceKey;
use std::collections::HashMap;

/// Normalize a track code/name for matching.
pub fn normalize_track(track: &str) -> String {
    track
        .trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

/// Normalize a runner / outcome name for matching across books.
pub fn normalize_runner(name: &str) -> String {
    let upper = name.trim().to_uppercase();
    let mut out = String::with_capacity(upper.len());
    let mut prev_space = false;
    for c in upper.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_space = false;
        } else if c == '\'' {
            // Drop apostrophes without inserting a space (Bay's → BAYS).
            continue;
        } else if c.is_whitespace() || c == '-' || c == '.' {
            if !prev_space && !out.is_empty() {
                out.push(' ');
                prev_space = true;
            }
        }
        // drop other punctuation
    }
    out.trim().to_string()
}

/// Canonical race key used as a HashMap key across adapters.
pub fn canonical_race_key(race: &RaceKey) -> String {
    format!(
        "{}|{}|{}",
        normalize_track(&race.track),
        race.date,
        race.race_number
    )
}

/// Group items by canonical race key.
pub fn group_by_race<T, F>(items: impl IntoIterator<Item = T>, key_fn: F) -> HashMap<String, Vec<T>>
where
    F: Fn(&T) -> RaceKey,
{
    let mut map: HashMap<String, Vec<T>> = HashMap::new();
    for item in items {
        let k = canonical_race_key(&key_fn(&item));
        map.entry(k).or_default().push(item);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runner_normalize_punctuation() {
        assert_eq!(
            normalize_runner("  Thunder-Bay's  Dream  "),
            "THUNDER BAYS DREAM"
        );
        assert_eq!(normalize_runner("SEA BISCUIT"), normalize_runner("Sea Biscuit"));
        assert_eq!(normalize_runner("O'Connor"), "OCONNOR");
    }

    #[test]
    fn track_normalize() {
        assert_eq!(normalize_track(" sa "), "SA");
        assert_eq!(normalize_track("Santa Anita"), "SANTAANITA");
    }
}
