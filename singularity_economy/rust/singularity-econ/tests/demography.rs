//! Demography-layer validation contract
//! (output/history/demography_design.md §5 + tension supplement).

use singularity_econ::demography::{DemographyParams, DemographyState};
use singularity_econ::{simulate, Loops, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// B2: Youth Channel B (the underemployed-graduate "revolutionary" stock) must be
// LIVE at the default underemploy_drain — a peak youth_sentiment_inflow > 0 over a
// displacement ramp — whereas the old 0.6 drain kept it structurally dead (blocked
// share never crossed the 0.18 protest threshold).
#[test]
fn youth_channel_is_live_at_default_drain() {
    fn peak_inflow(drain: f64) -> f64 {
        let mut dp = DemographyParams::default();
        dp.underemploy_drain = drain;
        let mut st = DemographyState::new(&dp, 1200.0, 1800.0);
        let mut peak = 0.0_f64;
        for i in 0..14 {
            let disp = (0.05 + 0.06 * i as f64).min(0.76);
            let out = st.step(&dp, disp, 0.1, 0.2, 0.05, 0.05, false, 0.1, 0.0, 1.0);
            peak = peak.max(out.youth_sentiment_inflow);
        }
        peak
    }
    assert!(
        peak_inflow(DemographyParams::default().underemploy_drain) > 0.0,
        "youth channel must fire at the default drain"
    );
    assert_eq!(peak_inflow(0.6), 0.0, "the old 0.6 drain kept the channel dead");
}

// C13: the bio layer's age_creep_mult (healthcare-deflation fiscal relief) must
// actually reach demography — it was computed then discarded. Stepping the layer
// with more relief (a lower mult) must leave a HIGHER transfer_cap_eff than the
// no-relief (mult = 1.0) path, i.e. the wire is live.
#[test]
fn age_creep_relief_reaches_the_transfer_cap() {
    fn cap_after(mult: f64) -> f64 {
        let dp = DemographyParams::default();
        let mut st = DemographyState::new(&dp, 100.0, 100.0);
        let mut out = st.step(&dp, 0.3, 0.1, 0.2, 0.05, 0.05, false, 0.1, 0.0, mult);
        for _ in 0..15 {
            out = st.step(&dp, 0.3, 0.1, 0.2, 0.05, 0.05, false, 0.1, 0.0, mult);
        }
        out.transfer_cap_eff
    }
    let no_relief = cap_after(1.0);
    let relief = cap_after(0.85);
    assert!(
        relief > no_relief,
        "healthcare-deflation relief must raise the transfer cap: {relief} !> {no_relief}"
    );
}

fn demo_off() -> Vec<YearState> {
    simulate(&Params {
        loops: Loops { d_demography: 0.0, ..Loops::default() },
        ..Params::default()
    })
}

fn by_year(v: &[YearState], y: i32) -> &YearState {
    v.iter().find(|s| s.year == y).unwrap()
}

// Contract #1 (youth-first sequencing): in the takeoff year the visible
// rate is still zero while graduates are already piling into the blocked
// stock — the earliest political signal is Channel B, not layoffs.
#[test]
fn youth_first_sequencing() {
    let b = base();
    let s27 = by_year(&b, 2027);
    assert!(
        s27.visible_disp_rate < 0.01,
        "2027 visible rate {} — freezes should absorb the first wave",
        s27.visible_disp_rate
    );
    assert!(
        s27.blocked_entrants_m > by_year(&b, 2026).blocked_entrants_m,
        "blocked entrants must be loading while layoffs stay at zero"
    );
}

// Contract #2 (restriction-first): the migration valve closes before
// transfers reach deep scale — openness must fall well below its start
// while the transfer ramp is still climbing.
#[test]
fn restriction_fires_before_transfer_cap() {
    let b = base();
    let first_restriction = b
        .iter()
        .find(|s| s.migration_openness < 0.30)
        .map(|s| s.year)
        .expect("restriction never fired");
    let cap_year = b
        .iter()
        .find(|s| s.transfer_share > 0.12)
        .map(|s| s.year)
        .unwrap_or(9999);
    assert!(
        first_restriction <= cap_year,
        "restriction {first_restriction} must not lag deep transfers {cap_year}"
    );
}

// Contract #7 (pool arithmetic): the physical pool grows ~+19M/yr net —
// the static 2400M spec was the material error.
#[test]
fn physical_pool_grows_cognitive_stays_flat() {
    let b = base();
    let last = b.last().unwrap();
    assert!(
        (2500.0..=2750.0).contains(&last.phys_pool_m),
        "phys pool 2036 = {} outside UN-projection band",
        last.phys_pool_m
    );
    assert!(
        (850.0..=1050.0).contains(&last.cog_pool_m),
        "cognitive pool should stay near-static: {}",
        last.cog_pool_m
    );
}

// Contract #6 (later-but-sharper, weak form): the freeze filter must not
// make politics FIRE EARLIER than the unfiltered legacy.
#[test]
fn filter_never_accelerates_politics() {
    let on = base();
    let off = demo_off();
    let first_hot = |v: &[YearState]| {
        v.iter().find(|s| s.sentiment > 0.5).map_or(9999, |s| s.year)
    };
    assert!(
        first_hot(&on) >= first_hot(&off),
        "signal filter made politics fire earlier: {} vs {}",
        first_hot(&on),
        first_hot(&off)
    );
}

// Care pull: the aging-core care gap adds robot demand; ablating the
// coupling must not increase the fleet.
#[test]
fn care_gap_pulls_robot_demand() {
    let mut p = Params::default();
    p.demography.care_pull_gain = 0.0;
    let no_pull = simulate(&p);
    let b = base();
    assert!(
        no_pull.last().unwrap().robot_fleet_m <= b.last().unwrap().robot_fleet_m + 1e-9,
        "removing the care pull should not grow the fleet"
    );
}

// Tension redirect: scapegoating routes backlash away from AI firms —
// a high-tension world must not have LOWER adoption than baseline, while
// carrying more unrest risk (conserved routed flow, not new suppression).
#[test]
fn tension_redirect_is_pro_adoption_and_corrosive() {
    let mut p = Params::default();
    p.demography.tension_gain = 5.0; // force a high-tension world
    p.demography.redirect_share = 0.5;
    let tense = simulate(&p);
    let b = base();
    assert!(
        tense.last().unwrap().adoption_level >= b.last().unwrap().adoption_level - 0.02,
        "redirection must not suppress adoption below baseline"
    );
    let max_tension = tense.iter().map(|s| s.tension).fold(f64::MIN, f64::max);
    assert!(max_tension > b.iter().map(|s| s.tension).fold(f64::MIN, f64::max));
}

// Master ablation: d_demography = 0 recovers static pools and raw-rate
// politics exactly (the layer must be a pure overlay).
#[test]
fn demography_off_recovers_static_legacy() {
    let off = demo_off();
    for s in &off {
        assert_eq!(s.cog_pool_m.to_bits(), 950.0f64.to_bits());
        assert_eq!(s.phys_pool_m.to_bits(), 2400.0f64.to_bits());
    }
    let on = base();
    assert!(
        (on.last().unwrap().transfer_share - off.last().unwrap().transfer_share).abs()
            > 1e-6,
        "layer on must change the political trajectory"
    );
}

// Red-team round (demography): the tension/scapegoating channel must be a
// LIVE stock, not pinned near its floor — a confirmed finding was that it
// decayed to ~0.04 all decade, making redirect ~3% vs the designed 55-75%.
#[test]
fn tension_is_a_live_channel_not_floor_pinned() {
    let states = base();
    let peak = states.iter().map(|s| s.tension).fold(f64::MIN, f64::max);
    assert!(
        peak > 0.5,
        "tension never activated (peak {peak}) — scapegoating channel is inert"
    );
    // and it decays after the sentiment pulse (acute self-excitation)
    let last = states.last().unwrap().tension;
    assert!(last < peak - 0.2, "tension never decayed from its peak");
}

// The care-robot pull must convert the care gap to robot UNITS (not the
// /1000 bug that made it numerically dead), and must respect the Japan
// eldercare-effectiveness gate (~0 substitution before 2031).
#[test]
fn care_pull_zero_pre_2031_then_positive() {
    // Run to 2050: robots are component-constrained until the 2040s, so
    // the care-demand pull (which raises fleet TARGET) only translates to
    // realized production once components stop binding — components-early.
    // Isolate the care-pull → fleet mechanism from the materials cap: when the
    // fleet is materials-gated (chip-fab-bound), extra DEMAND (care pull) does
    // not raise realized production, so disable the Liebig ceiling here to test
    // the demography coupling itself.
    let mut p = Params::default();
    p.demography.care_pull_gain = 2.0;
    p.end_year = 2050;
    p.materials.enabled = 0.0;
    let with_pull = simulate(&p);
    let mut p0 = Params::default();
    p0.demography.care_pull_gain = 0.0;
    p0.end_year = 2050;
    p0.materials.enabled = 0.0;
    let no_pull = simulate(&p0);
    // pre-2031 the effectiveness gate is 0: identical fleets
    let f30 = |v: &[YearState]| v.iter().find(|s| s.year == 2030).unwrap().robot_fleet_m;
    assert!((f30(&with_pull) - f30(&no_pull)).abs() < 1e-6, "care pull leaked pre-2031");
    // by 2050 the pull adds measurable fleet (not the dead /1000 coupling)
    let f50 = |v: &[YearState]| v.iter().find(|s| s.year == 2050).unwrap().robot_fleet_m;
    assert!(
        f50(&with_pull) > f50(&no_pull) + 1.0,
        "care pull still numerically dead post-2031: {} vs {}",
        f50(&with_pull), f50(&no_pull)
    );
}
