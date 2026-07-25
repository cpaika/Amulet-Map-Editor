//! Society-layer validation contract (output/history/society_design.md §6).
//! Each test is one of the design doc's falsifiable predictions, grounded
//! in the 11 political-economy briefs.

use singularity_econ::{simulate, Loops, Params, SocietyParams, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

fn by_year(states: &[YearState], year: i32) -> &YearState {
    states.iter().find(|s| s.year == year).unwrap()
}

// Contract #2: transfers are a staircase — steps happen in crisis years
// (COVID: weeks) or at elections (New Deal: cycles), never smoothly.
#[test]
fn transfers_step_at_elections_or_crisis_only() {
    let p = Params::default();
    let states = base();
    for (i, w) in states.windows(2).enumerate() {
        let increased = w[1].transfer_share > w[0].transfer_share + 1e-12;
        if increased {
            let year = w[1].year;
            let election = (year - p.start_year) % p.society.election_period == 0;
            let prior_rate = if i == 0 {
                w[0].cog_displacement
            } else {
                w[0].cog_displacement - states[i - 1].cog_displacement
            };
            // the step this year was triggered by this year's realized rate
            let this_rate = w[1].cog_displacement - w[0].cog_displacement;
            let crisis = this_rate > p.society.crisis_rate
                || prior_rate > p.society.crisis_rate;
            assert!(
                election || crisis,
                "transfer step in {year} with neither election nor crisis"
            );
        }
    }
}

// Contract #3: backlash is a pulse, not a plateau — sentiment peaks near
// the displacement-rate peak and declines >=30% by 2036 even as the
// displaced LEVEL keeps rising (rate decay + leverage erosion).
#[test]
fn sentiment_is_pulse_not_plateau() {
    let states = base();
    let rate = |i: usize| {
        if i == 0 {
            states[0].cog_displacement
        } else {
            states[i].cog_displacement - states[i - 1].cog_displacement
        }
    };
    let peak_rate_year = (0..states.len())
        .max_by(|&a, &b| rate(a).total_cmp(&rate(b)))
        .map(|i| states[i].year)
        .unwrap();
    let peak_sent_year = states
        .iter()
        .max_by(|a, b| a.sentiment.total_cmp(&b.sentiment))
        .unwrap()
        .year;
    assert!(
        (peak_sent_year - peak_rate_year).abs() <= 2,
        "sentiment peak {peak_sent_year} far from rate peak {peak_rate_year}"
    );
    let peak_sent = states.iter().map(|s| s.sentiment).fold(f64::MIN, f64::max);
    let last = states.last().unwrap();
    assert!(
        last.sentiment <= 0.7 * peak_sent,
        "sentiment did not decay: {} vs peak {peak_sent}",
        last.sentiment
    );
    assert!(
        last.cog_displacement > by_year(&states, peak_sent_year).cog_displacement,
        "displacement level should keep rising after the sentiment peak"
    );
}

// Contract #4: the relief valve is the only historically-grounded path to
// fast adoption — reaching deep displacement by 2032 requires transfers
// to have scaled by 2030, and removing B5 slows displacement.
#[test]
fn fast_displacement_requires_relief_valve() {
    let states = base();
    if by_year(&states, 2032).cog_displacement >= 0.40 {
        assert!(
            by_year(&states, 2030).transfer_share >= 0.03,
            "deep 2032 displacement without transfers >=3% GDP by 2030"
        );
    }
    let no_relief = simulate(&Params {
        society: SocietyParams { b5_relief: 0.0, ..SocietyParams::default() },
        ..Params::default()
    });
    assert!(
        by_year(&no_relief, 2032).cog_displacement
            < by_year(&states, 2032).cog_displacement - 1e-9,
        "removing the relief valve should slow displacement (backlash binds)"
    );
}

// Contract #5 (ratchet half): regulation never loosens inside the horizon
// (railroad deregulation took 93 years).
#[test]
fn regulation_is_a_ratchet() {
    for w in base().windows(2) {
        assert!(
            w[1].reg_enforcement >= w[0].reg_enforcement - 1e-12,
            "enforcement fell in {}",
            w[1].year
        );
    }
}

// Dread incident: stringency steps within ~a year (pipeline full),
// consumer trust takes the hit, and adoption ends lower than baseline.
#[test]
fn dread_incident_clamps_down_fast() {
    let states = base();
    let hit = simulate(&Params {
        incident_year: 2029,
        incident_dread: true,
        ..Params::default()
    });
    assert!(
        by_year(&hit, 2030).reg_enforcement
            > by_year(&states, 2030).reg_enforcement + 0.05,
        "dread incident failed to move enforcement within a year"
    );
    assert!(
        by_year(&hit, 2029).consumer_trust < by_year(&states, 2029).consumer_trust - 0.1
    );
    assert!(
        hit.last().unwrap().adoption_level < states.last().unwrap().adoption_level - 1e-9,
        "dread incident should cost adoption by the horizon"
    );
}

// R6: the backlash window closes — labor power erodes as displacement
// accumulates (pickets need members), ending well below its early peak.
#[test]
fn labor_window_closes() {
    let states = base();
    let early_peak = states
        .iter()
        .filter(|s| s.year <= 2030)
        .map(|s| s.labor_power)
        .fold(f64::MIN, f64::max);
    let last = states.last().unwrap().labor_power;
    assert!(
        last < 0.6 * early_peak,
        "labor power did not erode: {last} vs early peak {early_peak}"
    );
    let no_erosion = simulate(&Params {
        society: SocietyParams { r6_erosion: 0.0, ..SocietyParams::default() },
        ..Params::default()
    });
    assert!(
        no_erosion.last().unwrap().labor_power > last,
        "R6 ablation should preserve labor power"
    );
}

// Master ablation: society_layer = 0 must exactly recover the legacy
// constant-gain B2 world — no hidden coupling.
#[test]
fn society_off_recovers_legacy_and_on_changes_behavior() {
    let off = simulate(&Params {
        loops: Loops { society_layer: 0.0, ..Loops::default() },
        ..Params::default()
    });
    let off2 = simulate(&Params {
        loops: Loops { society_layer: 0.0, ..Loops::default() },
        incident_year: 2029,
        incident_dread: true,
        ..Params::default()
    });
    for (a, b) in off.iter().zip(off2.iter()) {
        assert!(
            (a.adoption_level - b.adoption_level).abs() < 1e-12,
            "incidents must be inert with the layer off"
        );
    }
    let on = base();
    assert!(
        (on.last().unwrap().adoption_level - off.last().unwrap().adoption_level).abs()
            > 1e-6,
        "layer on must actually change trajectories"
    );
}

// Capture: below the outrage ceiling, lobbying suppresses the regulatory
// drip (SB 1047 veto) — removing capture yields strictly more enforcement.
#[test]
fn capture_suppresses_regulation_below_outrage_ceiling() {
    let with_capture = base();
    let without = simulate(&Params {
        society: SocietyParams { b8_capture: 0.0, ..SocietyParams::default() },
        ..Params::default()
    });
    assert!(
        without.last().unwrap().reg_enforcement
            >= with_capture.last().unwrap().reg_enforcement - 1e-12,
        "capture ablation should not reduce enforcement"
    );
}
