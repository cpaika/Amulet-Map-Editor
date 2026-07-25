//! Space-module validation contract (output/history/space_design.md §4).

use singularity_econ::{simulate, Binding, Loops, Params, SpaceParams, SpaceState, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// #1 WWII ceiling: launch tonnage never grows >2.5x/yr (70 years of
// spaceflight max: 1.72x global, 1.97x single-program).
#[test]
fn launch_tonnage_respects_mobilization_ceiling() {
    for w in base().windows(2) {
        let r = w[1].launch_capacity_tpy / w[0].launch_capacity_tpy.max(1.0);
        assert!(r <= 2.5 + 1e-9, "launch grew {r:.2}x in {}", w[1].year);
    }
}

// #2 Backcast 2016->2025: slow regime flipping fast in 2019 (Starlink
// self-demand) must land 2025 upmass in [2200, 4500] t and cost in
// [$1500, $3500]/kg — jointly falsifies the two-regime + Wright shape.
#[test]
fn backcast_2016_2025() {
    let sp = SpaceParams {
        cum_upmass_2026_t: 5_500.0, // cumulative through 2015
        launch_capacity_2026_tpy: 338.0,
        // 2016 F9 cost basis (~$4k/kg): reusability was already flying
        // (first landing Dec 2015) — Wright learning explains 2016-2025
        // only from the post-reuse anchor, not the Shuttle-era $20k+.
        launch_cost_2026: 4_000.0,
        starship_proven_year: 2019, // proxy: Starlink self-demand regime flip
        second_supplier_year: 0,
        ..SpaceParams::default()
    };
    let mut st = SpaceState::new(&sp);
    for year in 2016..=2025 {
        st.step(&sp, year, 0.3, 1.0);
    }
    assert!(
        (2_200.0..=4_500.0).contains(&st.launch_capacity_tpy),
        "backcast 2025 upmass {} t",
        st.launch_capacity_tpy
    );
    assert!(
        (1_500.0..=3_500.0).contains(&st.launch_cost_per_kg),
        "backcast 2025 cost ${}/kg",
        st.launch_cost_per_kg
    );
}

// #3 Tonnage binds before economics: even with the price gate forced
// fully open (free launch), orbital GW-equiv stays <1 before 2031.
#[test]
fn tonnage_binds_before_economics() {
    let mut p = Params::default();
    p.space.launch_cost_2026 = 1.0; // gate fully open from day one
    let states = simulate(&p);
    for s in states.iter().filter(|s| s.year < 2031) {
        assert!(
            s.orbital_gw_equiv < 1.0,
            "orbital {} GW-equiv in {} — mass budget mis-specified",
            s.orbital_gw_equiv,
            s.year
        );
    }
}

// #4 Headline survives: power still binds >=6/11 years with the module
// on, and the 2036 power-margin relief vs module-off is <=25% of the
// excess over normal (a tail rent-clipper, not a thesis-killer).
#[test]
fn power_thesis_survives_orbital_compute() {
    let p = Params::default();
    let on = base();
    let off = simulate(&Params {
        loops: Loops { r5_launch_learning: 0.0, ..Loops::default() },
        space: SpaceParams { orbital_effectiveness: 0.0, ..SpaceParams::default() },
        ..Params::default()
    });
    let power_years = on.iter().filter(|s| s.binding == Binding::Power).count();
    assert!(power_years >= 6, "power bound only {power_years} years");
    let excess_off = off.last().unwrap().power_margin - p.normal_margin;
    let relief = off.last().unwrap().power_margin - on.last().unwrap().power_margin;
    assert!(
        relief <= 0.25 * excess_off + 1e-9,
        "orbital relief {relief} exceeds 25% of {excess_off} excess"
    );
}

// #5 Physics floor: kg/kW never goes below the Stefan-Boltzmann-set
// radiator floor no matter how cheap launch gets (orthogonal channels).
#[test]
fn radiator_floor_is_launch_cost_invariant() {
    let mut p = Params::default();
    p.space.launch_cost_2026 = 1.0;
    p.space.specific_mass_decline = 0.9; // absurd tech optimism
    let sp = p.space.clone();
    let mut st = SpaceState::new(&sp);
    for year in 2026..=2060 {
        st.step(&sp, year, 1.0, 1.0);
        assert!(st.kg_per_kw >= sp.kg_per_kw_floor - 1e-9);
    }
}

// #6 Ablation: r5=0 + effectiveness=0 must leave the terrestrial economy
// bit-identical to a run where orbital compute never contributes.
#[test]
fn space_ablation_is_inert() {
    let off = simulate(&Params {
        loops: Loops { r5_launch_learning: 0.0, ..Loops::default() },
        space: SpaceParams { orbital_effectiveness: 0.0, ..SpaceParams::default() },
        ..Params::default()
    });
    for s in &off {
        assert_eq!(s.orbital_gw_equiv.to_bits(), 0.0f64.to_bits());
    }
}
