# Singularity Economy — Model & Trade Research

Quantitative backbone for an investment research project on the thesis:
**software singularity 2027, general-purpose robotics ramp from 2028.**
Built, tested, and adversarially reviewed end-to-end: research sweep →
model → trade screen → two red-team rounds → systems-dynamics rebuild →
Rust port → code review → corrections.

**Read first:** `REPORT.md` (the full research report and trade book).

## The two models

| | v1 `model.py` | v2 `model_v2.py` + `rust/singularity-econ` |
|---|---|---|
| Paradigm | bottleneck accounting with exogenous supply caps | Meadows systems dynamics: stocks, flows, **endogenous feedback loops**, explicit construction delays |
| Supply growth | assumed ceilings | B1 balancing loops (scarcity rent → investment → delayed capacity → rent decay) |
| Capex behavior | demand growth assumption | R3 momentum on perceived (lagged) demand → endogenous queues |
| Credit | absent | B4: debt accumulation → spreads → capital ceiling (binds in ~33% of runs) |
| Rent duration | asserted from history | **emergent**: commodity silicon decays in a damped hog-cycle (median normalization 2032, endogenous 1.37x glut by 2036) while the IP-toll sector (no supply response) holds peak margins throughout |
| Speed | ~2.5ms/run (Python) | ~3µs/run (Rust) — 10k-run Monte Carlo in ~30ms |

Both models drive the same valuation layer (`valuation.py`, `companies.py`);
`test_scenarios_v2.py` locks the conclusions that must agree across them.
Design rationale: `meadows_design.md`. Meadows leverage-point analysis with
empirical sensitivities: `leverage_points.md`. Parameter evidence:
`calibration_notes.md`. Build/test (Buck2 + cargo): `BUILDING.md`.

## Robust findings (survive Monte Carlo, both models, and review)

1. **Power is the constraint of the decade** — binds in ~96–100% of runs
   every year through 2034; power rents never normalize within the horizon
   in any sampled parameter draw.
2. **Rent duration = moat type**: IP tolls hold peak margins through 2036;
   commodity silicon rents decay by ~2032 (median) into an endogenous glut —
   harvest capacity-scarcity positions into strength; only IP tolls are
   decade holds.
3. **Casualty decay is back-half loaded** (~7% displacement 2028 → ~41–50%
   2032 median) and is a **timing × integration-friction** variable — AI
   capability magnitude is irrelevant to it (ρ=0.01).
4. **Credit crunch in ~33% of runs**, driven by the externally-funded share
   of capex (ρ=+0.65) — the leading indicator for the levered-periphery
   accident.
5. **Robotics is a component story now, a labor story in the 2030s** —
   median 2032 production ~0.3M units/yr; physical displacement <5% by 2036.

## Process artifacts

- `output/verdicts.json`, `output/round2_verdicts.json` — trade red-team rounds
- `output/financials.json` — verified mid-2026 per-ticker financials
- `output/valuations.json` / `valuations_v2.json` — scenario DCFs per model
- `output/golden_v2.json` — Python↔Rust parity contract
- `output/mc_v2_rust.json`, `output/sensitivity_v2.json` — 10k/20k-run studies
- `report_artifact.html` — published report page

## Tests

~80 across five suites (see `BUILDING.md` for the inventory): invariants,
2026 calibration anchors, comparative statics, **per-loop ablations** (every
named feedback loop, disabled, must change behavior as theory predicts),
pipeline conservation, golden parity at 1e-6, property tests over the
parameter space, and cross-model conclusion locks.

```bash
buck2 test //... ; cargo test --manifest-path rust/singularity-econ/Cargo.toml
```

*Scenario analysis conditional on a stated thesis — not investment advice.*
