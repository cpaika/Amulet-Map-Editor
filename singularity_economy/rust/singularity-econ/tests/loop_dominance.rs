//! Loop-dominance lock (Sep-26 re-analysis, Wave 1). Knocks out each feedback
//! loop in turn and checks the measured signature: three loops carry the model,
//! the rest are second-order. A later change that silently revives or kills a
//! loop fails here — that is a review event: re-measure, then update the lock.
//!
//! Measured on the default baseline (knockout vs full model):
//!   B1 supply response        2029-36 capex −42%, robot output −68%
//!   R4 physical acceleration  2029-36 capex −26%, robot output −62%
//!   R1 recursive AI           2026-28 displacement −39%
//!   every other loop          within ±5% on both
//! R2 (robot bootstrap) and R5 (launch learning) are inert in the baseline
//! (<1% on robot output): documented, not asserted either way beyond ±5%.

use singularity_econ::{simulate, Loops, Params, YearState};

fn capex_2029_36(s: &[YearState]) -> f64 {
    s.iter().filter(|x| x.year >= 2029).map(|x| x.ai_capex).sum()
}

fn displacement_2026_28(s: &[YearState]) -> f64 {
    s.iter().filter(|x| x.year <= 2028).map(|x| x.cog_displacement).sum()
}

fn knockout(set: fn(&mut Loops)) -> (f64, f64) {
    let base = simulate(&Params::default());
    let mut loops = Loops::default();
    set(&mut loops);
    let ko = simulate(&Params { loops, ..Params::default() });
    (
        capex_2029_36(&ko) / capex_2029_36(&base) - 1.0,
        displacement_2026_28(&ko) / displacement_2026_28(&base) - 1.0,
    )
}

#[test]
fn supply_response_b1_carries_late_capex() {
    let (capex, _) = knockout(|l| l.b1_supply_response = 0.0);
    assert!(capex < -0.20, "B1 knockout moves 2029-36 capex only {capex:+.3}");
}

#[test]
fn physical_acceleration_r4_carries_late_capex() {
    let (capex, _) = knockout(|l| l.r4_physical_acceleration = 0.0);
    assert!(capex < -0.20, "R4 knockout moves 2029-36 capex only {capex:+.3}");
}

#[test]
fn recursive_ai_r1_carries_early_displacement() {
    let (_, disp) = knockout(|l| l.r1_recursive_ai = 0.0);
    assert!(disp < -0.30, "R1 knockout moves 2026-28 displacement only {disp:+.3}");
}

#[test]
fn the_other_loops_are_second_order() {
    let others: [(&str, fn(&mut Loops)); 9] = [
        ("B2 backlash", |l| l.b2_backlash = 0.0),
        ("B3 affordability", |l| l.b3_affordability = 0.0),
        ("R2 robot bootstrap", |l| l.r2_robot_bootstrap = 0.0),
        ("R3 capex momentum", |l| l.r3_capex_momentum = 0.0),
        ("B4 credit", |l| l.b4_credit = 0.0),
        ("society layer", |l| l.society_layer = 0.0),
        ("demography layer", |l| l.d_demography = 0.0),
        ("R5 launch learning", |l| l.r5_launch_learning = 0.0),
        ("B11 endogenous rates", |l| l.b11_endogenous_rates = 0.0),
    ];
    for (name, set) in others {
        let (capex, disp) = knockout(set);
        assert!(capex.abs() <= 0.05 && disp.abs() <= 0.05,
                "{name} knockout: capex {capex:+.3}, displacement {disp:+.3} — no longer second-order");
    }
}
