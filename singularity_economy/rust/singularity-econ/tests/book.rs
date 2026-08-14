//! Valuation-engine tests and trade-book conclusion locks — port of the
//! retired `test_valuation.py` and `test_scenarios_v2.py`. If a lock fails
//! after a model change, the book's foundations moved: that is a review
//! event, not a test to silence.

use singularity_econ::companies::universe;
use singularity_econ::scenarios::{
    scenario_params, scenario_states, scenario_states_enhanced, scenario_states_v2,
};
use singularity_econ::valuation::{
    earnings_and_rent, earnings_path, evaluate, evaluate_all, implied_cagr, pv, Company,
    EmergingPool, RatioPool, Stance, CAPTURE_DECAY, DISCOUNT_RATE, HORIZON, SCENARIO_PROBS,
};
use singularity_econ::{simulate, Params};

/// Equity-duration beta threaded into the valuation (audit A3). Matches the
/// `MacroParams::default().dr_beta`, so these locks price on the same discount
/// the shipped `main` uses.
const DR_BETA: f64 = 0.60;

fn test_company(pools: Vec<(RatioPool, f64)>, beta: f64, drift: f64) -> Company {
    Company {
        ticker: "TEST",
        name: "test",
        mcap_b: 100.0,
        ntm_earnings_b: 5.0,
        pools,
        pool_beta: beta,
        share_drift: drift,
        terminal_multiple: 15.0,
        stance: Stance::Watch,
        capture: vec![],
        taiwan_fab_exposure: 0.0,
        notes: "",
    }
}

// ---------------- pv / reverse DCF ----------------

#[test]
fn pv_orders_growth_and_terminal() {
    let flat = vec![10.0; HORIZON];
    let growing: Vec<f64> = (0..HORIZON).map(|i| 10.0 * 1.2_f64.powi(i as i32)).collect();
    assert!(pv(&growing, 15.0, DISCOUNT_RATE) > pv(&flat, 15.0, DISCOUNT_RATE));
    assert!(pv(&flat, 20.0, DISCOUNT_RATE) > pv(&flat, 10.0, DISCOUNT_RATE));
}

#[test]
fn implied_cagr_roundtrip() {
    let g = 0.12_f64;
    let e0 = 10.0;
    let path: Vec<f64> = (0..HORIZON).map(|i| e0 * (1.0 + g).powi(i as i32 + 1)).collect();
    let mcap = pv(&path, 15.0, DISCOUNT_RATE);
    assert!((implied_cagr(mcap, e0, 15.0) - g).abs() < 0.005);
}

#[test]
fn higher_price_implies_higher_growth() {
    assert!(implied_cagr(500.0, 10.0, 15.0) > implied_cagr(200.0, 10.0, 15.0));
}

// ---------------- earnings paths ----------------

#[test]
fn pool_growth_flows_through() {
    let states = simulate(&Params::default());
    let c = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.0);
    let path = earnings_path(&c, &states);
    assert!(path.last().unwrap() > &path[0]);
}

#[test]
fn casualty_pool_shrinks_earnings() {
    let states = simulate(&Params::default());
    let c = test_company(vec![(RatioPool::Bpo, 1.0)], 1.0, 0.0);
    let path = earnings_path(&c, &states);
    assert!(path.last().unwrap() < &path[0]);
}

#[test]
fn beta_amplifies_and_drift_compounds() {
    let states = simulate(&Params::default());
    let lo = test_company(vec![(RatioPool::Silicon, 1.0)], 0.5, 0.0);
    let hi = test_company(vec![(RatioPool::Silicon, 1.0)], 1.5, 0.0);
    assert!(earnings_path(&hi, &states).last().unwrap()
            > earnings_path(&lo, &states).last().unwrap());
    let gain = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.05);
    let lose = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, -0.05);
    assert!(earnings_path(&gain, &states).last().unwrap()
            > earnings_path(&lose, &states).last().unwrap());
}

#[test]
fn capture_adds_emerging_pool_earnings() {
    let states = simulate(&Params::default());
    let mut with = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    with.capture = vec![(EmergingPool::AiServices, 0.05)];
    let without = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    assert!(earnings_path(&with, &states).last().unwrap()
            > earnings_path(&without, &states).last().unwrap());
}

// Re-audit C2: the terminal value must apply a RENT-DOMINANCE haircut. A capture
// name's near-peak final-year rent is NOT a perpetuity (the scarcity share erodes),
// so its terminal slice is capitalized at multiple x dr/(dr+decay), strictly below
// the full multiple — while a name with NO capture is untouched (rent path all zero).
#[test]
fn terminal_rent_haircut_derates_only_capture_names() {
    let states = simulate(&Params::default());
    let dr = DISCOUNT_RATE;
    // Non-capture name: rent path is identically zero, so the mechanism is a no-op.
    let no_cap = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.0);
    let (_pn, rent_n) = earnings_and_rent(&no_cap, &states);
    assert!(rent_n.iter().all(|r| *r == 0.0), "no-capture name must carry zero rent");
    // Capture name: positive terminal rent, so the haircut PV must sit strictly below
    // the full-multiple capitalization of the identical earnings path.
    let mut cap = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    cap.capture = vec![(EmergingPool::Electricity, 0.02)];
    let (path, rent) = earnings_and_rent(&cap, &states);
    let last_rent = *rent.last().unwrap();
    assert!(last_rent > 0.0, "capture name must carry terminal rent");
    let full = pv(&path, cap.terminal_multiple, dr);
    let flow: f64 = path.iter().enumerate()
        .map(|(i, e)| e / (1.0 + dr).powi(i as i32 + 1)).sum();
    let base_last = path.last().unwrap() - last_rent;
    let rent_mult = cap.terminal_multiple * dr / (dr + CAPTURE_DECAY);
    let haircut = flow
        + (base_last * cap.terminal_multiple + last_rent * rent_mult)
            / (1.0 + dr).powi(path.len() as i32);
    assert!(rent_mult < cap.terminal_multiple, "rent multiple must be below the full multiple");
    assert!(haircut < full, "rent haircut must reduce a capture name's PV: {haircut} vs {full}");
}

// ---------------- scenario evaluation ----------------

#[test]
fn probs_sum_to_one_and_scenarios_present() {
    let total: f64 = SCENARIO_PROBS.iter().map(|(_, p)| p).sum();
    assert!((total - 1.0).abs() < 1e-9);
    let names: Vec<&str> = scenario_params().iter().map(|(n, _)| *n).collect();
    for (n, _) in SCENARIO_PROBS {
        assert!(names.contains(&n), "missing scenario {n}");
    }
}

#[test]
fn fizzle_is_worst_for_beneficiaries() {
    let states = scenario_states();
    let c = test_company(vec![(RatioPool::Silicon, 1.0)], 1.1, 0.0);
    let e = evaluate(&c, &states, DR_BETA);
    let worst = e.per_scenario.iter()
        .min_by(|a, b| a.upside.partial_cmp(&b.upside).unwrap())
        .unwrap();
    assert_eq!(worst.scenario, "fizzle");
}

#[test]
fn evaluate_all_ranks_by_expected_upside() {
    let states = scenario_states();
    let rows = evaluate_all(&universe(), &states, DR_BETA);
    for w in rows.windows(2) {
        assert!(w[0].expected_upside >= w[1].expected_upside);
    }
}

// ---------------- trade-book conclusion locks ----------------

fn book() -> Vec<(String, f64)> {
    let states = scenario_states();
    evaluate_all(&universe(), &states, DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect()
}

fn upside(rows: &[(String, f64)], ticker: &str) -> f64 {
    rows.iter().find(|(t, _)| t == ticker)
        .unwrap_or_else(|| panic!("{ticker} missing"))
        .1
}

#[test]
fn core_semi_longs_positive() {
    let rows = book();
    for t in ["TSM", "AVGO", "NVDA"] {
        assert!(upside(&rows, t) > 0.5, "{t}: {}", upside(&rows, t));
    }
}

#[test]
fn power_complex_positive() {
    // Thresholds recalibrated after re-audit #26/#28: the capture-rent terminal is now
    // capped at its self-consistent decaying-perpetuity value (1/(dr+decay)) and the
    // terminal discounts at the terminal-year rate, so power GENERATION (whose upside
    // was mostly capitalized rent) prices conservative — positive, no longer >50%.
    // Grid EQUIPMENT (GEV), which earns flows not rent, stays a strong long.
    let rows = book();
    assert!(upside(&rows, "GEV") > 1.0, "GEV: {}", upside(&rows, "GEV"));
    assert!(upside(&rows, "VST") > 0.4, "VST: {}", upside(&rows, "VST"));
    assert!(upside(&rows, "NRG") > 0.2, "NRG: {}", upside(&rows, "NRG"));
    assert!(upside(&rows, "CEG") > 0.0, "CEG: {}", upside(&rows, "CEG"));
}

#[test]
fn wage_linked_shorts_negative() {
    let rows = book();
    for t in ["RHI", "ADP", "PAYX", "MAN", "CHRW", "LSTR", "TCS.NS"] {
        assert!(upside(&rows, t) < -0.3, "{t}: {}", upside(&rows, t));
    }
}

#[test]
fn robotics_longs_split_component_vs_integrator() {
    // The robot-scaling crunch favors the COMPONENT bottleneck (6268.T Nabtesco —
    // precision reducers) over the integrator (SYM) and upstream materials (MP).
    // After the wf_f3a19c42 audit corrected the materials Liebig ceiling (which
    // had been unbounded, inflating the fleet ~5x) and conserved the reinvestment
    // budget, the fleet is materials-gated and the component upside COMPRESSED:
    // 6268.T is now the best-positioned of the three (positive) but no longer
    // clears the 20% conviction hurdle on the corrected trajectory. The split —
    // component beats integrator/materials — is the durable finding.
    let rows = book();
    let nab = upside(&rows, "6268.T");
    assert!(nab > 0.0, "component supplier should stay positive: {nab}");
    for t in ["SYM", "MP"] {
        let u = upside(&rows, t);
        assert!(u < nab, "{t} ({u}) must trail the component supplier ({nab})");
    }
}

// Enhanced-realism enable set (gated_dynamics_menu.md): turning the reflexive
// AI-capex spine + wealth effect + transmission lag + wage compression + JG tilt on
// TOGETHER must re-rank the book, not just perturb it. The load-bearing effect is the
// Minsky bust: the silicon/AI-capex complex (NVDA, AVGO, TSM, BESI) must lose upside
// vs the shipped baseline (fatter left tail), while the wage-linked shorts stay short.
// This is the review lens, not the shipped default — it must run without pathology.
#[test]
fn enhanced_realism_reranks_the_book() {
    let base = book();
    let enh: Vec<(String, f64)> = evaluate_all(&universe(), &scenario_states_enhanced(), DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect();
    // All finite, and the ranking actually MOVED (not a no-op enable set).
    assert!(enh.iter().all(|(_, u)| u.is_finite()), "enhanced book must stay finite");
    let base_top = &base[0].0;
    let enh_top = &enh[0].0;
    let moved = enh.iter().any(|(t, u)| (u - upside(&base, t)).abs() > 0.05);
    assert!(moved, "enhanced enable set must re-rank the book, base_top={base_top} enh_top={enh_top}");
    // The reflexive-bust left tail: the AI-capex complex must lose upside vs baseline.
    for t in ["NVDA", "AVGO", "TSM", "BESI.AS"] {
        assert!(
            upside(&enh, t) < upside(&base, t) - 0.10,
            "{t} must lose upside under the reflexive bust: enh {} vs base {}",
            upside(&enh, t),
            upside(&base, t)
        );
    }
    // The wage-linked shorts remain short (wage compression can only deepen them).
    for t in ["RHI", "ADP", "PAYX"] {
        assert!(upside(&enh, t) < -0.3, "{t} must stay short under enhanced realism: {}", upside(&enh, t));
    }
}

// First-principles v2: the most complete first-principles configuration (enhanced set +
// Wright learning + q-governor + two-sided power + AI commoditization). It must reprice
// the book coherently: (a) all finite; (b) the AI-services RENT names (MSFT, GOOGL) lose
// upside vs baseline as commoditization competes the provider surplus away; (c) the
// physical-bottleneck longs (POWL grid gear, robot components) still lead positive;
// (d) the wage-linked shorts stay short. This locks the composed lens against drift.
#[test]
fn first_principles_v2_reprices_coherently() {
    let base = book();
    let v2: Vec<(String, f64)> = evaluate_all(&universe(), &scenario_states_v2(), DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect();
    assert!(v2.iter().all(|(_, u)| u.is_finite()), "v2 book must stay finite");
    // AI-services rent commoditizes → MSFT/GOOGL lose upside vs baseline.
    for t in ["MSFT", "GOOGL"] {
        assert!(
            upside(&v2, t) < upside(&base, t) - 0.10,
            "{t} AI rent must commoditize in v2: {} vs {}",
            upside(&v2, t),
            upside(&base, t)
        );
    }
    // The physical bottleneck still leads. With the re-audited q-governor (operating-
    // return numerator, which brakes the compute buildout harder and earlier), the
    // robot-component chokepoint — less capex-reflexive than compute-derived grid demand
    // — leads, with grid switchgear (POWL) the strongest grid name and GEV a clear long.
    for t in ["002472.SZ", "POWL"] {
        assert!(upside(&v2, t) > 0.5, "{t} must stay a strong long in v2: {}", upside(&v2, t));
    }
    assert!(upside(&v2, "GEV") > 0.3, "GEV must stay a clear long in v2: {}", upside(&v2, "GEV"));
    // Wage-linked shorts remain short.
    for t in ["RHI", "ADP", "PAYX"] {
        assert!(upside(&v2, t) < -0.3, "{t} must stay short in v2: {}", upside(&v2, t));
    }
}

// Red-team round 3: geopolitics must be REACHABLE from the book — the
// taiwan_shock scenario has to price real damage into Taiwan-exposed
// names (TSM's shock-scenario fair value below its baseline fair value),
// and it must carry nonzero probability weight.
#[test]
fn taiwan_shock_is_priced() {
    use singularity_econ::valuation::SCENARIO_PROBS;
    let w = SCENARIO_PROBS
        .iter()
        .find(|(n, _)| *n == "taiwan_shock")
        .expect("taiwan_shock missing from scenario probabilities")
        .1;
    assert!(w >= 0.05, "taiwan tail weight collapsed: {w}");
    let states = singularity_econ::scenarios::scenario_states();
    let companies = singularity_econ::companies::universe();
    let tsm = companies.iter().find(|c| c.ticker == "TSM").unwrap();
    let ev = singularity_econ::valuation::evaluate(tsm, &states, DR_BETA);
    let base = ev.per_scenario.iter().find(|s| s.scenario == "baseline").unwrap();
    let shock = ev
        .per_scenario
        .iter()
        .find(|s| s.scenario == "taiwan_shock")
        .unwrap();
    // Taiwan shock must price REAL TSM damage. Threshold relaxed from 0.10 to
    // 0.03 after the batch-1 chip-cap-latch fix: post-invasion fabs now recover
    // as ASI rebuilds them (the latch was permanently pinning growth at 18%), so
    // an invasion is materially less PERMANENTLY damaging — the shock is still
    // priced (~5% haircut), just no longer catastrophic-forever.
    assert!(
        shock.upside < base.upside - 0.03,
        "TSM shows no Taiwan damage: shock {} vs base {}",
        shock.upside,
        base.upside
    );
}
