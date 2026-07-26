//! Critical-inputs supply-chain (Liebig minimum) validation contract.
//!
//! Calibrated to the robotics-component + raw-materials research
//! (output/history/robotics_components.md, raw_materials_mines.md,
//! asi_materials_optimization.md). Central finding it encodes: whether materials
//! BIND depends entirely on the ASI-substitution assumption. In the aggressive
//! baseline (RE-free motors, cycloidal/QDD reducers, AI materials discovery) the
//! chokepoints are designed out faster than the fleet grows into them, so the
//! layer is SLACK and the bind sits on throughput/energy/capital. Turn
//! substitution off and materials become the wall.

use singularity_econ::materials::MaterialsParams;
use singularity_econ::{simulate, Params, YearState};
use singularity_econ::geopolitics::{GeoShock, ShockKind};

fn to2050(mut p: Params) -> Vec<YearState> {
    p.end_year = 2050;
    simulate(&p)
}
fn fleet(v: &[YearState], y: i32) -> f64 {
    v.iter().find(|s| s.year == y).unwrap().robot_fleet_m
}

// --- step() unit behavior ---

// Ablation: with the layer off the ceiling is infinite and cost-neutral, so the
// robotics block is byte-identical to legacy (verified separately by parity).
#[test]
fn layer_off_is_infinite_and_costless() {
    let mut mp = MaterialsParams::default();
    mp.enabled = 0.0;
    let out = mp.step(10.0, 1.0, 8.0, 100.0, true);
    assert!(out.robot_ceiling_m.is_infinite());
    assert_eq!(out.cost_mult, 1.0);
}

// A China embargo must LOWER the ceiling (magnets are ~90% China-concentrated),
// and it must make magnets the binding input.
#[test]
fn embargo_lowers_the_ceiling() {
    let mp = MaterialsParams::default();
    let calm = mp.step(4.0, 0.6, 3.0, 1.0, false);
    let embargoed = mp.step(4.0, 0.6, 3.0, 1.0, true);
    assert!(
        embargoed.robot_ceiling_m < calm.robot_ceiling_m,
        "embargo must cut the ceiling: {} !< {}",
        embargoed.robot_ceiling_m,
        calm.robot_ceiling_m
    );
    assert_eq!(embargoed.binding, "rare_earth_magnets");
}

// Substitution (accumulated ASI-years designing out the chokepoint) must RAISE
// the ceiling — the RE-free-motor / cycloidal-reducer relief path.
#[test]
fn substitution_raises_the_ceiling() {
    let mp = MaterialsParams::default();
    let early = mp.step(5.0, 0.8, 1.0, 1.0, false); // 1 ASI-year of design-out
    let late = mp.step(5.0, 0.8, 12.0, 1.0, false); // 12 ASI-years
    assert!(
        late.robot_ceiling_m > early.robot_ceiling_m,
        "more substitution must raise the ceiling: {} !> {}",
        late.robot_ceiling_m,
        early.robot_ceiling_m
    );
}

// Scarcity rent: when demand presses past the binding ceiling, delivered unit
// cost rises above 1.0; when slack it is exactly 1.0.
#[test]
fn scarcity_rent_only_when_pressed() {
    let mp = MaterialsParams::default();
    let slack = mp.step(2.0, 0.5, 3.0, 0.01, false); // tiny demand
    assert_eq!(slack.cost_mult, 1.0);
    let pressed = mp.step(0.0, 0.0, 0.0, 100.0, false); // demand >> 2028 ceiling
    assert!(pressed.cost_mult > 1.0, "pressed demand must carry a rent");
}

// --- integration: the substitution bet ---

// The aggressive baseline is SLACK: the layer neither cuts the fleet nor moves
// unit cost, because substitution outpaces the ramp. Layer-on == layer-off.
#[test]
fn aggressive_baseline_is_non_binding() {
    let on = to2050(Params::default());
    let mut off_p = Params::default();
    off_p.materials.enabled = 0.0;
    let off = to2050(off_p);
    assert!(
        (fleet(&on, 2050) - fleet(&off, 2050)).abs() < 1e-6,
        "baseline must be non-binding (substitution keeps pace): {} vs {}",
        fleet(&on, 2050),
        fleet(&off, 2050)
    );
}

// Kill substitution and materials become the wall: the fleet is cut hard vs the
// no-layer counterfactual — the layer is genuinely live, the baseline slack is a
// finding about substitution, not an inert layer.
#[test]
fn pessimistic_substitution_makes_materials_bind() {
    let pess = |nomat: bool| {
        let mut p = Params::default();
        p.end_year = 2050;
        if nomat {
            p.materials.enabled = 0.0;
        }
        for i in p.materials.inputs.iter_mut() {
            i.substitution_ceiling *= 0.15; // ASI cannot design the chokepoint out
            i.asi_supply_boost *= 0.3;
            i.supply_growth *= 0.5;
        }
        p.geo_shocks = vec![GeoShock {
            kind: ShockKind::MineralsEmbargo,
            start_year: 2032,
            duration_years: 6.0,
        }];
        simulate(&p)
    };
    let on = pess(false);
    let off = pess(true);
    assert!(
        fleet(&on, 2050) < fleet(&off, 2050) * 0.5,
        "pessimistic substitution must let materials halve+ the fleet: {} vs {}",
        fleet(&on, 2050),
        fleet(&off, 2050)
    );
}

// Telemetry: the binding input walks from precision reducers (the tight early
// line) toward copper/bulk inputs as substitution + capacity relieve the exotic
// chokepoints — the "constraint migrates to bulk throughput" hand-off.
#[test]
fn binding_input_walks_reducers_to_bulk() {
    let v = to2050(Params::default());
    let early = v.iter().find(|s| s.year == 2029).unwrap();
    let late = v.iter().find(|s| s.year == 2050).unwrap();
    assert_eq!(early.materials_binding, "precision_reducers");
    assert_eq!(late.materials_binding, "copper");
}
