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

## Still open (this push)
- **G — capex/glut investment function:** demand-anchor + q-theory governor
  (parity-gated) + the `chip_growth_ceiling` fix (reprices the glut-detection
  convention across the backtest envelope). Highest risk; do with the envelope
  recalibration.
- **Follow-up:** physically-bounded dynamic capture share (name_GW / ai_power) to
  remove the residual fixed-share-of-growing-pool overstatement.
