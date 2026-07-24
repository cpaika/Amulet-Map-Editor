//! Golden-parity tests: the Rust port must reproduce the Python reference
//! implementation (model_v2.py) on the exported golden vectors.
//!
//! Tolerance is 1e-6 relative: formulas are identical IEEE-754 double
//! arithmetic, but libm transcendentals (exp/ln/powf) may differ in the
//! final ulps between CPython and Rust.

use serde::Deserialize;
use singularity_econ::{simulate, Loops, Params};

#[derive(Deserialize)]
struct GoldenYear {
    year: i32,
    binding: String,
    ai_capex: f64,
    compute_stock: f64,
    silicon_margin: f64,
    power_margin: f64,
    component_margin: f64,
    ip_toll_margin: f64,
    queue_ratio: f64,
    capacity_glut: f64,
    cog_displacement: f64,
    robot_fleet_m: f64,
    robot_cost_k: f64,
    sector_debt: f64,
    credit_multiplier: f64,
    gdp: f64,
    algo_eff: f64,
    profit_silicon: f64,
    profit_ip_tolls: f64,
    profit_electricity: f64,
}

fn golden() -> std::collections::HashMap<String, Vec<GoldenYear>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../output/golden_v2.json");
    let data = std::fs::read_to_string(path).expect("golden_v2.json missing — run: python3 -c 'regenerate goldens' in singularity_economy/");
    serde_json::from_str(&data).unwrap()
}

fn close(a: f64, b: f64, what: &str, year: i32) {
    let tol = 1e-6 * a.abs().max(b.abs()).max(1.0);
    assert!(
        (a - b).abs() <= tol,
        "{what} mismatch at {year}: rust={a} python={b}"
    );
}

fn check_case(name: &str, p: &Params) {
    let gold = golden();
    let expected = gold.get(name).unwrap_or_else(|| panic!("case {name} missing"));
    let states = simulate(p);
    assert_eq!(states.len(), expected.len(), "{name}: length");
    for (s, g) in states.iter().zip(expected) {
        assert_eq!(s.year, g.year);
        let b = format!("{:?}", s.binding).to_lowercase();
        assert_eq!(b, g.binding, "{name}: binding at {}", g.year);
        close(s.ai_capex, g.ai_capex, "ai_capex", g.year);
        close(s.compute_stock, g.compute_stock, "compute_stock", g.year);
        close(s.silicon_margin, g.silicon_margin, "silicon_margin", g.year);
        close(s.power_margin, g.power_margin, "power_margin", g.year);
        close(s.component_margin, g.component_margin, "component_margin", g.year);
        close(s.ip_toll_margin, g.ip_toll_margin, "ip_toll_margin", g.year);
        close(s.queue_ratio, g.queue_ratio, "queue_ratio", g.year);
        close(s.capacity_glut, g.capacity_glut, "capacity_glut", g.year);
        close(s.profits.silicon, g.profit_silicon, "profit_silicon", g.year);
        close(s.profits.ip_tolls, g.profit_ip_tolls, "profit_ip_tolls", g.year);
        close(s.profits.electricity, g.profit_electricity, "profit_electricity", g.year);
        close(s.cog_displacement, g.cog_displacement, "cog_displacement", g.year);
        close(s.robot_fleet_m, g.robot_fleet_m, "robot_fleet_m", g.year);
        close(s.robot_cost_k, g.robot_cost_k, "robot_cost_k", g.year);
        close(s.sector_debt, g.sector_debt, "sector_debt", g.year);
        close(s.credit_multiplier, g.credit_multiplier, "credit_multiplier", g.year);
        close(s.gdp, g.gdp, "gdp", g.year);
        close(s.algo_eff, g.algo_eff, "algo_eff", g.year);
    }
}

#[test]
fn parity_baseline() {
    check_case("baseline", &Params::default());
}

#[test]
fn parity_b1_off() {
    let p = Params {
        loops: Loops { b1_supply_response: 0.0, ..Loops::default() },
        ..Params::default()
    };
    check_case("b1_off", &p);
}

#[test]
fn parity_stress_credit() {
    let p = Params {
        internal_funding_share: 0.25,
        momentum_gain: 1.2,
        singularity_year: 2031,
        debt_revenue_tolerance: 0.8,
        ..Params::default()
    };
    check_case("stress_credit", &p);
}

#[test]
fn parity_capital_bound() {
    // exercises Binding::Capital and Binding::Chips branches (round-2 fix 3)
    let p = Params { power_efficiency_gain: 0.5, ..Params::default() };
    check_case("capital_bound", &p);
}

#[test]
fn parity_power_tight() {
    // exercises the power-utilization cap and starved-recovery path
    let p = Params {
        ai_power_2026: 20.0,
        power_additions_2026: 8.0,
        ..Params::default()
    };
    check_case("power_tight", &p);
}

#[test]
fn parity_fast() {
    let p = Params {
        singularity_boost: 3.0,
        chip_supply_gain: 3.0,
        momentum_gain: 1.0,
        ..Params::default()
    };
    check_case("fast", &p);
}
