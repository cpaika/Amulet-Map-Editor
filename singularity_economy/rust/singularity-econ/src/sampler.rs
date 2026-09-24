//! Monte Carlo parameter sampler, shared by the `mc`, `sa` and `book-mc` CLI
//! commands. Moved verbatim from main.rs (Sep-26 re-analysis F4) so every
//! consumer draws from one prior; seeded output is byte-identical to the
//! pre-move CLI. Cargo-only (feature `mc`): the Buck build is third-party-free.

use crate::Params;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct Sampler {
    rng: ChaCha8Rng,
}

impl Sampler {
    pub fn new(seed: u64) -> Self {
        Sampler { rng: ChaCha8Rng::seed_from_u64(seed) }
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

    /// Draw one full parameter set. Field order in the struct literal below IS the
    /// RNG draw order — reordering fields changes every seeded mc/sa result.
    pub fn params(&mut self) -> Params {
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
                let mut grng = crate::GeoRng::new(
                    (self.uniform(0.0, 1.0) * u64::MAX as f64) as u64,
                );
                crate::geopolitics::sample_shocks(&mut grng, 2026, 2036)
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
            // Bio/cyber layer LIVE in production MC (re-audit #21: the Params doc
            // calls dread_shocks "MC-drawn", but no draw ever populated them and
            // bio_layer stayed 0 — the entire dread-shock class and bio pools were
            // dead in every mc/sa run). Capability tracks are simple ramps: bio/cyber
            // operational capability rises through the horizon; defense lags offense.
            bio: crate::BioParams {
                bio_layer: 1.0,
                ..crate::BioParams::default()
            },
            dread_shocks: {
                let mut brng = crate::GeoRng::new(
                    (self.uniform(0.0, 1.0) * u64::MAX as f64) as u64,
                );
                let ramp = |y: i32| (((y - 2026) as f64) * 0.09).clamp(0.0, 1.0);
                crate::bio::shocks::sample_dread_shocks(
                    &mut brng,
                    &crate::BioParams::default(),
                    2026,
                    2036,
                    ramp,
                    |_| 0.35,
                    ramp,
                    |y| (0.40 + 0.05 * (y - 2026) as f64).min(0.9),
                )
            },
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

/// Every parameter the sampler varies, read back from a drawn `Params`: the
/// single list of what the Monte Carlo prior covers. `sa` ranks all of these
/// (it used to hard-code 17 of them and hid drivers the sampler already drew,
/// e.g. jevons_elasticity and the efficiency jump). Keep in sync with
/// `Sampler::params` — `tests/sampler.rs` fails if a listed field stops varying.
pub fn sampled_values(p: &Params) -> Vec<(&'static str, f64)> {
    vec![
        ("singularity_year", p.singularity_year as f64),
        ("robot_gap_years", (p.robotics_year - p.singularity_year) as f64),
        ("singularity_boost", p.singularity_boost),
        ("algo_eff_growth_pre", p.algo_eff_growth_pre),
        ("algo_eff_growth_post", p.algo_eff_growth_post),
        ("adoption_halflife", p.adoption_halflife),
        ("max_displacement_rate", p.max_displacement_rate),
        ("addressable_cognitive", p.addressable_cognitive),
        ("cognitive_demand_elasticity", p.cognitive_demand_elasticity),
        ("chip_base_growth", p.chip_base_growth),
        ("chip_supply_gain", p.chip_supply_gain),
        ("chip_growth_ceiling", p.chip_growth_ceiling),
        ("power_base_growth", p.power_base_growth),
        ("power_supply_gain", p.power_supply_gain),
        ("power_growth_ceiling", p.power_growth_ceiling),
        ("capex_gdp_cap", p.capex_gdp_cap),
        ("component_base_growth", p.component_base_growth),
        ("component_supply_gain", p.component_supply_gain),
        ("component_growth_ceiling", p.component_growth_ceiling),
        ("bootstrap_gain", p.bootstrap_gain),
        ("robot_learning_rate", p.robot_learning_rate),
        ("robot_cost_2028_k", p.robot_cost_2028_k),
        ("it_services_beta", p.it_services_beta),
        ("bpo_beta", p.bpo_beta),
        ("saas_beta", p.saas_beta),
        ("prof_info_beta", p.prof_info_beta),
        ("power_efficiency_gain", p.power_efficiency_gain),
        ("transition_drag", p.transition_drag),
        ("productivity_passthrough", p.productivity_passthrough),
        ("internal_funding_share", p.internal_funding_share),
        ("momentum_gain", p.momentum_gain),
        ("backlash_gain", p.backlash_gain),
        ("afford_gain", p.afford_gain),
        ("asi_diffusion_years", p.asi_diffusion_years),
        ("asi_delay_compression", p.asi_delay_compression),
        ("asi_ceiling_boost", p.asi_ceiling_boost),
        ("asi_integration_relief", p.asi_integration_relief),
        ("geo_shock_count", p.geo_shocks.len() as f64),
        ("incident_year", p.incident_year as f64),
        ("incident_dread", if p.incident_dread { 1.0 } else { 0.0 }),
        ("dread_shock_count", p.dread_shocks.len() as f64),
        ("efficiency_jump_year", p.efficiency_jump_year as f64),
        ("efficiency_jump_size", p.efficiency_jump_size),
        ("jevons_elasticity", p.jevons_elasticity),
    ]
}

/// Structural lens applied on top of each sampled path in `book-mc`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lens {
    Base,
    Enhanced,
    V2,
    /// lambda ~ U(0,1) blend from baseline structure to v2 (`scenarios::lens_blend`).
    Mix,
}

impl Lens {
    pub fn parse(s: &str) -> Option<Lens> {
        match s {
            "base" | "baseline" => Some(Lens::Base),
            "enhanced" => Some(Lens::Enhanced),
            "v2" => Some(Lens::V2),
            "mix" => Some(Lens::Mix),
            _ => None,
        }
    }
}

/// Probability mass of the no-singularity world in the book prior — the named
/// book's fizzle weight. The mc/sa prior draws the singularity in 2027-30 with
/// certainty, so it cannot price the scenario the named book weights at 9.3%.
pub fn fizzle_mass() -> f64 {
    crate::valuation::SCENARIO_PROBS
        .iter()
        .find(|(n, _)| *n == "fizzle")
        .map_or(0.0, |(_, p)| *p)
}

/// One `book-mc` draw: the sampled parameters (lens applied), the lens fraction
/// used (0 for Base, 1 for V2, the draw for Mix; Enhanced reports 0.5 as a label
/// only) and whether the path is a fizzle.
pub struct BookDraw {
    pub params: Params,
    /// The core mc/sa draw before fizzle and the lens were applied.
    pub core: Params,
    pub lambda: f64,
    pub fizzle: bool,
}

/// The `book-mc` prior: the mc/sa parameter prior plus fizzle mass and the
/// structural lens. The extra draws come from a SEPARATE stream so the core
/// parameter sequence for a given seed is the same one `mc`/`sa` see.
pub struct BookSampler {
    core: Sampler,
    aux: ChaCha8Rng,
    lens: Lens,
}

impl BookSampler {
    pub fn new(seed: u64, lens: Lens) -> Self {
        BookSampler {
            core: Sampler::new(seed),
            aux: ChaCha8Rng::seed_from_u64(seed ^ 0x9E37_79B9_7F4A_7C15),
            lens,
        }
    }

    pub fn draw(&mut self) -> BookDraw {
        let p = self.core.params();
        let core = p.clone();
        let u_fizzle: f64 = Uniform::new(0.0, 1.0).sample(&mut self.aux);
        let u_lambda: f64 = Uniform::new(0.0, 1.0).sample(&mut self.aux);
        // Forward-q growth seed (F5): observed 2026 AI-revenue growth +100-250%/yr.
        // Only read when the lens turns on the forward q-governor.
        let rev_growth: f64 = Uniform::new(1.0, 2.5).sample(&mut self.aux);
        // GW-per-capex-dollar trend (F3): legacy implies +3.5%/yr; Hopper->Blackwell
        // ran strongly negative; a constant chip-cost/chip-power ratio gives ~0.
        let gw_trend: f64 = Uniform::new(-0.10, 0.035).sample(&mut self.aux);
        // Inference demand elasticity (F5): clearing is structural; epsilon is the
        // uncertainty (0.7 = revenue shrinks as price falls, 1.4 = Jevons-like).
        let epsilon: f64 = Uniform::new(0.7, 1.4).sample(&mut self.aux);
        let fizzle = u_fizzle < fizzle_mass();
        let mut p = if fizzle { crate::scenarios::fizzle(p) } else { p };
        p.ai_rev_growth_2026 = rev_growth;
        p.gw_per_dollar_growth = Some(gw_trend);
        p.inference_clearing_gain = 1.0;
        p.inference_elasticity = epsilon;
        let (params, lambda) = match self.lens {
            Lens::Base => (p, 0.0),
            Lens::Enhanced => (crate::scenarios::enhanced_realism(p), 0.5),
            Lens::V2 => (crate::scenarios::first_principles_v2(p), 1.0),
            Lens::Mix => (crate::scenarios::lens_blend(p, u_lambda), u_lambda),
        };
        BookDraw { params, core, lambda, fizzle }
    }
}
