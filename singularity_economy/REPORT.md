# Singularity 2027: Economy Model & Trade Book

**Prepared:** 2026-07-24 (overnight deep-research run)
**Thesis (given):** software singularity 2027 — AI meets/exceeds expert human
capability across cognitive work with task costs 100–1000x below wages;
general-purpose robotics ramps from 2028.
**Process:** 17-agent research sweep (562 web lookups) → calibrated bottleneck
simulation with 800-run Monte Carlo (`model.py`, 36 tests) → 45-candidate
screen → per-ticker financial verification → adversarial red team (every trade
attacked on priced-in, thesis-failure, and structural lenses).

> **This is scenario analysis conditional on the stated thesis, not investment
> advice.** Position sizing must reflect that the thesis itself is the largest
> risk: prediction markets imply the marginal investor carries only ~20–35%
> probability of anything like a 2027 singularity. Everything below is
> probability-weighted across five scenarios including a fizzle.

---

## 1. The model and what it robustly says

The model is deliberately not a GDP forecaster. It is a bottleneck-accounting
engine: each year, desired AI expansion collides with four constraints —
**chips, power, capital, adoption friction** — and the binding constraint caps
growth and earns scarcity rents. Company earnings are then driven by mapped
profit pools (`valuation.py`, `companies.py`), valued at a punitive 12%
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
3. **Capital becomes the binding constraint late (2034+ in 30–60% of runs)** —
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
   ~10% of runs as displaced labor income outruns recycled demand. The
   consumer-credit and duration legs of the book hedge this tail.

## 2. What the market already prices (July 2026)

The market is mid-correction, not euphoric — this materially improves the
entry point for thesis-consistent trades:

- **Crowding is concentrated in semis** (record hedge-fund chipmaker exposure)
  yet the multiples are not bubbly: NVDA ~21x forward, TSM 19.5x, AVGO 24.9x.
  The risk in these names is the E, not the P/E (the Cisco lesson: revenue
  peaked 3 quarters *after* the stock).
- **Capex payers have been punished**: MSFT –19% YTD (20.6x fwd), GOOGL sold
  off on raising capex, META 18.6x, ORCL halved to 14.9x. If 2027 agent
  revenue vindicates 2026 capex, these are the *neglected* thesis longs.
- **First-order casualties are already dead**: Chegg $0.97, Concentrix –63%
  (1.9x fwd), EPAM –52%, Salesforce –49% (11.3x fwd), Adobe 8.2x. The market
  reprices casualties within 2–4 quarters of evidence. **The remaining short
  alpha is only in names still priced as durable** (ADP 20x, PAYX ~21x, RHI
  ~23x with falling earnings, ManpowerGroup UP 19% because robotics is beyond
  the market's dating horizon).
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
| 1 | **TSM** | Long TSMC | The convergence pick: named by the red team as the better expression of the ASML, MU, AND SK Hynix theses. Owns both 2026 chokepoints (N2 wafers, CoWoS packaging), 4 years of committed price hikes, 67.7% GM, net cash $69B — at 19.5x fwd with ~27% consensus EPS CAGR. IP-moat rent class (persists decades, per history). | Enter now. Invalidation: Taiwan escalation (book-level risk, hedge separately); CoWoS gap closing with pricing-power rollback. |
| 2 | **GOOGL** | Long Alphabet | Reframed by red team: "best-capitalized compute owner at market-plus price." TPU full stack (4.3M units/yr), DeepMind, Waymo; self-funds capex from search cash flow. | Add on capex-tantrum dips (~$317 zone). Invalidation: Cloud backlog growth stalling; paid-click erosion without AI-revenue offset. |
| 3 | **MSFT** | Long Microsoft | The punished aggregator: -25% over 1yr to 20.6x fwd on capex/seat fears while owning enterprise agent distribution + OpenAI economics. Asymmetric skew at this multiple. | Half now, half after Jul-29 print. Invalidation: Azure cc growth <38%. |
| 4 | **AVGO** | Long Broadcom | Custom-ASIC royalty on hyperscaler in-sourcing: $73B AI backlog, >$100B FY27 AI revenue line of sight, 75%+ GM, leverage now 1.0x. The inference mix-shift winner. | Enter now; most expensive of the semi longs (24.4x) — size below TSM. Invalidation: a named second-gen ASIC program re-compete loss. |
| 5 | **CEG** | Long Constellation | Contracted 20-yr nuclear PPAs (MSFT Crane, Meta Clinton, new Walmart) = the "royalty acre" of the buildout; entry got CHEAPER than thesis assumed (24x fwd after July dip vs 26-32x). | Add toward $255-260. Invalidation: Crane restart slipping past 2027; PJM cap regime extended AND tightened (rent clipping). |

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
| 1 | **RHI** | Short Robert Half | **Put spreads ONLY** (12-18mo, ≤50bps premium at risk): 24% SI + 8.7% dividend = >15%/yr carry on stock borrow. The fundamental case is the strongest of all shorts (perm placement = first AI casualty; payout > net income; still ~23x). | Enter spreads on post-earnings IV crush or squeeze rallies >$42. |
| 2 | **HUBS** | Short HubSpot | Moderate size, delta-one acceptable. Best SaaS short: demand cracks visible NOW (softer pipeline, longer cycles) + independent kill-shot (AI search destroying the inbound/SEO engine it sells). | Respect the floor: $1.4B net cash, M&A interest (Alphabet approach) — cover into capitulation, don't press below ~$8.5B EV. |
| 3 | **CHRW** | Short CH Robinson | Delta-one after Jul-29 print only — 4 straight beats, do not stand in front. AI-margin story is a cost-out sugar high while AI commoditizes brokerage itself. | Enter on post-beat strength $210-225, or on first AGP-per-load stabilization with falling volumes. |
| 4 | **MRVL / AVGO** | Pair: short Marvell vs long Broadcom | Half size. Socket erosion (Trainium3 lost to Alchip, Maia at risk to AVGO) vs the share gainer. | Enter short leg on relief toward $225-230. Kill if MRVL names a new hyperscale socket win. |
| 5 | **PLTR** | Short Palantir | Small, put spreads only, AFTER Aug-3 print. Growth is accelerating (+85%) and AIP is currently the agent BENEFICIARY — this is a valuation short, the weakest kind. | 3-6mo put spreads on failed post-print rally. |
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
- **Gold (GLD)**: modest allocation as the fiscal-endgame hedge (UBI-scale
  transfers into an existing $1.8T deficit); pairs with the steepener.

### Watchlist with hard revival triggers (killed today, right thesis)

| Ticker | Revives when |
|--------|--------------|
| GEV | ~$700-800 (low-30s NTM P/E) with order book intact, OR two consecutive Power-segment EBIT beats proving slot repricing reaches the P&L. Alternative now: ENR.DE (several turns cheaper). |
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
6954.T, TLT — each with explicit revival triggers (see watchlist).

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
6. **Model risk**: the bottleneck model's power-binding conclusion rests on
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

**Kill criteria for the whole construct:**
- Two consecutive quarters of hyperscaler capex GUIDANCE cuts → the capital
  constraint arrived early → collapse to fizzle weights (harvest
  picks-and-shovels, keep only IP-moat longs and casualty shorts).
- Model-capability plateau through 2027 (no agentic reliability step-change)
  → delayed/fizzle scenarios → unwind displacement shorts (they bleed carry),
  keep power/grid longs (they work on electrification alone).
