//! Property-based tests: invariants must hold over the whole parameter space,
//! not just the calibrated point.

use proptest::prelude::*;
use singularity_econ::{simulate, Params};

fn arb_params() -> impl Strategy<Value = Params> {
    (
        2027..=2031i32,                 // singularity year
        1..=2i32,                       // robotics gap
        0.10f64..0.30,                  // max displacement rate
        0.2f64..3.0,                    // chip supply gain
        0.2f64..1.2,                    // power supply gain
        0.2f64..1.2,                    // momentum gain
        0.25f64..0.8,                   // internal funding share
        35.0f64..90.0,                  // robot cost 2028
        0.5f64..4.0,                    // backlash gain
        1.3f64..3.0,                    // singularity boost
    )
        .prop_map(|(sing, gap, mdr, csg, psg, mg, ifs, rc, bg, sb)| Params {
            singularity_year: sing,
            robotics_year: sing + gap,
            max_displacement_rate: mdr,
            chip_supply_gain: csg,
            power_supply_gain: psg,
            momentum_gain: mg,
            internal_funding_share: ifs,
            robot_cost_2028_k: rc,
            backlash_gain: bg,
            singularity_boost: sb,
            ..Params::default()
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn invariants_hold_everywhere(p in arb_params()) {
        let states = simulate(&p);
        prop_assert_eq!(states.len() as i32, p.end_year - p.start_year + 1);
        let mut prev_disp = -1.0f64;
        let mut prev_pd = -1.0f64;
        for s in &states {
            // no NaNs or infinities anywhere important
            for v in [s.ai_capex, s.compute_stock, s.gdp, s.sector_debt,
                      s.robot_fleet_m, s.robot_cost_k, s.silicon_margin,
                      s.power_margin, s.component_margin, s.overshoot_ratio] {
                prop_assert!(v.is_finite(), "non-finite at {}", s.year);
            }
            // non-negativity
            prop_assert!(s.ai_capex >= 0.0);
            prop_assert!(s.robot_fleet_m >= 0.0);
            prop_assert!(s.sector_debt >= 0.0);
            prop_assert!(s.gdp > 0.0);
            // displacement ratchets, bounded
            prop_assert!(s.cog_displacement >= prev_disp);
            prop_assert!(s.cog_displacement <= 1.0);
            prev_disp = s.cog_displacement;
            prop_assert!(s.phys_displacement >= prev_pd);
            prop_assert!(s.phys_displacement <= p.physical_addressable + 1e-9);
            prev_pd = s.phys_displacement;
            // margins bounded by ceiling
            prop_assert!(s.silicon_margin <= p.margin_ceiling + 1e-9);
            prop_assert!(s.power_margin <= p.margin_ceiling + 1e-9);
            prop_assert!(s.component_margin <= p.margin_ceiling + 1e-9);
            // credit multiplier in (0, 1]
            prop_assert!(s.credit_multiplier > 0.0 && s.credit_multiplier <= 1.0);
            // robot cost respects floor and never rises
            prop_assert!(s.robot_cost_k >= p.robot_cost_floor_k - 1e-9);
        }
        // robot cost monotone non-increasing once production starts
        let costs: Vec<f64> = states.iter()
            .filter(|s| s.robot_prod_m > 0.0).map(|s| s.robot_cost_k).collect();
        for w in costs.windows(2) {
            prop_assert!(w[1] <= w[0] + 1e-9);
        }
    }

    #[test]
    fn capex_respects_capital_cap(p in arb_params()) {
        let states = simulate(&p);
        let mut prev_gdp = p.world_gdp;
        for s in &states {
            prop_assert!(s.ai_capex <= prev_gdp * p.capex_gdp_cap + 1e-9,
                         "capex {} above cap at {}", s.ai_capex, s.year);
            prev_gdp = s.gdp;
        }
    }

    #[test]
    fn stronger_supply_response_never_lengthens_silicon_rents(
        base in arb_params()) {
        let weak = Params { chip_supply_gain: 0.3, ..base.clone() };
        let strong = Params { chip_supply_gain: 3.0, ..base };
        let norm_year = |states: &[singularity_econ::YearState], sing: i32| {
            states.iter()
                .find(|s| s.year > sing && s.silicon_margin <= 0.24)
                .map_or(9999, |s| s.year)
        };
        let wy = norm_year(&simulate(&weak), weak.singularity_year);
        let sy = norm_year(&simulate(&strong), strong.singularity_year);
        prop_assert!(sy <= wy, "strong gain normalized later ({sy}) than weak ({wy})");
    }
}
