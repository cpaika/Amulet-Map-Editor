//! CLI: run the baseline, check golden parity, or run large Monte Carlo.
//!
//!   singularity-econ run                    # baseline summary (JSON)
//!   singularity-econ mc <n> <seed>          # Monte Carlo distributions (JSON)

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use singularity_econ::companies::{load_financials, universe};
use singularity_econ::scenarios::{
    scenario_states, scenario_states_enhanced, scenario_states_v2,
};
use singularity_econ::valuation::{evaluate_all, Stance};
use singularity_econ::{simulate, Params};
use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") | None => run_baseline(),
        Some("golden") => {
            let default = concat!(env!("CARGO_MANIFEST_DIR"),
                                  "/../../output/golden_v2.json");
            golden(args.get(2).map(String::as_str).unwrap_or(default));
        }
        Some("book") => {
            book(args.get(2).map(String::as_str), "baseline");
        }
        Some("book-enhanced") => {
            book(args.get(2).map(String::as_str), "enhanced");
        }
        Some("book-v2") => {
            book(args.get(2).map(String::as_str), "v2");
        }
        Some("sa") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(11);
            sensitivity(n, seed);
        }
        Some("mc") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(7);
            monte_carlo(n, seed);
        }
        Some(cmd) => {
            eprintln!("unknown command: {cmd} (use: run | book | book-enhanced | book-v2 [financials.json] | golden [path] | mc <n> <seed> | sa <n> <seed>)");
            std::process::exit(2);
        }
    }
}

fn run_baseline() {
    let states = simulate(&Params::default());
    let summary: Vec<_> = states.iter().map(|s| {
        serde_json::json!({
            "year": s.year, "binding": s.binding, "ai_capex": s.ai_capex,
            "silicon_margin": s.silicon_margin, "power_margin": s.power_margin,
            "component_margin": s.component_margin,
            "cog_displacement": s.cog_displacement,
            "robot_prod_m": s.robot_prod_m, "gdp": s.gdp,
        })
    }).collect();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

/// Regenerate the regression snapshot. Run ONLY on intentional behavior
/// changes; a diff in this file in review = the model changed.
fn golden(path: &str) {
    let cases: Vec<(&str, Params)> = vec![
        ("baseline", Params::default()),
        ("b1_off", Params {
            loops: singularity_econ::Loops {
                b1_supply_response: 0.0,
                ..singularity_econ::Loops::default()
            },
            ..Params::default()
        }),
        ("stress_credit", Params {
            internal_funding_share: 0.25, momentum_gain: 1.2,
            singularity_year: 2031, debt_revenue_tolerance: 0.8,
            ..Params::default()
        }),
        ("fast", Params {
            singularity_boost: 3.0, chip_supply_gain: 3.0, momentum_gain: 1.0,
            ..Params::default()
        }),
        ("capital_bound", Params {
            power_efficiency_gain: 0.5, ..Params::default()
        }),
        ("power_tight", Params {
            ai_power_2026: 20.0, power_additions_2026: 8.0, ..Params::default()
        }),
    ];
    let mut out = serde_json::Map::new();
    for (name, p) in cases {
        let rows: Vec<_> = simulate(&p).iter().map(|s| serde_json::json!({
            "year": s.year,
            "binding": format!("{:?}", s.binding).to_lowercase(),
            "ai_capex": s.ai_capex, "compute_stock": s.compute_stock,
            "silicon_margin": s.silicon_margin, "power_margin": s.power_margin,
            "component_margin": s.component_margin,
            "ip_toll_margin": s.ip_toll_margin,
            "queue_ratio": s.queue_ratio, "capacity_glut": s.capacity_glut,
            "cog_displacement": s.cog_displacement,
            "robot_fleet_m": s.robot_fleet_m, "robot_cost_k": s.robot_cost_k,
            "sector_debt": s.sector_debt, "credit_multiplier": s.credit_multiplier,
            "gdp": s.gdp, "algo_eff": s.algo_eff,
            "profit_silicon": s.profits.silicon,
            "profit_ip_tolls": s.profits.ip_tolls,
            "profit_electricity": s.profits.electricity,
        })).collect();
        out.insert(name.to_string(), serde_json::Value::Array(rows));
    }
    std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("snapshot written to {path}");
}

fn book(financials_path: Option<&str>, mode: &str) {
    let mut comps = universe();
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../output/financials.json");
    let path = financials_path.unwrap_or(default);
    match std::fs::read_to_string(path) {
        Ok(json) => load_financials(&mut comps, &json),
        Err(e) if financials_path.is_some() => {
            panic!("could not read financials {path}: {e}");
        }
        Err(_) => eprintln!("note: {path} not found; using built-in (synced) financials"),
    }
    let states = match mode {
        "enhanced" => {
            eprintln!("note: enhanced-realism set (spine+wealth+transmission+wage-compression+JG)");
            scenario_states_enhanced()
        }
        "v2" => {
            eprintln!("note: first-principles v2 (enhanced + Wright learning + q-governor + two-sided power + AI commoditization)");
            scenario_states_v2()
        }
        _ => scenario_states(),
    };
    let dr_beta = singularity_econ::macrofin::MacroParams::default().dr_beta;
    let rows = evaluate_all(&comps, &states, dr_beta);
    println!("{:<10} {:<6} {:>12} {:>8} {:>8} {:>8}",
             "ticker", "side", "impliedCAGR", "E[up]", "worst", "best");
    for r in &rows {
        let side = match r.stance {
            Stance::Long => "long",
            Stance::Short => "short",
            Stance::Watch => "watch",
        };
        println!("{:<10} {:<6} {:>11.1}% {:>7.1}% {:>7.1}% {:>7.1}%",
                 r.ticker, side, r.implied_cagr * 100.0,
                 r.expected_upside * 100.0,
                 r.worst_scenario_upside * 100.0,
                 r.best_scenario_upside * 100.0);
    }
}

struct Draw {
    rng: ChaCha8Rng,
}

impl Draw {
    fn new(seed: u64) -> Self {
        Draw { rng: ChaCha8Rng::seed_from_u64(seed) }
    }
    fn uniform(&mut self, lo: f64, hi: f64) -> f64 {
        Uniform::new(lo, hi).sample(&mut self.rng)
    }
    fn choice_weighted(&mut self, items: &[(i32, f64)]) -> i32 {
        let total: f64 = items.iter().map(|(_, w)| w).sum();
        let mut x = self.uniform(0.0, total);
        for &(v, w) in items {
            if x < w {
                return v;
            }
            x -= w;
        }
        items.last().unwrap().0
    }

    fn params(&mut self) -> Params {
        let sing = self.choice_weighted(&[(2027, 0.40), (2028, 0.30),
                                          (2029, 0.20), (2030, 0.10)]);
        let robot_gap = self.choice_weighted(&[(1, 0.67), (2, 0.33)]);
        Params {
            singularity_year: sing,
            robotics_year: sing + robot_gap,
            singularity_boost: self.uniform(1.3, 3.0),
            algo_eff_growth_pre: self.uniform(2.0, 3.2),
            algo_eff_growth_post: self.uniform(1.3, 1.8),
            adoption_halflife: self.uniform(1.0, 3.5),
            max_displacement_rate: self.uniform(0.10, 0.30),
            addressable_cognitive: self.uniform(0.6, 0.95),
            cognitive_demand_elasticity: self.uniform(1.1, 1.7),
            chip_base_growth: self.uniform(0.20, 0.45),
            chip_supply_gain: self.uniform(0.6, 3.0),
            chip_growth_ceiling: self.uniform(0.5, 1.0),
            power_base_growth: self.uniform(0.04, 0.15),
            power_supply_gain: self.uniform(0.2, 1.2),
            power_growth_ceiling: self.uniform(0.2, 0.55),
            capex_gdp_cap: self.uniform(0.035, 0.09),
            component_base_growth: self.uniform(0.25, 0.7),
            component_supply_gain: self.uniform(1.0, 3.5),
            component_growth_ceiling: self.uniform(0.8, 2.0),
            bootstrap_gain: self.uniform(0.03, 0.2),
            robot_learning_rate: self.uniform(0.15, 0.30),
            robot_cost_2028_k: self.uniform(35.0, 90.0),
            it_services_beta: self.uniform(0.6, 1.1),
            bpo_beta: self.uniform(0.9, 1.5),
            saas_beta: self.uniform(0.45, 1.0),
            prof_info_beta: self.uniform(0.2, 0.6),
            power_efficiency_gain: self.uniform(0.08, 0.18),
            transition_drag: self.uniform(0.2, 1.3),
            productivity_passthrough: self.uniform(0.2, 0.5),
            internal_funding_share: self.uniform(0.4, 0.8),
            momentum_gain: self.uniform(0.2, 1.2),
            backlash_gain: self.uniform(0.5, 4.0),
            afford_gain: self.uniform(0.3, 1.5),
            asi_diffusion_years: self.uniform(1.0, 4.0),
            asi_delay_compression: self.uniform(0.15, 0.55),
            asi_ceiling_boost: self.uniform(0.2, 0.9),
            asi_integration_relief: self.uniform(0.2, 0.8),
            // geopolitical shock path: escalation-ladder sampler (S1-S8),
            // seeded from this run's RNG so paths stay reproducible
            geo_shocks: {
                let mut grng = singularity_econ::GeoRng::new(
                    (self.uniform(0.0, 1.0) * u64::MAX as f64) as u64,
                );
                singularity_econ::geopolitics::sample_shocks(&mut grng, 2026, 2036)
            },
            // AI incident hazard ~10%/yr rising with deployment; dread
            // conditional p=0.25 (society design §3)
            incident_year: {
                let mut y = 0;
                for year in 2027..=2035 {
                    if self.uniform(0.0, 1.0) < 0.10 + 0.02 * (year - 2027) as f64 {
                        y = year;
                        break;
                    }
                }
                y
            },
            incident_dread: self.uniform(0.0, 1.0) < 0.25,
            // efficiency discontinuity: ~1.2 jumps/yr expected for >=3x
            // commodity events; sample one >=10x jump per path with a
            // tier-mixed Jevons elasticity (commodity ~1.25 / frontier ~0.5)
            efficiency_jump_year: {
                let mut y = 0;
                for year in 2027..=2034 {
                    if self.uniform(0.0, 1.0) < 0.12 { y = year; break; }
                }
                y
            },
            efficiency_jump_size: 3.0 + self.uniform(0.0, 1.0) * 12.0,
            jevons_elasticity: if self.uniform(0.0, 1.0) < 0.5 {
                self.uniform(0.85, 1.85) // commodity tier
            } else {
                self.uniform(0.30, 0.70) // frontier tier (bear)
            },
            ..Params::default()
        }
    }
}

fn pct(sorted: &[f64], q: f64) -> f64 {
    let idx = ((q * sorted.len() as f64) as usize).min(sorted.len() - 1);
    sorted[idx]
}

fn monte_carlo(n: usize, seed: u64) {
    let mut draw = Draw::new(seed);
    let base = Params::default();
    let years: Vec<i32> = (base.start_year..=base.end_year).collect();

    let mut binding_counts: BTreeMap<i32, BTreeMap<String, usize>> = BTreeMap::new();
    let mut silicon_norm_year: Vec<f64> = Vec::new();
    let mut power_norm_year: Vec<f64> = Vec::new();
    let mut credit_crunch = 0usize;
    let mut credit_tighten = 0usize;
    let mut queue_peak: Vec<f64> = Vec::new();
    let mut glut_2036: Vec<f64> = Vec::new();
    let mut disp_2032: Vec<f64> = Vec::new();
    let mut robot_2032: Vec<f64> = Vec::new();
    let mut min_gdp_growth: Vec<f64> = Vec::new();

    for _ in 0..n {
        let p = draw.params();
        let states = simulate(&p);
        for s in &states {
            let name = format!("{:?}", s.binding).to_lowercase();
            *binding_counts.entry(s.year).or_default().entry(name).or_default() += 1;
        }
        let norm = |sel: fn(&singularity_econ::YearState) -> f64| -> f64 {
            for s in &states {
                if s.year > p.singularity_year && sel(s) <= p.normal_margin + 0.02 {
                    return s.year as f64;
                }
            }
            9999.0
        };
        silicon_norm_year.push(norm(|s| s.silicon_margin));
        power_norm_year.push(norm(|s| s.power_margin));
        // Two credit metrics: TIGHTENING (>=10% haircut — common now that
        // geopolitical spreads and sovereign crowding stack) vs CRUNCH
        // (>=20% haircut: the levered-periphery accident, historically
        // railway-calls/fiber-debt class).
        if states.iter().any(|s| s.credit_multiplier < 0.90) {
            credit_tighten += 1;
        }
        if states.iter().any(|s| s.credit_multiplier < 0.80) {
            credit_crunch += 1;
        }
        queue_peak.push(states.iter().map(|s| s.queue_ratio)
            .fold(f64::MIN, f64::max));
        glut_2036.push(states.last().map_or(0.0, |s| s.capacity_glut));
        for s in &states {
            if s.year == 2032 {
                disp_2032.push(s.cog_displacement);
                robot_2032.push(s.robot_prod_m);
            }
        }
        let ming = states.windows(2)
            .map(|w| w[1].gdp / w[0].gdp - 1.0)
            .fold(f64::MAX, f64::min);
        min_gdp_growth.push(ming);
    }

    for v in [&mut silicon_norm_year, &mut power_norm_year, &mut queue_peak,
              &mut glut_2036,
              &mut disp_2032, &mut robot_2032, &mut min_gdp_growth] {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let dist = |v: &[f64]| serde_json::json!({
        "p10": pct(v, 0.10), "p50": pct(v, 0.50), "p90": pct(v, 0.90)
    });

    let binding_freq: BTreeMap<i32, BTreeMap<String, f64>> = years.iter().map(|y| {
        let counts = binding_counts.get(y).cloned().unwrap_or_default();
        (*y, counts.into_iter()
            .map(|(k, c)| (k, c as f64 / n as f64)).collect())
    }).collect();

    let out = serde_json::json!({
        "n_runs": n,
        "seed": seed,
        "binding_frequency": binding_freq,
        "silicon_rent_normalization_year": dist(&silicon_norm_year),
        "power_rent_normalization_year": dist(&power_norm_year),
        "credit_crunch_frequency": credit_crunch as f64 / n as f64,
        "credit_tightening_frequency": credit_tighten as f64 / n as f64,
        "queue_peak": dist(&queue_peak),
        "capacity_glut_2036": dist(&glut_2036),
        "cog_displacement_2032": dist(&disp_2032),
        "robot_prod_2032_m": dist(&robot_2032),
        "min_gdp_growth": dist(&min_gdp_growth),
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

// ---------------------------------------------------------------------------
// Sensitivity analysis: Spearman rank correlation of each sampled parameter
// against the investable outputs, over a large Monte Carlo sample.
// ---------------------------------------------------------------------------

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    for (rank, &i) in idx.iter().enumerate() {
        r[i] = rank as f64;
    }
    r
}

fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let (ra, rb) = (ranks(a), ranks(b));
    let n = ra.len() as f64;
    let ma = ra.iter().sum::<f64>() / n;
    let mb = rb.iter().sum::<f64>() / n;
    let mut cov = 0.0;
    let mut va = 0.0;
    let mut vb = 0.0;
    for i in 0..ra.len() {
        let da = ra[i] - ma;
        let db = rb[i] - mb;
        cov += da * db;
        va += da * da;
        vb += db * db;
    }
    cov / (va.sqrt() * vb.sqrt()).max(1e-12)
}

pub fn sensitivity(n: usize, seed: u64) {
    let mut draw = Draw::new(seed);
    let param_names = [
        "singularity_year", "singularity_boost", "adoption_halflife",
        "max_displacement_rate", "chip_supply_gain", "chip_base_growth",
        "power_supply_gain", "power_base_growth", "power_growth_ceiling",
        "capex_gdp_cap", "component_supply_gain", "robot_cost_2028_k",
        "robot_learning_rate", "internal_funding_share", "momentum_gain",
        "backlash_gain", "transition_drag",
    ];
    let mut param_vals: Vec<Vec<f64>> = vec![Vec::new(); param_names.len()];
    let mut out_silicon_norm: Vec<f64> = Vec::new();
    let mut out_power_margin_32: Vec<f64> = Vec::new();
    let mut out_credit_min: Vec<f64> = Vec::new();
    let mut out_disp_32: Vec<f64> = Vec::new();
    let mut out_robot_32: Vec<f64> = Vec::new();
    let mut out_capex_32: Vec<f64> = Vec::new();

    for _ in 0..n {
        let p = draw.params();
        let vals = [
            p.singularity_year as f64, p.singularity_boost, p.adoption_halflife,
            p.max_displacement_rate, p.chip_supply_gain, p.chip_base_growth,
            p.power_supply_gain, p.power_base_growth, p.power_growth_ceiling,
            p.capex_gdp_cap, p.component_supply_gain, p.robot_cost_2028_k,
            p.robot_learning_rate, p.internal_funding_share, p.momentum_gain,
            p.backlash_gain, p.transition_drag,
        ];
        for (i, v) in vals.iter().enumerate() {
            param_vals[i].push(*v);
        }
        let states = simulate(&p);
        let norm = states.iter()
            .find(|s| s.year > p.singularity_year
                  && s.silicon_margin <= p.normal_margin + 0.02)
            .map_or(2040.0, |s| s.year as f64);
        out_silicon_norm.push(norm);
        for s in &states {
            if s.year == 2032 {
                out_power_margin_32.push(s.power_margin);
                out_disp_32.push(s.cog_displacement);
                out_robot_32.push(s.robot_prod_m);
                out_capex_32.push(s.ai_capex);
            }
        }
        out_credit_min.push(states.iter()
            .map(|s| s.credit_multiplier).fold(f64::MAX, f64::min));
    }

    let outputs: [(&str, &Vec<f64>); 6] = [
        ("silicon_rent_normalization_year", &out_silicon_norm),
        ("power_margin_2032", &out_power_margin_32),
        ("min_credit_multiplier", &out_credit_min),
        ("cog_displacement_2032", &out_disp_32),
        ("robot_prod_2032", &out_robot_32),
        ("ai_capex_2032", &out_capex_32),
    ];

    let mut report = serde_json::Map::new();
    for (oname, ovals) in outputs {
        let mut rows: Vec<(String, f64)> = param_names.iter().enumerate()
            .map(|(i, pn)| (pn.to_string(), spearman(&param_vals[i], ovals)))
            .collect();
        rows.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
        let top: Vec<_> = rows.iter().take(6)
            .map(|(k, v)| serde_json::json!({"param": k, "rho": (v * 1000.0).round() / 1000.0}))
            .collect();
        report.insert(oname.to_string(), serde_json::Value::Array(top));
    }
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
