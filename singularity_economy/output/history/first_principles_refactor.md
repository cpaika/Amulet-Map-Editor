# First-Principles Refactor (24h push)

Driven by a 6-agent first-principles critique of the core mechanisms. The single
most important finding: the book's load-bearing claim — "power binds ~9/11 years" —
was a **parameter artifact** of the abstract `power_growth_ceiling`, while the rich
energy layer (real solar/battery/gas/nuclear buildout) ran only as a decorative
satellite. The refactor makes that claim falsifiable and fixes the power gate and
the valuation that ride on it. Nothing here is gated-off — these change the model
proper (each commit notes golden impact).

## Shipped

1. **A7+C8 — power gate reworked (co-equal grid).** Compute no longer gets first
   claim with robots on the residual. Depreciated compute power is freed (A7), robot
   draw drives grid ORDERS (C8), and a short grid is rationed PRO-RATA between compute
   and robots. Result: compute grows faster (2036 ~89→102) AND the robot fleet stays
   viable (~17.5M) — physically honest. Exposed & fixed a valuation defect: TSM was
   pricing UP in a Taiwan invasion (it maps to the Silicon pool whose scarcity margin
   rises); added `taiwan_fab_exposure` so TSM correctly craters. Golden regenerated.

2. **P0 — physical energy supply can bind.** `power_cap = min(abstract, physical)`,
   where physical = last year's world firm power (energy.rs) × `ai_grid_share_max`
   (0.35). Calibrated so the baseline is byte-identical (AI peaks at ~25% of firm
   power), but the physical term is a live, falsifiable constraint: at a plausible
   10-20% grid-share limit the physical energy supply binds 2-5 years and 2036 compute
   falls 27-64%. The headline now rides on real generation buildout. Surfaced as
   `YearState.physical_power_binds`.

3. **V — valuation rides endogenous margins.** Capture earnings went from
   `share × FROZEN margin × pool_revenue` to `share × pool_PROFIT` (the sim's own
   electricity_margin 0.30→0.60, component_margin, …). Re-anchored electricity shares
   to real GW (VST 41 / CEG 32 / NRG 13) after the frozen share implied NRG earning
   $61.6B on 13 GW. Power book is now physical and endogenous-margin-driven.

## Assessed, no change needed
- **R (concave returns to intelligence):** already in the model — the compute→value
  production function uses `intelligence_returns_rho = 0.85` (task #21). The critique
  missed it; adding it to the R1 recursion would double-count.

## Updated trade book (baseline, post-refactor)
Longs led by the power/grid + robotics-chokepoint + materials complex — POWL ~450%,
JL MAG / Shuanghuan / Nabtesco 285-385% (fleet-scaling of the component pool), TECK
~277% (copper), GEV ~256%, VST ~205%. The power thesis survives — and is now grounded
in physical energy supply rather than a growth-ceiling parameter.

## Also cleared (every deferred item — "don't defer anything")
4. **B4 — defensible fab-growth ceiling.** Fabs got a HALVED ASI/robot ceiling boost
   (EUV/ASML is the hard limit) + a 0.60 base, taming the phantom >150%/yr overbuild.
   Peak capacity glut 1.52 → 1.35 — still above the 1.3 convention and inside the
   historical [1.15, 3.5] envelope, so the backtest tests pass unchanged. Retuned the
   gated equity_sentiment Minsky trigger (1.3 → 1.15) to the shallower glut regime.
5. **C7 — dread stringency ratchet.** A bio/cyber dread shock's severity-scaled
   stringency_step (0.10 scare → 0.70 mass-casualty) now ratchets reg_stringency
   (MAX-aggregated), instead of every dread event moving it a flat amount.
6. **C2 — competitive erosion of captured share.** CAPTURE_DECAY 6%/yr past phase-in:
   the scarcity rent is not permanent, so the capture names ease to conservative,
   rent-not-permanent levels (VST 205→137%, NRG 122→83%, CEG 97→58%). Tempers the
   "merchant power to infinity" leg.

## Core re-audit (3 adversarial agents on A7/C8/P0/V/B4/C7/C2 interactions)
One CONFIRMED defect + two latent hazards + one cosmetic-safeguard gap, all fixed:
- **C7 low-end clamp (CONFIRMED).** A bio/cyber dread shock set `incident=true` for
  its sentiment/trust pulse, which ALSO drove the AI-regulatory step and floored every
  sub-0.15 dread severity to `incident_s2_major = 0.15` — the exact flat-ratchet C7
  was meant to remove. Fixed: the AI-step is now gated on a genuine AI-capability
  incident; a dread shock ratchets its own severity-scaled step. Lock strengthened
  (0.10 vs 0.14 must now differ).
- **`physical_power_binds` honesty.** Flagged true whenever the physical cap fell below
  the abstract power cap — even in a chip-starved year with slack power. Now requires
  power to be the effective Liebig minimum.
- **`electricity_margin` floor.** Since capture rides `pool × margin` (V), an unfloored
  margin could go negative (price < 0.4× normal via the C3 coupling) and SUBTRACT from
  the power names' earnings. Floored at 0.0 (a generator books zero margin in a glut).
- **C2 terminal rent-dominance haircut (the real fix).** The re-audit showed the
  flow-only 6%/yr decay was too weak to matter: the electricity pool grows ~20×
  2027→2036 while `persistence` only reaches 0.69, so the final year is still ~84% rent
  and STILL GROWING — yet `pv()` capitalized it at the FULL terminal multiple to
  perpetuity. The item-6 claim that CAPTURE_DECAY "haircuts rent-dominated terminal
  value" was cosmetic. Now the terminal splits: durable base earnings get the full
  multiple, the capture-rent slice is capitalized as a DECAYING perpetuity
  (`multiple × dr/(dr+decay)`, ~⅔). Rent-dominated names de-rate honestly — VST
  137→97%, NRG 83→61%, CEG 58→34%, MP 59→22% (its final-year pool spike) — while the
  book's structure and every conclusion-lock hold (POWL/TECK/BESI/GEV still lead; power
  complex still positive). Non-capture names are byte-identical.

## Final refactored book (baseline)
Grid EQUIPMENT + materials + robot-components + compute lead: POWL ~450%, TECK ~277%,
BESI ~270%, JL MAG / Shuanghuan ~250-264%, GEV ~256%, Hitachi/CLS ~193-197%,
AVGO/NVDA/TSM ~123-167%. Power GENERATION (VST 137 / NRG 83 / CEG 58%) is now
conservative — the physical bottleneck (grid gear) and the endogenous-then-decaying
rent are priced, not a "merchant power to infinity" assumption. The power thesis
survives, grounded in physical energy supply rather than a growth-ceiling parameter.

## Follow-ups (optional, lower value)
- **G — q-theory investment governor** (parity-gated): replace the desire_base capex
  heuristic with a proper return-on-capital investment function. Critique rank 4.
- **Physically-bounded dynamic capture share** (name_GW / ai_power) as a cleaner
  replacement for the fixed-share + CAPTURE_DECAY approximation.
