//! Demography-layer validation contract
//! (output/history/demography_design.md §5 + tension supplement).

use singularity_econ::{simulate, Loops, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
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
