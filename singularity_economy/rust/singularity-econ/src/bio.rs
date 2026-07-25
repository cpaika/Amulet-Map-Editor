//! Bio layer: AI drug-discovery pool, wet-lab-automation as an R1-physical-
//! science extension, healthcare-cost-deflation coupling into
//! `demography.age_creep`, and longevity/BCI as BOUNDED, OUT-OF-HORIZON
//! optionality pools + memetic frames.
//!
//! Design source: 6 research briefs (biorisk, AI drug discovery, longevity/
//! LEV, BCI, AI cyber, self-driving labs — July 2026) + Meadows synthesis.
//!
//! Two governing verdicts, enforced structurally:
//!
//! 1. **LEVEL vs RATE discipline.** AI delivers (a) a LEVEL down-shift in
//!    preclinical cost/time and a RATE increase in preclinical throughput,
//!    but NO RATE change in the Phase-2/3 clinical attrition that dominates
//!    NPV; (b) a one-time LEVEL throughput step per automated lab, NOT a
//!    compounding loop; (c) NO compression of the physical experiment
//!    clock. The R1->physical spillover coefficient is << 1 and saturating.
//!
//! 2. **Longevity and BCI are OUT-OF-HORIZON.** They contribute ~0 to
//!    `age_creep`, mortality, labor supply, and the index inside 2026-2036.
//!    Their only in-horizon channels are (a) tiny high-beta optionality
//!    pools marked on regulatory-milestone step events and (b) memetic
//!    frames feeding the society dread/sentiment machinery (backlash-heavy).
//!
//! The DREAD SHOCK CLASSES (biorisk pandemic + cyber) live in this file's
//! `shocks` section but are applied by lib.rs alongside the geopolitics
//! `GeoFx` and the existing AI-incident path; they READ the capability
//! stocks charged here and are a DISTINCT class from the generic AI
//! incident (different driver, different regulatory target).

// ===========================================================================
// Parameters
// ===========================================================================

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BioParams {
    // -- R1 -> physical-science spillover (self-driving labs) --
    /// Saturating spillover from software R1 (algo_eff) to physical
    /// discovery capability. WELL BELOW 1: software self-improvement is
    /// not physical self-improvement (SDL brief).
    pub r1_phys_spillover: f64,
    /// Cycle-time-class blend: realized acceleration is ~5-10x in fast
    /// domains (flow chem/formulation), ~1-2x in slow (battery/clinical/
    /// biology). This single blended ceiling multiplies discovery-per-$;
    /// it does NOT shrink the physical clock.
    pub sdl_accel_ceiling: f64,
    /// Diminishing returns: AF saturates at ~10-20 experiments/dimension.
    pub sdl_saturation: f64,
    /// Validation haircut on reported discoveries (A-Lab lesson: most
    /// "41 new" compounds were already known / mis-assigned).
    pub validation_haircut: f64,

    // -- AI drug discovery pool --
    /// 2026 realized pool ($B): tooling/SaaS only (Schrodinger-class),
    /// near-zero; every AI-native platform is pre-approval, cash-negative.
    pub drug_pool_2026_b: f64,
    /// Narrow tooling TAM central path by 2035 ($B) — NOT the broad
    /// AI-touched-pipeline definition (~10x dispersion; use the narrow one).
    pub drug_pool_2035_b: f64,
    /// Preclinical time/cost multiplier under full AI adoption (x0.5-0.75).
    pub preclinical_cost_mult: f64,
    /// Adoption share of pharma actually using AI in discovery (>40% had
    /// NOT materially adopted as of 2025). Rises over the horizon.
    pub pharma_adoption_2026: f64,
    pub pharma_adoption_2035: f64,
    /// Phase-1 gate multiplier for AI-discovered molecules (~1.6x; measures
    /// tolerability, NOT efficacy). Phase 2/3 multiplier is pinned to 1.0.
    pub phase1_gate_mult: f64,
    /// Overall clinical LOA (Phase-1 -> approval), unchanged by AI (~10%).
    pub clinical_loa: f64,
    /// Year the first AI-native royalty can contribute (Insilico Ph3 read).
    pub royalty_first_year: i32,

    // -- healthcare deflation -> age_creep (deliberately WEAK + LAGGED) --
    /// Pharma share of total health spend (~10-14%).
    pub pharma_share_of_health: f64,
    /// R&D-savings pass-through to drug PRICE (contested; pharma may retain
    /// as margin). Skeptical: ~0.05-0.15.
    pub price_passthrough: f64,
    /// kappa: fraction of age_creep relieved per unit healthcare deflation.
    /// SMALL (0.10-0.20); a large kappa would wrongly rescue transfer_cap.
    pub age_creep_kappa: f64,
    /// Lag (years) from R&D-cost decline to realized health-spend relief.
    pub deflation_lag: i32,

    // -- biorisk capability drivers (feed the BioPandemic shock hazard) --
    /// R1->VCT-style KNOWLEDGE-uplift map reference: algo_eff at which the
    /// normalized knowledge signal saturates to 1.0. VCT-expert crossing
    /// happened ~early-2025, so the 2026 intercept is already elevated.
    pub knowledge_uplift_ref: f64,
    /// KNOWLEDGE -> OPERATIONAL translation coefficient (Aum dissemination
    /// gate). ~0.1-0.3, slowly rising toward wet-lab automation. THE
    /// highest-leverage single parameter — expose for sensitivity.
    pub translation_coeff_2026: f64,
    pub translation_coeff_2036: f64,
    /// Synthesis-access uncovered fraction (IGSC screens ~80% => ~0.20).
    pub synth_gate_2026: f64,
    /// Benchtop-synth diffusion + AI screening-evasion erode the gate UP
    /// toward this by mid-2030s (re-closable by a mandatory-screening
    /// regulatory step armed by a bio dread event).
    pub synth_gate_2036: f64,
    /// Per-year gate re-closure when a mandatory-universal-screening
    /// ratchet is armed (B-RATCHET mitigation lever).
    pub synth_reclose_step: f64,

    // -- longevity / BCI optionality (bounded, out-of-horizon) --
    /// Longevity narrow tech-market pool 2035 ($B): ~$60-67B, 2-3 orders
    /// below AI-services/robots. Optionality, never an index driver.
    pub longevity_pool_2035_b: f64,
    /// BCI optionality pool 2035 ($B): milestone-stepped, near-zero base.
    pub bci_pool_2035_b: f64,
    /// Year of the first US Class-III BCI PMA regime switch (2028-2030);
    /// before it the BCI pool is heavily discounted.
    pub bci_pma_year: i32,
    /// Probability haircut on aspirational BCI channel-count roadmaps
    /// (Stevenson's-Law says targets slip ~15-20x optimistic).
    pub bci_roadmap_haircut: f64,

    // loop / layer switches (0 = off, deterministic baseline unchanged)
    pub bio_layer: f64,
}

impl Default for BioParams {
    fn default() -> Self {
        BioParams {
            r1_phys_spillover: 0.15,
            sdl_accel_ceiling: 4.0,
            sdl_saturation: 0.6,
            validation_haircut: 0.4,
            drug_pool_2026_b: 1.5,
            drug_pool_2035_b: 14.0,
            preclinical_cost_mult: 0.65,
            pharma_adoption_2026: 0.55,
            pharma_adoption_2035: 0.85,
            phase1_gate_mult: 1.6,
            clinical_loa: 0.10,
            royalty_first_year: 2029,
            pharma_share_of_health: 0.12,
            price_passthrough: 0.10,
            age_creep_kappa: 0.15,
            deflation_lag: 4,
            knowledge_uplift_ref: 50.0,
            translation_coeff_2026: 0.15,
            translation_coeff_2036: 0.35,
            synth_gate_2026: 0.20,
            synth_gate_2036: 0.35,
            synth_reclose_step: 0.06,
            longevity_pool_2035_b: 64.0,
            bci_pool_2035_b: 8.0,
            bci_pma_year: 2029,
            bci_roadmap_haircut: 0.30,
            bio_layer: 0.0, // OFF by default: baseline exactly unchanged
        }
    }
}

// ===========================================================================
// State
// ===========================================================================

#[derive(Debug, Clone)]
pub struct BioState {
    /// R1-physical-science extension: blended, cycle-time-FLOORED discovery
    /// capability (0..~1). Grows with R1 x spillover, saturates fast.
    pub physical_discovery_cap: f64,
    /// Candidates-in-clinical-development stock; inflow = preclinical
    /// throughput (adoption-gated), drains via ~90% clinical attrition.
    pub drug_pipeline: f64,
    /// Drug-development cost-per-candidate index (1.0 -> down with AI
    /// adoption). Feeds the small, lagged healthcare-deflation signal.
    pub hc_rd_cost_index: f64,

    // -- biorisk hazard drivers (read by the BioPandemic shock) --
    /// VCT-style accessible KNOWLEDGE uplift (0..1), from R1. Already
    /// partially "on" at 2026 (expert baseline crossed ~early-2025).
    pub bio_knowledge_uplift: f64,
    /// Uncovered synthesis fraction (0.20 -> ~0.35); slow-eroding balancing
    /// gate, re-closable by the mandatory-screening ratchet.
    pub synthesis_access_gate: f64,
    /// Habituation / nth-event escalation counter.
    pub cumulative_bio_incidents: u32,
    /// Latched once a mass-casualty bio event arms mandatory screening.
    pub screening_mandate_armed: bool,

    // -- optionality shadow stocks (~0 impact in-horizon) --
    pub longevity_trl: f64,
    pub bci_maturity: f64,
    /// Elite-longevity memetic salience (fast-charging; feeds BOTH
    /// accelerationism AND dread/backlash — backlash dominates in-horizon).
    pub elite_longevity_frame: f64,
    /// Transhumanist human-AI-merge frame (ambivalent sign; capped by the
    /// ~10 bits/s cognition ceiling).
    pub merge_frame: f64,

    // -- realized profit pools ($B) --
    pub ai_drug_pool_b: f64,
    pub longevity_pool_b: f64,
    pub bci_pool_b: f64,

    // history for the deflation lag
    hc_cost_history: Vec<f64>,
    years: i32,
}

/// Per-year outputs consumed by lib.rs, the valuation layer, and the
/// bio/cyber shock samplers.
#[derive(Debug, Clone, Copy)]
pub struct BioOutputs {
    /// OPERATIONAL bioweapon-capability uplift = knowledge x translation x
    /// synthesis-access. THIS is the BioPandemic hazard driver (not raw
    /// knowledge — reconciles RAND "no uplift" vs VCT "threshold crossed").
    pub bio_operational_uplift: f64,
    pub synthesis_access_gate: f64,
    /// Multiplier on demography.age_creep (<= 1.0): weak, lagged fiscal
    /// relief. age_creep_eff = age_creep * age_creep_mult.
    pub age_creep_mult: f64,
    /// Realized pool marks ($B) for the valuation cross-section.
    pub ai_drug_pool_b: f64,
    pub longevity_pool_b: f64,
    pub bci_pool_b: f64,
    /// New bio/materials discovery contribution routed to the robots pool
    /// ($B-order, capex/chips-gated — competes for the same bottlenecks).
    pub bio_materials_pool_b: f64,
    /// Additive society-sentiment delta from the elite-longevity/merge
    /// frames (net backlash: negative sentiment / trust in-horizon).
    pub frame_sentiment_delta: f64,
    /// True if this year's structure warrants arming a society dread
    /// incident from a bio-source scandal (AI-drug clinical-failure or
    /// longevity reprogramming adverse event) — the caller draws it.
    pub dread_incident_pressure: f64,
}

fn ramp(a: f64, b: f64, t: f64, span: f64) -> f64 {
    a + (b - a) * (t / span).clamp(0.0, 1.0)
}

impl BioState {
    pub fn new(bp: &BioParams) -> Self {
        BioState {
            physical_discovery_cap: 0.0,
            drug_pipeline: 0.0,
            hc_rd_cost_index: 1.0,
            bio_knowledge_uplift: 0.0,
            synthesis_access_gate: bp.synth_gate_2026,
            cumulative_bio_incidents: 0,
            screening_mandate_armed: false,
            longevity_trl: 0.02,
            bci_maturity: 0.0,
            elite_longevity_frame: 0.15,
            merge_frame: 0.10,
            ai_drug_pool_b: bp.drug_pool_2026_b,
            longevity_pool_b: 0.06 * 1000.0 / 1000.0, // ~$0.06T narrow base
            bci_pool_b: 0.5,
            hc_cost_history: Vec::new(),
            years: 0,
        }
    }

    /// Called on a mass-casualty bio event: arm the mandatory-universal-
    /// synthesis-screening ratchet (the specific mechanism that re-closes
    /// `synthesis_access_gate`). Idempotent latch.
    pub fn arm_screening_mandate(&mut self) {
        self.screening_mandate_armed = true;
    }

    /// Register a bio incident for habituation / nth-event escalation.
    pub fn register_bio_incident(&mut self) {
        self.cumulative_bio_incidents += 1;
    }

    /// Advance one year.
    ///
    /// * `algo_eff`      — R1 recursive-capability stock (software).
    /// * `adoption`      — economy-wide AI adoption level (0..~0.9).
    /// * `capex_gate`    — capital/chips-availability proxy in [0,1] for
    ///                     growing automated experiment capacity (SDLs
    ///                     compete for the SAME power/chips, not free upside).
    /// * `open_weight_share` — capacity-weighted open-model share (0..1);
    ///                     sets the safeguard floor for the bio hazard.
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        bp: &BioParams,
        algo_eff: f64,
        adoption: f64,
        capex_gate: f64,
        open_weight_share: f64,
    ) -> BioOutputs {
        if bp.bio_layer <= 0.0 {
            // Layer off: neutral outputs, baseline exactly unchanged.
            return BioOutputs {
                bio_operational_uplift: 0.0,
                synthesis_access_gate: self.synthesis_access_gate,
                age_creep_mult: 1.0,
                ai_drug_pool_b: 0.0,
                longevity_pool_b: 0.0,
                bci_pool_b: 0.0,
                bio_materials_pool_b: 0.0,
                frame_sentiment_delta: 0.0,
                dread_incident_pressure: 0.0,
            };
        }
        let t = self.years as f64;
        let year = 2026 + self.years;
        self.years += 1;

        // ---- R1 -> VCT-style KNOWLEDGE uplift (deployment-independent;
        // this is frontier-model capability, not adoption). Log-normalized
        // on algo_eff so the 2026 intercept is already elevated. ----
        self.bio_knowledge_uplift =
            (algo_eff.max(1.0).ln() / bp.knowledge_uplift_ref.ln()).clamp(0.0, 1.0);

        // ---- synthesis-access gate: erodes UP (benchtop + open-weight
        // screening-evasion) unless a mandatory-screening ratchet re-closes
        // it. Slow stock. ----
        let erosion_target = ramp(bp.synth_gate_2026, bp.synth_gate_2036, t, 10.0);
        self.synthesis_access_gate += 0.25 * (erosion_target - self.synthesis_access_gate);
        if self.screening_mandate_armed {
            self.synthesis_access_gate =
                (self.synthesis_access_gate - bp.synth_reclose_step).max(0.05);
        }

        // ---- KNOWLEDGE -> OPERATIONAL translation (the Aum gate). This is
        // what reconciles RAND (5% actual wet-lab uplift) with VCT (2x
        // expert knowledge). Rises as wet-lab automation matures. ----
        let translation = ramp(bp.translation_coeff_2026, bp.translation_coeff_2036, t, 10.0);
        let bio_operational_uplift =
            self.bio_knowledge_uplift * translation * self.synthesis_access_gate;

        // ---- R1 -> physical discovery capability (SDL extension). Spillover
        // << 1, saturating at ~10-20 exp/dimension, cycle-time-floored via
        // the blended accel ceiling, then a validation haircut. ----
        let design_signal = self.bio_knowledge_uplift; // AI improves DESIGN
        let throughput_gate = capex_gate.clamp(0.0, 1.0); // one-time LEVEL step
        let raw = bp.r1_phys_spillover * design_signal * (0.3 + 0.7 * throughput_gate);
        // saturating approach to the accel ceiling (normalized to 0..1)
        let target = (bp.sdl_accel_ceiling / (bp.sdl_accel_ceiling + 1.0)) * (1.0 - bp.sdl_saturation)
            + bp.sdl_saturation * design_signal;
        self.physical_discovery_cap +=
            raw * (target - self.physical_discovery_cap).max(0.0);
        self.physical_discovery_cap = self.physical_discovery_cap.clamp(0.0, 1.0);
        let validated_cap = self.physical_discovery_cap * (1.0 - bp.validation_haircut);

        // ---- AI drug-discovery pool: front-end throughput RICH, clinical
        // attrition CLAMPED. Realized royalties back-loaded to >= 2029. ----
        let pharma_adopt = ramp(bp.pharma_adoption_2026, bp.pharma_adoption_2035, t, 9.0);
        // preclinical throughput (shots-on-goal per $): RATE up, adoption-
        // and cost-gated; feeds the pipeline stock.
        let preclinical_throughput =
            pharma_adopt * (1.0 / bp.preclinical_cost_mult) * (0.5 + validated_cap);
        self.drug_pipeline += preclinical_throughput;
        // B-attrition clamp: ~90% drain to failure (Phase 2/3 unchanged).
        self.drug_pipeline *= 1.0 - (1.0 - bp.clinical_loa);
        // tooling/SaaS pool ramps deterministically; royalty option only
        // after the first Ph3 readout window.
        let tooling = ramp(bp.drug_pool_2026_b, bp.drug_pool_2035_b, t, 9.0) * pharma_adopt
            / bp.pharma_adoption_2035;
        let royalty = if year >= bp.royalty_first_year {
            // milestones x LOA; single-digit royalties on a small realized base
            self.drug_pipeline * bp.clinical_loa * 0.5
        } else {
            0.0
        };
        self.ai_drug_pool_b = tooling + royalty;

        // ---- healthcare-cost deflation -> age_creep (WEAK + LAGGED) ----
        self.hc_rd_cost_index +=
            0.15 * (bp.preclinical_cost_mult - self.hc_rd_cost_index) * pharma_adopt;
        self.hc_cost_history.push(self.hc_rd_cost_index);
        let lag = bp.deflation_lag as usize;
        let lagged_cost = if self.hc_cost_history.len() > lag {
            self.hc_cost_history[self.hc_cost_history.len() - 1 - lag]
        } else {
            1.0
        };
        // deflation = R&D-cost drop x pharma-share x price-passthrough
        let rd_drop = (1.0 - lagged_cost).max(0.0);
        let healthcare_deflation =
            rd_drop * bp.pharma_share_of_health * bp.price_passthrough;
        let age_creep_mult = (1.0 - bp.age_creep_kappa * healthcare_deflation).clamp(0.90, 1.0);

        // ---- optionality: longevity / BCI (bounded, out-of-horizon) ----
        // longevity TRL: slow integrator, ~15-25yr time-constant; masked
        // in-horizon. Funding beta to capital abundance (capex_gate proxy).
        self.longevity_trl += 0.02 * validated_cap * capex_gate;
        self.longevity_trl = self.longevity_trl.min(1.0);
        // longevity pool: tiny high-beta, grows ~9-10% CAGR, funding-cyclical
        self.longevity_pool_b *= 1.0 + 0.095 * (0.5 + capex_gate);
        self.longevity_pool_b =
            self.longevity_pool_b.min(bp.longevity_pool_2035_b);
        // BCI clinical maturity gated by the regulatory PMA regime switch
        let pma = year >= bp.bci_pma_year;
        let channel_realization = 1.0 - bp.bci_roadmap_haircut;
        self.bci_maturity = if pma {
            (self.bci_maturity + 0.12 * channel_realization).min(1.0)
        } else {
            (self.bci_maturity + 0.03).min(0.4)
        };
        // BCI optionality: milestone step-up on the PMA switch, else drift
        let bci_step = if year == bp.bci_pma_year { 3.0 } else { 1.0 };
        self.bci_pool_b = (self.bci_pool_b * (1.0 + 0.12 * capex_gate) * bci_step)
            .min(bp.bci_pool_2035_b);

        // ---- memetic frames -> society (net backlash in-horizon) ----
        self.elite_longevity_frame =
            (self.elite_longevity_frame * 0.85 + 0.05 * adoption).min(1.0);
        // merge frame damped by the ~10 bits/s cognition ceiling (bci_maturity
        // cannot runaway-inflate it)
        self.merge_frame = (self.merge_frame * 0.85
            + 0.04 * self.bci_maturity * (1.0 - self.merge_frame))
            .min(0.6);
        // inequality-frame backlash: negative sentiment/trust, small
        let frame_sentiment_delta =
            -0.02 * self.elite_longevity_frame + 0.005 * self.merge_frame;

        // dread pressure: rises with pipeline size (more shots => more
        // chances of a high-profile clinical-failure/safety scandal) and
        // with reprogramming-trial maturity (partial-reprogramming cancer
        // risk). The caller turns this into a low-rate incident draw.
        let dread_incident_pressure =
            0.02 * (self.drug_pipeline / 20.0).min(1.0) + 0.02 * self.longevity_trl;

        // bio/materials value routed to robots pool (capex/chips-gated)
        let bio_materials_pool_b = 20.0 * validated_cap * capex_gate;

        // open-weight share is passed through to the shock hazard via the
        // returned uplift; a higher share means a lower effective safeguard.
        let _ = open_weight_share;

        BioOutputs {
            bio_operational_uplift,
            synthesis_access_gate: self.synthesis_access_gate,
            age_creep_mult,
            ai_drug_pool_b: self.ai_drug_pool_b,
            longevity_pool_b: self.longevity_pool_b,
            bci_pool_b: self.bci_pool_b,
            bio_materials_pool_b,
            frame_sentiment_delta,
            dread_incident_pressure,
        }
    }
}

// ===========================================================================
// DREAD SHOCK CLASSES: biorisk pandemic + cyber
//
// Distinct from the generic AI-incident (`Params::incident_year`) and from
// the geopolitics `ShockKind` table: capability-INDEXED hazards, different
// regulatory targets, fat-tailed severity. Applied by lib.rs alongside
// `GeoFx` and the society incident path. The deterministic baseline draws
// NOTHING (empty option), so the regression guard holds.
// ===========================================================================

pub mod shocks {
    use super::BioParams;
    use crate::geopolitics::GeoRng;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum DreadShockKind {
        /// Engineered/accidental-release bio event RIDING ON TOP of the
        /// always-on natural-pandemic background.
        BioPandemic,
        /// Systemic, correlated AI-orchestrated cyber event (CrowdStrike-
        /// class). Chronic ransomware is a steady GDP tax, NOT this shock.
        CyberSystemic,
    }

    #[derive(Debug, Clone, Copy)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct DreadShock {
        pub kind: DreadShockKind,
        pub year: i32,
        /// Transient GDP LEVEL drop (fraction), mean-reverts over 1-3yr.
        pub gdp_drag: f64,
        /// Additive reg_stringency step (ratcheted, permanent LEVEL).
        pub stringency_step: f64,
        /// Arms the R7 credit spiral + (bio) the screening mandate.
        pub mass_casualty: bool,
        /// Credit-spread injection (systemic/mass-casualty only).
        pub spread: f64,
        /// Cross-sectional differential: defensive-pool earnings uplift
        /// (BioDefense / cyber-security) as a multiple, decaying.
        pub defensive_pool_mult: f64,
        /// Broad-beta drawdown (fast partial recovery).
        pub broad_beta_hit: f64,
    }

    // ---- hazard functions -------------------------------------------------

    /// Biorisk hazard: h0 * exp(k*(cap_op - cap*)) * (1 - m_safeguard)
    /// plus the AI-independent natural-pandemic background. `cap_op` is the
    /// OPERATIONAL uplift from `BioOutputs::bio_operational_uplift`.
    pub fn h_bio(cap_op: f64, open_weight_share: f64) -> f64 {
        const H0: f64 = 0.003; // FRI baseline 100k-death human-caused /yr
        const H_NATURAL: f64 = 0.028; // COVID-scale natural pandemic /yr
        const CAP_STAR: f64 = 0.03; // VCT-expert crossing (already ~reached)
        // k pinned so full-knowledge (pre-translation ~cap_op up to ~0.35)
        // yields ~5x on the deliberate channel at the VCT crossing.
        const K: f64 = 5.0;
        // effective safeguard: capacity-weighted blend of closed (~0.75) and
        // open-weight (~0.0) model share.
        let m_safeguard = 0.75 * (1.0 - open_weight_share);
        let deliberate =
            H0 * ((K * (cap_op - CAP_STAR)).exp()) * (1.0 - m_safeguard);
        deliberate.max(0.0) + H_NATURAL
    }

    /// Cyber systemic hazard: sigmoid race between offense overhang and
    /// defender AI maturity, gated by a usable-autonomy threshold crossed
    /// ~2025. `offense` and `defender` are 0..1 capability indices; the SIGN
    /// (offense- vs defense-favored) is emergent, not hardcoded.
    pub fn h_cyber(offense: f64, defender: f64, year: i32) -> f64 {
        const H_C0: f64 = 0.05;
        let autonomy_gate = if year >= 2025 { 1.0 } else { 0.1 };
        let race = offense - defender;
        let sig = 1.0 / (1.0 + (-6.0 * race).exp());
        H_C0 * sig * autonomy_gate
    }

    /// Fat-tailed (lognormal) GDP-severity draw for a bio event. Median
    /// event ~ Amerithrax (macro ~0); 90-95th pct COVID-class (-3.4%); tail
    /// engineered-pandemic (-8..-12%+). Returns (gdp_drag, mass_casualty).
    fn draw_bio_severity(rng: &mut GeoRng) -> (f64, bool) {
        // z ~ approx-normal via sum of uniforms (dependency-free)
        let z = (0..6).map(|_| rng.next_f64()).sum::<f64>() - 3.0;
        // ln-severity: mean small, heavy right tail
        let sev = (-4.0 + 1.6 * z).exp(); // median ~e^-4 ~ 0.018
        let gdp_drag = sev.min(0.14);
        let mass_casualty = gdp_drag > 0.03; // COVID-class or worse
        (gdp_drag, mass_casualty)
    }

    fn draw_cyber_severity(rng: &mut GeoRng) -> f64 {
        // CrowdStrike-class modal ~ $5.4B ~ 0.02% world GDP transient;
        // heavier tail on correlated cloud/CVE events.
        let z = (0..6).map(|_| rng.next_f64()).sum::<f64>() - 3.0;
        (-8.0 + 1.4 * z).exp().min(0.03) // median tiny; occasional larger
    }

    /// Draw the 2026-2036 bio + cyber dread-shock path. Hazards are
    /// CAPABILITY-INDEXED: the caller supplies per-year capability tracks
    /// pre-computed from a candidate run (or a schedule). To keep this
    /// dependency-light we accept closures returning the year's capability.
    #[allow(clippy::too_many_arguments)]
    pub fn sample_dread_shocks(
        rng: &mut GeoRng,
        _bp: &BioParams,
        start_year: i32,
        end_year: i32,
        cap_op: impl Fn(i32) -> f64,
        open_weight_share: impl Fn(i32) -> f64,
        cyber_offense: impl Fn(i32) -> f64,
        cyber_defender: impl Fn(i32) -> f64,
    ) -> Vec<DreadShock> {
        let mut out = Vec::new();
        for year in start_year..=end_year {
            // -- biorisk --
            let hb = h_bio(cap_op(year), open_weight_share(year));
            if rng.next_f64() < hb {
                let (gdp_drag, mass) = draw_bio_severity(rng);
                out.push(DreadShock {
                    kind: DreadShockKind::BioPandemic,
                    year,
                    gdp_drag,
                    // mass-casualty engineered pandemic is a STRONGER dread
                    // archetype than the AI incident (+0.55): +0.6..0.8;
                    // a contained scare +0.15..0.25 (may not arm R7).
                    stringency_step: if mass { 0.70 } else { 0.20 },
                    mass_casualty: mass,
                    spread: if mass { 0.35 } else { 0.05 },
                    // Moderna template: +100..+400% decaying
                    defensive_pool_mult: if mass { 3.0 } else { 1.3 },
                    broad_beta_hit: if mass { 0.30 } else { 0.05 },
                });
            }
            // -- cyber (systemic only; chronic ransomware handled as a RATE
            // tax elsewhere) --
            let hc = h_cyber(cyber_offense(year), cyber_defender(year), year);
            if rng.next_f64() < hc {
                let gdp_drag = draw_cyber_severity(rng);
                let systemic = gdp_drag > 0.01;
                out.push(DreadShock {
                    kind: DreadShockKind::CyberSystemic,
                    year,
                    gdp_drag,
                    // cyber is familiar => weaker dread step than bio
                    stringency_step: if systemic { 0.30 } else { 0.10 },
                    mass_casualty: systemic,
                    // reflexive-financing coupling fires ONLY on correlated/
                    // systemic events (insurance covers 10-20%, ~50% ceded)
                    spread: if systemic { 0.20 } else { 0.0 },
                    // DUAL sign: pumps the cyber profit pool
                    defensive_pool_mult: 1.5,
                    broad_beta_hit: if systemic { 0.15 } else { 0.02 },
                });
            }
        }
        out
    }

    /// Aggregate this year's dread shocks into effects, with the anti-
    /// double-count guard: when multiple dread events co-occur in one year
    /// the combined reg_stringency step is the MAX class step (not the sum),
    /// and `dread_armed` is a single idempotent latch (the R7 spiral is a
    /// fixed injection, never super-added).
    #[derive(Debug, Clone, Copy, Default)]
    pub struct DreadFx {
        pub gdp_drag: f64,       // summed transient level drop
        pub stringency_step: f64, // MAX across co-occurring classes
        pub arm_dread: bool,
        pub arm_screening: bool,
        pub spread: f64,
        pub bio_defensive_mult: f64,
        pub cyber_defensive_mult: f64,
        pub broad_beta_hit: f64,
        pub incident: bool,
    }

    pub fn effects_for_year(shocks: &[DreadShock], year: i32) -> DreadFx {
        let mut fx = DreadFx::default();
        for s in shocks.iter().filter(|s| s.year == year) {
            fx.incident = true;
            // GDP drags from independent physical causes DO sum.
            fx.gdp_drag += s.gdp_drag;
            // stringency: take the strongest dread archetype, not the sum
            fx.stringency_step = fx.stringency_step.max(s.stringency_step);
            fx.spread += s.spread;
            fx.broad_beta_hit = fx.broad_beta_hit.max(s.broad_beta_hit);
            if s.mass_casualty {
                fx.arm_dread = true;
            }
            match s.kind {
                DreadShockKind::BioPandemic => {
                    fx.bio_defensive_mult = fx.bio_defensive_mult.max(s.defensive_pool_mult);
                    if s.mass_casualty {
                        fx.arm_screening = true;
                    }
                }
                DreadShockKind::CyberSystemic => {
                    fx.cyber_defensive_mult =
                        fx.cyber_defensive_mult.max(s.defensive_pool_mult);
                }
            }
        }
        fx
    }
}
