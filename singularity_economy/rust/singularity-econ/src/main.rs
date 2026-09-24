//! CLI: run the baseline, check golden parity, or run large Monte Carlo.
//!
//!   singularity-econ run                    # baseline summary (JSON)
//!   singularity-econ mc <n> <seed>          # Monte Carlo distributions (JSON)
//!   (book commands take `--legacy` for the pre-Sep-26 valuation conventions)
//!   singularity-econ book-mc <n> <seed> [base|enhanced|v2|mix] [financials.json]
//!                                           # every name valued on every MC path
//!   singularity-econ book-sa <n> <seed> [lens] [financials.json]
//!                                           # per-name drivers (Spearman) over that prior

use singularity_econ::companies::{load_financials, universe};
use singularity_econ::sampler::{sampled_values, BookSampler, Lens, Sampler};
use singularity_econ::scenarios::{
    scenario_states, scenario_states_enhanced, scenario_states_v2,
};
use singularity_econ::valuation::{
    base_year_consistent, evaluate_all_with, value_on_path, PathContext, Stance,
    ValuationParams,
};
use singularity_econ::{simulate, Params};
use std::collections::BTreeMap;

fn main() {
    // Books value on first-principles conventions by default (F1: firm capacity cap,
    // steady-state flow terminal, power-margin routing); `--legacy` anywhere restores
    // the pre-Sep-26 valuation (the library default, which the book locks pin).
    let raw: Vec<String> = std::env::args().collect();
    let vp = if raw.iter().any(|a| a == "--legacy") {
        ValuationParams::default()
    } else {
        ValuationParams::first_principles()
    };
    let args: Vec<String> = raw.into_iter().filter(|a| a != "--legacy").collect();
    match args.get(1).map(String::as_str) {
        Some("run") | None => run_baseline(),
        Some("golden") => {
            let default = concat!(env!("CARGO_MANIFEST_DIR"),
                                  "/../../output/golden_v2.json");
            golden(args.get(2).map(String::as_str).unwrap_or(default));
        }
        Some("book") => {
            book(args.get(2).map(String::as_str), "baseline", &vp);
        }
        Some("book-enhanced") => {
            book(args.get(2).map(String::as_str), "enhanced", &vp);
        }
        Some("book-v2") => {
            book(args.get(2).map(String::as_str), "v2", &vp);
        }
        Some("book-mc") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(7);
            let lens = args.get(4).map_or(Some(Lens::Mix), |s| Lens::parse(s))
                .unwrap_or_else(|| {
                    eprintln!("lens must be one of: base | enhanced | v2 | mix");
                    std::process::exit(2);
                });
            book_mc(n, seed, lens, args.get(5).map(String::as_str), &vp);
        }
        Some("book-sa") => {
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(4_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(11);
            let lens = args.get(4).map_or(Some(Lens::Mix), |s| Lens::parse(s))
                .unwrap_or_else(|| {
                    eprintln!("lens must be one of: base | enhanced | v2 | mix");
                    std::process::exit(2);
                });
            book_sa(n, seed, lens, args.get(5).map(String::as_str), 4, &vp);
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
            eprintln!("unknown command: {cmd} (use: run | book | book-enhanced | book-v2 [financials.json] | book-mc <n> <seed> [lens] [financials.json] | book-sa <n> <seed> [lens] [financials.json] | golden [path] | mc <n> <seed> | sa <n> <seed>)");
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

fn load_universe(financials_path: Option<&str>) -> Vec<singularity_econ::valuation::Company> {
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
    comps
}

fn side_label(s: Stance) -> &'static str {
    match s {
        Stance::Long => "long",
        Stance::Short => "short",
        Stance::Watch => "watch",
    }
}

fn book(financials_path: Option<&str>, mode: &str, vp: &ValuationParams) {
    let comps = load_universe(financials_path);
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
    let rows = evaluate_all_with(&comps, &states, dr_beta, vp);
    println!("{:<10} {:<6} {:>12} {:>8} {:>8} {:>8} {:>6}",
             "ticker", "side", "impliedCAGR", "E[up]", "worst", "best", "term%");
    for r in &rows {
        println!("{:<10} {:<6} {:>11.1}% {:>7.1}% {:>7.1}% {:>7.1}% {:>5.0}%",
                 r.ticker, side_label(r.stance), r.implied_cagr * 100.0,
                 r.expected_upside * 100.0,
                 r.worst_scenario_upside * 100.0,
                 r.best_scenario_upside * 100.0,
                 r.terminal_share * 100.0);
    }
}

#[derive(Default)]
struct Rejections {
    total: usize,
    by_reason: BTreeMap<String, usize>,
}

impl Rejections {
    fn summary(&self, accepted: usize) -> String {
        let reasons: Vec<String> =
            self.by_reason.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        format!("2026 base-year rejection: {:.1}% of draws ({})",
                100.0 * self.total as f64 / (accepted + self.total) as f64,
                reasons.join(", "))
    }
}

/// ABC conditioning on the observed 2026: redraw until the path's base year agrees
/// with what is already known (chips bind, mapped pools near their anchors).
fn conditioned_draw(
    draws: &mut BookSampler,
    rej: &mut Rejections,
    n: usize,
) -> (singularity_econ::sampler::BookDraw, Vec<singularity_econ::YearState>) {
    loop {
        let d = draws.draw();
        let states = simulate(&d.params);
        match base_year_consistent(&states) {
            Ok(()) => return (d, states),
            Err(why) => {
                rej.total += 1;
                let key = why.split_whitespace().take(2).collect::<Vec<_>>().join(" ");
                *rej.by_reason.entry(key).or_default() += 1;
                assert!(rej.total < 50 * n + 1000, "base-year rejection runaway");
            }
        }
    }
}

/// Book sensitivity: for every name, the Spearman rank correlation of its upside
/// with every sampled parameter (plus the fizzle flag, the severe-Taiwan flag and,
/// under `mix`, the lens fraction) over the conditioned book-mc prior. Prints the
/// top drivers per name — what each call is actually a bet on.
fn book_sa(n: usize, seed: u64, lens: Lens, financials_path: Option<&str>, top: usize,
           vp: &ValuationParams) {
    let comps: Vec<_> = load_universe(financials_path)
        .into_iter()
        .filter(|c| c.ntm_earnings_b > 0.0)
        .collect();
    let mut draws = BookSampler::new(seed, lens);
    let mut rej = Rejections::default();
    let mut names: Vec<&'static str> = Vec::new();
    let mut cols: Vec<Vec<f64>> = Vec::new();
    let mut ups: Vec<Vec<f64>> = vec![Vec::with_capacity(n); comps.len()];
    for k in 0..n {
        let (d, states) = conditioned_draw(&mut draws, &mut rej, n);
        let ctx = PathContext::from_params(&d.params);
        let mut vals = sampled_values(&d.params);
        vals.push(("fizzle", d.fizzle as u8 as f64));
        vals.push(("severe_taiwan", ctx.taiwan_start.is_some() as u8 as f64));
        vals.push(("ai_rev_growth_2026", d.params.ai_rev_growth_2026));
        vals.push(("gw_per_dollar_growth", d.params.gw_per_dollar_growth.unwrap_or(0.035)));
        if lens == Lens::Mix {
            vals.push(("lens_lambda", d.lambda));
        }
        if k == 0 {
            names = vals.iter().map(|(nm, _)| *nm).collect();
            cols = vec![Vec::with_capacity(n); names.len()];
        }
        for (j, (_, v)) in vals.iter().enumerate() {
            cols[j].push(*v);
        }
        for (i, c) in comps.iter().enumerate() {
            let (fair, _) = value_on_path(c, &states, ctx, d.params.macrofin.dr_beta, vp);
            ups[i].push(fair / c.mcap_b - 1.0);
        }
    }
    println!("book-sa: n={n} seed={seed} lens={lens:?}  top {top} drivers per name (Spearman rho)");
    println!("{}", rej.summary(n));
    for (i, c) in comps.iter().enumerate() {
        let mut rhos: Vec<(&str, f64)> = names.iter().enumerate()
            .filter(|(j, _)| {
                let col = &cols[*j];
                col.iter().any(|v| (v - col[0]).abs() > 1e-12)
            })
            .map(|(j, nm)| (*nm, spearman(&cols[j], &ups[i])))
            .collect();
        rhos.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
        let cells: Vec<String> = rhos.iter().take(top)
            .map(|(nm, r)| format!("{nm} {r:+.2}"))
            .collect();
        println!("{:<10} {:<6} {}", c.ticker, side_label(c.stance), cells.join(" | "));
    }
}

/// The Monte Carlo book (re-analysis F4): value every name on every sampled path
/// instead of six hand-picked scenarios. The prior is the mc/sa prior plus the
/// named book's fizzle mass; the structural lens is fixed or (`mix`) sampled. Per
/// name: mean/percentiles of upside, P(loss), E[log(1+upside)] (a sizing-relevant
/// geometric view), the gap to the named book in the same lens, and — under `mix`
/// — the Spearman of upside on the lens fraction lambda: |rho| > 0.4 marks the
/// call as a bet on the structural hypotheses rather than on the paths.
fn book_mc(n: usize, seed: u64, lens: Lens, financials_path: Option<&str>, vp: &ValuationParams) {
    let comps: Vec<_> = load_universe(financials_path)
        .into_iter()
        .filter(|c| c.ntm_earnings_b > 0.0)
        .collect();
    let named_eup = |states: &[(&'static str, Vec<singularity_econ::YearState>)]| {
        let dr_beta = singularity_econ::macrofin::MacroParams::default().dr_beta;
        let rows = evaluate_all_with(&comps, states, dr_beta, vp);
        comps.iter().map(|c| {
            rows.iter().find(|r| r.ticker == c.ticker).map_or(f64::NAN, |r| r.expected_upside)
        }).collect::<Vec<f64>>()
    };
    let named: Vec<f64> = match lens {
        Lens::Base => named_eup(&scenario_states()),
        Lens::Enhanced => named_eup(&scenario_states_enhanced()),
        Lens::V2 => named_eup(&scenario_states_v2()),
        Lens::Mix => {
            let (b, v) = (named_eup(&scenario_states()), named_eup(&scenario_states_v2()));
            b.iter().zip(&v).map(|(x, y)| 0.5 * (x + y)).collect()
        }
    };

    let mut draws = BookSampler::new(seed, lens);
    let mut ups: Vec<Vec<f64>> = vec![Vec::with_capacity(n); comps.len()];
    let mut terms: Vec<f64> = vec![0.0; comps.len()];
    let mut lambdas: Vec<f64> = Vec::with_capacity(n);
    let (mut n_fizzle, mut n_taiwan) = (0usize, 0usize);
    let mut rej = Rejections::default();
    for _ in 0..n {
        let (d, states) = conditioned_draw(&mut draws, &mut rej, n);
        let ctx = PathContext::from_params(&d.params);
        n_fizzle += ctx.fizzle as usize;
        n_taiwan += ctx.taiwan_start.is_some() as usize;
        lambdas.push(d.lambda);
        for (i, c) in comps.iter().enumerate() {
            let (fair, tpv) = value_on_path(c, &states, ctx, d.params.macrofin.dr_beta, vp);
            ups[i].push(fair / c.mcap_b - 1.0);
            if fair.abs() > 1e-12 {
                terms[i] += tpv / fair;
            }
        }
    }

    struct Row {
        ticker: &'static str,
        side: &'static str,
        mean: f64,
        p10: f64,
        p50: f64,
        p90: f64,
        p_loss: f64,
        e_log: f64,
        named: f64,
        term: f64,
        rho_lambda: f64,
    }
    let mut rows: Vec<Row> = comps.iter().enumerate().map(|(i, c)| {
        let v = &ups[i];
        let mut sorted = v.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let nf = n as f64;
        Row {
            ticker: c.ticker,
            side: side_label(c.stance),
            mean: v.iter().sum::<f64>() / nf,
            p10: pct(&sorted, 0.10),
            p50: pct(&sorted, 0.50),
            p90: pct(&sorted, 0.90),
            p_loss: v.iter().filter(|u| **u < 0.0).count() as f64 / nf,
            e_log: v.iter().map(|u| (1.0 + u).max(1e-3).ln()).sum::<f64>() / nf,
            named: named[i],
            term: terms[i] / nf,
            rho_lambda: if lens == Lens::Mix { spearman(&lambdas, v) } else { f64::NAN },
        }
    }).collect();
    rows.sort_by(|a, b| b.mean.partial_cmp(&a.mean).unwrap());

    println!("book-mc: n={n} seed={seed} lens={lens:?} valuation={}  fizzle paths {:.1}%  severe-Taiwan paths {:.1}%",
             if *vp == ValuationParams::default() { "legacy" } else { "first-principles" },
             100.0 * n_fizzle as f64 / n as f64, 100.0 * n_taiwan as f64 / n as f64);
    println!("{}", rej.summary(n));
    println!("{:<10} {:<6} {:>8} {:>8} {:>8} {:>8} {:>6} {:>7} {:>8} {:>8} {:>6} {:>6}",
             "ticker", "side", "E[up]", "p10", "p50", "p90", "P(loss)", "E[log]",
             "named", "gap", "term%", "rho_l");
    for r in &rows {
        let flag = if r.rho_lambda.abs() > 0.4 { "  structural bet" } else { "" };
        println!("{:<10} {:<6} {:>7.1}% {:>7.1}% {:>7.1}% {:>7.1}% {:>6.0}% {:>7.2} {:>7.1}% {:>7.1}% {:>5.0}% {:>6.2}{}",
                 r.ticker, r.side, r.mean * 100.0, r.p10 * 100.0, r.p50 * 100.0,
                 r.p90 * 100.0, r.p_loss * 100.0, r.e_log, r.named * 100.0,
                 (r.mean - r.named) * 100.0, r.term * 100.0, r.rho_lambda, flag);
    }
}

fn pct(sorted: &[f64], q: f64) -> f64 {
    let idx = ((q * sorted.len() as f64) as usize).min(sorted.len() - 1);
    sorted[idx]
}

fn monte_carlo(n: usize, seed: u64) {
    let mut draw = Sampler::new(seed);
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
    let mut draw = Sampler::new(seed);
    // All sampled parameters (sampler::sampled_values) — previously a hard-coded
    // 17 of ~44, which hid drivers the sampler already drew.
    let mut param_names: Vec<&'static str> = Vec::new();
    let mut param_vals: Vec<Vec<f64>> = Vec::new();
    let mut out_silicon_norm: Vec<f64> = Vec::new();
    let mut out_power_margin_32: Vec<f64> = Vec::new();
    let mut out_credit_min: Vec<f64> = Vec::new();
    let mut out_disp_32: Vec<f64> = Vec::new();
    let mut out_robot_32: Vec<f64> = Vec::new();
    let mut out_capex_32: Vec<f64> = Vec::new();

    for _ in 0..n {
        let p = draw.params();
        let sv = sampled_values(&p);
        if param_names.is_empty() {
            param_names = sv.iter().map(|(k, _)| *k).collect();
            param_vals = vec![Vec::new(); sv.len()];
        }
        for (i, (_, v)) in sv.iter().enumerate() {
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
