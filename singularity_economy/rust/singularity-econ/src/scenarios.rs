//! The five named scenarios (port of the retired `scenarios_v2.py`).

use crate::{simulate, Params, YearState};

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
        ("fizzle", Params {
            singularity_year: 2099,
            robotics_year: 2099,
            algo_eff_growth_pre: 1.8,
            adoption_halflife: 4.0,
            max_displacement_rate: 0.05,
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
