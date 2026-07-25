//! Musk-case for launch/orbital compute: grant every disputed assumption
//! the physics allows. Starship proven THIS year, launch grows at the
//! WWII mobilization ceiling (2.5x/yr — faster than any industry ever,
//! incl. Falcon's own 1.97x record year) EVERY year, SpaceX's claimed
//! 10-12 kg/kW specific mass (vs 34-59 independent estimates), half the
//! usual Starlink/defense tonnage pre-emption, early competition
//! breaking the price wedge. Radiator floor (8 kg/kW) and orbital
//! attrition stay — those are physics, not planning.
use singularity_econ::{simulate, Params, SpaceParams};

fn main() {
    let musk = Params {
        space: SpaceParams {
            starship_proven_year: 2026,
            launch_growth_fast: 1.5, // => 2.5x/yr multiplier at the ceiling
            kg_per_kw_2026: 12.0,
            specific_mass_decline: 0.25,
            preempt_share: 0.35,
            second_supplier_year: 2029,
            launch_cost_2026: 1_500.0,
            ..SpaceParams::default()
        },
        ..Params::default()
    };
    let m = simulate(&musk);
    let b = simulate(&Params::default());
    println!("year | capacity t/yr |  $/kg | orbital GWe | power_margin (musk vs base)");
    for (sm, sb) in m.iter().zip(b.iter()) {
        println!("{} | {:>11.0} | {:>5.0} | {:>9.3} | {:.3} vs {:.3}",
            sm.year, sm.launch_capacity_tpy, sm.launch_cost_per_kg,
            sm.orbital_gw_equiv, sm.power_margin, sb.power_margin);
    }
    println!("\nMusk's literal claim check: 1M t/yr within 3 years of 2026");
    println!("  requires 4,000 -> 1,000,000 t/yr by 2029 = 6.3x/yr sustained");
    println!("  vs Falcon's all-time record year 1.97x, global record 1.72x,");
    println!("  Starship realized flights: 2023:2, 2024:4, 2025:5, 2026 H1:2");
    println!("  (zero full orbital insertions in 13 flights as of Jul 2026)");
}
