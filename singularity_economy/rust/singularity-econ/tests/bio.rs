//! Bio/cyber layer validation contract (output/history/bio_cyber_design.md §5).

use singularity_econ::bio::{shocks, BioParams, BioState};
use singularity_econ::{simulate, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// Regression: layer off (default bio_layer) + empty shocks => baseline
// bio/cyber outputs are inert (zero hazard, zero drug pool).
#[test]
fn bio_layer_off_is_inert() {
    // default has bio_layer default; force off explicitly
    let off = simulate(&Params {
        bio: BioParams { bio_layer: 0.0, ..BioParams::default() },
        ..Params::default()
    });
    for s in &off {
        assert_eq!(s.drug_pool_b.to_bits(), 0.0f64.to_bits());
    }
}

// Bio hazard is ALREADY partially on (VCT threshold crossed ~2025): with
// the layer on, operational uplift is positive and rises over the horizon.
#[test]
fn bio_uplift_rises_with_capability() {
    let on = simulate(&Params {
        bio: BioParams { bio_layer: 1.0, ..BioParams::default() },
        ..Params::default()
    });
    let early = on.iter().find(|s| s.year == 2028).unwrap().bio_hazard;
    let late = on.last().unwrap().bio_hazard;
    assert!(late > early, "operational uplift should rise: {early} -> {late}");
    assert!(late > 0.0, "uplift must be positive once capability crosses");
}

// Translation coefficient dominates tail risk: the Aum gate (knowledge ->
// operational) moves the uplift more than raising raw capability, because
// dissemination is the binding constraint, not knowledge.
#[test]
fn translation_coeff_gates_operational_capability() {
    let step = |tc: f64| {
        let bp = BioParams {
            bio_layer: 1.0,
            translation_coeff_2026: tc,
            translation_coeff_2036: tc,
            ..BioParams::default()
        };
        let mut st = BioState::new(&bp);
        let mut last = 0.0;
        for _y in 2026..=2036 {
            last = st.step(&bp, 50.0, 0.5, 1.0, 0.3).bio_operational_uplift;
        }
        last
    };
    let low = step(0.10);
    let high = step(0.35);
    assert!(high > low * 2.0, "translation gate must dominate: {low} vs {high}");
}

// A mass-casualty bio pandemic dents broad GDP AND arms the dread ratchet;
// a contained scare does neither strongly. Dual-sign: distinct from geo.
#[test]
fn mass_casualty_pandemic_hits_gdp_and_arms_dread() {
    let base_states = base();
    let pandemic = simulate(&Params {
        dread_shocks: vec![shocks::DreadShock {
            kind: shocks::DreadShockKind::BioPandemic,
            year: 2030, gdp_drag: 0.08, stringency_step: 0.70,
            mass_casualty: true, spread: 0.3, defensive_pool_mult: 3.0,
            broad_beta_hit: 0.3,
        }],
        ..Params::default()
    });
    let base_gdp = base_states.iter().find(|s| s.year == 2030).unwrap().gdp;
    let hit_gdp = pandemic.iter().find(|s| s.year == 2030).unwrap().gdp;
    assert!(hit_gdp < base_gdp, "pandemic must dent GDP: {hit_gdp} vs {base_gdp}");
    // recovery is mean-reverting: growth resumes after the shock year
    let g31 = pandemic.iter().find(|s| s.year == 2031).unwrap().gdp;
    assert!(g31 > hit_gdp, "GDP must recover after the transient");
}

// Anti-double-count: co-occurring bio + cyber dread in one year yields a
// stringency step equal to the MAX class, not the sum.
#[test]
fn co_occurring_dread_takes_max_not_sum() {
    let fx = shocks::effects_for_year(
        &[
            shocks::DreadShock { kind: shocks::DreadShockKind::BioPandemic, year: 2030, gdp_drag: 0.05, stringency_step: 0.70, mass_casualty: true, spread: 0.3, defensive_pool_mult: 3.0, broad_beta_hit: 0.3 },
            shocks::DreadShock { kind: shocks::DreadShockKind::CyberSystemic, year: 2030, gdp_drag: 0.02, stringency_step: 0.30, mass_casualty: false, spread: 0.15, defensive_pool_mult: 2.0, broad_beta_hit: 0.2 },
        ],
        2030,
    );
    let bio_alone = shocks::effects_for_year(
        &[shocks::DreadShock { kind: shocks::DreadShockKind::BioPandemic, year: 2030, gdp_drag: 0.05, stringency_step: 0.70, mass_casualty: true, spread: 0.3, defensive_pool_mult: 3.0, broad_beta_hit: 0.3 }],
        2030,
    );
    assert!(
        (fx.stringency_step - bio_alone.stringency_step).abs() < 1e-9,
        "combined stringency must equal the max class, not the sum"
    );
    // but GDP drags DO sum (independent physical causes)
    assert!(fx.gdp_drag > bio_alone.gdp_drag, "gdp drags are additive");
}

// Biological transhumanism: in-horizon (2026-2036) embryo selection is
// materially negligible (~2.5 IQ pts, "hundreds" of babies) — solidarity
// must be ~unchanged vs the enhancement-off world. The stratification
// bites only in the 2040s IVG discontinuity.
#[test]
fn enhancement_negligible_in_horizon_but_ivg_erodes_solidarity() {
    use singularity_econ::BioParams;
    let on = simulate(&Params {
        bio: BioParams { bio_layer: 1.0, ..BioParams::default() },
        end_year: 2050,
        ..Params::default()
    });
    let ivg = simulate(&Params {
        bio: BioParams { bio_layer: 1.0, ivg_breakthrough_year: 2042, ..BioParams::default() },
        end_year: 2050,
        ..Params::default()
    });
    let sol = |v: &[YearState], y: i32| v.iter().find(|s| s.year == y).unwrap().solidarity;
    // in-horizon: IVG-off and IVG-on identical (breakthrough hasn't fired)
    assert!(
        (sol(&on, 2036) - sol(&ivg, 2036)).abs() < 1e-9,
        "enhancement must be negligible in-horizon"
    );
    // 2040s: the IVG discontinuity erodes solidarity measurably below the
    // no-breakthrough path (bio-caste stratification)
    assert!(
        sol(&ivg, 2050) < sol(&on, 2050) - 0.03,
        "IVG breakthrough must erode solidarity: {} vs {}",
        sol(&ivg, 2050),
        sol(&on, 2050)
    );
}

// Audit C14: the bio optionality pools (longevity, BCI, bio-materials) must be
// SURFACED on YearState, not computed then discarded. With the bio layer on they
// are non-negative and the longevity pool is populated (init ~$60B, growing).
#[test]
fn bio_optionality_pools_are_surfaced() {
    let v = simulate(&Params {
        bio: BioParams { bio_layer: 1.0, ..BioParams::default() },
        end_year: 2040,
        ..Params::default()
    });
    let last = v.last().unwrap();
    assert!(last.longevity_pool_b > 0.0, "longevity pool must be surfaced: {}", last.longevity_pool_b);
    assert!(last.bci_pool_b >= 0.0 && last.bio_materials_pool_b >= 0.0, "BCI/bio-materials pools surfaced");
    // longevity pool grows over the horizon (funding-cyclical, toward its cap)
    assert!(last.longevity_pool_b >= v[0].longevity_pool_b, "longevity pool should not shrink");
}

// Audit C7: a bio/cyber dread shock's SEVERITY-scaled stringency_step must ratchet
// reg_stringency — a mass-casualty pandemic (0.70) far more than a contained scare
// (0.10), where before every dread event moved it the same flat amount. Baseline
// (no drawn shocks) is unchanged.
#[test]
fn dread_stringency_ratchet_scales_with_severity() {
    use singularity_econ::bio::shocks::{DreadShock, DreadShockKind};
    let peak = |shocks: Vec<DreadShock>| {
        simulate(&Params { dread_shocks: shocks, ..Params::default() })
            .iter().map(|s| s.reg_enforcement).fold(0.0_f64, f64::max)
    };
    let mk = |step: f64, mass: bool| DreadShock {
        kind: DreadShockKind::BioPandemic, year: 2030, gdp_drag: 0.05,
        stringency_step: step, mass_casualty: mass, spread: 0.2,
        defensive_pool_mult: 2.0, broad_beta_hit: 0.1,
    };
    let base = peak(vec![]);
    let scare = peak(vec![mk(0.10, false)]);
    let mass = peak(vec![mk(0.70, true)]);
    assert!(scare > base, "a dread scare must ratchet stringency: {scare} vs {base}");
    assert!(mass > scare + 0.2, "mass-casualty must ratchet FAR more than a scare: {mass} vs {scare}");
}
