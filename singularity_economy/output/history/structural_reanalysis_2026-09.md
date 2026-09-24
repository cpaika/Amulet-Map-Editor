# Re-analysis: singularity-econ, model versus world (lead modeler synthesis, 2026-09-24)

## 1. Verdict

The model's physical bookkeeping holds up: stocks, pipelines, binding logic, and the code bugs the August audits already fixed. What it is not yet is a pricing engine. Its main conclusions come from a few hand-set constants, not from the world it simulates.

- **"Power binds 2028-34"** is the long-run balance of four inputs:
  - a fixed 32%/yr growth in desired AI capex (`demand_growth_base`) that never decays and never has to earn a return;
  - rent ceilings the baseline sits on for 6 to 11 of 11 years (`margin_ceiling` 0.62, a literal 0.6 cap on electricity margin, adoption capped at 0.90);
  - a hand-set trend in grid power per capex dollar whose direction is the opposite of what the industry shows;
  - a valuation formula that raises a 19x ratio on a still-growing annual-spend pool to the power beta, then values the business at the pool's peak growth rate.
- **The way uncertainty becomes the book makes this worse.** E[upside] is a probability-weighted mean over six hand-picked paths, inside one hand-picked structural lens (base, enhanced or v2).
  - The model's own Monte Carlo (MC) run disagrees with the named book, sometimes in sign, across the merchant-power names (VST +70 → +12, CEG +10 → −16).
  - The lens choice is not sampled anywhere, yet it is the largest single driver of upside (Spearman −0.56 to −0.70).
- **What to withdraw.** Two claims should be dropped as conclusions: "power binds 2028-34" and "POWL is the best name in the book with a positive worst case." They should be restated as conditional on demand-growth, margin-ceiling and GW-per-dollar assumptions.
- **What survives.** The wage-pool displacement shorts keep their sign but not their size or timing. Freight and insurance shorts (CHRW, LSTR, MMC, 6954.T, FDS) are valuation-convention shorts, not AI shorts.

## 2. Confirmed and partial findings, ranked by impact on headline conclusions

Unless stated otherwise, numbers are named-book E[up] in the base lens at shipped defaults. The mean long is +121%.

### F1. The top of the long book comes from the valuation formula, not the world (CONFIRMED)

**What is wrong**
- `earnings_and_rent` applies `blended_ratio.powf(pool_beta)` with no limit on a firm's capacity to grow and no erosion of its share.
- The PowerEquipment pool grows 18.9x ($0.105T → $1.982T) and is still growing +48%/yr in 2036. Even fizzle gives 6.5x and +31%.
- POWL's implied share of that pool's profit doubles, from 0.41% to 0.86%.
- The terminal value is struck on 2036 earnings of a spend pool that is still accelerating. It makes up 83-85% of fair value for POWL, GEV and BESI.

**Measured impact on E[up]**

| Counterfactual | POWL | GEV | BESI | CLS | NVDA | AVGO | 6501.T |
|---|---|---|---|---|---|---|---|
| Shipped defaults | 450 | 256 | 351 | 199 | 176 | 206 | 198 |
| Revenue growth capped at 20%/yr | 80 | 39 | 108 | 62 | 47 | 104 | – |
| Terminal struck on steady-state spend | 59 | 19 | – | – | – | – | 64 |
| Steady-state terminal plus power-margin routing | 97 | 48 | – | – | – | – | – |

- At a 30%/yr cap, POWL is 330.
- Power-margin routing on its own makes things worse: POWL 450 → 581.
- In v2, routing takes POWL 94 → 18 and GEV 47 → −12.
- TECK stays near the top for a different reason (beta 1.4 on a GdpIndex/DcInfra blend). The cap barely moves it (288 → 272).

**Implementation plan**

`valuation.rs`
- Add `ValuationParams { capacity_gain, flow_terminal_gain, power_route_gain }`, all defaulting to 0 so `evaluate()` output is byte-identical.
- In `earnings_and_rent`, apply `g = g.min((1+c.max_rev_cagr).powi(i))` **before** `.powf(pool_beta)`.
- Flow terminal:
  - PowerEquipment: strike the terminal on `min(ratio_T, ss/p0)`, where `ss = power_equip_cost_per_gw × ai_power_gw_T × (1/life + g_T)`. This needs `power_equip_cost_per_gw` exposed from `Params` or the pools.
  - Silicon and DcInfra: use `compute_stock_T × (compute_deprec + g_T)` the same way.
- Power routing: multiply by `(power_margin/pm0)^w_pe` after the exponent, mirroring the silicon line. Ship it together with the flow terminal, never alone.

`companies.rs`
- Add `Company.max_rev_cagr`, default `INFINITY`.
- Seed values: 0.20 for POWL, BESI and LITE; 0.30 for GEV and 6501.T.
- The POWL 0.55 PowerEquipment / 0.45 GdpIndex remap belongs to the mapping-corrections queue item (Wave 0).

Tests (`tests/book.rs`)
- The existing guards lock gate-0 parity.
- New lock: with the flags on, no name without capture may have its implied share of pool profit rise more than 1.5x.
- New lock: `terminal_share` must be reported for every name.

### F2. Power binding and power rent come from an open-loop capex engine plus rent ceilings (CONFIRMED on the mechanism; PARTIAL on the proposed fixes)

This merges calibration #2, dynamics #1 and #2, and missing-first-order #1.

**What is wrong**
- Desired capex compounds on `demand_growth_base` (dgb) = 0.32 plus momentum in every scenario, and nothing ties it to revenue.
- Fizzle shows it plainly:
  - it binds CCPPPPPPPPP (C = chips, P = power, K = capital, D = demand) while ai_services is flat at $1.26T from 2030 to 2036;
  - capex still goes 1.33 → 4.50 over the same years;
  - fizzle upside stays positive for longs (POWL +52%, TECK +96%).
- Rents sit on their ceilings:
  - power utilization is ≥1.03 in all 11 baseline years, so the power-margin clamp binds 11/11 (10/11 in fizzle);
  - the literal 0.6 electricity margin binds 11/11;
  - `margin_ceiling` sets the 2033 power margin exactly (0.40 / 0.50 / 0.62 / 0.75 in, 0.400 / 0.500 / 0.620 / 0.749 out).
- Electricity is 60% of baseline power profit and barely reacts to `margin_ceiling`, because the separate 0.6 literal pins it.
- Physical utilization is tiny: ai_hew/ai_hew_raw is 0.097 in 2030 and 0.003 in 2036. The power that binds is power for GPUs the model itself leaves idle.
- Across MC lenses, power binds in 42-58% of 2028-34 run-years in the default lens, 0-5% in v2, and 70% → 12% over the window in enhanced.

**Measured impact on E[up]**
- dgb at 0.32 / 0.15 / 0 gives POWL 450 / 212 / 41 and GEV 256 / 120 / 12.
- Fizzle-only dgb 0.05 with momentum 0.2 moves the worst-case column: TECK +96 → −19 and POWL +52 → −54.
- `margin_ceiling` 0.45 takes the mean long from +121 to +70 (POWL −250pp, GEV −142pp) and flips 5 signs in v2.

**What does not work** (these are refuted in §4)
- Removing `0.5·adopt`, adding `power_anticipation`, or anchoring on revenue growth.
- q-governor at 0.5 alone still leaves 7 power years in the baseline.

**Implementation plan**

`lib.rs`, `demand_signal_growth`
- Add a gated `dgb_decay_tau` so that `dgb_t = g_gdp + (dgb0 − g_gdp)·exp(−t/τ)`. Default τ = ∞, which is golden-identical.
- Add a per-scenario `demand_growth_base` override in `scenarios.rs`: fizzle ≈ 0.10.
- Pin the 2026 starting `perceived_growth` separately, because a global dgb ≤ 0.20 flips the 2026 binding to Demand.
- Blend the whole signal, whatever `t_sing` is: `(1−g)·legacy + g·return_signal`. The return signal is a q-style ratio (ai_services profit over replacement value, from the F5 forward q), not revenue growth.

Rents (`margin_from` and `electricity_margin`)
- Add a gated `smooth_rents` saturation: `normal + (ceiling−normal)·tanh(slope·excess/(ceiling−normal))`.
- Promote `electricity_margin_cap` (default 0.6) to `Params`.
- Replace the scarcity `.min(2.0)` kink with the same saturating form.
- Expose `target_utilization` and the 0.15 width as parameters.

Diagnostics
- Add `years_at_margin_ceiling` and a per-year `clamp_occupancy` to book output.
- Flag names that get more than 50% of their present value from clamp-pinned years.

Tests
- With the gates at their calibrated values, fizzle and `singularity_year=2099` do **not** hold the ceiling through 2036.
- With the return anchor on, fizzle binds power for 3 years or fewer.
- `tests/loop_dominance.rs` locks the measured loop signatures: the power-supply response loop (B1) and the physical-acceleration loop (R4) each move 2029-36 capex by at least 20%; the recursive-AI loop (R1) moves 2026-28 displacement by at least 30%; the others stay within ±5%.

### F3. How much grid power each capex dollar buys has no anchor, and its direction is opposite the observed trend (CONFIRMED)

**What is wrong**
- `hw_cost_decline` 0.15 and `power_efficiency_gain` 0.12 together imply +3.5%/yr GW per dollar.
- Industry figures went the other way, from about $25-30B per GW (Hopper era) to about $50-60B (Blackwell/Rubin), so GW per dollar is falling.
- The 2026 level is $14.8B/GW.

**Measured impact** (holding `hw_cost_decline` fixed)

| GW-per-dollar trend | Power binds | POWL | GEV | VST | Other |
|---|---|---|---|---|---|
| +3.5%/yr (default) | 2028-34 | +450 | +256 | +70 | – |
| 0%/yr | 2029-34 | +417 | – | +36 | – |
| −5%/yr | 2032-33 only | +304 | – | −2 | – |
| −10%/yr | never | +126 | +65 | −30 | 2032 power margin 0.31; top 5 becomes BESI, TECK, AVGO, NVDA, MU |

- The flip threshold is about −3%/yr.
- The level alone does not move the binding: a consistent rescale leaves the binding string identical. It does reprice the electricity pool: VST +70 → −7 at $45B/GW.
- In the extended sensitivity run, `hw_cost_decline` is the largest driver of power-bound years (Spearman +0.67) and of NVDA (−0.75).

**Implementation plan**

`lib.rs` and `Params`
- Add `capex_per_gw_2026`, default 0.8125/55 (golden).
- Add `gw_per_dollar_growth`, default 0.88/0.85 − 1.
- Derive `gw_per_unit = cost_per_unit / capex_per_gw_t`.
- Rescale `ai_power_2026`, `power_additions_2026` and `electricity_price_normal` (or the electricity capture shares) from one place, so a level recalibration does not silently reprice VST, NRG and CEG.

Sampling and reporting
- Draw the level ($35-60B/GW) and the trend (−10% to +2%/yr) in the shared sampler.
- Report the book at trend 0 and at −5% next to the default until 2027 data resolves it.

Tests
- Add capex per GW ($40-60B) to `calibration_2026`, but only when the recalibration flag is on.

### F4. The book's uncertainty method is unsound (CONFIRMED)

This merges valuation #2, #4 and #5, calibration #5 and #6, and dynamics #6.

**What is wrong**
- The six named scenarios leave out the MC's dominant mode. Named capex never falls by more than a year's growth except in taiwan_shock (−38%). The MC shows a capex decline on 71% of paths (88% in enhanced, 100% in v2).
- Named E[up] versus MC mean, base lens:

  | Name | Named | MC mean |
  |---|---|---|
  | VST | 70 | 12 |
  | EQT | 67 | 17 |
  | CEG | 10 | −16 |
  | WDAY | −5 | +20 |
  | ASML | 20 | 8 (median −11, P(loss) 59%) |
  | POWL | 450 | 312 |

- The enhanced lens is bimodal. NVDA has a named value of −26 against an MC mean of +91 and median of −71; BESI is +224 by mean but −78 by median.
- The lens is the largest driver of upside and is not sampled:
  - Named NVDA reads 176 / −26 / −42 across base, enhanced and v2.
  - A mixed-lens MC gives NVDA mean +72, median +1, P(loss) 50%.
  - Spearman against the lens weight is −0.56 to −0.59 for silicon names and −0.68 to −0.70 for VST and CEG.
- Ranking by arithmetic mean rewards right-skew over a distribution where the terminal value dominates. By E[log] the order is POWL, TECK, BESI, GEV, MU, 6501.T, AVGO; NVDA is 12th.
- `sa` reports only 17 of about 36 drawn parameters. It hides drivers it already draws (jevons −0.489, efficiency jump −0.485) and draws none of dgb, `margin_ceiling`, `hw_cost_decline`, `price_adjustment` (−0.57 on the silicon normalization year), `q_governor_gain` or `compute_deprec`.
- The MC prior puts 0% on "never", while the book puts 9.3% on fizzle. Aligning them is under 2pp of book impact, but they should come from one source.

**Implementation plan**

`scenarios.rs`
- `pub fn sample_params(&mut ChaCha8Rng) -> Params`, moved out of `main.rs Draw::params`. `mc` and `sa` output must be byte-identical under the same seed.
- One `PRIOR` const that feeds both `SCENARIO_PROBS` and the sampler, adding a 9.3% `singularity_year=2099` mass and the 0.75 terminal-multiple factor on that branch.
- One `PARAM_TABLE` from which both the sampler and `sa`'s `param_names` are built. Add the missing drivers listed above with the ranges from the calibration and dynamics findings.

`main.rs`
- New command `book-mc <n> <seed> [base|enhanced|v2|mix]`.
- Per draw: simulate, then value every name with `evaluate()`'s per-path discount, terminal discount and rent haircut.
- Apply the Taiwan factor from each `TaiwanInvasion` start year.
- `mix` draws one coherent lens weight λ ~ U(0,1) on a **separate RNG stream**, scaling the 10 enhanced and v2 gains. There is also a variant with independent gains.
- Output per name: mean, p10/p50/p90, P(loss), E[log], named-minus-MC gap, and Spearman against λ and each gain. Label a name a "structural bet" when |ρ| > 0.4.
- Also a `book-sa` command.

`valuation.rs`
- Add `median_upside`, `p_loss`, `e_log` and `terminal_share` to `Evaluation`.
- Behind a flag (default off), sort `evaluate_all` by `e_log`, with `p_loss` as the tiebreak.
- Replace the `implied_cagr` display with `p_beat`, the fraction of MC paths whose 10-year earnings CAGR exceeds the price-implied CAGR.
- Flag "terminal on unsettled flow" when `terminal_share` > 0.8 and 2035→36 pool growth is above 20%. Today that flags POWL, GEV and BESI, which ties back to F1.

Tests
- Same-seed parity for `mc` and `sa`.
- `book-mc` must report the named-versus-MC gap rather than hide it.
- The hand-set 40/25/35 lens weights are replaced by the documented λ prior.

### F5. The v2 silicon verdict turns on uncalibrated settings of the revenue brake and the inference demand elasticity (CONFIRMED)

**The q-governor**
- It reads trailing ai_services profit. In 2027 that gives q = 0.035 and a capex cut to 0.55 from 0.66, which 2027 guidance already contradicts.
- v2 NVDA E[up] across the numerator form and the q_mult clamp:

  | q numerator | Clamp 1.0 | Clamp 1.25 | Clamp 3 |
  |---|---|---|---|
  | Trailing | −93.6 | −86.4 | −42.4 |
  | Forward | −20.1 | +121.8 | +394.7 |

- The growth-expectation seed (g_e) alone flips the sign at the same clamp: −46 versus +122.
- TSM spans −87 to +270.

**Inference demand elasticity (ε)**
- ai_services flattens at $10.2T while algorithmic efficiency (algo_eff) reaches 1,927x.
- With market clearing, E[up] at ε = 0.7 / 1.0 / 1.35:

  | Name | ε 0.7 | ε 1.0 | ε 1.35 |
  |---|---|---|---|
  | MSFT | −15 | +23 | +135 |
  | GOOGL | −12 | +44 | +105 |
  | NVDA | +3 | +176 | +196 |
  | PLTR | −69 | −28 | +92 |

- v2 NVDA is −91 at ε = 0.7 and +183 at ε = 1.35.

**Implementation plan**

Forward q (`lib.rs`, q-governor block)
- Gated `q_forward`. Numerator = trailing AI profit × annuity(g_e, r = long_rate + q_risk_premium, L = round(1/compute_econ_deprec)).
- g_e fades geometrically toward 0.15, is capped at 1.5, and is seeded from `ai_rev_growth_2026` (sampled 1.0-2.5).
- Clamp `q_mult` at 1.25 or below, and apply it to capex only, after `chip_utilization` (lib.rs:1438).

Inference clearing
- Gated `inference_clearing_gain`. Supply = `ai_hew_raw × (1 − training_share_t)`.
- Anchor the price path to observed $/token deflation, capped at 10x/yr.
- Sample ε over 0.7-1.4. Cap revenue at about 25% of GDP.
- Keep displacement on `min(supply, demand_hew)`.

Tests
- v2 2027 capex > 2026 capex at the default seed.
- NVDA E[up] non-decreasing in the g_e seed.
- ε = 1 byte-identical; MSFT and GOOGL monotone in ε (verified).
- Report the v2 silicon verdict as a band over seed × clamp × ε, not a point.

### F6. Short book: part AI thesis, part valuation convention; displacement plateau set by two constants (PARTIAL for the attribution; CONFIRMED for the plateau)

**Attribution**
- The market proxy (GdpIndex, beta 1, terminal multiple 16) has E[up] of −40% in every scenario, fizzle included. The 12% discount and terminal multiple (tm) convention is therefore off.
- To price the proxy fairly at 12%, tm would need to be 47 (fizzle) or 36 (baseline). The discount rate that zeroes it at tm 16 is 3.0%, not the 8-9% proposed.
- Names whose short is valuation-only (AI pools contribute ≤4pp): CHRW −77, LSTR −78, MMC −27, 6954.T −58, and FDS.
- Names where AI carries real weight: RHI 28pp, ADP 37pp, PAYX 38pp, MAN 27pp, TCS.NS 33pp.

**Displacement plateau**
- The plateau is exactly 0.765 = 0.85 × 0.90, where 0.90 is the hard-coded `.min(0.90)` at society.rs:326.
- Compute, algorithms and `ai_hew_2026_m` barely move it: rank-correlation of the book stays 0.998-1.000.
- A reorganisation lag with τ = 4 years takes 2033 displacement from 0.75 to 0.29 and the mean short from −46 to −36 (ADP −69 → −53, TCS −59 → −37, RHI −86 → −78). The long book is unchanged.

**Implementation plan**

`valuation.rs`
- Add `conv_gap`, `drift_delta` and `ai_delta` to `Evaluation`. Keep drift separate so a hand-set drift cannot pass as thesis.
- Behind a flag, the stance requires |ai_delta| > x. Names that fail are labelled "valuation-only".
- Calibrate the tm/discount pair jointly: a Gordon `tm = 1/(r − g_T)` flag, or per-name solved tm.

`society.rs` and `lib.rs`
- Promote `pre_ramp`, `adoption_gate_mult` (1.4), `adoption_ceiling_max` (0.90) and the logistic anchor to `Params` (golden defaults).
- Add a gated `tau_reorg` lag after the ratchet and a gated `reinstatement_rate`.
- Add `addressable_cognitive` and these constants to `PARAM_TABLE`.

Tests
- Largest yearly increase in displacement ≤ 0.22 by default and ≤ 0.08 with the calibrated gates (τ = 4 gives 0.054).
- The proxy's fizzle E[up] is within ±10% under the calibrated tm flag.

### F7. Material mechanics that bias the power timing and the left tails (CONFIRMED unless noted)

**a. Vintage power draw and a depreciation rate doing three jobs**
- The legacy model retroactively frees 34%/yr of stock power draw against 25% physical retirement.
- A vintage draw stock moves the first power year to 2027 and cuts cumulative capex by 9% (baseline) and 46% (enhanced).
- `compute_deprec` 0.15 / 0.25 / 0.35 gives v2 AVGO +2 / −22 / −51, a sign flip.
- Plan:
  - Add a `draw_stock` state plus a `vintage_power_draw` gain.
  - Substitute it at lib.rs:1333, 1364, 1434, 1451, 1677 and 2008. The scratch patch is byte-identical at gain 0.
  - Split `compute_retire_rate` from `compute_econ_deprec`.
  - Do **not** ship a retire rate of 0.17 without co-tuning `ai_power_2026`, because it flips the 2026 binding to Power.

**b. Capital cap at 5.5% of world GDP (about $6.3T in 2026)**
- It never binds before 2035.
- In fizzle, 14.2 of capex is booked as internally funded against 3.55 of cumulative AI profit.
- An operating-cash-flow cap at $0.60T growing 10%/yr: fizzle NVDA +5 → −20, book NVDA 176 → 157, baseline 2026-28 within 5%.
- Plan: gated `cashflow_funding_gain` with an external-equity term scaled by (1 + es_dev); the unfunded remainder flows to `sector_debt`.
- Lock: fizzle debt higher and 2036 capex lower than legacy.

**c. 2026 base year** (PARTIAL)
- A consistent rescale is book-neutral (rank-correlation 0.991).
- The 2026 binding is knife-edge: chips 0.662 against power 0.691, a 4% gap. That explains the 30% power share in the MC.
- An inconsistent base year produced +690%.
- Plan:
  - Tighten `anchors_2026`: binding == Chips with slack ≤ 0.97; AI electricity within an IEA-consistent bound; price $0.05-0.10/kWh.
  - Add a valuation-side assert `|states[0].pool/observed − 1| < 0.2` for every mapped pool. This is the highest-value line in this item.
  - Add ABC rejection on 2026 moments in `book-mc` and report the rejection rate.
  - Add a `checkpoints_2027` table (adoption 0.248, ai_services 0.328, capex 0.847, displacement 0.0127).

**d. Knife-edge in enhanced and v2** (PARTIAL)
- `chip_growth_ceiling` 0.59 → 0.595 moves enhanced cumulative power profit by −16%.
- Plan:
  - Blend the `prev_glut > 1.15` regime switch logistically with width ≥ 0.1.
  - Report book-enhanced and book-v2 as a neighbourhood or MC mean, not a point estimate.
  - Add an Erlang-k pipeline (a smoothed multi-year delay) as a gated, sampled option, without claiming a point effect.

**e. Algorithmic capability is cut off from outcomes**
- ai_hew is demand-bound in 86% of 2028-34 MC run-years.
- With algorithmic growth 1.0, 2034-36 displacement is identical to baseline.
- Plan: gated `capability_gated_adoption`, and a smooth saturation for the `btl_price.min(1.5)` clamp. This is a prerequisite for capture 6 (see §3).

## 3. Order of work, interleaved with the queue

The reason for this order: every later gate must be judged by how it shifts a **distribution**, and against **corrected mappings**. Loops that would otherwise be inert must be reconnected before new dynamics are layered on them.

**Wave 0: hygiene and display only (no structural change)**
1. Refresh stale marks (TECK financials; 002472.SZ and 300748.SZ; PLTR, NVDA, POWL, CLS; ASML EPS; short-interest sentinels).
2. **Mapping corrections**, all 17 items in weekend_deepdive §5. Identity and sign fixes first (WTKWY, CLS, SYM, EQT), then pool weights (POWL 0.55/0.45, GEV 0.75/0.25, LSTR +0.2 DcInfra, PAYX, MMC, 6268.T/6324.T capture). These are data edits with an updated book lock. Doing them first stops every later measurement from being taken on known-wrong mappings.
3. Move the sampler into the library and build `PARAM_TABLE`, with `mc`/`sa` byte-identical.
4. Display-only fields and diagnostics: `e_log`, `p_loss`, `median_upside`, `terminal_share`, `years_at_margin_ceiling`, `clamp_occupancy`.

**Wave 1: the measurement layer (F4, F7c, loop lock)**
5. `book-mc` with the lens sampled as λ, fizzle mass and a unified `PRIOR`; then `book-sa`.
6. Tighten `calibration_2026` and add the valuation base-year assert.
7. Add `tests/loop_dominance.rs`, so later work cannot silently revive or kill a loop.

**Wave 2: the power complex, the headline (F2 → F3 → F7a → F1)**
8. Decaying dgb plus fizzle's own dgb, with the 2026 `perceived_growth` pinned.
9. Reparameterize capex per GW (level and trend) and preserve the electricity pool.
10. `smooth_rents` plus `electricity_margin_cap`.
11. Vintage draw stock and the retire/economic depreciation split.
12. `ValuationParams`: capacity cap plus flow terminal plus power routing, shipped together. This goes last in the wave because the flow-terminal steady state reads the corrected spend and GW paths.

After Wave 2, re-state "power binds" as the MC p10/p50/p90 of power-window start, end and years.

**Wave 3: the revenue side, then the queued dynamics that depend on it**
13. `q_forward` with its guards.
14. `inference_clearing_gain`.
15. `cashflow_funding_gain`. This composes with capture rank 2 (the coupon/refinancing wall) and should precede it.
16. The F2 return-anchored demand blend, now that the forward q exists.
17. **Philosophy 5, the belief-fundamental wedge.** It must come after items 8, 13 and 16. The wedge inflates `perceived_growth` against a true fundamental and reveals through the q-governor, which needs:
    - a return-anchored demand signal (otherwise it sits on an already-open 32% loop);
    - a forward q that does not already force a spurious 2027 bust (otherwise the "bust while utilization is tight" lock cannot be told apart from the trailing-q artifact).
18. `capability_gated_adoption` (F7e), then **capture 6, endogenous algorithmic progress.** It must come after items 14 and 18. Measured today, algo_eff has no effect on displacement after 2031 (ai_hew is demand-bound 86% of the time) and none on AI revenue (demand is fixed-price). Capture 6's embargo and software-only locks would pass while being economically inert unless clearing and capability-gated adoption land first.

**Wave 4: labor, shorts and the rates regime**
19. F6 displacement parameters, `tau_reorg` and reinstatement; then the short-attribution columns.
20. Calibrate the tm/discount pair jointly (F6).
21. **Fiscal-dominance satellite.** It goes on top of the calibrated discount and after item 15, since both feed debt and discount. Today a flat 12% with hand-set tm already shorts the market proxy by 40%. Putting a stochastic term premium on that convention would double-count, and the result could not be told apart from convention drift. It can be built in parallel with Wave 3 but must merge after item 20.

## 4. Refuted or corrected, so they are not raised again

- **Setting `0.5·adopt` to 0 unpins the power margin:** no. Every scenario still ends at 0.62 (7 baseline years at the ceiling). It is still worth doing for silicon (NVDA −99pp).
- **`power_anticipation_gain` as a rent fix:** backwards. POWL goes to +844 / +1077 and fizzle stays pinned.
- **An anchor on revenue growth removes the baseline power bind:** no. The baseline stays CCPPPPPPPKK with capex +5%, and enhanced capex rises 29%. Anchor on return (q), not revenue growth.
- **Replacing `0.5·adopt` with a revenue link fixes fizzle:** it is inert in fizzle (the term sits behind `t_sing >= 0`) and raises baseline POWL 450 → 606.
- **q-governor at 0.5 or dgb = 0 alone removes the baseline power bind:** no. The results are 7 power years and a 2029-36 bind respectively.
- **Power-margin routing alone tames POWL:** it raises POWL to 581 in the base lens. Ship it only with the flow terminal.
- **A calibrated discount of about 8-9% zeroes the proxy:** no. It is 3.0% (5.3% in the baseline). This is a terminal-multiple problem (tm 36-47).
- **"The short book is not an AI thesis" as a blanket claim:** only for CHRW, LSTR, MMC, 6954.T and FDS. The wage-pool shorts carry 27-38pp of AI.
- **300748.SZ terminal share of 88%:** it is 66%.
- **NVDA 11th by E[log]:** it is 12th.
- **Utilisation of 68/31/7% for 2028-30:** off by a year. It is 1.00 / 0.44 / 0.097; the 2036 figure of 0.3% is correct.
- **Erlang-k delay gives −32% capex, −29% power profit and a peak glut of 15.25 in enhanced:** not reproduced. The measured results are +0..+19%, +0..+38% and 5.45 → 4.7.
- **A logistic regime-switch blend of width 0.05 removes the knife-edge:** no, it still leaves 14% jumps. Use width ≥ 0.1 plus neighbourhood averaging.
- **Unsampled structural parameters dominate the power-window distribution:** adding 7 draws leaves years p10/50/90 at 0/5/9. The low explained variance in `sa` is an artifact of its parameter list.
- **The 2026 GW-level mis-scaling reorders the book:** a consistent rescale gives rank-correlation 0.991 and an identical binding. Only the electricity pool (VST) is level-sensitive, and the +690% came from an inconsistent rescale.
- **Reweighting book scenarios to the MC singularity prior is material:** no. Rank-correlation 0.997, 0 flips, under 2pp. Unify the prior for hygiene only.
- **`ai_hew_2026_m` / `algo_eff_growth_pre` matter for the book:** no. Rank-correlation is 0.998-1.000. They are unidentified but book-neutral.
- **Forward g_e = 0.5 → +101%:** not reproduced. The results are −45.7 (chasing) and +3.8 (fixed) at clamp 1.25.
- **Retire rate 0.17 as the default:** it flips the 2026 binding to Power. It needs a co-tuned `ai_power_2026`.
- **An operating-cash-flow cap of $0.50T growing 5%/yr keeps the baseline within 5%:** no. Capital binds in 2027-28 and baseline-scenario NVDA goes 232 → 98. Use $0.60T at +10% plus an external-equity term.

Files referenced:
- `/home/user/Amulet-Map-Editor/singularity_economy/rust/singularity-econ/src/{lib.rs,valuation.rs,companies.rs,scenarios.rs,society.rs,main.rs}`
- `/home/user/Amulet-Map-Editor/singularity_economy/output/history/{weekend_deepdive.md,report_card_2026-09.md,philosophy_dynamics_shortlist.md,capture_dynamics_shortlist.md,next_cycle_worklist.md}`

This is scenario analysis, not investment advice.
