//! Efficiency-discontinuity / Jevons contract (gap-scan thread #2).
//! The DeepSeek-R1 natural experiment (Jan 27 2025: VST -28%, CEG -21%,
//! NVDA -$589B on a believed efficiency jump) is the power sleeve's true
//! bear case — and it was structurally inexpressible before this.

use singularity_econ::{simulate, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// Neutral Jevons (eps=1.0) reproduces the baseline exactly: efficiency
// gains are filled by demand, power-neutral (the legacy assumption).
#[test]
fn jevons_unity_is_power_neutral() {
    let jump = simulate(&Params {
        efficiency_jump_year: 2029,
        efficiency_jump_size: 10.0,
        jevons_elasticity: 1.0,
        ..Params::default()
    });
    let b = base();
    for (a, j) in b.iter().zip(jump.iter()) {
        assert!(
            (a.power_margin - j.power_margin).abs() < 1e-9,
            "eps=1 must be power-neutral in {}",
            j.year
        );
    }
}

// Frontier-bear (eps<1): a big efficiency jump PERMANENTLY cuts net power
// demand -> power rents collapse. This is the power-sleeve kill scenario
// that had probability zero before the layer existed.
#[test]
fn frontier_efficiency_jump_kills_power_rents() {
    let b = base();
    let bear = simulate(&Params {
        efficiency_jump_year: 2029,
        efficiency_jump_size: 10.0,
        jevons_elasticity: 0.5,
        ..Params::default()
    });
    let pm = |v: &[YearState], y: i32| v.iter().find(|s| s.year == y).unwrap().power_margin;
    assert!(
        pm(&bear, 2032) < pm(&b, 2032) - 0.2,
        "efficiency jump must collapse power rents: {} vs {}",
        pm(&bear, 2032),
        pm(&b, 2032)
    );
    // and it is PERSISTENT, not a one-year dip
    assert!(pm(&bear, 2034) < pm(&b, 2034) - 0.15, "efficiency cut must persist");
}

// Commodity-Jevons (eps>1): demand more than fills efficiency, so a jump
// tightens power further after a transient — the opposite sign, and why
// the sign is the load-bearing uncertainty (SECTION 8 monitorable).
#[test]
fn commodity_jevons_sign_is_opposite() {
    let bear = simulate(&Params {
        efficiency_jump_year: 2029, efficiency_jump_size: 10.0,
        jevons_elasticity: 0.5, ..Params::default()
    });
    let comm = simulate(&Params {
        efficiency_jump_year: 2029, efficiency_jump_size: 10.0,
        jevons_elasticity: 1.25, ..Params::default()
    });
    let pm = |v: &[YearState], y: i32| v.iter().find(|s| s.year == y).unwrap().power_margin;
    assert!(
        pm(&comm, 2032) > pm(&bear, 2032) + 0.2,
        "commodity Jevons must hold power far better than frontier: {} vs {}",
        pm(&comm, 2032),
        pm(&bear, 2032)
    );
}

// No jump = baseline (regression guard).
#[test]
fn no_jump_is_baseline() {
    let none = simulate(&Params { efficiency_jump_year: 0, ..Params::default() });
    for (a, b) in none.iter().zip(base().iter()) {
        assert_eq!(a.power_margin.to_bits(), b.power_margin.to_bits());
    }
}
