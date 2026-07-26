//! Society layer: political economy, regulation, and consumer dynamics.
//!
//! Design source: output/history/society_design.md (11 mechanism briefs,
//! July 2026). Two principles govern everything here:
//!
//! 1. **Politics is a RATE detector with saturating level terms.** Every
//!    inflow keys off the displacement *rate* (or incident pulses), never
//!    the displaced level. Autos killed 40k/yr for decades unregulated;
//!    TMI killed ~0 and froze an industry — stringency tracks
//!    dread x concentration x zero-warning, not body count.
//! 2. **Asymmetric reversibility.** Stringency ratchets up and never
//!    decays inside the horizon (railroad deregulation took 93 years);
//!    sentiment spikes fast and decays in years; transfers step at
//!    elections (or in weeks under crisis) and partially ratchet.
//!
//! Double-counting guards (from the design doc): this layer's only
//! adoption actuators are the endogenized B2 multiplier, the regulation
//! rate-penalty, and the consumer-trust ceiling; its only financial
//! channel injects into the existing B4 credit loop; its only GDP effects
//! are transfer demand-support, compliance cost, and sovereign-debt
//! crowding — the existing transition drag keeps reallocation friction.

/// Tunable parameters for the society layer. All rates are per year.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SocietyParams {
    // -- S1 public sentiment (grievance salience) --
    /// Displacement-rate threshold below which job loss is silently
    /// absorbed into attrition (ILWU: ~4%/yr).
    pub attrition_threshold: f64,
    /// Inflow gain on max(0, disp_rate - threshold)^1.5.
    pub sentiment_gain: f64,
    /// AI harms are attributable to named firms (Standard Oil pattern),
    /// unlike diffuse "trade" harms. Multiplier 1..3.
    pub attributability: f64,
    /// Sentiment half-life unremediated (years).
    pub sentiment_halflife: f64,
    /// Sentiment half-life once transfers > 3% GDP (grievance drains).
    pub sentiment_halflife_relieved: f64,

    // -- B5 relief valve (transfers) --
    /// Sustained displacement rate that puts transfers on the next
    /// election's agenda (2pp/yr).
    pub transfer_trigger_rate: f64,
    /// Displacement-rate spike that triggers crisis mode: transfers step
    /// within a year, bypassing the election calendar (COVID precedent).
    pub crisis_rate: f64,
    /// Transfer step at an election trigger (share of GDP).
    pub transfer_step: f64,
    /// Transfer step in crisis mode (CARES-class).
    pub transfer_crisis_step: f64,
    /// Cap on transfer share of GDP inside the horizon.
    pub transfer_cap: f64,
    /// Fraction of each emergency step that ratchets permanent
    /// (CTC/UI lapsed; Alaska PFD endured: 0.3 baseline).
    pub ratchet_fraction: f64,
    /// Years of sustained trigger-rate displacement required before the
    /// election channel acts (two-election rule ~= 2yr sustained + next
    /// even-year election).
    pub sustained_years: i32,
    /// Election cadence in years (US midterm rhythm).
    pub election_period: i32,
    /// B2 attenuation while transfers are active (relief valve is the
    /// historically-grounded path to fast adoption).
    pub transfer_backlash_damp: f64,

    // -- B6/B7/B8 regulation --
    /// Annual stringency drip while sentiment stays above s1_reg_threshold.
    pub reg_drip: f64,
    pub s1_reg_threshold: f64,
    /// Lobby suppression of the drip below the outrage ceiling
    /// (SB 1047 veto), vanishing above it (99-1 Senate vote).
    pub capture_suppression: f64,
    pub capture_s1_ceiling: f64,
    /// First-order time constant from statute to enforcement (GDPR:
    /// 2018 -> 2021 fines; ICC +19yr is the long tail).
    pub enforcement_tau: f64,
    /// Adoption-rate penalty at full enforcement (GDPR class: -25%).
    pub reg_adoption_penalty: f64,
    /// GDP compliance cost at full enforcement (share of gdp growth).
    pub compliance_drag: f64,
    /// Dread-incident stringency step (TMI class).
    pub incident_s2_dread: f64,
    /// Ordinary-major incident stringency step.
    pub incident_s2_major: f64,
    /// Sentiment pulse from an incident.
    pub incident_s1: f64,
    /// Consumer-trust hit from a first incident (habituates x0.5/repeat).
    pub incident_trust_hit: f64,

    // -- S4 labor power / R6 leverage erosion --
    pub labor_power_2026: f64,
    pub organizing_gain: f64,
    /// Years by which displacement erodes labor power (pickets need
    /// members; erosion follows displacement with a lag).
    pub erosion_lag: i32,

    // -- S5 institutional trust / R5 radicalization --
    pub inst_trust_2026: f64,
    /// Trust floor below which backlash reroutes anti-system: B2 halves
    /// but policy incoherence doubles the election period.
    pub trust_reroute_floor: f64,

    // -- S6 consumer trust / B10+R8 --
    pub consumer_trust_2026: f64,
    pub familiarity_gain: f64,
    /// Adoption ceiling = min(0.90, ceiling_base + consumer_trust).
    pub consumer_ceiling_base: f64,

    // -- macro couplings --
    /// Precautionary-savings drag on the demand signal when sentiment
    /// is elevated (GFC: +4pp saving rate).
    pub precautionary_gain: f64,
    /// Transfers offset transition drag up to this fraction.
    pub transfer_drag_offset: f64,
    /// Sovereign debt/GDP starting point and crowding-out tolerance.
    pub gov_debt_2026: f64,
    pub gov_debt_tolerance: f64,
    pub crowding_gain: f64,

    // -- loop switches for ablation (1.0 = on) --
    pub b5_relief: f64,
    pub b6_regulation: f64,
    pub b8_capture: f64,
    pub r6_erosion: f64,
    pub r8_familiarity: f64,
}

impl Default for SocietyParams {
    fn default() -> Self {
        SocietyParams {
            attrition_threshold: 0.04,
            sentiment_gain: 6.0,
            attributability: 2.5,
            sentiment_halflife: 7.0,
            sentiment_halflife_relieved: 2.5,
            transfer_trigger_rate: 0.02,
            // Crisis bypass must sit ABOVE the transfer trigger, else every
            // triggered transfer also counts as a crisis and the election-gated
            // 0.04-step path is dead. Aligned with the >4pp/yr emergency-powers
            // throttle: only a genuine displacement crisis fires the 0.08 step.
            crisis_rate: 0.045,
            transfer_step: 0.04,
            transfer_crisis_step: 0.08,
            transfer_cap: 0.15,
            ratchet_fraction: 0.3,
            sustained_years: 2,
            election_period: 2,
            transfer_backlash_damp: 0.7,
            reg_drip: 0.02,
            s1_reg_threshold: 0.3,
            capture_suppression: 0.55,
            capture_s1_ceiling: 0.55,
            enforcement_tau: 3.5,
            reg_adoption_penalty: 0.25,
            compliance_drag: 0.02,
            incident_s2_dread: 0.55,
            incident_s2_major: 0.15,
            incident_s1: 0.12,
            incident_trust_hit: 0.20,
            labor_power_2026: 0.12,
            organizing_gain: 0.02,
            erosion_lag: 5,
            inst_trust_2026: 0.27,
            trust_reroute_floor: 0.20,
            consumer_trust_2026: 0.50,
            familiarity_gain: 0.05,
            consumer_ceiling_base: 0.40,
            precautionary_gain: 0.04,
            transfer_drag_offset: 0.5,
            gov_debt_2026: 1.0,
            gov_debt_tolerance: 1.2,
            crowding_gain: 0.3,
            b5_relief: 1.0,
            b6_regulation: 1.0,
            b8_capture: 1.0,
            r6_erosion: 1.0,
            r8_familiarity: 1.0,
        }
    }
}

/// Mutable society stocks, stepped once per simulated year.
#[derive(Debug, Clone)]
pub struct SocietyState {
    pub sentiment: f64,        // S1, 0..1 grievance salience
    pub reg_stringency: f64,   // S2, statute ratchet
    pub reg_enforcement: f64,  // S2e, first-order follower of S2
    pub transfer_permanent: f64,
    pub transfer_emergency: f64,
    pub labor_power: f64,      // S4
    pub inst_trust: f64,       // S5
    pub consumer_trust: f64,   // S6
    pub gov_debt_gdp: f64,
    /// Unrest intensity (0..1): rises under unremediated high sentiment
    /// with low institutional trust; costs adoption and GDP (the design's
    /// R5 consequence — trust collapse must not be pro-adoption).
    pub unrest: f64,
    years_disp_hot: i32,
    years_s1_high: i32,
    years_transfers_active: i32,
    incidents_seen: u32,
    trust_latched: bool,
    emergency_fired: bool,
}

impl SocietyState {
    pub fn new(sp: &SocietyParams) -> Self {
        SocietyState {
            sentiment: 0.20,
            reg_stringency: 0.08,
            reg_enforcement: 0.02,
            transfer_permanent: 0.005,
            transfer_emergency: 0.0,
            labor_power: sp.labor_power_2026,
            inst_trust: sp.inst_trust_2026,
            consumer_trust: sp.consumer_trust_2026,
            gov_debt_gdp: sp.gov_debt_2026,
            unrest: 0.0,
            years_disp_hot: 0,
            years_transfers_active: 0,
            years_s1_high: 0,
            incidents_seen: 0,
            trust_latched: false,
            emergency_fired: false,
        }
    }

    pub fn transfer_share(&self) -> f64 {
        self.transfer_permanent + self.transfer_emergency
    }

    /// B11 fiscal collision: when sovereign debt service tops ~4.5% of
    /// GDP, the effective transfer cap erodes — the bond market squeezes
    /// the fiscal room the displacement path requires.
    pub fn throttle_transfer_cap(&mut self, squeeze: f64) {
        let cap = (self.transfer_permanent - squeeze).max(0.0);
        self.transfer_permanent = self.transfer_permanent.min(cap.max(0.02));
    }

    /// Dynamic debt-financing share of transfers: early transfers are ~60%
    /// debt-financed, falling toward ~15% once a winner-tax base matures after
    /// ~5 active-transfer years. Exposed so the macro-finance layer prices the
    /// sovereign snowball off the SAME dynamic share instead of a hardcoded 0.6.
    pub fn debt_financing_share(&self) -> f64 {
        if self.years_transfers_active > 5 { 0.15 } else { 0.6 }
    }

    /// External grievance injection (e.g. a food-price shock): adds directly to
    /// public sentiment, the same stock displacement backlash feeds. Clamped to
    /// the 0..1 salience range. Off unless a caller wires a positive coupling.
    pub fn add_external_stress(&mut self, s: f64) {
        self.sentiment = (self.sentiment + s.max(0.0)).clamp(0.0, 1.0);
    }

    fn transfers_active(&self) -> bool {
        self.transfer_share() > 0.03
    }

    /// Endogenized B2 multiplier (design doc §5.1). Log-law in the rate
    /// (0 at the attrition threshold, ~1 at 0.3%/yr, ~2 at 3%/yr),
    /// scaled by salience, labor power, transfer damping, and the
    /// anti-system reroute. Normalized so ~1.0 at a mid-transition
    /// pre-relief state; the constant-gain legacy behavior is recovered
    /// with the layer off.
    pub fn backlash_multiplier(&self, sp: &SocietyParams, disp_rate: f64) -> f64 {
        let log_law = (disp_rate.max(1e-9) / 0.0003).log10().max(0.0);
        let salience = 0.3 + self.sentiment;
        let labor = (self.labor_power / sp.labor_power_2026).max(0.1).sqrt();
        let damp = if self.transfers_active() {
            1.0 - sp.transfer_backlash_damp
        } else {
            1.0
        };
        let reroute = if self.inst_trust < sp.trust_reroute_floor {
            0.5
        } else {
            1.0
        };
        (log_law * salience * labor * damp * reroute / 2.0).min(3.0)
    }

    /// Multiplier on this year's adoption INCREMENT (never the level —
    /// regulation slows the rate; it does not un-adopt). Unrest adds its
    /// own throttle: strikes and street pressure slow deployment even
    /// without statutes (IMF: -15-30% activity effects for 2-3 years).
    pub fn adoption_rate_mult(&self, sp: &SocietyParams) -> f64 {
        (1.0 - sp.reg_adoption_penalty * self.reg_enforcement * sp.b6_regulation)
            * (1.0 - 0.25 * self.unrest)
    }

    /// GDP cost of unrest (level effect, distinct from transition drag).
    pub fn unrest_gdp_cost(&self) -> f64 {
        0.01 * self.unrest
    }

    /// Consumer-trust ceiling on the adoption level (B10).
    pub fn adoption_ceiling(&self, sp: &SocietyParams) -> f64 {
        (sp.consumer_ceiling_base + self.consumer_trust).min(0.90)
    }

    /// Demand-signal drag from precautionary savings (sentiment cuts
    /// consumption before displacement does).
    pub fn precautionary_drag(&self, sp: &SocietyParams) -> f64 {
        if self.sentiment > 0.5 {
            sp.precautionary_gain * (self.sentiment - 0.5) / 0.5
        } else {
            0.0
        }
    }

    /// Extra credit tightening injected into B4: sovereign crowding-out
    /// plus the R7 regulatory-cost spiral (only armed by a dread incident
    /// with high enforcement — the nuclear conjunction).
    pub fn credit_injection(&self, sp: &SocietyParams, dread_armed: bool) -> f64 {
        let crowding =
            sp.crowding_gain * (self.gov_debt_gdp - sp.gov_debt_tolerance).max(0.0);
        crowding + self.credit_spiral(dread_armed)
    }

    /// The R7 regulatory-cost SPIRAL portion of the credit injection, WITHOUT the
    /// sovereign-crowding term. Used when B11 is on and the macro layer already
    /// owns crowding via the risk-free rate — so we add the spiral at full
    /// strength instead of scaling the combined signal by 0.3 (which had kept
    /// 30% of crowding too, a partial double-count).
    pub fn credit_spiral(&self, dread_armed: bool) -> f64 {
        if dread_armed && self.reg_enforcement > 0.5 {
            0.15
        } else {
            0.0
        }
    }

    /// GDP-growth adjustment: transfers offset part of transition drag;
    /// compliance costs subtract.
    pub fn drag_multiplier(&self, sp: &SocietyParams) -> f64 {
        1.0 - sp.transfer_drag_offset * (self.transfer_share() / 0.05).min(1.0)
    }

    pub fn compliance_cost(&self, sp: &SocietyParams) -> f64 {
        sp.compliance_drag * self.reg_enforcement * sp.b6_regulation
    }

    /// Advance all stocks by one year.
    ///
    /// `disp_rate` — this year's realized cognitive-displacement increment;
    /// `adoption` — this year's adoption level;
    /// `lagged_disp_rate` — displacement increment `erosion_lag` years ago;
    /// `election_year` — calendar even-year flag;
    /// `incident`/`dread` — shock flags for this year (MC-drawn; 0 in the
    /// deterministic baseline).
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        sp: &SocietyParams,
        disp_rate: f64,
        adoption: f64,
        adoption_accel: f64,
        lagged_disp_rate: f64,
        election_year: bool,
        incident: bool,
        dread: bool,
    ) {
        // ---- S1 sentiment: rate-triggered inflow with a 1-yr anticipation
        // lead (WGA struck at ~0% realized displacement) and logistic-style
        // saturation instead of a hard clamp (red-team round 3: the pinned
        // 1.0 plateau was a saturation artifact, not a pulse).
        let anticipation = 0.5 * adoption_accel.max(0.0);
        let inflow = (sp.sentiment_gain
            * (disp_rate - sp.attrition_threshold).max(0.0).powf(1.5)
            * sp.attributability
            + anticipation)
            * (1.0 - self.sentiment);
        let halflife = if self.transfers_active() {
            sp.sentiment_halflife_relieved
        } else {
            sp.sentiment_halflife
        };
        let decay = 1.0 - (2f64.ln() / halflife).min(0.9);
        self.sentiment = (self.sentiment * decay + inflow).min(1.0);
        if incident {
            let habituation = 0.5f64.powi(self.incidents_seen as i32);
            self.sentiment = (self.sentiment + sp.incident_s1 * habituation).min(1.0);
        }

        // ---- B5 relief valve: staircase, never smooth
        self.transfer_emergency *= 0.5; // emergency portion decays tau ~1yr
        if sp.b5_relief > 0.0 {
            if disp_rate > sp.transfer_trigger_rate {
                self.years_disp_hot += 1;
            } else {
                self.years_disp_hot = 0;
            }
            let incoherent = self.inst_trust < sp.trust_reroute_floor;
            let crisis = disp_rate > sp.crisis_rate;
            let scheduled = self.years_disp_hot >= sp.sustained_years
                && election_year
                && !incoherent;
            if (crisis || scheduled) && self.transfer_share() < sp.transfer_cap {
                let step = if crisis {
                    sp.transfer_crisis_step
                } else {
                    sp.transfer_step
                };
                self.transfer_permanent += sp.ratchet_fraction * step;
                self.transfer_emergency += (1.0 - sp.ratchet_fraction) * step;
                // cap binds on the total, trimming the emergency portion
                let over = (self.transfer_share() - sp.transfer_cap).max(0.0);
                self.transfer_emergency = (self.transfer_emergency - over).max(0.0);
            }
        }
        // Debt-financed at first; the winner-tax arrives with the design's
        // ~5-7yr lag and takes over most funding — giving the sovereign
        // path an EQUILIBRIUM instead of an unbounded slide (red-team
        // round 3 final: credit erosion had no fixed point and the test
        // passed only by horizon truncation).
        if self.transfers_active() {
            self.years_transfers_active += 1;
        }
        let debt_share = if self.years_transfers_active > 5 { 0.15 } else { 0.6 };
        self.gov_debt_gdp = (self.gov_debt_gdp + self.transfer_share() * debt_share
            - 0.03 * self.gov_debt_gdp)
            .max(0.5);

        // ---- S2/S2e regulation: capture-gated drip + incident steps, ratchet
        if sp.b6_regulation > 0.0 {
            if self.sentiment > sp.s1_reg_threshold {
                self.years_s1_high += 1;
            } else {
                self.years_s1_high = 0;
            }
            if self.years_s1_high >= 2 {
                let capture = if sp.b8_capture > 0.0
                    && self.sentiment < sp.capture_s1_ceiling
                {
                    sp.capture_suppression
                } else {
                    0.0
                };
                self.reg_stringency += sp.reg_drip * (1.0 - capture);
            }
            if incident {
                self.reg_stringency += if dread {
                    sp.incident_s2_dread
                } else {
                    sp.incident_s2_major
                };
            }
            // Emergency-powers throttle (design §3): a displacement-rate
            // crisis >4pp/yr triggers executive action in WEEKS, bypassing
            // the legislative calendar; ~30% of the step ratchets.
            if disp_rate > 0.04 && !self.emergency_fired {
                self.reg_stringency += 0.09;
                self.emergency_fired = true;
            }
            self.reg_stringency = self.reg_stringency.min(1.0);
            self.reg_enforcement +=
                (self.reg_stringency - self.reg_enforcement) / sp.enforcement_tau;
        }

        // ---- S4 labor power: organizing vs R6 leverage erosion
        self.labor_power = (self.labor_power + sp.organizing_gain * self.sentiment
            - sp.r6_erosion * lagged_disp_rate * self.labor_power * 4.0)
            .clamp(0.02, 1.0);

        // ---- unrest (R5 consequence): unremediated grievance + low trust
        // boils over; drains once transfers flow or sentiment cools.
        let boiling = self.sentiment > 0.6
            && !self.transfers_active()
            && self.inst_trust < 0.35;
        self.unrest = if boiling {
            (self.unrest + 0.35).min(1.0)
        } else {
            self.unrest * 0.5
        };
        if self.unrest > 0.3 {
            self.inst_trust = (self.inst_trust - 0.03).max(0.05);
        }

        // ---- S5 institutional trust
        let mut dt = -0.005;
        if disp_rate > sp.transfer_trigger_rate && !self.transfers_active() {
            dt -= 0.02; // unremediated displacement corrodes trust
        }
        if self.transfers_active() {
            dt += 0.01; // delivery rebuilds it slowly
        }
        self.inst_trust = (self.inst_trust + dt).clamp(0.05, 0.90);

        // ---- S6 consumer trust: familiarity vs incident shocks (latch)
        if incident {
            let habituation = 0.5f64.powi(self.incidents_seen as i32);
            let hit = sp.incident_trust_hit * habituation * if dread { 1.5 } else { 1.0 };
            self.consumer_trust = (self.consumer_trust - hit).max(0.05);
            if self.inst_trust < 0.30 {
                self.trust_latched = true; // GMO-Europe absorbing state
            }
            self.incidents_seen += 1;
        }
        let recovery_mult = if self.trust_latched { 0.1 } else { 1.0 };
        self.consumer_trust = (self.consumer_trust
            + sp.r8_familiarity * sp.familiarity_gain * adoption
                * (1.0 - self.consumer_trust)
                * recovery_mult)
            .min(1.0);
    }
}
