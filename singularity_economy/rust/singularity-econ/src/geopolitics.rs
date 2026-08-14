//! Geopolitical shock module.
//!
//! Spec: output/history/geopolitics_shock_table.md (10 research briefs +
//! architect synthesis, July 2026). Shocks are EXPLICIT DRAWS carried in
//! `Params::geo_shocks` — the deterministic baseline (empty vec) is exactly
//! unchanged, satisfying the spec's regression guard: "geopolitics must be
//! a tail module, not a hidden drag." The Monte Carlo sampler implements
//! the hazard table and the escalation ladder (Markov, NOT independent
//! draws: quarantines escalate to blockades 25%/yr; a Taiwan blockade
//! implies a minerals embargo at p=0.8; export-control rungs draw
//! tit-for-tat minerals squeezes at p=0.8 within the year).

/// Shock taxonomy S1..S8 from the design table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShockKind {
    TaiwanCrisisSpike,   // S1
    TaiwanQuarantine,    // S2 — flow shock: capacity intact, exports throttled
    TaiwanBlockade,      // S3 — recoverable stock shock (LNG buffer 11 days)
    TaiwanInvasion,      // S4 — permanent destruction, EUV-capped rebuild
    ExportControlRung,   // S5
    MineralsSqueeze,     // S6 — licensing pulse (Apr-2025 pattern: +72% magnets)
    MineralsEmbargo,     // S7 — hard supply gate on robot components
    EnergyChokepoint,    // S8 — never touches chips (channel separation)
}

/// One drawn shock episode.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoShock {
    pub kind: ShockKind,
    pub start_year: i32,
    pub duration_years: f64,
}

/// Combined effect vector for one simulated year.
#[derive(Debug, Clone, Copy)]
pub struct GeoFx {
    pub chip_mult: f64,       // multiplies effective chip capacity
    pub chip_destruction: f64, // permanent stock loss applied once (S4)
    pub rebuild_cap: bool,     // post-invasion: chip growth capped at 18%/yr
    pub power_mult: f64,       // multiplies energized-GW additions
    pub comp_cost_mult: f64,   // robot component cost
    pub comp_supply_mult: f64, // robot component availability
    pub metals_target: f64,    // metals-index shock target (mean-reverting)
    pub demand_mult: f64,      // AI capex demand
    pub spread: f64,           // credit-spread injection (fraction, 600bp=0.6)
    pub onshoring: bool,       // scare response: supply buildout boost
    pub rare_earth_embargo: bool, // S6/S7 minerals shock — triggers the Liebig China cut
}

impl Default for GeoFx {
    fn default() -> Self {
        GeoFx {
            chip_mult: 1.0,
            chip_destruction: 0.0,
            rebuild_cap: false,
            power_mult: 1.0,
            comp_cost_mult: 1.0,
            comp_supply_mult: 1.0,
            metals_target: 1.0,
            demand_mult: 1.0,
            spread: 0.0,
            onshoring: false,
            rare_earth_embargo: false,
        }
    }
}

/// Per-kind effect constants (design table §1). Severity decay for
/// minerals shocks (12%/yr post-2028: substitution + ex-China ramp) and
/// the falling Taiwan share for chip shocks are applied by the caller
/// via `year`.
fn kind_fx(kind: ShockKind, year: i32, active: bool, years_since_end: i32) -> GeoFx {
    let mut fx = GeoFx::default();
    // Ex-China mitigation decays minerals severity after 2028.
    let decay = 0.88f64.powi((year - 2028).max(0));
    // Non-Taiwan leading-edge share rises ~4pp/yr: invasion/blockade
    // haircuts shrink over time (silicon-shield erosion works both ways).
    let taiwan_share = (0.90 - 0.04 * (year - 2026).max(0) as f64).max(0.60);
    if active {
        match kind {
            ShockKind::TaiwanCrisisSpike => {
                fx.chip_mult = 0.95;
                fx.comp_cost_mult = 1.08;
                fx.metals_target = 1.05;
                fx.demand_mult = 1.02; // panic pre-ordering
                fx.spread = 0.15;
            }
            ShockKind::TaiwanQuarantine => {
                fx.chip_mult = 0.70; // flow throttle, capacity intact
                fx.power_mult = 0.87;
                fx.comp_cost_mult = 1.30;
                fx.metals_target = 1.15;
                fx.demand_mult = 0.90;
                fx.spread = 0.25;
            }
            ShockKind::TaiwanBlockade => {
                fx.chip_mult = 1.0 - taiwan_share * 0.85; // output to ~10-30%
                fx.power_mult = 0.35; // no GPUs to energize halls
                fx.comp_cost_mult = 2.0;
                fx.comp_supply_mult = 0.6;
                fx.metals_target = 1.5;
                fx.demand_mult = 0.70;
                fx.spread = 0.60;
            }
            ShockKind::TaiwanInvasion => {
                fx.chip_destruction = taiwan_share; // permanent, once
                fx.chip_mult = 1.0; // destruction handles it
                fx.rebuild_cap = true;
                fx.power_mult = 0.15;
                fx.comp_cost_mult = 3.0;
                fx.comp_supply_mult = 0.3;
                fx.metals_target = 2.5;
                fx.demand_mult = 0.35;
                fx.spread = 1.0;
            }
            ShockKind::ExportControlRung => {
                fx.chip_mult = 0.99;
                fx.power_mult = 0.97;
                fx.comp_cost_mult = 1.10;
                fx.metals_target = 1.25;
                fx.spread = 0.075;
            }
            ShockKind::MineralsSqueeze => {
                fx.comp_cost_mult = 1.0 + 0.5 * decay;
                fx.power_mult = 0.97;
                fx.metals_target = 1.0 + 0.4 * decay;
                fx.spread = 0.025;
                fx.rare_earth_embargo = true;
            }
            ShockKind::MineralsEmbargo => {
                fx.chip_mult = 0.98;
                fx.power_mult = 0.88; // BESS/solar/transformer imports
                fx.comp_cost_mult = 1.0 + 2.5 * decay;
                fx.comp_supply_mult = 1.0 - 0.85 * decay; // hard magnet gate
                fx.metals_target = 1.0 + 2.0 * decay;
                fx.demand_mult = 0.95;
                fx.spread = 0.20;
                fx.rare_earth_embargo = true;
            }
            ShockKind::EnergyChokepoint => {
                // Never touches chips (channel-separation invariant).
                fx.power_mult = 0.95;
                fx.comp_cost_mult = 1.05;
                fx.metals_target = 1.35;
                fx.demand_mult = 0.96;
                fx.spread = 0.15;
            }
        }
    } else if years_since_end >= 0 && years_since_end < 3 {
        // Post-shock tails: reshoring boom after major chip shocks,
        // power catch-up after blockade, demand rebuild after invasion.
        match kind {
            ShockKind::TaiwanBlockade => {
                fx.power_mult = 1.2;
                fx.demand_mult = 1.35;
                fx.onshoring = true;
            }
            ShockKind::TaiwanInvasion => {
                fx.rebuild_cap = true; // EUV output caps rebuild for years
                fx.demand_mult = 1.4; // state-directed rebuild
                fx.onshoring = true;
            }
            ShockKind::TaiwanQuarantine | ShockKind::MineralsEmbargo => {
                fx.onshoring = true;
            }
            _ => {}
        }
    }
    fx
}

/// Aggregate all shocks into one effect vector for `year`.
pub fn effects_for_year(shocks: &[GeoShock], year: i32) -> GeoFx {
    let mut agg = GeoFx::default();
    // Same-kind episodes must not stack multiplicatively (one embargo
    // counted twice squares a single physical supply gate): only the
    // first ACTIVE episode of each kind applies; tails dedupe likewise.
    // Two-pass (re-audit #15): episodes arrive chronologically, so an OLDER
    // episode's post-shock tail used to be processed before a NEWER same-kind
    // active episode — the tail-branch active_seen guard was dead code, and a
    // repeat crisis had its demand collapse nearly cancelled by the prior
    // episode's recovery tail. First pass: mark kinds with an active episode
    // this year; second pass: tails of those kinds are skipped outright.
    let mut has_active = [false; 8];
    for s in shocks {
        let end = s.start_year as f64 + s.duration_years;
        if year >= s.start_year && (year as f64) < end {
            has_active[s.kind as usize] = true;
        }
    }
    let mut active_seen = [false; 8];
    let mut tail_seen = [false; 8];
    for s in shocks {
        let end = s.start_year as f64 + s.duration_years;
        let active = year >= s.start_year && (year as f64) < end;
        let since_end = year - end.ceil() as i32;
        let idx = s.kind as usize;
        if active {
            if active_seen[idx] {
                continue;
            }
            active_seen[idx] = true;
        } else if (0..3).contains(&since_end) {
            if tail_seen[idx] || has_active[idx] {
                continue;
            }
            tail_seen[idx] = true;
        }
        // Pro-rate sub-year durations in the start year.
        let weight = if active {
            s.duration_years.min(1.0).min(end - year as f64).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let fx = kind_fx(s.kind, year, active, since_end);
        let blend = |m: f64| 1.0 + (m - 1.0) * weight;
        agg.chip_mult *= blend(fx.chip_mult);
        agg.power_mult *= blend(fx.power_mult);
        agg.comp_cost_mult *= blend(fx.comp_cost_mult);
        agg.comp_supply_mult *= blend(fx.comp_supply_mult);
        agg.demand_mult *= blend(fx.demand_mult);
        agg.metals_target = agg.metals_target.max(blend(fx.metals_target));
        agg.spread += fx.spread * weight;
        if year == s.start_year {
            agg.chip_destruction = agg.chip_destruction.max(fx.chip_destruction);
        }
        agg.rebuild_cap |= fx.rebuild_cap;
        agg.onshoring |= fx.onshoring;
        agg.rare_earth_embargo |= fx.rare_earth_embargo;
    }
    agg
}

// ---------------------------------------------------------------------------
// Monte Carlo sampler: hazard table + escalation ladder.
// Dependency-free xorshift64* PRNG so buck2 builds stay third-party-free.
// ---------------------------------------------------------------------------

pub struct GeoRng(pub u64);

impl GeoRng {
    pub fn new(seed: u64) -> Self {
        GeoRng(seed.max(1))
    }
    pub fn next_f64(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Draw one 2026-2036 shock path per the hazard table and ladder.
pub fn sample_shocks(rng: &mut GeoRng, start_year: i32, end_year: i32) -> Vec<GeoShock> {
    let mut shocks = Vec::new();
    let mut escalation: f64 = 2.0; // 2026: chronic friction
    let mut invaded = false;
    let mut blockade_active_until = start_year - 100;
    let mut quarantine_active_until = start_year - 100;
    let mut s1_recent = start_year - 100;
    let mut s5_tension_until = start_year - 100;
    let mut squeezes_recent: Vec<i32> = Vec::new();

    for year in start_year..=end_year {
        if invaded {
            break; // absorbing: effects carried by the drawn S4 episode
        }
        let window = if (2027..=2030).contains(&year) { 1.5 } else { 1.0 };
        let tension = if year <= s5_tension_until { 1.25 } else { 1.0 };

        let push = |k: ShockKind, dur: f64, v: &mut Vec<GeoShock>| {
            v.push(GeoShock { kind: k, start_year: year, duration_years: dur });
        };

        // S5 export-control rung: 25%/yr post-2018 regime.
        if rng.next_f64() < 0.25 {
            push(ShockKind::ExportControlRung, 1.0, &mut shocks);
            escalation += 1.0;
            s5_tension_until = year + 2;
            // S6 tit-for-tat within the year at p=0.8.
            if rng.next_f64() < 0.80 {
                push(ShockKind::MineralsSqueeze, 0.35, &mut shocks);
                squeezes_recent.push(year);
            }
        }
        // S6 standalone: trimmed so total squeeze count (standalone +
        // tit-for-tat) lands in the contract band E[count] in [2,4].
        let p_s6 = (0.24 - 0.02 * (year - start_year) as f64).max(0.16);
        if rng.next_f64() < p_s6 {
            push(ShockKind::MineralsSqueeze, 0.35, &mut shocks);
            squeezes_recent.push(year);
        }
        squeezes_recent.retain(|&y| year - y <= 3);

        // S1 crisis spike.
        let p_s1 = 0.08 * (1.0 + 0.25 * (escalation - 2.0).max(0.0)) * window;
        if rng.next_f64() < p_s1.min(0.6) {
            push(ShockKind::TaiwanCrisisSpike, 0.15, &mut shocks);
            s1_recent = year;
        }

        // Ladder: quarantine -> blockade -> invasion.
        if year <= quarantine_active_until && rng.next_f64() < 0.25 {
            push(ShockKind::TaiwanBlockade, 1.0, &mut shocks);
            blockade_active_until = year + 1;
            quarantine_active_until = start_year - 100;
            escalation += 1.5;
            if rng.next_f64() < 0.80 {
                push(ShockKind::MineralsEmbargo, 1.5, &mut shocks);
            }
        } else if year <= blockade_active_until && rng.next_f64() < 0.25 {
            push(ShockKind::TaiwanInvasion, 4.0, &mut shocks);
            invaded = true;
            if rng.next_f64() < 0.80 {
                push(ShockKind::MineralsEmbargo, 2.0, &mut shocks);
            }
        } else {
            // Base rates trimmed below the spec's per-shock marginals: the
            // spec's analytic target is ~45% cumulative for S2-or-worse,
            // and the escalation ladder (S1 ratchet, S5 tension, S2->S3
            // transitions) adds mass the independent marginals don't
            // carry — so the bases are calibrated to preserve the TOTAL
            // (validation contract #1), not the marginals.
            let s1_mult = if year - s1_recent <= 1 { 1.25 } else { 1.0 };
            let p_s2 = 0.025 * window * s1_mult;
            let p_s3 = 0.012 * window * tension;
            let p_s4 = 0.010
                * window
                * tension
                * if year > 2032 { 0.85f64.powi(year - 2032) } else { 1.0 };
            let r = rng.next_f64();
            // Invasions are overwhelmingly ladder-gated; a 10% surprise
            // allowance covers bolt-from-blue (validation contract #3).
            let surprise_ok = escalation >= 3.0 || rng.next_f64() < 0.10;
            if r < p_s4 {
                // The invasion band consumes its probability mass whether or not the
                // bolt-from-blue gate passes (re-audit #14): a blocked surprise means
                // DETERRENCE — no shock — not a fall-through into the blockade band,
                // which was silently converting ~90% of blocked invasion mass into
                // phantom blockades in calm years.
                if surprise_ok {
                    push(ShockKind::TaiwanInvasion, 4.0, &mut shocks);
                    invaded = true;
                    if rng.next_f64() < 0.80 {
                        push(ShockKind::MineralsEmbargo, 2.0, &mut shocks);
                    }
                }
            } else if r < p_s4 + p_s3 {
                push(ShockKind::TaiwanBlockade, 1.0, &mut shocks);
                blockade_active_until = year + 1;
                escalation += 1.5;
                if rng.next_f64() < 0.80 {
                    push(ShockKind::MineralsEmbargo, 1.5, &mut shocks);
                }
            } else if r < p_s4 + p_s3 + p_s2 {
                push(ShockKind::TaiwanQuarantine, 0.75, &mut shocks);
                quarantine_active_until = year + 1;
                escalation += 0.5;
            }
        }

        // S7 standalone (ratchet if repeated squeezes).
        let p_s7 = 0.015 * if squeezes_recent.len() >= 2 { 1.5 } else { 1.0 };
        if rng.next_f64() < p_s7 {
            push(ShockKind::MineralsEmbargo, 1.5, &mut shocks);
        }

        // S8 energy chokepoint: 8%/yr recurrence from 2027. The spec's
        // "2026 initializes active" is deliberately NOT drawn: the model's
        // 2026 anchors are calibrated to the as-is world, which already
        // contains the active Hormuz/Red Sea state — re-injecting it
        // double-counts (red-team round 3 follow-up).
        let p_s8 = if year == start_year { 0.0 } else { 0.08 };
        if rng.next_f64() < p_s8 {
            push(ShockKind::EnergyChokepoint, 0.75, &mut shocks);
        }

        escalation = (escalation - 0.3).max(2.0);
    }
    shocks
}
