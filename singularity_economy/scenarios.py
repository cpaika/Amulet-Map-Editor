"""
Scenario definitions and Monte Carlo robustness analysis for the
singularity economy model.

The point of the Monte Carlo is NOT to predict — it is to find conclusions
that survive parameter uncertainty:
  * Which constraint binds (and therefore earns scarcity rents) in each year,
    across the plausible parameter space?
  * How fast do casualty pools (IT services, BPO, seat SaaS) decay across runs?
  * How large do silicon / power-equipment / robot-component pools get?

A trade is only interesting if it works across most of the parameter space,
or if its failure modes are cheap to hedge.
"""

from __future__ import annotations

import json
import random
import statistics
from dataclasses import replace

from model import Params, simulate, summarize


# ----------------------------------------------------------------------------
# Named scenarios
# ----------------------------------------------------------------------------

SCENARIOS: dict[str, Params] = {
    # The thesis as stated by the user.
    "baseline": Params(),

    # Everything goes right for AI, fast: stronger recursive burst, faster
    # adoption, robots ramp harder.
    "fast_takeoff": Params(
        singularity_boost=3.0,
        recursive_years=4,
        adoption_halflife=1.0,
        max_displacement_rate=0.30,
        robot_prod_growth_max=1.5,
        component_capacity_growth=1.3,
        chip_capacity_growth_max=0.70,
        power_additions_growth_max=0.45,
        capex_gdp_cap=0.08,
    ),

    # Singularity slips to 2029; robotics to 2030. The "AI is real but slower"
    # world — closest to current consensus.
    "delayed": Params(
        singularity_year=2029,
        robotics_year=2030,
        adoption_halflife=2.5,
        max_displacement_rate=0.12,
    ),

    # Capability arrives but diffusion is throttled by regulation, liability,
    # enterprise inertia, and compute rationing.
    "friction": Params(
        adoption_halflife=3.5,
        max_displacement_rate=0.08,
        addressable_cognitive=0.55,
        capex_gdp_cap=0.035,
    ),

    # The bear case for the thesis: no singularity, AI plateaus as a strong
    # but ordinary technology. This is the world where casualty shorts bleed
    # carry and picks-and-shovels de-rate.
    "fizzle": Params(
        singularity_year=2099,          # never within horizon
        robotics_year=2099,
        algo_eff_growth_pre=1.8,
        adoption_halflife=4.0,
        max_displacement_rate=0.05,
    ),
}


# ----------------------------------------------------------------------------
# Monte Carlo
# ----------------------------------------------------------------------------

def _draw(rng: random.Random) -> Params:
    """Sample a parameter set from wide-but-defensible priors."""
    sing_year = rng.choices([2027, 2028, 2029, 2030], weights=[0.40, 0.30, 0.20, 0.10])[0]
    return Params(
        singularity_year=sing_year,
        robotics_year=sing_year + rng.choice([1, 1, 2]),
        singularity_boost=rng.uniform(1.3, 3.0),
        recursive_years=rng.choice([2, 3, 3, 4]),
        algo_eff_growth_pre=rng.uniform(2.0, 3.2),
        algo_eff_growth_post=rng.uniform(1.3, 1.8),
        adoption_halflife=rng.uniform(1.0, 3.5),
        max_displacement_rate=rng.uniform(0.10, 0.30),
        addressable_cognitive=rng.uniform(0.6, 0.95),
        cognitive_demand_elasticity=rng.uniform(1.1, 1.7),
        chip_capacity_growth_max=rng.uniform(0.35, 0.75),
        power_additions_growth_max=rng.uniform(0.18, 0.50),
        capex_gdp_cap=rng.uniform(0.035, 0.09),
        robot_prod_growth_max=rng.uniform(0.6, 1.6),
        component_capacity_growth=rng.uniform(0.5, 1.4),
        robot_learning_rate=rng.uniform(0.15, 0.30),
        robot_cost_2028_k=rng.uniform(45.0, 95.0),
        it_services_beta=rng.uniform(0.6, 1.1),
        bpo_beta=rng.uniform(0.9, 1.5),
        saas_beta=rng.uniform(0.45, 1.0),
        prof_info_beta=rng.uniform(0.2, 0.6),
        power_efficiency_gain=rng.uniform(0.08, 0.18),
    )


def monte_carlo(n: int = 800, seed: int = 7) -> dict:
    rng = random.Random(seed)
    years = list(range(Params().start_year, Params().end_year + 1))

    binding_counts = {y: {} for y in years}
    pool_samples: dict[str, dict[int, list[float]]] = {}
    disp_samples = {y: [] for y in years}
    capex_samples = {y: [] for y in years}
    robot_prod_2032 = []

    for _ in range(n):
        p = _draw(rng)
        states = simulate(p)
        for s in states:
            binding_counts[s.year][s.binding] = binding_counts[s.year].get(s.binding, 0) + 1
            disp_samples[s.year].append(s.cog_displacement)
            capex_samples[s.year].append(s.ai_capex)
            for k, v in s.pools.items():
                pool_samples.setdefault(k, {y: [] for y in years})[s.year].append(v)
            if s.year == 2032:
                robot_prod_2032.append(s.robot_prod_m)

    def pct(vals, q):
        vals = sorted(vals)
        return vals[min(int(q * len(vals)), len(vals) - 1)]

    def dist(samples_by_year):
        return {y: {"p10": round(pct(v, 0.10), 3),
                    "p50": round(pct(v, 0.50), 3),
                    "p90": round(pct(v, 0.90), 3)}
                for y, v in samples_by_year.items()}

    return {
        "n_runs": n,
        "binding_constraint_frequency": {
            y: {k: round(c / n, 3) for k, c in sorted(cnt.items(), key=lambda kv: -kv[1])}
            for y, cnt in binding_counts.items()
        },
        "ai_capex_dist_$T": dist(capex_samples),
        "cognitive_displacement_dist": dist(disp_samples),
        "robot_prod_2032_m": {
            "p10": round(pct(robot_prod_2032, 0.10), 2),
            "p50": round(pct(robot_prod_2032, 0.50), 2),
            "p90": round(pct(robot_prod_2032, 0.90), 2),
        },
        "pool_dists_$T": {k: dist(v) for k, v in pool_samples.items()},
    }


def run_all() -> dict:
    out = {"scenarios": {}, "monte_carlo": None}
    for name, params in SCENARIOS.items():
        out["scenarios"][name] = summarize(simulate(params))
    out["monte_carlo"] = monte_carlo()
    return out


if __name__ == "__main__":
    print(json.dumps(run_all(), indent=2))
