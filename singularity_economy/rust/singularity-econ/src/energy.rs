//! Energy generation mix — solar + battery (Wright-law exponentials) vs gas +
//! nuclear (dispatchable firm power).
//!
//! Electricity is the binding constraint on the whole model (compute and a
//! TW-scale robot fleet share one grid — see the power gate in `lib.rs`). This
//! module makes the SUPPLY side explicit so the question "is the grid a hard
//! ceiling or a self-relieving exponential?" is answered by structure:
//!
//!   - Solar rides Wright's law (~20-25% cost decline per cumulative doubling)
//!     and is deployment-cheap, so it is the reinforcing exponential. Batteries
//!     (LFP, their own Wright curve) convert intermittent solar into dispatchable
//!     "clean firm" power. Together they are the fast, cheap, scalable supply —
//!     and robots building solar (ASI) accelerates it further.
//!   - Gas is the dispatchable BRIDGE: fast to burn but turbine-lead-time-limited
//!     (backlogs into the late 2020s), the firming source until storage is deep.
//!   - Nuclear/SMR is slow, expensive firm power — a small, late contributor.
//!
//! Output is FIRM (dispatchable-equivalent) GW the economy can actually run
//! compute + robots on, plus a blended electricity-cost index and clean share.
//! Satellite/ablatable: `enabled = 0` emits a frozen legacy mix.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnergyParams {
    pub enabled: f64,
    // --- solar (the exponential) ---
    pub solar_gw_2026: f64,
    pub cum_solar_gw_2026: f64,
    pub solar_cost_2026: f64,       // $/W
    pub solar_wright_lr: f64,       // learning rate per cumulative doubling
    pub solar_capacity_factor: f64, // ~0.20 (annual avg)
    pub solar_buildout_2026_gwpy: f64, // GW/yr additions base
    pub solar_buildout_ceiling_gwpy: f64, // max GW/yr the world can install
    // --- batteries (firm-maker) ---
    pub battery_gwh_2026: f64,
    pub battery_cost_2026: f64, // $/kWh pack
    pub battery_wright_lr: f64,
    pub storage_hours_target: f64, // hours of storage per GW solar for firmness
    // --- gas (dispatchable bridge) ---
    pub gas_gw_2026: f64,
    pub gas_buildout_gwpy: f64, // turbine-lead-time limited
    // --- nuclear / SMR (slow firm) ---
    pub nuclear_gw_2026: f64,
    pub nuclear_buildout_gwpy: f64,
    /// How strongly demand pull accelerates buildout (scarcity → investment).
    pub demand_pull_gain: f64,
}

impl Default for EnergyParams {
    fn default() -> Self {
        // Calibrated to output/history/energy_production.md. Global scale, GW.
        EnergyParams {
            enabled: 1.0,
            solar_gw_2026: 2200.0,            // cumulative ~2.2 TW (2025)
            cum_solar_gw_2026: 2200.0,
            solar_cost_2026: 0.10,            // ~$0.10/W module
            solar_wright_lr: 0.20,            // ~20%/doubling (Swanson); installed slower
            solar_capacity_factor: 0.20,
            solar_buildout_2026_gwpy: 650.0,  // ~600-700 GW/yr, ~60% China
            solar_buildout_ceiling_gwpy: 1800.0, // ~mfg cap 1.8 TW/yr (not binding)
            battery_gwh_2026: 400.0,
            battery_cost_2026: 70.0,          // LFP pack ~$70/kWh (BNEF 2025)
            battery_wright_lr: 0.21,          // ~19-24%/doubling
            storage_hours_target: 4.0,        // 4-hr sweet spot
            gas_gw_2026: 1800.0,
            gas_buildout_gwpy: 30.0,          // turbine hard cap ~20-40 GW/yr global
            nuclear_gw_2026: 400.0,
            nuclear_buildout_gwpy: 10.0,      // slow, back-loaded; SMR 2030+
            demand_pull_gain: 0.8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnergyState {
    pub solar_gw: f64,
    pub cum_solar_gw: f64,
    pub battery_gwh: f64,
    pub cum_battery_gwh: f64,
    pub gas_gw: f64,
    pub nuclear_gw: f64,
    pub solar_addition: f64,
}

impl EnergyState {
    pub fn new(p: &EnergyParams) -> Self {
        EnergyState {
            solar_gw: p.solar_gw_2026,
            cum_solar_gw: p.cum_solar_gw_2026,
            battery_gwh: p.battery_gwh_2026,
            cum_battery_gwh: p.battery_gwh_2026,
            gas_gw: p.gas_gw_2026,
            nuclear_gw: p.nuclear_gw_2026,
            solar_addition: p.solar_buildout_2026_gwpy,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct EnergyOutputs {
    /// Firm (dispatchable-equivalent) generation capacity, GW — what compute +
    /// robots can actually be run on.
    pub firm_power_gw: f64,
    /// Blended electricity cost index (2026 = 1.0), falling as cheap solar+
    /// storage displaces gas.
    pub cost_index: f64,
    /// Clean (solar+battery+nuclear firmed) share of firm power.
    pub clean_share: f64,
    pub solar_gw: f64,
    pub battery_gwh: f64,
    pub gas_gw: f64,
    pub nuclear_gw: f64,
    pub solar_cost: f64,
}

impl EnergyState {
    /// Advance one year. `demand_gw` is the grid draw the economy wants (compute
    /// + robots); `region_mult` scales buildout (China ~1 fast, US/EU slower);
    /// `asi` accelerates solar/battery deployment (robots build generation).
    pub fn step(
        &mut self,
        p: &EnergyParams,
        demand_gw: f64,
        region_mult: f64,
        asi: f64,
    ) -> EnergyOutputs {
        if p.enabled <= 0.0 {
            let firm = self.gas_gw + self.nuclear_gw + self.solar_gw * p.solar_capacity_factor;
            return EnergyOutputs {
                firm_power_gw: firm,
                cost_index: 1.0,
                clean_share: 0.0,
                solar_gw: self.solar_gw,
                battery_gwh: self.battery_gwh,
                gas_gw: self.gas_gw,
                nuclear_gw: self.nuclear_gw,
                solar_cost: p.solar_cost_2026,
            };
        }
        // Current firm capacity, and the gap demand is pulling against.
        let firm_now = self.gas_gw + self.nuclear_gw
            + self.solar_gw * p.solar_capacity_factor * self.storage_firmness(p);
        let scarcity = ((demand_gw - firm_now) / firm_now.max(1e-9)).clamp(0.0, 3.0);
        let pull = 1.0 + p.demand_pull_gain * scarcity;

        // --- solar: Wright cost decline + accelerating deployment ---
        let solar_cost = p.solar_cost_2026
            * (self.cum_solar_gw / p.cum_solar_gw_2026).powf(-wright_b(p.solar_wright_lr));
        // deployment grows with demand pull, region capacity, ASI (robots build
        // panels), and cheapness (lower cost → more deployed), capped physically.
        let cheap = (p.solar_cost_2026 / solar_cost).sqrt();
        self.solar_addition = (self.solar_addition
            * (1.0 + 0.12 * region_mult) // secular buildout momentum
            * pull
            * cheap
            * (1.0 + 0.6 * asi))
            .min(p.solar_buildout_ceiling_gwpy * region_mult.max(0.3));
        self.solar_gw += self.solar_addition;
        self.cum_solar_gw += self.solar_addition;

        // --- batteries: track solar for firmness, own Wright curve ---
        let battery_target_gwh = self.solar_gw * p.storage_hours_target;
        let battery_add = ((battery_target_gwh - self.battery_gwh) * 0.35).max(0.0);
        self.battery_gwh += battery_add;
        self.cum_battery_gwh += battery_add;

        // --- gas: dispatchable firming, turbine-lead-time limited ---
        self.gas_gw += (p.gas_buildout_gwpy * pull.min(2.0)).min(p.gas_buildout_gwpy * 2.5)
            * region_mult.max(0.5);

        // --- nuclear/SMR: slow firm ---
        self.nuclear_gw += p.nuclear_buildout_gwpy * (1.0 + 0.5 * asi) * region_mult.max(0.5);

        let firmness = self.storage_firmness(p);
        let solar_firm = self.solar_gw * p.solar_capacity_factor * firmness;
        let firm_power = solar_firm + self.gas_gw + self.nuclear_gw;
        let clean = (solar_firm + self.nuclear_gw) / firm_power.max(1e-9);

        // Blended cost: cheap solar+storage pulls the index down toward the
        // solar+storage LCOE as it displaces gas.
        let battery_cost = p.battery_cost_2026
            * (self.cum_battery_gwh / p.battery_gwh_2026).powf(-wright_b(p.battery_wright_lr));
        let solar_lcoe_index = (solar_cost / p.solar_cost_2026) * 0.6
            + (battery_cost / p.battery_cost_2026) * 0.4;
        let cost_index = (clean * solar_lcoe_index + (1.0 - clean) * 1.0).clamp(0.15, 1.5);

        EnergyOutputs {
            firm_power_gw: firm_power,
            cost_index,
            clean_share: clean,
            solar_gw: self.solar_gw,
            battery_gwh: self.battery_gwh,
            gas_gw: self.gas_gw,
            nuclear_gw: self.nuclear_gw,
            solar_cost,
        }
    }

    /// Fraction of solar output that storage can make dispatchable (0..1):
    /// deep storage relative to the target → solar becomes near-firm.
    fn storage_firmness(&self, p: &EnergyParams) -> f64 {
        let target = (self.solar_gw * p.storage_hours_target).max(1e-9);
        (0.35 + 0.65 * (self.battery_gwh / target)).clamp(0.35, 1.0)
    }
}

/// Wright exponent b such that cost ∝ cumulative^(-b) for a given learning rate.
fn wright_b(lr: f64) -> f64 {
    -(1.0 - lr).log2()
}
