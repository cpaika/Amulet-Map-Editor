//! Monte Carlo book (`book-mc`) contract: the path-level valuation, the path
//! context derived from sampled params, the structural-lens blend and the book
//! prior. Cargo-only (feature `mc`) where the sampler is involved.

use singularity_econ::companies::universe;
use singularity_econ::scenarios::{
    first_principles_v2, fizzle, lens_blend, scenario_params, scenario_states,
};
use singularity_econ::valuation::{
    base_year_consistent, evaluate, value_on_path, PathContext, ValuationParams,
};
use singularity_econ::{simulate, Params};

const DR_BETA: f64 = 0.60;

// Deriving the context from a named scenario's PARAMS must agree with deriving it
// from its NAME — otherwise the MC book and the named book value the same world
// differently.
#[test]
fn path_context_from_params_matches_named_scenarios() {
    // Policy differs by design: named books price it in expectation, sampled paths
    // carry realized draws. The scenario-defining fields must agree.
    for (name, p) in scenario_params() {
        let (a, b) = (PathContext::from_params(&p), PathContext::named(name));
        assert_eq!((a.fizzle, a.taiwan_start), (b.fizzle, b.taiwan_start),
                   "context mismatch for named scenario {name}");
        assert!(a.policy.is_some() && b.policy.is_none());
    }
}

#[test]
fn value_on_path_is_the_named_evaluation() {
    let states = scenario_states();
    for c in universe().iter().filter(|c| c.ntm_earnings_b > 0.0) {
        let ev = evaluate(c, &states, DR_BETA);
        for (name, s) in &states {
            let (fair, _) = value_on_path(c, s, PathContext::named(name), DR_BETA, &ValuationParams::default());
            let named = ev.per_scenario.iter().find(|v| v.scenario == *name).unwrap();
            assert_eq!(fair.to_bits(), named.fair_value_b.to_bits(), "{} {name}", c.ticker);
        }
    }
}

#[test]
fn fizzle_helper_is_the_named_fizzle() {
    let named = scenario_params().into_iter().find(|(n, _)| *n == "fizzle").unwrap().1;
    assert_eq!(
        format!("{:?}", simulate(&named)),
        format!("{:?}", simulate(&fizzle(Params::default())))
    );
}

#[test]
fn lens_blend_endpoints_are_base_and_v2() {
    let base = Params::default();
    assert_eq!(
        format!("{:?}", simulate(&lens_blend(base.clone(), 0.0))),
        format!("{:?}", simulate(&base))
    );
    assert_eq!(
        format!("{:?}", simulate(&lens_blend(base.clone(), 1.0))),
        format!("{:?}", simulate(&first_principles_v2(base)))
    );
}

// Every named scenario shares the calibrated 2026 base year, so every one must
// pass the base-year check the MC book conditions on.
#[test]
fn named_scenarios_have_a_consistent_base_year() {
    for (name, s) in scenario_states() {
        base_year_consistent(&s).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
}

#[test]
fn base_year_check_rejects_a_counterfactual_2026() {
    // Power-starved 2026: power, not chips, binds — contradicts the observed year.
    let p = Params { ai_power_2026: 20.0, power_additions_2026: 8.0, ..Params::default() };
    assert!(base_year_consistent(&simulate(&p)).is_err());
}

#[cfg(feature = "mc")]
mod prior {
    use super::*;
    use singularity_econ::sampler::{fizzle_mass, BookSampler, Lens, Sampler};

    // The aux stream (fizzle flag, lens fraction) must not perturb the core draw:
    // a non-fizzle book draw under the Base lens is exactly the mc/sa draw.
    #[test]
    fn book_prior_keeps_the_core_parameter_stream() {
        let mut core = Sampler::new(21);
        let mut book = BookSampler::new(21, Lens::Base);
        for _ in 0..200 {
            let a = core.params();
            let d = book.draw();
            assert_eq!(a.singularity_boost.to_bits(), d.params.singularity_boost.to_bits());
            assert_eq!(a.jevons_elasticity.to_bits(), d.params.jevons_elasticity.to_bits());
            if !d.fizzle {
                assert_eq!(a.singularity_year, d.params.singularity_year);
            }
        }
    }

    #[test]
    fn book_prior_carries_the_named_fizzle_mass() {
        let mut book = BookSampler::new(5, Lens::Mix);
        let n = 4000;
        let draws: Vec<_> = (0..n).map(|_| book.draw()).collect();
        let f = draws.iter().filter(|d| d.fizzle).count() as f64 / n as f64;
        assert!((f - fizzle_mass()).abs() < 0.02, "fizzle share {f} vs {}", fizzle_mass());
        assert!(draws.iter().filter(|d| d.fizzle)
            .all(|d| PathContext::from_params(&d.params).fizzle));
        let mean_l = draws.iter().map(|d| d.lambda).sum::<f64>() / n as f64;
        assert!((mean_l - 0.5).abs() < 0.03, "lens fraction should be ~U(0,1), mean {mean_l}");
    }

    // Finding lock (Sep-26 book-mc): the displacement/services shorts lose on
    // essentially every sampled path in every structural lens — their verdict does
    // not depend on the lens hypotheses, unlike the silicon longs. If this fails, the
    // short book became path- or lens-dependent: re-read book-mc before trading it.
    #[test]
    fn displacement_shorts_are_robust_across_paths_and_lenses() {
        let comps = universe();
        let shorts = ["RHI", "MAN", "CHRW", "ADP", "PAYX"];
        let mut book = BookSampler::new(7, Lens::Mix);
        let n = 300;
        let mut losses = vec![0usize; shorts.len()];
        for _ in 0..n {
            let d = book.draw();
            let states = simulate(&d.params);
            let ctx = PathContext::from_params(&d.params);
            for (i, t) in shorts.iter().enumerate() {
                let c = comps.iter().find(|c| c.ticker == *t).unwrap();
                let (fair, _) = value_on_path(c, &states, ctx, d.params.macrofin.dr_beta, &ValuationParams::default());
                losses[i] += (fair < c.mcap_b) as usize;
            }
        }
        for (i, t) in shorts.iter().enumerate() {
            let p = losses[i] as f64 / n as f64;
            assert!(p > 0.95, "{t}: P(loss) {p} — the short is no longer path-robust");
        }
    }
}

// Review fix (Sep 24): the fizzle overrides act from 2027 — the observed 2026 base year
// of a fizzle path is identical to the same draw without fizzle, so conditioning on
// 2026 cannot thin out the fizzle mass.
#[test]
fn fizzle_leaves_the_observed_2026_untouched() {
    let b = simulate(&Params::default());
    let f = simulate(&fizzle(Params::default()));
    assert_eq!(b[0].ai_capex, f[0].ai_capex);
    assert_eq!(b[0].binding, f[0].binding);
}

// Review fix (Sep 24): the valuation discounts REAL flows, so a pure rise in expected
// inflation (long rate and inflation premium up together) leaves every unlevered value
// unchanged. (With leverage on, inflation correctly shifts value from net-cash holders
// to net debtors until the balance reprices.)
#[test]
fn valuation_is_invariant_to_pure_expected_inflation() {
    let vp = ValuationParams { leverage_gain: 0.0, ..ValuationParams::first_principles() };
    let base = simulate(&Params::default());
    let mut inflated = base.clone();
    for s in inflated.iter_mut().skip(1) {
        s.long_rate += 0.03;
        s.infl_premium += 0.03;
    }
    for c in universe().iter().filter(|c| c.ntm_earnings_b > 0.0) {
        let a = value_on_path(c, &base, PathContext::default(), DR_BETA, &vp).0;
        let b = value_on_path(c, &inflated, PathContext::default(), DR_BETA, &vp).0;
        assert!((a - b).abs() < 1e-9 * a.abs().max(1.0), "{}: {a} vs {b}", c.ticker);
    }
}

// Review fix (Sep 24): the grid flow terminal sustains the INSTALLED equipment base,
// not the compute draw; in a demand bust the installed grid far exceeds the draw.
#[test]
fn installed_grid_exceeds_draw_in_a_bust() {
    let s = simulate(&first_principles_v2(fizzle(Params::default())));
    let last = s.last().unwrap();
    assert!(last.ai_power_installed_gw > 5.0 * last.ai_power_gw,
            "installed {} vs draw {}", last.ai_power_installed_gw, last.ai_power_gw);
    assert!(s.windows(2).all(|w| w[1].ai_power_installed_gw >= w[0].ai_power_installed_gw));
}
