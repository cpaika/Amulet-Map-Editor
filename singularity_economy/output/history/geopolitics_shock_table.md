# geopolitics.rs — Shock Module Design Specification

Target crate: `/home/user/Amulet-Map-Editor/singularity_economy/rust/singularity-econ/src/geopolitics.rs`
Integration points (verified against `src/lib.rs`): `chip_capacity` stock + `chip_pipeline_stages` (Pipeline), `power_pipeline_stages`/`power_additions`, `robot_components` pool + `component_base_growth`, `credit_gain`/`internal_funding_share`, `demand_growth_base`/adoption, `max_displacement_rate` (state input). Model timestep = annual; sub-year durations apply pro-rata within the year.

---

## 0. Core state & interfaces

```rust
pub struct GeoState {
    pub escalation: f64,          // E ∈ [0,6]; 0=detente, 2=chronic friction (2026 init=2.0), 4=quarantine, 5=blockade, 6=war
    pub active: Vec<ActiveShock>, // shock id, years_remaining, severity draw
    pub magnet_capacity_exchina_kt: f64,   // explicit stock #1 (init 7.0)
    pub grid_equipment_capacity: f64,      // explicit stock #2, index 1.0 = 2026 US/allied LPT+switchgear throughput
    pub nontaiwan_leading_edge_share: f64, // exogenous ramp + endogenous accel (init 0.05)
    pub metals_cost_index: f64,            // 1.0 baseline; mean-reverting
    pub credit_spread_bp: f64,             // additive geopolitical spread
    pub years_in_detente: f64,             // severity amplifier (concentration persists)
}
```

Effect vector every shock must emit (exactly the six required channels):

```rust
pub struct ShockEffects {
    pub chip_capacity_mult: f64,        // multiplies effective chip capacity (flow or stock per shock kind)
    pub power_delay_years: f64,         // added to power Pipeline transit
    pub power_haircut_mult: f64,        // multiplies energized-GW additions
    pub robot_component_cost_mult: f64, // multiplies component cost; >1.5 also gates component supply (see S2)
    pub metals_cost_mult: f64,          // multiplies metals_cost_index
    pub demand_mult: f64,               // multiplies AI capex demand / adoption inflow
    pub credit_spread_bp: f64,          // added to credit_spread_bp; feeds credit_gain *= 1/(1 + spread/1000)
}
```

---

## 1. Shock event table (8 types)

Annual hazard = `base_p × esc_mult(state)`, clamped to [0, 0.6]. Durations: LogNormal(μ,σ) in years, truncated at [min,max].

| ID | Shock | base_p/yr | Escalation multiplier (state-dependent) | Duration |
|---|---|---|---|---|
| **S1** | `TaiwanCrisisSpike` (missile tests, cable cuts, days-long closures) | 8.0% | ×(1 + 0.25·(E−2)) · ×1.3 in 2027–2030 window | LN(μ=ln 0.15, σ=0.5), [0.05, 0.5] |
| **S2** | `TaiwanQuarantine` (CCG customs regime, throughput throttle) | 3.5% | ×1.5 if S1 fired in prior 12mo; ×1.5 in 2027–2030; ×(1+0.2·max(0, share_exchina−0.25)/0.05) (silicon-shield erosion) | LN(ln 0.5, 0.4), [0.25, 1.5] |
| **S3** | `TaiwanBlockade` (kinetic-adjacent, LNG cut) | 1.5% | mostly via ladder (§2); ×1.5 in 2027–2030; ×1.5 if China hard-landing active (diversionary, 12–18mo lag) | LN(ln 1.0, 0.4), [0.5, 2] |
| **S4** | `TaiwanInvasion` (war; permanent stock destruction) | 1.2% | ×1.5 in 2027–2030, decaying ×0.85/yr after 2032 (demographic window closes); ×(1 + 0.5·china_selfsufficiency_gap_closure) | fixed 4 yrs effect horizon; capacity rebuild ≤18%/yr of lost stock (ASML EUV cap) |
| **S5** | `ExportControlRung` (major US rung + Chinese tit-for-tat within the year) | 25% | ×(1 + 1.0·displacement_rate/0.22) (domestic labor backlash → protectionism); ×0.5 if E<1 (detente) | LN(ln 1.5, 0.4), [0.5, 3] |
| **S6** | `MineralsSqueeze` (REE/magnet licensing pulse; Ga/Ge/W/Sb co-fire) | 30% (2026–27, scheduled Nov-2026 trigger), decay −2pp/yr to 20% floor 2030 | fires automatically at 80% if S5 fires (retaliation ≤1yr lag); ×1.3 if Myanmar-type upstream stress (fold into severity, not separate shock) | LN(ln 0.35, 0.5), [0.15, 1.0], + 12-mo price-decay tail |
| **S7** | `MineralsEmbargo` (full China REE/magnet/graphite cutoff of West) | 1.5% standalone | ladder-dominated: p→80% conditional on S3/S4 (§2); ×1.5 if S6 fired twice in 3 yrs (ratchet) | LN(ln 1.5, 0.4), [1, 3] |
| **S8** | `EnergyChokepoint` (Hormuz/Red Sea/EU-gas joint macro draw — ONE correlated event, not three) | 8% (post-2023 updated prior; 2026 initializes ACTIVE, resolution hazard 60%/yr) | ×1.4 while any Taiwan shock ≥S3 active | LN(ln 0.75, 0.5), [0.25, 2] |

Grid-equipment crunch (transformers 128–144wk lead times) is modeled as the **baseline constraint via stock #2 (§3)**, not a shock draw — it is already binding, per briefs.

### Quantified effects per shock (the six channels)

| ID | chip_capacity_mult | power delay / haircut | robot_component_cost_mult | metals_cost_mult | demand_mult | credit_spread_bp |
|---|---|---|---|---|---|---|
| **S1** | 0.95 (flow, duration only) | +0 yr / ×1.0 | ×1.08 | ×1.05 | ×1.02 (panic pre-ordering) | +150, decay 1 yr |
| **S2** | 0.70 (throughput/flow shock; capacity intact; backlog clears +2 qtrs after) | +0.25 yr / ×0.87 | ×1.30 | ×1.15 | ×0.90 (financing risk) | +250 |
| **S3** | 0.20 (stock offline U[0.15,0.30]; recovery 2–4 qtrs post-lift) | +0.5 yr / ×0.35 during; ×1.2 catch-up 2 yrs after | ×2.0, component supply ×0.6 | ×1.5 (Cu/Ga/REE war premium) | ×0.70 during; ×1.35 reshoring boom 3 yrs after | +600; equity −25–40% analog |
| **S4** | 0.12 permanent destruction (survivor floor = nontaiwan_leading_edge_share + 0.05 Samsung/Intel); rebuild ≤18%/yr | +1 yr / ×0.15 yrs 1–2, ×0.5 yr 3 | ×3.0, supply ×0.3 yrs 1–2 | ×2.5 (REE ×5–10, Cu ×1.5 blended) | ×0.35 yrs 1–2, ×1.4 state-directed yrs 3–6 | +1000 |
| **S5** | 0.99 (world; China tranche −80% frontier access) | +0 / ×0.97 (China grid-gear friction) | ×1.10 | ×1.25 (Ga/Ge/Sb legs) | ×1.02 US, China substitution capex up | +75 |
| **S6** | 0.995 | +0 / ×0.97 (wind + BESS channel only) | ×1.5·sev(t) for 2–3 mo, tail to ×1.15 over 12 mo | ×1.4 (Dy/Tb ×2–4, NdPr ×1.4 blended) | ×0.999 | +25 |
| **S7** | 0.98 | +0.25 yr / ×0.88 (BESS, solar, transformer imports) | ×3.5 yr 1, ×2.0 yr 2; **hard supply gate** ×(1−0.85·china_dependence(t)) where china_dependence = 1 − magnet_capacity_exchina/western_demand | ×3.0 | ×0.95; robot demand deferred not destroyed | +200 |
| **S8** | 1.0 (never touches chips — per briefs) | +0.25 yr / ×0.95 global (×0.5 Gulf tranche 3–6% of adds) | ×1.05 (freight/energy) | ×1.35 (oil ×1.6, LNG ×2 blended into index) | ×0.96 | +150 |

Implementation notes: S2 is a **flow** shock (apply to Pipeline outflow), S3 a recoverable **stock** shock, S4 permanent stock destruction — do not implement with one mechanism. Chip shortage gates power: `energized_gw = min(shell_completions, gpu_deliveries)`; the S3/S4 power haircuts above are the derived result, so if you implement the min() coupling, drop the explicit power haircut for S3/S4 to avoid double-counting. Hoarding bullwhip: any shock with chip_mult ≤0.7 adds demand ×1.1 for 2 quarters, matching glut (×0.93) after resolution.

---

## 2. Correlation / escalation ladder

Markov transitions (per year while source state active) — **not independent draws**:

```text
S1 fired            → S2 hazard ×1.5 for 12 mo (ratchet; each S1 occurrence permanent +10% baseline stockpiling demand)
S2 active           → P(→S3) = 25%/yr        (CSIS: blockades escalate, don't resolve)
S3 active           → P(→S4) = 25%/yr        (20–30% band, midpoint)
S3 or S4 fires      → S7 fires with P = 0.80 immediately (Taiwan blockade ⇒ minerals embargo ~certain)
S5 fires            → S6 fires within same year with P = 0.80 (tit-for-tat, 1–4 wk observed lag)
S5 fires            → S2..S4 hazards ×1.25 for 2 yrs (tension state); E += 0.5
S6 fires ≥2 in 3yrs → S7 hazard ×1.5
S7 fires            → S5 hazard ×1.5 (spiral)
S8 active           → S3/S4 hazard ×1.1 only (separate theater, weak coupling); EU-gas + Gulf-DC legs co-draw within S8 (ρ=1, one Iran driver)
Detente (E<1)       → all hazards ×0.5, BUT severity_mult += 3%/yr of detente (reshoring stalls, concentration persists)
Each year E≥3       → chokepoint concentration decays extra 2pp/yr (forced substitution)
```

Escalation index update: `E += 1.0·(S5) + 1.5·(S3) + 2·(S4) − 0.3/yr` decay toward 2.0 floor (up fast, down slow — hysteresis). Cross-theater rho for Monte Carlo without full Markov: draw {S2,S3,S4} on one Taiwan latent factor, {S5,S6,S7} on one decoupling factor, correlate the two factors at ρ=0.65; S8 independent except the ×1.1 term.

---

## 3. Explicit critical-minerals stocks (exactly 2)

**Stock A — ex-China NdFeB magnet capacity (kt/yr).** Init 7.0 (2026: MP 3 + eVAC 2 + Japan spare ~2). Committed pipeline: 20–30 kt by 2029, 35–50 kt by 2031 (MP 10X 2028, Lynas 12kt NdPr ~2028, eVAC→12kt, Niron 2027+). Buildout delay: 3-stage pipeline (≈3 yrs commissioning). Western demand rises with robot stock: 4 kt per 1M humanoids/yr — hard coupling: Western robot production >3M units/yr requires Chinese imports OR this stock; S7 severity uses `china_dependence = 1 − stockA/western_magnet_demand`.

**Stock B — grid-equipment (LPT/switchgear/GOES) capacity index.** Init 1.0, **already binding**: baseline power pipeline carries ~50% of incremental 2026 US DC capacity slipping 6–18 mo (encode as baseline `power_pipeline_stages` +1 through 2028). Capacity: 1.0 → 1.25 (2028, Siemens Charlotte 2027 + Hitachi S. Boston 2028) → 1.6 (2030). Buildout delay: 2.5 yrs. Deepening-crunch tail (p=25%/yr through 2028): index ×0.8 for 2.5 yrs.

Copper is deliberately **not** a stock: briefs show it is a price channel, not a quantity gate, through ~2028 (Grasberg −3% supply → ×1.4 price, <5% physical buildout change) and mine supply is pipeline-fixed to 2032. Copper lives inside `metals_cost_index` with pass-through elasticity 0.08 to robot cost and 0.10 to power capex.

---

## 4. Mitigation / onshoring dynamics

```rust
// share ramps (baseline, no scare):
nontaiwan_leading_edge_share: 0.05 (2026) → 0.08 (2028) → 0.13 (2030) → 0.22 (2033)   // TSMC AZ/Intel 18A/Samsung; CoWoS lags: floor share −3pp until 2028
onshoring_scare_multiplier: after any shock ≥S2 (chips) or ≥S6×2 / S7 (magnets):
    buildout inflow ×2.75 for 4 yrs   // observed: China semicap localization slope tripled post-Oct-2022; MP/eVAC post-2025
    subject to delay: fabs 3 yrs, packaging 2 yrs, magnets 2–3 yrs, transformers 2.5 yrs — NO step relief
severity_decay: S6/S7 robot-cost and supply effects ×(1 − 0.12)^max(0, year−2028)     // ~10–15%/yr post-2028; HREE leg decays at only 5%/yr (China >90% Dy/Tb through 2030)
S3/S4 chip haircut floor: 1 − 0.90·taiwan_share(t) where taiwan_share falls ~4pp/yr    // −90% (2026) → −65% (2030-31)
detente_penalty: severity multipliers grow +3%/yr while E<1 (mitigation capex stalls)
```

---

## 5. Parameter value table with justification

| Parameter | Value | Brief justification |
|---|---|---|
| S1 p | 8%/yr | Taiwan brief scenario table; 1995-96 base rate + 2/yr PLA exercise tempo normalized |
| S2 p | 3.5%/yr | Taiwan brief; Metaculus 10% any-attack-or-blockade 2026 minus kinetic mass |
| S3 p | 1.5%/yr | Taiwan brief; CSIS blockade wargames; Bloomberg −5.3% world GDP anchor |
| S4 p | 1.2%/yr | Metaculus ~37-43% by-2030 cumulative → 1–1.5%/yr pure-invasion after removing coercion; consistent with 2.5–3%/yr great-power-war base rate (which includes non-Taiwan theaters — residual assigned to S8 escalations) |
| 2027–30 window ×1.5 | ×1.5 | Davidson-window PLA capability deadline; demographics say hazard rises then falls post-2032 |
| S2→S3 25%/yr, S3→S4 25%/yr | 0.25 | CSIS "Lights Out": blockades escalate rather than resolve; brief gives 25% / 20–30% |
| S3/S4 ⇒ S7 p=0.8 | 0.8 | REE brief: "correlate 0.8 with any Taiwan chip shock"; export-controls brief ρ 0.6–0.7 |
| S5 p | 25%/yr | Observed 1.5–2 major rungs/yr 2022–26; macro base rate 1.3/decade pre-2018 accelerating — 25% is the post-2018 regime |
| S5⇒S6 p=0.8, lag ≤1yr | 0.8 | Every US BIS action drew Chinese minerals response within days–weeks, 6 events in 30 mo |
| S6 p | 30%→20% | 2 control actions/yr 2023–25; Nov-10-2026 suspension expiry = scheduled trigger (35–40% 2026-27 per brief; 30% conservative given detente) |
| S7 standalone p | 1.5%/yr | 2010 Japan REE embargo precedent; full embargo ≈ 9%/yr total in briefs but ~80% of that mass is Taiwan-conditional, leaving ~1.5% standalone |
| S8 p | 8%/yr recurrence | Hormuz realized 2026 (prior ~2–4%/yr updated ×2-4); Red Sea 3-for-3 years folded in |
| S3 chip stock ×0.15-0.30 | U draw | LNG 11d/coal 49d → fabs ≤30% util within 6 wks; brief −70 to −85% |
| S4 chip ×0.12, rebuild ≤18%/yr | 0.12, 0.18 | Survivor floor = AZ N4 + Samsung + Intel ≈ 10–20%; ASML EUV output 50–60/yr caps rebuild 15–20%/yr |
| S2 chip flow ×0.70 | 0.70 | Quarantine brief: export throughput ×0.6–0.8, capacity intact |
| S7 robot supply gate 0.85·dependence | 0.85 | Ex-China magnet availability −80–90% instantly; 1–3 mo Western inventories |
| S6 robot cost ×1.5 pulse, 12-mo tail | 1.5 | Apr–Jun 2025 realized: magnets +72%, exports −93% then 7×rebound; magnets 5–10% of humanoid BOM |
| Magnet stock ramp 7→45 kt | per §3 | MP 10kt-by-2029 + Lynas + eVAC 12kt path + Niron; briefs' consensus 35–50 kt 2031 |
| Grid stock 1.0→1.6 by 2030 | §3 | $2bn+ named plants (Charlotte Jun-2027, S. Boston 2028); lead times bind through 2028 |
| Scare buildout ×2.75/4yrs | 2.75 | China WFE localization 7%→35% in 5 yrs under controls = ~3× slope; CHIPS/$165B→$465B TSMC AZ escalation post-scare |
| Severity decay 12%/yr post-2028 | 0.12 | REE brief: "shock severity decays 10–15%/yr after 2028"; HREE slower (5%/yr) |
| Metals index mean-reversion half-life 2.5 yrs | 2.5 | Cobalt/nickel substitution self-limits 2–3 yrs; export-control price spikes halve in 12–18 mo (Russia/Iran leakage law) |
| Credit spread S3 +600bp / S4 +1000bp | | Equity −25–40% war analog; risk premium +100–200bp for mere crisis (S1) per brief |
| Demand ×0.7 (S3) / ×0.35 (S4) | | Bloomberg −5.3%/−10.2% world GDP; AI capex demand collapses then state rebuild ×1.4 |
| Detente hazard ×0.5, severity +3%/yr | | Export-control brief: detente 25%/yr state suppresses reshoring, raising later severity |
| Displacement→S5 multiplier (1 + dr/0.22) | | Ties module to `max_displacement_rate=0.22`; labor backlash → protectionism (model's B2 loop) |

---

## 6. Validation contract (property tests → `tests/geopolitics.rs`)

1. **P(≥1 major chip supply shock, S2 or worse, 2026–2036) ∈ [35%, 55%]** over ≥10k Monte Carlo paths. (Analytic check: combined ~6%/yr hazard with window multiplier → ~45%.)
2. **Invasion tail shape:** conditional on S4, leading-edge `chip_capacity_mult ∈ [0.10, 0.40]` for ≥2 consecutive years, and recovery never exceeds 20% of lost capacity per year; a 2031 invasion produces a floor ≥1.5× higher than a 2027 invasion (mitigation ramp).
3. **Ladder consistency:** P(S3 within 1 yr | S2 active) ∈ [20%, 30%]; P(S7 | S3 or S4) ≥ 0.75; P(S6 within 1 yr | S5) ≥ 0.7. No path reaches S4 without E ≥ 4 first in ≥90% of invasion paths (ladder, not bolt-from-blue, with ≤10% surprise allowance).
4. **Recurrence & decay:** E[# S6 episodes 2026–2036] ∈ [2.0, 4.0]; an S6 fired in 2032 produces ≤55% of the robot_component_cost impulse of an identical 2026 draw; metals_cost_index returns within 20% of baseline ≤3 yrs after any non-S4 shock.
5. **Macro base-rate match:** per-decade frequencies across paths — geopolitics-driven credit-spread event >300bp with demand_mult <0.9: 0.3–0.6 (target 0.35 equity-drawdown analog); metals index doubling within 12 mo: 0.4–1.0 (target 0.7); chokepoint closure >1 mo (S3+S8 severe): 0.2–0.8 (target 0.27 pre-2023, updated up); any great-power-war state (S4 or S8-escalated): 1.5–3%/yr unconditional.

Also assert (regression guard): median (no-shock) path leaves all six channels within 2% of baseline — geopolitics must be a tail module, not a hidden drag; and S8 never moves `chip_capacity_mult` (channel-separation invariant from the chokepoint brief).