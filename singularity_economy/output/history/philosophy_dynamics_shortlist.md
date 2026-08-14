# Philosophy-Derived Missing Dynamics — Implementation Shortlist

Six philosopher-agents read the model through traditions it had never been exposed to
(political legitimacy, critical political economy, philosophy of technology,
epistemology, meaning/virtue/status, mimetics/religion); a synthesizer deduped ~25
proposals, verified each against the code (line references below checked), and ranked
the five most load-bearing. Convergence between independent traditions was treated as
signal. All five are to be implemented as GATED satellites (gain 0.0 => byte-identical).

## Rank 1 — R10 fiscal-legitimacy closure: austerity backlash + decaying transfer efficacy
*Traditions: political-legitimacy + meaning/virtue (independent convergence).*
**Gap (verified):** `macrofin` computes a transfer-cap squeeze which `throttle_transfer_cap`
applies silently — a transfer CUT has zero political consequence; and the transfer-relief
constants (backlash damp, relieved halflife, unrest gate) are eternal — pacification never
fades, so no mature-UBI end state can produce unrest. Cuts are politically free and
pacification is eternal: both wrong.
**Mechanism:** (a) austerity backlash — store `prev_transfer_share`; a cut produces a
sentiment pulse `austerity_backlash_gain * cut_frac * (1 - sentiment)` and suspends the
relieved halflife that year. (b) efficacy decay — `e = jg_share + (1-jg_share) *
exp(-years_transfers_active / meaning_tau)` (JG retains meaning; pure UBI leaks it);
apply `e` to the backlash damp, the relieved halflife, and the unrest gate
(`transfers_active && e > unrest_efficacy_floor`).
**Params:** `austerity_backlash_gain` (0.0 gate; ~6.0), `meaning_tau` (5.0),
`unrest_efficacy_floor` (0.4). New state: `prev_transfer_share`.
**Locks:** debt-squeeze scenario ⇒ sentiment/unrest spike within 2yr of the first
throttle + 2036 adoption < baseline; at year 9 of transfers, backlash damp with
jg_share=0 < jg_share=0.6 (UBI leaks pacification, JG retains it).

## Rank 2 — B13 effective-demand realization closure (who buys the output)
*Tradition: critical political economy (Kalecki/Marx realization problem).*
**Gap (verified):** the GDP update is pure supply-side; once displacement plateaus, a
permanent 30pp wage-share loss with capped transfers has ZERO steady-state GDP effect,
and ai_services books 45% of displaced wage value regardless of whether that income was
ever respent. No level identity for demand anywhere.
**Mechanism:** derived (no new stock): `funded_share = (human wage bills + transfers +
mpc_profits × total profits) / gdp`; `gap = max(0, consumption_target - funded_share)`
with `consumption_target` auto-calibrated at t0 (gap=0 in 2026 by construction).
Couplings: `-realization_gain × gap` in the GDP growth expression; scale ai_services
and casualty-pool revenues by `(1 - real_pass × gap)`. Downstream free: lower
ai_services → q-governor brakes earlier; lower GDP → worse r−g in macrofin.
**Params:** `realization_gain` (0.0 gate; ~0.5), `mpc_profits` (0.25), `real_pass` (0.5).
**Locks:** Kalecki lock — high-displacement, transfer_cap 0.03 vs 0.15: with gain on,
2036 GDP AND profits.ai_services both strictly higher in the high-cap run (transfers
fund the profits that buy the transfers); gap monotone in wage_compression_cog_gain.

## Rank 3 — Protected-share ratchet: carve-outs remove task-space LEVEL, not rate
*Traditions: STRONGEST convergence — Polanyi decommodification + Durkheim consecration
+ guild licensing, independently demanding the same state variable.*
**Gap (verified):** `addressable_cognitive` is a constant; every political actuator is
rate-shaped or a recoverable ceiling — no social event can shrink the addressable pie,
so every backlash path reaches the same asymptote, just later. Terminal DCF values ride
the asymptote, so this changes where adoption STOPS, not how fast.
**Mechanism:** new ratchet stock `protected_share ∈ [0, protect_cap]` (no in-horizon
decay, like reg_stringency). Inflows: drip `carveout_gain × sentiment ×
(labor_power/labor_power_2026) × headroom` (Polanyi/guild channel — reuses S4, making
the R6-erosion-vs-carveout race meaningful); consecration pulse `taboo_pulse ×
dread_severity × headroom` on dread incidents (Durkheim channel). Actuation:
`addressable × (1 - protected_share)` on cognitive and physical, max-overlap composed
with demography's care-share to avoid double-count; sentiment halflife interpolates
toward relieved as protection substitutes for compensation (lowers transfer ramp and
long rate).
**Params:** `carveout_gain` (0.0 gate; ~0.03/yr), `protect_cap` (0.25), `taboo_pulse`
(0.1). New stock: `protected_share`.
**Locks:** asymptote lock — gain on ⇒ 2036 disp, displaced_value, ai_services all
strictly lower AND long_rate lower; terminal displacement monotone decreasing in
protect_cap; ratchet never decreases.

## Rank 4 — Endogenous winner-tax extraction capacity with pool incidence
*Traditions: political-legitimacy + offshore-mobility (crit-pol-econ) + elite-scapegoat
(mimetics) — three traditions on one line of code.*
**Gap (verified):** `debt_financing_share` flips 0.6 → 0.15 on a pure `years > 5` TIMER,
and no tax is ever subtracted from any profit pool — revenue appears fiscally without
existing economically. The book holds both halves of a fiscal contradiction: cheap
sovereign financing AND untaxed capture pools.
**Mechanism:** new stock `T` (extraction capacity, init 0.10): `dT = capacity_buildout ×
pressure × (1 - suppression) - base_mobility × T`, pressure = transfer-cap squeeze
(bond-market coercion), suppression = the existing B8 capture gate (mass outrage unlocks
taxation exactly as it unlocks regulation). Couplings: `debt_financing_share = 0.6 -
0.45 × min(T/0.75, 1)` (replacing the timer when gated on); incidence
`profits.ai_services ×= (1 - winner_tax_rate × T)`.
**Params:** `capacity_buildout` (0.0 gate → legacy timer verbatim; ~0.08/yr),
`base_mobility` (0.05/yr), `winner_tax_rate` (0.25). New stock: `T`.
**Locks:** two-sided — capture-dominant run keeps debt share >0.5 and higher long_rate
(extraction failure priced); mobilization run taxes ai_services below baseline while
debt/GDP and long_rate fall; assert the orderings are OPPOSITE across the two runs.

## Rank 5 — Belief-fundamental wedge: the epistemic bust (Goodhart/mimetic/prophecy)
*Traditions: epistemology (signal decay, Goodharted metrics) + mimetics (rivalry capex,
prophecy disappointment) — four proposals converging.*
**Gap (verified):** perceived_growth chases TRUE demand with only a smoothing lag; the
spine's sole crack condition is a true physical glut (>1.15); the q-governor reads true
profit. The model cannot produce history's dominant bust mode — the dot-com crack that
fires while physical utilization is still tight, because the METRIC was corrupted, not
the capacity.
**Mechanism:** new stock `g ≥ 0` (the measured-vs-true wedge): builds with boom pressure
`wedge_gain × (es_dev⁺ + capex_growth⁺) × (1 - g/0.5)` (capex-flow-referenced, so it
does NOT shut off when demand rolls over); perceived_growth chases `demand_signal ×
(1+g)` (capex desire and the spine's boom term run on the inflated number; true ai_hew /
displacement / pools keep truth). Revelation: `g_closed = reveal_rate × adopt × g`
closes the wedge as deployment reveals reality, each closure pulsing
`equity_sentiment -= reveal_es_pulse × g_closed` — disappointment as an operator.
**Params:** `wedge_gain` (0.0 gate; ~0.5), `reveal_rate` (0.3), `reveal_es_pulse` (0.5).
New stock: `g` + `capex_2yr_ago`.
**Locks:** new-regime lock — spine+wedge on ⇒ at least one year with equity_sentiment
< 0.8 while THAT year's glut < 1.15 (a bust the baseline architecture provably cannot
produce; assert its absence at wedge_gain=0); capex overshoot monotone in wedge_gain.

## Implementation order
1 → 2 → 3 → 5 → 4. (1 and 2 close existing half-built loops; 3 is the strongest
convergence; 5 adds a qualitatively new bust regime; 4 is the most invasive replacement
of an existing mechanism and goes last.) Each: gated, golden-parity-checked, test-locked,
menu-documented, committed separately. Scenario analysis, not investment advice.
