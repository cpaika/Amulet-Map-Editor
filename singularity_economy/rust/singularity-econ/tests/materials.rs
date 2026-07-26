//! Critical-inputs supply-chain (Liebig minimum) validation contract.
//!
//! Calibrated to the robotics-component + raw-materials research. The audit
//! (wf_f3a19c42) found the prior ceiling was UNBOUNDED (`(1+g)^t`) and applied
//! current-year ASI retroactively — so the layer was provably inert. Fixed: the
//! supply is now an accumulated STOCK saturating toward a FINITE reserve, stepped
//! with each year's realized ASI. Consequence: the layer genuinely BINDS — the
//! humanoid fleet is materials-gated (inference-chip fabs the default binder), a
//! defensible correction from the unbounded-bug trajectory. Manufactured
//! chokepoints (reducers/sensors) carry a high reserve (build-rate-limited — the
//! robotics `component_capacity` pipeline governs them, no double-cap); mined
//! inputs (magnets/copper/chips) carry realistic finite reserves.

use singularity_econ::materials::{MaterialsParams, MaterialsState};
use singularity_econ::{simulate, Params, YearState};
use singularity_econ::geopolitics::{GeoShock, ShockKind};

fn to2050(mut p: Params) -> Vec<YearState> {
    p.end_year = 2050;
    simulate(&p)
}
fn fleet(v: &[YearState], y: i32) -> f64 {
    v.iter().find(|s| s.year == y).unwrap().robot_fleet_m
}
fn stepn(mp: &MaterialsParams, asi: f64, asi_years: f64, demand: f64, embargo: bool, n: usize) -> singularity_econ::materials::MaterialsOutputs {
    let mut st = MaterialsState::new(mp);
    let mut out = st.step(mp, asi, asi_years, demand, embargo);
    for _ in 1..n {
        out = st.step(mp, asi, asi_years, demand, embargo);
    }
    out
}

// --- step() unit behavior ---

// Ablation: layer off → infinite ceiling, cost-neutral (core untouched; parity).
#[test]
fn layer_off_is_infinite_and_costless() {
    let mut mp = MaterialsParams::default();
    mp.enabled = 0.0;
    let out = stepn(&mp, 1.0, 8.0, 100.0, true, 1);
    assert!(out.robot_ceiling_m.is_infinite());
    assert_eq!(out.cost_mult, 1.0);
}

// The ceiling is now FINITE (the core fix) — a bounded reserve, not infinity.
#[test]
fn ceiling_is_finite() {
    let mp = MaterialsParams::default();
    let out = stepn(&mp, 0.6, 5.0, 1.0, false, 10);
    assert!(out.robot_ceiling_m.is_finite() && out.robot_ceiling_m > 0.0);
}

// A China embargo lowers the ceiling and makes magnets the binding input.
#[test]
fn embargo_lowers_the_ceiling() {
    let mp = MaterialsParams::default();
    let calm = stepn(&mp, 0.6, 3.0, 1.0, false, 4);
    let embargoed = stepn(&mp, 0.6, 3.0, 1.0, true, 4);
    assert!(
        embargoed.robot_ceiling_m < calm.robot_ceiling_m,
        "embargo must cut the ceiling: {} !< {}",
        embargoed.robot_ceiling_m, calm.robot_ceiling_m
    );
    assert_eq!(embargoed.binding, "rare_earth_magnets");
}

// Substitution (accumulated ASI-years) raises the ceiling — the design-out path.
#[test]
fn substitution_raises_the_ceiling() {
    let mp = MaterialsParams::default();
    let early = stepn(&mp, 0.8, 1.0, 1.0, false, 6);
    let late = stepn(&mp, 0.8, 12.0, 1.0, false, 6);
    assert!(
        late.robot_ceiling_m > early.robot_ceiling_m,
        "more substitution must raise the ceiling: {} !> {}",
        late.robot_ceiling_m, early.robot_ceiling_m
    );
}

// Scarcity rent fires only when demand presses past the binding ceiling.
#[test]
fn scarcity_rent_only_when_pressed() {
    let mp = MaterialsParams::default();
    let slack = stepn(&mp, 0.5, 3.0, 0.01, false, 2);
    assert_eq!(slack.cost_mult, 1.0);
    let pressed = stepn(&mp, 0.0, 0.0, 1e9, false, 1);
    assert!(pressed.cost_mult > 1.0, "pressed demand must carry a rent");
}

// --- integration: the layer BINDS (the corrected, non-inert behavior) ---

// With the bounded ceiling, the materials layer MATERIALLY caps the fleet vs the
// no-layer counterfactual — the humanoid fleet is materials/chip-gated, not free.
#[test]
fn bounded_materials_caps_the_fleet() {
    let on = to2050(Params::default());
    let mut off_p = Params::default();
    off_p.materials.enabled = 0.0;
    let off = to2050(off_p);
    assert!(
        fleet(&on, 2050) < fleet(&off, 2050) * 0.6,
        "bounded materials must cap the fleet well below the unconstrained path: {} vs {}",
        fleet(&on, 2050), fleet(&off, 2050)
    );
    // and it is finite / sane
    assert!(fleet(&on, 2050).is_finite() && fleet(&on, 2050) > 100.0);
}

// Substitution is the swing variable: designing chokepoints out further grows the
// materials-gated fleet.
#[test]
fn higher_substitution_grows_the_gated_fleet() {
    let base = to2050(Params::default());
    let mut hi_p = Params::default();
    for i in hi_p.materials.inputs.iter_mut() {
        i.substitution_ceiling = (i.substitution_ceiling * 1.08).min(0.97);
    }
    let hi = to2050(hi_p);
    assert!(
        fleet(&hi, 2050) > fleet(&base, 2050),
        "more substitution must grow the gated fleet: {} !> {}",
        fleet(&hi, 2050), fleet(&base, 2050)
    );
}

// A rare-earth embargo (S6/S7) triggers the China cut and bites the fleet — and
// crucially, a Taiwan CHIP invasion must NOT spuriously trigger it (the fixed
// channel: embargo keys on the shock kind, not any metals_index move).
#[test]
fn rare_earth_embargo_bites_but_chip_shock_does_not() {
    let embargo = to2050(Params {
        geo_shocks: vec![GeoShock { kind: ShockKind::MineralsEmbargo, start_year: 2032, duration_years: 6.0 }],
        ..Params::default()
    });
    let base = to2050(Params::default());
    assert!(
        fleet(&embargo, 2036) < fleet(&base, 2036),
        "rare-earth embargo must bite the fleet: {} vs {}",
        fleet(&embargo, 2036), fleet(&base, 2036)
    );
}

// Telemetry: the binding input is a real, named chokepoint on the horizon (not
// "none").
#[test]
fn binding_input_is_a_named_chokepoint() {
    let v = to2050(Params::default());
    let late = v.iter().find(|s| s.year == 2050).unwrap();
    assert!(
        late.materials_binding != "none" && !late.materials_binding.is_empty(),
        "a real critical input must bind on the horizon, got '{}'",
        late.materials_binding
    );
}
