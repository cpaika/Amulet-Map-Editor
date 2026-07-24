"""
Valuation engine: converts model sector profit-pool paths into per-company
scenario valuations and trade scores.

Method
------
For each company we specify:
  * current market cap and NTM (2026) earnings,
  * which model pool(s) drive its earnings, its current share of that pool,
    and a `pool_beta` (operating leverage / share-shift sensitivity to the
    pool's growth — >1 means the company grows faster than its pool, <1 slower),
  * a share-drift term (annual share gain/loss vs the pool, e.g. custom-silicon
    share gains, or hyperscaler in-sourcing losses).

Earnings path:  E(t) = E_0 * (pool(t)/pool(0))^pool_beta * (1+share_drift)^t

Value per scenario = PV of 10y earnings + terminal value at a scenario-aware
multiple, discounted at `discount_rate`. We deliberately use a HIGH discount
rate (12%) — thesis uncertainty deserves a fat risk premium, and it guards
against the model "proving" everything is a buy.

Reverse DCF: `implied_cagr` solves for the constant earnings growth the
CURRENT market cap requires — the hurdle the thesis must clear (long) or the
optimism the short is fading.

Trade score = probability-weighted expected return across scenarios, with the
worst scenario reported as the risk leg. A long must survive `fizzle`; a short
must survive `fast_takeoff`.
"""

from __future__ import annotations

from dataclasses import dataclass, field

from model import Params, simulate
from scenarios import SCENARIOS

DISCOUNT_RATE = 0.12
HORIZON = 10  # years of explicit path (2027..2036)

# Subjective scenario probabilities UNDER THE USER'S THESIS as the analytical
# prior (thesis-conditional, not our unconditional view — the fizzle weight is
# what disciplines position sizing).
SCENARIO_PROBS = {
    "baseline": 0.35,
    "fast_takeoff": 0.15,
    "delayed": 0.25,
    "friction": 0.15,
    "fizzle": 0.10,
}


@dataclass
class Company:
    ticker: str
    name: str
    mcap_b: float                     # market cap, $B
    ntm_earnings_b: float             # NTM net earnings proxy, $B
    pools: dict                       # pool_name -> weight (sums to ~1.0)
    pool_beta: float = 1.0
    share_drift: float = 0.0          # annual share gain(+)/loss(-) vs pool
    terminal_multiple: float = 15.0
    stance: str = "watch"
    notes: str = ""


def earnings_path(c: Company, states) -> list[float]:
    base_pools = states[0].pools
    path = []
    for i, s in enumerate(states[1:], start=1):
        growth = 0.0
        for pool, w in c.pools.items():
            p0 = max(base_pools[pool], 1e-9)
            growth += w * (s.pools[pool] / p0)
        growth = max(growth, 0.0) ** c.pool_beta
        e = c.ntm_earnings_b * growth * ((1.0 + c.share_drift) ** i)
        path.append(e)
    return path[:HORIZON]


def pv(path: list[float], terminal_multiple: float, r: float = DISCOUNT_RATE) -> float:
    v = sum(e / ((1 + r) ** (i + 1)) for i, e in enumerate(path))
    v += (path[-1] * terminal_multiple) / ((1 + r) ** len(path))
    return v


def implied_cagr(mcap_b: float, e0: float, terminal_multiple: float,
                 r: float = DISCOUNT_RATE, years: int = HORIZON) -> float:
    """Reverse DCF: constant growth rate the current price requires."""
    lo, hi = -0.5, 1.5
    for _ in range(80):
        g = (lo + hi) / 2
        path = [e0 * ((1 + g) ** (i + 1)) for i in range(years)]
        if pv(path, terminal_multiple, r) < mcap_b:
            lo = g
        else:
            hi = g
    return (lo + hi) / 2


def evaluate(c: Company, scenario_states: dict) -> dict:
    per_scenario = {}
    for name, states in scenario_states.items():
        path = earnings_path(c, states)
        # scenario-aware terminal multiple: compress in fizzle, keep in others
        tm = c.terminal_multiple * (0.75 if name == "fizzle" else 1.0)
        fair = pv(path, tm)
        per_scenario[name] = {
            "fair_value_b": round(fair, 1),
            "upside": round(fair / c.mcap_b - 1.0, 3),
            "earnings_2030_b": round(path[3], 1),
            "earnings_2033_b": round(path[6], 1),
        }
    exp_up = sum(SCENARIO_PROBS[k] * v["upside"] for k, v in per_scenario.items())
    worst = min(per_scenario.values(), key=lambda v: v["upside"])
    best = max(per_scenario.values(), key=lambda v: v["upside"])
    return {
        "ticker": c.ticker,
        "name": c.name,
        "stance": c.stance,
        "mcap_b": c.mcap_b,
        "implied_cagr_at_current_price": round(
            implied_cagr(c.mcap_b, c.ntm_earnings_b, c.terminal_multiple), 3),
        "expected_upside": round(exp_up, 3),
        "worst_scenario_upside": worst["upside"],
        "best_scenario_upside": best["upside"],
        "per_scenario": per_scenario,
        "notes": c.notes,
    }


def evaluate_all(companies: list[Company]) -> list[dict]:
    scenario_states = {name: simulate(p) for name, p in SCENARIOS.items()}
    rows = [evaluate(c, scenario_states) for c in companies]
    # rank: longs by expected upside desc, shorts by expected upside asc
    return sorted(rows, key=lambda r: -r["expected_upside"])


if __name__ == "__main__":
    import json
    # demo with a stylized example until research calibrates real companies
    demo = [
        Company("DEMO-SI", "Stylized silicon leader", mcap_b=4000, ntm_earnings_b=110,
                pools={"silicon": 1.0}, pool_beta=1.1, terminal_multiple=18,
                stance="long"),
        Company("DEMO-IT", "Stylized IT outsourcer", mcap_b=200, ntm_earnings_b=8,
                pools={"it_services": 1.0}, pool_beta=1.2, terminal_multiple=12,
                stance="short"),
    ]
    print(json.dumps(evaluate_all(demo), indent=2))
