# Singularity 2027: Economy Model & Trade Book

**Prepared:** 2026-07-24 (overnight deep-research run)
**Thesis (given):** software singularity 2027 — AI meets/exceeds expert human
capability across cognitive work with task costs 100–1000x below wages;
general-purpose robotics ramps from 2028.
**Process:** 17-agent research sweep (562 web lookups) → calibrated bottleneck
simulation, later rebuilt as a Rust systems-dynamics engine (47 tests incl. loop ablations and conclusion locks) → 45-candidate
screen → per-ticker financial verification → adversarial red team (every trade
attacked on priced-in, thesis-failure, and structural lenses).

> **This is scenario analysis conditional on the stated thesis, not investment
> advice.** Position sizing must reflect that the thesis itself is the largest
> risk: prediction markets imply the marginal investor carries only ~20–35%
> probability of anything like a 2027 singularity. All expected values below
> are **conditional on the thesis** — the scenario weights (baseline 35% /
> fast-takeoff 15% / delayed 25% / friction 15% / fizzle 10%) put ~90% on
> singularity variants by construction. At an unconditional p(thesis) of ~30%,
> mentally triple the fizzle weight: most longs' expected upside compresses
> ~40-60% and the shorts weaken more — which is why every short carries
> structural discipline (spreads, gates, pairs).

---

## 1. The model and what it robustly says

The model is deliberately not a GDP forecaster. It is a bottleneck-accounting
engine: each year, desired AI expansion collides with four constraints —
**chips, power, capital, adoption friction** — and the binding constraint caps
growth and earns scarcity rents. Company earnings are then driven by mapped
profit pools (`rust/singularity-econ/src/valuation.rs`, `companies.rs`), valued at a punitive 12%
discount rate across five scenarios (baseline / fast-takeoff / delayed /
friction / fizzle).

Findings that survive the 800-run Monte Carlo over wide parameter priors:

1. **Power is the constraint of the decade.** It binds in 84% of runs in 2026
   and 60–81% of runs every year through 2033. The research base: gas turbines
   sold out through 2030 (GEV selling 2031 slots at +10–20% pricing),
   large-transformer lead times 3–5 years, MV switchgear sold out through
   2028, interconnection queues 5–10 years, Gartner projecting 40% of AI
   datacenters power-constrained by 2027. **Scarcity rents concentrate in
   energized capacity and the equipment that creates it.**
2. **Chips get a secondary window (2027–2030, ~25–33% of runs)** — CoWoS
   packaging, HBM stacking, and EUV tool output are the specific chokepoints,
   after which capacity growth catches up.
3. **Capital becomes the binding constraint late (2034+, reaching ~45–60% of runs by 2035–36)** —
   the boom ends not with demand exhaustion but with financing discipline;
   already visible in embryo (Oracle –49.5% on credit fears, first negative
   FCF quarter at Alphabet, $250B+ AI bonds outstanding).
4. **The "Cisco moment" (AI capex growth <15%) has a median arrival of 2036,
   p10 2034, under the thesis.** If the singularity is real, the
   picks-and-shovels trade has *years* of runway; the early-deceleration tail
   is precisely the fizzle worlds. Discipline comes from scenario weights, not
   from timing the top.
5. **Casualty decay is back-half loaded:** median cognitive-work displacement
   ~7% (2028) → ~25% (2030) → ~50% (2032); median IT-services pool –32% by
   2033; BPO down ~60%+. History (newspapers 2005–09: stocks –80–90% on –25%
   revenue) says multiples die 2–4 years before revenue — **under the thesis,
   mid-2026 is "early," which is exactly when shorts must be placed.**
6. **Robotics is a component story now, a labor story later.** Median 2032
   robot production ~0.8M units/yr (p90 2.4M); physical-labor displacement <5%
   even by 2036. Component chains (reducers, actuators, magnets) must inflect
   2–4 years before the labor market notices — but pure plays priced at 400x
   already assume the p90.
7. **Transition-recession tail:** worst-year world GDP growth goes negative in
   ~10% of runs as displaced labor income outruns recycled demand. **At
   inception this tail is only partially hedged**: the credit shorts (ALLY,
   SYF-via-OMF) and the steepener are trigger-armed, not live. The always-on
   protection is the GLD allocation and the defined-risk structure of the
   shorts; treat the recession tail as accepted risk until the credit
   triggers arm.

## 1b. Model v2 addendum (Meadows systems-dynamics rebuild)

After the book shipped, the model was rebuilt as a formal systems-dynamics
artifact (`rust/singularity-econ`, snapshot-regression
tested; design in `meadows_design.md`): supply growth, capex behavior, credit
conditions, and adoption friction are now **endogenous feedback loops with
explicit construction delays**, not assumed caps. Every named loop passes an
ablation test. 10k-run Monte Carlo (30ms in Rust) and a 20k-run sensitivity
study sharpen the book's foundations:

- **Power rents never normalize within the horizon in any sampled world** —
  stronger than v1's claim. Power binds ~96–100% of runs through 2034.
- **Silicon rent duration is regime-dependent and bimodal** (median
  normalizes 2032; the p90 tail never does): it emerges from the
  supply-response gain — the formal version of the IP-moat vs
  commodity-capacity distinction. An explicit IP-toll sector (no supply
  response) holds peak margins through 2036 while commodity silicon decays
  in a damped hog-cycle — the design's headline contrast, now implemented
  and adversarially code-reviewed (12 findings fixed, incl. a utilization
  double-count that had overstated rent persistence).
- **Median 2036 silicon capacity ends at ~1.37x demand — an endogenous glut
  even under the thesis** — sharpening the rule: harvest commodity-capacity
  semis into strength; only IP tolls are decade holds.
- **Credit crunch occurs in ~33% of runs**, driven almost entirely by the
  externally-funded share of capex (Spearman ρ=+0.65) — the circular-financing
  risk (§5b) now has a model probability and a leading indicator.
- **Peak desired/deliverable queue ratio ~2.3x** — sold-out-through-2030
  order books are the model's natural state, not an anomaly.
- **Cross-coupling discovery: power scarcity shields silicon rents**
  (power-supply ceiling ρ=+0.23 on chip-rent duration). Permitting/queue
  reform would rotate rents from power stocks to silicon stocks — a
  monitorable regime switch added to §8.
- **Displacement is timing × integration friction** (singularity year ρ=−0.62,
  adoption half-life −0.61; capability magnitude ρ=0.01): casualty shorts are
  bets on *when*, which is why every one of them carries a data trigger.
- Robot production 2032 median drops further, to ~0.34M units/yr — the
  robotics-leg patience discipline strengthens.

**Cross-model validation of the book** (regression-locked in the crate's
`tests/book.rs` conclusion locks): running the trade valuations on v2 preserves every
core conclusion — semi/power longs strongly positive, wage-linked shorts
deeply negative — and *strengthens* the power-generation legs (electricity
prices now rise endogenously with scarcity: EQT +338%, VST +326% v2 EVs).
The one family that flips is robotics-linked longs: MP +314%→−59%,
Nabtesco +540%→−14%, Symbotic +96%→−38% — v2's endogenous ramp independently
reproduces the red team's kills via a different mechanism. **Book change: MP
is demoted from "half now, half on flush" to hold-only — keep the staged
option position if already entered, do not add; the DoD-floor story needs the
p90 robot path to clear a 161x multiple.** Harmonic Drive gains a fundamental
short case under v2 (even the thesis can't carry 145x through the endogenous
component-pool path), though the squeeze-mechanics kill on the outright short
stands.

**R4 addendum (superintelligence-aggressive baseline):** on challenge, the
model gained an R4 physical-acceleration loop — post-2027, ASI compresses
construction pipelines, raises scaling ceilings, and dissolves robot
integration friction (design: `meadows_design.md` §R4). Result: **power still
binds in ~95% of runs and never normalizes in any draw** — the book's anchor
conclusion survives ASI-speed construction, because acceleration feeds
compute demand as fast as supply. Robot production rises ~25% at the 2032
median (to ~0.4M/yr) — directionally toward the faster-robots view, still an
order of magnitude short of making this decade a labor story.

Full leverage-point analysis (Meadows' 12 points with empirical gains and
three new monitorables): `leverage_points.md`. Build system: `BUILDING.md`
(Buck2 + cargo, all suites green).

## 2. What the market already prices (July 2026)

The market is mid-correction, not euphoric — this materially improves the
entry point for thesis-consistent trades:

- **Crowding is concentrated in semis** (record hedge-fund chipmaker exposure)
  yet the multiples are not bubbly: NVDA ~21x forward, TSM 19.5x, AVGO 24.4x.
  The risk in these names is the E, not the P/E (the Cisco lesson: revenue
  peaked 3 quarters *after* the stock).
- **Capex payers have been punished**: MSFT –19% YTD (20.6x fwd), GOOGL sold
  off on raising capex, META 18.6x, ORCL halved to 14.9x. If 2027 agent
  revenue vindicates 2026 capex, these are the *neglected* thesis longs.
- **First-order casualties are already dead**: Chegg $0.97, Concentrix –63%
  (1.9x fwd), EPAM –52%, Salesforce –49% (11.3x fwd), Adobe 8.2x. The market
  reprices casualties within 2–4 quarters of evidence. **The remaining short
  alpha is only in names still priced as durable** — though the red team
  subsequently killed most single-name expressions of even this (ADP's
  employment elasticity is empirically <0.5; PAYX's SMB base is physical-labor
  heavy; MAN is a 19.9%-SI squeeze trap). What survived: RHI via put spreads,
  HUBS, CHRW post-print. The kill discipline of §5 applies here first.
- **The bond market prices no singularity**: 10y at 4.71%, negative simple
  ERP, S&P short interest at record 3.7% of float — a hedged melt-up.
- **Private marks price the thesis far harder than public markets** (Anthropic
  $965B at ~21x ARR, OpenAI $852B, ~$2T of lab marks needing IPO exits
  Oct-2026/2027) — both a validation signal and the cycle's key event risk.

## 3. Historical value-capture laws applied

From six technology revolutions (rail, electrification, Wintel, telecom/dotcom,
mobile, shale), three laws govern the book's construction:

1. **Rent duration is determined by moat type, not shortage intensity.**
   Capacity-scarcity rents (fiber 1999, DRAM every cycle, cooling) collapse
   within 1–3 years of the supply response. IP/standard/ecosystem rents (x86,
   ASML litho, CUDA, TSMC process leadership) persist for decades.
   → Own TSM/ASML-class tolls for the decade; own memory/optics scarcity
   tactically and exit into the supply response (capacity lands ~2027-28).
2. **Terminal value accrues to aggregators, not infrastructure.** Amazon and
   Google were built ON the dotcom glut, after it. Under the thesis, the 2030s
   aggregators are whoever owns customer relationships + the agent data loop —
   at current prices that argument points at the punished hyperscalers, not at
   new infrastructure names at peak multiples.
3. **Overshoot tells:** late-cycle debt/SPV financing, vendor financing,
   circular revenue, receivables outrunning sales, second-tier players at
   first-tier multiples. Several are already flashing (OpenAI's $1.4T→$600B
   pledge walk-back; $120B+ off-balance-sheet DC SPVs; neocloud GPU-backed
   debt at 6-year depreciation on 2–3 year silicon). The levered periphery
   (neoclouds, credit-funded SPVs) is where the bust concentrates if it comes —
   the cash-funded core is not telecom 2000.

## 4. The book

Sizing buckets are relative (full / medium / starter / option-like); absolute
sizing depends on your portfolio and, above all, on your true probability for
the thesis. Every entry below survived a dedicated adversarial review; entry
rules and invalidation triggers come from those reviews.

### Core longs (full size)

| # | Ticker | Trade | Why it survives everything | Entry / invalidation |
|---|--------|-------|---------------------------|---------------------|
| 1 | **TSM** | Long TSMC | The convergence pick: named by the red team as the better expression of the ASML, MU, AND SK Hynix theses. Owns both 2026 chokepoints (N2 wafers, CoWoS packaging), 4 years of committed price hikes, 67.7% GM, net cash $69B — at 19.5x fwd with ~27% consensus EPS CAGR. IP-moat rent class (persists decades, per history). | Round 2: half-weight AND rolling ~20%-OTM put overlay (not either/or); prefer the 2330.TW local line — the ADR still embeds ~14% of meltable premium (26%→13.7% in five months). Haircut EPS quality: Section 232 tariff costs, US-fab margin dilution (3-4pts), NTD risk (~0.4pt OM per 1%); size on FCF yield (FCF -17.5% yoy at record earnings), not P/E. Invalidation: CoWoS gap closing with pricing-power rollback. |
| 2 | **GOOGL** | Long Alphabet | DEMOTED to medium by round 2: the "self-funded" pillar is dead — June 2026 ~$80-85B equity raise, buybacks at $0 (first time since 2017), FCF margin 21%→9.2%. Assets intact (TPU, DeepMind, Waymo, Cloud +82%); Berkshire anchored the raise. But Anthropic is now >40% of the $462B Cloud backlog while Google invests up to $40B into Anthropic — circular financing at the fund's own #2 long. | Dips are now also equity-supply events — no automatic adds. Invalidation: a second equity raise without buyback resumption; any Anthropic markdown or IPO delay (that event risk IS the backlog's credit quality). Restore to core when buybacks resume or Anthropic is a rated public counterparty. |
| 3 | **MSFT** | Long Microsoft | DEMOTED to medium by round 2: OpenAI is 45% of the $625B commercial RPO and its spend is vendor-financed — the Azure kill trigger was contaminated (watching a contractual floor, not demand). The April-2026 amendment capped the OpenAI revenue share and terminates IP/API exclusivity on verified AGI — under this fund's own thesis, capped upside with retained per-seat downside. Distribution + the 27% OpenAI stake keep it a long. | Half now, half post-print. NEW invalidation metric: ex-OpenAI commercial RPO growth <20% (currently 26%, decelerating) — not headline Azure. |
| 4 | **AVGO** | Long Broadcom | CUT to half-size by round 2: on the marginal AI dollar Broadcom is becoming a vendor-financier, not a royalty collector — ~$30B of SPV lease guarantees, an $18B OpenAI phase-1 financing snag, and the make-or-break Microsoft ~40% offtake NOT signed. Disclosed AI GM ~65%, not 75%, falling on rack mix. The backlog is real; its credit quality is not uniform. | Haircut the OpenAI-linked 30-50% of backlog until MSFT signs the offtake or OpenAI closes phase-1 financing on its own paper — those are the re-upsize triggers. Kill: SPV guarantees growing faster than AI revenue; a second-gen re-compete loss. |
| 5 | **CEG** | Long Constellation | CUT to half-size by round 2: the modeled company no longer exists — Calpine closed Jan 7, 2026 (fleet ~55GW, roughly half competitive gas; +16% share count). The "cheap 24x" is paid partly on spark-spread gas earnings that deserve 10-14x, and the Illinois collar refunds merchant upside on ~6.6GW (~$1.9B already credited back). The nuclear-PPA thesis itself is intact (Crane on track, Walmart added). | Suspend adds until the DOJ/FERC 4.4GW divestiture clears and the Calpine sponsors' lockup passes. Underwrite zero merchant upside on the collared Illinois units. Kill: Crane slips past 2027. |

### Second-line longs (medium)

| # | Ticker | Trade | Notes | Entry / invalidation |
|---|--------|-------|-------|---------------------|
| 6 | **NVDA** | Long NVIDIA | 21x fwd prices deceleration; thesis makes the TAM 2-4x consensus. Share loss to ASICs is real but inside a pool growing faster. The risk is the E in fizzle worlds — which scenario weights already price. | Enter now, medium size (crowding). Invalidation: two consecutive hyperscaler capex guidance cuts. |
| 7 | **VST** | Long Vistra | Outright merchant-scarcity long (NOT a CEG-convergence trade — the discount is structural: $19.9B net debt, no PPAs). | Now; add on theme selloffs with ERCOT/PJM forwards intact. |
| 8 | **NRG** | Long NRG | Highest-torque, highest-fragility power expression (bought 13GW gas below replacement; 4.2x levered). | Size at 50-60% of other power longs. Add post-Q2 if synergies track. |
| 9 | **6501.T** | Long Hitachi | Cheapest claim on the confirmed transformer bottleneck (3-5yr lead times, +60-80% pricing) — but it IS 24x for a conglomerate where the thesis asset is ~25% of EBITA. | Add on Japan-macro dips, not chases. Invalidation: Hitachi Energy book-to-bill <1.2x. |
| 10 | **CLS** | Long Celestica | Cheapest AI-networking growth (~30x for 50%+ growth), but 6-7% margin contract manufacturer, top-3 customers = 65% of revenue. | Starter 1/3 before Jul-27 print only; add on raise-and-dip or any 12-15% drawdown. Kill: a top-3 customer in-sources/dual-sources away. |
| 11 | **ASML** | Long ASML (starter) | The monopoly is real but 49x fwd is double its 10-yr median — "IP-moat protects the business, not your entry multiple" (2021→2022 = -50% with monopoly intact). | Starter now; scale toward an effective ~35x on export-control scare dips. |

### Option-like (small, venture-sized)

| # | Ticker | Trade | Notes |
|---|--------|-------|-------|
| 12 | **MP** | Long MP Materials | The only Western magnet chain; DoD $110/kg floor + Apple deal; -55% from high. But 161x NTM and China export-control listing aims at MP's own equipment supply chain. Half now (~$44), half reserved for truce-headline flush toward $35-38. |
| 13 | **U-UN.TO** | Long Sprott Physical Uranium | 9.6% NAV discount; 197Mlb reactor demand vs ~160Mlb mine supply; every hyperscaler nuclear PPA adds demand. Ballast, not torque. |

### Shorts (all with structural discipline from red team)

| # | Ticker | Trade | Structure | Trigger / invalidation |
|---|--------|-------|-----------|----------------------|
| 1 | **RHI** | Short Robert Half | DOWNGRADED by round 2 — the mechanics round 1 "solved" were only prepaid: at ~25% SI, put-call parity embeds the borrow + 6.2% dividend into the synthetic forward (12-18mo forwards ~10-15% below spot), so long-dated spreads pay only on falls beyond an already-discounted strip. The thesis payload also shrank: perm placement is ~9% of revenue and growing again; Protiviti (35% of revenue) monetizes the AI-transformation wave. | Premium budget cut to 25bps. 4-7mo put-spread ladders around earnings dates, strikes off the synthetic forward; fund premium with defined-risk call spreads above spot (captures the embedded discount instead of paying it). |
| 2 | **HUBS** | Short HubSpot | DOWNGRADED to small by round 2, on kill-watch: on 2026-04-14 HubSpot inverted to outcome-based agent pricing ($0.50/resolved conversation; credits +67% q/q; NRR guided UP on credits) — the thesis's substitution now runs through HubSpot's own billing meter. Q1 beat every line. The short survives only if credit revenue can't outrun seat decay. | No delta-one through the early-Aug print (8.7% SI + active $1B buyback). Small put spreads, or re-enter post-print. Full kill if Q2 shows credits re-accelerating with NRR >104. |
| 3 | **CHRW** | Short CH Robinson | Delta-one after Jul-29 print only — 4 straight beats, do not stand in front. AI-margin story is a cost-out sugar high while AI commoditizes brokerage itself. | Enter on post-beat strength $210-225, or on first AGP-per-load stabilization with falling volumes. |
| 4 | **MRVL / AVGO** | Pair: short Marvell vs long Broadcom | Half size. Socket erosion (Trainium3 lost to Alchip, Maia at risk to AVGO) vs the share gainer. | Enter short leg on relief toward $225-230. Kill if MRVL names a new hyperscale socket win. |
| 5 | **PLTR** | Short Palantir (relative) | Small, put spreads only, AFTER Aug-3 print — run explicitly against the MSFT/GOOGL long sleeve as a relative bet (per §6): agents commoditize the integration layer faster than the aggregators. Growth is accelerating (+85%), so never naked. | 3-6mo put spreads on failed post-print rally. |
| 6 | **ALLY** | Short Ally (GATED) | Do not enter today: EPS mechanically rising on CD repricing. The 2027-28 displacement-credit thesis is real but early. | Arm when Manheim posts 3 consecutive down months OR ALLY 30+/60+ DPD inflects up 2 quarters. Then delta-one. |

### Structured macro (replacing killed expressions)

- **Rates**: 10s30s steepener or payer swaptions — NOT a TLT ETF short (carry
  -1.5-2.5%/yr eats the ~8-10% target move). Trade it tactically AFTER a QRA
  that raises coupon sizes (bills at 21.9% of debt vs 15-20% TBAC band = the
  supply channel reopens eventually).
- **Natural gas**: long-dated 2027-28 Henry Hub calls instead of EQT equity —
  the back of the curve is where the power-demand repricing happens, without
  paying a post-rally equity multiple through a storage glut.
- **Copper**: 2028+ deferred futures/calls instead of FCX at 19x on a
  tariff-inflated COMEX premium with a proven one-session collapse mode.
- **Gold (GLD)**: modest ALWAYS-ON allocation as the fiscal-endgame hedge
  (UBI-scale transfers into an existing $1.8T deficit) and the book's only
  live tail hedge at inception; pairs with the steepener when it arms.
- **Taiwan hedge**: the TSM/AVGO/NVDA/ASML/CLS complex shares one geopolitical
  failure mode. Either run TSM at half weight (chosen default above) or carry
  rolling 12-month ~20%-OTM TSM puts at ~2-3%/yr against the full semi sleeve.
  There is no intra-sector diversification against this risk.

### Book-level scenario P&L (rough, % of book NAV)

The headline "13 longs / 6 shorts" overstates balance: at inception the LIVE
short book is HUBS (delta-one) plus RHI put spreads capped at ~50bps premium —
CHRW, PLTR, and ALLY are all gated behind entries/dates. Estimated book
drawdowns (long sleeve sized ~2:1 vs live shorts, GLD ~5%):

| Scenario | Est. book P&L | What happens |
|---|---|---|
| Fizzle (10% weight) | **-25 to -40%** | ~9 of 13 longs de-rate 30-60%; shorts squeeze first, help later; GLD flat. The dominant risk. |
| Taiwan event | **-20 to -35%** | 6 longs hit directly (TSM half-weight + put overlay caps the worst leg); nothing in the book profits. |
| 2027 credit accident (levered periphery) | **-15 to -25%** | All AI longs de-rate together (2001 pattern); casualty shorts and GLD offset partially. |
| Baseline thesis path | **+40 to +90%** (probability-weighted engine EVs, thesis-conditional) | Rents accrue to power/tolls; casualties decay on schedule. |

These three loss scenarios are correlated expressions of ONE bet (the thesis,
via Taiwan-concentrated supply chains, funded by fragile credit). Sizing the
whole construct against total capital is the real risk decision.

### Watchlist with hard revival triggers (killed today, right thesis)

| Ticker | Revives when |
|--------|--------------|
| GEV | ~$700-800 (low-30s NTM P/E) with order book intact, OR two consecutive Power-segment EBIT beats proving slot repricing reaches the P&L. Alternative now: ENR.DE (several turns cheaper). |
| 6324.T pair (short HDS vs long components) | Explicit PASS at inception: Japanese borrow mechanics + narrative momentum make even the paired version unattractive until the first humanoid-order disappointment; revisit on any FY3/27 guidance miss. |
| SPCX (SpaceX/xAI, Nasdaq — IPO'd Jun 12 2026 at $135, now ~$116/$1.52T) | KILLED as a long now (high conf): sum-of-parts imputes ~$900B-1.1T to xAI (4x its own Feb-26 merger mark) on $3.2B revenue and accelerating burn, while a lockup waterfall releases >10x the float by Dec 9, 2026 (first big unlock 2 days after the Aug 4 debut print). LONG trigger: post-day-180 unlock flush toward sub-$700-800B mcap (~$55-65/share), where Starlink (+48% growth, 63% EBITDA margins) + launch cover >50% of EV and you get xAI near its merger mark. Tactical defined-risk short only into the Q3 28% unlock if Q2 shows xAI burn >$3B/qtr; invalidated by a Starlink spin announcement. Not shortable in size: 31% SI on a ~4% float, Musk holds 82% of votes and the headline machine. |
| TSLA | KILLED both ways at ~$320/$1.20T (high conf): ~156x NTM EPS pays upfront for ~$15-20B of 2029 robotics/robotaxi profit (1M+ humanoids/yr equivalent) from a program with 0 units produced through Q2-26, while auto GAAP NI fell 5% and FCF went negative on >$25B capex. LONG trigger: ~$200-230 (~100x de-risked 2027 EPS) or verified Fremont Optimus output >1k units/week with third-party confirmation (internal training-data units count as zero). SHORT trigger: second consecutive negative-FCF quarter PLUS explicit slip of the 1M-capacity YE2027 target — defined-risk puts sized for a 30% narrative rally, never delta-one against Musk catalysts. |
| Robotics basket long | The 2028 robotics leg is deliberately UNEXPRESSED at inception (every component long died in review). Build trigger: a shipping humanoid program naming a non-China reducer/actuator supplier, or Optimus actually crossing ~1,000 units/week — then revisit 2049.TW, KGX.DE, TER. |
| MU | 30-40% drawdown on a spot-price rollover scare WITHOUT SCA floors cracking — that proves the floor structure and creates the entry. (Q4 FQ guide: $50B rev / 86% GM — peak-cycle numbers.) |
| 000660.KS | Post-Jul-28 print: in-line KRW 60-62T OP without new lows = seller exhaustion → enter. LTA structures cap upside — size accordingly. |
| BESI.AS | Mid-€140s-160s (~25x fwd), or Samsung/Hynix re-adopting hybrid bonding for HBM4 (currently SHELVED per Jul-2026 TrendForce — the pitched catalyst was factually dead). Alternative: ASMPT 522.HK at half the multiple with the live TCB sockets. |
| POWL | ~22-25x fwd (≈40% derate or 18mo earnings catch-up) with backlog still growing; P&L currently refusing to confirm the order book. |
| LITE | 12-14x FY27 with EML share intact, or broad CPO CW-laser design wins. Optics has NEVER held scarcity rents through a capacity cycle. Alternative: COHR (cheaper, same ramp). |
| SYM | <$25 (~$15B mcap) where backlog gross-profit math covers the cap, or GM sustainably >28-30%. Alternative: KGX.DE (Dematic) at a fraction of the multiple. |
| SYF short | 30+ DPD formation up 2 consecutive quarters + payment rate <15.5% → then Jan-2028 OTM puts (RSAs absorb ~50% of NCO upside — the equity is half-hedged by contract). Purer expression: OMF short (no loss-sharing buffer). |
| WDAY short | cRPO growth <10% or subscription guide cut → Jan-2028 put spreads (mgmt already re-pricing to agent SKUs, cut own headcount 8.5%, raised margins — the EPS denominator is defended). |
| ADP short | Two consecutive quarters of NEGATIVE pays-per-control (2009 precedent: employment -5%, ADP revenue ~flat — realized elasticity <0.5, the pitched mechanism was empirically wrong). Then Jan-2028 puts. |

## 5. Red-team verdicts (scoreboard)

45 trades attacked by 15 adversarial agents across three lenses (priced-in /
thesis-failure / structural). **20 survived, 25 killed.** Full verdicts with
counterarguments: `output/verdicts.json`.

Survivors: TSM, AVGO, NVDA, GOOGL, MSFT, CEG, VST, NRG, 6501.T, CLS, MP,
ASML, U-UN.TO, WTKWY* (*model overrides — see §6), RHI, HUBS, CHRW, ALLY
(gated), MRVL (pair), PLTR (restructured).

Killed: GEV, MU, 000660.KS, BESI.AS, POWL, LITE, SYM, SHA.DE, 6268.T, MAN,
6324.T, FCX, EQT, ADP, PAYX, TCS.NS, WDAY, TEAM, FDS, MMC, LSTR, SYF, EQIX,
6954.T, TLT. Ten carry explicit revival triggers (watchlist below); FCX, EQT
and TLT were replaced by structured-macro expressions of the same views;
MAN, TCS.NS, TEAM, FDS, MMC, LSTR, EQIX, 6954.T, SHA.DE, 6268.T and 6324.T
are hard passes at any nearby price (mechanism refuted or path unsurvivable) —
full dispositions in `output/verdicts.json`.

**Deliberate passes by vertical** (silence ≠ oversight): healthcare produced
no expression that beat its risks (MEDP/IQV cases never cleared the screen —
trial-volume upside vs in-silico disruption nets unclear); defense has no
clean thesis vehicle (closest name, PLTR, is a relative short here); China
listings (Leaderdrive, Shuanghuan, UBTech, Innolight) are where the model says
much of the robotics pool lands, but A-share/HK access, delisting risk, and
short-borrow impossibility make them unactionable for this book — this is an
acknowledged coverage hole, not a view; logistics longs (GXO, RXO) lost to
SYM/KGX.DE on the same exposure and those died too; memory (MU/Hynix) and
optics (LITE) are watchlist-gated capacity-scarcity rents, not passes.

The kills that mattered most (would have been expensive mistakes):

1. **TCS short** (high conf): incoherent with the thesis itself — enterprise
   AI *integration* is what TCS sells; 2026-28 is a services super-cycle
   BEFORE displacement. The AI run-rate ($2.6B, +13% q/q) is additive now.
2. **TEAM short** (high conf): the 125:1 "capture ratio" was a category
   error — Claude Code ARR is inference spend, not reallocated Jira budget;
   Atlassian shipped agent integrations INTO the coding tools.
3. **SYF short** (high conf): Retailer Share Arrangements contractually absorb
   ~50% of incremental charge-offs — the equity is half-hedged against the
   exact scenario being shorted.
4. **6268.T long** (high conf): product-category error — humanoids use
   harmonic/planetary actuators and roller screws, NOT RV cycloidal reducers
   sized for 100kg+ industrial arms.
5. **MU long**: the blowout is SPOT commodity DRAM (+700% yoy), not the
   contracted HBM story — the most mean-reverting price series in semis,
   capitalized at $1.12T. SCA floors are untested through a downturn.
6. **MAN short**: right 2028 thesis, un-survivable path — 19.9% SI, +34.8%
   squeeze day already demonstrated, sell-side capitulation upgrades still
   ahead as fuel.
7. **GEV long**: the quarter cited as confirmation contained a 33% EBIT miss;
   the FCF guide raise is substantially prepayments (deposits on 2031 slots),
   not margin — the P&L hasn't yet proven the slot repricing.

## 5b. Second-round review: the survivors re-attacked

After the book was assembled, seven fresh adversaries re-attacked the live
positions with instructions to find only what BOTH the pitch and round one
missed. Result: **zero kills, seven downgrades** — every core position kept
its direction but lost size or gained structure (adjustments embedded in §4).
Full verdicts: `output/round2_verdicts.json`.

The systemic finding that matters more than any single trade: **the OpenAI/
Anthropic circular-financing nexus is one correlated factor across three core
longs.** OpenAI is 45% of MSFT's commercial RPO and the disputed anchor of
AVGO's backlog; Anthropic is >40% of GOOGL's Cloud backlog while Google
finances it with up to $40B. The report's §3 "overshoot tells" (vendor
financing, circular revenue) were applied to NVDA and neoclouds — but they are
live inside the hyperscaler core longs too. The frontier-lab IPO window
(Anthropic Oct 2026, OpenAI 2027) is therefore not just "event risk" — it is
the credit quality of the book's largest backlogs. De-gross on any lab
markdown, funding snag, or IPO delay; that trigger sits above every
single-name rule.

## 6. Model-level kills (pre-red-team) and disagreements

The quantitative model killed several candidates before the red team ran:

- **Wolters Kluwer long — CUT.** Even the moatiest info-services name shows
  ~–28% expected value because the professional-info pool itself decays under
  the thesis. Moat quality inside a shrinking pool is not enough.
- **FANUC short — DEMOTED.** Crediting even a 5% humanoid-pool capture at 20%
  margin roughly justifies the current price; the "halo with no revenue" short
  is fair-value, not mispriced.
- **Harmonic Drive short — RESTRUCTURED.** As an absolute short it is a short
  call option on the user's own thesis (best case for the stock +1800%). The
  actual mispricing is WHO captures the reducer pool (Chinese entrants at
  30–40% lower price supplying Tesla). Only acceptable as a relative pair.
- **WTKWY long — NO POSITION (unresolved disagreement).** The red team kept
  it (gated on the Aug-5 print); the model says even a strong moat inside the
  decaying professional-info pool nets negative. When the model and the red
  team disagree, we pass.
- **PLTR short — WEAKENED.** A 1.5% capture of the AI-services pool at 30%
  margin justifies much of the price under the thesis; shorting it is
  partially shorting the thesis. Relative-value only, small.

## 7. Book-level risks

1. **The thesis is the book.** ~20-35% is what markets imply for anything like
   a 2027 singularity. If capability plateaus, the longs de-rate (fizzle
   scenario: most longs -30-60%) and the shorts squeeze on relief. The fizzle
   weights (10%) and the 12% discount rate are load-bearing; size accordingly.
2. **Taiwan.** TSM is the anchor AND the single point of failure for the
   entire semi complex. There is no diversification against this inside the
   sector — only sizing and out-of-book hedges.
3. **The levered periphery cracks first** (neoclouds, DC SPVs, Oracle credit):
   a 2027 refinancing accident produces a correlated drawdown across ALL AI
   longs regardless of end demand (WorldCom 2001 pattern: the fraud/blowup
   repriced the honest suppliers too).
4. **IPO event risk**: Anthropic (Oct 2026) and OpenAI (2027) IPOs are the
   cycle's marks-to-market for ~$2T of private value. A failed/discounted
   IPO = de-gross signal for the whole book.
5. **Crowding asymmetry**: the longs are consensus-adjacent (semis crowded);
   the shorts are contrarian-timing trades. In a broad risk-off, both legs
   lose initially (longs de-rate faster than casualty shorts fall).
6. **Circular-financing concentration (round-2 finding):** MSFT, AVGO, and
   GOOGL — three of five core longs — carry the same hidden counterparty
   factor (OpenAI/Anthropic vendor-financed commitments). A lab funding
   accident propagates through 40-45% of the backlogs justifying their
   multiples. This factor, not Taiwan, is the book's most correlated
   non-obvious exposure.
7. **The robotics leg of the thesis is deliberately unexpressed at
   inception.** Every component long died in adversarial review (wrong
   component category, Chinese commoditization, hype multiples); MP is a
   geopolitical-scarcity play, not a robot-volume play. If robotics surprises
   EARLY, this book underperforms the thesis — the watchlist build-trigger
   (named non-China supplier in a shipping program) is the re-entry.
8. **Model risk**: the bottleneck model's power-binding conclusion rests on
   turbine/transformer/queue lead times holding. A demand air-pocket
   (2027 digestion year) converts "sold out through 2030" into cancellations
   — backlogs are deposits, not GAAP revenue.


## 8. Monitoring plan & invalidation triggers

**Thesis trackers (quarterly):**
- Token consumption growth (Google: 3.2 quadrillion/mo, 7x yoy — sustained
  >4x yoy = on-thesis; <2x = delayed scenario).
- Anthropic/OpenAI revenue run-rates ($47B / $25B annualized mid-2026;
  the 2027 singularity implies $150B+ combined exiting 2027).
- Frontier-lab IPO outcomes (Anthropic Oct 2026 window): a failed/discounted
  IPO reprices the whole complex's terminal values — de-gross the book.
- Enterprise agent deployment evidence vs pilot purgatory; TCS/Accenture
  headcount trajectory (TCS –3.85% yoy = early confirmation).

**Bottleneck trackers:**
- Turbine/transformer lead times and pricing (GEV order book, slot pricing).
  Lead-time compression = power-rent erosion = trim power legs.
- CoWoS supply/demand gap (currently 10–20%): gap closure 2027 = trim
  capacity-scarcity semis into strength, keep IP-moat names.
- HBM contract pricing and 2027 capacity landings (the fiber-glut candidate):
  memory legs are TACTICAL — exit on first evidence of supply catching demand.

**Casualty trackers:**
- ADP pays-per-control and white-collar payroll trends; SMB formation.
- IT-services pricing on renewals (T&M rate cards), not bookings headlines.
- Consumer credit: NCO trajectories at SYF/ALLY vs white-collar metro
  unemployment; the shorts activate on the SECOND derivative.

**FX policy for non-USD legs:** hedge the JPY exposure (6501.T) — the book's
US-rates view (term-premium expansion) is yen-negative via rate differentials,
so unhedged JPY fights the macro leg; leave CAD (U-UN.TO) unhedged (commodity
currency moves with the position); EUR names (ASML via ADR, watchlist ENR.DE/
BESI.AS) are small enough to leave unhedged at starter size; if the Korea
watchlist entry (000660.KS) triggers, hedge KRW half-weight (won weakens in
the risk-off scenarios where the entry would trigger). TSM ADR carries NTD
sensitivity (~0.3pp OM per 1%) — accept it; it is part of the Taiwan factor
already sized for.

**Implementation notes (options and borrow):** the RHI put-spread market is
mid-cap wide — work limit orders at mid, accept partial fills, and treat the
50bps premium cap as binding; Jan-2028 LEAPs for the trigger trades (WDAY,
ADP, SYF-via-OMF, ALLY) should be priced BEFORE the trigger fires so stale
quotes don't force chasing; Henry Hub 2027-28 calls and 2028+ copper calls are
liquid at the exchanges but roll costs matter — size once, don't roll monthly;
payer swaptions require an ISDA — the retail-accessible proxy is short TLT
call spreads (defined risk, carry-capped). HUBS borrow: ~12% SI, GC borrow as
of July 2026 — check the rate before sizing delta-one. Sizing map (indicative,
% of book NAV): full 4-5%, medium 2-3%, starter ~1%, option-like 0.5-1%,
GLD ~5%. Rebalance quarterly plus trigger-driven; trim any long that doubles
its weight; re-run the red-team pass semiannually — the July 2026 verdicts
have expiration dates (most cite specific prints and levels).

**Kill criteria for the whole construct:**
- Two consecutive quarters of hyperscaler capex GUIDANCE cuts → the capital
  constraint arrived early → collapse to fizzle weights (harvest
  picks-and-shovels, keep only IP-moat longs and casualty shorts).
- Model-capability plateau through 2027 (no agentic reliability step-change)
  → delayed/fizzle scenarios → unwind displacement shorts (they bleed carry),
  keep power/grid longs (they work on electrification alone).
