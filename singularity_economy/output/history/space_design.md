# Space Module Spec — `space.rs` for `singularity-econ`

**Verdict on the one question:** Orbital compute does **not** relax the terrestrial power constraint inside 2026–2036. It is tonnage-gated, not capital-gated: 1 GW-IT in orbit costs 10,000–30,000 t to LEO (10–30 kg/kW) against a 2026 world launch base of ~3,500–4,000 t/yr. Even growing at the model's own 2.5x/yr WWII ceiling *after* a Starship full-reuse proof (Bernoulli, p≈0.6–0.75 by end-2027), cumulative orbital GW-equiv stays <1 GW before ~2031 and reaches central 1–3 GW (bull 8–12) by 2033–2036, versus a terrestrial `ai_power` path in the hundreds of GW. The module's honest output is a **2034–2036 tail rent-clipper** (shaves `power_margin` excess 10–25% in the bull branch, ~0 in central) plus a long-run ceiling on power rents — it must leave the "power binds ≥6/11 years, never normalizes endogenously" result intact. The two gating thresholds: **launch price < breakeven_$/kg = 200 + 300×power_rent_index** (i.e. $200/kg vs unconstrained terrestrial per Google Suncatcher, ~$500/kg vs 2026-level queue rents per McKinsey/Starcloud — the model's own power rents endogenously widen the gate), and **kg_per_kw < ~20** (radiator+solar specific mass, which does NOT improve with cheap launch — Stefan-Boltzmann area at chip temps is launch-cost-orthogonal).

---

## 1. Stocks (5, plus one Pipeline)

Add `pub mod space;` with `SpaceParams`, `SpaceState`. All fields f64, annual step, same idiom as robotics.

```rust
pub struct SpaceState {
    pub cum_upmass_t: f64,        // Wright-law state. init 28_000.0 (cumulative kg-to-orbit since 2000, McDowell payload-mass series)
    pub launch_capacity_tpy: f64, // init 4_000.0 (2025 realized 3,193 t + trend)
    pub launch_cost_per_kg: f64,  // COST, init 2_000.0; derived from cum_upmass each year
    pub kg_per_kw: f64,           // specific-mass tech stock, init 40.0, floor 8.0
    pub orbital_gw_it: f64,       // energized GW-IT in orbit, init ~0.0 (2e-5)
    pub orbital_pipe: Pipeline,   // 2 stages (design/manifest → commission), init flow 0.0
    pub starship_proven: bool,    // Bernoulli drawn in MC harness, like incident_year
    pub second_supplier: bool,    // rent-wedge collapse Bernoulli
}
```

**Per-year step (call between the R4 block and the constraints block in `simulate`):**

```text
1. kg_per_kw *= (1 - specific_mass_decline);  kg_per_kw = kg_per_kw.max(kg_per_kw_floor)
2. launch_cost_per_kg = cost_2026 * (cum_upmass_t / 28_000).powf((1.0 - wright_lr).log2())   // Wright on cumulative mass
   price = launch_cost_per_kg * rent_wedge   // wedge 1.8 → 1.15 once second_supplier (SpaceX internal deployment pays wedge 1.2)
3. growth = if starship_proven { launch_growth_fast } else { launch_growth_slow };
   growth = growth.min(reg_cadence_ceiling(year));            // 0.6/yr pre-2029, 1.5 after (FAA lag)
   launch_capacity_tpy *= (1.0 + growth).min(2.5);            // WWII ceiling, hard
4. upmass = launch_capacity_tpy * 0.9;  cum_upmass_t += upmass
5. replacement_t = orbital_gw_it * kg_per_kw * 1000.0 * orbital_attrition   // B-replacement tax
   available_t = (upmass * (1.0 - preempt_share) - replacement_t).max(0.0)  // Starlink+defense pre-empt
6. power_rent_index = ((power_margin - normal_margin) / (margin_ceiling - normal_margin)).clamp(0.0, 1.0)
   breakeven = 200.0 + 300.0 * power_rent_index               // THE coupling to existing rent machinery
   gate = logistic((breakeven - price) / 50.0)                // soft economic gate
   allocated_t = available_t * gate
7. delivered_gw = orbital_pipe.step(allocated_t / (kg_per_kw * 1000.0))     // t → GW; 2-yr material delay; NO step_accel — launch cadence is regulatory/physical, ASI doesn't compress FAA reviews (defensible either way; keep it conservative)
8. orbital_gw_it = orbital_gw_it * (1.0 - orbital_attrition) + delivered_gw
9. orbital_gw_equiv = orbital_gw_it * orbital_effectiveness   // 0.60 penalty stack
```

Two regimes for growth (the 2016-2019 flat / 2019-2025 42%/yr lesson): `launch_growth_slow = 0.12`, `launch_growth_fast = 0.35`. `starship_proven` flips via MC draw p=0.7 by end-2027 (as of Jul 2026 Starship has never completed full orbital insertion — the gate is live, not formality).

## 2. Coupling into the power bottleneck

Exactly two lines change in `simulate()` — orbital GW-equiv relaxes **only** the power cap; chips, capital, and demand gates are untouched, so orbital compute units still consume silicon and capex through the existing machinery (this is the physics-honest structure for free):

```rust
// line ~734: power_headroom
let eff_power = ai_power + space.orbital_gw_equiv;   // was: ai_power
let power_headroom = (eff_power + power_additions - compute_stock * gw_per_unit).max(0.0);
// line ~779: power_utilization denominator
let power_utilization = (power_demand_gw / (ai_power + space.orbital_gw_equiv).max(1e-9)).min(1.35);
```

**Penalty structure** (`orbital_effectiveness = 0.60`): 0.95 availability (solar events) × 0.70 workload addressability (training + batch inference only; real-time inference can't fly; ground↔orbit multi-Tbps ingress undemonstrated) × 0.90 SEU/ECC/checkpoint + downlink overhead. Radiation is deliberately NOT a failure channel — it's solved-enough (Trillium 15 krad, H100s flying); it lives in the 5–6 yr life (`orbital_attrition = 0.18`) instead, which is the buried killer: whole-stack consumable vs 15-yr terrestrial facility. **No space→grid flow ever** — power beaming is ~40–45% end-to-end; orbital power is monetizable only through co-located compute.

Loop registry: **R5** (Wright launch learning: upmass → doublings → $/kg ↓ → gate opens → more upmass) with gain `r5_launch_learning` in `Loops`; **B11** (power-rent arbitrage: rents ↑ → breakeven ↑ → gate opens → orbital GW → rents ease — the model's only endogenous power relief, sized to a trickle); **B12** (replacement tax: stock × 18%/yr × kg/kW permanently consumes capacity). Setting `r5 = 0.0` and `orbital_effectiveness = 0.0` must exactly recover golden vectors.

## 3. Parameters and justification

| Param | Default | Justification |
|---|---|---|
| `cum_upmass_2026_t` | 28,000 | McDowell payload-mass series (pick ONE series; 16,185 t in orbit end-2025 + pre-2015 cumulative) |
| `launch_capacity_2026_tpy` | 4,000 | 2025 realized 3,193 t, 28%/yr trend |
| `wright_lr` | 0.212 | PNAS Nexus 2026 experience curve, 21.2%/doubling — on COST, not price |
| `rent_wedge` | 1.8 → 1.15 | F9 list price flat 2016–2026 while cost fell (monopoly); symmetric with power-rents mechanic. Second-supplier Bernoulli p≈0.35 by 2031 (New Glenn / Zhuque-3 heavy) |
| `launch_cost_2026` | $2,000/kg | F9 ~$2,700 reused price, Starship realized ~$2,500 at 40 t; cost basis lower |
| `kg_per_kw` init / floor | 40 / 8 | Independent engineering 34–59 kg/kW today; floor = radiator ~2 (T⁴ area floor at 50–80°C) + solar ~4 + compute/structure ~2. SpaceX's 10 kg/kW claim lives only in the MC bull tail (sample log-uniform [10, 60]) |
| `specific_mass_decline` | 0.15/yr | 85→10–15 kg/kW over ~decade per Starcloud/Suncatcher roadmaps, TRL-honest (LDR stuck at TRL 3-4 for 40 yrs — no 7x jumps before 2032) |
| `preempt_share` | 0.65 | Starlink replacement (20%/yr of 9,500+ sats), Kuiper, NSSL/Golden Dome floor — recurring demand, launcher's own payloads were 85% of 2019-25 growth |
| `orbital_attrition` | 0.18 | 5–6 yr whole-stack LEO life (drag, radiation, coating degradation) |
| `orbital_effectiveness` | 0.60 | penalty stack above |
| `p_starship_by_2027` | 0.70 | 13 flights, zero full orbital insertions, July 2026 booster loss; schedule-slip base rate 3–5 yr |
| MC ranges | — | tonnes/yr 2030: bear 8k / central 15–25k / bull 60k; orbital GW 2033: 0.3 / 1–3 / 8–12; $/kg cost 2030: $400–700 |

## 4. Validation contract (falsifiable, add to `tests/backtest.rs`)

1. **WWII ceiling on launch tonnage**: add `(launch_capacity_tpy, floor 500.0)` to the ratio list in `no_stock_outruns_wwii_mobilization_ceiling`. Domain evidence the ceiling is right: 70 years of spaceflight max is 1.72x/yr global (2023-24), 1.97x single-program (F9 2021-22) — nothing has ever violated 2.5x. Launch tonnage is the one stock that can legitimately *approach* the ceiling (Starship per-flight mass step).
2. **Backcast 2016→2025**: initialized at 338 t/yr and slow-regime growth, flipping to fast-regime in 2019 (Starlink self-demand proxy), the module must land 2025 upmass in [2,200, 4,500] t and cost in [$1,500, $3,500]/kg. Falsifies the two-regime + Wright structure jointly.
3. **Tonnage binds before economics**: in ≥90% of MC draws, `orbital_gw_equiv < 1.0` before 2031 even when the price gate is forced fully open (`gate = 1.0`). If economics ever binds first pre-2031, the mass budget is mis-specified.
4. **Headline result survives**: with the module on at defaults, `Binding::Power` still ≥6 of 11 years (existing test unchanged), and 2036 `power_margin` reduction vs module-off is ≤25% of the excess over `normal_margin` — and exactly 0 in draws where `starship_proven` never fires.
5. **Physics floor**: delivered GW per allocated tonne never exceeds 1/(8,000 t/GW) in any draw (kg_per_kw floor 8) — catches any accidental coupling of kg/kW to launch cost, which are orthogonal.
6. **Ablation parity**: `r5_launch_learning = 0` + `orbital_effectiveness = 0` reproduces golden vectors bit-for-bit (regression guard, same pattern as `geo_shocks: Vec::new()`).

## 5. What NOT to model

- **SBSP / power beaming to ground** — no space→grid flow, period. 50 years of studies, mW-received demos, ESA's own roadmap says commercial GW 2040. Its absence *is* the finding that power never normalizes.
- **Kessler / debris as a trend** — at most a `GeoShock` variant (1–3%/yr hazard, freezes deployment 1–2 yr, correlated SPCX downside); skip in v1.
- **Launch counts, vehicles, sites** — tonnage only (counts grew 16%/yr vs mass 28%/yr; counts mislead).
- **Radiation as failure risk** — folded into attrition + effectiveness. Keep a 20% MC branch where fleet-scale HBM failures push attrition to 0.30.
- **China space stack** — lags US 4–6 yrs on reusable heavy-lift (10% of world mass on 28% of flights); immaterial in-horizon.
- **Orbital latency/market microstructure, insurance, robotic servicing** — servicing is optionally one Bernoulli (life 5→10 yr post-2032, bull tail only).
- **Every SpaceX narrative number** — 1M t/yr (300x world upmass), 100 kW/t, $10–20/kg, 1M-sat FCC filing (<10% of filed sats ever fly). Default to independent numbers; company claims live only in the bull tail.

## 6. SPCX book implications

- The orbital-compute leg of the $1.50T mark ($114, below the $135 offer, −50% from ATH) is a **real but 2033+ call option** — this module prices it as tonnage-gated optionality, not terminal value. Any bank model showing orbital GW relieving power pre-2031 is falsified by test #3's mass budget.
- **Picks-and-shovels asymmetry**: SpaceX gets paid (Google Suncatcher launches, Starcloud manifests) whether or not orbital DCs pencil; the rent wedge means SpaceX-internal deployment crosses the gate ~2 yrs before third parties. Upside concentrates in SPCX; the RKLB/rad-hard basket is a 5–10% capex skim at best (Neutron 13 t is not DC tonnage; NVIDIA's commercial-node rad-tolerance kills the rad-hard-prime moat).
- **Binary to watch before the lockup waterfall**: full orbital insertion + booster recovery before the final Dec 8 2026 release (book says Dec 9 — fix). Waterfall supply peaks Aug–Nov (20% + 7% tranches + ~28% Q3-earnings release); the better entry is the Oct/Nov tranche, Dec 8 is likely anticlimax. F13 (sats deployed, booster lost) ≈ valuation-neutral.
- **Downside correlations the module makes explicit**: SPCX is long power-rent persistence (rents are what make orbital investable — B11) and short debris regulation (a Kessler-class clamp hits Starlink and the compute option simultaneously). The revenue mix (61–67% Starlink at 63% EBITDA, defense launch floor) argues less AI-beta than the drawdown priced; the funding hole ($84B/yr external need, no FCF before 2035 per MS) is the real bear case, and this module's gate structure says the orbital narrative cannot close it this decade.

**Files**: implement as `/home/user/Amulet-Map-Editor/singularity_economy/rust/singularity-econ/src/space.rs`; wire-in points at `src/lib.rs` lines ~734 (`power_headroom`) and ~779 (`power_utilization`); ceiling test extension in `tests/backtest.rs` (`no_stock_outruns_wwii_mobilization_ceiling`, line 249); new `Loops` gain `r5_launch_learning` alongside line 45. Python spec (`model_v2.py`) must be updated first per the parity contract in `tests/parity.rs`.