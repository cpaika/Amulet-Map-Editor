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

_[Populated after adversarial red-team verdicts — see sections 5–7.]_

## 5. Red-team verdicts

_[Pending phase-2 workflow completion.]_

## 6. Killed and demoted trades

The model itself already killed several candidates before the red team:

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
- **PLTR short — WEAKENED.** A 1.5% capture of the AI-services pool at 30%
  margin justifies much of the price under the thesis; shorting it is
  partially shorting the thesis. Relative-value only, small.

## 7. Book-level risks

_[Finalized with the book.]_

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
