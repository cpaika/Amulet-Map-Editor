//! Company universe (port of the retired `companies.py` — this file is now
//! the specification). Pool weights, betas, share drifts, and terminal
//! multiples are analytic judgments; mcap/NTM are SYNCED (last refresh 2026-09-23)
//! from output/financials.json (phase-2 verified) and can be re-refreshed at
//! runtime via `load_financials`.

use crate::valuation::{Company, EmergingPool as E, RatioPool as R, Stance, DEFAULT_MAX_REV_CAGR};

pub fn universe() -> Vec<Company> {
    use Stance::*;
    let c = |ticker, name, mcap_b, ntm, pools, beta, drift, tm, stance, capture, notes| Company {
        ticker, name, mcap_b, ntm_earnings_b: ntm, pools, pool_beta: beta,
        share_drift: drift, terminal_multiple: tm, stance, capture,
        taiwan_fab_exposure: 0.0, max_rev_cagr: DEFAULT_MAX_REV_CAGR, net_debt_b: 0.0, notes,
    };
    let mut universe = vec![
        // ---- LONGS: compute complex ----
        c("TSM", "TSMC", 2040.0, 101.75, vec![(R::Silicon, 0.9), (R::GdpIndex, 0.1)],
          0.9, 0.01, 18.0, Long, vec![],
          "Dual chokepoint (N2 + CoWoS); IP-moat rent class"),
        c("NVDA", "NVIDIA", 5450.0, 291.0, vec![(R::Silicon, 1.0)],
          1.15, -0.03, 18.0, Long, vec![],
          "Share loss to ASICs inside a pool growing 2-4x consensus"),
        c("AVGO", "Broadcom", 1690.0, 82.44, vec![(R::Silicon, 0.8), (R::GdpIndex, 0.2)],
          1.05, 0.03, 19.0, Long, vec![],
          "Custom-ASIC royalty; round-2: vendor-financier drift, half-size"),
        c("MU", "Micron", 1210.0, 165.3, vec![(R::Silicon, 1.0)],
          0.6, 0.01, 10.0, Long, vec![],
          "Capacity-scarcity rent class -> harvest early (watchlist-gated)"),
        c("000660.KS", "SK Hynix", 989.35, 217.44, vec![(R::Silicon, 1.0)],
          0.6, 0.0, 8.5, Long, vec![],
          "HBM leader; same cycle caveat as MU (watchlist-gated)"),
        c("BESI.AS", "BE Semiconductor", 17.43, 0.508, vec![(R::Silicon, 1.0)],
          1.3, 0.03, 22.0, Long, vec![],
          "Hybrid bonding share gain; drift aligned with LITE (Sep-26)"),
        c("ASML", "ASML", 663.26, 21.37, vec![(R::Silicon, 1.0)],
          0.9, 0.01, 24.0, Long, vec![],
          "Litho monopoly; IP-moat rent class; scale in toward ~35x"),
        // ---- LONGS: power / grid ----
        c("GEV", "GE Vernova", 253.5, 5.584,
          vec![(R::PowerEquipment, 0.75), (R::GdpIndex, 0.25)],
          1.1, 0.01, 20.0, Long, vec![],
          "Gas turbines + grid; wind (~25%) structurally impaired (Sep-26 remap 0.75/0.25)"),
        c("VST", "Vistra", 46.31, 3.456, vec![(R::GdpIndex, 1.0)],
          0.4, 0.0, 13.0, Long, vec![(E::Electricity, 0.020)],
          "Merchant torque to power scarcity"),
        c("NRG", "NRG Energy", 21.17, 2.201, vec![(R::GdpIndex, 1.0)],
          0.4, 0.01, 12.0, Long, vec![(E::Electricity, 0.0065)],
          "13GW gas below replacement; size 50-60% of other power legs"),
        c("CEG", "Constellation", 93.49, 4.387, vec![(R::GdpIndex, 1.0)],
          0.4, 0.01, 17.0, Long, vec![(E::Electricity, 0.016)],
          "Contracted nuclear PPAs; round-2: half gas post-Calpine, half-size"),
        c("POWL", "Powell Industries", 7.02, 0.235,
          vec![(R::PowerEquipment, 0.55), (R::GdpIndex, 0.45)],
          1.3, 0.01, 15.0, Long, vec![],
          "Switchgear; ~45% oil/gas/petrochem (Sep-26 remap 0.55 PowerEq / 0.45 GDP)"),
        c("6501.T", "Hitachi", 156.92, 6.58,
          vec![(R::PowerEquipment, 0.45), (R::DcInfra, 0.15), (R::GdpIndex, 0.40)],
          1.0, 0.005, 18.0, Long, vec![],
          "Hitachi Energy transformer bottleneck inside a conglomerate"),
        // ---- LONGS: networking / optics ----
        c("LITE", "Lumentum", 84.05, 1.944, vec![(R::Silicon, 1.0)],
          1.2, 0.03, 15.0, Long, vec![],
          "EML scarcity; drift aligned with BESI's intensity-gain story (Sep-26)"),
        c("CLS", "Celestica", 41.67, 1.728, vec![(R::DcInfra, 0.8), (R::Silicon, 0.2)],
          1.0, 0.02, 14.0, Long, vec![],
          "Assembler: captures assembly value, not silicon rent (Sep-26 remap DcInfra 0.8/Si 0.2)"),
        // ---- LONGS: robotics chain (v2 flips these negative) ----
        c("MP", "MP Materials", 8.71, 0.075, vec![(R::GdpIndex, 1.0)],
          1.0, 0.10, 15.0, Long, vec![(E::RobotComponents, 0.10)],
          "Geopolitical magnet scarcity; v2 demotes to hold-only"),
        c("6268.T", "Nabtesco", 3.42, 0.164, vec![(R::GdpIndex, 1.0)],
          1.0, 0.0, 16.0, Watch, vec![],
          "RV cycloidal reducers != humanoid joints: capture removed (Sep-26 category fix); no AI thesis left -> Watch"),
        c("SHA.DE", "Schaeffler", 7.51, 0.535, vec![(R::GdpIndex, 1.0)],
          1.0, -0.01, 10.0, Long, vec![(E::RobotComponents, 0.05)],
          "KILLED (humanoid revenue immaterial on horizon)"),
        // overnight components deep-dive: cheapest reasonably-valued chokepoints
        c("002472.SZ", "Shuanghuan", 4.75, 0.229, vec![(R::GdpIndex, 1.0)],
          1.0, 0.05, 24.0, Long, vec![(E::RobotComponents, 0.06)],
          "Cheapest reducer play (~24x, profitable RV arm; new 500k/yr Suzhou plant 2026)"),
        c("300748.SZ", "JL MAG", 4.78, 0.143, vec![(R::GdpIndex, 1.0)],
          1.1, 0.04, 36.0, Long, vec![(E::RobotComponents, 0.05)],
          "China NdFeB magnet benchmark; RE-motor feedstock, substitution risk two-sided"),
        c("TECK", "Teck Resources", 32.77, 1.66,
          vec![(R::GdpIndex, 0.75), (R::PowerEquipment, 0.15), (R::DcInfra, 0.10)],
          1.2, 0.0, 14.0, Long, vec![],
          "Copper: DCs are low-single-digit % of demand, grid more (Sep-26 remap); Anglo Teck merger pending SAMR"),
        c("SYM", "Symbotic", 26.42, 0.414, vec![(R::GdpIndex, 1.0)],
          1.2, 0.10, 22.0, Watch, vec![],
          "Integrator, not a component chokepoint: demand is retailer capex (Sep-26: RobotServices capture removed); no AI thesis left -> Watch"),
        // ---- LONGS: platforms / commodities / info ----
        c("MSFT", "Microsoft", 3720.0, 146.86,
          vec![(R::SeatSaas, 0.3), (R::GdpIndex, 0.7)],
          0.9, 0.01, 22.0, Long, vec![(E::AiServices, 0.12)],
          "Aggregator; round-2: watch ex-OpenAI RPO growth, medium size"),
        c("GOOGL", "Alphabet", 4130.0, 163.63,
          vec![(R::GdpIndex, 0.55), (R::ProfInfo, 0.20), (R::DcInfra, 0.10), (R::Silicon, 0.15)],
          1.0, 0.01, 20.0, Long, vec![(E::AiServices, 0.08)],
          "Barbell: LLM-disruptable search rent (ProfInfo) + Cloud (DcInfra) + TPU (Si) (Sep-26 remap)"),
        c("FCX", "Freeport-McMoRan", 104.23, 5.07,
          vec![(R::GdpIndex, 0.75), (R::PowerEquipment, 0.15), (R::DcInfra, 0.10)],
          1.2, 0.0, 12.0, Long, vec![],
          "Copper proxy (killed at price; expression moved to deferred curve)"),
        c("EQT", "EQT", 31.94, 1.956, vec![(R::GdpIndex, 1.0)],
          0.8, 0.0, 10.0, Watch, vec![],
          "Gas producer: sells molecules, not MWh — Electricity capture removed (Sep-26 sign fix); no gas-price pool yet -> Watch"),
        c("WTKWY", "Wolters Kluwer", 17.16, 1.498, vec![(R::ProfInfo, 1.0)],
          0.9, 0.02, 12.0, Long, vec![],
          "No position: model/red-team disagreement"),
        // ---- SHORTS ----
        c("ADP", "ADP", 104.75, 4.87,
          vec![(R::HumanCognitiveWages, 0.9), (R::GdpIndex, 0.1)],
          0.6, 0.02, 16.0, Short, vec![],
          "Pays-per-control elasticity <0.5 empirically: beta 1.2->0.6 (Sep-26)"),
        c("PAYX", "Paychex", 37.2, 2.155,
          vec![(R::HumanPhysicalWages, 0.55), (R::HumanCognitiveWages, 0.35), (R::GdpIndex, 0.1)],
          1.0, 0.015, 15.0, Short, vec![],
          "SMB base is physical-labor heavy (Sep-26 remap: majority HumanPhysicalWages, beta 1.0)"),
        c("RHI", "Robert Half", 3.76, 0.163, vec![(R::HumanCognitiveWages, 1.0)],
          2.0, -0.02, 8.0, Short, vec![],
          "Live short via 4-7mo put-spread ladders (round-2 restructure)"),
        c("MAN", "ManpowerGroup", 2.67, 0.192,
          vec![(R::HumanPhysicalWages, 0.6), (R::HumanCognitiveWages, 0.4)],
          1.8, -0.02, 8.0, Short, vec![],
          "KILLED (19.9% SI squeeze trap)"),
        c("TCS.NS", "Tata Consultancy", 78.78, 5.83, vec![(R::ItServices, 1.0)],
          1.3, 0.0, 11.0, Short, vec![],
          "KILLED high conf (thesis-incoherent: they sell the integration)"),
        c("WDAY", "Workday", 46.36, 2.907, vec![(R::SeatSaas, 1.0)],
          1.2, 0.0, 13.0, Short, vec![],
          "KILLED as delta-one; trigger-gated put spreads (cRPO <10%)"),
        c("TEAM", "Atlassian", 49.38, 1.372, vec![(R::SeatSaas, 1.0)],
          1.3, -0.02, 14.0, Short, vec![],
          "KILLED high conf (capture-ratio category error)"),
        c("HUBS", "HubSpot", 11.27, 0.753, vec![(R::SeatSaas, 1.0)],
          1.3, -0.02, 13.0, Short, vec![],
          "Live short, small, kill-watch (outcome-pricing pivot)"),
        c("FDS", "FactSet", 9.94, 0.681,
          vec![(R::SeatSaas, 0.5), (R::ProfInfo, 0.5)],
          1.0, -0.01, 10.0, Short, vec![],
          "KILLED by red team"),
        c("MMC", "Marsh McLennan", 81.31, 5.239,
          vec![(R::ProfInfo, 0.25), (R::GdpIndex, 0.75)],
          1.0, 0.0, 14.0, Short, vec![],
          "Broking is premium-linked (GDP); only Mercer/OW is ProfInfo (Sep-26 remap)"),
        c("CHRW", "CH Robinson", 17.49, 0.809,
          vec![(R::HumanPhysicalWages, 0.5), (R::GdpIndex, 0.5)],
          1.0, -0.05, 12.0, Short, vec![],
          "Live short post-print ($210-225 entry)"),
        c("LSTR", "Landstar", 5.67, 0.233,
          vec![(R::HumanPhysicalWages, 0.5), (R::GdpIndex, 0.3), (R::DcInfra, 0.2)],
          1.0, -0.04, 11.0, Short, vec![],
          "Freight arm of the AI buildout: +0.2 DcInfra (Sep-26)"),
        c("MRVL", "Marvell", 228.79, 4.785, vec![(R::Silicon, 1.0)],
          1.0, -0.05, 22.0, Short, vec![],
          "ASIC share loss partly offset by electro-optics: drift -0.08->-0.05 (Sep-26)"),
        c("6954.T", "FANUC", 34.59, 1.313, vec![(R::GdpIndex, 1.0)],
          0.9, -0.02, 22.0, Short, vec![(E::Robots, 0.10)],
          "World's largest industrial-robot maker: Robots capture 0.05->0.10 (Sep-26)"),
        c("6324.T", "Harmonic Drive", 3.71, 0.047, vec![(R::GdpIndex, 1.0)],
          1.0, 0.0, 18.0, Short, vec![],
          "Harmonic reducers; RobotComponents accrues to Chinese entrants — capture removed (Sep-26)"),
        c("EQIX", "Equinix", 102.39, 4.24,
          vec![(R::DcInfra, 0.3), (R::GdpIndex, 0.7)],
          0.5, -0.01, 18.0, Short, vec![],
          "KILLED by red team"),
        c("PLTR", "Palantir", 460.88, 4.558,
          vec![(R::SeatSaas, 0.3), (R::GdpIndex, 0.7)],
          1.1, 0.0, 27.0, Short, vec![(E::AiServices, 0.015)],
          "Relative short vs MSFT/GOOGL sleeve; options only post-Aug-3"),
    ];
    // Taiwan-fab exposure: TSMC's leading-edge capacity is overwhelmingly on-island
    // (Arizona/Japan a small, growing share), so an invasion destroys most of it —
    // a company-specific hit the global Silicon pool (whose scarcity margin RISES)
    // cannot express. Fabless names take the volume hit through the pool, exposure 0.
    if let Some(tsm) = universe.iter_mut().find(|c| c.ticker == "TSM") {
        tsm.taiwan_fab_exposure = 0.78; // Sep-26: ~0.85 today, overseas fabs ramping; value at the 2028 shock
    }
    // Firm capacity (F1): single-product physical suppliers whose revenue-growth
    // ceiling is set by their own plant, not the pool. Everyone else carries the
    // default ceiling. Read only when ValuationParams::capacity_gain > 0.
    for c in universe.iter_mut() {
        if matches!(c.ticker, "POWL" | "BESI.AS" | "LITE") {
            c.max_rev_cagr = 0.20;
        }
    }
    // Net debt ($B, negative = net cash), synced from output/financials.json
    // (refresh 2026-09-23). Read only when ValuationParams::leverage_gain > 0.
    for c in universe.iter_mut() {
        c.net_debt_b = match c.ticker {
            "TSM" => -76.99,
            "NVDA" => -23.61,
            "AVGO" => 35.44,
            "MU" => -19.65,
            "000660.KS" => -48.75,
            "BESI.AS" => -0.172,
            "ASML" => -6.38,
            "GEV" => -9.0,
            "VST" => 20.07,
            "NRG" => 23.31,
            "CEG" => 24.0,
            "POWL" => -0.631,
            "6501.T" => -5.21,
            "LITE" => -1.07,
            "CLS" => 0.444,
            "MP" => -0.47,
            "6268.T" => -0.26,
            "SHA.DE" => 6.87,
            "002472.SZ" => 0.18,
            "300748.SZ" => -0.34,
            "TECK" => 2.65,
            "SYM" => -1.72,
            "MSFT" => 51.97,
            "GOOGL" => -121.68,
            "FCX" => 6.28,
            "EQT" => 5.54,
            "WTKWY" => 4.62,
            "ADP" => 1.29,
            "PAYX" => 3.69,
            "RHI" => -0.084,
            "MAN" => 1.24,
            "TCS.NS" => -3.51,
            "WDAY" => 0.369,
            "TEAM" => -0.007,
            "HUBS" => -1.1,
            "FDS" => 1.27,
            "MMC" => 20.68,
            "CHRW" => 1.82,
            "LSTR" => -0.209,
            "MRVL" => 1.35,
            "6954.T" => -4.83,
            "6324.T" => -0.03,
            "EQIX" => 22.38,
            "PLTR" => -9.2,
            _ => 0.0,
        };
    }
    universe
}

/// Override mcap / NTM / terminal multiple from the phase-2 verified
/// financials JSON ({ticker: {mcap_b, ntm_earnings_b,
/// suggested_terminal_multiple, ...}}).
#[cfg(feature = "serde")]
pub fn load_financials(companies: &mut [Company], json: &str) {
    let parsed: serde_json::Value = serde_json::from_str(json)
        .unwrap_or_else(|e| panic!("malformed financials JSON: {e}"));
    for c in companies.iter_mut() {
        if let Some(d) = parsed.get(c.ticker) {
            if let Some(m) = d.get("mcap_b").and_then(|v| v.as_f64()) {
                c.mcap_b = m;
            }
            if let Some(e) = d.get("ntm_earnings_b").and_then(|v| v.as_f64()) {
                if e > 0.0 {
                    c.ntm_earnings_b = e;
                }
            }
            if let Some(nd) = d.get("net_debt_b").and_then(|v| v.as_f64()) {
                c.net_debt_b = nd;
            }
            if let Some(t) = d.get("suggested_terminal_multiple")
                .and_then(|v| v.as_f64())
            {
                if (4.0..=30.0).contains(&t) {
                    c.terminal_multiple = t;
                }
            }
        }
    }
}
