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

impl crate::Profits {
    /// Emerging-pool PROFIT ($T) — the pool revenue with the model's ENDOGENOUS
    /// margin already applied (electricity_margin, component_margin, …). Capture
    /// earnings ride this instead of a frozen per-company margin constant (audit V).
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
    /// (pool, share_of_pool) — adds share * pool_PROFIT($T) * 1000 to earnings
    /// ($B), phased in over ~4 years. The margin is the model's endogenous one
    /// (audit V), so only the captured SHARE is a per-company constant now.
    pub capture: Vec<(EmergingPool, f64)>,
    /// Share of the company's productive capacity physically located in Taiwan
    /// (0 = none). In the `taiwan_shock` (invasion) scenario this capacity is
    /// destroyed, so the name takes a fair-value haircut the global Silicon pool
    /// CANNOT express — the pool's scarcity margin RISES in the shock, which would
    /// otherwise perversely mark TSMC (the fab being destroyed) UP. Fabless names
    /// take the volume hit through the pool instead and keep exposure ~0.
    pub taiwan_fab_exposure: f64,
    /// Ceiling on the firm's revenue-growth multiple, as a CAGR (F1): a firm cannot
    /// ride a pool that grows 19x without building 19x the plant or ceding share.
    /// Read only when `ValuationParams::capacity_gain > 0`.
    pub max_rev_cagr: f64,
    /// Net debt, $B (negative = net cash). Read only when
    /// `ValuationParams::leverage_gain > 0`.
    pub net_debt_b: f64,
    /// Share of earnings exposed to export-control / trade restrictions on the
    /// China corridor (Western names: sales into China; Chinese names: sales to
    /// Western customers). Read only when `ValuationParams::policy_gain > 0`.
    pub china_revenue_share: f64,
    /// 1.0 for listings a foreign (US) holder could be forced out of by an
    /// investment ban (China A-shares), 0 otherwise.
    pub foreign_access_risk: f64,
    pub notes: &'static str,
}

/// Annual hazard of an export-control escalation rung — the geopolitics sampler's
/// S5 rate (25%/yr), used as the expectation in the named books.
pub const EXPORT_RUNG_HAZARD: f64 = 0.25;
/// Annual hazard of a US ban forcing foreign holders out of China A-shares
/// (NS-CMIC-list / outbound-investment-rule style), tripled from the first year of
/// a severe Taiwan episode.
pub const ACCESS_BAN_HAZARD: f64 = 0.03;
pub const ACCESS_BAN_TAIWAN_MULT: f64 = 3.0;

/// Default firm revenue-growth ceiling: 30%/yr sustained for a decade (~14x) is
/// the top of the historical large-cap envelope.
pub const DEFAULT_MAX_REV_CAGR: f64 = 0.30;

/// Valuation conventions (F1, Sep-26 re-analysis). Every gain defaults to 0, which
/// is the legacy valuation byte-for-byte; `first_principles()` turns all three on.
/// Ship them together: power routing alone RAISES POWL (450 -> 581 measured).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValuationParams {
    /// Blend weight on capping each name's pool-revenue multiple at
    /// (1 + max_rev_cagr)^t, applied before pool_beta.
    pub capacity_gain: f64,
    /// Blend weight on striking the TERMINAL on steady-state spend: capex-driven
    /// pools (Silicon, DcInfra, PowerEquipment) are capitalized at the spend that
    /// would sustain the year-10 installed base growing at `long_run_growth`, not at
    /// a still-accelerating year-10 build rate. Only ever lowers the terminal.
    pub flow_terminal_gain: f64,
    /// Blend weight on routing PowerEquipment revenue through the model's
    /// endogenous power margin, mirroring the silicon-margin line.
    pub power_route_gain: f64,
    /// Perpetual growth of the installed base in the flow terminal.
    pub long_run_growth: f64,
    /// Use each name's multiple-implied perpetual growth (Gordon: g = r - 1/tm,
    /// clamped to [0, 8%]) instead of `long_run_growth`, so the steady-state spend
    /// grows at exactly the rate the terminal multiple already capitalizes.
    pub growth_from_multiple: bool,
    /// Economic life (years) of grid equipment in the flow terminal.
    pub power_equipment_life: f64,
    /// $T per GW of grid equipment — must match `Params::power_equip_cost_per_gw`.
    pub power_equip_cost_per_gw: f64,
    /// Compute economic depreciation — must match `Params::compute_deprec`.
    pub compute_deprec: f64,
    /// Blend weight on capital structure (capture shortlist #1): the business, not
    /// net income, grows with the pools; after-tax interest on net debt is
    /// subtracted, with the debt repricing toward the modeled long rate as it
    /// refinances. Net cash earns the rate symmetrically. Gives levered names their
    /// equity torque to both the business and rates (NRG: net debt > market cap).
    pub leverage_gain: f64,
    /// Credit spread over the long rate paid on net debt.
    pub debt_spread: f64,
    /// Average debt maturity (years): the share repriced by year t is 1-(1-1/M)^t.
    pub debt_maturity: f64,
    pub tax_rate: f64,
    /// Blend weight on policy risk (4 channels). Export controls: each escalation
    /// rung after 2026 removes `export_rung_bite` of the remaining China-corridor
    /// earnings (realized rungs on MC paths; the S5 hazard in expectation on the
    /// named books). Investment access: a foreign holder of an exposed listing is
    /// forced to sell at `access_haircut` below fair value if a ban lands in the
    /// horizon. Power windfall levy: once the electricity price runs 20% above
    /// 2026, a UK-EGL / EU-cap style levy takes `power_levy` of the generator margin
    /// above the 0.30 normal (sticky). AI windfall tax: once cognitive displacement
    /// passes `ai_tax_trigger`, `ai_windfall_tax` of AI-services capture rent is
    /// taxed away (sticky).
    pub policy_gain: f64,
    pub export_rung_bite: f64,
    pub access_haircut: f64,
    pub power_levy: f64,
    pub ai_tax_trigger: f64,
    pub ai_windfall_tax: f64,
    /// Upper bound on the compute unit-cost decline used by the flow terminal (the
    /// calendar `Params::hw_cost_decline`); the actual rate is read off the path.
    /// In a dollar steady state falling unit cost keeps the UNIT stock growing, so
    /// the steady-state spend must be computed in dollars, not units.
    pub compute_cost_decline: f64,
}

impl Default for ValuationParams {
    fn default() -> Self {
        ValuationParams {
            capacity_gain: 0.0,
            flow_terminal_gain: 0.0,
            power_route_gain: 0.0,
            long_run_growth: 0.04,
            growth_from_multiple: false,
            power_equipment_life: 30.0,
            power_equip_cost_per_gw: 0.0035,
            compute_deprec: 0.25,
            compute_cost_decline: 0.15,
            leverage_gain: 0.0,
            debt_spread: 0.015,
            debt_maturity: 6.0,
            tax_rate: 0.21,
            policy_gain: 0.0,
            export_rung_bite: 0.3,
            access_haircut: 0.4,
            power_levy: 0.45,
            ai_tax_trigger: 0.15,
            ai_windfall_tax: 0.25,
        }
    }
}

impl ValuationParams {
    pub fn first_principles() -> Self {
        ValuationParams {
            capacity_gain: 1.0,
            flow_terminal_gain: 1.0,
            power_route_gain: 1.0,
            growth_from_multiple: true,
            leverage_gain: 1.0,
            policy_gain: 1.0,
            ..Self::default()
        }
    }

    /// Steady-state / actual spend ratio for a capex-driven pool at the path's last
    /// year, capped at 1 (the flow terminal only removes build-rate froth).
    fn steady_state_ratio(&self, pool: RatioPool, states: &[YearState], tm: f64) -> f64 {
        let n = states.len();
        if n < 2 {
            return 1.0;
        }
        let (last, prev) = (&states[n - 1], &states[n - 2]);
        // Vintage steady state: with spend growing at g, assets retiring at rate d
        // and replacement cost falling at c, the installed base valued at current
        // cost is V = spend / (1 - (1-d)(1-c)/(1+g)); invert for the spend that
        // sustains the year-10 base.
        let g = if self.growth_from_multiple {
            (DISCOUNT_RATE - 1.0 / tm.max(1.0)).clamp(0.0, 0.08)
        } else {
            self.long_run_growth
        };
        let ss_factor = |d: f64, c: f64| 1.0 - (1.0 - d) * (1.0 - c) / (1.0 + g);
        let ratio = match pool {
            RatioPool::PowerEquipment => {
                let base_value = self.power_equip_cost_per_gw * last.ai_power_installed_gw;
                base_value * ss_factor(1.0 / self.power_equipment_life, 0.0)
                    / last.pools.power_equipment.max(1e-12)
            }
            RatioPool::Silicon | RatioPool::DcInfra => {
                // Units added this year; capex bought them, so capex/added is the
                // current unit cost and stock × that is the base at current cost.
                let added = last.compute_stock - prev.compute_stock * (1.0 - self.compute_deprec);
                if added <= 1e-12 {
                    return 1.0;
                }
                // Unit-cost decline read off the path itself (capex / units added over
                // the last two years), capped at the calendar rate: under Wright's-law
                // learning (v2) the late-horizon decline is ~0.10-0.12, ~0 in a bust.
                let c = if n >= 3 {
                    let prev2 = &states[n - 3];
                    let added_prev =
                        prev.compute_stock - prev2.compute_stock * (1.0 - self.compute_deprec);
                    if added_prev > 1e-12 && prev.ai_capex > 1e-12 {
                        let unit_now = last.ai_capex / added;
                        let unit_prev = prev.ai_capex / added_prev;
                        (1.0 - unit_now / unit_prev).clamp(0.0, self.compute_cost_decline)
                    } else {
                        self.compute_cost_decline
                    }
                } else {
                    self.compute_cost_decline
                };
                last.compute_stock * ss_factor(self.compute_deprec, c) / added
            }
            _ => 1.0,
        };
        ratio.clamp(0.0, 1.0)
    }
}

/// Fraction of Taiwan-located capacity lost in the taiwan_shock (invasion)
/// scenario — a multi-year, EUV-capped rebuild (design table S4).
pub const TAIWAN_FAB_LOSS: f64 = 0.55;

/// Annual erosion of a captured emerging-pool SHARE past its ~4yr phase-in
/// (audit C2). A fixed-capacity name's share of a GROWING pool declines as
/// competitors add supply — the scarcity rent is not permanent. This also haircuts
/// rent-dominated terminal value (the terminal earnings a full multiple is applied
/// to are already decayed), so a peak-rent 2036 is not capitalized to perpetuity.
pub const CAPTURE_DECAY: f64 = 0.06;

pub fn earnings_path(c: &Company, states: &[YearState]) -> Vec<f64> {
    earnings_and_rent(c, states).0
}

/// Earnings path AND its capture-RENT component, year-aligned. The rent slice is the
/// part of each year's earnings coming from decaying emerging-pool capture; it is
/// separated so the TERMINAL value can capitalize it as a decaying stream rather than
/// a perpetuity (re-audit C2: the flow-only CAPTURE_DECAY was too weak to keep a
/// near-peak, still-rent-dominated final year from being capitalized at the full
/// multiple). `.0` is the total path; `.1` is the rent-only path.
pub fn earnings_and_rent(c: &Company, states: &[YearState]) -> (Vec<f64>, Vec<f64>) {
    let r = earnings_with(c, states, &ValuationParams::default());
    (r.path, r.rent)
}

/// Earnings path, its rent slice, and the base (non-rent) earnings the TERMINAL is
/// struck on — equal to the last base earnings unless the flow terminal is on.
pub struct EarningsPath {
    pub path: Vec<f64>,
    pub rent: Vec<f64>,
    pub terminal_base: Option<f64>,
}

/// Company base-earnings multiple at year index `i` for a given set of pool
/// ratios (`ratio_of(pool)` = pool_t / pool_2026).
fn base_growth(
    c: &Company,
    s: &YearState,
    i: usize,
    base_silicon_margin: f64,
    base_power_margin: f64,
    vp: &ValuationParams,
    ratio_of: &dyn Fn(RatioPool) -> f64,
) -> f64 {
    // pool_beta semantics (re-audit #29, documented decision): beta is COMPANY-
    // level operating leverage applied to the blended revenue base — the weighted
    // SUM of pool ratios raised to beta — not per-pool torque. The whole book was
    // calibrated under this reading; changing it would silently reprice every
    // multi-pool name (TECK/FCX ~19%). Intentional; do not "fix" to per-pool.
    let mut growth = 0.0;
    let mut w_silicon = 0.0;
    let mut w_power = 0.0;
    for &(pool, w) in &c.pools {
        growth += w * ratio_of(pool);
        if pool == RatioPool::Silicon {
            w_silicon += w;
        }
        if pool == RatioPool::PowerEquipment {
            w_power += w;
        }
    }
    let mut growth = growth.max(0.0);
    if vp.capacity_gain > 0.0 && c.max_rev_cagr.is_finite() {
        let cap = (1.0 + c.max_rev_cagr).powi(i as i32);
        let g = vp.capacity_gain.clamp(0.0, 1.0);
        growth = (1.0 - g) * growth + g * growth.min(cap);
    }
    // Route the SILICON pool through its ENDOGENOUS margin (audit #5): the B1
    // capacity rent compresses (silicon_margin ~0.50 -> ~0.24), so pricing silicon
    // names on revenue^beta at a frozen 2026 margin overstates earnings. Applied
    // POST-exponent, weighted by the company's silicon exposure (re-audit #27):
    // inside the beta base the correction was muted to margin^beta·w — a beta<1
    // name kept most of the vanished margin; a beta>1 name over-shed it. A margin
    // is a scalar on earnings, not a growth term: earnings = revenue^beta × margin.
    let margin_ratio = (s.silicon_margin / base_silicon_margin).max(1e-9);
    let mut e = growth.powf(c.pool_beta) * margin_ratio.powf(w_silicon);
    if vp.power_route_gain > 0.0 && w_power > 0.0 {
        let pm = (s.power_margin / base_power_margin).max(1e-9).powf(w_power);
        let g = vp.power_route_gain.clamp(0.0, 1.0);
        e *= (1.0 - g) + g * pm;
    }
    e * (1.0 + c.share_drift).powi(i as i32)
}

pub fn earnings_with(c: &Company, states: &[YearState], vp: &ValuationParams) -> EarningsPath {
    let base = &states[0].pools;
    let base_silicon_margin = states[0].silicon_margin.max(1e-6);
    let base_power_margin = states[0].power_margin.max(1e-6);
    let mut path = Vec::with_capacity(HORIZON);
    let mut rent = Vec::with_capacity(HORIZON);
    let mut terminal_base = None;
    let (mut levy_on, mut ai_tax_on) = (false, false);
    let last_i = states.len().min(HORIZON + 1) - 1;
    for (i, s) in states.iter().enumerate().skip(1) {
        if path.len() == HORIZON {
            break;
        }
        let ratio_of = |pool: RatioPool| s.pools.ratio(pool) / base.ratio(pool).max(1e-9);
        let lever = |mult: f64| -> f64 {
            let plain = c.ntm_earnings_b * mult;
            if vp.leverage_gain <= 0.0 || c.net_debt_b == 0.0 {
                return plain;
            }
            let r0 = states[0].long_rate;
            let spread = if c.net_debt_b > 0.0 { vp.debt_spread } else { 0.0 };
            let after_tax = 1.0 - vp.tax_rate;
            let interest0 = c.net_debt_b * (r0 + spread);
            let unlevered0 = c.ntm_earnings_b + interest0 * after_tax;
            let repriced = 1.0 - (1.0 - 1.0 / vp.debt_maturity.max(1.0)).powi(i as i32);
            // Real burden: inflation above the anchor erodes nominal principal.
            let cost_t = r0 + spread + repriced * (s.long_rate - r0) - s.infl_premium;
            let levered = unlevered0 * mult - c.net_debt_b * cost_t * after_tax;
            let g = vp.leverage_gain.clamp(0.0, 1.0);
            (1.0 - g) * plain + g * levered
        };
        let base_e = lever(base_growth(c, s, i, base_silicon_margin, base_power_margin, vp, &ratio_of));
        if i == last_i && vp.flow_terminal_gain > 0.0 {
            terminal_base = Some({
                let window = &states[..=i];
                let ss_ratio_of = |pool: RatioPool| ratio_of(pool) * vp.steady_state_ratio(pool, window, c.terminal_multiple);
                let ss_e = lever(base_growth(c, s, i, base_silicon_margin, base_power_margin, vp, &ss_ratio_of));
                let g = vp.flow_terminal_gain.clamp(0.0, 1.0);
                (1.0 - g) * base_e + g * ss_e
            });
        }
        let phase = (i as f64 / 4.0).min(1.0);
        // Competitive erosion of the captured share past phase-in (audit C2): the
        // rent is not permanent — a fixed-capacity name's share of a growing pool
        // decays as supply responds.
        let persistence = (1.0 - CAPTURE_DECAY).powf((i as f64 - 4.0).max(0.0));
        // Capture earnings ride the model's ENDOGENOUS profit pool (audit V): the
        // captured share times the pool's PROFIT ($T, margin already applied by the
        // sim) — not a frozen per-company margin constant. So VST/NRG/CEG electricity
        // capture now tracks the endogenous electricity_margin (0.30 -> ~0.60) and
        // component capture tracks component_margin, instead of a hand-set number.
        if vp.policy_gain > 0.0 {
            if s.electricity_price > 1.2 * states[0].electricity_price {
                levy_on = true;
            }
            if s.cog_displacement > vp.ai_tax_trigger {
                ai_tax_on = true;
            }
        }
        let mut r = 0.0;
        for &(pool, share) in &c.capture {
            let mut slice = phase * persistence * share * s.profits.emerging(pool) * 1000.0;
            if vp.policy_gain > 0.0 {
                let g = vp.policy_gain.clamp(0.0, 1.0);
                match pool {
                    EmergingPool::Electricity if levy_on && s.pools.electricity > 1e-12 => {
                        let m = s.profits.electricity / s.pools.electricity;
                        let excess = ((m - 0.30) / m.max(1e-9)).clamp(0.0, 1.0);
                        slice *= 1.0 - g * vp.power_levy * excess;
                    }
                    EmergingPool::AiServices if ai_tax_on => {
                        slice *= 1.0 - g * vp.ai_windfall_tax;
                    }
                    _ => {}
                }
            }
            r += slice;
        }
        path.push(base_e + r);
        rent.push(r);
    }
    EarningsPath { path, rent, terminal_base }
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
    /// Share of this scenario's fair value that is the year-10 terminal slice.
    pub terminal_share: f64,
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
    /// Probability-weighted share of fair value coming from the terminal slice.
    /// Above ~0.8 the name is priced almost entirely on 2036 earnings — read its
    /// upside as a bet on the terminal-year pool, not on the 2026-35 path.
    pub terminal_share: f64,
    pub per_scenario: Vec<ScenarioValue>,
    /// Attribution (F6). `conv_upside`: E[up] of the SAME name if every pool it
    /// maps to just tracked the GDP index, with no capture and no share drift —
    /// the part of the call that is valuation convention (12% discount, the name's
    /// multiple) rather than thesis. The market proxy prints about -40% here in
    /// every scenario. `drift_delta`: E[up] added by the hand-set share drift.
    /// `ai_delta`: the rest, i.e. what the modeled AI economy adds or subtracts.
    /// expected_upside = conv_upside + drift_delta + ai_delta (exactly).
    pub conv_upside: f64,
    pub drift_delta: f64,
    pub ai_delta: f64,
}

/// |ai_delta| below this and the call is valuation-only, not an AI thesis.
pub const THESIS_THRESHOLD: f64 = 0.05;

impl Evaluation {
    pub fn is_valuation_only(&self) -> bool {
        self.ai_delta.abs() < THESIS_THRESHOLD
    }
}

/// The name with no AI thesis: every mapped pool replaced by the GDP index at the
/// same weight, capture cleared (share drift kept; `no_drift` clears it too).
fn counterfactual(c: &Company, no_drift: bool) -> Company {
    let mut k = c.clone();
    k.pools = c.pools.iter().map(|&(_, w)| (RatioPool::GdpIndex, w)).collect();
    k.capture.clear();
    if no_drift {
        k.share_drift = 0.0;
    }
    k
}

/// Per-scenario discount rate: the flat 12% plus an equity-duration beta
/// on the scenario's mean modeled long rate above the 4.5% anchor (B11).
/// A high-transfer/high-debt scenario carries a higher discount than
/// fizzle — compressing exactly the power/toll EVs where they win, which
/// the flat-rate model silently ignored (gap-scan #1).
///
/// `dr_beta` is the equity-duration beta from `MacroParams` (audit A3: this
/// was a frozen `const 0.60`, decoupling the discount from the layer that owns
/// it). Threading it makes the coupling tunable; default 0.60 is
/// behavior-preserving. NB: the current callers still pass the DEFAULT
/// (`book()` and `main`), so the discount is not yet perturbed per-draw — wiring
/// the MC sampler to pass `p.macrofin.dr_beta` into the valuation path is a
/// separate follow-up.
fn scenario_discount(states: &[YearState], dr_beta: f64) -> f64 {
    // Mean over the FLOW years only (re-audit #28): states[0] is the scenario-
    // invariant 2026 base year; including it diluted the cross-scenario rate spread.
    let flows = &states[1..states.len().max(2)];
    let n = flows.len().max(1) as f64;
    // Real flows are discounted at the REAL component of the rate move: the
    // fiscal-dominance expected-inflation premium is stripped (Fisher); it is 0
    // when that regime is off, so the legacy discount is unchanged.
    let mean_long: f64 = flows.iter().map(|s| s.long_rate - s.infl_premium).sum::<f64>() / n;
    DISCOUNT_RATE + dr_beta * (mean_long - 0.045)
}

/// Terminal-slice discount (re-audit #28): the terminal dominates PV, and a flat
/// path-mean rate on it left the B11 rate coupling a near-no-op. The terminal is a
/// perpetuity struck at horizon end, so it prices off the TERMINAL-year long rate.
fn terminal_discount(states: &[YearState], dr_beta: f64) -> f64 {
    let last = states.last().map_or(0.045, |s| s.long_rate - s.infl_premium);
    DISCOUNT_RATE + dr_beta * (last - 0.045)
}

/// Path-level valuation context: which scenario-specific adjustments apply to a
/// single simulated path. The named book derives it from the scenario name; the
/// Monte Carlo book (`book-mc`) derives it from the drawn `Params`, so both value
/// every path through one code path (`value_on_path`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PathContext {
    /// No-singularity path: the terminal multiple is cut 25% (growth never arrives).
    pub fizzle: bool,
    /// First year of a severe Taiwan episode; flows from this year on (and the
    /// terminal) carry the company's fab-destruction haircut.
    pub taiwan_start: Option<i32>,
    /// Policy draws for a sampled path; `None` = named book, which prices policy
    /// in expectation (the hazards above).
    pub policy: Option<PolicyDraw>,
}

/// Realized policy events on one Monte Carlo path.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PolicyDraw {
    /// Export-control rungs STARTING in each year, indexed from the base year.
    pub rungs: [u8; 16],
    /// Year a foreign-investor access ban lands on China A-shares, if any.
    pub access_ban_year: Option<i32>,
}

impl PathContext {
    /// Context for one of the six named scenarios.
    pub fn named(name: &str) -> Self {
        PathContext {
            policy: None,
            fizzle: name == "fizzle",
            taiwan_start: if name == "taiwan_shock" {
                Some(crate::scenarios::TAIWAN_SHOCK_START)
            } else {
                None
            },
        }
    }

    /// Context for a sampled path. Fizzle = the singularity never arrives in the
    /// horizon (the named fizzle's `singularity_year: 2099` convention). A path is
    /// "severe Taiwan" when it contains a blockade or invasion — the named
    /// `taiwan_shock` is exactly a quarantine escalating to a blockade — and the
    /// haircut starts at that path's first Taiwan quarantine/blockade/invasion,
    /// mirroring the named scenario starting it at the 2028 quarantine.
    pub fn from_params(p: &crate::Params) -> Self {
        use crate::ShockKind::*;
        let severe = p
            .geo_shocks
            .iter()
            .any(|g| matches!(g.kind, TaiwanBlockade | TaiwanInvasion));
        let taiwan_start = if severe {
            p.geo_shocks
                .iter()
                .filter(|g| matches!(g.kind, TaiwanQuarantine | TaiwanBlockade | TaiwanInvasion))
                .map(|g| g.start_year)
                .min()
        } else {
            None
        };
        let mut rungs = [0u8; 16];
        for g in p.geo_shocks.iter().filter(|g| g.kind == ExportControlRung) {
            let i = g.start_year - p.start_year;
            if (0..16).contains(&i) {
                rungs[i as usize] = rungs[i as usize].saturating_add(1);
            }
        }
        PathContext {
            fizzle: p.singularity_year > p.end_year,
            taiwan_start,
            policy: Some(PolicyDraw { rungs, access_ban_year: None }),
        }
    }
}

/// 2026 base-year anchors for the pools the valuation maps onto, plus AI capex:
/// `(name, value, relative tolerance)`. Values are the calibrated 2026 state
/// (Params::default); tolerances are the band a path's base year may sit in and
/// still be valued against these ratio mappings (re-analysis F7c: an inconsistent
/// base year once produced +690%). Wider bands where the observed level itself is
/// uncertain (AI-services revenue, the electricity pool). Known gaps vs Sep-2026
/// observations, NOT enforced here because the default misses them (queued for the
/// Wave 2 power/capex recalibration): AI capex observed ~$0.85-0.95T (model 0.66)
/// and electricity price observed ~$0.05-0.10/kWh (model 0.136).
pub const ANCHORS_2026: [(&str, f64, f64); 8] = [
    ("silicon", 0.364, 0.20),
    ("dc_infra", 0.2978, 0.20),
    ("power_equipment", 0.105, 0.20),
    ("ai_capex", 0.662, 0.20),
    ("it_services", 1.548, 0.10),
    ("gdp_index", 115.0, 0.05),
    ("ai_services", 0.0422, 0.50),
    ("electricity", 0.1026, 0.35),
];

/// Is this path's 2026 base year consistent with what is already known about
/// 2026? Chips (memory + packaging) are the observed binding constraint, and every
/// mapped pool must sit within its `ANCHORS_2026` band. `book-mc` rejects draws
/// that fail (ABC-style conditioning on the observed year) and reports the rate.
pub fn base_year_consistent(states: &[YearState]) -> Result<(), String> {
    let s0 = states.first().ok_or("empty path")?;
    if s0.binding != crate::Binding::Chips {
        return Err(format!("2026 binding {:?}, observed Chips", s0.binding));
    }
    for (name, anchor, tol) in ANCHORS_2026 {
        let v = match name {
            "silicon" => s0.pools.silicon,
            "dc_infra" => s0.pools.dc_infra,
            "power_equipment" => s0.pools.power_equipment,
            "ai_capex" => s0.ai_capex,
            "it_services" => s0.pools.it_services,
            "gdp_index" => s0.pools.gdp_index,
            "ai_services" => s0.pools.ai_services,
            "electricity" => s0.pools.electricity,
            _ => unreachable!(),
        };
        if (v / anchor - 1.0).abs() > tol {
            return Err(format!("2026 {name} {v:.4} outside {anchor} ±{:.0}%", tol * 100.0));
        }
    }
    Ok(())
}

/// Value one company on one simulated path: `(fair_value_b, terminal_pv_b)`.
pub fn value_on_path(
    c: &Company,
    states: &[YearState],
    ctx: PathContext,
    dr_beta: f64,
    vp: &ValuationParams,
) -> (f64, f64) {
    let EarningsPath { path, rent, terminal_base } = earnings_with(c, states, vp);
    let tm = if ctx.fizzle {
        c.terminal_multiple * 0.75
    } else {
        c.terminal_multiple
    };
    let dr = scenario_discount(states, dr_beta);
    // Company-specific Taiwan-fab destruction in the invasion scenario — the
    // damage the global Silicon pool can't express (it marks scarcity UP).
    // Applied PER-YEAR from the invasion's start (re-audit #30): the discounted
    // 2027 flow is earned before the 2028 invasion and keeps full value; only
    // flows from the shock year on (and the terminal) carry the destruction.
    let (taiwan_factor, shock_year) = match ctx.taiwan_start {
        Some(y) if c.taiwan_fab_exposure > 0.0 => {
            ((1.0 - c.taiwan_fab_exposure * TAIWAN_FAB_LOSS).max(0.0), y)
        }
        _ => (1.0, i32::MAX),
    };
    let base_year = states[0].year;
    // Policy (export controls): surviving share of China-corridor earnings by the
    // i-th flow year; rungs in the observed 2026 are already in NTM consensus.
    let pg = vp.policy_gain.clamp(0.0, 1.0);
    let export_factor = |i: usize| -> f64 {
        if pg <= 0.0 || c.china_revenue_share <= 0.0 {
            return 1.0;
        }
        let years = i as i32 + 1;
        let survive = match &ctx.policy {
            None => (1.0 - EXPORT_RUNG_HAZARD * vp.export_rung_bite).powi(years),
            Some(d) => {
                let k: i32 = (1..=years).map(|t| d.rungs.get(t as usize).copied().unwrap_or(0) as i32).sum();
                (1.0 - vp.export_rung_bite).powi(k)
            }
        };
        1.0 - pg * c.china_revenue_share * (1.0 - survive)
    };
    // Flow PV over the explicit horizon.
    let mut fair: f64 = path
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let year = base_year + i as i32 + 1;
            let f = if year >= shock_year { taiwan_factor } else { 1.0 };
            let e = if pg > 0.0 { e * export_factor(i) } else { *e };
            e * f / (1.0 + dr).powi(i as i32 + 1)
        })
        .sum();
    // Terminal value with a RENT-DOMINANCE haircut (re-audit C2). Durable base
    // earnings get the full perpetuity multiple, but the capture RENT is not
    // perpetual — a fixed-capacity name's scarcity share erodes as supply responds
    // (CAPTURE_DECAY). The rent slice is a DECAYING perpetuity: multiple scaled by
    // dr/(dr+decay), CAPPED at 1/(dr+decay) — the value its own decay assumption
    // supports (re-audit #26: scaling tm alone still capitalized tm>1/dr names'
    // decaying rent above the self-consistent maximum). Terminal discounted at the
    // TERMINAL-year rate (re-audit #28) so the B11 coupling reaches the slice that
    // dominates PV. A name with no capture keeps the plain tm perpetuity.
    let mut terminal_pv = 0.0;
    if let (Some(&last), Some(&last_rent)) = (path.last(), rent.last()) {
        let dr_t = terminal_discount(states, dr_beta);
        let base_last = terminal_base.unwrap_or(last - last_rent).max(0.0);
        let rent_mult = (tm * dr_t / (dr_t + CAPTURE_DECAY)).min(1.0 / (dr_t + CAPTURE_DECAY));
        let mut terminal = (base_last * tm + last_rent.max(0.0) * rent_mult) * taiwan_factor;
        if pg > 0.0 {
            terminal *= export_factor(path.len() - 1);
        }
        terminal_pv = terminal / (1.0 + dr_t).powi(path.len() as i32);
        fair += terminal_pv;
    }
    if vp.leverage_gain > 0.0 && fair < 0.0 {
        // Limited liability: a levered equity is worth zero, not less.
        return (0.0, 0.0);
    }
    // Policy (investment access): a foreign holder forced out sells at a discount.
    if pg > 0.0 && c.foreign_access_risk > 0.0 {
        let p_ban = match &ctx.policy {
            None => {
                let mut survive = 1.0;
                for i in 0..path.len() {
                    let year = base_year + i as i32 + 1;
                    let h = if ctx.taiwan_start.map_or(false, |t| year >= t) {
                        ACCESS_BAN_HAZARD * ACCESS_BAN_TAIWAN_MULT
                    } else {
                        ACCESS_BAN_HAZARD
                    };
                    survive *= 1.0 - h;
                }
                1.0 - survive
            }
            Some(d) => d.access_ban_year.map_or(0.0, |_| 1.0),
        };
        let keep = 1.0 - pg * c.foreign_access_risk * vp.access_haircut * p_ban;
        fair *= keep;
        terminal_pv *= keep;
    }
    (fair, terminal_pv)
}

pub fn evaluate(
    c: &Company,
    scenario_states: &[(&'static str, Vec<YearState>)],
    dr_beta: f64,
) -> Evaluation {
    evaluate_with(c, scenario_states, dr_beta, &ValuationParams::default())
}

pub fn evaluate_with(
    c: &Company,
    scenario_states: &[(&'static str, Vec<YearState>)],
    dr_beta: f64,
    vp: &ValuationParams,
) -> Evaluation {
    let mut ev = evaluate_core(c, scenario_states, dr_beta, vp);
    let conv = evaluate_core(&counterfactual(c, true), scenario_states, dr_beta, vp).expected_upside;
    let with_drift =
        evaluate_core(&counterfactual(c, false), scenario_states, dr_beta, vp).expected_upside;
    ev.conv_upside = conv;
    ev.drift_delta = with_drift - conv;
    ev.ai_delta = ev.expected_upside - with_drift;
    ev
}

fn evaluate_core(
    c: &Company,
    scenario_states: &[(&'static str, Vec<YearState>)],
    dr_beta: f64,
    vp: &ValuationParams,
) -> Evaluation {
    let mut per: Vec<ScenarioValue> = Vec::new();
    for (name, states) in scenario_states {
        let (fair, terminal_pv) =
            value_on_path(c, states, PathContext::named(name), dr_beta, vp);
        per.push(ScenarioValue {
            scenario: name,
            fair_value_b: fair,
            upside: fair / c.mcap_b - 1.0,
            terminal_share: if fair.abs() > 1e-12 { terminal_pv / fair } else { 0.0 },
        });
    }
    assert!(!per.is_empty(), "evaluate() needs at least one scenario");
    let expected = SCENARIO_PROBS.iter().map(|(n, p)| {
        per.iter()
            .find(|v| v.scenario == *n)
            .unwrap_or_else(|| panic!("scenario {n} missing from states"))
            .upside * p
    }).sum();
    let prob = |n: &str| SCENARIO_PROBS.iter().find(|(k, _)| *k == n).map_or(0.0, |(_, p)| *p);
    let terminal_share = per.iter().map(|v| v.terminal_share * prob(v.scenario)).sum::<f64>()
        / per.iter().map(|v| prob(v.scenario)).sum::<f64>().max(1e-12);
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
        terminal_share,
        per_scenario: per,
        conv_upside: 0.0,
        drift_delta: 0.0,
        ai_delta: 0.0,
    }
}

pub fn evaluate_all(
    companies: &[Company],
    scenario_states: &[(&'static str, Vec<YearState>)],
    dr_beta: f64,
) -> Vec<Evaluation> {
    evaluate_all_with(companies, scenario_states, dr_beta, &ValuationParams::default())
}

pub fn evaluate_all_with(
    companies: &[Company],
    scenario_states: &[(&'static str, Vec<YearState>)],
    dr_beta: f64,
    vp: &ValuationParams,
) -> Vec<Evaluation> {
    let mut rows: Vec<Evaluation> = companies
        .iter()
        .filter(|c| c.ntm_earnings_b > 0.0)
        .map(|c| evaluate_with(c, scenario_states, dr_beta, vp))
        .collect();
    rows.sort_by(|a, b| b.expected_upside.partial_cmp(&a.expected_upside).unwrap());
    rows
}
