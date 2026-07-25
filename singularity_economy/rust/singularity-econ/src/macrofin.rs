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
            spread_passthrough: 0.25,
            rate_smoothing: 0.5,
            baseline_deficit: 0.06,
            gov_debt_2026: 1.0,
            dr_beta: 0.60,
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
}

impl MacroState {
    pub fn new(mp: &MacroParams) -> Self {
        let long_rate = mp.r_star_nominal + mp.tp_2026_bp / 10_000.0;
        MacroState {
            priv_duration: mp.priv_duration_2026,
            long_rate,
            gov_debt_gdp: mp.gov_debt_2026,
            tp_bp: mp.tp_2026_bp,
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

        // term premium on the STOCK excess over the 2026 anchor
        let tp_target = mp.tp_2026_bp
            + mp.term_premium_gain * (self.priv_duration - mp.priv_duration_2026) * 100.0;
        self.tp_bp += mp.rate_smoothing * (tp_target - self.tp_bp);
        let long_target = mp.r_star_nominal + self.tp_bp / 10_000.0;
        self.long_rate += mp.rate_smoothing * (long_target - self.long_rate);

        // sovereign snowball: debt/GDP += primary_deficit - (g - r)*debt
        let primary = mp.baseline_deficit + transfer_share * debt_share;
        self.gov_debt_gdp = (self.gov_debt_gdp + primary
            - (nominal_growth - self.long_rate) * self.gov_debt_gdp)
            .max(0.3);
        let debt_service = self.long_rate * self.gov_debt_gdp;

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
