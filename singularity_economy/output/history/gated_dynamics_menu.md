# Gated New-Dynamics Menu (v-next)

Every dynamic added in the re-audit/brainstorm cycle is a **gated satellite**: its
gain/share defaults to `0.0`, so the shipped baseline and the `output/golden_v2.json`
snapshot are **byte-identical** with all of them off. Turn them on (individually or
together) to build an "enhanced" scenario. Each has a regression test locking its
directional effect.

| Param (Params / sub-struct) | Default | What it does when ON | Trade relevance |
|---|---|---|---|
| `wage_compression_cog_gain`, `wage_compression_phys_gain` | 0.0 | Displaced-labor reserve compresses the *price* of remaining jobs; wage bill falls faster than headcount (floored at 0.45). | Deepens the wage-linked **short** sleeve (RHI, ADP, PAYX, MAN). |
| `equity_sentiment_gain` (R9 spine) | 0.0 | Reflexive AI-capex bubble/bust: euphoria amplifies capex + eases credit; a glut >1.3 cracks sentiment asymmetrically (Minsky). At 1.0: sentiment ~1.28→0.39, ai_capex ~2.9 vs ~7.6 baseline by 2036. | Fat **left tail** on every silicon/power name; favors contracted (CEG) over merchant (VST/NRG) in the bust. |
| `wealth_effect_gain` (chains off spine) | 0.0 | ΔEquity_sentiment → consumption via MPC × AI wealth share; makes GdpIndex cyclical. Inert unless the spine is also on. | Deepens bust-scenario returns on broad-GDP and cyclical shorts. |
| `transmission_gain` (+ `transmission_growth`) | 0.0 (rate 0.26) | Transformer/HVDC delivery lag: generation that outruns the transmission stock can't energize. At 1.0: 2036 power_margin 0.58→0.62, compute ~-12%. | Bullish **grid-equipment/IPPs** (GEV, POWL, VST, CEG) — the bottleneck is their product. |
| `society.jg_share` (UBI↔Job-Guarantee) | 0.0 (pure UBI) | JG recovers part of its cost → lowers debt-financing share of transfers → smaller sovereign snowball. At 0.6: 2036 debt/GDP 1.95→1.84, long rate 5.39%→5.22%. | Largest **discount lever**: lower rate lifts duration-heavy power/toll longs. |
| `compute_governance_gain` (B12) | 0.0 | Compute-cap/licensing regime that tightens as capability rises, throttling compute added per capex dollar at the SOURCE of the R1 flywheel. At 0.5: 2036 compute 89→52 (still growing). | Caps the silicon/HBM **volume tail** (NVDA/TSM/MU); raises licensed-incumbent value. |
| `ai_commoditization_gain` | 0.0 | AI-services provider rent erosion. The provider's share of displaced-wage surplus is a constant 0.45 by default (durable pricing power forever); with this on it falls toward `0.45 × (1 − gain × adoption)` as the market matures (open weights, multiple providers). At 0.5 the ai_services pool ~halves by 2036 (3.59→2.07). | The "**do AI labs keep pricing power?**" knob — bearish AI-services incumbents (MSFT/GOOGL AI rent, the ai_services capture names) if capability commoditizes; leaves the infra/compute layer (which sells picks-and-shovels regardless) intact. |
| `power_glut_price_gain` | 0.0 | Two-sided merchant power pricing. The scarcity price is one-sided by default (rises above normal when utilization exceeds target, never falls below). With this on, sub-target utilization pushes the price BELOW normal (floored at 25% of normal, a must-run cost) — a power glut crashes merchant/spot power. Inert when power binds (the base case); engages in a demand-fizzle/generation-overbuild glut. | Prices the merchant-power (VST/NRG) **downside** in a power glut — the one-sided price could only ever mark them up. A demand fizzle with generation overbuild now craters their electricity revenue (~47% in the test glut). |
| `wright_gain` (+ `wright_learning_rate`) | 0.0 (LR 0.20) | Wright's-law learning curve: compute unit cost falls with CUMULATIVE production instead of calendar time (each doubling cuts cost by the learning rate). Makes cost decline endogenous/reflexive (buildout → cheaper compute → more units/dollar) and scenario-dependent (a fizzle learns slower than a boom); the curve saturates late, unlike the constant-forever calendar. | Ties the silicon/compute **cost trajectory** to realized volume — a fizzle keeps compute expensive (bearish the whole AI-capex complex), a boom cheapens it; couples directly into the q-governor's replacement-cost denominator. |
| `q_governor_gain` (+ `q_risk_premium`, G) | 0.0 (premium 0.10) | Tobin's-q investment governor: overlays a return-on-capital channel on the momentum-driven capex desire. q = (last year's OPERATING return on installed compute — ai_services value / replacement cost) / hurdle, where the hurdle is ENDOGENOUS = sovereign `long_rate` (B11) + equity risk premium + compute depreciation, so a debt-crowding rate spike tightens the investment hurdle automatically. (Re-audit fix: the numerator is ai_services alone — silicon/ip_tolls are the vendors' capex-flow revenue, not the operator's stock return.) q>1 accelerates investment, q<1 brakes it. Reveals the model's **supply-vs-demand asymmetry**: on the way up capex is chip/power-bound so the accelerator is muted (realistic), on the way down a sub-hurdle return makes DEMAND bind and cuts capex — an endogenous brake momentum lacks. A higher hurdle monotonically lowers cumulative capex. | Grounds the AI-capex **cycle** in profitability, not just demand; brakes the silicon/AI-capex complex when margins compress even while demand still grows (a distinct, earlier bust signal than the glut-triggered spine). |

## Suggested coherent "enhanced-realism" scenario (for review, not shipped)
A defensible combined setting to see how the book re-ranks (all still hypotheses):
`equity_sentiment_gain = 1.0`, `wealth_effect_gain = 1.0`, `transmission_gain = 0.5`,
`wage_compression_cog_gain = 0.4`, `wage_compression_phys_gain = 0.4`,
`society.jg_share = 0.3`. Net expected tilt: fatter left tails on the silicon/AI-capex
complex (spine+wealth), a firmer/longer power-rent thesis (transmission), a stronger
short sleeve (wage compression), partly offset on the discount by a JG tilt.

All six compose cleanly together — the `enhanced_scenario_composes_without_pathology`
test runs all of them on at once and verifies no NaN / no collapse / boom-bust intact.

**Interaction note — q-governor × spine.** The Tobin's-q governor is deliberately NOT in
the enhanced-realism set; it composes into first-principles v2 instead. Composed with the
reflexive spine it AMPLIFIES the boom-bust: q-theory investment is procyclical — it
accelerates when the *operating* return on compute is high (mid-boom), which is exactly
when the future glut is sown, so the glut runs hotter (peak ~2.5 → ~3.6) and the
sentiment trough falls deeper (~0.65 → ~0.36). (This corrects an earlier note based on a
mis-specified q numerator — see the re-audit fix below — which wrongly reported the
governor as *dampening* the bust; on a proper operating-return basis it deepens it.)
Whether to include it is a scenario-design choice: run it on its own (`q_governor_gain >
0`), via `book-v2`, or add it explicitly.

## Still to build (need owner input — reprice the trade book directly)
- **Circular vendor financing** (chip→cloud recycled-capital revenue) — needs a
  `valuation.rs` earnings-quality haircut on NVDA/AVGO; deferred like C2.
- **Treaty-pause hazard scenario** (clean, gated — a new `scenarios.rs` case),
  **reserve-currency convenience-yield erosion**, **alignment-liability law** — see
  `next_cycle_worklist.md` for mechanisms; each shifts specific tickers.

## Deferred audit items (documented in-code, need owner input)
A7/C8 (power-gate rework), B4 (fab-ceiling — reprices the glut/backtest envelope),
C2 (capture-term decay), C7 (dread stringency ratchet).
