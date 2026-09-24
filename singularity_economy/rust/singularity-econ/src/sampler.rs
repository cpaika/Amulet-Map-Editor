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
