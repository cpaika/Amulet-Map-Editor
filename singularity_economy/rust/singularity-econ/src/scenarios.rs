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
