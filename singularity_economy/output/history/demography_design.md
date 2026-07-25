# Demography Layer Design — `demography.rs`

**Design stance (Meadows):** demography enters this model as (a) a set of slowly-drifting sinks and sources on the two labor stocks, (b) a signal-filter between raw displacement and the political rate-detector in `society.rs`, and (c) two slow stocks (solidarity, blocked entrants) that determine *which* political channel fires and *how big* the transfer response can be. It is not a population model. Everything below couples to the existing structure (verified against `src/lib.rs` and `src/society.rs`: `attrition_threshold: 0.04`, `transfer_step: 0.04`, `transfer_cap: 0.15`, `transfer_trigger_rate: 0.02`, `election_period: 2`, `labor_power`, `physical_addressable: 0.65`, annual step, disp_rate = year-over-year delta of cumulative `cog_displacement`).

---

## 1. New stocks (5) and the pool conversion

The static `cognitive_workers_m: 950.0` and `physical_workers_m: 2400.0` move from `Params` to `DemographyState` and become dynamic via entry/retirement flows. That is a conversion, not a new stock. New stocks:

| # | Stock | Init (2026) | Units | Dynamics |
|---|-------|-------------|-------|----------|
| D1 | `blocked_entrants_m` | 8.0 | M workers | in: hiring-freeze diversion; out: underemployment drain 0.6/yr → physical pool; slow scar decay 0.07/yr |
| D2 | `care_demand_m` | 185.0 | M worker-equiv | grows 2.6 %/yr (80+ cohort demographics, exogenous) |
| D3 | `care_workers_m` | 162.0 | M workers (subset of physical pool) | relaxes toward `care_demand_m`, supply growth capped 2 %/yr; absorbs displaced physical workers |
| D4 | `migration_openness` | 0.35 | 0–1 policy stock | asymmetric ratchet: max −0.5/yr down, +0.05/yr up; election-quantized restriction events |
| D5 | `solidarity` | 0.75 | 0–1 slow stock | pulled toward composition-implied level (0.80 cognitive-wave / 0.45 physical-wave); eroded by chauvinism ∝ transfer_share × perceived out-group |

Pool flows (per year, on the now-dynamic pools):

```
cog:  entry  = 0.026 * pool * (1 - freeze_frac)     // baseline grads absorbed
      retire = r_ret_cog(t) * pool                   // 0.025 → 0.021 linearly 2026→2036
phys: entry  = 0.026 * pool + 0.6 * blocked_entrants_m   // incl. underemployment spillover
      retire = 0.018 * pool
```
Net drift with zero AI: cognitive +0.1 %/yr (defensibly ~static), physical +0.8 %/yr → ~2,600 M by 2036. The physical denominator was the material error in the static spec (+200 M over the decade).

**Displacement split (the core filter).** Raw cognitive displacement demand `D_cog` (M/yr, from the AI layer) is routed:

```
freeze      = min(D_cog, 0.7 * cog_entry_target)   // Dauth channel: entrants first
→ blocked_entrants_m += freeze                      // never enters visible rate
visible_cog = (D_cog - freeze) / cog_pool           // this is what society.rs sees
```

Physical: `visible_phys = max(0, robot_HEW_added − vacancy_gap(t) − care_absorption) / phys_pool`, with `vacancy_gap(t)` = 8 M/yr (2026) → 14 M/yr (2036) (aging-core working-age decline: China −5→−10 M/yr, EU −2→−2.5, JP/KR). At current fleet trajectory (0.4 M units/yr ≈ 1 M HEW/yr) this stays pinned at 0 until mid-2030s: robots fill vacancies before they displace.

---

## 2. Explicit couplings — existing parameters made endogenous

| Existing parameter | Becomes | Formula | Rationale |
|---|---|---|---|
| `attrition_threshold` (0.04 const) | `attrition_threshold(t) = r_ret_cog(t) + 0.015` | 0.040 (2026) → 0.036 (2036) | Threshold = retirement absorption + churn slack. Reproduces current default at t₀; decays as the boomer wave (Peak-65 2024–27) passes — later trigger, sharper snap. **Double-count guard:** retirement absorption lives ONLY here; it is never also subtracted from `visible_cog`. |
| displacement input to `society.rs` | `visible_cog + visible_phys` (not raw) | see split above | Hiring freezes and vacancy-filling robots produce zero layoffs; keying politics on raw displacement fires 2–4 yrs too early. |
| `transfer_cap` (0.15) | `(0.15 − age_creep(t)) * (0.70 + 0.30*solidarity)` | age_creep = 0.0025/yr GDP | Alesina-Glaeser: −5pp GDP sustained ceiling at US-level heterogeneity; fiscal aging pre-empts 1.5–4pp headroom by 2036. Chronic cap ≈ 9.5–11% out-group-coded, 13–15% in-group-coded. |
| `transfer_step` (0.04) | `0.04 * (0.5 + 0.5*solidarity)` | crisis mode (`crisis_rate` exceeded) bypasses both multipliers for 2 yrs | COVID evidence: heterogeneity penalizes the chronic ceiling, not crisis speed. |
| `physical_addressable` (0.65) | `0.65 − care_workers_m / phys_pool` | 0.582 (2026) → ~0.56 (2036) | Care is the automation-resistant floor; Japan's 20-yr experiment: 7.6% facility adoption of transfer robots, >50% shelfware. |
| robot fleet demand (`fleet_target`, lib.rs:790) | `+= 0.5 * unmet_care_gap_m / HEW_per_unit` | `unmet_care_gap = care_demand − care_workers − 0.3*care_robot_equiv` | China's 10M-caregiver gap is the earliest mass humanoid market; pull-forward ~1–2 yrs. Pulled units carry **0 displacement weight** (gap-filling, augmentation regime). |
| `labor_power` update | `+ 0.005 * core_scarcity(t)` term; blocked_entrants contribute **zero** | core_scarcity ∝ vacancy_gap | Aging-core scarcity props wages until AI wins (~2030); blocked youth have no strike power — sentiment and labor_power must diverge (that divergence is what routes pressure to transfers instead of bargaining). |
| election mechanics | restriction-first branch | at an election with transfer trigger armed, if `solidarity < 0.5` and `openness > 0.15`: fire restriction *instead of* transfer step — `openness −= 0.2`, `sentiment −= 0.05`, visible-rate mask −0.004 for 3 yrs | 1921/1930/1964/1973/2016–25: migration is always cut before transfers scale. Delays first transfer step 1–2 cycles; SF Fed +5.5pp vacancy per 1M/yr cut = mechanical displacement masking. |
| sentiment inflows | + wage-downgrade drain: `0.35 * care_absorption/pool * sentiment_gain`; + retrenchment spike: any forced transfer cut adds sentiment at 2.5× the step that created it | | Care churn absorbs bodies at −35% pay (calm statistics, eroding sentiment); Fetzer: cuts are 2–3× more explosive than absence — ratchet asymmetry on the transfer stock. |
| transfer trigger arming | armed if `visible_rate > transfer_trigger_rate` **OR** `blocked_share > 0.25` | | Blocked graduates are the constituency that makes UBI electorally viable with layoff rates still calm. |

One-off event: **2032–33 entitlement fight** (US OASI depletion, statutory 21–23% cut) — suppress transfer steps for one election cycle; sovereign-debt channel already exists (`gov_debt_gdp`), no new stock.

---

## 3. Parameter table

| Parameter | Value | Justifying brief |
|---|---|---|
| `r_ret_cog` | 0.025/yr → 0.021 by 2036, linear | BLS exits 4.7%/yr gross, retirement-only ~2.5–3% in cognitive-core geographies, boomer crest 2026–30 (briefs 1, 2) |
| `r_entry_cog` | 0.026/yr of pool | 40M grads/yr ≈ 4.2% of pool, ~62% historically absorbed into cognitive work → net +0.1%/yr (briefs 1, 3) |
| `r_ret_phys` / `r_entry_phys` | 0.018 / 0.026 per yr | global blend; net +19M/yr, SSA +23M/yr entrants dominated (briefs 1, 2) |
| `divert_max` (freeze cap) | 0.7 of entry flow | Dauth: 100% of robot incidence on entrants, zero incumbent displacement (briefs 2, 3, 8) |
| `entry_automation_lead` | 1 yr ahead of mid-career | entry tasks are the routine ones; Canaries: 22–25yo −13% while older flat (briefs 3, 8) |
| `youth_thresholds` | 0.18 apathy / 0.25 protest / 0.40 rupture on `blocked_share` | China 16–19% = suppressible lying-flat; Arab Spring 30–40% grad unemployment; Spain/Greece 40–50% → new-party rupture (brief 3) |
| `blocked_share` window | blocked / (3 × annual entry cohort) | 3-yr scarring window, NY Fed cohort framing (brief 3) |
| `youth_salience` | 2.5× per capita | educated unemployed 2–3× political potency (Tunisia 40% vs 24%) (brief 3) |
| `rupture_lag` | 3-yr third-order delay, discharges at election ticks | Podemos/Syriza: 3–4 yrs from threshold to electoral rupture (brief 3) |
| `underemploy_drain` | 0.6/yr of D1 → physical pool | 41.5% underemployed vs 5.7% unemployed ≈ 7:1 buffer (brief 3) |
| `blocked_init` | 8 M | entry postings −35% since 2023 with near-zero layoffs; grad-unemployment inversion already observed (brief 3) |
| `care_demand_growth` | 0.026/yr | 80+ cohort +3–4%/yr China, +3% OECD; 185→~240M WE by 2036 (brief 6) |
| `care_supply_cap` | 0.02/yr growth | US: 9.7M gross openings vs 0.86M net — churn-limited (brief 6) |
| `care_wage_penalty` | 0.35 | care median ≈ 60% of economy median (brief 6) |
| `care_robot_effectiveness` | 0.3 × nominal; task cap 0 pre-2031 → 0.15 by 2040 | Japan: 7.6% adoption after decades of 50% subsidies, >50% unused (brief 6) |
| `vacancy_gap(t)` | 8 → 14 M/yr, 2026→2036 | China −4.9→−10.1 M/yr, EU −2.0→−2.5, JP+KR −0.9→−1.4 (brief 1) |
| `openness_init` | 0.35 | mid-restrictionist wave: US net migration negative 2025, UK −50% in one yr, EU Pact live June 2026 (briefs 4, 9) |
| `openness_rates` | −0.5/yr max down, +0.05/yr max up | Germany 1973: closed in 5 weeks; Hart-Celler: 25-yr reopening — 10:1 asymmetry (brief 4) |
| `restriction_trigger` | visible rate > 0.025 or sentiment > 0.35, election-quantized | restriction is the cheap first resort — deliberately below the 0.04 threshold (brief 4) |
| `restriction_relief` | sentiment −0.05, visible-rate mask −0.004 for 3 yrs | one-cycle bump ≈ small transfer at 0% GDP; SF Fed vacancy evidence (brief 4) |
| `solidarity_init` | 0.75 | wave-1 displaced are in-group-coded (college, majority-group, 1.5–2.7× female) (briefs 5, 8) |
| `solidarity_targets` | 0.80 cognitive-dominant / 0.45 physical-dominant displaced mix; adjustment 0.05/yr | race/education skew reverses between waves (briefs 5, 8) |
| `chauvinism_k` | 0.5/yr × transfer_share × perceived_outgroup (0.35 = 2.5× foreign-born 0.14) | perception runs 2–3× actual, immune to correction; makes sustained 15% GDP unstable at US heterogeneity by construction (briefs 5, 9) |
| `cap_multiplier` | 0.70 + 0.30 × solidarity | −5pp GDP ceiling at US-vs-EU fractionalization gap (brief 5) |
| `age_creep` | 0.0025 GDP-share/yr; +1pp shock 2032–33 | US SS+health 11.9→15.5% GDP; OASI depletion FY2032 (brief 6) |
| `retrench_spike` | 2.5× | Fetzer: cuts +4–12pp anti-system vs weak expansion dampening (brief 9) |
| `concentration_salience` | 0.4 wave-1 (diffuse), 0.7 wave-2 (clustered) — fixed multipliers on the two visible-rate sentiment inflows | rust-belt vs Japan asymmetry; wave-1 victims 5% unionized (briefs 2, 8) |
| `care_pull_gain` | 0.5 gap→fleet-target coupling | China gap 10M now vs 0.5M professionals; pulled units displace ~0 (brief 6) |

---

## 4. The youth-blockage channel (separate sentiment inflow)

Two orthogonal political detectors, never averaged:

```
// Channel A — mid-career (existing, unchanged form, filtered input):
inflow_A = sentiment_gain * attributability
         * max(0, visible_rate - attrition_threshold(t)).powf(1.5)
         * concentration_salience                    // 0.4 cognitive / 0.7 physical era

// Channel B — youth blockage (NEW, stock-threshold, lagged):
blocked_share = blocked_entrants_m / (3.0 * entry_cohort_m);
inflow_B = youth_gain(=2.0) * youth_salience(2.5)
         * max(0, blocked_share - 0.18).powf(1.5);   // delivered via 3-yr delay3
// at blocked_share > 0.40: election-tick rupture event — inst_trust -0.05,
// election outcome forced onto transfer/restriction branch (Podemos template)
```

Channel A is a *rate* with threshold 0.040→0.036/yr on **visible** displacement (fast, election-cycle salience). Channel B is a *stock share* with thresholds 0.18/0.25/0.40 and a 3-yr lag (slow, higher amplitude — the historically revolutionary variable). Signature difference the model must preserve: 2026–29 shows A silent while B loads; B writes to `sentiment`, `inst_trust`, and transfer-trigger arming but **never** to `labor_power`.

---

## 5. Validation contract — falsifiable predictions

1. **Youth-first sequencing:** in baseline runs, `blocked_share` crosses 0.18 by 2027±1 while `visible_cog` stays under threshold until 2030–2033; the first sentiment escalation is Channel-B-driven. *Falsified if* observed politics 2026–29 are driven by layoff rates (JOLTS discharges rising >2pp) with grad-cohort indicators calm.
2. **Restriction-first:** first migration restriction event precedes the first transfer step by 1–2 election cycles in ≥80% of Monte-Carlo runs; deleting the migration valve pulls the first transfer step 2–4 yrs earlier. *Falsified if* any OECD country legislates UBI-scale transfers before major migration restriction.
3. **Silent physical decade:** `visible_phys` = 0 until robot HEW additions exceed ~10 M/yr (fleet ≳ 4 M units/yr, ≈2035–38 at trajectory). *Falsified by* organized anti-robot backlash (strike channel) anywhere in the aging core before ~2033.
4. **Care pull magnitude:** unmet_care_gap adds 0.5–2 M units/yr to robot demand by mid-2030s; ablating the coupling cuts 2032–36 fleet growth 15–30%. *Falsified if* early humanoid deployments concentrate outside care/vacancy-gap markets.
5. **Chronic-cap instability:** any run sustaining transfers >11% GDP with solidarity <0.5 retrenches within 2 election cycles, and the retrenchment sentiment spike ≥2× the original relief. *Falsified by* a high-heterogeneity polity sustaining >11% GDP universal transfers without chauvinist erosion.
6. **Later-but-sharper:** vs the constant-threshold ablation, political trigger arrives 2–4 yrs later but peak sentiment slope is ≥1.5× (the Meadows delayed-feedback trap: masking suppresses the signal, then the threshold is crossed with a jump).
7. **Pool arithmetic:** physical pool 2,580–2,620 M by 2036; denominator growth alone shaves ~0.1pp/yr off measured physical displacement rate vs the static 2400 M spec.
8. **Entitlement collision:** runs with the 2032–33 event reach `transfer_cap` ~2 yrs later than runs without it; the event never *prevents* cap-hitting when displacement persists (contractual crowding delays, doesn't substitute).

---

## 6. What NOT to model, and why

- **Fertility beyond a fiscal constant.** Births cannot touch either pool before 2045 (20-yr lag); TFR endogeneity to transfers saturates at +0.15–0.25 TFR — politics-side noise. Keep only the `age_creep` ramp (which already embodies the dependency trajectory). No TFR stock, no pronatal-spending competition (observed 0.07–0.13% GDP: below model resolution).
- **Sub-national/spatial detail.** Concentration effects enter as the two fixed salience multipliers (0.4/0.7) — the published evidence (rust belt vs Japan; Sweden depopulation) supports an elasticity, not a geography, at this aggregation.
- **Group stocks (race/gender/nativist parties).** Composition dynamics enter *only* as parameters: solidarity targets by displaced-wave mix, the 2.5× perception multiplier, chauvinism erosion, the restriction-first branch. Published elasticities (Alesina-Glaeser slope, Halla-Wagner-Zweimüller 0.16–0.4pp/pp, Fetzer 2–3× cut asymmetry) — no group narratives, no separate nativist-vote stock (its ceiling/plateau behavior is fully absorbed by the openness ratchet + branch probability, and a vote-share stock would double-count sentiment).
- **Geographic pool buckets.** The core/periphery split folds into the drift rates and `vacancy_gap(t)`; society politics are already OECD-weighted. Two extra stocks would buy resolution the couplings never consume.
- **Care-robot capability micro-modeling pre-2032.** Japan's 20-year natural experiment fixes substitution ≈ 0 regardless of capability assumptions; the binding constraints (liability, training, cost-effectiveness) are not in the model's state space.
- **SSA premature-deindustrialization / migration-pressure dynamics.** Real and large (+23 M workers/yr meeting an AI-closed export ladder) but outside the OECD-election political core this model resolves; the periphery enters only through `r_entry_phys` and the openness stock. Flag as a scope limitation in `REPORT.md`, not a mechanism.

**Double-counting guards (summary):** retirement absorption only in `attrition_threshold(t)`; hiring-freeze only in the visible/blocked split; care absorption subtracts from visible physical rate but pays a 0.35-weight sentiment toll; migration is politics/market-tightness only (pools are global aggregates — migration is reallocation, not global supply); solidarity gates cap/step and leaves `transfer_backlash_damp` untouched; care-pull robot units carry zero displacement weight.

**Implementation:** new module `rust/singularity-econ/src/demography.rs` — `DemographyParams`, `DemographyState`, stepped once per year *before* `SocietyState::step`, consuming raw `D_cog`/robot HEW additions and emitting `DemographyOutputs { visible_cog_rate, visible_phys_rate, attrition_threshold_t, transfer_cap_eff, transfer_step_eff, physical_addressable_eff, robot_pull_units, youth_sentiment_inflow, cog_pool_m, phys_pool_m, restriction_fired }`; `lib.rs` swaps `p.cognitive_workers_m`/`p.physical_workers_m` reads (lines 186–187, 790, 812) for the state values and `p.physical_addressable` (line 247) for the effective value; `society.rs` takes the three endogenous overrides as per-step arguments rather than params, plus `inflow_B` as a second sentiment inflow and the trigger-arming OR-condition. Ablation switch `d_demography: f64` mirrors the existing loop switches; with it off, all outputs collapse to current constants (0.04/0.04/0.15/0.65, raw rates) — bit-identical legacy behavior for regression tests.