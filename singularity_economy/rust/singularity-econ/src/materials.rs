//! Critical-inputs supply chain — Liebig's law of the minimum.
//!
//! The prior model gated robot cost on a single aggregate `metals_index`. Real
//! robot scaling binds on the SINGLE tightest input, not an average: if precision
//! reducers cap ~0.5M humanoids/yr (Epoch), it does not matter how much copper
//! there is. This layer models a handful of critical inputs, each with
//!   - a global supply expressed as the M robots/yr it can feed (2028 base),
//!     growing at a physical rate that ASI accelerates (autonomous mining, new
//!     extraction — bioleaching / DLE / tailings, recycling / urban mining),
//!   - a per-robot INTENSITY that ASI designs down toward a substitution ceiling
//!     (rare-earth-free motors, cycloidal vs harmonic reducers, thrifting), which
//!     raises the robots-per-unit-supply and thus the effective capacity, and
//!   - a single-source (China) concentration an embargo can knock out.
//!
//! The binding materials ceiling is the MINIMUM effective capacity across inputs.
//! It binds early ("components early") with a named mechanism, and is relieved by
//! intelligence on a datable path rather than by hand-wave. Ablation: with
//! `materials_layer = 0` the layer emits an infinite ceiling and unit cost
//! multiplier 1.0, recovering the legacy behavior exactly.

/// One critical input (a chokepoint). All rates are annual.
#[derive(Clone, Debug)]
pub struct Input {
    pub name: &'static str,
    /// Robots/yr (millions) this input can supply at 2028 physical supply and
    /// 2028 per-robot intensity — i.e. supply_2028 / intensity_2028.
    pub robots_supported_2028_m: f64,
    /// Base annual growth of the *physical* supply (mine output, fab capacity,
    /// reducer lines) absent superintelligence.
    pub supply_growth: f64,
    /// Extra supply growth per unit ASI: autonomous mining, novel extraction,
    /// recycling / urban mining bringing secondary supply online.
    pub asi_supply_boost: f64,
    /// Max fraction of per-robot intensity that substitution/thrifting can design
    /// out (RE-free motors, cycloidal reducers, less material per unit). Raises
    /// effective capacity toward 1/(1 - ceiling).
    pub substitution_ceiling: f64,
    /// Speed at which substitution approaches its ceiling, per ASI-year
    /// (exponential approach). AI materials discovery compresses the historical
    /// 10-20yr discover→deploy cycle.
    pub substitution_speed: f64,
    /// Single-source (China) share of supply — the fraction lost if an embargo
    /// fires against that input.
    pub china_share: f64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialsParams {
    /// Layer master switch (0 = off, legacy behavior recovered exactly).
    pub enabled: f64,
    // The input set carries `&'static str` names (not Deserialize-able) and is
    // never round-tripped as custom config, so serde reconstructs the calibrated
    // default set rather than serializing it.
    #[cfg_attr(feature = "serde", serde(skip, default = "default_inputs"))]
    pub inputs: Vec<Input>,
    /// When an embargo is active, effective supply of a China-concentrated input
    /// is cut by `china_share * embargo_severity` (1.0 = full loss of the
    /// Chinese share; <1 = partial, grey-market leakage).
    pub embargo_severity: f64,
    /// Cost convexity: as the binding input's utilization exceeds 1.0 (demand
    /// above ceiling) the delivered unit cost rises by this elasticity — scarcity
    /// rents on the chokepoint, before production is finally hard-capped.
    pub scarcity_cost_elasticity: f64,
}

impl Default for MaterialsParams {
    fn default() -> Self {
        // NOTE: 2028 base capacities and intensities are calibrated to the
        // robotics-component + raw-materials research (output/history/
        // robotics_components.md, raw_materials_mines.md). Reducers are the
        // tightest near-term line; rare-earth magnets carry the China-embargo
        // risk; inference chips compete with datacenters; copper is elastic.
        MaterialsParams {
            enabled: 1.0,
            embargo_severity: 1.0,
            scarcity_cost_elasticity: 0.15,
            inputs: default_inputs(),
        }
    }
}

/// The calibrated critical-input set (see robotics_components / raw_materials
/// research). Factored out so serde's default reconstructs it.
pub fn default_inputs() -> Vec<Input> {
    vec![
                // Epoch AI (2026): reducers are THE binding line — planetary
                // (~3M units/yr → 500k humanoids), cycloidal/RV (~2M → 500k),
                // strain-wave (~12M → ~1M). Binding ~0.5M humanoids/yr, ~30x the
                // ~16k built in 2025.
                Input {
                    name: "precision_reducers",
                    robots_supported_2028_m: 0.5,
                    supply_growth: 0.45, // dedicated line buildout, aggressive
                    asi_supply_boost: 0.9,
                    // cycloidal / planetary / novel transmissions dodge the
                    // harmonic-flexspline bottleneck almost entirely.
                    substitution_ceiling: 0.85,
                    substitution_speed: 0.6,
                    china_share: 0.35,
                },
                // Epoch secondary ceilings: torque sensors & encoders ~1.25M/yr.
                Input {
                    name: "torque_sensors_encoders",
                    robots_supported_2028_m: 1.25,
                    supply_growth: 0.40,
                    asi_supply_boost: 0.9,
                    substitution_ceiling: 0.70, // proprioceptive/current-sensing dodges
                    substitution_speed: 0.6,
                    china_share: 0.25,
                },
                // Epoch: ball screws / linear guides ~2.5M humanoids/yr (NSK/THK).
                Input {
                    name: "ball_screws",
                    robots_supported_2028_m: 2.5,
                    supply_growth: 0.35,
                    asi_supply_boost: 0.7,
                    substitution_ceiling: 0.60,
                    substitution_speed: 0.5,
                    china_share: 0.20,
                },
                Input {
                    name: "rare_earth_magnets",
                    // ~3.5 kg NdFeB/humanoid (2x an EV). ~100kt/yr high-perf
                    // NdFeB ÷ 3.5kg ≈ 28M robots, but robots compete with EVs/wind
                    // for that magnet output → ~12M robot-equivalents. The EXTREME
                    // long-run chokepoint (10B robots ≈ 186x current NdFeB, vs 3x
                    // copper) AND the worst embargo risk: China ~90% of magnets +
                    // refining. Relieved by RE-free motors (Tesla already ships
                    // them) + ferrite / iron-nitride — most RE demand is
                    // designable-out, which is exactly how the 186x is dodged.
                    robots_supported_2028_m: 12.0,
                    supply_growth: 0.18,
                    asi_supply_boost: 0.5,
                    substitution_ceiling: 0.82,
                    substitution_speed: 0.45,
                    china_share: 0.90, // refining + magnet-making dominance
                },
                Input {
                    name: "inference_chips",
                    // Robot brains compete with datacenters for leading-edge fab.
                    robots_supported_2028_m: 5.0,
                    supply_growth: 0.35,
                    asi_supply_boost: 0.8,
                    substitution_ceiling: 0.60, // edge/neuromorphic/older nodes
                    substitution_speed: 0.5,
                    china_share: 0.20, // Taiwan concentration handled by geo layer
                },
                Input {
                    name: "copper",
                    robots_supported_2028_m: 8.0,
                    supply_growth: 0.10,
                    asi_supply_boost: 0.6, // DLE-style extraction, tailings, recycling
                    substitution_ceiling: 0.40, // aluminium substitution, thrifting
                    substitution_speed: 0.4,
                    china_share: 0.10,
                },
    ]
}

#[derive(Clone, Debug, Default)]
pub struct MaterialsOutputs {
    /// Hard cap on robot production this year (M robots/yr) — the Liebig minimum
    /// across inputs. `f64::INFINITY` when the layer is off.
    pub robot_ceiling_m: f64,
    /// Multiplier on delivered robot unit cost from scarcity rents on the binding
    /// input (>= 1.0). 1.0 when slack.
    pub cost_mult: f64,
    /// Name of the currently binding input (diagnostics).
    pub binding: &'static str,
    /// Effective capacity of the binding input (M robots/yr) — same as ceiling
    /// when finite; exposed for tests/telemetry.
    pub binding_capacity_m: f64,
}

impl MaterialsParams {
    /// Compute the materials ceiling and scarcity cost for a given year.
    /// `asi` in [0,1] is the diffusion fraction; `asi_years` is cumulative
    /// ASI-weighted years since singularity (drives substitution progress);
    /// `t` is years since 2028 (supply compounding base); `robot_demand_m` is
    /// this year's desired production; `embargo` fires the China cut.
    pub fn step(
        &self,
        t: f64,
        asi: f64,
        asi_years: f64,
        robot_demand_m: f64,
        embargo: bool,
    ) -> MaterialsOutputs {
        if self.enabled <= 0.0 || self.inputs.is_empty() {
            return MaterialsOutputs {
                robot_ceiling_m: f64::INFINITY,
                cost_mult: 1.0,
                binding: "none",
                binding_capacity_m: f64::INFINITY,
            };
        }
        let t = t.max(0.0);
        let mut ceiling = f64::INFINITY;
        let mut binding = "none";
        for inp in &self.inputs {
            // physical supply compounds; ASI accelerates it (mining, extraction,
            // recycling). Substitution designs intensity down toward the ceiling,
            // raising robots-per-unit-supply by 1/(1-sub).
            let supply_mult = (1.0 + inp.supply_growth + inp.asi_supply_boost * asi)
                .powf(t);
            let sub = inp.substitution_ceiling
                * (1.0 - (-inp.substitution_speed * asi_years).exp());
            let intensity_relief = 1.0 / (1.0 - sub).max(1e-3);
            let embargo_mult = if embargo {
                (1.0 - inp.china_share * self.embargo_severity).max(0.0)
            } else {
                1.0
            };
            let cap = inp.robots_supported_2028_m * supply_mult * intensity_relief
                * embargo_mult;
            if cap < ceiling {
                ceiling = cap;
                binding = inp.name;
            }
        }
        // Scarcity rents: cost rises as demand presses past the binding ceiling.
        let utilization = (robot_demand_m / ceiling.max(1e-9)).max(0.0);
        let cost_mult = 1.0
            + self.scarcity_cost_elasticity * (utilization - 1.0).max(0.0).min(3.0);
        MaterialsOutputs {
            robot_ceiling_m: ceiling,
            cost_mult,
            binding,
            binding_capacity_m: ceiling,
        }
    }
}
