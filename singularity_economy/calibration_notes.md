# Calibration Notes

Parameters updated 2026-07-24 from the 17-agent research sweep (primary sources:
Q2-2026 earnings, company guidance, industry forecasts). Key evidence per
parameter, so every number can be challenged.

## Compute supply chain

| Param | Value | Evidence |
|---|---|---|
| `ai_capex_2026` | $0.65T | Big-4 hyperscaler 2026 capex ~$725B planned (+77% yoy), ~75% AI-tied; broader universe $600–750B incl. neoclouds. 2027 consensus >$1T. |
| desired-capex growth | 32%/yr base + singularity shock | 2026→2027 consensus step is ~+50%; Goldman cumulative 2025–2030 $5.3T. 32% base keeps pre-shock path just under consensus (conservative). |
| `chip_capacity_growth_max` | 0.55 | CoWoS capacity +~80%/yr (35k→75k→125–140k wpm) but ≤7nm wafer capacity only +69% across 2024–28 and EUV tool output +30–48%/yr; blended accelerator-output ceiling ~55%. MC prior 0.35–0.85. |
| `silicon_share_of_capex` | 0.55 | Accelerator spend $350–450B of $600–750B total 2026. |
| `compute_deprec` | 0.25 | 4–6yr book lives vs 2–3yr competitive lives debate (CoreWeave 6yr vs bear case 2–3yr); 25% ≈ 4yr. |

## Power supply chain

| Param | Value | Evidence |
|---|---|---|
| `ai_power_2026` | 55 GW | US DC total 75.8 GW (2026, all datacenters); AI-specific globally ~50–60 GW. |
| `power_additions_2026` | 24 GW/yr | ~12 GW US DC additions 2026 (only ⅓ under active construction), global AI-relevant additions ~20–28 GW. |
| `power_additions_growth_max` | 0.28 | GEV turbine capacity 20→24→30 GW/yr by 2030 (~+11%/yr); global GT orders ~70–100 GW/yr vs 40–50 historical; transformer lead times 3–5 yrs; MV switchgear sold out through 2028; interconnection queues 5–10 yrs. 28% is generous vs turbine-only ramp — assumes solar+storage+restarts fill gap. MC prior 0.15–0.42. |
| Turbine pricing evidence | — | Heavy-duty $/kW → ~$600/kW by end-2027, ~3× 2019; GEV sold out through 2030, selling 2031 slots. Justifies `bottleneck_margin`=0.55 for power-equipment sector when power binds. |

## Labor & casualty pools

| Param | Value | Evidence |
|---|---|---|
| `it_services_pool` | $1.55T | Worldwide IT services subset of $6.31T IT spend; ACN/TCS/INFY revenue base. |
| `bpo_pool` | $0.36T | Global BPO $358.6B (2026). |
| casualty de-rating already priced | — | ACN -56% (9.7x fwd), CTSH -46% (7.4x), CNXC 1.9x fwd, TEP 4.0x; but ADP 20.3x, PAYX ~21x, RHI ~23x fwd still price durability → shorts must be name-specific, not sector-wide. |
| displacement leading indicators | — | TCS headcount -3.85% yoy (first sustained decline); recent-grad unemployment 5.7%, underemployment 42.5%; ~80% drop in entry-level analyst postings; monday.com -20% workforce. Supports adoption_halflife ~1.6y post-singularity, max_displacement_rate 0.22. |

## Robotics

| Param | Value | Evidence |
|---|---|---|
| `robot_prod_2028_m` | 0.12M | 2025 actual 13.3k (+480%); 2026E >50k; Tesla target 50–100k 2026 (0 produced through Q2-26 — targets slip); Figure BotQ 12k/yr capacity; consensus 250k by 2030. Thesis case 120k in 2028 sits between consensus and OEM promises. |
| `robot_cost_2028_k` | $50k | China BOM ~$35k (2025) → <$17k by 2030; Western builds $90–100k; Unitree G1 retail $16k. Blended $50k 2028, Wright 22%/doubling, $8k floor. |
| `component_capacity_growth` | 0.90 | Reducer capacity: Nabtesco RV doubling by 2026; Leaderdrive +1M/yr; Shuanghuan +500k/yr — components CAN nearly double annually. Actuators 40–56% of BOM; 14–40 reducers/humanoid; 26+ week Western lead times. |
| Rare-earth check | — | ~2–4 kg NdFeB per robot; 10M robots/yr = 20–40 kt vs China ~300 kt production — magnets constrain the WEST (94% China share), not the world. MP/Lynas are geopolitical-scarcity plays, not global-shortage plays. Noted for trade construction. |

## Macro

| Param | Value | Evidence |
|---|---|---|
| `world_gdp` | $115T | 2026 nominal. |
| labor income data | — | ADP June-26 payrolls +98k; pay growth 4.4%; white-collar softness concentrated in entry level so far. |
| copper | — | DC intensity 27–50 t/MW; thesis delta +1.5–2.5 Mt/yr by 2030 vs 28 Mt market and 17-yr mine lead times → structural deficit supports commodity legs. |

## Deliberate conservatisms (bias against the thesis)

1. Realized capex is capped by the binding constraint — the model treats
   announced capex as desire, not delivery (2026: $0.51T realized vs $0.65T
   announced, reflecting power-gated energization).
2. `addressable_cognitive` 0.85 < 1.0 and adoption is logistic, not step.
3. Robot HEW 1.4 assumes robots beat one human only via multi-shift, not skill.
4. Discount rate 12% in valuation layer; fizzle scenario gets 10% probability
   even though the user's thesis says singularity happens.

## v2 additions (2026-07-24, post code-review)

| Param | Value | Rationale |
|---|---|---|
| `chip_capacity_2026` | 0.28 pre-delivery | With start-of-year pipeline delivery, post-delivery 2026 capacity ≈ $0.36T ≈ 2026 silicon demand — "sold out" as observed (CoWoS/HBM). |
| `ai_power_2026` / `power_additions_2026` | 58 GW / 30 GW/yr | Restores the 2026 realized-capex anchor (~$0.49T) with un-decayed first-year power intensity (55 GW/unit held in 2026 per review fix 3). |
| `chip_base_growth` etc. | order-rate semantics | Growth parameters are ORDER rates entering 2-3 stage pipelines; realized capacity growth lags and is lower at steady state (review finding 5 — documented, not re-tuned). |
| `price_adjustment` | 0.6 | Contract/LTA price inertia; damps the endogenous hog-cycle to realistic amplitude. |
| `afford_gain` | 0.4 | B3 closure gain; 0.8 rang violently against the 1-yr information delay. |
| `debt_amortization` | 0.90 | Prior-stock survival; parameterized per review finding 9. |
| IP-toll sector | 18% slice, no B1 | Litho/EDA/IP share of silicon flow; monopoly capacity grows at base rate only — implements the design's IP-moat rent-persistence contrast. |
