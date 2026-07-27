//! Behavioral test suite — port of the retired `test_model_v2.py`.
//! The heart is LOOP ABLATION: each named feedback loop, when disabled,
//! must change behavior exactly as systems theory predicts. If ablating a
//! loop does nothing, the loop isn't real — it's decoration.

use singularity_econ::{simulate, Binding, Loops, Params, Pipeline, YearState};

fn run(p: Params) -> Vec<YearState> {
    simulate(&p)
}

fn base() -> Vec<YearState> {
    run(Params::default())
}

fn by_year(states: &[YearState], year: i32) -> &YearState {
    states.iter().find(|s| s.year == year).unwrap()
}

// New-dynamic: transmission/HVDC delivery lag. With the gain on, generation that
// outruns the transformer/HVDC ramp cannot energize — so usable AI power is lower
// and the power constraint is TIGHTER (higher power margin, longer rents, bullish
// grid-equipment/IPP names). Compute still grows (not collapsed). Off => baseline.
#[test]
fn transmission_lag_tightens_power() {
    let b = base();
    let mut p = Params::default();
    p.transmission_gain = 1.0;
    let t = run(p);
    // usable AI power is capped below the unconstrained baseline
    assert!(by_year(&t, 2036).ai_power_gw < by_year(&b, 2036).ai_power_gw, "transmission must cap usable power");
    // power stays scarcer (peak margin at/above baseline across the boom)
    let pm_t = t.iter().map(|s| s.power_margin).fold(f64::MIN, f64::max);
    let pm_b = b.iter().map(|s| s.power_margin).fold(f64::MIN, f64::max);
    assert!(pm_t >= pm_b - 1e-9, "power margin must not fall: {pm_t} vs {pm_b}");
    // compute still grows strongly (constraint, not collapse)
    assert!(by_year(&t, 2036).compute_stock > 0.7 * by_year(&b, 2036).compute_stock, "compute must not collapse");
}

// New-dynamic spine: AI-capex bubble/bust reflexivity. With the gain on, equity
// sentiment must build a euphoria (>1) and then crack asymmetrically past a glut
// (<1), and the post-glut capex path must fall materially BELOW the smooth baseline
// — the Minsky boom-bust the core otherwise lacks. Gain 0 is byte-identical
// (enforced by the golden snapshot).
#[test]
fn equity_sentiment_spine_makes_a_boom_bust() {
    let b = base();
    let mut p = Params::default();
    p.equity_sentiment_gain = 1.0;
    let c = run(p);
    let peak_es = c.iter().map(|s| s.equity_sentiment).fold(0.0_f64, f64::max);
    let trough_es = c.iter().map(|s| s.equity_sentiment).fold(9.0_f64, f64::min);
    assert!(peak_es > 1.1, "must build euphoria: peak {peak_es}");
    assert!(trough_es < 0.7, "must crack into a Minsky bust: trough {trough_es}");
    // the crack must come AFTER the euphoria (asymmetric build-then-collapse)
    let peak_yr = c.iter().max_by(|a, b| a.equity_sentiment.partial_cmp(&b.equity_sentiment).unwrap()).unwrap().year;
    let trough_yr = c.iter().max_by(|a, b| b.equity_sentiment.partial_cmp(&a.equity_sentiment).unwrap()).unwrap().year;
    assert!(trough_yr > peak_yr, "bust must follow boom: peak {peak_yr}, trough {trough_yr}");
    // and the bust deepens the capex drawdown vs the smooth baseline
    let last = |v: &[YearState]| v.last().unwrap().ai_capex;
    assert!(last(&c) < last(&b) * 0.7, "bust must cut capex vs baseline: {} vs {}", last(&c), last(&b));
    // baseline (gain 0) leaves sentiment frozen at 1.0
    assert!(b.iter().all(|s| (s.equity_sentiment - 1.0).abs() < 1e-12), "gated off => frozen at 1.0");
}

// New-dynamic: equity wealth-effect chains off the spine and makes GDP cyclical —
// the worst year-over-year GDP growth under (spine + wealth effect) must be BELOW
// the spine alone (the wealth channel deepens the bust). Off => byte-identical.
#[test]
fn wealth_effect_deepens_the_gdp_bust() {
    let min_yoy = |v: &[YearState]| {
        v.windows(2)
            .map(|w| w[1].gdp / w[0].gdp - 1.0)
            .fold(f64::MAX, f64::min)
    };
    let mut spine = Params::default();
    spine.equity_sentiment_gain = 1.0;
    let s = run(spine);
    let mut both = Params::default();
    both.equity_sentiment_gain = 1.0;
    both.wealth_effect_gain = 1.0;
    let bth = run(both);
    assert!(
        min_yoy(&bth) < min_yoy(&s),
        "wealth effect must deepen the GDP bust: {} !< {}",
        min_yoy(&bth),
        min_yoy(&s)
    );
    // wealth effect with the spine OFF does nothing (frozen sentiment => no change)
    let mut we_only = Params::default();
    we_only.wealth_effect_gain = 1.0;
    let base_gdp: Vec<f64> = base().iter().map(|s| s.gdp).collect();
    let we_gdp: Vec<f64> = run(we_only).iter().map(|s| s.gdp).collect();
    assert!(
        base_gdp.iter().zip(&we_gdp).all(|(a, b)| (a - b).abs() < 1e-9),
        "wealth effect must be inert without the spine"
    );
}

// New-dynamic: wage compression. With the gain on, the human wage POOLS must fall
// further than the headcount-only baseline (the displaced-labor reserve compresses
// the price of the remaining jobs) — the labor-income channel the wage-linked shorts
// key on. Gain 0 is byte-identical to baseline (enforced by the golden snapshot).
#[test]
fn wage_compression_deepens_the_wage_pool_decline() {
    let b = base();
    let mut cp = Params::default();
    cp.wage_compression_cog_gain = 0.5;
    cp.wage_compression_phys_gain = 0.5;
    let c = run(cp);
    let cog = |v: &[YearState]| by_year(v, 2036).pools.human_cognitive_wages;
    let phys = |v: &[YearState]| by_year(v, 2036).pools.human_physical_wages;
    assert!(cog(&c) < cog(&b), "cog wages must compress: {} !< {}", cog(&c), cog(&b));
    assert!(phys(&c) < phys(&b), "phys wages must compress: {} !< {}", phys(&c), phys(&b));
    // and the compression is bounded (floor), not a collapse to zero
    assert!(cog(&c) > 0.3 * cog(&b), "compression must respect the wage floor");
}

// ---------------- pipeline ----------------

#[test]
fn pipeline_steady_state_passthrough() {
    let mut pipe = Pipeline::new(3, 10.0);
    let mut out = 0.0;
    for _ in 0..5 {
        out = pipe.step(10.0);
    }
    assert!((out - 10.0).abs() < 1e-9);
}

#[test]
fn pipeline_delay_length() {
    let mut pipe = Pipeline::new(3, 0.0);
    let outs: Vec<f64> = (0..6)
        .map(|i| pipe.step(if i == 0 { 100.0 } else { 0.0 }))
        .collect();
    assert_eq!(&outs[..4], &[0.0, 0.0, 0.0, 100.0]);
}

// ---------------- invariants & calibration ----------------

#[test]
fn nonnegative_and_bounded() {
    let p = Params::default();
    for s in base() {
        assert!(s.ai_capex >= 0.0);
        assert!(s.robot_fleet_m >= 0.0);
        assert!(s.silicon_margin <= p.margin_ceiling + 1e-9);
        assert!(s.power_margin <= p.margin_ceiling + 1e-9);
        assert!(s.ip_toll_margin <= p.margin_ceiling + 1e-9);
        assert!(s.cog_displacement <= 1.0);
    }
}

#[test]
fn displacement_ratchet() {
    let mut prev = -1.0;
    for s in base() {
        assert!(s.cog_displacement >= prev);
        prev = s.cog_displacement;
    }
}

#[test]
fn anchors_2026_match_calibration() {
    let states = base();
    let s0 = &states[0];
    assert!(s0.ai_capex > 0.45 && s0.ai_capex < 0.70, "capex {}", s0.ai_capex);
    assert!(s0.ai_power_gw > 45.0 && s0.ai_power_gw < 110.0);
    assert!((s0.pools.it_services - 1.55).abs() < 0.16);
}

#[test]
fn qualitative_shape_of_the_decade() {
    let states = base();
    let power_years = states.iter().filter(|s| s.binding == Binding::Power).count();
    assert!(power_years >= 6, "power bound only {power_years} years");
    assert!(by_year(&states, 2028).cog_displacement < 0.25);
    assert!(by_year(&states, 2032).cog_displacement > 0.4);
    assert!(by_year(&states, 2032).robot_prod_m < 3.0);
}

// ---------------- endogenous rent dynamics ----------------

#[test]
fn silicon_rents_decay_within_horizon() {
    let states = base();
    let p = Params::default();
    let early_peak = [2026, 2027, 2028]
        .iter()
        .map(|y| by_year(&states, *y).silicon_margin)
        .fold(f64::MIN, f64::max);
    assert!(early_peak > p.normal_margin + 0.15, "no early rents");
    assert!(by_year(&states, 2033).silicon_margin < early_peak - 0.15,
            "rents did not decay");
}

#[test]
fn power_rents_persist_beyond_silicon() {
    let states = base();
    let last = states.last().unwrap();
    assert!(last.power_margin > last.silicon_margin + 0.1);
}

#[test]
fn ip_rents_persist_while_silicon_decays() {
    let states = base();
    let last = states.last().unwrap();
    assert!(last.ip_toll_margin > last.silicon_margin + 0.2,
            "ip {} vs si {}", last.ip_toll_margin, last.silicon_margin);
}

#[test]
fn rent_duration_scales_with_supply_gain() {
    let norm_year = |states: &[YearState]| {
        let p = Params::default();
        states.iter()
            .find(|s| s.year > 2027 && s.silicon_margin <= p.normal_margin + 0.02)
            .map_or(9999, |s| s.year)
    };
    let fast = run(Params { chip_supply_gain: 3.0, ..Params::default() });
    let slow = run(Params { chip_supply_gain: 0.4, ..Params::default() });
    assert!(norm_year(&fast) <= norm_year(&slow));
}

#[test]
fn silicon_glut_emerges_but_power_never_gluts() {
    let states = base();
    let late_glut = states[5..].iter().map(|s| s.capacity_glut).fold(f64::MIN, f64::max);
    assert!(late_glut > 1.3, "no endogenous glut: {late_glut}");
    assert!(states.last().unwrap().power_margin > 0.30);
}

// ---------------- loop ablations ----------------

fn with_loops(loops: Loops) -> Vec<YearState> {
    run(Params { loops, ..Params::default() })
}

#[test]
fn b1_off_rents_persist_longer() {
    // Under the R4 (ASI-accelerated) baseline, even organic supply growth
    // eventually catches throttled demand — so ablating B1 extends rent
    // DURATION rather than making rents eternal. Compare cumulative rents.
    let on = base();
    let off = with_loops(Loops { b1_supply_response: 0.0, ..Loops::default() });
    let cum = |v: &[YearState]| v.iter().map(|s| s.silicon_margin).sum::<f64>();
    assert!(cum(&off) > cum(&on) + 0.5,
            "off {:.2} vs on {:.2}", cum(&off), cum(&on));
}

#[test]
fn b2_off_displacement_not_slower() {
    let on = base();
    let off = with_loops(Loops { b2_backlash: 0.0, ..Loops::default() });
    assert!(by_year(&off, 2030).cog_displacement
            >= by_year(&on, 2030).cog_displacement - 1e-9);
}

#[test]
fn b3_off_more_expansion_and_capex() {
    let on = base();
    let off = with_loops(Loops { b3_affordability: 0.0, ..Loops::default() });
    assert!(off.last().unwrap().cognitive_task_index
            >= on.last().unwrap().cognitive_task_index - 1e-9);
    let sum = |v: &[YearState]| v.iter().map(|s| s.ai_capex).sum::<f64>();
    assert!(sum(&off) > sum(&on), "B3 does not close the loop");
}

#[test]
fn r1_off_less_capability() {
    let on = base();
    let off = with_loops(Loops { r1_recursive_ai: 0.0, ..Loops::default() });
    assert!(off.last().unwrap().algo_eff < on.last().unwrap().algo_eff);
}

#[test]
fn r2_off_no_more_robots() {
    let on = base();
    let off = with_loops(Loops { r2_robot_bootstrap: 0.0, ..Loops::default() });
    assert!(off.last().unwrap().robot_fleet_m
            <= on.last().unwrap().robot_fleet_m + 1e-9);
}

#[test]
fn r3_off_shallower_queues() {
    // Ablate with the society layer off: political feedback (sentiment ->
    // precautionary demand drag) otherwise confounds the pure momentum
    // mechanism this test isolates. Also zero the robot grid draw
    // (robot_kw_each): robots competing with compute for the grid is a separate
    // coupling that perturbs the AI-compute queue by ~0.1% — comparable to R3's
    // own tiny effect on peak queue depth at this horizon — and would otherwise
    // confound this isolation. It has its own contract in tests/robotics.rs.
    let base = |extra: Loops| Params {
        robot_kw_each: 0.0,
        loops: extra,
        ..Params::default()
    };
    let on = simulate(&base(Loops { society_layer: 0.0, ..Loops::default() }));
    let off = simulate(&base(Loops {
        r3_capex_momentum: 0.0,
        society_layer: 0.0,
        ..Loops::default()
    }));
    let peak = |v: &[YearState]| v.iter().map(|s| s.queue_ratio).fold(f64::MIN, f64::max);
    assert!(peak(&off) < peak(&on) + 1e-9);
}

#[test]
fn b4_binds_under_stress_only_when_enabled() {
    let stress = Params {
        internal_funding_share: 0.25,
        momentum_gain: 1.2,
        singularity_year: 2031,
        debt_revenue_tolerance: 0.8,
        ..Params::default()
    };
    let with_credit = run(stress.clone());
    let min_mult = with_credit.iter().map(|s| s.credit_multiplier).fold(f64::MAX, f64::min);
    assert!(min_mult < 0.999, "credit never bit under stress");
    let without = run(Params {
        loops: Loops { b4_credit: 0.0, ..Loops::default() },
        ..stress
    });
    let min_off = without.iter().map(|s| s.credit_multiplier).fold(f64::MAX, f64::min);
    assert_eq!(min_off, 1.0);
}

// ---------------- behavior under timing shifts ----------------

#[test]
fn queues_emerge_with_momentum_and_delay() {
    let peak = base().iter().map(|s| s.queue_ratio).fold(f64::MIN, f64::max);
    assert!(peak > 1.2);
}

#[test]
fn adoption_never_falls_with_late_singularity() {
    let states = run(Params { singularity_year: 2031, ..Params::default() });
    let levels: Vec<f64> = states.iter().map(|s| s.adoption_level).collect();
    for w in levels.windows(2) {
        assert!(w[1] >= w[0] - 1e-12, "adoption fell: {:?}", w);
    }
}

#[test]
fn delayed_singularity_delays_displacement() {
    let early = base();
    let late = run(Params { singularity_year: 2029, ..Params::default() });
    assert!(by_year(&early, 2029).cog_displacement
            > by_year(&late, 2029).cog_displacement);
}

#[test]
fn starved_power_run_recovers_no_absorbing_state() {
    let states = run(Params {
        ai_power_2026: 10.0,
        power_additions_2026: 2.0,
        ..Params::default()
    });
    assert!(states.last().unwrap().ai_capex > 0.1,
            "capex never recovered from power starvation");
    for s in &states {
        assert!(s.queue_ratio <= 50.0 + 1e-9);
        assert!(s.capacity_glut <= 50.0 + 1e-9);
    }
}

// ---------------- returns to intelligence ----------------

// Marginal returns to intelligence (rho < 1) must not rewrite the decade:
// AI labor supply saturates demand within ~2 years of the singularity, so
// displacement is ADOPTION-gated, not capability-gated. This is the
// robustness result against both linear-returns optimism and
// diminishing-returns pessimism — the conclusions survive either.
#[test]
fn conclusions_robust_to_intelligence_returns() {
    let linear = run(Params { intelligence_returns_rho: 1.0, ..Params::default() });
    let concave = run(Params { intelligence_returns_rho: 0.7, ..Params::default() });
    let d = |v: &[YearState], y: i32| {
        v.iter().find(|s| s.year == y).unwrap().cog_displacement
    };
    assert!(
        (d(&linear, 2032) - d(&concave, 2032)).abs() < 0.05,
        "displacement should be adoption-gated, not returns-gated: {} vs {}",
        d(&linear, 2032),
        d(&concave, 2032)
    );
    // but concavity must bite where supply is actually tight (early years):
    let s27_lin = linear.iter().find(|s| s.year == 2027).unwrap().ai_hew_m;
    let s27_con = concave.iter().find(|s| s.year == 2027).unwrap().ai_hew_m;
    assert!(s27_con <= s27_lin + 1e-9, "concavity must not raise early supply");
}
