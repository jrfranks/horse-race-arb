# Architecture

## Goals

1. Detect **surebets** across horse-racing books using decimal-odds math.
2. Emit **alerts and stake sizes** — never place bets or automate wagering.
3. Stay offline-friendly: mock books + local JSON only. No live scrape of bookmaker sites.

## Crate layout

```text
race-arb/
  src/
    lib.rs          # public API re-exports
    main.rs         # CLI: `demo`, `scan --odds …`
    types.rs        # DecimalOdds, RaceKey, Quote, Surebet, …
    math.rs         # Σ(1/odds), equalized stakes, evaluate_market
    matching.rs     # track/date/race key + runner name normalize
    scan.rs         # cross-book best-price aggregation → surebets
    adapters/
      mock.rs       # MockBookA / MockBookB (intentional misprice)
      json_file.rs  # generic local JSON
      xpressbet.rs  # stub: local JSON snapshot only
  fixtures/         # demo + sample snapshots
  tests/            # integration (demo + JSON fixture)
```

## Data flow

```text
Adapters ──► Vec<Quote>
                │
                ▼
        group by RaceKey (track+date+race#)
                │
                ▼
     per MarketKind (win | horse_vs_field)
                │
                ▼
   best decimal odds per normalized outcome
                │
                ▼
     math::evaluate_market  (arb iff S < 1)
                │
                ▼
           Surebet alerts
```

Place and Show markets are defined on `MarketKind` but **skipped** by the scanner until payoff semantics (top-N) are modeled carefully.

## Matching

- **Race**: `normalize_track(track) + date + race_number`.
- **Runner**: uppercase, strip most punctuation, collapse whitespace (`Thunder-Bay's` → `THUNDER BAYS`).

Books that spell the same horse differently still merge if normalization converges.

## Adapters & ToS

| Source | Notes |
|--------|-------|
| Mock A/B | In-process fixtures for demos/tests |
| JSON file | User-supplied export |
| Xpressbet stub | **Local file only.** Do not scrape xpressbet.com / 1/ST; that would violate ToS and is out of scope |

### Operational realities (not code bugs)

- **Account limiting**: books detect arbers; limits and closures are common.
- **Latency**: the edge on screen may be gone by the time you click.
- **Jurisdiction**: only bet where legal; this tool does not check that for you.
- **No auto-betting**: by design. Wiring this to a betting API would be a separate, high-risk project and is intentionally absent.

## Extending

1. Add an adapter that implements `OddsAdapter` and returns `Quote`s from a **permitted** local/export source.
2. Register it in the CLI `scan` path (or a new subcommand).
3. Keep Place/Show stubs until you define exhaustive outcome sets (e.g. each runner’s place price is *not* a simple mutually exclusive partition).

## Testing

- Unit tests in `math` / `matching` cover arb inequalities and name normalize.
- `tests/demo_fixture.rs` asserts MockBookA×B and `fixtures/demo_odds.json` produce surebets with consistent stake/profit math.
