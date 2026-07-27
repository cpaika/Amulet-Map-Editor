//! Macro-finance (B11) validation contract.
//! Spec: output/history/gap_scan.json thread #1.

use singularity_econ::{simulate, Loops, Params, YearState};

fn base() -> Vec<YearState> {
    simulate(&Params::default())
}

// The transfer ramp raises the long rate: with B11 on, the 2036 long rate
// exceeds the 4.5% anchor (bond market prices the duration supply).
#[test]
fn transfer_ramp_raises_long_rate() {
    let states = base();
    let last = states.last().unwrap();
    assert!(
        last.long_rate > 0.05,
        "long rate {} never responded to the transfer ramp",
        last.long_rate
    );
    // and it rises monotonically as debt accumulates
    for w in states.windows(2).skip(1) {
        assert!(w[1].long_rate >= w[0].long_rate - 1e-6, "rate fell in {}", w[1].year);
    }
}

// Sovereign snowball: debt/GDP climbs under the transfer ramp and debt
// service crosses the 4.5%-of-GDP fiscal-collision threshold.
#[test]
fn sovereign_debt_snowballs_and_squeezes_transfers() {
    let states = base();
    let last = states.last().unwrap();
    assert!(last.gov_debt_gdp > 1.5, "debt/GDP {} too low", last.gov_debt_gdp);
    assert!(
        last.debt_service > 0.045,
        "debt service {} never crossed the collision threshold",
        last.debt_service
    );
    // the collision throttles transfers back from their peak
    let peak_transfer = states.iter().map(|s| s.transfer_share).fold(f64::MIN, f64::max);
    assert!(
        last.transfer_share < peak_transfer,
        "transfers never retreated under debt-service squeeze: peak {peak_transfer}, last {}",
        last.transfer_share
    );
}

// New-dynamic: a job-guarantee tilt lowers the debt-financing share of transfers,
// shrinking the sovereign snowball — so both debt/GDP and the long rate end BELOW
// the pure-UBI baseline (the discount lever that lifts duration-heavy longs). jg
// share 0 is the baseline (byte-identical, enforced by the golden snapshot).
#[test]
fn job_guarantee_shrinks_the_snowball() {
    let base = base();
    let mut p = Params::default();
    p.society.jg_share = 0.6;
    let jg = simulate(&p);
    let b = base.last().unwrap();
    let j = jg.last().unwrap();
    assert!(j.gov_debt_gdp < b.gov_debt_gdp, "JG must lower debt/GDP: {} !< {}", j.gov_debt_gdp, b.gov_debt_gdp);
    assert!(j.long_rate < b.long_rate, "JG must lower the long rate: {} !< {}", j.long_rate, b.long_rate);
}

// B11 ablation: with endogenous rates off, the long rate stays at the
// exogenous anchor and never moves — the legacy flat-DCF world.
#[test]
fn b11_ablation_freezes_the_rate() {
    let off = simulate(&Params {
        loops: Loops { b11_endogenous_rates: 0.0, ..Loops::default() },
        ..Params::default()
    });
    let first = off.first().unwrap().long_rate;
    for s in &off {
        assert!(
            (s.long_rate - first).abs() < 1e-9,
            "rate moved in {} with B11 off",
            s.year
        );
    }
    let on = base();
    assert!(
        on.last().unwrap().long_rate > off.last().unwrap().long_rate + 1e-4,
        "B11 on must lift rates above the frozen anchor"
    );
}
