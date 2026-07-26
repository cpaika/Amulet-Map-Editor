//! Regional political economy — US / China / EU blocs.
//!
//! The core model runs a single global aggregate. This overlay decomposes the
//! transition into three blocs with structurally different political economies,
//! because the binding constraint DIFFERS by bloc: China is energy-and-state-
//! capacity-rich but demand-and-legitimacy-constrained; the US is capital-and-
//! frontier-rich but energy-permitting- and backlash-constrained; the EU is
//! regulation-first, energy-expensive, and capital-fragmented. The blocs diverge
//! because whoever can (a) mobilize capital, (b) build POWER, and (c) absorb
//! displacement politically pulls ahead — and those three capacities are
//! anti-correlated across the blocs.
//!
//! Satellite by default: it reads the global trajectory and produces a bloc
//! decomposition + a leadership gap + per-bloc fracture risk, without feeding
//! back into the core (so `region_layer = 0`, or leaving it unread, changes
//! nothing). A gated race-feedback is exposed for scenario work.

/// One bloc's structural political-economy parameters (relative multipliers;
/// 1.0 ≈ the fastest bloc on that axis).
#[derive(Clone, Debug)]
pub struct Bloc {
    pub name: &'static str,
    /// Share of the 2026 global AI/robot capability base (US frontier lead).
    pub capability_share_2026: f64,
    /// How fast the bloc mobilizes capital into capability (state direction +
    /// deep capital markets). China state-directed; US deep markets; EU frag.
    pub capital_mobilization: f64,
    /// Power-capacity growth multiplier — the decisive axis. China builds the
    /// grid ~an order of magnitude faster than the permitting-bound US/EU.
    pub energy_buildout: f64,
    /// Displacement → political-stress accumulation. Democracies convert
    /// displacement into backlash quickly; an autocracy suppresses it (lower
    /// gain) but pays via `brittleness`.
    pub backlash_gain: f64,
    /// Fiscal/transfer capacity to buy social peace (drains stress).
    pub fiscal_space: f64,
    /// Regulation/precaution drag on adoption (EU highest, China lowest).
    pub regulation_drag: f64,
    /// Autocratic brittleness: suppressed stress converts to a nonlinear
    /// legitimacy-crisis hazard past a threshold (0 for open systems that vent
    /// continuously through elections).
    pub brittleness: f64,
    /// Fraction of the bloc's capability GROWTH gated by Taiwan-fabbed
    /// leading-edge silicon (C10). The US frontier lead is the most
    /// leading-edge-concentrated; China is already export-controlled off EUV
    /// and has pivoted toward indigenous mature-node + robotics/energy, so a
    /// Taiwan supply shock hits it LESS — the "silicon shield" cuts the US lead
    /// harder and lets China close the gap. 0 ⇒ chip-shock-immune.
    pub chip_dependence: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegionParams {
    /// Layer switch (0 = off; the satellite emits a frozen decomposition).
    pub enabled: f64,
    // `&'static str` bloc names are not Deserialize-able and are never
    // round-tripped as custom config; serde reconstructs the calibrated set.
    #[cfg_attr(feature = "serde", serde(skip, default = "default_blocs"))]
    pub blocs: Vec<Bloc>,
    /// Stress natural decay (growth + adaptation dissipates backlash).
    pub stress_decay: f64,
    /// Threshold above which brittle-bloc stress becomes a fracture hazard.
    pub fracture_threshold: f64,
}

impl Default for RegionParams {
    fn default() -> Self {
        RegionParams {
            enabled: 1.0,
            stress_decay: 0.15,
            // Below China's peak suppressed stress (~0.86) so the autocratic-
            // brittleness fracture hazard actually fires — the design's "no
            // electoral release valve → stored stress cracks nonlinearly."
            fracture_threshold: 0.7,
            blocs: default_blocs(),
        }
    }
}

/// Calibrated bloc set (see output/history/{china,us,europe}_poliecon.md).
pub fn default_blocs() -> Vec<Bloc> {
    vec![
        // US: frontier lead (NVIDIA ~85% compute, hyperscaler ~$250B/yr capex,
        // deep capital) but slow electrons — ~2.6 TW interconnection queue, ~5yr
        // wait, ~4x slower power buildout than China — and HIGH democratic
        // backlash on a 2-4yr electoral clock. Fiscal shock-absorber half-spent
        // (debt →118% by 2035; ~2-4% GDP deployable normally). "Fast idea,
        // slow deploy, politically brittle."
        Bloc {
            name: "US",
            capability_share_2026: 0.56,
            capital_mobilization: 1.00,
            energy_buildout: 0.30,
            backlash_gain: 1.00,
            fiscal_space: 0.45,
            regulation_drag: 0.35,
            brittleness: 0.0,
            chip_dependence: 0.85, // frontier lead is leading-edge-concentrated
        },
        // China: 429 GW added 2024 (~8x US ~50 GW), 54% of global robot installs
        // (2.5x adoption), state-directed capital — but frontier compute ~0.4x
        // (export-controlled; DeepSeek/open-weight the counter), exhausted local
        // fiscal (LGFV >46% GDP), and NO electoral release valve for the
        // displaced against a −24% working-age clock → low overt backlash but a
        // fat, discontinuous regime-shift tail (brittleness).
        Bloc {
            name: "China",
            capability_share_2026: 0.33,
            capital_mobilization: 0.75,
            energy_buildout: 1.00,
            backlash_gain: 0.35,
            fiscal_space: 0.45,
            regulation_drag: 0.15,
            brittleness: 0.85,
            chip_dependence: 0.45, // export-controlled off EUV; mature-node + robotics pivot
        },
        // EU: the triple bind — industrial power ~2.3-2.6x US cost, capital
        // mobilization ~0.15-0.30x US (Draghi's €800B/yr = 4.4% GDP gap),
        // highest regulatory drag (AI Act, penalties to 7% turnover). Only edge:
        // a welfare buffer ~1.4-1.5x US that absorbs displacement shocks. Slowest
        // growth, structural laggard; Italy the fragility node.
        Bloc {
            name: "EU",
            capability_share_2026: 0.11,
            capital_mobilization: 0.25,
            energy_buildout: 0.22,
            backlash_gain: 1.10,
            fiscal_space: 0.60,
            regulation_drag: 1.00,
            brittleness: 0.1,
            chip_dependence: 0.65, // ASML owner but fabless; imports leading-edge silicon
        },
    ]
}

/// Per-bloc evolving state.
#[derive(Clone, Debug, Default)]
pub struct BlocState {
    pub capability: f64, // relative AI/robot capability index
    pub energy_cap: f64, // relative power capacity index
    pub stress: f64,     // political-stress stock
}

#[derive(Clone, Debug, Default)]
pub struct RegionOutputs {
    pub names: Vec<&'static str>,
    pub capability: Vec<f64>,
    pub energy_cap: Vec<f64>,
    pub stress: Vec<f64>,
    pub fracture_risk: Vec<f64>,
    /// China capability share minus US share — >0 means China leads.
    pub china_us_gap: f64,
    /// Combined West (US+EU) share of global capability.
    pub west_share: f64,
    /// Divergence of bloc capabilities (0 = balanced, higher = a runaway
    /// leader) — a "race vs coordination" proxy.
    pub divergence: f64,
}

pub struct RegionState {
    blocs: Vec<BlocState>,
}

impl RegionState {
    pub fn new(p: &RegionParams) -> Self {
        RegionState {
            blocs: p
                .blocs
                .iter()
                .map(|b| BlocState {
                    capability: b.capability_share_2026,
                    energy_cap: b.energy_buildout, // start proportional to build rate
                    stress: 0.0,
                })
                .collect(),
        }
    }

    /// Advance one year. `adoption_growth` is the global capability growth this
    /// year (e.g. YoY compute or adoption growth), `power_growth` the global
    /// grid-additions growth, `displacement` the global displacement rate, and
    /// `asi` the diffusion fraction (accelerates the energy edge — robots build
    /// solar fastest where the state directs them).
    pub fn step(
        &mut self,
        p: &RegionParams,
        adoption_growth: f64,
        power_growth: f64,
        displacement: f64,
        asi: f64,
        chip_supply_index: f64,
    ) -> RegionOutputs {
        let n = p.blocs.len();
        if p.enabled <= 0.0 {
            // Frozen decomposition: report the 2026 shares, no divergence.
            let shares: Vec<f64> = p.blocs.iter().map(|b| b.capability_share_2026).collect();
            let idx = |name: &str| p.blocs.iter().position(|b| b.name == name);
            let us = idx("US").map_or(0.0, |i| shares[i]);
            let cn = idx("China").map_or(0.0, |i| shares[i]);
            let eu = idx("EU").map_or(0.0, |i| shares[i]);
            return RegionOutputs {
                names: p.blocs.iter().map(|b| b.name).collect(),
                capability: shares,
                energy_cap: p.blocs.iter().map(|b| b.energy_buildout).collect(),
                stress: vec![0.0; n],
                fracture_risk: vec![0.0; n],
                china_us_gap: cn - us,
                west_share: us + eu,
                divergence: 0.0,
            };
        }
        for i in 0..n {
            let b = &p.blocs[i];
            let s = &mut self.blocs[i];
            // Energy capacity compounds at the bloc's buildout multiplier,
            // amplified by ASI (autonomous, state-directed power construction).
            s.energy_cap *= 1.0 + power_growth * b.energy_buildout * (1.0 + 0.5 * asi);
            // Capability grows with capital mobilization and is GATED by the
            // bloc's own power (you cannot run the fleet you can't power) and
            // slowed by regulation. Energy is the decisive multiplier.
            let energy_gate = (s.energy_cap / (s.energy_cap + 0.5)).clamp(0.0, 1.0);
            let reg = 1.0 - 0.5 * b.regulation_drag;
            // C10: a Taiwan leading-edge chip-supply shock throttles capability
            // growth in proportion to the bloc's chip dependence. `chip_supply_index`
            // is 1.0 on the deterministic baseline (no drawn shock) ⇒ chip_gate=1.0
            // and this term is inert; under a blockade/invasion it bites the US
            // frontier lead hardest (highest dependence), letting China close.
            let chip_gate = (1.0 - b.chip_dependence * (1.0 - chip_supply_index)).clamp(0.0, 1.0);
            s.capability *=
                1.0 + adoption_growth * b.capital_mobilization * reg * energy_gate * chip_gate;
            // Political stress: displacement drives it, fiscal transfers and
            // natural decay drain it. Democracies vent through backlash (high
            // gain, but it caps adoption elsewhere); autocracies suppress it
            // (low gain) but store brittleness.
            let vented = p.stress_decay + b.fiscal_space * 0.2;
            s.stress = (s.stress
                + b.backlash_gain * displacement * (1.0 - b.fiscal_space * 0.5)
                - vented * s.stress)
                .max(0.0);
        }
        // Normalize capability shares.
        let total_cap: f64 = self.blocs.iter().map(|s| s.capability).sum::<f64>().max(1e-9);
        let shares: Vec<f64> = self.blocs.iter().map(|s| s.capability / total_cap).collect();

        let idx = |name: &str| p.blocs.iter().position(|b| b.name == name);
        let us = idx("US").map_or(0.0, |i| shares[i]);
        let cn = idx("China").map_or(0.0, |i| shares[i]);
        let eu = idx("EU").map_or(0.0, |i| shares[i]);

        // Fracture risk: open systems fracture from raw stress; brittle systems
        // fracture nonlinearly once suppressed stress crosses the threshold.
        let fracture: Vec<f64> = (0..n)
            .map(|i| {
                let b = &p.blocs[i];
                let st = self.blocs[i].stress;
                let base = st.min(1.5);
                let brittle = if b.brittleness > 0.0 && st > p.fracture_threshold {
                    b.brittleness * (st - p.fracture_threshold)
                } else {
                    0.0
                };
                (base + brittle).min(3.0)
            })
            .collect();

        let mean_share = 1.0 / n as f64;
        let divergence = shares.iter().map(|s| (s - mean_share).powi(2)).sum::<f64>().sqrt();

        RegionOutputs {
            names: p.blocs.iter().map(|b| b.name).collect(),
            capability: shares,
            energy_cap: self.blocs.iter().map(|s| s.energy_cap).collect(),
            stress: self.blocs.iter().map(|s| s.stress).collect(),
            fracture_risk: fracture,
            china_us_gap: cn - us,
            west_share: us + eu,
            divergence,
        }
    }
}
