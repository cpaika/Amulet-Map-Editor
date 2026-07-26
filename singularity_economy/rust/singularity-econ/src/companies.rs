//! Company universe (port of the retired `companies.py` — this file is now
//! the specification). Pool weights, betas, share drifts, and terminal
//! multiples are analytic judgments; mcap/NTM/terminal multiples are SYNCED
//! from output/financials.json (phase-2 verified) and can be re-refreshed at
//! runtime via `load_financials`.

use crate::valuation::{Company, EmergingPool as E, RatioPool as R, Stance};

pub fn universe() -> Vec<Company> {
    use Stance::*;
    let c = |ticker, name, mcap_b, ntm, pools, beta, drift, tm, stance, capture, notes| Company {
        ticker, name, mcap_b, ntm_earnings_b: ntm, pools, pool_beta: beta,
        share_drift: drift, terminal_multiple: tm, stance, capture, notes,
    };
    vec![
        // ---- LONGS: compute complex ----
        c("TSM", "TSMC", 2184.0, 112.0, vec![(R::Silicon, 0.9), (R::GdpIndex, 0.1)],
          0.9, 0.01, 18.0, Long, vec![],
          "Dual chokepoint (N2 + CoWoS); IP-moat rent class"),
        c("NVDA", "NVIDIA", 5060.0, 241.0, vec![(R::Silicon, 1.0)],
          1.15, -0.03, 18.0, Long, vec![],
          "Share loss to ASICs inside a pool growing 2-4x consensus"),
        c("AVGO", "Broadcom", 1830.0, 75.0, vec![(R::Silicon, 0.8), (R::GdpIndex, 0.2)],
          1.05, 0.03, 19.0, Long, vec![],
          "Custom-ASIC royalty; round-2: vendor-financier drift, half-size"),
        c("MU", "Micron", 1120.0, 162.0, vec![(R::Silicon, 1.0)],
          0.6, 0.01, 10.0, Long, vec![],
          "Capacity-scarcity rent class -> harvest early (watchlist-gated)"),
        c("000660.KS", "SK Hynix", 923.0, 140.0, vec![(R::Silicon, 1.0)],
          0.6, 0.0, 8.5, Long, vec![],
          "HBM leader; same cycle caveat as MU (watchlist-gated)"),
        c("BESI.AS", "BE Semiconductor", 20.7, 0.51, vec![(R::Silicon, 1.0)],
          1.3, 0.04, 22.0, Long, vec![],
          "Hybrid bonding share gain (killed at entry by red team; watchlist)"),
        c("ASML", "ASML", 692.5, 14.0, vec![(R::Silicon, 1.0)],
          0.9, 0.01, 24.0, Long, vec![],
          "Litho monopoly; IP-moat rent class; scale in toward ~35x"),
        // ---- LONGS: power / grid ----
        c("GEV", "GE Vernova", 274.6, 5.67, vec![(R::PowerEquipment, 1.0)],
          1.1, 0.01, 20.0, Long, vec![],
          "Sold out through 2030 (killed at price by red team; watchlist $700-800)"),
        c("VST", "Vistra", 57.0, 3.05, vec![(R::GdpIndex, 1.0)],
          0.4, 0.0, 13.0, Long, vec![(E::Electricity, 0.05, 0.5)],
          "Merchant torque to power scarcity"),
        c("NRG", "NRG Energy", 30.2, 2.18, vec![(R::GdpIndex, 1.0)],
          0.4, 0.01, 12.0, Long, vec![(E::Electricity, 0.045, 0.45)],
          "13GW gas below replacement; size 50-60% of other power legs"),
        c("CEG", "Constellation", 99.3, 4.5, vec![(R::GdpIndex, 1.0)],
          0.4, 0.01, 17.0, Long, vec![(E::Electricity, 0.06, 0.5)],
          "Contracted nuclear PPAs; round-2: half gas post-Calpine, half-size"),
        c("POWL", "Powell Industries", 8.83, 0.21, vec![(R::PowerEquipment, 1.0)],
          1.3, 0.01, 15.0, Long, vec![],
          "Switchgear sold out (killed at price by red team; watchlist 22-25x)"),
        c("6501.T", "Hitachi", 147.0, 6.15,
          vec![(R::PowerEquipment, 0.45), (R::DcInfra, 0.15), (R::GdpIndex, 0.40)],
          1.0, 0.005, 18.0, Long, vec![],
          "Hitachi Energy transformer bottleneck inside a conglomerate"),
        // ---- LONGS: networking / optics ----
        c("LITE", "Lumentum", 64.9, 1.24, vec![(R::Silicon, 1.0)],
          1.2, 0.02, 15.0, Long, vec![],
          "EML scarcity (killed at price by red team; watchlist 12-14x)"),
        c("CLS", "Celestica", 38.4, 1.3, vec![(R::DcInfra, 0.6), (R::Silicon, 0.4)],
          1.1, 0.02, 14.0, Long, vec![],
          "Cheapest AI-networking growth; starter, add per rules"),
        // ---- LONGS: robotics chain (v2 flips these negative) ----
        c("MP", "MP Materials", 7.95, 0.05, vec![(R::GdpIndex, 1.0)],
          1.0, 0.10, 15.0, Long, vec![(E::RobotComponents, 0.10, 0.25)],
          "Geopolitical magnet scarcity; v2 demotes to hold-only"),
        c("6268.T", "Nabtesco", 4.0, 0.155, vec![(R::GdpIndex, 1.0)],
          1.0, 0.0, 16.0, Long, vec![(E::RobotComponents, 0.08, 0.22)],
          "KILLED (product-category error); kept for cross-checks"),
        c("SHA.DE", "Schaeffler", 8.68, 0.5, vec![(R::GdpIndex, 1.0)],
          1.0, -0.01, 10.0, Long, vec![(E::RobotComponents, 0.05, 0.15)],
          "KILLED (humanoid revenue immaterial on horizon)"),
        // overnight components deep-dive: cheapest reasonably-valued chokepoints
        c("002472.SZ", "Shuanghuan", 4.1, 0.18, vec![(R::GdpIndex, 1.0)],
          1.0, 0.05, 24.0, Long, vec![(E::RobotComponents, 0.06, 0.22)],
          "Cheapest reducer play (~24x, profitable RV arm; new 500k/yr Suzhou plant 2026)"),
        c("300748.SZ", "JL MAG", 4.2, 0.11, vec![(R::GdpIndex, 1.0)],
          1.1, 0.04, 36.0, Long, vec![(E::RobotComponents, 0.05, 0.20)],
          "China NdFeB magnet benchmark; RE-motor feedstock, substitution risk two-sided"),
        c("TECK", "Teck Resources", 29.6, 1.76,
          vec![(R::GdpIndex, 0.6), (R::DcInfra, 0.4)],
          1.4, 0.0, 14.0, Long, vec![],
          "Cheapest copper (7x EV/EBITDA); QB2 +80% Cu ramp, Anglo merger overhang"),
        c("SYM", "Symbotic", 24.45, 0.35, vec![(R::GdpIndex, 1.0)],
          1.2, 0.10, 22.0, Long, vec![(E::RobotServices, 0.03, 0.25)],
          "KILLED at price (backlog math); watchlist <$25"),
        // ---- LONGS: platforms / commodities / info ----
        c("MSFT", "Microsoft", 2830.0, 137.0,
          vec![(R::SeatSaas, 0.3), (R::GdpIndex, 0.7)],
          0.9, 0.01, 22.0, Long, vec![(E::AiServices, 0.12, 0.40)],
          "Aggregator; round-2: watch ex-OpenAI RPO growth, medium size"),
        c("GOOGL", "Alphabet", 3890.0, 175.0,
          vec![(R::GdpIndex, 0.85), (R::Silicon, 0.15)],
          1.0, 0.01, 20.0, Long, vec![(E::AiServices, 0.08, 0.40)],
          "TPU stack + Waymo; round-2: buyback suspension, medium size"),
        c("FCX", "Freeport-McMoRan", 91.29, 4.6,
          vec![(R::GdpIndex, 0.6), (R::DcInfra, 0.4)],
          1.4, 0.0, 12.0, Long, vec![],
          "Copper proxy (killed at price; expression moved to deferred curve)"),
        c("EQT", "EQT", 33.4, 2.3, vec![(R::GdpIndex, 1.0)],
          0.8, 0.0, 10.0, Long, vec![(E::Electricity, 0.05, 0.35)],
          "Gas (killed at price; expression moved to Henry Hub calls)"),
        c("WTKWY", "Wolters Kluwer", 15.6, 1.5, vec![(R::ProfInfo, 1.0)],
          0.9, 0.02, 12.0, Long, vec![],
          "No position: model/red-team disagreement"),
        // ---- SHORTS ----
        c("ADP", "ADP", 97.1, 4.8,
          vec![(R::HumanCognitiveWages, 0.9), (R::GdpIndex, 0.1)],
          1.2, 0.02, 16.0, Short, vec![],
          "KILLED (elasticity empirically <0.5); trigger-gated puts"),
        c("PAYX", "Paychex", 39.4, 2.1,
          vec![(R::HumanCognitiveWages, 0.9), (R::GdpIndex, 0.1)],
          1.2, 0.015, 15.0, Short, vec![],
          "KILLED (SMB base is physical-labor heavy)"),
        c("RHI", "Robert Half", 3.6, 0.16, vec![(R::HumanCognitiveWages, 1.0)],
          2.0, -0.02, 8.0, Short, vec![],
          "Live short via 4-7mo put-spread ladders (round-2 restructure)"),
        c("MAN", "ManpowerGroup", 2.43, 0.17,
          vec![(R::HumanPhysicalWages, 0.6), (R::HumanCognitiveWages, 0.4)],
          1.8, -0.02, 8.0, Short, vec![],
          "KILLED (19.9% SI squeeze trap)"),
        c("TCS.NS", "Tata Consultancy", 97.5, 6.5, vec![(R::ItServices, 1.0)],
          1.3, 0.0, 11.0, Short, vec![],
          "KILLED high conf (thesis-incoherent: they sell the integration)"),
        c("WDAY", "Workday", 31.6, 2.9, vec![(R::SeatSaas, 1.0)],
          1.2, 0.0, 13.0, Short, vec![],
          "KILLED as delta-one; trigger-gated put spreads (cRPO <10%)"),
        c("TEAM", "Atlassian", 20.3, 1.51, vec![(R::SeatSaas, 1.0)],
          1.3, -0.02, 14.0, Short, vec![],
          "KILLED high conf (capture-ratio category error)"),
        c("HUBS", "HubSpot", 9.7, 0.72, vec![(R::SeatSaas, 1.0)],
          1.3, -0.02, 13.0, Short, vec![],
          "Live short, small, kill-watch (outcome-pricing pivot)"),
        c("FDS", "FactSet", 8.68, 0.68,
          vec![(R::SeatSaas, 0.5), (R::ProfInfo, 0.5)],
          1.0, -0.01, 10.0, Short, vec![],
          "KILLED by red team"),
        c("MMC", "Marsh McLennan", 84.2, 5.24,
          vec![(R::ProfInfo, 0.8), (R::GdpIndex, 0.2)],
          1.0, 0.0, 14.0, Short, vec![],
          "KILLED by red team"),
        c("CHRW", "CH Robinson", 24.2, 0.76,
          vec![(R::HumanPhysicalWages, 0.5), (R::GdpIndex, 0.5)],
          1.0, -0.05, 12.0, Short, vec![],
          "Live short post-print ($210-225 entry)"),
        c("LSTR", "Landstar", 7.06, 0.21,
          vec![(R::HumanPhysicalWages, 0.6), (R::GdpIndex, 0.4)],
          1.0, -0.04, 11.0, Short, vec![],
          "KILLED by red team"),
        c("MRVL", "Marvell", 183.3, 3.98, vec![(R::Silicon, 1.0)],
          1.0, -0.08, 22.0, Short, vec![],
          "Pair vs AVGO, half size, enter on relief"),
        c("6954.T", "FANUC", 44.0, 1.26, vec![(R::GdpIndex, 1.0)],
          0.9, -0.02, 22.0, Short, vec![(E::Robots, 0.05, 0.20)],
          "KILLED (fair with humanoid credit)"),
        c("6324.T", "Harmonic Drive", 4.43, 0.031, vec![(R::GdpIndex, 1.0)],
          1.0, 0.0, 18.0, Short, vec![(E::RobotComponents, 0.06, 0.28)],
          "KILLED outright (squeeze); pair-only revival"),
        c("EQIX", "Equinix", 101.9, 4.24,
          vec![(R::DcInfra, 0.3), (R::GdpIndex, 0.7)],
          0.5, -0.01, 18.0, Short, vec![],
          "KILLED by red team"),
        c("PLTR", "Palantir", 323.3, 4.0,
          vec![(R::SeatSaas, 0.3), (R::GdpIndex, 0.7)],
          1.1, 0.0, 27.0, Short, vec![(E::AiServices, 0.015, 0.30)],
          "Relative short vs MSFT/GOOGL sleeve; options only post-Aug-3"),
    ]
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
