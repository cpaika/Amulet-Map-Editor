# Society Layer Design — AI Transition Stock-Flow Model (2026–2036)

**Design principle (Meadows):** politics is a RATE detector with saturating level terms; every brief independently converges on this. All new inflows key off `d(displaced_share)/dt` and incident pulses, never off `displaced_share` directly. Second principle: **asymmetric reversibility** — every stock gets separate up/down time constants (stringency ratchets up in steps, decays ~0; sentiment spikes in weeks, decays in years; transfers ratchet at elections, decay only if emergency-framed).

**Double-counting guards:** the existing **B2 backlash gain** becomes the *output actuator* of this layer (endogenized, §5) — do not add a second sentiment→adoption path. The existing **B4 credit loop** is the *only* financial-amplification channel — incidents/unrest/regulatory cost inject *into* B4 as spread shocks rather than getting their own capital-flight loop. The existing **transition drag** absorbs reallocation friction — new GDP effects are limited to (a) transfer demand support, (b) unrest cost, (c) compliance cost, each a distinct mechanism.

---

## 1. New Stocks (6)

| # | Stock | Units, init (2026) | Inflow | Outflow |
|---|-------|--------------------|--------|---------|
| S1 | `public_sentiment` (grievance/salience) | 0–1, init **0.20** (salience, not the saturated concern level of 0.50 — briefs 6 & 9 insist these differ; we model salience) | `k_s·max(0, disp_rate − 0.04)^1.5 · attributability(2.5) · concentration(1..3)` + `0.5·announced_displacement_rate` (anticipation, 1-yr lead) + incident pulses | decay: half-life **7 yr** unremediated → **2.5 yr** when `transfer_share > 0.03` |
| S2 | `reg_stringency` | 0–1, init **US 0.04 / EU 0.17** (two-region if model supports; else GDP-weighted 0.08) | discrete legislation steps (§3 events + diffuse drip `+0.02/yr·1{S1>0.3 sustained 2yr}`) gated by capture (B8) | **≈0** (reversal lag 40–93 yr; hard ratchet in horizon) |
| S2e | `reg_enforcement` (auxiliary smooth) | 0–1, init 0.02 | first-order adjust toward S2, **τ = 3.5 yr** | — |
| S3 | `transfer_share` (of GDP) | 0–0.15, init **0.005** | staircase steps at trigger/election (B5); split emergency (decays τ 0.5–1.5 yr) vs permanent (ratchet_fraction of peak) | permanent part: ~0; emergency part: τ ≈ 1 yr |
| S4 | `labor_power` | 0–1, init **0.12** economy-wide (per-sector: 0.8–1.0 chokepoint, 0.5–0.7 licensed, 0.05–0.15 diffuse cognitive) | organizing `+0.02/yr·S1` | erosion ∝ cumulative sector displacement (~1:1, 5-yr delay) |
| S5 | `inst_trust` | 0–1, init **0.27** | +0.01/yr while transfers rising & delivered | −0.005/yr baseline; −0.02/yr while `disp_rate>0.02`; −0.03 per unrest episode |
| S6 | `consumer_trust` | 0–1, init **0.50** | familiarity R8: `+0.05/yr·adoption` | incident steps −0.20 first, ×0.5 per repeat; recovery half-life 1.5 yr (direct-payoff apps) / 7 yr (diffuse); **latch ×0.1 recovery if `inst_trust<0.30` at incident time** |

*Deliberately NOT stocks:* legislative pipeline (full as of 2026 in every jurisdiction — treat as constant `pipeline=1`, which sets incident→statute lag at the SHORT end, 0.5–1 yr); wealth concentration (drives S1 via a visibility term but its own dynamics live in the capital side of the existing model); unrest momentum (implemented as a 2-yr hazard multiplier after episodes, §3).

---

## 2. Named Loops

Existing variables referenced: `adoption_rate`, `disp_rate = d(displaced_share)/dt`, `displaced_share`, `gdp`, `credit_spread` (B4), `debt_gdp`, `backlash_gain` (B2), `afford_gain`.

| Loop | Sign | Path (reads → writes) | Gain | Delay |
|------|------|----------------------|------|-------|
| **B5 ReliefValve** | − on S1; **disables B2** | `S1`, `disp_rate` → `transfer_share` step; writes `gdp` (+demand, multiplier 1.5), `debt_gdp` (100% debt-financed first 5 yr → feeds B4), and `backlash_gain ×0.3` while active | step +0.03–0.10 GDP per trigger; drains S1 (half-life 7→2.5 yr) | legislative lag **endogenous**: 0.1 yr if `disp_rate` spikes >2pp/yr; else quantized to next election (≤2 yr). Delivery 0.05 yr (rails exist post-2020) |
| **B6 DiffuseRegulation** (slow arm) | − on adoption | sustained `S1` → S2 drip → S2e → `adoption_rate ×(1 − 0.25·S2e)` + compliance cost into transition drag | mild: −25% adoption at S2e=1 (GDPR class) | statute 3–6 yr from S1>0.3; +3.5 yr enforcement (S2e) |
| **B7 IncidentClampdown** (fast arm) | − on adoption, threshold | incident pulse (§3) → S2 step +0.4–0.7 (dread class) or +0.10–0.25 (ordinary-major) → adoption multiplier; embodied-AI sector: local −100% for 0.75–1 yr regardless of S2 | can zero sector adoption (nuclear precedent) | **0.5–1 yr** (pipeline full); suspension 0.02–0.06 yr for embodied |
| **B8 Capture** | − on S2 inflow (protects adoption) | `adoption_rate` → AI profits → lobby capital → legislation-probability ×(1−0.55), federal channel ×0.35, + concern→policy conversion ×0.7 | 50–70% suppression **below** S1 threshold 0.55; **→0 above it** (99-1 precedent) | 0.5 yr; preemption litigation 1–3 yr |
| **B9 BuyoffValve** | − on S1; **~0 on adoption** | `S1 × labor_power` → buyoff deal (cost 5–15% of sector automation surplus, paid from AI profits) → drains 60–90% of sector S1, creates protected cohort suppressing S1 inflow 15 yr; commitments **default if B4 credit crunch → deferred S1 release** | dominant historical路径 in high-leverage sectors; adoption proceeds unthrottled | 1–2 yr to deal |
| **B10 ConsumerTrustThrottle** | − on consumer adoption | `S6` → consumer-adoption ceiling `min(0.9, 0.4+S6)`; bystander veto: if non-user concern >0.70 → public-facing/embodied deployment ×0.1 for 2 yr, 10-yr recovery | segment-scoped | <0.25 yr shock, slow recovery |
| **R5 TrustErosion/Radicalization** | + (destabilizing) | unremediated `S1 × (1−inst_trust)` → S5↓ → policy incoherence (B5/B6 delays ×1.5) → S1 persists → S5↓. **Regime switch at S5<0.20:** 50%+ of backlash reroutes anti-system: `backlash_gain ×0.5` but policy whipsaw (stringency variance ×3), backsliding hazard ×3 | 0.02–0.05/yr compounding | 2–5 yr/turn; engages only after >3 yr unremediated displacement >2pp/yr |
| **R6 LeverageErosion** | + (pro-adoption) | `adoption` → sector displacement → `labor_power`↓ → B9/B2 throttle↓ → faster adoption | ~1:1 with cumulative sector displacement | 5-yr delay. Creates a **backlash window 2026–29** that then closes |
| **R7 RegulatoryCostSpiral** | + (industry-kill), **conditional** | active only if `S2e>0.5 AND dread incident occurred`: compliance/delay → cost ×1.3–2/cycle → inject `credit_spread +150bp` into **B4** → overruns → S1 → S2 | adoption →0.0–0.3 in affected sector within 5 yr | 1–2 yr/cycle |
| **R8 Familiarity** | + on S6 | adoption → hands-on use → S6↑ → adoption ceiling↑; races B7/B10 | +0.05/yr·adoption | 0.5–1 yr |

---

## 3. Shock Events (stochastic, Monte Carlo)

| Event | Base-rate prior | Effects |
|-------|----------------|---------|
| **AI incident, ordinary-major** (Tier-1/2: vivid harm, scandal, embodied injury) | hazard = `0.10 + 0.4·embodied_deployment_share` /yr; concealment multiplier ×1.5–2 w.p. 0.3 | S1 +0.10–0.15 (×0.5^n habituation); S6 −0.20 first/×0.5 repeats; embodied sector local shutdown 0.75–1 yr; `credit_spread +100bp` (B4) for 2 yr; laggard-firm exit w.p. 0.5 |
| **AI dread incident** (zero-warning, involuntary, concentrated — TMI-class) | p = 0.25 conditional on any major incident (≈3–5%/yr rising with deployment) | S2 step +0.4–0.7 within 0.5–1 yr (pipeline full); arms R7; S6 latch check vs `inst_trust` |
| **Landmark ruling** (US preemption resolution) | resolves 2027–2029, p 0.4/yr; P(preemption cap breaks) = 0.5 baseline, 0.85 if any dread incident or post-flip | cap-holds: US S2 ceiling 0.15 until next flip; cap-breaks: state funnel adds +0.03/yr to S2 |
| **Election flip / realignment** | quantized 2026/28/30/…; P(flip) = 0.7/cycle while `disp_rate>0.02` sustained ≥2 yr (vote loss 0.7pp per pp, ×1.75 downside asymmetry, memory 1 yr); **full realignment needs 2 consecutive cycles** | after 2nd cycle: B5 step +3–5% GDP within 1 yr; state_posture ±0.5 → `backlash_gain ×(1±1)`, ban half-life 6→30 yr if pro-labor |
| **General strike / major unrest** | base 3%/yr; ×2 if `d(unemployment)>4pp/2yr`; ×3–5 for 2 yr after any episode (autocorrelation); ×`(2−inst_trust/0.5)` | adoption −15–30% for 2–3 yr; `gdp −1pp` at 18 mo; `credit_spread +100–300bp`; S5 −0.03; forces B5 or B9 trigger check |
| **Emergency-powers throttle** (only if crisis-scale: disp_rate>4pp/yr or unrest wave) | p 0.9 within 1 yr of qualifying crisis | S2 +0.3 in **weeks** (bypasses legislative lag) but deposits into a ratchet: 30% persists permanently; backsliding hazard ×2 |

---

## 4. Parameter Table

| Parameter | Value [range] | Justifying brief |
|-----------|--------------|-----------------|
| `attrition_threshold` | 0.04/yr [0.03–0.05] | Labor resistance (silent absorption below attrition; ILWU) |
| `attributability_mult` | 2.5 [1–3] | Democratic shocks (Funke: financial-vs-recession null; AI = named firms) |
| `sentiment_halflife` | 7 yr → 2.5 yr w/ transfers | Realignment (china-shock incubation; New Deal drain) |
| `anticipation_weight` | 0.5, 1-yr lead | Labor resistance (WGA struck at ~0% realized displacement) |
| `incident_step_S2_dread` | +0.55 [0.4–0.7] | Regulation speed (TMI/nuclear) |
| `incident_to_statute_lag` | 0.75 yr [0.5–1] (pipeline full) | Regulation speed (1938/1962/1966: 3–10 mo w/ stocked bill; 2026 pipeline: 1,100+ bills/yr) |
| `diffuse_reg_lag` | 12 yr from 2024 onset [10–20] | Regulation speed (railroads 17, privacy 20+) |
| `enforcement_tau` | 3.5 yr | Regulation speed (GDPR 2018→2021 fines; ICC+19) |
| `adoption_mult_diffuse_reg` | 1 − 0.25·S2e | Regulation speed (GDPR: −26% VC, ⅓ product exit) |
| `capture_suppression` | 0.55 below S1=0.55; 0 above | Regulation speed (SB1047 veto vs 99-1 vote) |
| `buyoff_cost_share` | 0.10 of surplus [0.05–0.15] | Labor resistance (M&M ≈5%, ILA ≈10–15%) |
| `ban_halflife` | 1.5 / 6 / 15 / 30 yr (exit / contract / grandfather / statute) | Labor resistance (Wapping / ILA / ITU / rail firemen) |
| `leverage_by_sector` | 0.9 chokepoint, 0.6 licensed, 0.5 guild, 0.10 diffuse cognitive | Labor resistance |
| `vote_elasticity` | −0.7pp/pp unemployment rate, ×1.75 asymmetry, 1-yr memory | Realignment (1932; postwar economic voting) |
| `sociotropic_mult` | 3 [2–4] — cut from 7 for geographic diffusion | Realignment (china shock 5–10, AI diffuse) |
| `villain_salience` | 0.7 [0.6–0.8] (fraction of backlash routed anti-corporate) | Realignment (named firms ≈ Standard Oil, not "trade") |
| `crisis_mode_threshold` | disp_rate > 2pp/yr | Fiscal (COVID 4 wk vs Depression 3.5 yr; interpolate) |
| `max_single_bill` / `max_2yr_cum` | 10% / 25% GDP | Fiscal (CARES/ARP) |
| `ratchet_fraction` | 0.3 [0.2–0.4]; **1.0 if dividend-framed** (policy switch) | Fiscal (CTC/UI lapse vs Alaska PFD 44 yr) |
| `inflation_passthrough` | 0.12pp CPI /1%GDP ×0.3 supply-elasticity factor (AI expands supply); 1.0 for housing/services | Fiscal (COVID 2.5–3pp; Kenya slack bound) |
| `labor_supply_elasticity` | **0** below 30% income replacement | Fiscal (Alaska/Finland/Kenya ≈0 — do not add this loop) |
| `winner_tax_lag` | 7 yr [5–10] post displacement>15%; drag elasticity −0.2/pp | Fiscal (0 enactments 2017–26; Soc Sec 1935→37) |
| `trust_shock_first / habituation` | −0.20 / ×0.5 per repeat | Consumer trust (TMI −23, Chernobyl −12, Fukushima −5) |
| `trust_recovery_halflife` | 1.5 yr payoff / 7 yr diffuse / ×0.1 latched | Consumer trust (crypto/FTX vs nuclear vs EU-GMO) |
| `bystander_veto_threshold` | 0.70 non-user concern | Consumer trust (Google Glass 72%) |
| `narrative_activation` | AI-attributed layoffs ≥0.03%/yr workforce | Incidents 2016–26 (55k/165M made AI #1 layoff story) |
| `backlash_gain_law` | ∝ log10(disp_rate/0.0003): 0 at threshold, ≈1 @0.3%/yr, ≈2 @3%/yr | Incidents 2016–26 |
| `opinion_speed_mult` | 5 [4–8] — divide legacy perception delays | Info environment (1–1.5→6–11 pp/yr) |
| `polarization_dampener` | 0.5 on federal channel [0.4–0.6] | Info environment (166% throughput gap) |
| `counter_persuasion` | ×0.7 on concern→policy 2026, →×0.55 by 2030 | Info environment (+81% LLM persuasion RCT, $125M PAC) |
| `unrest_base_hazard` | 3%/yr, multipliers §3 | Inequality (Ponticelli-Voth; IMF autocorrelation ×10) |
| `unrest_adoption_cut` / `gdp_cost` | −15–30% 2–3 yr / −1pp @18 mo | Inequality (IMF) |
| `breakdown_hazard` (consolidated) | 0.2%/yr; ×2 if disp>3pp/yr 3 yr; ×5 if emergency-ratchet>2 AND S5<0.5 | Democratic shocks (0 Depression breakdowns in consolidated democracies) |
| `backsliding_hazard` | 1.2%/yr ×(1+2·crisis) ×f(S5) | Democratic shocks (5–6/38 OECD per 15 yr) |
| `precautionary_savings` | +4pp saving rate [3–5; 8 tail] when sentiment>0.5, τ 2 yr | Consumer spending (GFC; Benito −2%C per SD job risk) |

---

## 5. Endogenized Existing Parameters

1. **`backlash_gain` (B2)** — becomes fully computed:
 `backlash_gain = g0 · log_law(disp_rate) · S1 · max(labor_power, 0.1) · trust_schedule(S5) · (1 − 0.7·transfer_active) · counter_persuasion · state_posture_mult`. B2 stops being a constant; it is now a *pulse* peaking 2028–30 (rate peak × pre-erosion labor power) and fading.
2. **`afford_gain`** — `afford_gain = base + 1.5·transfer_share − 0.6·precautionary_savings(S1)`. Transfers are the demand backstop; sentiment itself now cuts consumption before displacement does.
3. **B4 credit spread** — add exogenous-shock injection port: `spread += incident_shock + unrest_shock + R7_spiral_term + debt_penalty(debt_gdp>1.2)`. Also: B9 buyoff commitments default when B4 tightens (deferred grievance release).
4. **Transition drag** — add `compliance_cost = 0.02·S2e·sector_revenue` and unrest GDP cost; do NOT route regulation through drag twice (B6's adoption multiplier is the other channel — one hits level of output, the other hits adoption rate; keep them distinct).
5. **Adoption ceiling** — cap at 0.85–0.90, gated by `consumer_trust` for consumer-facing share (was presumably 1.0).

---

## 6. Validation Contract (testable predictions)

1. **Regulation lags displacement:** effective (enforced) stringency `S2e` lags the displacement-rate peak by **2–4 yr** in no-incident runs (statute 1–2 yr post-realignment + 3.5 yr enforcement, minus anticipation); with a dread incident, statute lag collapses to **0.5–1.5 yr** but enforcement still trails 3+ yr.
2. **Transfers scale within 1 yr** of any displacement-rate spike >2pp/yr (crisis mode); below that threshold, transfer steps occur **only at election dates** (staircase, never smooth).
3. **Backlash is a pulse, not a plateau:** `public_sentiment` peaks within ~1 yr of peak `disp_rate` (expected 2028–2031) and declines ≥30% by 2035 even as `displaced_share` keeps rising — driven by rate decay + R6 leverage erosion + habituation.
4. **Baseline infeasibility without relief:** no run reaches ≥40% cognitive-task displacement by 2032 with `transfer_share < 0.03`; runs that get there have transfers ≥3% GDP by 2030 and B2 throttle attenuated ×0.3 (relief valve is the only historically-grounded path to fast adoption).
5. **Opinion/policy decoupling:** after any dread incident, `consumer_trust` recovers to within 0.05 of pre-shock in 2–5 yr (unless latched), while `reg_stringency` remains ≥90% of its post-incident peak through 2036 (ratchet). Latched runs (incident while `inst_trust<0.30`) show zero trust recovery for the full horizon.
6. **Two-election rule:** binding national throttling/relief policy never appears earlier than the **second** election cycle after the displacement rate first breaches 2pp/yr (3–6 yr total, matching 1930→1933, 2008→2010).
7. **Freeze state is a conjunction, not a default:** the nuclear-style absorbing state (sector adoption <0.1× potential for ≥5 yr) occurs in **5–12% of Monte Carlo runs** and *only* in runs containing all three of: dread incident, `S2e>0.5`, and an active B4 tightening — never from diffuse regulation alone.
8. **Capture is bimodal:** US stringency trajectories are bimodal — near-flat (<0.15) until an incident or realignment, then step overshoot — while EU stringency rises smoothly from 0.17 with a 1–2 yr slip; a sensitivity sweep on `capture_suppression` between 0.4–0.7 should not change which mode a run lands in (mode selection belongs to the incident lottery and elections, not the capture constant).

**Implementation order:** S1 + B5 + endogenized `backlash_gain` first (they dominate every trajectory), then S2/B6/B7/B8, then shocks, then S4/B9/R6, then S5/S6/R5/R7/B10. Backtest targets per brief 7: Depression-pulse+high-trust → transfers, no breakdown; same pulse+low-trust+blocked B5 → R5 takeover in ≤4 yr; 2008-scale pulse → +30% anti-system vote, no levies.