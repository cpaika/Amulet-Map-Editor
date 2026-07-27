Verified against the live tree before writing. Key correction: audit finding #6 (two glut tests RED) does **not** reproduce — `cargo test --all-features` is fully green (19/19 suites pass, `silicon_glut_emerges_but_power_never_gluts` and `bust_lag_within_century_envelope` both `ok`). Findings #1 (fracture inversion, arithmetic confirmed from default params), #5 (unused import, warning confirmed), and #2/#3 (frozen-multiple + stale note, confirmed from regions.rs:248-281) are real. Work-list below.

---

# Next-Cycle Work-List — singularity-econ

## 1. MUST-FIX (regressions / bugs, most severe first)

1. **[HIGH] `bloc_fracture_risk` inverts the design thesis — China scores LOWEST** (`regions.rs:269-281`). Base term `st.min(1.5)` saturates for the high-stress democracies (US/EU peak ~1.5-1.58) while China's suppressed stress (~1.02) + brittle add-on only reaches ~1.29. The trade book reads US/EU as *more* regime-shift-prone than China — exact inverse of intent. Make the brittle term superlinear (`brittleness*(st-thr).powi(2)*GAIN`) and/or gate the raw-stress base OFF for brittle blocs, then assert China peak > US/EU on the default run. *This is the flagship fix — the fat-tail geopolitics signal the book keys off is backwards.*

2. **[MED] Green-ammonia `wire` damping is inverted for the new sub-1.0 energy driver** (`food.rs:121`). C4 switched food's fuel driver to `energy_out.cost_index` (falls to ~0.53 by 2040), but the pre-C4 wire flip was calibrated for an always-up spike signal — so at year≥2035 it *raises* `fert_target` (0.673→0.888) and shrinks the relief exactly when the commit claims fertilizer becomes more electricity-coupled. Split by direction: keep wire damping only for `fuel_price_index>=1.0`, full coupling below. Add a regression asserting the food-price decline does not slow at `green_ammonia_year`. Existing test only probes fuel=3.0, giving false comfort.

3. **[MED→LOW] `displacement_modifier` does not un-freeze the stress ordering it claims to** (`regions.rs:50-58, 250-256`). It's still a compile-time constant → China stress is a fixed 0.49× of US at every step (was 0.35×); ordering is permanently pinned. Either correct the comment to "rescales the fixed fraction," or (if reordering is genuinely wanted, and it's needed for fix #1 to bite) drive the modifier from a state-dependent trajectory (bloc robot-adoption / brittleness feedback). Pairs with #1.

4. **[LOW] Unused `ShockKind` import** (`tests/materials.rs:16`) — confirmed compiler warning on every `cargo test --test materials`. Change line 16 to `use singularity_econ::geopolitics::GeoShock;`. Trivial.

5. **[LOW] Stale calibration note** (`regions.rs:81-84`) — comment cites China peak stress ~0.86; post-C11 it's ~1.02. Update the parenthetical. Trivial; do alongside #1/#3.

6. **[LOW] `dr_beta` "live in MC sampler" is an overclaim** (`valuation.rs:197-200`). Plumbing is correct but no caller perturbs it — `book()` hardcodes `MacroParams::default().dr_beta`, and `Draw::params()`/`monte_carlo` never touch valuation. Either wire `book()` to pass drawn `p.macrofin.dr_beta` + add it to the sampler, or soften the doc. Cheap doc fix now, real wiring when the MC valuation path lands.

7. **[LOW/LATENT] `longevity_pool` cap binds in year 1** (`bio.rs:280` init 60.0 vs `bio.rs:164` cap 64.0). Growth `*=1+0.095*(...)` slams the cap immediately, killing the "9-10% CAGR, funding-cyclical" dynamic. Currently inert (no consumer), so no live regression — but re-anchor init to ~$26B (or raise the cap) and add a guard test (`pool at end_year < cap`) before it's ever wired into valuation.

8. **[VERIFY / likely resolved] Glut conclusion-lock tests** (`behavior.rs:131`, `backtest.rs:97`) — audit reported these RED; they **pass on the current committed tree**. The 1.3 glut-detection convention is intact. No action beyond keeping the `capacity_glut>1.3` convention documented and not letting future capex/ceiling retunes push the peak back under 1.3.

---

## 2. NEW DYNAMICS (ranked by value / effort)

**Best 5-8, do first:**

1. **Occupational wage compression** — `demography.rs` wage-index stocks (`wage_index_cog/phys`, default gains 0.0). *Quick win (small effort).* Adds the missing price channel on the wage-bill stock: pool falls faster than headcount, reviving the RHI/ADP/PAYX/MAN shorts currently KILLED on "elasticity <0.5"; the two-sided scarcity counter-loop props HumanPhysicalWages (trades) and guards CHRW/LSTR. Single highest-leverage knob for the short sleeve; golden-safe at gain 0.

2. **Equity-wealth-effect on consumption** — one term at `lib.rs:1446` + `equity_wealth` stock, `wealth_effect_gain=0.0`. *Quick win (small effort).* Transmits a financial bust into real GDP (MPC ~4¢/$), making the broad GdpIndex pool genuinely cyclical and deepening bust-scenario short returns (RHI/CHRW/LSTR hit by displacement AND recession). Chains onto #3.

3. **AI-capex bubble/bust reflexivity loop** — new `equity_sentiment` stock in `macrofin`, feeds `desired_capex` (lib.rs:969) + `credit_mult` (lib.rs:993), asymmetric collapse on `capacity_glut>1.3`. *Medium effort — build this as the spine.* The Soros/Minsky loop the model structurally lacks; converts the smooth glut into a sharp boom-bust, fattening the left tail on all silicon/power names and re-ranking toward contracted cash flows (CEG) over merchant torque (VST/NRG).

4. **Transmission / HVDC delivery lag** — new `transmission.rs`, `deliverable_gw = min(generation, transformer throughput)`, gated `transmission_gain=0.0`. *Medium effort; author's #1 conviction mover.* Answers "is generation or delivery the ceiling" — today a generated GW = a usable GW. Structurally longer scarcity rents for GEV/POWL (the bottleneck *is* their product) and extends the IPP thesis (VST/CEG/NRG); mild cap on pure compute-volume upside.

5. **Circular vendor financing (chip→cloud→neocloud)** — `vendor_credit` stock in `macrofin`, split the external slice at `lib.rs:1557`, `vendor_financing_share=0.0`. *Medium effort; chains off #3.* Makes AVGO's existing "vendor-financier drift" note endogenous; NVDA/AVGO earnings quality falls in a bust (revenue that was recycled capital), arguing to trim NVDA into the boom and favor the MRVL-vs-AVGO short.

6. **UBI vs Job-Guarantee transfer split** — `society.rs` B5, `jg_gain=0.0`. *Medium effort; largest discount lever in the set.* JG lowers net fiscal cost → smaller sovereign snowball → lower `scenario_discount` → lifts duration-heavy power/toll longs and props the wage floor; UBI does the opposite. The switch reweights the whole long/short book.

7. **Compute-governance / licensing regime (B12)** — new `governance.rs`, multiplier on `units_added` (lib.rs:1116) + `chip_growth` ceiling (lib.rs:1024), empty-vec gated. *Medium effort.* Throttles the R1 training flywheel at its source (not just diffusion, which society already does); caps the silicon/HBM scarcity-rent tail (NVDA/TSM/MU) and raises licensed-incumbent value.

8. **International treaty / pause hazard** — new `('pause',…)` scenario in `scenarios.rs` + `pause_year=0` gate, added to `SCENARIO_PROBS`. *Medium effort.* The book's missing AI-specific left-tail hedge *trigger*: a pause craters the capex complex but is bullish incumbent-model rent — quantifies the strike for puts / silicon-beta cuts when treaty indicators fire.

**Also worthwhile (second tier):**

- **Reserve-currency convenience-yield erosion** — `macrofin` `reserve_share` stock, multiplies `term_premium_gain`; high value/medium. Rates-up amplifier that tilts the book ex-US and toward real assets (TSM/ASML/TECK/MP) in the high-debt tail.
- **Alignment-liability law regime** — `society.rs` `liability_stringency`, reuses `compliance_cost`/`credit_spiral` plumbing; high value/medium. Widens the self-insurable-megacap vs marginal-deployer gap (long MSFT/GOOGL, short marginal deployers).
- **Broad tariff / reshoring ladder** — `geopolitics.rs` two new ShockKinds + `inflation_add`; high value/medium. Strengthens domestic reshoring longs (GEV/POWL/CLS), weakens China-component longs in a tariff tail.
- **Sovereign-AI compute nationalism** — per-bloc `sovereign_compute` in `regions.rs`; high value but **large effort**. Reframes the NVDA China-ban bear case (duplication expands total GPU capex) and is broadly bullish the power/DC complex + undercuts the EQIX short.

**Lower priority (defer):** reskilling pipeline (medium/medium, mostly a scenario-fan widener), gig/informal absorption (medium/medium), water+cooling (medium/small — flags a VRT/nVent WATCH add), land/permitting queue (medium/small), e-waste circular loop (medium/medium, caps a commodity tail), governance divergence across blocs (medium/medium), insurance/pension LDI fire-sale (medium/**large** — high plumbing cost for a tail-only multiplier).

---

## Recommended next 3 actions

**First, fix the two theses-inverting bugs before any new dynamics ship:** correct `bloc_fracture_risk` so China is the standout fat-tail (MUST-FIX #1, superlinear brittle term + verify China>US/EU) and split the green-ammonia `wire` by direction so food-price relief stays monotone under the new falling cost driver (#2) — both feed the trade book a currently-backwards signal, and both need a new regression test. Sweep the trivial companions in the same PR (unused import #4, stale note #5, `displacement_modifier` comment #3, `dr_beta`/longevity doc-and-guard #6/#7), and record that the glut-lock tests are green so the 1.3 convention isn't accidentally retuned. **Second, land the two small-effort golden-safe quick wins** — occupational wage compression (revives the killed short sleeve) and the equity-wealth-effect term (closes the bust→GDP channel) — since both are gated to 0.0 and give immediate trade leverage for low cost. **Third, start the financial-fragility spine**: build the AI-capex bubble/bust `equity_sentiment` loop as the anchor stock, because the vendor-financing and wealth-effect ideas both chain off it — that cluster is the single largest structural gap (the model has no reflexivity/Minsky loop today) and is what makes every silicon/power left-tail credible.