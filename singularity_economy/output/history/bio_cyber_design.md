The module compiles and the deterministic baseline is intact. Here is the synthesis.

---

# Bio + Cyber layers — implementation-ready design

Two modules, both inert by default (`bio_layer = 0.0`, empty shock vecs) so the deterministic baseline is byte-identical — same regression discipline as `geopolitics.rs`. Written to `rust/singularity-econ/src/bio.rs` (compiles clean against the crate; `pub mod bio;` added to `lib.rs`). Module A is the bio economy + R1-physical extension + optionality; Module B is the two dread shock classes (`bio::shocks`).

## (1) Minimal stocks / pools

**Module A — `BioState` (bio.rs):**
| Stock | Type | Role |
|---|---|---|
| `physical_discovery_cap` | 0..1 | R1→physical-science (SDL) extension; spillover ≪1, cycle-time-floored, validation-haircut |
| `drug_pipeline` | count | candidates-in-clinic; inflow = adoption-gated preclinical throughput, drains ~90% to attrition |
| `hc_rd_cost_index` | LEVEL 1.0→ | drug-dev cost/candidate falling with AI adoption; feeds healthcare-deflation |
| `bio_knowledge_uplift` | 0..1 | VCT-style knowledge from R1 (already elevated at 2026) |
| `synthesis_access_gate` | 0.20→0.35 | uncovered synthesis fraction; slow erosion, re-closable by mandate |
| `cumulative_bio_incidents` | counter | habituation / nth-event ratchet |
| `longevity_trl`, `bci_maturity` | 0..1 | SHADOW stocks, ~0 in-horizon |
| `elite_longevity_frame`, `merge_frame` | 0..1 | memetic, feed society sentiment (backlash-heavy) |
| pools `ai_drug_pool_b`, `longevity_pool_b`, `bci_pool_b` | $B | valuation cross-section marks |

**Module B — `bio::shocks`:** `DreadShockKind{BioPandemic, CyberSystemic}`, drawn into `Params` mirroring `geo_shocks`; no new persistent stocks — the cyber offense/defender race indices are computed from the existing R1 track.

## (2) Hazard functions vs capability + magnitude distributions

**Biorisk** (rides ON TOP of the always-on natural pandemic; do not double-count):
```
h_bio = h0·exp(k·(cap_op − cap*))·(1 − m_safeguard) + h_natural
  h0=0.003/yr (FRI 100k-death), h_natural=0.028/yr, cap*=0.03 (VCT crossing ~2025),
  k=5 (pins the FRI 5× multiplier at crossing), m_safeguard=0.75·(1−open_weight_share)
  cap_op = bio_knowledge_uplift · translation_coeff(0.15→0.35) · synthesis_access_gate
```
`translation_coeff` is the load-bearing Aum-gate parameter that reconciles RAND "no uplift" (5% actual) with VCT "threshold crossed" (2× expert) — expose it for sensitivity. Severity = lognormal: median event macro ≈0 (Amerithrax), 90-95th pct −3 to −4% (COVID), tail −8 to −12%. Mass-casualty (>3% drag) → +0.70 stringency, arms R7 + screening mandate; contained → +0.20.

**Cyber** (systemic/correlated only; chronic ransomware is a separate steady RATE tax, not this shock):
```
h_cyber = h_c0·sigmoid(6·(offense − defender))·autonomy_gate(year≥2025)
```
Sign is **emergent**: net-offense transition window ~2025–late-2020s (patch cycle 30-60d ≫ exploit hours), flips defense-favored as `defender` compounds (14% CAGR). Severity modal ~$5.4B ≈ 0.02% world GDP transient; DUAL-sign — pumps the cyber profit pool while denting broad beta. Systemic step +0.30 (weaker than bio — cyber is familiar, lower dread).

## (3) Couplings to EXISTING structure (double-count guards)

| Writes to | Mechanism | Guard vs existing AI-incident |
|---|---|---|
| `reg_stringency` (society.rs) | additive ratcheted step, mirrors `incident_s2_dread=0.55` | **`effects_for_year` takes MAX across co-occurring dread classes, not sum** — three dread events one year ≠ 3× |
| `dread_armed` (lib.rs:1041) | single idempotent latch → R7 credit spiral | spiral is a FIXED 0.15 injection gated on latch+enforcement; already non-additive — AI+bio+cyber cannot super-add |
| `credit_injection` / spread | systemic bio/cyber inject into same `gfx.spread` channel | gated to mass-casualty/systemic only; distinct physical cause from geo |
| GDP | transient mean-reverting LEVEL drop (1-3yr recovery), separate from geo chip/power/demand channels and from the chronic ransomware RATE tax | no overlap with geo `demand_mult` |
| `age_creep` (demography.rs:302) | `age_creep_eff = age_creep · (1 − κ·healthcare_deflation)`, κ=0.15, lag 4yr; deflation = rd_drop·pharma_share(0.12)·passthrough(0.10) | second-order by construction — raises `transfer_cap_eff` marginally |
| R1 | READS `algo_eff` → knowledge_uplift + physical_discovery (×spillover 0.15) | frontier capability, not adoption — cyber arms BOTH sides from same stock |
| valuation pools | new `ai_drug`/BioDefense/cyber/longevity/BCI marks; differential = redistribution | does NOT re-rate the index; broad-beta hit transient |
| society sentiment | frame_sentiment_delta (net negative) + low-rate bio-scandal dread draw | distinct from generic AI incident (different regulatory target: GoF/synthesis/model-release) |

The generic `incident_year` stays as the AI product-safety/misalignment class; bio and cyber are **distinct drivers and regulatory targets** that co-occur with it.

## (4) Parameter table (from briefs, in `BioParams::default()` + `shocks`)

`r1_phys_spillover=0.15` · `sdl_accel_ceiling=4.0` (fast≈6-10×, slow≈1-2×) · `validation_haircut=0.40` (A-Lab) · `drug_pool 2026=$1.5B→2035=$14B` (narrow tooling TAM) · `preclinical_cost_mult=0.65` · `pharma_adoption 0.55→0.85` · `phase1_gate=1.6, phase2/3=1.0, clinical_loa=0.10` · `royalty_first_year=2029` · `pharma_share_of_health=0.12` · `price_passthrough=0.10` · `age_creep_kappa=0.15, lag=4` · `translation_coeff 0.15→0.35` · `synth_gate 0.20→0.35, reclose=0.06` · `longevity_pool_2035=$64B` · `bci_pool_2035=$8B, pma_year=2029, roadmap_haircut=0.30`. Shocks: `h0=0.003, h_natural=0.028, cap*=0.03, k=5, m_safeguard=0.75·closed_share, h_c0=0.05`.

## (5) Validation contract (falsifiable)

1. **Regression:** empty shock vecs + `bio_layer=0` → baseline byte-identical (holds; tests pass).
2. **Bio hazard already partially on:** E[h_bio(2026)] ∈ [1.2×,2×]·h0, rising to ~5× by ~2030 pre-mitigation.
3. **Translation coeff dominates:** sweeping 0.1→0.3 moves E[bio tail-loss] >2×, exceeding k-sensitivity.
4. **Fat tail:** median drawn bio event |ΔGDP|<0.3%; 90th pct ≤−3%; ≥1-in-~200 paths ≤−8%.
5. **Mean reversion:** bio/cyber GDP recovers within 1% of counterfactual in ≤3yr UNLESS the reg ratchet + R7 spiral persist.
6. **Cyber sign flip:** net-offense ≤2029 (hazard rising), defender ≥ offense in ≥50% of paths by ~2033 (hazard declining).
7. **Ratchet idempotency:** co-occurring AI+bio+cyber dread in one year → same R7 spiral magnitude as a single event; combined stringency step ≤ max class step.
8. **age_creep second-order:** drug layer ON vs OFF → `transfer_cap_eff(2036)` differs <0.5pp; longevity/BCI contribute exactly 0 to age_creep in-horizon.
9. **Cyber countercyclical:** systemic-cyber year → cyber pool earnings up while gdp_index down; dispersion widens, index level unchanged.

## (6) What NOT to model (strict)

- **Longevity/BCI dependency-ratio or retirement relief in-horizon = ZERO.** Dependency ratios are demographically LOCKED; LEV relief is 2040s+. Letting longevity relieve `age_creep` inside the run is the single biggest error to avoid — `bci_maturity`/`longevity_trl` are shadow stocks masked to 0 impact.
- **No lifespan/mortality change** (LE +<2yr, decelerating — Olshansky). Only healthspan→participation, itself <15% likely; not wired to labor.
- **BCI out of labor + bottleneck subsystems entirely** (cumulative implants 10³–10⁵ ≪ 0.1% of labor). Merge-frame capped by the ~10 bits/s cognition ceiling.
- **No clinical-probability uplift from AI drug discovery** — Phase 2/3 stay ~37-40%, overall LOA ~10%. Only Phase-1 gate ×1.6 + preclinical cost/time. No Eroom reversal. Pool realized ≈0 before 2029.
- **No physical-cycle-time compression** — AI raises throughput + design quality, never the physical clock; spillover ≪1, saturating at 10-20 exp/dim; A-Lab-style counts get the validation haircut.
- **Don't double the natural-pandemic base rate** — the AI channel rides on top.
- Optionality pools never move the index (~$64B/$8B, 2-3 orders below AI-services/robots).

## Trade-book honesty — in-horizon vs out-of-horizon

**IN-HORIZON (moves the book 2026-2036):**
- **Cyber shock + cyber profit pool** — the most trade-relevant addition. Active now (first AI-orchestrated campaign Nov 2025), countercyclical DUAL-sign hedge. Names: CRWD, PANW, ZS, FTNT, MSFT, S; negative-sign on cyber-insurers/reinsurers on systemic events.
- **Biorisk pandemic shock** — partially ON now (VCT crossed); moves left-tail/expected-loss and the differential (BioDefense +100-400% Moderna template vs broad beta −20-35%). A tail hedge + dispersion widener, not a base-case earnings driver.
- **Bio/cyber dread → reg_stringency → R7 credit spiral** — systemic-risk channel that can drag valuation broadly.
- **`ai_drug_discovery` pool** — marginally in: Schrödinger (SDGR) SaaS is real recurring revenue now (cleanest, lowest-beta capture, SaaS+royalty hybrid); royalty option back-loaded ≥2029; RXRX/XtalPi = clinical-stage volatility; **Isomorphic not investable (route to Alphabet)**.
- Wet-lab-automation R1-extension → bio/materials + robots pool: modest, capex/chips-gated (competes for the same bottlenecks, not free upside).

**OUT-OF-HORIZON OPTIONALITY (does not move the base case):**
- **Longevity pool / LEV** — 2040s+ payoff; only in-horizon channel is memetic backlash (dread), not upside.
- **BCI optionality** — first US PMA 2028-2030 is the only in-horizon signal; **all pure-plays are PRIVATE** (Neuralink/Synchron/Precision/Paradromics) — no clean public long exists at purity; flag illiquid secondaries / diluted proxies explicitly.
- **Healthcare-deflation → age_creep relief** — real but second-order and lagged; does not move the fiscal trajectory enough to trade.

Files: `/home/user/Amulet-Map-Editor/singularity_economy/rust/singularity-econ/src/bio.rs` (module A + `bio::shocks` module B); `lib.rs` line 15 (`pub mod bio;`). Remaining wiring to activate (currently inert): call `bio.step(algo_eff, adopt, capex_gate, open_weight_share)` in the year loop after R1; multiply `p.demography.age_creep` by `age_creep_mult`; add pool fields to `Pools`/`EmergingPool` for the three marks; draw `bio::shocks::sample_dread_shocks` in main.rs MC and apply `effects_for_year` alongside `gfx` (GDP drag, spread, `dread_armed |=`, `arm_screening`, stringency MAX-merge into the society incident path).