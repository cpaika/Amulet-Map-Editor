# Leverage Points: Where This System Can Be Moved

Donella Meadows' twelve leverage points, applied to the AI/robotics economy
as formalized in `rust/singularity-econ` — now with *empirical* gain estimates from the
20k-run Spearman sensitivity study (`output/sensitivity_v2.json`). Meadows'
ordering (12 = weakest, 1 = strongest) holds up strikingly well: the things
markets argue about daily sit at the weak end; the things that actually move
the model's outputs sit at the strong end and are barely priced.

## The empirical gain table (what actually moves outputs)

| Output | Dominant drivers (Spearman ρ) | Reading |
|---|---|---|
| Total capex 2032 | power_growth_ceiling +0.22, power_supply_gain +0.20, power_base_growth +0.13 | The size of the whole buildout is set by the power supply side. Demand parameters barely register. |
| Cognitive displacement 2032 | singularity_year −0.62, adoption_halflife −0.61, backlash_gain −0.14 | Displacement is timing × integration friction. Capability magnitude (singularity_boost ρ=0.01) is irrelevant within the sampled range. |
| Credit-crunch depth | internal_funding_share +0.65 (nothing else above 0.07) | The financing accident is a capital-structure choice, not a demand outcome. Monitor the externally-funded share of capex. |
| Power margin 2032 | singularity_year +0.30, all else <0.02 | Power rents are robust to nearly everything — the only thing that moves them is when the demand shock lands. |
| Robot production 2032 | singularity_year −0.80, component_supply_gain +0.10 | This decade's robot volume is a start-date variable. Learning rates, costs, bootstrap gains — all second order before 2033. |
| Silicon rent duration | chip_base_growth −0.38, chip_supply_gain −0.30, momentum_gain +0.15, power_supply_gain +0.10 | Post-review, silicon rent duration is dominated by chips' own supply side; the power cross-coupling persists at smaller magnitude (+0.10) — permitting reform still rotates rents toward silicon, more mildly than first estimated. |

## Meadows' twelve points, annotated

| # | Leverage point | In this system | Empirical/trade note |
|---|---|---|---|
| 12 | Constants, subsidies, standards | rate cuts, chip subsidies, tax credits | Weakest lever, loudest headlines. The model barely moves. Fade macro-headline repricings of structural positions. |
| 11 | Buffer sizes | strategic reserves (magnets, turbines), inventory | Small effects; China's rare-earth stockpile is the one buffer with teeth (it gates MP's downside protection value). |
| 10 | Stock-and-flow structure | the grid, fab shells, ports | Physically slow to change — this is *why* power rents are the most durable output in the model. |
| 9 | Delay lengths | interconnection queues (5–10yr), transformer lead times (3–5yr), fab builds (2–3yr) | Second-strongest practical lever. Cutting the power pipeline from 3 stages to 2 in the model materially accelerates capex and kills power rents earlier. **Queue-reform news is the single most important signal for the power book.** |
| 8 | Balancing-loop strength (B1) | permitting reform, turbine capacity licensing, China component entry | The empirically confirmed rent-killer. chip_supply_gain −0.16 on rent duration; component_supply_gain is what made every robotics pure-play fail review. For each rent position, ask: what strengthens this sector's B1? That's the kill signal. |
| 7 | Reinforcing-loop gain (R1, R3) | recursive-AI R&D allocation; capex herding | momentum_gain +0.20 on silicon rent duration: herding extends queues and rents until it doesn't (overshoot ~2.3x). R1 saturation assumptions dominate long-run capability but almost nothing tradeable before 2030. |
| 6 | Information flows | queue transparency, capability evals, disclosure of lab commitments (RPO concentration!) | Cheap and powerful: the MSFT/AVGO/GOOGL circular-financing exposure only exists because disclosure is partial. Any regime forcing counterparty disclosure reprices the complex. |
| 5 | Rules | export controls, AI liability, robot safety certification | Export controls are the live rule-lever (China truce expiry Nov 2026 = MP's binary). Liability law is a B2-amplifier: it converts backlash into a hard adoption gate. |
| 4 | Self-organization | open-source models, robots building robots (R2) | R2's empirical gain is tiny this decade (ρ≈0.0 on 2032 outputs) — the "robots build robots" singularity is real but post-2033 in every sampled world. |
| 3 | Goals | national AI-race vs safety-first postures | Determines whether B2 backlash is *allowed* to bind. A race posture caps backlash_gain; a safety posture raises it. The displacement-timing bets (casualty shorts) are really bets on this. |
| 2 | Paradigm | "labor is optional" acceptance; UBI | The rates/gold leg of the book. transition_drag's tail (mild recessions in ~10% of runs) is the paradigm fight showing up in GDP. |
| 1 | Transcending paradigms | — | Not tradeable. Meadows would note the model itself is a paradigm and its blind spots (geopolitics, war, pandemics) are the real tail. |

## The three monitorables this analysis adds to the book

1. **Power-delay reform** (leverage #9/#8): any credible interconnection-queue
   or permitting reform → trim power-rent longs (GEV-class, merchant power),
   rotate toward silicon tolls — the model says the rent migrates there.
2. **External-funding share of AI capex** (the +0.52 credit driver): when the
   marginal capex dollar stops coming from hyperscaler cash flow (bonds, SPVs,
   vendor financing — already $250B+), the B4 accident probability rises
   steeply. This is a better crash indicator than any demand statistic.
3. **Integration friction, not capability benchmarks**: for casualty-short
   timing, enterprise deployment half-life (adoption_halflife) matters as much
   as the singularity date and far more than model quality. Watch agent
   deployment case studies, not eval scores.
