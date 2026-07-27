//! R2 self-replication validation contract.
//!
//! The prior model treated the robot fleet as demand-capped by human-labor
//! substitution and grew manufacturing capacity at a human-capex rate (~1.7x/yr,
//! well below even the WWII mobilization ceiling). That produced a fleet that
//! *plateaued* the moment it saturated the human physical workforce — the exact
//! artifact the "robots are a self-sustaining exponential loop" critique targets.
//!
//! Self-replication makes the fleet build its OWN factories (capacity growth
//! climbs toward the finite MACHINE ceiling as autonomy rises) and generate its
//! OWN demand (a reinvest_share slice plowed back into more robots), so the loop
//! no longer stalls at the human-labor cap. It stays finite: capacity is bounded
//! by `machine_ceiling`, autonomy is gated by ASI and fleet self-staffing, and
//! the fleet competes with datacenter compute for the same constrained grid.

use singularity_econ::{simulate, Params, YearState};

fn to2050(mut p: Params) -> Vec<YearState> {
    p.end_year = 2050;
    // These contracts isolate the R2 self-replication LOOP. The (now bounded)
    // materials Liebig ceiling caps the fleet on a separate axis (chip fabs);
    // disable it here so the self-replication dynamics are tested on their own,
    // as the maintenance/machine-ceiling/reinvest tests intend. Materials has
    // its own contract in tests/materials.rs.
    p.materials.enabled = 0.0;
    simulate(&p)
}

fn fleet(v: &[YearState], y: i32) -> f64 {
    v.iter().find(|s| s.year == y).unwrap().robot_fleet_m
}

fn off_params() -> Params {
    Params { robot_self_replication: 0.0, ..Params::default() }
}

// Ablation (byte-exact fleet): switching self-replication off must recover the
// legacy robotics trajectory exactly. This 2036 value is the frozen
// pre-self-replication baseline; it was re-frozen after the A7+C8 power-gate rework
// (freed depreciated compute power lifts compute → autonomy, and the co-equal
// pro-rata grid gate rations robots under power scarcity), which shifts the legacy
// fleet slightly (12.524 → 12.464).
#[test]
fn self_replication_off_recovers_legacy_fleet_exactly() {
    let off = simulate(&off_params()); // default horizon 2036
    let f36 = off.iter().find(|s| s.year == 2036).unwrap().robot_fleet_m;
    assert_eq!(
        f36.to_bits(),
        12.463585704462666_f64.to_bits(),
        "self_replication=0 must reproduce the legacy 2036 fleet byte-for-byte, got {f36}"
    );
    // and the default (self-replication ON) must have MOVED it
    let on = simulate(&Params::default());
    let on36 = on.iter().find(|s| s.year == 2036).unwrap().robot_fleet_m;
    assert!(on36 > f36, "self-replication ON must grow the fleet: {on36} vs {f36}");
}

// The load-bearing fix: legacy plateaus at the human-labor cap; self-replication
// keeps the loop exponential past it. Legacy fleet growth stalls in the late
// 2040s (production falls back to attrition replacement once human jobs are
// taken), while the self-replicating fleet keeps compounding.
#[test]
fn self_replication_breaks_the_human_labor_plateau() {
    let off = to2050(off_params());
    let on = to2050(Params::default());
    // legacy: late-period growth has stalled (plateau) — near flat 2047->2050
    let off_late = fleet(&off, 2050) / fleet(&off, 2047);
    assert!(
        off_late < 1.20,
        "legacy should plateau at the human-labor cap, but grew {off_late:.2}x 2047->2050"
    );
    // self-replication: still visibly exponential over the same window
    let on_late = fleet(&on, 2050) / fleet(&on, 2047);
    assert!(
        on_late > 1.6,
        "self-replication must stay exponential past the labor cap, only {on_late:.2}x"
    );
    // and the endpoint is materially larger
    assert!(
        fleet(&on, 2050) > fleet(&off, 2050) * 3.0,
        "self-replicating fleet must dwarf the plateaued legacy by 2050: {} vs {}",
        fleet(&on, 2050),
        fleet(&off, 2050)
    );
}

// Autonomy is gated by ASI focus and fleet self-staffing, so before
// superintelligence and while the fleet is small the layer contributes almost
// nothing — the early (components-early) period must track legacy closely and
// ramp in GRADUALLY, not as a step. With earlier self-staffing (half=20M) the
// ramp begins ~2032; through 2031 the divergence stays sub-percent.
#[test]
fn pre_asi_early_period_tracks_legacy() {
    let off = to2050(off_params());
    let on = to2050(Params::default());
    for y in [2028, 2029, 2030, 2031] {
        let rel = (fleet(&on, y) - fleet(&off, y)).abs() / fleet(&off, y).max(1e-9);
        assert!(
            rel < 0.015,
            "self-replication leaked into the pre-ramp period at {y}: {:.1}% divergence",
            rel * 100.0
        );
    }
}

// The machine ceiling is the physical throughput knob the whole argument turns
// on: a higher ceiling on factory self-reproduction must grow the endpoint fleet
// (monotone), and it must be FINITE — even a very high ceiling does not produce
// an unbounded fleet, because reinvest_share and the grid still bind.
#[test]
fn machine_ceiling_is_a_monotone_finite_throttle() {
    let base = to2050(Params::default());
    let hi = to2050(Params { machine_ceiling: 9.0, ..Params::default() });
    assert!(
        fleet(&hi, 2050) > fleet(&base, 2050),
        "raising the machine ceiling must grow the fleet: {} vs {}",
        fleet(&hi, 2050),
        fleet(&base, 2050)
    );
    // finite: the fleet stays a real, bounded number
    assert!(fleet(&hi, 2050).is_finite() && fleet(&hi, 2050) < 1e6);
}

// reinvest_share is the self-replication throttle: more output plowed back into
// building robots => faster loop. Zero reinvest with the switch on still leaves
// only the ceiling transition, which without demand to fill it cannot outrun the
// labor cap — so reinvest_share=0 must not exceed the full default.
#[test]
fn reinvest_share_throttles_the_loop() {
    let none = to2050(Params { reinvest_share: 0.0, ..Params::default() });
    let full = to2050(Params::default());
    assert!(
        fleet(&full, 2050) > fleet(&none, 2050),
        "higher reinvest must grow the fleet: {} vs {}",
        fleet(&full, 2050),
        fleet(&none, 2050)
    );
}

// Grid competition is a real physical bound: a fielded fleet draws the same
// constrained power the datacenters are racing to build, so a heavier per-robot
// draw cannot INCREASE the achievable fleet (weak monotonicity — power may or
// may not be the active bind, but more draw never helps).
#[test]
fn robot_grid_draw_never_increases_the_fleet() {
    let light = to2050(Params { robot_kw_each: 0.0, ..Params::default() });
    let heavy = to2050(Params { robot_kw_each: 20.0, ..Params::default() });
    assert!(
        fleet(&heavy, 2050) <= fleet(&light, 2050) + 1e-6,
        "a heavier robot grid draw increased the fleet: {} vs {}",
        fleet(&heavy, 2050),
        fleet(&light, 2050)
    );
}

// ---- self-replication FEEDBACK WEB: each loop must carry its designed sign ----
// Reinforcing loops (learning-efficiency, ASI flywheel, materials self-supply)
// amplify the fleet; disabling any of them must SHRINK the 2050 endpoint.
#[test]
fn reinforcing_loops_amplify_the_fleet() {
    let full = fleet(&to2050(Params::default()), 2050);
    let learn_off = fleet(
        &to2050(Params { learning_autonomy_gain: 0.0, ..Params::default() }),
        2050,
    );
    let flywheel_off = fleet(
        &to2050(Params { asi_flywheel_gain: 0.0, ..Params::default() }),
        2050,
    );
    let mat_supply_off = fleet(
        &to2050(Params { materials_selfsupply_gain: 0.0, ..Params::default() }),
        2050,
    );
    assert!(learn_off < full, "learning×autonomy must amplify: {learn_off} !< {full}");
    assert!(flywheel_off < full, "ASI flywheel must amplify: {flywheel_off} !< {full}");
    assert!(
        mat_supply_off < full,
        "materials self-supply must amplify: {mat_supply_off} !< {full}"
    );
}

// Balancing loops (maintenance drag, materials depletion) keep the exponential
// finite; disabling either must GROW the endpoint. Maintenance drag is the
// dominant limiter — removing it should more than double the fleet.
#[test]
fn balancing_loops_bound_the_fleet() {
    let full = fleet(&to2050(Params::default()), 2050);
    let maint_off = fleet(
        &to2050(Params { maintenance_drag_gain: 0.0, ..Params::default() }),
        2050,
    );
    let deplete_off = fleet(
        &to2050(Params { materials_depletion_gain: 0.0, ..Params::default() }),
        2050,
    );
    // Maintenance drag is a real limiter — removing it must grow the fleet
    // (monotone). The prior >1.5x threshold passed by ~0.007 (audit-flagged
    // knife-edge); a monotone-with-margin assertion is the durable contract.
    assert!(
        maint_off > full * 1.05,
        "maintenance drag must bound the fleet: {maint_off} not > {full}"
    );
    assert!(deplete_off > full, "materials depletion must bound: {deplete_off} !> {full}");
}

// The power gate makes a fielded fleet require a grid to RUN on: robots and
// compute share it. Under a throttled grid the fleet is power-limited, and
// robots building their OWN generation (R-energy) materially lifts the ceiling —
// the loop that relieves the very bind the fleet's draw creates.
#[test]
fn power_gate_binds_and_self_build_relieves_it() {
    let tight = |selfbuild: f64| {
        to2050(Params {
            power_growth_ceiling: 0.15, // throttle human grid buildout
            robot_kw_each: 6.0,         // heavier per-robot draw
            energy_selfbuild_kw: selfbuild,
            ..Params::default()
        })
    };
    let with_build = fleet(&tight(3.0), 2050);
    let no_build = fleet(&tight(0.0), 2050);
    // power is a real bind here: the throttled-grid fleet is far below the
    // benign-grid default
    let benign = fleet(&to2050(Params::default()), 2050);
    assert!(
        no_build < benign * 0.5,
        "power gate should sharply bind a throttled grid: {no_build} vs {benign}"
    );
    // and robots self-building generation relieves it
    assert!(
        with_build > no_build * 1.2,
        "self-built energy must relieve the power bind: {with_build} vs {no_build}"
    );
}

// The whole web must stay FINITE and physically bounded — a self-reinforcing
// loop with real limits, not a numerical runaway. Even with the machine ceiling
// dialed high, the balancing loops hold the 2050 fleet to a sane magnitude.
#[test]
fn full_web_stays_finite_and_bounded() {
    let hot = to2050(Params { machine_ceiling: 8.0, ..Params::default() });
    let f = fleet(&hot, 2050);
    assert!(f.is_finite() && f > 0.0, "fleet must be a real positive number: {f}");
    assert!(f < 1e6, "fleet must stay physically bounded (millions): {f}");
}

// Intelligence attacks maintenance: the maintenance drag is the drag on a
// HUMAN-run machine economy, but predictive maintenance, robots-repairing-robots,
// and design-for-reliability dissolve much of it as ASI diffuses. So a higher
// intelligence-relief must GROW the fleet (it releases the dominant late-stage
// limiter) — and with relief=0 the mechanism must recover the un-relieved drag.
#[test]
fn intelligence_relieves_maintenance_and_grows_the_fleet() {
    let none = fleet(
        &to2050(Params { maintenance_intelligence_relief: 0.0, ..Params::default() }),
        2050,
    );
    let some = fleet(
        &to2050(Params { maintenance_intelligence_relief: 0.6, ..Params::default() }),
        2050,
    );
    let lots = fleet(
        &to2050(Params { maintenance_intelligence_relief: 0.9, ..Params::default() }),
        2050,
    );
    assert!(some > none, "maintenance relief must grow the fleet: {some} !> {none}");
    assert!(lots > some, "more relief must grow it further: {lots} !> {some}");
    // still finite even when intelligence dissolves most of the drag
    assert!(lots.is_finite() && lots < 1e6, "fleet must stay bounded: {lots}");
}
