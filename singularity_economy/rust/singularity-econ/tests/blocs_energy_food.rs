//! Validation contract for the regional (US/China/EU), energy-mix, and food
//! layers. Calibrated to output/history/{china,us,europe}_poliecon.md,
//! energy_production.md, food_production.md.

use singularity_econ::energy::{EnergyParams, EnergyState};
use singularity_econ::food::{FoodParams, FoodState};
use singularity_econ::regions::{RegionParams, RegionState};
use singularity_econ::{simulate, Params, YearState};

fn to2050(mut p: Params) -> Vec<YearState> {
    p.end_year = 2050;
    simulate(&p)
}
fn at(v: &[YearState], y: i32) -> &YearState {
    v.iter().find(|s| s.year == y).unwrap()
}

// ---------- ENERGY: solar+battery is the self-relieving exponential ----------

// Solar rides Wright's law: cumulative capacity compounds and the blended
// electricity cost index FALLS as cheap solar+storage displaces gas.
#[test]
fn solar_wright_cost_falls_and_capacity_grows() {
    let v = to2050(Params::default());
    assert!(at(&v, 2050).solar_gw > at(&v, 2030).solar_gw * 3.0, "solar must compound");
    assert!(
        at(&v, 2050).electricity_cost_index < at(&v, 2030).electricity_cost_index,
        "Wright learning must pull the cost index down"
    );
    assert!(
        at(&v, 2050).clean_power_share > at(&v, 2030).clean_power_share,
        "clean share must rise"
    );
}

// Ablation: with the layer off the mix is frozen (cost index 1.0, no solar
// growth) — and the core is untouched (verified by parity).
#[test]
fn energy_off_is_frozen() {
    let mut st = EnergyState::new(&EnergyParams::default());
    let p = EnergyParams { enabled: 0.0, ..EnergyParams::default() };
    let a = st.step(&p, 5000.0, 1.0, 1.0);
    assert_eq!(a.cost_index, 1.0);
}

// ---------- REGIONS: divergent political economies ----------

// The US keeps its frontier lead but China closes part of the gap on its
// energy-buildout edge, and the EU fades — the structural ranking.
#[test]
fn us_leads_china_closes_eu_fades() {
    let v = to2050(Params::default());
    let us = |s: &YearState| s.bloc_capability[0];
    let cn = |s: &YearState| s.bloc_capability[1];
    let eu = |s: &YearState| s.bloc_capability[2];
    let a = at(&v, 2028);
    let b = at(&v, 2050);
    assert!(us(b) > cn(b), "US must keep the frontier lead");
    assert!(cn(b) > cn(a), "China must close some of the gap on its energy edge");
    assert!(eu(b) < eu(a), "EU must fade (the triple bind)");
}

// The autocracy suppresses displacement backlash (low overt stress) while the
// democracies vent it — US/EU political stress ends far above China's.
#[test]
fn democracies_vent_autocracy_suppresses() {
    let v = to2050(Params::default());
    let s = at(&v, 2050);
    assert!(
        s.bloc_stress[0] > s.bloc_stress[1] && s.bloc_stress[2] > s.bloc_stress[1],
        "US ({}) and EU ({}) stress must exceed China's suppressed {}",
        s.bloc_stress[0], s.bloc_stress[2], s.bloc_stress[1]
    );
}

// Ablation: layer off freezes the decomposition at the 2026 shares.
#[test]
fn regions_off_is_frozen() {
    let p = RegionParams { enabled: 0.0, ..RegionParams::default() };
    let mut st = RegionState::new(&p);
    let out = st.step(&p, 0.2, 0.1, 0.1, 1.0, 1.0);
    assert!((out.capability[0] - 0.56).abs() < 1e-9, "US frozen at 2026 share");
    assert_eq!(out.divergence, 0.0);
}

// ---------- FOOD: the gas → fertilizer → food → unrest chain ----------

// A fuel/gas spike raises fertilizer, then food price, then unrest — the
// marquee cross-layer transmission (2008/2011 pattern).
#[test]
fn gas_spike_transmits_to_food_and_unrest() {
    let p = FoodParams::default();
    let mut calm = FoodState::new();
    let mut shocked = FoodState::new();
    for y in 2027..=2032 {
        calm.step(&p, 1.0, 0.3, y);
        shocked.step(&p, 3.0, 0.3, y); // sustained gas spike
    }
    let c = calm.step(&p, 1.0, 0.3, 2033);
    let s = shocked.step(&p, 3.0, 0.3, 2033);
    assert!(s.fertilizer_index > c.fertilizer_index, "gas must raise fertilizer");
    assert!(s.food_price_index > c.food_price_index, "fertilizer must raise food price");
    assert!(s.unrest_pressure > c.unrest_pressure, "food price must raise unrest");
}

// Green ammonia severs the gas↔fertilizer wire: post-2035 a gas spike passes
// through far less than pre-2035.
#[test]
fn green_ammonia_severs_the_gas_wire() {
    let p = FoodParams::default();
    let pre = {
        let mut st = FoodState::new();
        st.step(&p, 3.0, 0.3, 2032).fertilizer_index
    };
    let post = {
        let mut st = FoodState::new();
        st.step(&p, 3.0, 0.3, 2040).fertilizer_index
    };
    assert!(post < pre, "green ammonia must weaken the gas→fertilizer pass-through");
}

// Ablation: food layer off → price pinned at 1.0, zero unrest.
#[test]
fn food_off_is_inert() {
    let p = FoodParams { enabled: 0.0, ..FoodParams::default() };
    let mut st = FoodState::new();
    let a = st.step(&p, 3.0, 0.3, 2032);
    assert_eq!(a.food_price_index, 1.0);
    assert_eq!(a.unrest_pressure, 0.0);
}

// Food→tension coupling: switching on the gain must raise political sentiment
// vs the pure-satellite baseline (the food-price channel into unrest).
#[test]
fn food_tension_coupling_raises_backlash() {
    let base = to2050(Params::default()); // food_tension_gain = 0
    let mut coupled_p = Params::default();
    coupled_p.food_tension_gain = 0.5;
    let coupled = to2050(coupled_p);
    let peak = |v: &[YearState]| v.iter().map(|s| s.sentiment).fold(f64::MIN, f64::max);
    assert!(
        peak(&coupled) >= peak(&base),
        "food unrest must not lower backlash: {} vs {}",
        peak(&coupled),
        peak(&base)
    );
}

// C3: the energy layer must be able to feed the CORE, not just food. With the
// coupling gain on, the solar+battery-driven generation cost index pulls the AI
// sector's effective electricity price below the pure-utilization baseline — the
// "cheap clean power relieves the power constraint" channel. Gain 0 is a pure
// satellite (baseline unchanged, enforced by the golden snapshot elsewhere).
#[test]
fn energy_price_coupling_relieves_the_core_price() {
    let base = to2050(Params::default()); // energy_price_gain = 0
    let mut coupled_p = Params::default();
    coupled_p.energy_price_gain = 0.5;
    let coupled = to2050(coupled_p);
    // by late horizon the generation cost index is well below 1.0, so the coupled
    // electricity price must sit below the uncoupled one.
    assert!(
        at(&coupled, 2050).electricity_price < at(&base, 2050).electricity_price,
        "energy coupling must relieve the core electricity price: {} !< {}",
        at(&coupled, 2050).electricity_price,
        at(&base, 2050).electricity_price
    );
}

// C4: the food fertilizer driver is the ENERGY cost index, not datacenter compute
// pricing — so as solar+battery Wright's law pulls the blended energy cost below
// 1.0, the fertilizer(Haber-Bosch)/green-ammonia channel must pass that RELIEF
// through to a FALLING food price. Previously food was fed a one-sided
// datacenter-scarcity ratio (>=1 always), flooring fertilizer at 1.0 and making
// the clean-N relief path unreachable.
#[test]
fn cheap_energy_relieves_food_price() {
    let v = to2050(Params::default());
    let early = at(&v, 2030).food_price_index;
    let late = at(&v, 2050).food_price_index;
    // energy cost index falls over the horizon...
    assert!(
        at(&v, 2050).electricity_cost_index < at(&v, 2030).electricity_cost_index,
        "precondition: energy cost index must fall"
    );
    // ...and food price must follow it DOWN (the two-way coupling is live).
    assert!(
        late < early,
        "cheap energy must relieve food price: {late} !< {early}"
    );
}

// C10: a Taiwan chip-supply shock must be REGIONALLY DECISIVE — a blockade throttles
// capability growth in proportion to each bloc's leading-edge chip dependence, and the
// US frontier lead is the most chip-concentrated, so China closes the capability gap
// versus a shock-free baseline. Previously the regions layer saw only global scalars
// and a Taiwan war had ZERO effect on the China-vs-US split (the design's decisive axis).
#[test]
fn taiwan_blockade_closes_china_us_gap() {
    use singularity_econ::geopolitics::{GeoShock, ShockKind};
    let base = to2050(Params::default());
    let mut shocked_p = Params::default();
    shocked_p.geo_shocks = vec![GeoShock {
        kind: ShockKind::TaiwanBlockade,
        start_year: 2030,
        duration_years: 2.0,
    }];
    let shocked = to2050(shocked_p);
    // china_us_capability_gap = China share - US share (less negative ⇒ closing).
    let gap = |v: &[YearState], y: i32| at(v, y).china_us_capability_gap;
    assert!(
        gap(&shocked, 2035) > gap(&base, 2035),
        "Taiwan blockade must let China close the gap: shocked {} vs base {}",
        gap(&shocked, 2035),
        gap(&base, 2035)
    );
}

// C9: the autocratic-brittleness fracture hazard must be LIVE, not dead — China's
// suppressed stress crosses the brittleness threshold and produces a nonzero
// regime-shift risk somewhere on the horizon (previously computed then discarded,
// with the threshold set above the peak so it never fired).
#[test]
fn china_fracture_hazard_is_live() {
    let v = to2050(Params::default());
    let china_peak = v.iter().map(|s| s.bloc_fracture_risk[1]).fold(0.0_f64, f64::max);
    assert!(
        china_peak > 0.0,
        "China's brittleness fracture hazard never fires: peak {china_peak}"
    );
}
