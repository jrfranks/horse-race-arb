# horse-race-arb (`race-arb`)

Horse-race **surebet / odds arbitrage** scanner. Compares decimal odds across books and alerts when stakes can lock a profit regardless of outcome.

**Alerts + stake sizes only — this tool never places bets.**

## Surebet math

For a mutually exclusive, exhaustive set of outcomes, take the **best** decimal odds available for each outcome across books. An arbitrage exists iff:

```text
Σ (1 / best_decimal_odds_i) < 1
```

Stakes are proportional to `1/odds` so every outcome returns the same amount:

```text
stake_i = (1/odds_i) / S × total_stake
profit  = total_stake × (1/S − 1)
```

where `S = Σ(1/odds_i)`. The scanner reports edge % (`(1−S)×100`), profit on **$100** and **$1000**, and per-leg stakes.

## Markets

| Market | Status |
|--------|--------|
| **Win** (full field) | Scanned |
| **Horse-vs-field** / two-way high-low | Scanned |
| Place / Show | Stubbed (not scanned yet) |

## Quick start

```bash
# Requires Rust 1.70+
cargo build --release

# Built-in demo with MockBookA / MockBookB (intentional misprices)
cargo run -- demo

# Scan a local odds JSON file
cargo run -- scan --odds fixtures/demo_odds.json

# Optionally merge a local Xpressbet *snapshot* (no live scrape)
cargo run -- scan --odds fixtures/demo_odds.json --xpressbet fixtures/xpressbet_sample.json
```

### Sample demo output

```text
Found 2 surebet(s):

─── #1 ───
SUREBET SA 2026-09-13 R5 | market=win | edge=22.10% | Σ(1/odds)=0.7790
  Profit on $100: $28.36 | on $1000: $283.65
  Stakes (for $100 total):
    Meadow Lark @ MockBookA odds 4.200 → stake $30.56
    Coastal Fog @ MockBookB odds 3.600 → stake $35.66
    Thunder Bay @ MockBookA odds 3.800 → stake $33.78
  ⚠ Alert only — do not auto-bet. Confirm legality & ToS.

─── #2 ───
SUREBET SA 2026-09-13 R5 | market=horse_vs_field | edge=10.10% | Σ(1/odds)=0.8990
  Profit on $100: $11.24 | on $1000: $112.36
  Stakes (for $100 total):
       Field @ MockBookB odds 2.200 → stake $50.56
    Thunder Bay @ MockBookA odds 2.250 → stake $49.44
  ⚠ Alert only — do not auto-bet. Confirm legality & ToS.
```

## Odds JSON schema

```json
[
  {
    "book": "BookAlpha",
    "track": "SA",
    "date": "2026-09-13",
    "race_number": 5,
    "market": "win",
    "outcome": "Thunder Bay",
    "odds": 3.8
  }
]
```

`market` is one of: `win`, `horse_vs_field`, `place`, `show`.

Matching key: **track + date + race number**. Runner names are normalized (case / punctuation) before compare.

## Adapters

| Adapter | Behavior |
|---------|----------|
| `MockBookA` / `MockBookB` | Synthetic books with intentional cross-book misprices for `demo` |
| `JsonFileAdapter` | Reads the schema above from disk |
| `XpressbetStub` | Reads a **local JSON snapshot only** — does **not** scrape Xpressbet or any bookmaker |

There are **no live odds feeds** and **no scrapers** in this repo.

## Legal / ToS caveats

- Only wager where it is **legal** for you to do so.
- Bookmaker terms of service often **restrict or ban** arbers; accounts get limited or closed.
- Odds move; **latency** and stake limits eat theoretical edge.
- This software is for education / research alerts. **You** are responsible for compliance.

See [ARCHITECTURE.md](ARCHITECTURE.md) for design notes.

## Develop

```bash
cargo test
cargo run -- demo
```

CI runs `cargo test` on push via GitHub Actions.

## License

MIT — see [LICENSE](LICENSE).
