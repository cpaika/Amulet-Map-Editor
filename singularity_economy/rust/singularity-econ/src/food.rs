//! Food production — an energy-coupled production system AND a political-stability
//! driver.
//!
//! Two couplings dominate (output/history/food_production.md):
//!   food ← energy: synthetic nitrogen (Haber-Bosch ammonia) is ~3-5% of global
//!     gas and 70-90% of ammonia cost, and feeds ~48% of humanity. Ammonia price
//!     elasticity to gas ≈ 0.8; a gas spike (2020→22, ~4.7x ammonia) passes through
//!     to fertilizer → farmer N cut → yield hit 1-2 seasons later.
//!   food → tension: the FAO real food-price index crossing its disruption band
//!     (the 2008/2011 pattern → Arab Spring) drives unrest, gained by import-
//!     dependence and food's income share (MENA/SSA), damped by subsidy buffers.
//!     Unrest tracks the LEVEL and TREND of price, not volatility (Bellemare 2015).
//!
//! Food is a low-automation, nitrogen-and-water-bound system: the farm gate is
//! only ~12% of the food dollar, so AI/robots cut cost only ~1-3%/yr (a trend
//! accelerator, not a discontinuity — biology iterates on seasons). Alt-protein
//! (precision fermentation, cultured meat; parity ~2030) is a slow logistic
//! relief. This layer emits a food-price index and an unrest-pressure that the
//! society/tension layer consumes. Ablatable: `enabled = 0` freezes price at 1.0
//! and emits zero unrest.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FoodParams {
    pub enabled: f64,
    /// Fertilizer(ammonia) price elasticity to the fuel/gas price (~0.8).
    pub ammonia_gas_elasticity: f64,
    /// Fertilizer's weight in the delivered food price (N feeds ~half of yields,
    /// but farm-gate is ~12% of retail → a modest pass-through weight).
    pub fertilizer_food_weight: f64,
    /// Secular yield/productivity trend (+1.3%/yr, decelerating).
    pub yield_trend: f64,
    /// AI/robot food-cost decline (−1 to −3%/yr), ceiling-limited by the 12%
    /// farm-gate share; ramps with automation/ASI.
    pub ai_cost_decline: f64,
    /// Climate yield drag in vulnerable regions (−0.35%/yr, worsening).
    pub climate_drag: f64,
    /// Green-ammonia year: severs the gas↔fertilizer wire once clean N scales.
    pub green_ammonia_year: i32,
    /// Alt-protein logistic: parity year (~2030) and 2050 share ceiling.
    pub altprotein_parity_year: i32,
    pub altprotein_2050_share: f64,
    /// FAO real-index disruption threshold (rebased; 1.0 = 2026 baseline, ~1.6
    /// ≈ the old-basis 210 riot band).
    pub unrest_threshold: f64,
    /// Import-dependence / food-income-share gain on the price→unrest map
    /// (MENA/SSA are the high-gain nodes).
    pub import_dependence_gain: f64,
    /// Subsidy/buffer damping on unrest.
    pub subsidy_buffer: f64,
}

impl Default for FoodParams {
    fn default() -> Self {
        FoodParams {
            enabled: 1.0,
            ammonia_gas_elasticity: 0.8,
            fertilizer_food_weight: 0.12,
            yield_trend: 0.013,
            ai_cost_decline: 0.02,
            climate_drag: 0.0035,
            green_ammonia_year: 2035,
            altprotein_parity_year: 2030,
            altprotein_2050_share: 0.18,
            unrest_threshold: 1.6,
            import_dependence_gain: 1.0,
            subsidy_buffer: 0.4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FoodState {
    pub food_price_index: f64,
    pub prev_food_price: f64,
    pub fertilizer_index: f64,
}

impl FoodState {
    pub fn new() -> Self {
        FoodState { food_price_index: 1.0, prev_food_price: 1.0, fertilizer_index: 1.0 }
    }
}

impl Default for FoodState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
pub struct FoodOutputs {
    pub food_price_index: f64,
    /// Unrest pressure (0 = calm) the society/tension layer consumes.
    pub unrest_pressure: f64,
    pub fertilizer_index: f64,
    pub altprotein_share: f64,
}

impl FoodState {
    /// Advance one year. `fuel_price_index` is the gas/energy price (1.0 = 2026;
    /// >1 = energy crunch), `automation` in [0,1] scales the AI cost decline,
    /// `year` drives the green-ammonia cut and alt-protein / climate ramps.
    pub fn step(
        &mut self,
        p: &FoodParams,
        fuel_price_index: f64,
        automation: f64,
        year: i32,
    ) -> FoodOutputs {
        if p.enabled <= 0.0 {
            return FoodOutputs {
                food_price_index: 1.0,
                unrest_pressure: 0.0,
                fertilizer_index: 1.0,
                altprotein_share: 0.0,
            };
        }
        // Fertilizer tracks fuel with ~0.8 elasticity — UNLESS green ammonia has
        // severed the wire, after which fertilizer decouples toward clean cost.
        let wire = if year >= p.green_ammonia_year { 0.3 } else { 1.0 };
        let fert_target = fuel_price_index.max(0.1).powf(p.ammonia_gas_elasticity * wire);
        self.fertilizer_index += 0.5 * (fert_target - self.fertilizer_index); // ~1-2 season lag

        // Alt-protein logistic relief (frees land/feed, caps price) post-parity.
        let t_alt = (year - p.altprotein_parity_year) as f64;
        let altprotein_share =
            p.altprotein_2050_share / (1.0 + (-0.28 * t_alt).exp());

        // Climate drag worsens over time in the vulnerable regions.
        let clim = p.climate_drag * (1.0 + 0.03 * (year - 2026) as f64).max(1.0);

        // Net annual food-price change: fertilizer pass-through pushes up;
        // productivity, AI cost-out, and alt-protein push down; climate up.
        let fert_push = p.fertilizer_food_weight
            * (self.fertilizer_index - 1.0);
        let productivity = p.yield_trend + p.ai_cost_decline * automation;
        let altprotein_relief = 0.04 * altprotein_share;
        let delta = fert_push - productivity + clim - altprotein_relief;

        self.prev_food_price = self.food_price_index;
        self.food_price_index = (self.food_price_index * (1.0 + delta)).max(0.3);

        // Unrest: level ABOVE the disruption threshold plus the rate-of-rise,
        // gained by import-dependence, damped by subsidy buffers.
        let level = ((self.food_price_index - p.unrest_threshold) / p.unrest_threshold)
            .max(0.0);
        let trend = ((self.food_price_index - self.prev_food_price)
            / self.prev_food_price.max(1e-9))
            .max(0.0);
        let unrest = (p.import_dependence_gain * (level + 3.0 * trend))
            * (1.0 - p.subsidy_buffer);

        FoodOutputs {
            food_price_index: self.food_price_index,
            unrest_pressure: unrest.max(0.0),
            fertilizer_index: self.fertilizer_index,
            altprotein_share,
        }
    }
}
