//! Macro-finance layer: endogenous long rates, sovereign-debt snowball,
//! and per-scenario discount rates.
//!
//! Spec: output/history/gap_scan.json thread #1 (B11 endogenous long rate,
//! scored 9.5/10 — "the model proves the thesis is fiscally contingent,
//! then values every cash flow as if the bond market never notices").
//!
//! The bond market prices the transfer ramp: net duration supply
//! (sovereign coupon issuance + AI IG issuance, less Fed absorption)
//! drives a term premium at ~4bp per 1pp-of-GDP of privately-held
//! 10y-equivalents. That long rate then (a) tightens B4 capital via the
//! risk-free curve, (b) drives the sovereign-debt snowball (replacing the
//! hand-coded society.rs debt drain with r-vs-g dynamics), (c) squeezes
//! the transfer cap once debt service exceeds ~4.5% of GDP, and (d) sets
//! a per-scenario discount rate in the valuation. Named module `macrofin`
//! because `macro` is a reserved word in Rust.

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MacroParams {
    /// bp of 10y term premium per 1pp-of-GDP of privately-held
    /// 10y-equivalents (QE-era literature clusters ~3.7-4.4bp; STOCK
    /// basis). MC: lognormal(ln 4, p10=1/p90=8).
    pub term_premium_gain: f64,
    /// Neutral nominal anchor for the long rate (r* + inflation).
    pub r_star_nominal: f64,
    /// 2026 term premium (bp), ACM ~70.
    pub tp_2026_bp: f64,
    /// 2026 privately-held duration stock (fraction of GDP).
    pub priv_duration_2026: f64,
    /// Share of the deficit issued as duration (vs bills).
    pub dur_factor: f64,
    /// AI IG issuance duration factor (long-tenor).
    pub ig_dur_factor: f64,
    /// Fraction of the term-premium move landing on private credit
    /// spreads vs the risk-free curve (sovereign supply lands ~85-95% on
    /// the risk-free curve — convenience-yield channel).
    pub spread_passthrough: f64,
    /// Info smoothing on the long rate (1-yr delay).
    pub rate_smoothing: f64,
    /// Baseline primary deficit (share of GDP) before transfers.
    pub baseline_deficit: f64,
    /// 2026 sovereign debt/GDP.
    pub gov_debt_2026: f64,
    /// Equity-duration beta: discount rate rises this much per 1.0 of
    /// long-rate above the 4.5% anchor.
    pub dr_beta: f64,
    /// Fiscal-dominance / inflation regime (gate, 0 = legacy duration-supply-only
    /// long rate). The 2026 selloff to 5.1% came from term premium + fiscal +
    /// sticky inflation; the legacy rate only knew duration supply and reached that
    /// level in 2036. At > 0 two premia join the long rate from 2027:
    /// an inflation-expectations premium that tracks the TOTAL deficit (primary +
    /// debt service) above `deficit_threshold` and real growth above 3% (boom
    /// demand pressure), and a convex fiscal risk premium on debt/GDP above its
    /// 2026 level. Endogenous inflation also enters nominal growth, so the
    /// inflation tax erodes debt/GDP — the channel that keeps the answer to "why
    /// not 15%?" finite. Capped at `max_long_rate`.
    pub fiscal_dominance_gain: f64,
    /// pp of expected inflation per pp of total deficit above the threshold.
    pub inflation_fiscal_gain: f64,
    /// pp of expected inflation per pp of real growth above 3%.
    pub inflation_boom_gain: f64,
    pub deficit_threshold: f64,
    pub inflation_smoothing: f64,
    /// bp of fiscal risk premium per pp of debt/GDP above 2026 (Laubach ~3-4).
    pub fiscal_rp_bp_per_pp: f64,
    /// Extra bp per pp^2 (convexity: markets charge more as debt compounds).
    pub fiscal_rp_convexity: f64,
    pub max_long_rate: f64,
    /// Bohn fiscal reaction under the regime: the primary balance improves this
    /// much per unit of debt/GDP above 2026 (Bohn 2008 ~0.03-0.10). The stabilizer
    /// that keeps fiscal dominance from being a mechanical doom loop.
    pub bohn_response: f64,
}

impl Default for MacroParams {
    fn default() -> Self {
        MacroParams {
            term_premium_gain: 4.0,
            r_star_nominal: 0.038,
            tp_2026_bp: 70.0,
            priv_duration_2026: 0.55,
            dur_factor: 0.55,
            ig_dur_factor: 1.2,
            // Audit B7: sovereign duration supply lands ~85-95% on the risk-free
            // curve (convenience-yield channel), i.e. only ~5-15% leaks to private
            // credit spreads. The old 0.25 contradicted this module's own field
            // doc and understated the risk-free B4 injection the comment says
            // dominates. 0.10 sits mid-band.
            spread_passthrough: 0.10,
            rate_smoothing: 0.5,
            // PRIMARY deficit (ex-interest), ~3% of GDP (re-audit #16). Interest
            // enters the snowball only through the (r-g) term; the old 0.06 was the
            // TOTAL-deficit magnitude and double-counted interest.
            baseline_deficit: 0.03,
            gov_debt_2026: 1.0,
            dr_beta: 0.60,
            fiscal_dominance_gain: 0.0,
            inflation_fiscal_gain: 0.35,
            inflation_boom_gain: 0.30,
            deficit_threshold: 0.045,
            inflation_smoothing: 0.5,
            fiscal_rp_bp_per_pp: 3.0,
            fiscal_rp_convexity: 0.05,
            max_long_rate: 0.15,
            bohn_response: 0.05,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MacroOutputs {
    /// 10y risk-free long rate (fraction).
    pub long_rate: f64,
    /// Extra B4 credit tightening from the risk-free curve.
    pub credit_injection: f64,
    /// Debt service as share of GDP.
    pub debt_service: f64,
    /// Transfer-cap erosion once debt service tops ~4.5% of GDP.
    pub transfer_cap_squeeze: f64,
    pub gov_debt_gdp: f64,
    pub term_premium_bp: f64,
}

#[derive(Debug, Clone)]
pub struct MacroState {
    priv_duration: f64,
    long_rate: f64,
    gov_debt_gdp: f64,
    tp_bp: f64,
    /// Effective coupon on the OUTSTANDING debt stock (re-audit #16): legacy debt
    /// was issued at past rates and reprices only as it rolls (~15%/yr toward the
    /// current 10y), so debt service is coupon x stock — not the whole stock
    /// instantly repriced at today's long rate.
    coupon: f64,
    /// Fiscal-dominance state: expected-inflation premium (fraction) and the
    /// fiscal risk premium (bp); steps taken (the 2026 step is the observed year).
    infl_premium: f64,
    fiscal_rp_bp: f64,
    steps: u32,
    debt_service_prev: f64,
}

impl MacroState {
    pub fn new(mp: &MacroParams) -> Self {
        let long_rate = mp.r_star_nominal + mp.tp_2026_bp / 10_000.0;
        MacroState {
            priv_duration: mp.priv_duration_2026,
            long_rate,
            gov_debt_gdp: mp.gov_debt_2026,
            tp_bp: mp.tp_2026_bp,
            // ~3.3%: the 2026 average coupon on the legacy stock (issued across the
            // low-rate decade), below the 4.5%+ marginal 10y.
            coupon: 0.033,
            infl_premium: 0.0,
            fiscal_rp_bp: 0.0,
            steps: 0,
            debt_service_prev: 0.033 * mp.gov_debt_2026,
        }
    }

    /// Advance one year. `transfer_share` and `debt_share` come from the
    /// society layer; `ai_ig_issuance` is AI-sector external funding as a
    /// share of GDP (B4 duration supply); `nominal_growth` dilutes stocks.
    pub fn step(
        &mut self,
        mp: &MacroParams,
        transfer_share: f64,
        debt_share: f64,
        ai_ig_issuance: f64,
        nominal_growth: f64,
    ) -> MacroOutputs {
        // net duration supply into private hands (fraction of GDP)
        let sovereign_supply =
            (mp.baseline_deficit + transfer_share * debt_share) * mp.dur_factor;
        let ig_supply = ai_ig_issuance * mp.ig_dur_factor;
        // stock accumulates, diluted by nominal growth
        self.priv_duration = (self.priv_duration + sovereign_supply + ig_supply
            - self.priv_duration * nominal_growth.max(0.0))
        .max(0.0);

        // term premium on the STOCK excess over the 2026 anchor. Smoothed ONCE
        // (re-audit #18): smoothing tp_bp and then long_rate toward a target built
        // from the already-smoothed tp_bp compounded to ~25% year-one passthrough
        // instead of the documented ~50% 1-yr delay, lagging the B4 credit and
        // q-governor responses by roughly a year.
        let tp_target = mp.tp_2026_bp
            + mp.term_premium_gain * (self.priv_duration - mp.priv_duration_2026) * 100.0;
        self.tp_bp += mp.rate_smoothing * (tp_target - self.tp_bp);
        self.long_rate = mp.r_star_nominal + self.tp_bp / 10_000.0;
        let fd = mp.fiscal_dominance_gain.max(0.0);
        let mut nominal_growth = nominal_growth;
        if fd > 0.0 && self.steps > 0 {
            let primary = mp.baseline_deficit + transfer_share * debt_share;
            let total_deficit = primary + self.debt_service_prev;
            let real_growth = nominal_growth - 0.02;
            let infl_target = fd
                * (mp.inflation_fiscal_gain * (total_deficit - mp.deficit_threshold).max(0.0)
                    + mp.inflation_boom_gain * (real_growth - 0.03).max(0.0));
            self.infl_premium += mp.inflation_smoothing * (infl_target - self.infl_premium);
            let excess_pp = ((self.gov_debt_gdp - mp.gov_debt_2026) * 100.0).max(0.0);
            let rp_target = fd
                * (mp.fiscal_rp_bp_per_pp * excess_pp + mp.fiscal_rp_convexity * excess_pp * excess_pp);
            self.fiscal_rp_bp += mp.rate_smoothing * (rp_target - self.fiscal_rp_bp);
            self.long_rate = (self.long_rate + self.infl_premium + self.fiscal_rp_bp / 10_000.0)
                .min(mp.max_long_rate);
            nominal_growth += self.infl_premium;
        }
        self.steps += 1;

        // sovereign snowball: debt/GDP += PRIMARY deficit - (g - r)*debt. Interest
        // enters ONLY through the (r-g) term (re-audit #16: baseline_deficit had been
        // set to the ~6% TOTAL-deficit magnitude while r also entered via -(g-r)d,
        // double-counting interest).
        let mut primary = mp.baseline_deficit + transfer_share * debt_share;
        if fd > 0.0 && self.steps > 1 {
            primary -= fd.min(1.0) * mp.bohn_response * (self.gov_debt_gdp - mp.gov_debt_2026).max(0.0);
        }
        self.gov_debt_gdp = (self.gov_debt_gdp + primary
            - (nominal_growth - self.long_rate) * self.gov_debt_gdp)
            .max(0.3);
        // Debt service prices off the effective COUPON, which rolls toward the
        // current 10y as legacy stock matures (~15%/yr) — not the whole stock
        // repriced instantly (re-audit #16: that fired the 4.5% fiscal collision
        // unconditionally from 2026 with transfers still ~0).
        self.coupon += 0.15 * (self.long_rate - self.coupon);
        let debt_service = self.coupon * self.gov_debt_gdp;
        self.debt_service_prev = debt_service;

        // the term-premium move splits: (1-passthrough) on the risk-free
        // curve tightening B4 capital, passthrough onto private spreads
        // (already in sector_debt), so B4 gets the risk-free part
        let rate_excess = (self.long_rate - 0.045).max(0.0);
        let credit_injection = (1.0 - mp.spread_passthrough) * rate_excess * 20.0;

        MacroOutputs {
            long_rate: self.long_rate,
            credit_injection,
            debt_service,
            transfer_cap_squeeze: 0.5 * (debt_service - 0.045).max(0.0),
            gov_debt_gdp: self.gov_debt_gdp,
            term_premium_bp: self.tp_bp,
        }
    }

    pub fn long_rate(&self) -> f64 {
        self.long_rate
    }
}
