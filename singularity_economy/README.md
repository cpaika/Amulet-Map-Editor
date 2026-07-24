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
- `test_model.py` — 22 tests: invariants, 2026 calibration vs actuals,
  comparative statics
- `calibration_notes.md` — parameter sources from the research sweep
- `output/results.json` — full scenario + Monte Carlo output

## Run

```bash
python3 -m unittest test_model   # test suite
python3 model.py                 # baseline scenario summary
python3 scenarios.py             # all scenarios + Monte Carlo (JSON)
```

## Headline robust findings (survive the Monte Carlo)

- **Power is the most persistent bottleneck** — binds in ~63% of runs in 2026
  and 30–40% of runs every year through the mid-2030s. Chip capacity binds
  mostly 2028–2030 and then catches up.
- **Scarcity rents migrate and then compress**: after ~2031 the binding
  constraint is increasingly *demand/adoption friction*, not physical supply —
  the classic setup for a picks-and-shovels margin peak (cf. 2000 telecom capex).
- **Casualty decay is back-half loaded**: median cognitive-work displacement is
  ~7% (2028) → ~25% (2030) → ~50% (2032). Shorts on human-cognitive-arbitrage
  businesses are directionally robust but early entry costs carry.
- **Robotics is an early-2030s labor story but a late-2020s component story**:
  median 2032 robot production is under 1M units/yr, physical-labor displacement
  <5% by 2036 even in aggressive ramps — yet component supply chains (reducers,
  actuators, magnets) must expand years ahead of the ramp.

*This model is a scenario-analysis tool, not investment advice.*
