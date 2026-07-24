# Singularity Economy Model

A multi-sector, bottleneck-propagating simulation of the world economy 2026–2036
under the thesis: **software singularity in 2027, general-purpose robotics ramp
from 2028**.

Built as the quantitative backbone for an investment research project: the model
does not try to predict GDP — it tries to answer the four questions that decide
where equity value migrates:

1. **Which physical constraint binds in which years** (chips, power, capital,
   or demand/adoption friction) — the binding constraint earns scarcity rents.
2. **How large do the beneficiary revenue/profit pools get** (silicon, power
   equipment, datacenter infra, electricity, robot components).
3. **How fast do the casualty pools decay** (IT services/outsourcing, BPO,
   seat-priced SaaS, professional info services) as AI displaces cognitive labor.
4. **How robust are these answers across parameter uncertainty** (800-run
   Monte Carlo over singularity timing, adoption speed, supply-chain ramp rates,
   learning curves).

## Files

- `model.py` — core annual-step simulation (pure stdlib, no dependencies)
- `scenarios.py` — named scenarios (baseline / fast_takeoff / delayed / friction
  / fizzle) + Monte Carlo
- `test_model.py` / `test_valuation.py` — 36 tests: invariants, 2026
  calibration vs actuals, comparative statics, valuation engine
- `calibration_notes.md` — parameter sources from the research sweep
- `output/results.json` — full scenario + Monte Carlo output

## Run

```bash
python3 -m unittest test_model   # test suite
python3 model.py                 # baseline scenario summary
python3 scenarios.py             # all scenarios + Monte Carlo (JSON)
```

## Headline robust findings (survive the Monte Carlo, post-calibration)

- **Power is THE constraint of the decade** — binds in 84% of runs in 2026 and
  60–81% of runs every year through 2033 (turbines sold out through 2030,
  3–5yr transformer lead times, 5–10yr interconnection queues). Chip capacity
  gets a secondary window 2027–2030 (~25–33% of runs).
- **Capital becomes the constraint late**: by 2035–36, ~45–60% of runs are
  capped by capital willingness (capex hitting % of GDP ceilings), not physics —
  scarcity rents migrate from equipment owners to capital providers.
- **The "Cisco moment" (capex growth <15%) lands 2034–2036 under the thesis**
  (p10 2034) — years of runway before picks-and-shovels multiple compression,
  IF the singularity happens. In fizzle worlds it arrives ~immediately, which
  is what scenario weights and the 12% discount rate price.
- **Casualty decay is back-half loaded**: median cognitive-work displacement
  ~7% (2028) → ~25% (2030) → ~50% (2032); median IT-services pool -32% by 2033.
  History says casualty multiples collapse 2–4 years before revenue — under the
  thesis, mid-2026 is "early," which is when shorts must be placed.
- **Robotics is an early-2030s labor story but a late-2020s component story**:
  median 2032 robot production ~0.8M units/yr (p90 2.4M), physical-labor
  displacement <5% by 2036 — yet component supply chains (reducers, actuators,
  magnets) must expand years ahead, and hype-priced pure plays (Harmonic Drive
  at 404x) already trade as if the 2032 p90 is certain.
- **Transition-recession tail**: worst-year world GDP growth goes negative in
  ~10% of runs — the consumer-credit short leg hedges this tail.

*This model is a scenario-analysis tool, not investment advice.*
