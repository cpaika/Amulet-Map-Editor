//! Space module: launch growth, Wright-law costs, orbital compute as a
//! tonnage-gated power-relief channel.
//!
//! Spec: output/history/space_design.md (5 briefs, July 2026). Verdict
//! encoded here: orbital compute does NOT relax the terrestrial power
//! constraint inside 2026-2036 — 1 GW-IT needs 10,000-30,000 t to LEO
//! against a ~4,000 t/yr 2026 world launch base. The module is a
//! 2034-2036 tail rent-clipper (B11: power rents widen the orbital
//! breakeven gate — the model's only endogenous power relief, sized to
//! a trickle) plus the R5 Wright-learning loop on launch. No space->grid
//! power flow ever (beaming ~40% end-to-end); orbital power monetizes
//! only through co-located compute.

use crate::Pipeline;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceParams {
    pub cum_upmass_2026_t: f64,
    pub launch_capacity_2026_tpy: f64,
    /// Wright learning rate per cumulative doubling (PNAS 2026: 21.2%),
    /// on COST — the rent wedge separates price from cost (F9 list price
    /// flat 2016-2026 while cost fell: monopoly pricing).
    pub wright_lr: f64,
    pub launch_cost_2026: f64,
    pub rent_wedge_mono: f64,
    pub rent_wedge_competitive: f64,
    /// Year a second heavy-reuse supplier breaks the wedge (0 = never;
    /// MC draws ~p 0.35 by 2031).
    pub second_supplier_year: i32,
    /// Year Starship full-reuse is proven (0 = never; deterministic
    /// central path 2028; MC Bernoulli p~0.7 by end-2027).
    pub starship_proven_year: i32,
    pub launch_growth_slow: f64,
    pub launch_growth_fast: f64,
    /// Specific mass of orbital compute (radiator+solar+structure).
    /// Floor is Stefan-Boltzmann-set and launch-cost-ORTHOGONAL.
    pub kg_per_kw_2026: f64,
    pub kg_per_kw_floor: f64,
    pub specific_mass_decline: f64,
    /// Starlink replacement + defense pre-empt most tonnage.
    pub preempt_share: f64,
    /// Whole-stack orbital life 5-6yr => 18%/yr replacement tax (B12).
    pub orbital_attrition: f64,
    /// Availability x workload-addressability x overhead penalty stack.
    pub orbital_effectiveness: f64,
}

impl Default for SpaceParams {
    fn default() -> Self {
        SpaceParams {
            cum_upmass_2026_t: 28_000.0,
            launch_capacity_2026_tpy: 4_000.0,
            wright_lr: 0.212,
            launch_cost_2026: 2_000.0,
            rent_wedge_mono: 1.8,
            rent_wedge_competitive: 1.15,
            second_supplier_year: 2031,
            starship_proven_year: 2028,
            launch_growth_slow: 0.12,
            launch_growth_fast: 0.35,
            kg_per_kw_2026: 40.0,
            kg_per_kw_floor: 8.0,
            specific_mass_decline: 0.15,
            preempt_share: 0.65,
            orbital_attrition: 0.18,
            orbital_effectiveness: 0.60,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpaceState {
    pub cum_upmass_t: f64,
    pub launch_capacity_tpy: f64,
    pub launch_cost_per_kg: f64,
    pub launch_price_per_kg: f64,
    pub kg_per_kw: f64,
    pub orbital_gw_it: f64,
    pub orbital_gw_equiv: f64,
    pipe: Pipeline,
}

fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

impl SpaceState {
    pub fn new(sp: &SpaceParams) -> Self {
        SpaceState {
            cum_upmass_t: sp.cum_upmass_2026_t,
            launch_capacity_tpy: sp.launch_capacity_2026_tpy,
            launch_cost_per_kg: sp.launch_cost_2026,
            launch_price_per_kg: sp.launch_cost_2026 * sp.rent_wedge_mono,
            kg_per_kw: sp.kg_per_kw_2026,
            orbital_gw_it: 0.0,
            orbital_gw_equiv: 0.0,
            pipe: Pipeline::new(2, 0.0),
        }
    }

    /// Advance one year. `power_rent_index` in [0,1] measures how far
    /// power margins sit above normal — the B11 coupling: terrestrial
    /// scarcity is what makes orbit investable.
    pub fn step(&mut self, sp: &SpaceParams, year: i32, power_rent_index: f64, r5: f64) {
        // launch capacity: two-regime growth (2016-19 flat vs 2019-25
        // Starlink-loop 42%/yr lesson), hard-capped by the WWII ceiling
        let proven = sp.starship_proven_year > 0 && year >= sp.starship_proven_year;
        let g = if proven { sp.launch_growth_fast } else { sp.launch_growth_slow };
        self.launch_capacity_tpy *= (1.0 + g).min(2.5);
        let upmass = self.launch_capacity_tpy;
        self.cum_upmass_t += upmass;

        // Wright cost on cumulative doublings (R5), price = cost x wedge
        if r5 > 0.0 {
            let doublings = (self.cum_upmass_t / sp.cum_upmass_2026_t).log2().max(0.0);
            self.launch_cost_per_kg = sp.launch_cost_2026
                * (1.0 - sp.wright_lr * r5).powf(doublings);
        }
        let wedge = if sp.second_supplier_year > 0 && year >= sp.second_supplier_year {
            sp.rent_wedge_competitive
        } else {
            sp.rent_wedge_mono
        };
        self.launch_price_per_kg = self.launch_cost_per_kg * wedge;

        // specific mass improves slowly; the radiator floor never does
        self.kg_per_kw = (self.kg_per_kw * (1.0 - sp.specific_mass_decline))
            .max(sp.kg_per_kw_floor);

        // tonnage available after Starlink/defense pre-emption
        let available_t = upmass * (1.0 - sp.preempt_share)
            - self.orbital_gw_it * self.kg_per_kw * 1000.0 * sp.orbital_attrition;
        let available_t = available_t.max(0.0);

        // B11 economic gate: power rents widen the orbital breakeven
        let breakeven = 200.0 + 300.0 * power_rent_index.clamp(0.0, 1.0);
        let gate = logistic((breakeven - self.launch_price_per_kg) / 50.0);

        // deliver through a 2-yr manifest/commission pipeline — no
        // step_accel: ASI does not compress FAA reviews
        let delivered_gw = self.pipe.step(available_t * gate / (self.kg_per_kw * 1000.0));
        self.orbital_gw_it =
            self.orbital_gw_it * (1.0 - sp.orbital_attrition) + delivered_gw;
        self.orbital_gw_equiv = self.orbital_gw_it * sp.orbital_effectiveness;
    }
}
