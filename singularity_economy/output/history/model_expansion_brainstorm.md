# Model Expansion Brainstorm — what to make more dynamic/accurate

Standing map of candidate dynamics, ranked by leverage × tractability. Layers
already built are noted. This is the running backlog for the systems model.

## Built this session
- **Energy generation mix** (`energy.rs`) — solar/battery Wright exponentials vs
  gas/nuclear firm power; answers "ceiling or exponential?" (→ exponential,
  gated by turbines/grid 2026-32).
- **Regional political economy** (`regions.rs`) — US/China/EU divergence on the
  three anti-correlated axes: capital, power-buildout, displacement-absorption.
- **Food** (`food.rs`) — gas→fertilizer→food-price→unrest chain + green-ammonia
  cut + alt-protein; the one with a live feedback into tension.

## Tier 1 — highest leverage, build next
1. **Regions ↔ core two-way coupling.** Today regions/energy are satellites. Wire
   the bloc energy-buildout into the *actual* power supply (China's 8x grid edge
   should let the China bloc run more compute/robots), and a "race dynamic" where
   a runaway leader pulls global adoption up (no time for safety/regulation). This
   turns the decomposition into a driver.
2. **Reflexive financing (R9, Soros).** AI/robot equity valuations feed capex
   capacity: as the thesis proves out, capital floods in → faster buildout →
   higher valuations, until a confidence break reverses it. The model's biggest
   missing reinforcing-then-collapsing loop (the "Cisco moment" is currently only
   in R3 capex momentum). Already flagged as gap-scan task.
3. **Trade / tariffs / decoupling.** Bloc-to-bloc flows: export controls (compute),
   rare-earth embargo (have the channel in materials), tariffs, friend-shoring.
   Decoupling raises everyone's cost and slows diffusion — a first-order 2026-30
   dynamic the model treats only as one-off geopolitics shocks.
4. **Fiscal-monetary regime under AI.** Dollar hegemony / reserve-currency status
   as AI concentrates output; the bond-market tripwire (have B11) as a *regime
   switch* not a smooth drag; inflation from energy+materials vs deflation from
   AI productivity — which wins, and the Fed reaction function.

## Tier 2 — real dynamics, moderate effort
5. **Climate feedback loop.** Warming → crop stress (have the food drag) →
   migration → tension (have channels) → and energy-transition politics. Close the
   loop: emissions from gas buildout → warming → food/migration.
6. **Water resources.** Ag is ~70% of freshwater; datacenter cooling + chip fabs
   are water-intensive. A water-scarcity constraint on both food and compute in
   specific regions (US Southwest, China North, MENA).
7. **Human capital / talent flows.** Education lag vs displacement speed; brain
   drain toward the leading bloc; the reskilling-can't-keep-up dynamic that sets
   how fast displaced labor re-absorbs (or doesn't).
8. **Housing / real estate.** Endogenous — matters for the wealth-effect on
   consumption and for the user's own balance sheet; interacts with rates (B11)
   and with where displaced workers can afford to live.
9. **Insurance / reinsurance.** Climate + AI-liability + cyber (have cyber dread)
   stress the risk-transfer system; an insurance-retreat dynamic (already visible
   in Florida/California) that gates real activity.

## Tier 3 — richer versions of existing layers
10. **Consumer demand as endogenous stock.** Precautionary savings (have a society
    hook) → demand contraction → the demand-side of overshoot. Currently demand
    "never contracts" (a noted design simplification).
11. **Labor as discrete events.** Strikes/unionization as regime-switch events
    (WGA/ILA pattern) rather than a smooth backlash multiplier.
12. **Election cycles as explicit regime switches** per bloc (US 2-4yr clock is in
    the research but modeled as a smooth stress stock).
13. **Two-tier compute** — robot inference chips vs datacenter training chips both
    gated by TSMC 4nm (materials has inference_chips; make them share fab output).
14. **Memetics / ideology contagion** (SIS model) — pending task; frames as
    epidemics routing policy. NRx / accelerationism / millenarianism already
    sketched in prior notes.
15. **Space industrialization** beyond orbital compute — lunar/asteroid materials
    relieving the materials chokepoints on a long horizon (have space.rs stub).

## Tier 4 — long-horizon / speculative
16. MAD-under-AI stability, coordination/treaty hazard (pending task #27).
17. Biological transhumanism → politics (have bio.rs enhancement stub).
18. Longevity/LEV → demography (pension/retirement-age dynamics; have BCI/longevity
    shadow stocks in bio.rs).

## Cross-cutting accuracy upgrades
- **Endogenize what are currently exogenous shocks** (geopolitics metals/Taiwan)
  into probability-generating stocks (tension → embargo hazard).
- **Regional disaggregation of the CORE**, not just a satellite — the single
  biggest accuracy jump, but a large refactor.
- **Uncertainty bands on every output** via the MC harness (have `mc`), reported
  as p10/p50/p90 rather than point paths.
