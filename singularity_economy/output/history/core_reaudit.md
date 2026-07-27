# Core Re-Audit (post-refactor, 3 adversarial agents)

Three independent skeptics traced the refactored core's interaction surfaces for
cumulative regressions — the four golden-touching changes (A7/C8, P0, V, B4) plus the
gated C7/C2. Disposition of every finding below. Fixed items are committed; the
skipped ones are conservative or immaterial with a stated reason, not silent.

## Fixed (3 commits)
| # | Finding | Severity | Fix |
|---|---|---|---|
| C7 | Bio/cyber dread shock forced `incident=true`, flooring every sub-0.15 dread step to the AI-incident 0.15 — the flat ratchet C7 was meant to remove. | **CONFIRMED** | Gate the AI-step on a genuine AI-capability incident; dread ratchets its own severity step. Lock strengthened (0.10 vs 0.14 must differ). |
| P0 | `physical_power_binds` flagged true whenever the physical cap fell below the abstract cap — even in a chip-starved year with slack power. | PLAUSIBLE | Require power to be the effective Liebig minimum. |
| V | `electricity_margin` had an upper cap but no lower floor; since capture rides `pool × margin`, a sub-0.4× price could flip capture NEGATIVE. | PLAUSIBLE (latent) | Floor at 0.0 — a generator books zero margin in a glut, not negative. |
| C2 | Flow-only CAPTURE_DECAY too weak vs ~20× pool growth: final year still ~84% rent and growing, capitalized at the FULL terminal multiple. Item-6's "haircuts terminal value" claim was cosmetic. | PLAUSIBLE (material) | Split the terminal — durable base at full multiple, rent slice at `multiple × dr/(dr+decay)` (~⅔). De-rates VST 137→97 / NRG 83→61 / CEG 58→34 / MP 59→22%; book structure + all locks hold. |

## Verified CLEAN (no change)
- **No power double-subtraction / re-charge.** A7 subtracts the depreciated-retained
  compute draw once, in both the abstract and physical headrooms (identical terms).
  `ration ∈ [0,1]`, divide-by-zero safe, `power_cap ≥ 0`. Baseline byte-identical.
- **`firm_power_prev` lag correct** — reads year Y-1, updated after energy.step; the
  2700 seed is dead after year 1.
- **C7 MAX-not-sum** across co-occurring bio+cyber dread is real in the code (not just
  the test); `stringency_step` consumed exactly once.
- **B4 `fab_ceiling_mult ≥ 1.0` always** (every term `1 + non-negative`); cannot cut
  the ceiling or re-inflate the >150%/yr overbuild. Default cap ~0.86/yr.
- **B4 × spine:** the lower fab ceiling did NOT starve the Minsky trigger — baseline
  peak glut 1.352 > 1.15, the crack still fires (sentiment 1.37→0.65 with the gate on).
- **Minsky trigger** correctly lagged, gated behind `equity_sentiment_gain>0` (baseline
  byte-identical), no boundary kink.
- **Valuation units** (`×1000`: $T pool profit → $B earnings), the `(i-4).max(0)` decay
  exponent (no off-by-one, no overlap with phase-in), the Taiwan-haircut precedence,
  and the electricity re-anchoring (holds for ALL of VST/NRG/CEG, not just NRG) all
  check out.

## Assessed, not fixed (conservative / immaterial — stated, per the "no silent caps" rule)
- **Robot draw pre- vs post-attrition (Agent 1 #2).** The power caps reserve for the
  full last-year fleet; the ration gate powers the attrition-survived fleet (8% less).
  So the caps over-reserve ~8% of robot draw against compute — an internal
  inconsistency, but CONSERVATIVE (it under-states compute, never over-states it) and
  ~1.3 TW at endgame scale. Aligning it would regen the golden for a marginal,
  favorable-direction change; deferred to the next intentional golden bump.
- **`firm_power_prev` 2700 seed ~14% above the energy layer's own 2026 firm power.**
  Dead after year 1; biases only a scenario tuned to bind exactly at 2026. Calibration
  nit, not a live bug.
- **Physical ceiling omits `orbital_gw_equiv`.** Orbital solar <1 GW in-horizon;
  negligible, no parity break.
- **EQT mapped to the Electricity pool without a GW anchor (Agent 2 #3).** A gas E&P
  earning "electricity capture" is a modeling stretch, but the position is flagged
  killed in-notes and doesn't distort the live book. Company-mapping judgment, not a
  code defect — left for owner review.

## Net
The refactored core holds up under adversarial trace. One real defect (C7) is fixed;
two latent hazards are closed defensively; the one material valuation gap (terminal
rent) now has a genuine mechanism replacing a cosmetic one. Everything green
(20 cargo + 15 buck2), model golden untouched by the valuation change, baseline
byte-identical throughout. Scenario analysis, not investment advice.
