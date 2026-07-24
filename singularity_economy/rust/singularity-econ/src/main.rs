//! CLI: run the baseline, check golden parity, or run large Monte Carlo.
//!
//!   singularity-econ run                    # baseline summary (JSON)
//!   singularity-econ mc <n> <seed>          # Monte Carlo distributions (JSON)

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use singularity_econ::{simulate, Params};
use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") | None => run_baseline(),
        Some("mc") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(7);
            monte_carlo(n, seed);
        }
        Some(cmd) => {
            eprintln!("unknown command: {cmd} (use: run | mc <n> <seed>)");
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
    let mut overshoot_peak: Vec<f64> = Vec::new();
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
        if states.iter().any(|s| s.credit_multiplier < 0.98) {
            credit_crunch += 1;
        }
        overshoot_peak.push(states.iter().map(|s| s.overshoot_ratio)
            .fold(f64::MIN, f64::max));
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

    for v in [&mut silicon_norm_year, &mut power_norm_year, &mut overshoot_peak,
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
        "overshoot_peak": dist(&overshoot_peak),
        "cog_displacement_2032": dist(&disp_2032),
        "robot_prod_2032_m": dist(&robot_2032),
        "min_gdp_growth": dist(&min_gdp_growth),
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
