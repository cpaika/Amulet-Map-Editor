//! Valuation engine: converts model profit-pool paths into per-company
//! scenario valuations and trade scores. Port of the retired `valuation.py`
//! (this file is now the specification).
//!
//! Method: each company maps to model pools by RATIO (stable-base pools
//! only) plus optional NEW-POOL CAPTURE (share-of-emerging-pool economics,
//! phased over ~4 years). Scenario PVs use a punitive 12% discount rate;
//! the reverse DCF solves for the growth rate the current price requires.

use crate::{Pools, YearState};

pub const DISCOUNT_RATE: f64 = 0.12;
pub const HORIZON: usize = 10;

/// Thesis-conditional scenario probabilities (fizzle weight is the
/// discipline; see REPORT.md for the unconditional-probability caveat).
pub const SCENARIO_PROBS: [(&str, f64); 6] = [
    // taiwan_shock carved proportionally out of the prior five (its 7%
    // ≈ the sampler's cumulative quarantine->blockade probability mass
    // landing in the valuation-relevant 2027-2031 window).
    ("baseline", 0.325),
    ("fast_takeoff", 0.14),
    ("delayed", 0.2325),
    ("friction", 0.1395),
    ("fizzle", 0.093),
    ("taiwan_shock", 0.07),
];

/// Pools with a meaningful, stable 2026 revenue base — the ONLY pools that
/// may be mapped by growth ratio. Emerging pools (ai_services, robots,
/// robot_components, robot_services, electricity, ip_tolls) must use
/// `capture` instead: ratio mapping on a near-zero base explodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RatioPool {
    Silicon,
    DcInfra,
    PowerEquipment,
    ItServices,
    Bpo,
    SeatSaas,
    ProfInfo,
    HumanCognitiveWages,
    HumanPhysicalWages,
    GdpIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmergingPool {
    AiServices,
    Electricity,
    Robots,
    RobotComponents,
    RobotServices,
    IpTolls,
}

impl Pools {
    pub fn ratio(&self, p: RatioPool) -> f64 {
        match p {
            RatioPool::Silicon => self.silicon,
            RatioPool::DcInfra => self.dc_infra,
            RatioPool::PowerEquipment => self.power_equipment,
            RatioPool::ItServices => self.it_services,
            RatioPool::Bpo => self.bpo,
            RatioPool::SeatSaas => self.seat_saas,
            RatioPool::ProfInfo => self.prof_info,
            RatioPool::HumanCognitiveWages => self.human_cognitive_wages,
            RatioPool::HumanPhysicalWages => self.human_physical_wages,
            RatioPool::GdpIndex => self.gdp_index,
        }
    }

    pub fn emerging(&self, p: EmergingPool) -> f64 {
        match p {
            EmergingPool::AiServices => self.ai_services,
            EmergingPool::Electricity => self.electricity,
            EmergingPool::Robots => self.robots,
            EmergingPool::RobotComponents => self.robot_components,
            EmergingPool::RobotServices => self.robot_services,
            EmergingPool::IpTolls => self.ip_tolls,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    Long,
    Short,
    Watch,
}

#[derive(Clone, Debug)]
pub struct Company {
    pub ticker: &'static str,
    pub name: &'static str,
    pub mcap_b: f64,
    pub ntm_earnings_b: f64,
    /// (pool, weight) — weights sum to ~1.0
    pub pools: Vec<(RatioPool, f64)>,
    pub pool_beta: f64,
    pub share_drift: f64,
    pub terminal_multiple: f64,
    pub stance: Stance,
    /// (pool, share_of_pool, net_margin) — adds share*margin*pool($T)*1000
    /// to earnings ($B), phased in over ~4 years.
    pub capture: Vec<(EmergingPool, f64, f64)>,
    pub notes: &'static str,
}

pub fn earnings_path(c: &Company, states: &[YearState]) -> Vec<f64> {
    let base = &states[0].pools;
    let base_silicon_margin = states[0].silicon_margin.max(1e-6);
    let mut path = Vec::with_capacity(HORIZON);
    for (i, s) in states.iter().enumerate().skip(1) {
        if path.len() == HORIZON {
            break;
        }
        let mut growth = 0.0;
        for &(pool, w) in &c.pools {
            let p0 = base.ratio(pool).max(1e-9);
            // Route the SILICON revenue pool through its ENDOGENOUS margin
            // (audit #5): the B1 capacity rent compresses (silicon_margin
            // ~0.50 -> ~0.24), so pricing silicon names on revenue^beta at a
            // frozen 2026 margin overstates earnings. Other ratio pools have no
            // model margin and stay revenue proxies.
            let margin_mult = if pool == RatioPool::Silicon {
                s.silicon_margin / base_silicon_margin
            } else {
                1.0
            };
            growth += w * (s.pools.ratio(pool) / p0) * margin_mult;
        }
        let growth = growth.max(0.0).powf(c.pool_beta);
        let mut e = c.ntm_earnings_b * growth * (1.0 + c.share_drift).powi(i as i32);
        let phase = (i as f64 / 4.0).min(1.0);
        for &(pool, share, margin) in &c.capture {
            e += phase * share * margin * s.pools.emerging(pool) * 1000.0;
        }
        path.push(e);
    }
    path
}

pub fn pv(path: &[f64], terminal_multiple: f64, r: f64) -> f64 {
    let mut v = 0.0;
    for (i, e) in path.iter().enumerate() {
        v += e / (1.0 + r).powi(i as i32 + 1);
    }
    if let Some(last) = path.last() {
        v += last * terminal_multiple / (1.0 + r).powi(path.len() as i32);
    }
    v
}

/// Reverse DCF: constant growth rate the current price requires.
pub fn implied_cagr(mcap_b: f64, e0: f64, terminal_multiple: f64) -> f64 {
    let (mut lo, mut hi) = (-0.5_f64, 1.5_f64);
    for _ in 0..80 {
        let g = (lo + hi) / 2.0;
        let path: Vec<f64> = (0..HORIZON)
            .map(|i| e0 * (1.0 + g).powi(i as i32 + 1))
            .collect();
        if pv(&path, terminal_multiple, DISCOUNT_RATE) < mcap_b {
            lo = g;
        } else {
            hi = g;
        }
    }
    (lo + hi) / 2.0
}

#[derive(Clone, Debug)]
pub struct ScenarioValue {
    pub scenario: &'static str,
    pub fair_value_b: f64,
    pub upside: f64,
}

#[derive(Clone, Debug)]
pub struct Evaluation {
    pub ticker: &'static str,
    pub stance: Stance,
    pub mcap_b: f64,
    pub implied_cagr: f64,
    pub expected_upside: f64,
    pub worst_scenario_upside: f64,
    pub best_scenario_upside: f64,
    pub per_scenario: Vec<ScenarioValue>,
}

/// Per-scenario discount rate: the flat 12% plus an equity-duration beta
/// on the scenario's mean modeled long rate above the 4.5% anchor (B11).
/// A high-transfer/high-debt scenario carries a higher discount than
/// fizzle — compressing exactly the power/toll EVs where they win, which
/// the flat-rate model silently ignored (gap-scan #1).
fn scenario_discount(states: &[YearState]) -> f64 {
    let n = states.len().max(1) as f64;
    let mean_long: f64 = states.iter().map(|s| s.long_rate).sum::<f64>() / n;
    const DR_BETA: f64 = 0.60;
    DISCOUNT_RATE + DR_BETA * (mean_long - 0.045)
}

pub fn evaluate(
    c: &Company,
    scenario_states: &[(&'static str, Vec<YearState>)],
) -> Evaluation {
    let mut per: Vec<ScenarioValue> = Vec::new();
    for (name, states) in scenario_states {
        let path = earnings_path(c, states);
        let tm = if *name == "fizzle" {
            c.terminal_multiple * 0.75
        } else {
            c.terminal_multiple
        };
        let dr = scenario_discount(states);
        let fair = pv(&path, tm, dr);
        per.push(ScenarioValue {
            scenario: name,
            fair_value_b: fair,
            upside: fair / c.mcap_b - 1.0,
        });
    }
    assert!(!per.is_empty(), "evaluate() needs at least one scenario");
    let expected = SCENARIO_PROBS.iter().map(|(n, p)| {
        per.iter()
            .find(|v| v.scenario == *n)
            .unwrap_or_else(|| panic!("scenario {n} missing from states"))
            .upside * p
    }).sum();
    let worst = per.iter().map(|v| v.upside).fold(f64::MAX, f64::min);
    let best = per.iter().map(|v| v.upside).fold(f64::MIN, f64::max);
    Evaluation {
        ticker: c.ticker,
        stance: c.stance,
        mcap_b: c.mcap_b,
        implied_cagr: implied_cagr(c.mcap_b, c.ntm_earnings_b, c.terminal_multiple),
        expected_upside: expected,
        worst_scenario_upside: worst,
        best_scenario_upside: best,
        per_scenario: per,
    }
}

pub fn evaluate_all(
    companies: &[Company],
    scenario_states: &[(&'static str, Vec<YearState>)],
) -> Vec<Evaluation> {
    let mut rows: Vec<Evaluation> = companies
        .iter()
        .filter(|c| c.ntm_earnings_b > 0.0)
        .map(|c| evaluate(c, scenario_states))
        .collect();
    rows.sort_by(|a, b| b.expected_upside.partial_cmp(&a.expected_upside).unwrap());
    rows
}
