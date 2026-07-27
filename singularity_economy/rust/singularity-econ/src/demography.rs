//! Demography layer: dynamic labor pools, youth blockage, care economy,
//! migration politics, solidarity, and intergroup tension.
//!
//! Design: output/history/demography_design.md (9 briefs) +
//! output/history/tension_supplement.json (2 briefs). Stance: demography
//! enters as (a) drifting sinks/sources on the two labor pools, (b) a
//! SIGNAL FILTER between raw displacement and the political rate-detector
//! in society.rs (hiring freezes and vacancy-filling robots produce zero
//! layoffs — keying politics on raw displacement fires 2-4 years early),
//! and (c) slow stocks (solidarity, blocked entrants, tension) that decide
//! WHICH political channel fires and how big the transfer response can be.
//!
//! Composition dynamics enter ONLY as published elasticities (solidarity
//! targets, the 2.5x perception multiplier, chauvinism erosion, the
//! restriction-first branch, tension inflow = migration-rate x economic
//! stress gated by salience). No group stocks, no group narratives; the
//! contested diversity->trust literature is deliberately NOT hard-coded
//! (meta-analyses attribute the effect to segregation + deprivation).

/// Tunables. All rates per year.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DemographyParams {
    // pool flows
    pub r_entry_cog: f64,
    pub r_ret_cog_2026: f64,
    pub r_ret_cog_2036: f64,
    pub r_entry_phys: f64,
    pub r_ret_phys: f64,
    /// Max share of the cognitive entry flow that AI absorbs as hiring
    /// freezes before any incumbent layoff (Dauth: 100% of robot
    /// incidence fell on entrants).
    pub divert_max: f64,
    /// Blocked entrants drain to underemployment (physical pool) at this
    /// rate. Audit B2: at 0.6 the ~0.37/yr retention pinned blocked_share at
    /// ~0.14 — below the 0.18 protest threshold — so Youth Channel B (the
    /// "historically revolutionary" underemployed-graduate stock) was
    /// structurally inert (youth_sentiment_inflow == 0 always). Lowered to 0.45:
    /// the stock persists enough that blocked_share reaches ~0.25 (inside the
    /// design's 0.18/0.25/0.40 band) and the channel fires. NOTE (calibration
    /// boundary): the channel is deliberately kept near its activation edge —
    /// dropping much below ~0.40 makes youth_sentiment_inflow (scaled by
    /// youth_gain*youth_salience = 5.0 and applied as a SATURATING rate on
    /// sentiment) large enough to pin sentiment at 1.0, which would need a joint
    /// re-tune of {drain, blocked_share cap, youth gains}. 0.45 keeps the peak
    /// inflow ~0.09 — meaningful but not dominating.
    pub underemploy_drain: f64,
    // youth channel
    pub blocked_init_m: f64,
    pub youth_gain: f64,
    pub youth_salience: f64,
    pub youth_protest_threshold: f64,
    // care economy
    pub care_demand_2026_m: f64,
    pub care_workers_2026_m: f64,
    pub care_demand_growth: f64,
    pub care_supply_cap: f64,
    pub care_pull_gain: f64,
    // migration / restriction
    pub openness_init: f64,
    pub restriction_step: f64,
    pub restriction_sent_relief: f64,
    pub restriction_mask: f64,
    // solidarity / tension
    pub solidarity_init: f64,
    pub solidarity_target_cog: f64,
    pub solidarity_target_phys: f64,
    pub solidarity_adjust: f64,
    pub chauvinism_k: f64,
    pub perceived_outgroup: f64,
    pub tension_gain: f64,
    pub tension_acute_decay: f64,
    pub redirect_share: f64,
    // fiscal aging
    pub age_creep: f64,
    /// Aging-core vacancy gap (M/yr) that robots fill before displacing.
    pub vacancy_gap_2026_m: f64,
    pub vacancy_gap_2036_m: f64,
}

impl Default for DemographyParams {
    fn default() -> Self {
        DemographyParams {
            r_entry_cog: 0.026,
            r_ret_cog_2026: 0.025,
            r_ret_cog_2036: 0.021,
            r_entry_phys: 0.026,
            r_ret_phys: 0.018,
            divert_max: 0.7,
            underemploy_drain: 0.45,
            blocked_init_m: 8.0,
            youth_gain: 2.0,
            youth_salience: 2.5,
            youth_protest_threshold: 0.18,
            care_demand_2026_m: 185.0,
            care_workers_2026_m: 162.0,
            care_demand_growth: 0.026,
            care_supply_cap: 0.02,
            care_pull_gain: 0.5,
            openness_init: 0.35,
            restriction_step: 0.20,
            restriction_sent_relief: 0.05,
            restriction_mask: 0.004,
            solidarity_init: 0.75,
            solidarity_target_cog: 0.80,
            solidarity_target_phys: 0.45,
            solidarity_adjust: 0.05,
            chauvinism_k: 0.5,
            perceived_outgroup: 0.35,
            tension_gain: 0.8,
            tension_acute_decay: 0.5,
            redirect_share: 0.35,
            age_creep: 0.0025,
            vacancy_gap_2026_m: 8.0,
            vacancy_gap_2036_m: 14.0,
        }
    }
}

/// Per-year outputs consumed by lib.rs / society wiring.
#[derive(Debug, Clone, Copy)]
pub struct DemographyOutputs {
    /// What the political rate-detector actually sees (share of pool/yr).
    pub visible_cog_rate: f64,
    pub visible_phys_rate: f64,
    /// Endogenized society parameters.
    pub attrition_threshold: f64,
    pub transfer_cap_eff: f64,
    pub transfer_step_mult: f64,
    /// Channel B: youth-blockage sentiment inflow (separate detector).
    pub youth_sentiment_inflow: f64,
    /// Dynamic pools (M).
    pub cog_pool_m: f64,
    pub phys_pool_m: f64,
    /// Care share reduces the robot-addressable slice of physical work.
    pub physical_addressable_eff: f64,
    /// Care-gap demand pull on the robot fleet (M units), 0 displacement.
    pub robot_pull_units_m: f64,
    /// Restriction fired this year: suppress the transfer step (the cheap
    /// first resort — migration is always cut before transfers scale).
    pub restriction_fired: bool,
    /// Tension redirect: share of B2 backlash routed AWAY from AI firms
    /// (scapegoating is perversely pro-adoption, corrosive socially).
    pub backlash_redirect: f64,
    /// Unrest hazard multiplier into the society layer.
    pub unrest_mult: f64,
}

#[derive(Debug, Clone)]
pub struct DemographyState {
    pub cog_pool_m: f64,
    pub phys_pool_m: f64,
    pub blocked_entrants_m: f64,
    pub care_demand_m: f64,
    pub care_workers_m: f64,
    pub migration_openness: f64,
    pub solidarity: f64,
    /// Intergroup tension: fast self-exciting acute component (half-life
    /// ~4 months => mostly gone within the annual step) + slow chronic
    /// component mediated by segregation/salience.
    pub tension: f64,
    prev_disp_level: f64,
    restriction_mask_years: i32,
    years: i32,
}

impl DemographyState {
    pub fn new(dp: &DemographyParams, cog_pool_m: f64, phys_pool_m: f64) -> Self {
        DemographyState {
            cog_pool_m,
            phys_pool_m,
            blocked_entrants_m: dp.blocked_init_m,
            care_demand_m: dp.care_demand_2026_m,
            care_workers_m: dp.care_workers_2026_m,
            migration_openness: dp.openness_init,
            solidarity: dp.solidarity_init,
            tension: 0.15,
            prev_disp_level: 0.0,
            restriction_mask_years: 0,
            years: 0,
        }
    }

    /// Advance one year. Inputs: cumulative cognitive displacement share
    /// (raw, from the AI layer), robot human-equivalent-work added this
    /// year (M), current sentiment, transfer share, GDP growth, election
    /// flag, physical displacement share.
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        dp: &DemographyParams,
        raw_cog_disp_level: f64,
        robot_hew_added_m: f64,
        sentiment: f64,
        transfer_share: f64,
        gdp_growth: f64,
        election_year: bool,
        phys_disp_level: f64,
        enh_solidarity_erosion: f64,
        age_creep_mult: f64,
    ) -> DemographyOutputs {
        let t = self.years as f64;
        let year_i = 2026 + self.years;
        self.years += 1;

        // ---- raw displacement demand this year (M workers) ----
        let raw_rate = (raw_cog_disp_level - self.prev_disp_level).max(0.0);
        self.prev_disp_level = raw_cog_disp_level;
        let d_cog_m = raw_rate * self.cog_pool_m;

        // ---- the core filter: entrants first (hiring freezes), then
        // incumbents; only incumbent layoffs are politically visible ----
        let entry_target = dp.r_entry_cog * self.cog_pool_m;
        let freeze = d_cog_m.min(dp.divert_max * entry_target);
        self.blocked_entrants_m += freeze;
        let visible_cog_rate =
            ((d_cog_m - freeze) / self.cog_pool_m.max(1.0)).max(0.0);

        // blocked entrants drain to underemployment (physical pool) and
        // scar-decay out of the political window
        let drain = dp.underemploy_drain * self.blocked_entrants_m;
        self.blocked_entrants_m =
            (self.blocked_entrants_m - drain) * (1.0 - 0.07);

        // ---- pools drift ----
        let r_ret_cog = dp.r_ret_cog_2026
            + (dp.r_ret_cog_2036 - dp.r_ret_cog_2026) * (t / 10.0).min(1.0);
        // Displacement does NOT shrink the pool: the pool is labor SUPPLY
        // (displaced workers still exist); only entry, retirement, and
        // the freeze diversion move it.
        self.cog_pool_m += entry_target - freeze - r_ret_cog * self.cog_pool_m;
        self.phys_pool_m += dp.r_entry_phys * self.phys_pool_m
            - dp.r_ret_phys * self.phys_pool_m
            + drain;

        // ---- care economy: demand grows with the 80+ cohort; supply is
        // churn-capped; the unmet gap pulls robots (zero displacement) ----
        self.care_demand_m *= 1.0 + dp.care_demand_growth;
        let care_gap = (self.care_demand_m - self.care_workers_m).max(0.0);
        self.care_workers_m += (dp.care_supply_cap * self.care_workers_m)
            .min(care_gap);
        // care_gap is in M worker-equivalents; each robot supplies ~1.4
        // HEW, and effectiveness is heavily discounted pre-2031 (Japan's
        // 20-yr eldercare-robot experiment: ~0 substitution) rising to
        // ~0.15 by 2040. Converts to robot UNITS (red-team fix: was /1000).
        let care_robot_eff = if year_i < 2031 { 0.0 } else { (0.15 * (t - 5.0) / 9.0).clamp(0.0, 0.15) };
        let robot_pull_units_m = dp.care_pull_gain * care_gap * care_robot_eff / 1.4;

        // ---- physical visibility: robots fill the aging-core vacancy gap
        // before they displace anyone ----
        let vacancy_gap = dp.vacancy_gap_2026_m
            + (dp.vacancy_gap_2036_m - dp.vacancy_gap_2026_m) * (t / 10.0).min(1.0);
        let visible_phys_rate = ((robot_hew_added_m - vacancy_gap
            - dp.care_supply_cap * self.care_workers_m)
            .max(0.0)
            / self.phys_pool_m.max(1.0))
        .min(1.0);

        // ---- endogenous attrition threshold: political slack = churn
        // (constant 1.5%) + retirement absorption; reproduces the legacy
        // 0.040 at t0 and decays as the boomer wave passes ----
        let attrition_threshold = r_ret_cog + 0.015;

        // ---- solidarity: pulled toward the composition-implied target of
        // whoever is being displaced (wave 1 in-group-coded cognitive,
        // wave 2 physical), eroded by chauvinism when transfers are large ----
        let phys_wave = (phys_disp_level / 0.10).min(1.0);
        let target = dp.solidarity_target_cog
            + (dp.solidarity_target_phys - dp.solidarity_target_cog) * phys_wave;
        self.solidarity += dp.solidarity_adjust * (target - self.solidarity)
            - dp.chauvinism_k * transfer_share * dp.perceived_outgroup
                * self.tension.max(0.2)
            // biological enhancement is the ultimate out-group coding —
            // a heritable, purchasable, permanent advantage; its salience
            // erodes redistributive solidarity beyond income inequality.
            - 0.05 * enh_solidarity_erosion;
        self.solidarity = self.solidarity.clamp(0.2, 0.95);

        // ---- tension: inflow = migration-rate x economic-stress gated by
        // salience (sentiment proxies national salience); acute component
        // decays fast (self-excitation half-life months, not years).
        // Immigration LEVEL deliberately absent (contested/near-null). ----
        // economic stress (Miguel arm) + salience; the chronic component
        // tracks displacement-driven grievance so scapegoating is a live
        // channel, not a floor-pinned inert stock (red-team fix).
        // The economic arm is DISPLACEMENT (the live dislocation that drives
        // scapegoating even in a boom), not just recession: the prior
        // (-gdp_growth) term is dead whenever growth is positive, which
        // collapsed tension into a mere lag of sentiment. Keep the recession
        // term (live in busts) and add the displacement driver so tension is an
        // independent channel (audit A6).
        let disp_level = (raw_cog_disp_level + phys_disp_level).max(0.0);
        let econ_stress = 0.15 * disp_level
            + (-gdp_growth / 0.05).max(0.0)
            + 2.0 * (sentiment - 0.3).max(0.0);
        let inflow = dp.tension_gain * econ_stress * (1.0 - self.tension);
        self.tension = self.tension * dp.tension_acute_decay + inflow + 0.02;
        self.tension = self.tension.clamp(0.0, 1.0);

        // ---- restriction-first branch: at elections under pressure, the
        // system cuts migration BEFORE it scales transfers (1921/1930/
        // 1964/1973/2016-25 pattern). Vents salience, not grievance. ----
        // The valve FIRES on salience alone (restriction is the cheap
        // first resort — its trigger sits deliberately below the layoff
        // threshold); whether it SUBSTITUTES for the transfer step is a
        // separate, solidarity-gated question answered by the caller.
        let mut restriction_fired = false;
        if election_year
            && self.migration_openness > 0.15
            && (visible_cog_rate > 0.025 || sentiment > 0.35)
        {
            self.migration_openness =
                (self.migration_openness - dp.restriction_step).max(0.05);
            self.restriction_mask_years = 3;
            restriction_fired = true;
        } else if sentiment < 0.30 {
            // reopening is an order of magnitude slower (10:1 asymmetry)
            // and only happens once salience has cooled
            self.migration_openness = (self.migration_openness + 0.02).min(0.6);
        }
        let masked_visible_cog = if self.restriction_mask_years > 0 {
            self.restriction_mask_years -= 1;
            (visible_cog_rate - dp.restriction_mask).max(0.0)
        } else {
            visible_cog_rate
        };

        // ---- youth channel B: stock-share detector, 3-yr window ----
        let entry_cohort = entry_target.max(1.0);
        let blocked_share = self.blocked_entrants_m / (3.0 * entry_cohort);
        let youth_sentiment_inflow = dp.youth_gain
            * dp.youth_salience
            * (blocked_share - dp.youth_protest_threshold).max(0.0).powf(1.5);

        // ---- transfer politics gates ----
        // age_creep_mult (<=1.0, audit C13) is the bio layer's weak, lagged
        // healthcare-deflation relief on the demographic aging drain — it slows
        // the fiscal-space erosion age_creep imposes. 1.0 when bio is off/absent.
        let transfer_cap_eff = (0.15 - dp.age_creep * age_creep_mult * t).max(0.08)
            * (0.70 + 0.30 * self.solidarity);
        let transfer_step_mult = 0.5 + 0.5 * self.solidarity;

        DemographyOutputs {
            visible_cog_rate: masked_visible_cog,
            visible_phys_rate,
            attrition_threshold,
            transfer_cap_eff,
            transfer_step_mult,
            youth_sentiment_inflow,
            cog_pool_m: self.cog_pool_m,
            phys_pool_m: self.phys_pool_m,
            physical_addressable_eff: (0.65
                - self.care_workers_m / self.phys_pool_m.max(1.0))
            .max(0.4),
            robot_pull_units_m,
            restriction_fired: restriction_fired && self.solidarity < 0.55,
            backlash_redirect: dp.redirect_share * self.tension,
            unrest_mult: 1.0 + 1.5 * self.tension,
        }
    }
}
