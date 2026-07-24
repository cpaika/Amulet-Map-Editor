"""
v2 scenario presets mirroring the v1 five-scenario frame, plus an adapter so
the valuation engine (which consumes `states[i].pools`) can run directly on
the systems-dynamics model. YearV2 exposes the same pools dict as v1, so the
DCF layer is model-agnostic by construction.
"""

from __future__ import annotations

from model_v2 import ParamsV2, simulate_v2

SCENARIOS_V2: dict[str, ParamsV2] = {
    "baseline": ParamsV2(),
    "fast_takeoff": ParamsV2(
        singularity_boost=3.0,
        adoption_halflife=1.0,
        max_displacement_rate=0.30,
        chip_growth_ceiling=1.0,
        power_growth_ceiling=0.55,
        component_growth_ceiling=2.0,
        momentum_gain=1.0,
        capex_gdp_cap=0.08,
    ),
    "delayed": ParamsV2(
        singularity_year=2029,
        robotics_year=2030,
        adoption_halflife=2.5,
        max_displacement_rate=0.12,
    ),
    "friction": ParamsV2(
        adoption_halflife=3.5,
        max_displacement_rate=0.08,
        addressable_cognitive=0.55,
        backlash_gain=4.0,
        capex_gdp_cap=0.035,
    ),
    "fizzle": ParamsV2(
        singularity_year=2099,
        robotics_year=2099,
        algo_eff_growth_pre=1.8,
        adoption_halflife=4.0,
        max_displacement_rate=0.05,
    ),
}


def scenario_states_v2() -> dict:
    return {name: simulate_v2(p) for name, p in SCENARIOS_V2.items()}


if __name__ == "__main__":
    import json

    from companies import COMPANIES, load_financials
    from valuation import evaluate

    import sys
    comps = COMPANIES
    if len(sys.argv) > 1:
        comps = load_financials(sys.argv[1])
    comps = [c for c in comps if c.ntm_earnings_b > 0]
    states = scenario_states_v2()
    rows = sorted((evaluate(c, states) for c in comps),
                  key=lambda r: -r["expected_upside"])
    json.dump(rows, open("output/valuations_v2.json", "w"), indent=1)
    hdr = f"{'ticker':10} {'stance':6} {'E[up] v2':>9} {'worst':>7} {'best':>7}"
    print(hdr)
    print("-" * len(hdr))
    for r in rows:
        print(f"{r['ticker']:10} {r['stance']:6} {r['expected_upside']:>9.1%} "
              f"{r['worst_scenario_upside']:>7.1%} {r['best_scenario_upside']:>7.1%}")
