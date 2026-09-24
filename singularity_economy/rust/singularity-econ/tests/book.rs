//! Valuation-engine tests and trade-book conclusion locks — port of the
//! retired `test_valuation.py` and `test_scenarios_v2.py`. If a lock fails
//! after a model change, the book's foundations moved: that is a review
//! event, not a test to silence.

use singularity_econ::companies::universe;
use singularity_econ::scenarios::{
    scenario_params, scenario_states, scenario_states_enhanced, scenario_states_v2,
};
use singularity_econ::valuation::{
    earnings_and_rent, earnings_path, evaluate, evaluate_all, implied_cagr, pv, Company,
    EmergingPool, RatioPool, Stance, CAPTURE_DECAY, DISCOUNT_RATE, HORIZON, SCENARIO_PROBS,
};
use singularity_econ::{simulate, Params};

/// Equity-duration beta threaded into the valuation (audit A3). Matches the
/// `MacroParams::default().dr_beta`, so these locks price on the same discount
/// the shipped `main` uses.
const DR_BETA: f64 = 0.60;

fn test_company(pools: Vec<(RatioPool, f64)>, beta: f64, drift: f64) -> Company {
    Company {
        ticker: "TEST",
        name: "test",
        mcap_b: 100.0,
        ntm_earnings_b: 5.0,
        pools,
        pool_beta: beta,
        share_drift: drift,
        terminal_multiple: 15.0,
        stance: Stance::Watch,
        capture: vec![],
        taiwan_fab_exposure: 0.0,
        max_rev_cagr: f64::INFINITY,
        net_debt_b: 0.0,
        china_revenue_share: 0.0,
        foreign_access_risk: 0.0,
        notes: "",
    }
}

// ---------------- pv / reverse DCF ----------------

#[test]
fn pv_orders_growth_and_terminal() {
    let flat = vec![10.0; HORIZON];
    let growing: Vec<f64> = (0..HORIZON).map(|i| 10.0 * 1.2_f64.powi(i as i32)).collect();
    assert!(pv(&growing, 15.0, DISCOUNT_RATE) > pv(&flat, 15.0, DISCOUNT_RATE));
    assert!(pv(&flat, 20.0, DISCOUNT_RATE) > pv(&flat, 10.0, DISCOUNT_RATE));
}

#[test]
fn implied_cagr_roundtrip() {
    let g = 0.12_f64;
    let e0 = 10.0;
    let path: Vec<f64> = (0..HORIZON).map(|i| e0 * (1.0 + g).powi(i as i32 + 1)).collect();
    let mcap = pv(&path, 15.0, DISCOUNT_RATE);
    assert!((implied_cagr(mcap, e0, 15.0) - g).abs() < 0.005);
}

#[test]
fn higher_price_implies_higher_growth() {
    assert!(implied_cagr(500.0, 10.0, 15.0) > implied_cagr(200.0, 10.0, 15.0));
}

// ---------------- earnings paths ----------------

#[test]
fn pool_growth_flows_through() {
    let states = simulate(&Params::default());
    let c = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.0);
    let path = earnings_path(&c, &states);
    assert!(path.last().unwrap() > &path[0]);
}

#[test]
fn casualty_pool_shrinks_earnings() {
    let states = simulate(&Params::default());
    let c = test_company(vec![(RatioPool::Bpo, 1.0)], 1.0, 0.0);
    let path = earnings_path(&c, &states);
    assert!(path.last().unwrap() < &path[0]);
}

#[test]
fn beta_amplifies_and_drift_compounds() {
    let states = simulate(&Params::default());
    let lo = test_company(vec![(RatioPool::Silicon, 1.0)], 0.5, 0.0);
    let hi = test_company(vec![(RatioPool::Silicon, 1.0)], 1.5, 0.0);
    assert!(earnings_path(&hi, &states).last().unwrap()
            > earnings_path(&lo, &states).last().unwrap());
    let gain = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.05);
    let lose = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, -0.05);
    assert!(earnings_path(&gain, &states).last().unwrap()
            > earnings_path(&lose, &states).last().unwrap());
}

#[test]
fn capture_adds_emerging_pool_earnings() {
    let states = simulate(&Params::default());
    let mut with = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    with.capture = vec![(EmergingPool::AiServices, 0.05)];
    let without = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    assert!(earnings_path(&with, &states).last().unwrap()
            > earnings_path(&without, &states).last().unwrap());
}

// Re-audit C2: the terminal value must apply a RENT-DOMINANCE haircut. A capture
// name's near-peak final-year rent is NOT a perpetuity (the scarcity share erodes),
// so its terminal slice is capitalized at multiple x dr/(dr+decay), strictly below
// the full multiple — while a name with NO capture is untouched (rent path all zero).
#[test]
fn terminal_rent_haircut_derates_only_capture_names() {
    let states = simulate(&Params::default());
    let dr = DISCOUNT_RATE;
    // Non-capture name: rent path is identically zero, so the mechanism is a no-op.
    let no_cap = test_company(vec![(RatioPool::Silicon, 1.0)], 1.0, 0.0);
    let (_pn, rent_n) = earnings_and_rent(&no_cap, &states);
    assert!(rent_n.iter().all(|r| *r == 0.0), "no-capture name must carry zero rent");
    // Capture name: positive terminal rent, so the haircut PV must sit strictly below
    // the full-multiple capitalization of the identical earnings path.
    let mut cap = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    cap.capture = vec![(EmergingPool::Electricity, 0.02)];
    let (path, rent) = earnings_and_rent(&cap, &states);
    let last_rent = *rent.last().unwrap();
    assert!(last_rent > 0.0, "capture name must carry terminal rent");
    let full = pv(&path, cap.terminal_multiple, dr);
    let flow: f64 = path.iter().enumerate()
        .map(|(i, e)| e / (1.0 + dr).powi(i as i32 + 1)).sum();
    let base_last = path.last().unwrap() - last_rent;
    let rent_mult = cap.terminal_multiple * dr / (dr + CAPTURE_DECAY);
    let haircut = flow
        + (base_last * cap.terminal_multiple + last_rent * rent_mult)
            / (1.0 + dr).powi(path.len() as i32);
    assert!(rent_mult < cap.terminal_multiple, "rent multiple must be below the full multiple");
    assert!(haircut < full, "rent haircut must reduce a capture name's PV: {haircut} vs {full}");
}

// ---------------- scenario evaluation ----------------

#[test]
fn probs_sum_to_one_and_scenarios_present() {
    let total: f64 = SCENARIO_PROBS.iter().map(|(_, p)| p).sum();
    assert!((total - 1.0).abs() < 1e-9);
    let names: Vec<&str> = scenario_params().iter().map(|(n, _)| *n).collect();
    for (n, _) in SCENARIO_PROBS {
        assert!(names.contains(&n), "missing scenario {n}");
    }
}

#[test]
fn fizzle_is_worst_for_beneficiaries() {
    let states = scenario_states();
    let c = test_company(vec![(RatioPool::Silicon, 1.0)], 1.1, 0.0);
    let e = evaluate(&c, &states, DR_BETA);
    let worst = e.per_scenario.iter()
        .min_by(|a, b| a.upside.partial_cmp(&b.upside).unwrap())
        .unwrap();
    assert_eq!(worst.scenario, "fizzle");
}

#[test]
fn evaluate_all_ranks_by_expected_upside() {
    let states = scenario_states();
    let rows = evaluate_all(&universe(), &states, DR_BETA);
    for w in rows.windows(2) {
        assert!(w[0].expected_upside >= w[1].expected_upside);
    }
}

// ---------------- trade-book conclusion locks ----------------

fn book() -> Vec<(String, f64)> {
    let states = scenario_states();
    evaluate_all(&universe(), &states, DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect()
}

fn upside(rows: &[(String, f64)], ticker: &str) -> f64 {
    rows.iter().find(|(t, _)| t == ticker)
        .unwrap_or_else(|| panic!("{ticker} missing"))
        .1
}

#[test]
fn core_semi_longs_positive() {
    let rows = book();
    for t in ["TSM", "AVGO", "NVDA"] {
        assert!(upside(&rows, t) > 0.5, "{t}: {}", upside(&rows, t));
    }
}

#[test]
fn power_complex_positive() {
    // Thresholds recalibrated after re-audit #26/#28: the capture-rent terminal is now
    // capped at its self-consistent decaying-perpetuity value (1/(dr+decay)) and the
    // terminal discounts at the terminal-year rate, so power GENERATION (whose upside
    // was mostly capitalized rent) prices conservative — positive, no longer >50%.
    // Grid EQUIPMENT (GEV), which earns flows not rent, stays a strong long.
    let rows = book();
    assert!(upside(&rows, "GEV") > 1.0, "GEV: {}", upside(&rows, "GEV"));
    assert!(upside(&rows, "VST") > 0.4, "VST: {}", upside(&rows, "VST"));
    assert!(upside(&rows, "NRG") > 0.2, "NRG: {}", upside(&rows, "NRG"));
    assert!(upside(&rows, "CEG") > 0.0, "CEG: {}", upside(&rows, "CEG"));
}

#[test]
fn wage_linked_shorts_negative() {
    let rows = book();
    for t in ["RHI", "ADP", "PAYX", "MAN", "CHRW", "LSTR", "TCS.NS"] {
        assert!(upside(&rows, t) < -0.3, "{t}: {}", upside(&rows, t));
    }
}

#[test]
fn robotics_longs_split_component_vs_integrator() {
    // The robot-scaling crunch favors the COMPONENT bottleneck over the integrator
    // (SYM) and upstream materials (MP). Sep-26 remap: the lock previously used
    // Nabtesco (6268.T), but its RobotComponents capture was the category error the
    // Aug deep dive proved (RV cycloidal reducers are not humanoid joints) and has
    // been removed. The book's actual humanoid-component chokepoint is Shuanghuan
    // (002472.SZ: RV + harmonic + roller screws, humanoid design wins). The durable
    // finding — component beats integrator/materials — is tested on the right name.
    let rows = book();
    let comp = upside(&rows, "002472.SZ");
    assert!(comp > 0.0, "component supplier should stay positive: {comp}");
    for t in ["SYM", "MP"] {
        let u = upside(&rows, t);
        assert!(u < comp, "{t} ({u}) must trail the component supplier ({comp})");
    }
}

// Enhanced-realism enable set (gated_dynamics_menu.md): turning the reflexive
// AI-capex spine + wealth effect + transmission lag + wage compression + JG tilt on
// TOGETHER must re-rank the book, not just perturb it. The load-bearing effect is the
// Minsky bust: the silicon/AI-capex complex (NVDA, AVGO, TSM, BESI) must lose upside
// vs the shipped baseline (fatter left tail), while the wage-linked shorts stay short.
// This is the review lens, not the shipped default — it must run without pathology.
#[test]
fn enhanced_realism_reranks_the_book() {
    let base = book();
    let enh: Vec<(String, f64)> = evaluate_all(&universe(), &scenario_states_enhanced(), DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect();
    // All finite, and the ranking actually MOVED (not a no-op enable set).
    assert!(enh.iter().all(|(_, u)| u.is_finite()), "enhanced book must stay finite");
    let base_top = &base[0].0;
    let enh_top = &enh[0].0;
    let moved = enh.iter().any(|(t, u)| (u - upside(&base, t)).abs() > 0.05);
    assert!(moved, "enhanced enable set must re-rank the book, base_top={base_top} enh_top={enh_top}");
    // The reflexive-bust left tail: the AI-capex complex must lose upside vs baseline.
    for t in ["NVDA", "AVGO", "TSM", "BESI.AS"] {
        assert!(
            upside(&enh, t) < upside(&base, t) - 0.10,
            "{t} must lose upside under the reflexive bust: enh {} vs base {}",
            upside(&enh, t),
            upside(&base, t)
        );
    }
    // The wage-linked shorts remain short (wage compression can only deepen them).
    for t in ["RHI", "ADP", "PAYX"] {
        assert!(upside(&enh, t) < -0.3, "{t} must stay short under enhanced realism: {}", upside(&enh, t));
    }
}

// First-principles v2: the most complete first-principles configuration (enhanced set +
// Wright learning + q-governor + two-sided power + AI commoditization). It must reprice
// the book coherently: (a) all finite; (b) the AI-services RENT names (MSFT, GOOGL) lose
// upside vs baseline as commoditization competes the provider surplus away; (c) the
// physical-bottleneck longs (POWL grid gear, robot components) still lead positive;
// (d) the wage-linked shorts stay short. This locks the composed lens against drift.
#[test]
fn first_principles_v2_reprices_coherently() {
    let base = book();
    let v2: Vec<(String, f64)> = evaluate_all(&universe(), &scenario_states_v2(), DR_BETA)
        .into_iter()
        .map(|e| (e.ticker.to_string(), e.expected_upside))
        .collect();
    assert!(v2.iter().all(|(_, u)| u.is_finite()), "v2 book must stay finite");
    // AI-services rent commoditizes → MSFT/GOOGL lose upside vs baseline.
    for t in ["MSFT", "GOOGL"] {
        assert!(
            upside(&v2, t) < upside(&base, t) - 0.10,
            "{t} AI rent must commoditize in v2: {} vs {}",
            upside(&v2, t),
            upside(&base, t)
        );
    }
    // The physical bottleneck still leads. With the re-audited q-governor (operating-
    // return numerator, which brakes the compute buildout harder and earlier), the
    // robot-component chokepoint — less capex-reflexive than compute-derived grid demand
    // — leads, with grid switchgear (POWL) the strongest grid name and GEV a clear long.
    // Sep-26 recalibration: POWL (0.55 PowerEquipment / 0.45 GdpIndex) and GEV
    // (0.75 / 0.25) are remapped to their real business mix, so under v2 discipline
    // the grid names are positive but no longer strong; the component chokepoint is
    // the one v2 long that stays strong.
    // Wave 4: v2 now carries the fiscal-dominance rates regime (via enhanced realism),
    // which raises every discount rate; the chokepoint drops 0.85 -> 0.42 but stays a
    // clear long (book-mc shows it is the most lens-robust name).
    assert!(upside(&v2, "002472.SZ") > 0.25, "component chokepoint must stay a clear long in v2: {}", upside(&v2, "002472.SZ"));
    // Wave 2 (F2): v2 also decays the capex-desire growth base toward GDP, which takes
    // GEV to roughly fair value (-1%); POWL stays the positive grid name. GEV is locked
    // as "not a short", not as a long.
    assert!(upside(&v2, "POWL") > 0.1, "POWL must stay a long in v2: {}", upside(&v2, "POWL"));
    assert!(upside(&v2, "GEV") > -0.10, "GEV must stay near fair in v2: {}", upside(&v2, "GEV"));
    // Wage-linked shorts remain short.
    for t in ["RHI", "ADP", "PAYX"] {
        assert!(upside(&v2, t) < -0.3, "{t} must stay short in v2: {}", upside(&v2, t));
    }
}

// Red-team round 3: geopolitics must be REACHABLE from the book — the
// taiwan_shock scenario has to price real damage into Taiwan-exposed
// names (TSM's shock-scenario fair value below its baseline fair value),
// and it must carry nonzero probability weight.
#[test]
fn taiwan_shock_is_priced() {
    use singularity_econ::valuation::SCENARIO_PROBS;
    let w = SCENARIO_PROBS
        .iter()
        .find(|(n, _)| *n == "taiwan_shock")
        .expect("taiwan_shock missing from scenario probabilities")
        .1;
    assert!(w >= 0.05, "taiwan tail weight collapsed: {w}");
    let states = singularity_econ::scenarios::scenario_states();
    let companies = singularity_econ::companies::universe();
    let tsm = companies.iter().find(|c| c.ticker == "TSM").unwrap();
    let ev = singularity_econ::valuation::evaluate(tsm, &states, DR_BETA);
    let base = ev.per_scenario.iter().find(|s| s.scenario == "baseline").unwrap();
    let shock = ev
        .per_scenario
        .iter()
        .find(|s| s.scenario == "taiwan_shock")
        .unwrap();
    // Taiwan shock must price REAL TSM damage. Threshold relaxed from 0.10 to
    // 0.03 after the batch-1 chip-cap-latch fix: post-invasion fabs now recover
    // as ASI rebuilds them (the latch was permanently pinning growth at 18%), so
    // an invasion is materially less PERMANENTLY damaging — the shock is still
    // priced (~5% haircut), just no longer catastrophic-forever.
    assert!(
        shock.upside < base.upside - 0.03,
        "TSM shows no Taiwan damage: shock {} vs base {}",
        shock.upside,
        base.upside
    );
}

// F2 lock (Sep-26 re-analysis, Wave 2): with no singularity nothing sustains the
// 32%/yr capex-desire engine. The named fizzle must bind on chips in the observed 2026,
// then on DEMAND; it must not hold power scarcity (legacy: power-bound 2028-36 on
// flat AI revenue) or compound capex like the singularity world, and the longs' worst
// case must be a loss.
#[test]
fn fizzle_is_demand_bound_not_power_bound() {
    use singularity_econ::Binding;
    let fz = scenario_states().into_iter().find(|(n, _)| *n == "fizzle").unwrap().1;
    assert_eq!(fz[0].binding, Binding::Chips, "2026 must stay chips-bound");
    let power_years = fz.iter().filter(|s| s.binding == Binding::Power).count();
    assert!(power_years <= 3, "fizzle power-bound {power_years} years");
    let base = simulate(&Params::default());
    let (fz36, b36) = (fz.last().unwrap().ai_capex, base.last().unwrap().ai_capex);
    assert!(fz36 < 0.5 * b36, "fizzle 2036 capex {fz36:.2} vs baseline {b36:.2}");
    let rows = evaluate_all(&universe(), &scenario_states(), DR_BETA);
    for t in ["POWL", "GEV", "TECK", "NVDA"] {
        let r = rows.iter().find(|r| r.ticker == t).unwrap();
        assert!(r.worst_scenario_upside < 0.0, "{t} worst case {} must be a loss", r.worst_scenario_upside);
    }
}

// The dgb decay converges demand growth toward trend GDP: a strong decay in the
// no-singularity world lowers late capex monotonically, and rate 0 is the legacy path.
#[test]
fn dgb_decay_lowers_no_singularity_capex_monotonically() {
    let mut last = f64::MAX;
    for rate in [0.0, 0.1, 0.2, 0.4] {
        let p = Params {
            singularity_year: 2099,
            robotics_year: 2099,
            dgb_decay_rate: rate,
            ..Params::default()
        };
        let c = simulate(&p).last().unwrap().ai_capex;
        assert!(c <= last + 1e-9, "rate {rate}: capex {c} rose vs {last}");
        last = c;
    }
}

// F5 lock (Sep-26 re-analysis, Wave 2): the v2 q-governor reads FORWARD q. The legacy
// trailing q cut 2027 capex to 0.55 from 0.66 against rising 2027 guidance.
#[test]
fn v2_forward_q_does_not_bust_2027() {
    let s = simulate(&singularity_econ::scenarios::first_principles_v2(Params::default()));
    assert!(s[1].ai_capex > s[0].ai_capex, "v2 2027 capex {} vs 2026 {}", s[1].ai_capex, s[0].ai_capex);
}

// The v2 silicon verdict IS the growth seed (F5): the forward-q system has a regime
// switch near seed ~1.35. Below it the baseline busts (capex peaks ~0.9 then decays,
// silicon margin at its floor); above it the boom runs. The value surface is NOT
// monotone (a bigger early boom brings the glut forward: fast_takeoff NVDA falls
// 197 -> 139 from seed 1.0 to 1.2), refuting the plan's monotone lock. Lock the
// switch, so the book reports v2 silicon as a band (book-mc samples the seed).
#[test]
fn v2_silicon_verdict_switches_regime_on_growth_seed() {
    let nvda_at = |seed: f64| {
        let states: Vec<_> = scenario_params()
            .into_iter()
            .map(|(n, p)| {
                let mut p = singularity_econ::scenarios::first_principles_v2(p);
                p.ai_rev_growth_2026 = seed;
                (n, simulate(&p))
            })
            .collect();
        evaluate_all(&universe(), &states, DR_BETA)
            .into_iter()
            .find(|r| r.ticker == "NVDA")
            .unwrap()
            .expected_upside
    };
    // Wave 4: fab discipline removed the phantom-glut bust (legacy -88%), so the bust
    // regime is milder; the lock is the regime GAP, not a bust level.
    let (bust, boom) = (nvda_at(0.8), nvda_at(1.5));
    // With cash-flow funding + the rates regime in v2, the boom is funding-capped
    // (~$4.6T capex in 2036) and the switch sits near seed ~1.1: bust ~-69%, boom ~+17%.
    assert!(bust < -0.3, "low-seed v2 NVDA {bust} should be the bust regime");
    assert!(boom > 0.0, "high-seed v2 NVDA {boom} should be the boom regime");
    assert!(boom - bust > 0.6, "regime gap {} too small", boom - bust);
}

// F1 lock (Sep-26 re-analysis, Wave 2): the first-principles valuation conventions
// (capacity cap + steady-state flow terminal + power routing, shipped together).
// (a) they only remove build-rate froth — no name gains from the flow terminal +
// cap; routing alone may lift grid names, which is why it never ships alone;
// (b) the grid names' terminal share falls (legacy struck the terminal on a pool
// still growing ~48%/yr in 2036); (c) names with no capex-pool exposure are
// untouched; (d) the book stays finite.
#[test]
fn first_principles_valuation_removes_build_rate_froth() {
    use singularity_econ::valuation::{evaluate_all_with, ValuationParams};
    let st = scenario_states();
    let legacy = evaluate_all(&universe(), &st, DR_BETA);
    // F1 pieces only (leverage and policy have their own locks).
    let f1 = ValuationParams { leverage_gain: 0.0, policy_gain: 0.0, ..ValuationParams::first_principles() };
    let no_route = ValuationParams { power_route_gain: 0.0, ..f1 };
    let fp_nr = evaluate_all_with(&universe(), &st, DR_BETA, &no_route);
    let fp = evaluate_all_with(&universe(), &st, DR_BETA, &f1);
    let get = |rows: &[singularity_econ::valuation::Evaluation], t: &str| {
        rows.iter().find(|r| r.ticker == t).unwrap().clone()
    };
    for r in &legacy {
        let n = get(&fp_nr, r.ticker);
        assert!(n.expected_upside <= r.expected_upside + 1e-9,
                "{}: cap+flow raised E[up] {} -> {}", r.ticker, r.expected_upside, n.expected_upside);
        assert!(get(&fp, r.ticker).expected_upside.is_finite());
    }
    for t in ["POWL", "GEV"] {
        let (l, f) = (get(&legacy, t), get(&fp, t));
        assert!(f.terminal_share < l.terminal_share - 0.1,
                "{t}: terminal share {} vs legacy {}", f.terminal_share, l.terminal_share);
        assert!(f.expected_upside < 0.5 * l.expected_upside, "{t}: {} vs {}", f.expected_upside, l.expected_upside);
    }
    for t in ["RHI", "PAYX", "MSFT", "002472.SZ", "VST"] {
        let (l, f) = (get(&legacy, t), get(&fp, t));
        assert!((f.expected_upside - l.expected_upside).abs() < 1e-9, "{t} moved without capex-pool exposure");
    }
}

// F6 attribution lock (Sep-26 re-analysis, Wave 4): E[up] splits EXACTLY into the
// valuation convention (the name with every pool on the GDP index), the hand-set
// share drift, and the AI thesis. Finding lock: the staffing/payroll/offshore-IT
// shorts are AI-thesis shorts; the MRVL and PLTR shorts are pure convention — the
// modeled AI economy RAISES them — so they must not be read as AI calls.
#[test]
fn short_book_attribution() {
    let rows = evaluate_all(&universe(), &scenario_states(), DR_BETA);
    for r in &rows {
        let sum = r.conv_upside + r.drift_delta + r.ai_delta;
        assert!((sum - r.expected_upside).abs() < 1e-9, "{}: parts {sum} vs {}", r.ticker, r.expected_upside);
    }
    let ai = |t: &str| rows.iter().find(|r| r.ticker == t).unwrap().ai_delta;
    for t in ["RHI", "MAN", "TCS.NS", "PAYX", "ADP"] {
        assert!(ai(t) < -0.20, "{t}: AI delta {} — no longer an AI-thesis short", ai(t));
    }
    for t in ["MRVL", "PLTR"] {
        assert!(ai(t) > 0.0, "{t}: AI delta {} — short is convention, AI should lift it", ai(t));
    }
    // A name with no AI mapping carries no AI delta at all.
    let eqt = rows.iter().find(|r| r.ticker == "EQT").unwrap();
    assert!(eqt.is_valuation_only() && eqt.ai_delta.abs() < 1e-9);
}

// Capital structure (capture shortlist #1): with the leverage gate on, a rate shock
// hits a levered equity harder than the same business unlevered (refinancing at
// higher rates eats the equity slice), and a net-cash name is cushioned. With flat
// rates and a flat business the levered path reproduces NTM earnings exactly.
#[test]
fn leverage_gives_equity_rate_torque() {
    use singularity_econ::valuation::{value_on_path, PathContext, ValuationParams};
    let base: Vec<_> = simulate(&Params::default());
    let mut shocked = base.clone();
    for s in shocked.iter_mut().skip(1) {
        s.long_rate += 0.03;
    }
    let vp = ValuationParams { leverage_gain: 1.0, ..ValuationParams::default() };
    let mut levered = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    levered.net_debt_b = 100.0; // net debt = market cap
    let mut cash = levered.clone();
    cash.net_debt_b = -50.0;
    let unlevered = test_company(vec![(RatioPool::GdpIndex, 1.0)], 1.0, 0.0);
    let hit = |c: &Company| {
        let v0 = value_on_path(c, &base, PathContext::default(), DR_BETA, &vp).0;
        let v1 = value_on_path(c, &shocked, PathContext::default(), DR_BETA, &vp).0;
        v1 / v0 - 1.0
    };
    let (hl, hu, hc) = (hit(&levered), hit(&unlevered), hit(&cash));
    assert!(hl < hu - 0.05, "levered {hl} vs unlevered {hu}");
    assert!(hc > hu, "net cash {hc} vs unlevered {hu}");
    // Identity: flat business + flat rates -> NTM earnings every year.
    let mut flat = base.clone();
    for s in flat.iter_mut() {
        s.pools.gdp_index = base[0].pools.gdp_index;
        s.long_rate = base[0].long_rate;
    }
    let path = singularity_econ::valuation::earnings_with(&levered, &flat, &vp).path;
    assert!(path.iter().all(|e| (e - levered.ntm_earnings_b).abs() < 1e-9), "{path:?}");
}

// Policy risk (Sep-24): four gated channels — export controls, foreign-investor
// access bans, power windfall levy, AI windfall tax. Locks: no channel ever raises a
// name; names with no policy exposure are untouched; a realized access ban on a sampled
// path costs exactly the forced-sale haircut; China A-shares carry access risk.
#[test]
fn policy_risk_layer() {
    use singularity_econ::valuation::{evaluate_all_with, value_on_path, PathContext, PolicyDraw, ValuationParams};
    let st = scenario_states();
    let fp = ValuationParams::first_principles();
    let off = ValuationParams { policy_gain: 0.0, ..fp };
    let (a, b) = (evaluate_all_with(&universe(), &st, DR_BETA, &off),
                  evaluate_all_with(&universe(), &st, DR_BETA, &fp));
    for r in &a {
        let n = b.iter().find(|x| x.ticker == r.ticker).unwrap();
        assert!(n.expected_upside <= r.expected_upside + 1e-9, "{}: policy raised value", r.ticker);
    }
    for t in ["RHI", "ADP", "GEV", "WTKWY"] {
        let (x, y) = (a.iter().find(|r| r.ticker == t).unwrap(), b.iter().find(|r| r.ticker == t).unwrap());
        assert!((x.expected_upside - y.expected_upside).abs() < 1e-9, "{t} has no policy exposure");
    }
    let comps = universe();
    let shx = comps.iter().find(|c| c.ticker == "002472.SZ").unwrap();
    assert_eq!(shx.foreign_access_risk, 1.0);
    let states = simulate(&Params::default());
    let quiet = PathContext { policy: Some(PolicyDraw::default()), ..PathContext::default() };
    let banned = PathContext {
        policy: Some(PolicyDraw { access_ban_year: Some(2030), ..PolicyDraw::default() }),
        ..PathContext::default()
    };
    let v0 = value_on_path(shx, &states, quiet, DR_BETA, &fp).0;
    let v1 = value_on_path(shx, &states, banned, DR_BETA, &fp).0;
    assert!((v1 / v0 - (1.0 - fp.access_haircut)).abs() < 1e-12, "{v1} vs {v0}");
}
