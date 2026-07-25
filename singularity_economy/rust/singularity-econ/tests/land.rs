//! Land/teleoplexy diagnostics contract
//! (output/history/land_supplement.json). The Landian claim and the
//! thermostat claim are now both expressible; these tests pin which one
//! the model produces under which conditions.

use singularity_econ::{simulate, Loops, Params, SocietyParams, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// SPLIT VERDICT (the diagnostic's honest finding): the autonomization
// TREND is real — the index rises all decade (0.008 -> ~0.185) as capex
// outgrows wage-financed demand and displacement shrinks wages — but it
// stays far below the production-for-production regime threshold (~1.0,
// where China 2008-15 and Soviet forced industrialization sat before
// reverting). In-horizon: Land right about the trend, wrong about the
// politics (see the meltdown test). This test pins BOTH halves.
#[test]
fn autonomization_trends_up_but_stays_below_regime_threshold() {
    let states = base();
    let first = states.first().unwrap().autonomization_index;
    let last = states.last().unwrap().autonomization_index;
    assert!(
        last > first + 1e-9,
        "the means-ends-reversal trend should be present: {first} -> {last}"
    );
    assert!(
        last < 0.5,
        "autonomization {last} approaching production-for-production regime inside the horizon — recheck"
    );
}

// Meltdown ratio: economic change rate vs political response rate spikes
// early (politics lags) then COLLAPSES as transfers+regulation arrive —
// spike-then-collapse, not Landian divergence. Regulatory-latency history
// (web->GDPR 23yr, social 15yr, ChatGPT->AI Act 1.5yr) backs the shape.
#[test]
fn meltdown_ratio_spikes_then_collapses() {
    let states = base();
    let peak_year = states
        .iter()
        .max_by(|a, b| a.meltdown_ratio.total_cmp(&b.meltdown_ratio))
        .unwrap()
        .year;
    assert!(
        peak_year <= 2030,
        "meltdown ratio peaked late ({peak_year}) — response never engaged"
    );
    let peak = states.iter().map(|s| s.meltdown_ratio).fold(f64::MIN, f64::max);
    let last = states.last().unwrap().meltdown_ratio;
    assert!(
        last < 0.35 * peak,
        "meltdown ratio did not collapse: {last} vs peak {peak} — Land divergence"
    );
}

// The model CAN express Land's world — ablate the balancing ensemble
// (no transfers, no backlash, no affordability) and autonomization must
// end HIGHER than baseline. If ablating every brake changes nothing, the
// diagnostic is decoration.
#[test]
fn land_regime_expressible_under_balancing_ablation() {
    let land_world = simulate(&Params {
        loops: Loops {
            b2_backlash: 0.0,
            b3_affordability: 0.0,
            society_layer: 0.0,
            ..Loops::default()
        },
        society: SocietyParams { b5_relief: 0.0, ..SocietyParams::default() },
        ..Params::default()
    });
    let base_last = base().last().unwrap().autonomization_index;
    let land_last = land_world.last().unwrap().autonomization_index;
    assert!(
        land_last > base_last + 1e-9,
        "ablating every balancing loop must raise autonomization: {land_last} vs {base_last}"
    );
}
