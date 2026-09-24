//! The five named scenarios (port of the retired `scenarios_v2.py`).

use crate::{simulate, Params, YearState};

/// First shock year of the `taiwan_shock` scenario (the 2028 quarantine below).
/// The valuation's per-year fab-destruction haircut keys on this so pre-invasion
/// flows keep full value (re-audit #30). Keep in sync with the scenario definition.
pub const TAIWAN_SHOCK_START: i32 = 2028;

/// The fizzle overrides: the singularity never arrives in the horizon. One
/// definition shared by the named `fizzle` scenario and the `book-mc` prior's
/// fizzle mass, so both mean the same world.
pub fn fizzle(mut p: Params) -> Params {
    p.singularity_year = 2099;
    p.robotics_year = 2099;
    p.algo_eff_growth_pre = 1.8;
    p.adoption_halflife = 4.0;
    p.max_displacement_rate = 0.05;
    // F2: without a singularity nothing sustains the 32%/yr capex-desire engine.
    // Demand growth base 0.10 with weak herding; 2026 perception pinned to the
    // observed year. Legacy fizzle kept binding POWER through 2036 while its AI
    // revenue was flat, so longs showed gains in the no-AI world.
    p.demand_growth_base = 0.10;
    p.momentum_gain = 0.2;
    p.perceived_growth_2026 = Some(0.32);
    p
}

pub fn scenario_params() -> Vec<(&'static str, Params)> {
    vec![
        ("baseline", Params::default()),
        ("fast_takeoff", Params {
            singularity_boost: 3.0,
            adoption_halflife: 1.0,
            max_displacement_rate: 0.30,
            chip_growth_ceiling: 1.0,
            power_growth_ceiling: 0.55,
            component_growth_ceiling: 2.0,
            momentum_gain: 1.0,
            capex_gdp_cap: 0.08,
            ..Params::default()
        }),
        ("delayed", Params {
            singularity_year: 2029,
            robotics_year: 2030,
            adoption_halflife: 2.5,
            max_displacement_rate: 0.12,
            ..Params::default()
        }),
        ("friction", Params {
            adoption_halflife: 3.5,
            max_displacement_rate: 0.08,
            addressable_cognitive: 0.55,
            backlash_gain: 4.0,
            capex_gdp_cap: 0.035,
            ..Params::default()
        }),
        ("fizzle", fizzle(Params::default())),
        // Red-team round 3: the geopolitics layer must be reachable from
        // the trade book. Median severe path from the escalation ladder:
        // a 2028 quarantine that escalates to a 2029 blockade with the
        // near-certain minerals embargo, plus the standing energy shock.
        ("taiwan_shock", Params {
            geo_shocks: vec![
                crate::GeoShock {
                    kind: crate::ShockKind::TaiwanQuarantine,
                    start_year: 2028,
                    duration_years: 0.75,
                },
                crate::GeoShock {
                    kind: crate::ShockKind::TaiwanBlockade,
                    start_year: 2029,
                    duration_years: 1.0,
                },
                crate::GeoShock {
                    kind: crate::ShockKind::MineralsEmbargo,
                    start_year: 2029,
                    duration_years: 1.5,
                },
                crate::GeoShock {
                    kind: crate::ShockKind::EnergyChokepoint,
                    start_year: 2028,
                    duration_years: 0.75,
                },
            ],
            ..Params::default()
        }),
    ]
}

pub fn scenario_states() -> Vec<(&'static str, Vec<YearState>)> {
    scenario_params()
        .into_iter()
        .map(|(name, p)| (name, simulate(&p)))
        .collect()
}

/// The coherent "enhanced-realism" enable set (output/history/gated_dynamics_menu.md).
/// Turns the gated satellites on TOGETHER — the reflexive AI-capex spine + wealth
/// effect (fatter left tails), transmission-delivery lag (firmer power-rent), wage
/// compression (deeper wage-linked shorts), and a partial job-guarantee tilt (lower
/// sovereign rate). Every gain is a hypothesis, not the shipped baseline; this is the
/// review lens, not the default. Applied on top of whatever a scenario already sets.
pub fn enhanced_realism(mut p: Params) -> Params {
    p.equity_sentiment_gain = 1.0;
    p.wealth_effect_gain = 1.0;
    p.transmission_gain = 0.5;
    p.wage_compression_cog_gain = 0.4;
    p.wage_compression_phys_gain = 0.4;
    p.society.jg_share = 0.3;
    p
}

pub fn scenario_params_enhanced() -> Vec<(&'static str, Params)> {
    scenario_params()
        .into_iter()
        .map(|(name, p)| (name, enhanced_realism(p)))
        .collect()
}

pub fn scenario_states_enhanced() -> Vec<(&'static str, Vec<YearState>)> {
    scenario_params_enhanced()
        .into_iter()
        .map(|(name, p)| (name, simulate(&p)))
        .collect()
}

/// "First-principles v2" — the most complete first-principles configuration: the
/// enhanced-realism dynamics PLUS the four supply/cost/pricing mechanisms grounded from
/// first principles this cycle. Compute cost rides Wright's law (cumulative-volume
/// learning, not calendar); investment rides a Tobin's-q return-on-capital brake with an
/// endogenous (sovereign-rate + premium + depreciation) hurdle; merchant power prices
/// two-sided (crashes in a glut); and AI-provider rent commoditizes as the market
/// matures. Net effect vs enhanced-realism: the q-brake tempers the reflexive bubble
/// (investment discipline offsets animal spirits — a deliberately-included interaction),
/// while commoditization + two-sided power deepen the AI-services and merchant-power
/// downside. Still a review lens of hypotheses, not the shipped default.
pub fn first_principles_v2(p: Params) -> Params {
    let mut p = enhanced_realism(p);
    p.wright_gain = 1.0;             // compute cost tied to cumulative volume
    p.q_governor_gain = 0.5;         // return-on-capital investment brake
    p.power_glut_price_gain = 0.6;   // two-sided merchant power price
    p.ai_commoditization_gain = 0.5; // AI-provider rent competes away as adoption saturates
    p.dgb_decay_rate = 0.15;         // capex-desire growth base converges to GDP (tau ~6.7y)
    p.q_forward = true;              // q on expected, not trailing, AI profit (F5)
    p.q_mult_cap = 1.25;             // optimism accelerates investment only modestly
    p.smooth_rents = 1.0;            // saturating rents, no hard ceiling kinks (F2)
    p.vintage_power_draw = 1.0;      // racked GPUs keep their build-year draw (F7a)
    p
}

pub fn scenario_states_v2() -> Vec<(&'static str, Vec<YearState>)> {
    scenario_params()
        .into_iter()
        .map(|(name, p)| (name, simulate(&first_principles_v2(p))))
        .collect()
}

/// Structural-lens blend: every gain `first_principles_v2` sets, moved a fraction
/// `lambda` of the way from `p`'s value toward the v2 value (0 = baseline
/// structure, 1 = full v2). `book-mc --lens mix` samples lambda ~ U(0,1), so the
/// choice between the review lenses is priced as uncertainty rather than picked.
pub fn lens_blend(p: Params, lambda: f64) -> Params {
    let v2 = first_principles_v2(p.clone());
    let lerp = |a: f64, b: f64| a + lambda * (b - a);
    let mut q = p;
    q.equity_sentiment_gain = lerp(q.equity_sentiment_gain, v2.equity_sentiment_gain);
    q.wealth_effect_gain = lerp(q.wealth_effect_gain, v2.wealth_effect_gain);
    q.transmission_gain = lerp(q.transmission_gain, v2.transmission_gain);
    q.wage_compression_cog_gain = lerp(q.wage_compression_cog_gain, v2.wage_compression_cog_gain);
    q.wage_compression_phys_gain = lerp(q.wage_compression_phys_gain, v2.wage_compression_phys_gain);
    q.society.jg_share = lerp(q.society.jg_share, v2.society.jg_share);
    q.wright_gain = lerp(q.wright_gain, v2.wright_gain);
    q.q_governor_gain = lerp(q.q_governor_gain, v2.q_governor_gain);
    q.power_glut_price_gain = lerp(q.power_glut_price_gain, v2.power_glut_price_gain);
    q.ai_commoditization_gain = lerp(q.ai_commoditization_gain, v2.ai_commoditization_gain);
    q.dgb_decay_rate = lerp(q.dgb_decay_rate, v2.dgb_decay_rate);
    q.smooth_rents = lerp(q.smooth_rents, v2.smooth_rents);
    q.vintage_power_draw = lerp(q.vintage_power_draw, v2.vintage_power_draw);
    // Form choices, not intensities: inert while q_governor_gain is 0 (lambda = 0).
    q.q_forward = v2.q_forward;
    q.q_mult_cap = v2.q_mult_cap;
    q
}
