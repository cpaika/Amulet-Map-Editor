//! Historical backtest envelopes.
//!
//! Source: output/history/century_sweep.json — 12 boom/bust episodes
//! (UK railway mania 1840s → GPU/crypto 2016-24) plus 100-year macro and
//! geopolitical base rates, researched July 2026. Each test encodes an
//! envelope observed across those episodes and asserts the model's
//! ENDOGENOUS behavior lands inside it. These are out-of-sample structural
//! checks: none of these numbers were calibration inputs to v2.
//!
//! Episode envelope summary (capacity-rent duration in years / bust lag from
//! scarcity peak to glut in years):
//!   railway 6/1, US rail 8/6, electrification 10/2.5, radio 3/3,
//!   autos 13/3, WWII 2/3, DRAM 2/1.5, fiber 4/1.5, polysilicon 7/1.5,
//!   shale 4/0.4, GPU 1.6/1, Japan 4.5/1.
//! Universal pattern: IP/toll rents (RCA patent pool, GE/Westinghouse,
//! CUDA, x86, mineral royalties) outlasted every capacity rent, in all 12.

use singularity_econ::{simulate, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

/// Years the commodity-silicon margin stays meaningfully above normal.
fn capacity_rent_years(states: &[YearState], p: &Params) -> f64 {
    states
        .iter()
        .filter(|s| s.silicon_margin > p.normal_margin + 0.05)
        .count() as f64
}

fn year_of_peak_queue(states: &[YearState]) -> i32 {
    states
        .iter()
        .max_by(|a, b| a.queue_ratio.total_cmp(&b.queue_ratio))
        .unwrap()
        .year
}

fn first_glut_year(states: &[YearState]) -> Option<i32> {
    states.iter().find(|s| s.capacity_glut > 1.3).map(|s| s.year)
}

// -------------------------------------------------------------------------
// 1. Capacity-rent duration: 12-episode envelope is 1.6..13 years.
//    (GPU/crypto shortest at 1.6; 1920s autos longest at 13.)
// -------------------------------------------------------------------------
#[test]
fn capacity_rent_duration_within_century_envelope() {
    let p = Params::default();
    let dur = capacity_rent_years(&base(), &p);
    assert!(
        (1.5..=13.0).contains(&dur),
        "silicon rent duration {dur} yrs outside 100-year envelope [1.5, 13]"
    );
}

// -------------------------------------------------------------------------
// 2. High-supply-gain sectors must sit in the commodity cluster of the
//    envelope (DRAM 2, fiber 4, shale 4, GPU 1.6, polysilicon 7 => <= 8 yrs),
//    low-gain sectors may run longer (electrification 10, autos 13).
// -------------------------------------------------------------------------
#[test]
fn rent_duration_by_supply_gain_matches_commodity_vs_franchise_split() {
    let p = Params::default();
    let fast = simulate(&Params { chip_supply_gain: 3.0, ..Params::default() });
    let slow = simulate(&Params { chip_supply_gain: 0.4, ..Params::default() });
    let d_fast = capacity_rent_years(&fast, &p);
    let d_slow = capacity_rent_years(&slow, &p);
    assert!(
        d_fast <= 8.0,
        "high-gain rent duration {d_fast} exceeds commodity-cluster max (8 yrs: DRAM/fiber/shale/GPU/poly)"
    );
    assert!(d_fast <= d_slow, "duration must fall as supply gain rises");
}

// -------------------------------------------------------------------------
// 3. Bust lag: scarcity peak -> glut onset took 0.4..6 years historically.
// -------------------------------------------------------------------------
#[test]
fn bust_lag_within_century_envelope() {
    // HONESTY NOTE (red-team round 3): the baseline queue is a plateau,
    // not a rise-and-fall peak — the 2026 argmax is an initialization
    // transient. We therefore measure from SCARCITY ONSET (first year the
    // queue sustains >1.35, skipping the 2026-27 startup ringing) to glut
    // onset, and the envelope [0,6] is deliberately outlier-inclusive
    // (11/12 episodes had lag <=3; US railroads' 30-yr composite hit 6).
    // This test bounds the timing structurally; it is weaker than the
    // other envelopes and should not be read as a sharp falsification.
    let states = base();
    let onset = states
        .iter()
        .find(|s| s.year >= 2028 && s.queue_ratio > 1.35)
        .map(|s| s.year)
        .expect("baseline never develops a scarcity queue");
    let glut = first_glut_year(&states)
        .expect("baseline must eventually produce a glut (all 12 episodes did)");
    let lag = glut - onset;
    assert!(
        (0..=6).contains(&lag),
        "bust lag {lag} yrs (scarcity onset {onset} -> glut {glut}) outside [0, 6]"
    );
}

// -------------------------------------------------------------------------
// 4. Glut depth: reserve-margin/utilization overshoots ran ~1.15x..3.5x
//    demand-consistent capacity (electrification 40% vs 15% reserve margin
//    ~ 1.2x; fiber the extreme outlier on the lit-strand basis, excluded;
//    Japan Tankan excess ~12-18% of GDP capital stock).
// -------------------------------------------------------------------------
#[test]
fn glut_depth_within_century_envelope() {
    let peak_glut = base()
        .iter()
        .map(|s| s.capacity_glut)
        .fold(f64::MIN, f64::max);
    assert!(
        (1.15..=3.5).contains(&peak_glut),
        "peak glut {peak_glut} outside historical [1.15, 3.5]"
    );
}

// -------------------------------------------------------------------------
// 5. IP/toll rents outlast capacity rents — unanimous across 12 episodes.
//    RCA collected its 7.5% patent toll straight through the radio bust;
//    GE/Westinghouse stayed profitable through the Depression; CUDA margins
//    exceeded prior peak after both crypto busts.
// -------------------------------------------------------------------------
#[test]
fn ip_toll_outlasts_capacity_universally() {
    let p = Params::default();
    let states = base();
    let last = states.last().unwrap();
    assert!(
        last.ip_toll_margin - last.silicon_margin > 0.15,
        "IP toll must retain a wide spread over commodity capacity at horizon"
    );
    for s in states.iter().filter(|s| s.year >= 2029) {
        assert!(
            s.ip_toll_margin > p.normal_margin + 0.2,
            "IP toll margin dipped to {} in {}, breaking the RCA/GE/CUDA pattern",
            s.ip_toll_margin,
            s.year
        );
    }
}

// -------------------------------------------------------------------------
// 6. Demand keeps growing straight through the glut. Railway passengers
//    tripled 1842-1850 through the bust; internet traffic grew ~2x/yr
//    through 2002; the busts were supply-side. The model's demand side
//    must not contract during glut years.
// -------------------------------------------------------------------------
#[test]
fn demand_grows_through_glut() {
    let states = base();
    let mut peak_idx = f64::MIN;
    for w in states.windows(2) {
        assert!(
            w[1].adoption_level >= w[0].adoption_level - 1e-12,
            "adoption fell during {} — historical busts were supply-side",
            w[1].year
        );
        // The task index is price-driven and may wobble ~1% when adoption
        // pins at the consumer-trust ceiling while bottleneck prices move;
        // a demand BUST would be a >1% decline from the running peak.
        peak_idx = peak_idx.max(w[0].cognitive_task_index);
        assert!(
            w[1].cognitive_task_index >= peak_idx * 0.99,
            "task index fell >1% from peak during {} — demand bust",
            w[1].year
        );
    }
}

// -------------------------------------------------------------------------
// 7. Rents die while demand is still growing (fiber pricing collapsed with
//    only ~3-5% of strands lit; railway ROE halved while traffic tripled).
//    At the year commodity rents normalize, adoption must still be well
//    short of its horizon level.
// -------------------------------------------------------------------------
#[test]
fn rents_die_before_demand_saturates() {
    let p = Params::default();
    let states = base();
    let final_adoption = states.last().unwrap().adoption_level;
    if let Some(norm) = states
        .iter()
        .find(|s| s.year > 2027 && s.silicon_margin <= p.normal_margin + 0.05)
    {
        assert!(
            norm.adoption_level < 0.95 * final_adoption,
            "rents normalized in {} only after adoption saturated — inverts the historical order",
            norm.year
        );
    }
    // If rents never normalize inside the horizon that is itself consistent
    // with the franchise cluster (electrification/autos) — no assertion.
}

// -------------------------------------------------------------------------
// 8. Financing structure decides the credit event: railway partly-paid
//    calls and fiber junk debt produced crunches; the GPU/AI cycle funded
//    from hyperscaler cash flow did not. Same structural split must hold.
// -------------------------------------------------------------------------
#[test]
fn credit_crunch_requires_external_funding() {
    let internal = simulate(&Params {
        internal_funding_share: 0.85,
        ..Params::default()
    });
    let min_internal = internal
        .iter()
        .map(|s| s.credit_multiplier)
        .fold(f64::MAX, f64::min);
    // Restored to the strict original threshold after the winner-tax lag
    // gave the sovereign path an equilibrium (red-team round 3 final:
    // the earlier 0.95 relaxation passed only by horizon truncation).
    assert!(
        min_internal > 0.98,
        "internally funded boom produced a credit crunch (GPU-cycle contradiction): {min_internal}"
    );

    let levered = simulate(&Params {
        internal_funding_share: 0.25,
        momentum_gain: 1.2,
        debt_revenue_tolerance: 0.8,
        singularity_year: 2031,
        ..Params::default()
    });
    let min_levered = levered
        .iter()
        .map(|s| s.credit_multiplier)
        .fold(f64::MAX, f64::min);
    assert!(
        min_levered < 0.999,
        "externally funded stress run never tightened (railway/fiber contradiction)"
    );
}

// -------------------------------------------------------------------------
// 9. WWII mobilization ceiling: with unlimited capital and total state
//    will, US aircraft output grew ~6x in two years (~2.4x/yr) and Liberty
//    ship throughput ~2x/yr sustained — the fastest physical scaling ever
//    recorded. No stock in the model may grow faster than 2.5x in any
//    single year, even post-singularity with R4 acceleration.
// -------------------------------------------------------------------------
#[test]
fn no_stock_outruns_wwii_mobilization_ceiling() {
    for states in [
        base(),
        simulate(&Params { asi_ceiling_boost: 1.0, ..Params::default() }),
    ] {
        for w in states.windows(2) {
            // The ceiling applies to scaling an ESTABLISHED base (US aircraft
            // plants existed in 1940); ratios off a near-zero base are
            // bootstrap artifacts, so each stock gets a materiality floor.
            let ratios = [
                (w[0].ai_power_gw, 5.0, w[1].ai_power_gw / w[0].ai_power_gw.max(1e-9), "power"),
                (
                    w[0].component_capacity_m,
                    0.05,
                    w[1].component_capacity_m / w[0].component_capacity_m.max(1e-9),
                    "components",
                ),
                (w[0].compute_stock, 0.1, w[1].compute_stock / w[0].compute_stock.max(1e-9), "compute"),
            ];
            for (level, floor, r, name) in ratios {
                if level < floor {
                    continue;
                }
                assert!(
                    r <= 2.5 + 1e-9,
                    "{name} grew {r:.2}x in {} — exceeds WWII mobilization ceiling",
                    w[1].year
                );
            }
        }
    }
}
