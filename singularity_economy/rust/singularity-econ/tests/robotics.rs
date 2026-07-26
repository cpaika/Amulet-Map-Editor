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
    simulate(&p)
}

fn fleet(v: &[YearState], y: i32) -> f64 {
    v.iter().find(|s| s.year == y).unwrap().robot_fleet_m
}

fn off_params() -> Params {
    Params { robot_self_replication: 0.0, ..Params::default() }
}

// Ablation (byte-exact fleet): switching self-replication off must recover the
// legacy robotics trajectory exactly. Robotics reads only capacity and demand —
// never the power balance — so the always-on robot grid draw cannot perturb it.
// This 2036 value is the frozen pre-self-replication baseline.
#[test]
fn self_replication_off_recovers_legacy_fleet_exactly() {
    let off = simulate(&off_params()); // default horizon 2036
    let f36 = off.iter().find(|s| s.year == 2036).unwrap().robot_fleet_m;
    assert_eq!(
        f36.to_bits(),
        12.524033295701301_f64.to_bits(),
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
// nothing — the early (components-early) period must track legacy closely.
#[test]
fn pre_asi_early_period_tracks_legacy() {
    let off = to2050(off_params());
    let on = to2050(Params::default());
    for y in [2029, 2031, 2033] {
        let rel = (fleet(&on, y) - fleet(&off, y)).abs() / fleet(&off, y).max(1e-9);
        assert!(
            rel < 0.03,
            "self-replication leaked into the pre-ASI period at {y}: {:.1}% divergence",
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
    let hi = to2050(Params { machine_ceiling: 6.0, ..Params::default() });
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
