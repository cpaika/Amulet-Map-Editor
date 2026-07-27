//! Valuation-engine tests and trade-book conclusion locks — port of the
//! retired `test_valuation.py` and `test_scenarios_v2.py`. If a lock fails
//! after a model change, the book's foundations moved: that is a review
//! event, not a test to silence.

use singularity_econ::companies::universe;
use singularity_econ::scenarios::{scenario_params, scenario_states};
use singularity_econ::valuation::{
    earnings_path, evaluate, evaluate_all, implied_cagr, pv, Company,
    EmergingPool, RatioPool, Stance, DISCOUNT_RATE, HORIZON, SCENARIO_PROBS,
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
    let rows = book();
    for t in ["VST", "NRG", "GEV"] {
        assert!(upside(&rows, t) > 0.5, "{t}: {}", upside(&rows, t));
    }
    assert!(upside(&rows, "CEG") > 0.3);
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
