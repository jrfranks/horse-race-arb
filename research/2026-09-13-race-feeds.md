# Horse Racing Data & Odds Feeds for surebet/arb scanning

**Project:** `jrfranks/horse-race-arb` (alerts-only; no auto-betting)  
**Audience:** John Franks (US, California)  
**Fetch date:** 2026-09-13  
**Scope:** Licensed / documented APIs preferred over scrapes. Facts only; unknowns marked.

---

## Critical accuracy notes for arb (read first)

### Pari-mutuel tote vs fixed odds

| Model | How price forms | Cross-book arb? |
|-------|-----------------|-----------------|
| **Pari-mutuel (tote)** | All ADWs/OTBs that commingle into the **same host pool** share one pool. Displayed “odds” are approximate until final; payout is determined by the host tote after takeout. | **Little/no true arb** on the same pool. Apparent TwinSpires vs FanDuel Racing vs TVG mismatches are usually **display latency**, not lockable edges. |
| **Fixed odds** | Book/ADW posts a price; that price (subject to limits/voids) is what you get paid. | **Where multi-book arb can exist** — different books, different prices on the same mutually exclusive outcomes. |
| **Exchange** | Peer back/lay; best for **price discovery** where available. | Arb vs books possible in theory; **Betfair Exchange is not available to US residents** (incl. California) as of 2026. |

**US ADW reality (TwinSpires, FanDuel Racing, TVG, Xpressbet, NYRA Bets, etc.):** core win/place/show and most exotics are **pari-mutuel into host pools**. Scraping or polling their UIs for “live odds” does **not** create a multi-book arb surface for US Thoroughbred/harness the way DraftKings vs FanDuel does for NFL.

**Scratches & late moves:** A locked arb dies if a runner is scratched (market voids/refactors) or fixed odds move before both legs are filled. Feeds that push **scratches/changes as fast as odds** matter as much as price latency.

**Ranking axes (separate):**
- **(A) Official race card / results accuracy** — entries, scratches, charts, payoffs.
- **(B) Multi-book fixed-odds for arb detection** — independent book prices on the same race/market.

---

## 1. Executive ranking

### Top 5 for (A) — official race card / results accuracy

| Rank | Source | Why |
|------|--------|-----|
| 1 | **Equibase / The Jockey Club Thoroughbred Data Hub (Racing API)** | Official NA Thoroughbred database; entries, scratches/changes, chart-level results, workouts. Licensed REST API. |
| 2 | **Racing Australia materials via authorised wholesalers** (BetMakers, Mediality, BettorData, News Perform, Racing & Sports) | Official AU Thoroughbred racing materials (fields, form, results) under national wholesaler framework (from 2025-07-01). |
| 3 | **AmTote Spectrum / United Tote (host tote)** | Source of truth for **commingled US pool** odds, will-pays, results settlement. Enterprise ADW/track contracts only — not a DIY consumer API. |
| 4 | **Timeform (UK/IRE form + commercial API)** | Long-standing official-quality UK/IRE form, ratings, race-day enrichment; commercial GSA / Horse Racing API (no book prices in API). |
| 5 | **Podium Racing API** (rights-cleared regions) | Global racecards/results/betting markets via REST + PUSH; full coverage requires multiple rights-owner agreements. Strong for multi-jurisdiction official cards when licensed. |

**Honorable:** DRF Pro Services / Brisnet (US PP & charts, enterprise); The Racing API (UK/IRE/HK cards+results, affordable, not “official rights” operator feed).

### Top 5 for (B) — multi-book fixed-odds arb comparison

| Rank | Source | Why |
|------|--------|-----|
| 1 | **odds-api.net Racing API** | Documented multi-book win/place for **AU/NZ/IE/GB** thoroughbred/harness/greyhound; SSE/WS streams; published pricing from $30/mo; includes arb/positive-EV endpoints on plan. **No US racing** in published racing catalog. |
| 2 | **Betfair Exchange API + Stream API** | Best **true price discovery** for UK/IRE/AUS racing where eligible; low-latency stream. **Not usable from US/CA accounts.** Live App Key £499; commercial use needs Betfair approval. |
| 3 | **The Racing API (Standard/Pro + odds)** | UK/IRE racecards with **20+ bookmaker odds**; updates every **3 minutes** today / 15 min tomorrow — usable for slow arb alerts, not tick-level. Prohibited for betting operators/sportsbooks. |
| 4 | **LSports Horse Racing API** | B2B global racing fixtures/markets (claims 140K+ fixtures/yr); enterprise sales; suited to operators needing multi-jurisdiction fixed markets. Pricing **unknown** (sales). |
| 5 | **OddsMatrix (EveryMatrix) horse racing platform** | Operator-grade racing content + odds aggregation for sportsbooks (10K+ races/mo claim); enterprise contract. Not a self-serve arb scanner feed. |

**Not ranked for (B):** The Odds API (`the-odds-api.com` / `theoddsapi.com`) — **no horse racing** in published sports catalog (2026-07-04 coverage page). OpticOdds/OddsJam — sports list does **not** include horse racing in getting-started docs. US ADW tote UIs — same-pool, not multi-book arb.

---

## 2. Comparison table (serious candidates)

| Provider | Data types | Latency / update | Coverage | Access | Rough pricing (public) | License / redistribution | Arb fit (B) | Docs / sales |
|----------|------------|------------------|----------|--------|------------------------|---------------------------|-------------|--------------|
| **Thoroughbred Data Hub (Equibase/TJC)** | Entries, ML, scratches/changes, workouts, chart results, speed figs | “As they happen” for scratches (contract); not a live tote odds feed | NA Thoroughbred | Paid commercial / incubator; non-commercial self-serve claimed | Incubator: $1,500 entry; Racing months 1–3 $500 one-time then ramping to **$3,000/mo** + 15% revenue share (indicative, contract) | Contract; commercial redistribution restricted | **(A) excellent**; **(B) poor** (no multi-book fixed odds) | https://test-hub.jockeyclub.com/ (hub fetch returned **401** on 2026-09-13); incubator https://test-hub.jockeyclub.com/apply/incubator |
| **Equibase Industry Services** | PP programs, simulcast PP, partner web | Industry electronic delivery | US + simulcast outlets | Licensed partners (tracks, ADWs, media) | **Unknown** (sales) | Official supplier; scraping ≠ licensed | Card accuracy only | https://www.equibase.com/about/industryservices.cfm |
| **AmTote Spectrum GWS** | Event info, betting info, win odds, pools/probs, will-pays; PM + fixed-odds modes | Real-time tote cycles (enterprise) | Claims **>70% NA PM pools**; global guest/host | Enterprise ADW/operator | **Unknown** | Operator licence; not public DIY | Tote truth; **not cross-book arb** | https://www.amtote.com/products-and-services |
| **United Tote ToteLink** | Wagering apps, odds/probables for CDI/UT customers | Real-time (operator) | NA tracks/OTBs (CDI-owned) | Sales to customers | **Unknown** | Enterprise | Same as tote | https://www.unitedtote.com/products/totelink-web-services/ |
| **Racing Australia wholesalers** | Nominations, acceptances, form, results, silks, etc. | Official raceday ops | AU Thoroughbred (harness/grey via other rights) | Wholesaler contract (BetMakers CoreAPI, Mediality, etc.) | **Unknown** | Must use authorised wholesaler | **(A) AU**; odds via separate books | https://racingaustralia.horse/ ; BetMakers announcement https://betmakers.com/articles/betmakers-appointed-racing-australia-data-wholesaler |
| **Betfair Exchange API** | Markets, runners, back/lay, BSP, results; Stream API | Stream: low-latency push; Delayed key: **1–180s** | UK/IRE/AUS racing + global sports (where account eligible) | Free delayed key; Live key **£499** one-off; KYC account | £499 live activation; transaction charges apply | Personal betting default; **commercial needs approval**; US residents **blocked** | **Best (B) where legal** | https://developer.betfair.com/exchange-api/ ; App keys docs |
| **odds-api.net Racing** | Racecards, runners, multi-book win/place; SSE/WS | Streaming after snapshot; polling discouraged near jump | **AU, NZ, IE, GB** horse/grey/harness — **not US** | Paid self-serve API key | Starter **$30/mo** (50K req); Builder **$90**; Live **$250**; Pro **$500** | Commercial OK on paid plans; no raw-feed resale | **Strong (B) for AU/UK/IRE** | https://odds-api.net/racing ; https://odds-api.net/pricing |
| **The Odds API** (the-odds-api.com) | Sports odds | — | **No horse racing** in sports list (updated 2026-07-04) | N/A for racing | N/A | N/A | **None for horses** | https://theoddsapi.com/sports/ |
| **The Racing API** | Racecards, results, analysis; odds on Standard+ | Today cards/odds/results **every 3 min**; tomorrow **15 min**; future daily | Core: UK/IRE/HK; AU/NA add-ons **unavailable to new subscribers** (as of fetch) | Self-serve subscription | Third-party summaries cite Free / ~$19 / ~$59 / Enterprise — **verify on site checkout** (pricing page not fully rendered in fetch) | **Forbidden for betting operators/sportsbooks** | Moderate (B) UK/IRE if allowed for personal alerts tool | https://www.theracingapi.com/ ; docs https://api.theracingapi.com/documentation |
| **Podium Racing API** | Cards, entrants, ratings, odds history, results, tote returns, PUSH raceday | PUSH for time-critical; REST otherwise | 300+ courses claim; rights per region | Enterprise sales | **Unknown** | Rights-owner agreements required for full coverage | Good if licensed multi-book markets included | https://podiumsports.com/horse-racing-api/ ; PDF guide https://podiumsports.com/wp-content/uploads/2025/05/Podium-Racing-API-User-Guide.pdf |
| **Sportradar / Betradar UOF** | Pre-match & live odds; horse racing sport id 55; fixtures/stages | AMQP push odds | Operator coverage (incl. major races); virtual HR too | Enterprise sportsbook | **Unknown** (large B2B) | Operator licence | Operator odds feed — may not expose multi-book consumer prices | https://docs.sportradar.com/uof/ ; https://betradar.com/sportsbook-support/unified-odds-feed/ |
| **OddsMatrix / EveryMatrix** | Managed racing platform + odds aggregation | Operator real-time | 20+ jurisdictions claim; 10K+ races/mo claim | Enterprise | **Unknown** | Operator | B2B only | https://everymatrix.com/oddsmatrix/horse-racing/ |
| **LSports** | Fixtures, ante-post, place markets, settlement | Claims near-zero latency (marketing) | Global; major calendar events | Enterprise / free trial sales | **Unknown** | B2B | Potential (B) if multi-book or rich fixed markets | https://www.lsports.eu/horse-racing-api/ |
| **Genius Sports** | Odds APIs for sportsbooks | Low-latency in-play (sports) | Official league rights emphasis; **horse racing not highlighted** in public odds-API pages | Enterprise | **Unknown** | Operator | **Unknown** for racing | https://www.geniussports.com/bet/odds-feeds-api/ |
| **OpticOdds / OddsJam** | Multi-book sports odds, SSE | Real-time | Published sports list: baseball, basketball, football, soccer, etc. — **horse racing not listed** | Paid | **Unknown** for racing (N/A) | Commercial | **No confirmed racing** | https://developer.opticodds.com/docs/odds-api-getting-started-guide |
| **OddsChecker** | Multi-book comparison UI | Real-time web | Strong UK racing | **No public API**; scrape/ToS risk; unofficial Parse wrappers exist | N/A official | Affiliate site, not data vendor | Avoid scrape | https://www.oddschecker.com/ |
| **RebelBetting** | Surebet/value software (incl. horse racing sure betting) | Product-dependent | 90+ books; HR in sure betting not value | Consumer SaaS — **not a raw data API** for embedding | Product pricing on site | Software ToS | Competitive product, not a feed | https://www.rebelbetting.com/en-us/bookmakers |
| **Timeform API / GSA** | Pre/race/post cards, ratings, comments, stats — **no bookmaker prices** | Commercial SLA **unknown** | UK/IRE heritage; global products via GSA | Commercial only | **Unknown** (commercial@timeform.com) | Commercial packages only | **(A)/(form)** not (B) | https://api.timeform.com/horseracingapi/ ; https://www.timeform.com/commercial/products |
| **DRF Pro Services** | APIs/feeds, handicapping, charts, ratings; historical DRF Data Services API (Beyer/TimeformUS) | Real-time claimed for partner APIs | US Thoroughbred (+ harness partners) | Sales (rforbeck@drf.com) | **Unknown** | Partner licence | Form/card; not multi-book arb | https://promos.drf.com/services |
| **Brisnet** | PP files, programs, figures | Retail/product cadence | US tracks | Retail / industry (Equibase ecosystem) | Retail PP pricing varies; **API pricing unknown** | Product ToS; no public open API found | Form accuracy | https://www.brisnet.com/ |
| **BloodHorse** | News, pedigrees, editorial | — | Industry | No public racing odds API found | N/A | — | Not an odds feed | — |
| **TwinSpires / FanDuel Racing / TVG / Xpressbet / DK Racing** | UI: entries, tote odds, pools, video | UI refresh; same host pool | US ADW + international simulcast (varies) | **No public wagering/odds API** for consumers | N/A | **ToS forbid scrape/automation** typically | **Tote ≠ arb**; do not scrape | Product sites only |
| **Sportmonks** | Football/cricket etc. | — | **No horse racing API** found | N/A | N/A | — | None | https://www.sportmonks.com/ |
| **HorseRaceDatabase** | Historical UK/IRE/HK (+ USA plans “coming soon”) | API products may be marked coming soon | HK/UK focus | Paid datasets | e.g. historical unlock marketing **€590/mo** (verify) | No raw redistribution without agreement | Historical research | https://horseracedatabase.com/api/ |

---

## 3. Category deep dives

### 3.1 Official / industry primary sources

**Equibase / The Jockey Club**  
- Official supplier of racing information/statistics to America’s Best Racing, Breeders’ Cup, Brisnet, DRF, FanDuel Racing, NTRA, TJC, TRA, TwinSpires, 1/ST (per Equibase industry page).  
- **Thoroughbred Data Hub** exposes Pedigree + Racing REST APIs (`/v1/entries`, `/v1/results`, `/v1/workouts`, `/v1/tracks`) with scratches/changes. Hub homepage returned **HTTP 401** via WebFetch on 2026-09-13 — treat sales/docs as login-gated.  
- Fit: **canonical (A)** for US Thoroughbred identity, entries, scratches, charts. Does **not** replace multi-book fixed odds for (B).

**AmTote (1/ST) Spectrum**  
- Processes majority of NA pari-mutuel handle; GWS API for event/betting informational feeds and transactional ADW integration; supports PM **and** fixed-odds modes.  
- Fit: source of **tote board truth** for host pools. Wrong tool for cross-ADW arb on the same pool.

**United Tote (Churchill Downs Inc.)**  
- ToteLink Web Services for customer wagering apps; EnterBet shows odds/probables. Contact: sales@unitedtote.com.  
- Fit: same as AmTote — host pool infrastructure.

**Racing Australia**  
- Does not sell Materials direct as wholesaler anymore; five authorised wholesalers from **1 July 2025**. BetMakers CoreAPI advertises official nominations/acceptances/trials/results/form.  
- Fit: required path for **licensed AU card accuracy**.

**Betfair Exchange (UK/IRE/AUS)**  
- Documented Betting API + Exchange Stream API; Horse Racing is a first-class event type with venues.  
- **US/California:** Exchange not available to US residents; NJ experiment ended years ago. Do not plan Betfair as a CA-legal wagering venue.  
- Fit: if John ever operates from an eligible jurisdiction or uses delayed data only for research under Betfair ToS — outstanding (B) discovery; otherwise out of stack for live CA use.

**NTRA**  
- Industry association / marketing; not a primary odds API vendor. Equibase supplies them data.

### 3.2 Commercial odds aggregation APIs

| Name | Horse racing? | Notes |
|------|---------------|-------|
| **The Odds API** (the-odds-api.com) | **No** | 26 sports listed; no racing keys. |
| **odds-api.net** | **Yes** (AU/NZ/IE/GB) | Distinct product from The Odds API. Best self-serve multi-book racing odds found. |
| **OpticOdds / OddsJam** | **Not confirmed** | Getting-started sports list omits horse racing. |
| **OddsChecker** | UI yes / API **no** | Do not scrape. |
| **RebelBetting** | Product yes | Closed software, not embeddable feed. |
| **Sportmonks / API-Football** | **No racing** | Football/cricket focus. |
| **Sportradar UOF / OddsMatrix / LSports / Genius** | Operator racing products | Enterprise; pricing unknown; aimed at sportsbooks not indie scanners. |

### 3.3 Form / past performance / entries

| Source | Strength for runners/scratches | Prices? |
|--------|--------------------------------|---------|
| Equibase / Data Hub | Highest for US official entries/scratches/charts | Morning line; not live multi-book |
| DRF / TimeformUS / Brisnet | Excellent PP & figures; partner APIs | Not arb prices |
| Timeform UK | Excellent UK/IRE form & ratings | Explicitly **no bookmaker prices** in API |
| Racing Post | Industry standard UK cards (commercial syndication **unknown** in this research) | Comparison site / media |
| Podium | Cards + Timeform ratings in entrant payloads (per onboarding PDF) | Odds history / tote returns in PUSH |
| The Racing API | Good UK/IRE cards + odds on higher plans | Book odds yes (latency minutes) |

**Accuracy note:** Prefer Equibase/RA wholesaler for **who runs**; prefer fixed-odds aggregators for **what price**. Mixing tote approximate odds into an arb Σ(1/odds) calculation is a design error.

### 3.4 ADW / book-specific (US)

| ADW | Public API? | Authenticated API? | Scrape risk |
|-----|-------------|--------------------|-------------|
| TwinSpires | No | Not offered to consumers | High ToS / bot risk |
| FanDuel Racing | No | No public | High |
| TVG | No | No public | High |
| Xpressbet | No | Industry rumor of volume-gated partner APIs — **unverified** | High |
| DraftKings Racing | No public racing odds API found | Unknown | High |
| AmWager / Watch & Wager | Historical forum claims of volume APIs — **unverified** | Contact sales | Still ToS-bound |

**Recommendation:** Treat US ADWs as **manual confirmation / execution venues**, not as automated price feeds. `horse-race-arb` should ingest licensed aggregator JSON, then alert; human places bets.

### 3.5 Exchange / peer prices

- **Betfair Exchange:** gold standard for UK/IRE/AUS racing liquidity and discovery. Stream API for low latency. Delayed App Key free (delayed prices); Live £499. Commercial redistribution needs Betfair approval.  
- **US exchanges (Novig, ProphetX, etc.):** sports-oriented; **not** substitutes for Thoroughbred tote/racing depth as of this research.  
- **CA user constraint:** cannot rely on Betfair for production alerts that assume fillable exchange prices.

---

## 4. Recommended stack for `horse-race-arb` (phased)

### Phase 0 — Architecture guardrails (now)

1. Schema already separates `book` + decimal odds — keep **strict market type tags**: `fixed_win` vs `tote_approx` vs `exchange`.  
2. Never compute surebets across legs that are **tote approximations of the same host pool**.  
3. Require **scratch/void** channel before alerting near post; invalidate arb if runner status ≠ active.  
4. Alerts-only (already): no place-bet APIs.

### Phase 1 — Free / cheap prototype (weeks)

| Layer | Choice | Role |
|-------|--------|------|
| Cards/results (UK/IRE) | **The Racing API** Free→Basic | Schedule, runners, results; learn IDs |
| Fixed odds (AU/UK/IRE) | **odds-api.net** Starter **$30/mo** | Multi-book win/place snapshots + stream; wire into existing scanner JSON |
| Demo/fixtures | Keep mock + local JSON | Regression tests |
| US cards (optional cheap) | Equibase public pages **view-only** / paid PP retail — **no scrape automation** | Manual QA of entries |

**Geographic honesty:** Phase 1 arb discovery will be **strongest on AU/GB/IE fixed-odds books**, not California ADW tote. That still validates the scanner math and alert pipeline.

### Phase 2 — Paid accuracy (US-focused card layer)

| Layer | Choice | Role |
|-------|--------|------|
| US official cards | **Thoroughbred Data Hub Racing API** (Incubator if product qualifies) | Entries, scratches, results as system of record |
| Form enrichment | DRF Pro or Brisnet partner feed | PP context in alerts (optional) |
| Odds | Continue odds-api.net; evaluate LSports/Podium sales for broader fixed-odds | Expand books/jurisdictions |

### Phase 3 — Enterprise / if expanding jurisdictions or productizing

- **Racing Australia** via BetMakers or Mediality for AU official Materials.  
- **Podium** or **LSports** for global cards + markets under contract.  
- **Sportradar / OddsMatrix** only if becoming an operator or white-label — overkill for personal alerts.  
- **Betfair** only if account/jurisdiction allows and commercial terms clear — never for CA-resident Exchange use.

### Explicit non-goals / rejects

- Scraping TwinSpires/TVG/Xpressbet/OddsChecker.  
- Using The Odds API or OpticOdds expecting horse racing (not offered).  
- Treating tote board diffs across ADWs as surebets.  
- Auto-betting or ADW credential automation.

---

## 5. Fit matrix for John’s CA use case

| Goal | Realistic path |
|------|----------------|
| Accurate US entries/scratches/results | Equibase / Data Hub licence |
| US ADW tote “odds” monitoring | Low value for arb; same pool |
| Lockable multi-book horse arbs while in CA | Focus on **offshore/international fixed-odds jurisdictions you can legally bet** (legal review required) **or** accept that US Thoroughbred is mostly **not** an arb sport under PM rules |
| Best price discovery globally | Betfair Exchange — **not available in US** |
| Cheap pipeline to prove software | odds-api.net ($30) + The Racing API |

**Legal note (non-advice):** California ADW is legal for licensed operators; which international fixed-odds books a CA resident may use is a **separate compliance question**. Feed choice ≠ wagering legality. Keep alerts informational until counsel clears venues.

---

## 6. Sources (URLs) — fetch date 2026-09-13

| # | Source | URL | Notes |
|---|--------|-----|-------|
| 1 | Thoroughbred Data Hub | https://test-hub.jockeyclub.com/ | WebFetch **401** |
| 2 | Data Hub Incubator pricing | https://test-hub.jockeyclub.com/apply/incubator | Pricing table fetched via search snippets + related pages |
| 3 | Equibase Industry Services | https://www.equibase.com/about/industryservices.cfm | |
| 4 | AmTote products | https://www.amtote.com/products-and-services | |
| 5 | United Tote ToteLink | https://www.unitedtote.com/products/totelink-web-services/ | |
| 6 | Racing Australia services | https://racingaustralia.horse/aboutus/services.aspx | |
| 7 | RA wholesaler release PDF | https://www.racingaustralia.horse/uploadimg/media-releases/Racing-Materials-distribution-Wholesaler-Agreement.pdf | |
| 8 | BetMakers RA wholesaler | https://betmakers.com/articles/betmakers-appointed-racing-australia-data-wholesaler | |
| 9 | Mediality Racing | https://medialityracing.com.au/ | |
| 10 | Betfair Exchange API | https://developer.betfair.com/exchange-api/ | |
| 11 | Betfair App Keys | https://betfair-developer-docs.atlassian.net/wiki/spaces/1smk3cen4v3lu3yomq5qye0ni/pages/2687105/Application+Keys | £499 live key |
| 12 | Betfair API costs | https://support.developer.betfair.com/hc/en-us/articles/115003864531-Are-there-any-costs-associated-with-API-access | |
| 13 | odds-api.net Racing | https://odds-api.net/racing | |
| 14 | odds-api.net Pricing | https://odds-api.net/pricing | $30–$500 plans |
| 15 | The Odds API sports | https://theoddsapi.com/sports/ | No horse racing (updated 2026-07-04) |
| 16 | The Racing API | https://www.theracingapi.com/ | 3-min updates; operator use prohibited |
| 17 | The Racing API docs | https://api.theracingapi.com/documentation | WebFetch **422** |
| 18 | Podium Racing API | https://podiumsports.com/horse-racing-api/ | |
| 19 | Podium API user guide PDF | https://podiumsports.com/wp-content/uploads/2025/05/Podium-Racing-API-User-Guide.pdf | |
| 20 | Sportradar UOF horse racing | https://docs.sportradar.com/uof/api-and-structure/api-stage-structure/horse-racing | |
| 21 | Betradar UOF | https://betradar.com/sportsbook-support/unified-odds-feed/ | |
| 22 | OddsMatrix racing | https://everymatrix.com/oddsmatrix/horse-racing/ | |
| 23 | LSports horse racing | https://www.lsports.eu/horse-racing-api/ | |
| 24 | Genius Sports odds APIs | https://www.geniussports.com/bet/odds-feeds-api/ | |
| 25 | OpticOdds getting started | https://developer.opticodds.com/docs/odds-api-getting-started-guide | No HR in sports list |
| 26 | Timeform commercial | https://www.timeform.com/commercial/products | No book prices in API |
| 27 | Timeform Horse Racing API | https://api.timeform.com/horseracingapi/ | |
| 28 | DRF Pro Services | https://promos.drf.com/services | |
| 29 | RebelBetting bookmakers | https://www.rebelbetting.com/en-us/bookmakers | HR in sure betting |
| 30 | TwinSpires wager FAQ (pari-mutuel) | https://support.twinspires.com/hc/en-us/articles/360055533551-Wager-Types-and-Cost-FAQ | |

---

## 7. Bottom line

1. **Most accurate US race data (A):** licensed **Equibase / Thoroughbred Data Hub**.  
2. **Best multi-book fixed-odds feed you can buy today without enterprise sales (B):** **odds-api.net Racing** (AU/NZ/IE/GB) at **$30+/mo**.  
3. **US ADW tote is the wrong arb surface** — same host pools; alerts would be false positives.  
4. **Betfair Exchange** is the best discovery feed **where legal**; **not for California residents**.  
5. **Phased stack:** odds-api.net + The Racing API → Data Hub for US cards → wholesaler/enterprise only if productizing or expanding AU/global rights.

*Report generated 2026-09-13 from public web sources via WebSearch/WebFetch. Pricing not invented; unknowns left marked. No login-gated scrapes performed.*
