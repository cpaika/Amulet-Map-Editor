"""
Company universe: maps each candidate trade to model pools with analytic
judgments (pool weights, operating-leverage beta, share drift, terminal
multiple). Market caps / NTM earnings are placeholders from the research sweep
until `load_financials()` overrides them with the phase-2 deep-dive output.

Pool-mapping conventions
------------------------
- `pools` weights sum to ~1.0. `gdp_index` is the neutral "grows with the
  economy" bucket for conglomerate/legacy segments.
- `pool_beta` >1: operating leverage or price-taker torque on the pool;
  <1: contracted/backlogged smoothing.
- `share_drift`: annual share gain(+)/loss(-) vs the pool — company-specific
  judgments (socket losses, commoditization, capacity-led share gains).
- Consumer-credit and duration trades (SYF, ALLY, TLT) are NOT pool-mappable;
  they are evaluated qualitatively in the report (transition-recession tail).
"""

from __future__ import annotations

import json

from valuation import Company

# stance, mcap_b, ntm_earnings_b are refreshed from phase-2 financials.
COMPANIES: list[Company] = [
    # ---------------- LONGS: compute complex ----------------
    Company("TSM", "TSMC", 1930, 100, {"silicon": 0.9, "gdp_index": 0.1},
            pool_beta=0.9, share_drift=0.01, terminal_multiple=18, stance="long",
            notes="Dual chokepoint (N2 + CoWoS); IP-moat rent class -> durable"),
    Company("NVDA", "NVIDIA", 5060, 240, {"silicon": 1.0},
            pool_beta=1.15, share_drift=-0.03, terminal_multiple=16, stance="long",
            notes="Share loss to ASICs inside a pool growing 2-4x consensus"),
    Company("AVGO", "Broadcom", 1870, 75, {"silicon": 0.8, "gdp_index": 0.2},
            pool_beta=1.05, share_drift=0.03, terminal_multiple=20, stance="long",
            notes="Custom-ASIC royalty on hyperscaler in-sourcing"),
    Company("MU", "Micron", 959, 120, {"silicon": 1.0},
            pool_beta=0.6, share_drift=0.01, terminal_multiple=8, stance="long",
            notes="Capacity-scarcity rent class -> harvest early; low terminal multiple"),
    Company("000660.KS", "SK Hynix", 1200, 170, {"silicon": 1.0},
            pool_beta=0.6, share_drift=0.0, terminal_multiple=8, stance="long",
            notes="HBM leader; same cycle caveat as MU"),
    Company("BESI.AS", "BE Semiconductor", 15, 0.35, {"silicon": 1.0},
            pool_beta=1.3, share_drift=0.04, terminal_multiple=22, stance="long",
            notes="Hybrid bonding adoption = share gain against stacking intensity"),
    Company("ASML", "ASML", 450, 12.5, {"silicon": 1.0},
            pool_beta=0.9, share_drift=0.01, terminal_multiple=22, stance="long",
            notes="Litho monopoly; IP-moat rent class"),
    # ---------------- LONGS: power / grid ----------------
    Company("GEV", "GE Vernova", 274, 8.5, {"power_equipment": 1.0},
            pool_beta=1.1, share_drift=0.01, terminal_multiple=18, stance="long",
            notes="Sold out through 2030; backlog = contracted scarcity"),
    Company("VST", "Vistra", 65, 3.8, {"gdp_index": 1.0},
            pool_beta=0.4, share_drift=0.0, terminal_multiple=14, stance="long",
            new_pool_capture={"electricity": (0.05, 0.5)},
            notes="Merchant torque to power scarcity at a discount to CEG"),
    Company("NRG", "NRG Energy", 35, 2.4, {"gdp_index": 1.0},
            pool_beta=0.4, share_drift=0.01, terminal_multiple=13, stance="long",
            new_pool_capture={"electricity": (0.045, 0.45)}),
    Company("CEG", "Constellation", 110, 3.6, {"gdp_index": 1.0},
            pool_beta=0.4, share_drift=0.01, terminal_multiple=18, stance="long",
            new_pool_capture={"electricity": (0.06, 0.5)},
            notes="Contracted 20yr PPAs = royalty-acre economics"),
    Company("POWL", "Powell Industries", 4.5, 0.19, {"power_equipment": 1.0},
            pool_beta=1.3, share_drift=0.01, terminal_multiple=15, stance="long"),
    Company("6501.T", "Hitachi", 130, 6.0,
            {"power_equipment": 0.45, "dc_infra": 0.15, "gdp_index": 0.40},
            pool_beta=1.0, share_drift=0.005, terminal_multiple=15, stance="long",
            notes="Hitachi Energy transformer bottleneck inside a conglomerate"),
    # ---------------- LONGS: networking / optics ----------------
    Company("LITE", "Lumentum", 12, 0.45, {"silicon": 1.0},
            pool_beta=1.2, share_drift=0.02, terminal_multiple=14, stance="long",
            notes="EML sold out past 2027; capacity-scarcity class -> monitor supply response"),
    Company("CLS", "Celestica", 22, 0.65, {"dc_infra": 0.6, "silicon": 0.4},
            pool_beta=1.1, share_drift=0.02, terminal_multiple=15, stance="long"),
    # ---------------- LONGS: robotics chain ----------------
    Company("MP", "MP Materials", 8, 0.05, {"gdp_index": 1.0},
            pool_beta=1.0, share_drift=0.10, terminal_multiple=20, stance="long",
            new_pool_capture={"robot_components": (0.10, 0.25)},
            notes="Geopolitical scarcity (non-China magnets), DoD price floor"),
    Company("6268.T", "Nabtesco", 4, 0.15, {"gdp_index": 1.0},
            pool_beta=1.0, share_drift=0.0, terminal_multiple=16, stance="long",
            new_pool_capture={"robot_components": (0.08, 0.22)}),
    Company("SHA.DE", "Schaeffler", 6, 0.7, {"gdp_index": 1.0},
            pool_beta=1.0, share_drift=-0.01, terminal_multiple=9, stance="long",
            new_pool_capture={"robot_components": (0.05, 0.15)},
            notes="Deep value + robot components optionality; ICE drag real"),
    Company("SYM", "Symbotic", 26, 0.35, {"gdp_index": 1.0},
            pool_beta=1.2, share_drift=0.10, terminal_multiple=22, stance="long",
            new_pool_capture={"robot_services": (0.03, 0.25)},
            notes="Contracted backlog converts ahead of humanoid hype"),
    # ---------------- LONGS: platforms / commodities / info ----------------
    Company("MSFT", "Microsoft", 3400, 118, {"seat_saas": 0.3, "gdp_index": 0.7},
            pool_beta=0.9, share_drift=0.01, terminal_multiple=18, stance="long",
            new_pool_capture={"ai_services": (0.12, 0.40)},
            notes="Aggregator: agent distribution offsets seat erosion"),
    Company("GOOGL", "Alphabet", 2400, 135, {"gdp_index": 0.85, "silicon": 0.15},
            pool_beta=1.0, share_drift=0.01, terminal_multiple=16, stance="long",
            new_pool_capture={"ai_services": (0.08, 0.40)},
            notes="TPU full stack + Waymo optionality at market multiple"),
    Company("FCX", "Freeport-McMoRan", 65, 4.2, {"gdp_index": 0.6, "dc_infra": 0.4},
            pool_beta=1.4, share_drift=0.0, terminal_multiple=12, stance="long",
            notes="Copper deficit proxy; beta carries the commodity torque"),
    Company("EQT", "EQT", 35, 2.2, {"gdp_index": 1.0},
            pool_beta=0.8, share_drift=0.0, terminal_multiple=12, stance="long",
            new_pool_capture={"electricity": (0.05, 0.35)}),
    Company("U-UN.TO", "Sprott Physical Uranium", 5.2, 0.0, {"gdp_index": 1.0},
            pool_beta=0.8, share_drift=0.0, terminal_multiple=1, stance="long",
            notes="NAV vehicle - valuation engine not meaningful; qualitative"),
    Company("WTKWY", "Wolters Kluwer", 30, 1.6, {"prof_info": 1.0},
            pool_beta=0.9, share_drift=0.02, terminal_multiple=15, stance="long",
            notes="Regulated-workflow moat inside a decaying pool class"),
    # ---------------- SHORTS ----------------
    Company("ADP", "ADP", 105, 4.3, {"human_cognitive_wages": 0.9, "gdp_index": 0.1},
            pool_beta=1.2, share_drift=0.02, terminal_multiple=14, stance="short",
            notes="Pays-per-control elasticity 0.7-1.0 to shrinking employment"),
    Company("PAYX", "Paychex", 48, 1.9, {"human_cognitive_wages": 0.9, "gdp_index": 0.1},
            pool_beta=1.2, share_drift=0.015, terminal_multiple=14, stance="short"),
    Company("RHI", "Robert Half", 4.5, 0.20, {"human_cognitive_wages": 1.0},
            pool_beta=2.0, share_drift=-0.02, terminal_multiple=9, stance="short",
            notes="Perm placement = first casualty; payout > NI"),
    Company("MAN", "ManpowerGroup", 3.5, 0.25, {"human_physical_wages": 0.6,
                                                "human_cognitive_wages": 0.4},
            pool_beta=1.8, share_drift=-0.02, terminal_multiple=8, stance="short",
            notes="Industrial staffing unpriced for 2028+ robotics"),
    Company("TCS.NS", "Tata Consultancy", 95, 6.0, {"it_services": 1.0},
            pool_beta=1.3, share_drift=0.0, terminal_multiple=10, stance="short",
            notes="Premium multiple vs peers with max labor-arbitrage exposure"),
    Company("WDAY", "Workday", 31.6, 2.4, {"seat_saas": 1.0},
            pool_beta=1.2, share_drift=0.0, terminal_multiple=11, stance="short"),
    Company("TEAM", "Atlassian", 20.3, 1.4, {"seat_saas": 1.0},
            pool_beta=1.3, share_drift=-0.02, terminal_multiple=11, stance="short",
            notes="Agent-native pipelines collapse the tracked-work loop"),
    Company("HUBS", "HubSpot", 9.7, 0.65, {"seat_saas": 1.0},
            pool_beta=1.3, share_drift=-0.02, terminal_multiple=11, stance="short"),
    Company("FDS", "FactSet", 12, 0.60, {"seat_saas": 0.5, "prof_info": 0.5},
            pool_beta=1.0, share_drift=-0.01, terminal_multiple=12, stance="short",
            notes="ASV tied to front-office headcount"),
    Company("MMC", "Marsh McLennan", 95, 4.6, {"prof_info": 0.8, "gdp_index": 0.2},
            pool_beta=1.0, share_drift=0.0, terminal_multiple=14, stance="short"),
    Company("CHRW", "CH Robinson", 18, 0.75, {"human_physical_wages": 0.5,
                                              "gdp_index": 0.5},
            pool_beta=1.0, share_drift=-0.05, terminal_multiple=11, stance="short",
            notes="AI-advantage narrative vs AI commoditizing brokerage itself"),
    Company("LSTR", "Landstar", 6, 0.30, {"human_physical_wages": 0.6, "gdp_index": 0.4},
            pool_beta=1.0, share_drift=-0.04, terminal_multiple=10, stance="short"),
    Company("MRVL", "Marvell", 200, 3.2, {"silicon": 1.0},
            pool_beta=1.0, share_drift=-0.08, terminal_multiple=13, stance="short",
            notes="Socket erosion inside the boom; pair vs AVGO long"),
    Company("6954.T", "FANUC", 44, 1.5, {"gdp_index": 1.0},
            pool_beta=0.9, share_drift=-0.02, terminal_multiple=18, stance="short",
            new_pool_capture={"robots": (0.05, 0.20)},
            notes="Humanoid halo at 39x with zero humanoid revenue"),
    Company("6324.T", "Harmonic Drive", 6, 0.02, {"gdp_index": 1.0},
            pool_beta=1.0, share_drift=0.0, terminal_multiple=25, stance="short",
            new_pool_capture={"robot_components": (0.06, 0.28)},
            notes="404x trailing; Corning-2000 setup vs Chinese commoditization"),
    Company("EQIX", "Equinix", 75, 2.6, {"dc_infra": 0.3, "gdp_index": 0.7},
            pool_beta=0.5, share_drift=-0.01, terminal_multiple=16, stance="short",
            notes="Margin squeeze between equipment suppliers and hyperscale tenants"),
    Company("PLTR", "Palantir", 320, 2.3, {"seat_saas": 0.3, "gdp_index": 0.7},
            pool_beta=1.1, share_drift=0.0, terminal_multiple=25, stance="short",
            new_pool_capture={"ai_services": (0.015, 0.30)},
            notes="139x trailing: growth is real, hurdle is impossible"),
]


def load_financials(path: str) -> list[Company]:
    """Override placeholder mcap/NTM/terminal-multiple with phase-2 agent data."""
    with open(path) as f:
        fin = json.load(f)
    out = []
    for c in COMPANIES:
        d = fin.get(c.ticker)
        if d:
            c.mcap_b = d.get("mcap_b", c.mcap_b)
            ntm = d.get("ntm_earnings_b", c.ntm_earnings_b)
            if ntm and ntm > 0:
                c.ntm_earnings_b = ntm
            tm = d.get("suggested_terminal_multiple")
            if tm and 4 <= tm <= 30:
                c.terminal_multiple = tm
        out.append(c)
    return out


if __name__ == "__main__":
    import sys
    from valuation import evaluate_all
    comps = COMPANIES
    if len(sys.argv) > 1:
        comps = load_financials(sys.argv[1])
    comps = [c for c in comps if c.ntm_earnings_b > 0]  # NAV vehicles handled separately
    rows = evaluate_all(comps)
    json.dump(rows, open("output/valuations.json", "w"), indent=1)
    hdr = f"{'ticker':10} {'stance':6} {'implied CAGR':>12} {'E[up]':>7} {'worst':>7} {'best':>7}"
    print(hdr); print("-" * len(hdr))
    for r in rows:
        print(f"{r['ticker']:10} {r['stance']:6} {r['implied_cagr_at_current_price']:>12.1%} "
              f"{r['expected_upside']:>7.1%} {r['worst_scenario_upside']:>7.1%} "
              f"{r['best_scenario_upside']:>7.1%}")
