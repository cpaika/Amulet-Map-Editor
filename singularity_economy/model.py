"""
Singularity Economy Model
=========================

A multi-sector, bottleneck-propagating simulation of the world economy 2026-2036
under the thesis:

  * Software singularity in 2027: AI agents reach/exceed expert human capability
    across cognitive work, with cost per task orders of magnitude below wages.
    AI R&D becomes partially recursive (AI improving AI), accelerating algorithmic
    efficiency for several years.
  * General-purpose robotics ramps from 2028: humanoid/specialized robots begin
    substituting physical labor, production compounding through the early 2030s.

Design philosophy
-----------------
This is NOT a general-equilibrium model. It is a *bottleneck accounting* model:
at each annual step, desired AI expansion is confronted with a set of physical
supply constraints (chips, power, capital, robot components). The binding
constraint caps growth and earns a scarcity rent; everything downstream of the
constraint is throttled, everything upstream of demand is deflated. The
investable outputs are:

  1. WHICH constraint binds in WHICH years (the scarcity-rent schedule),
  2. sector revenue / profit-pool trajectories (who grows, who decays, how fast),
  3. casualty decay curves (human-cognitive-arbitrage revenue),
  4. robustness of the above across a Monte Carlo over uncertain parameters.

All money figures are in trillions of 2026 USD unless noted. Compute is an
index (1.0 = 2026 installed effective AI compute). Power is GW dedicated to AI.

Pure stdlib on purpose: auditable, portable, no dependencies.
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field, asdict


# ----------------------------------------------------------------------------
# Parameters
# ----------------------------------------------------------------------------

@dataclass
class Params:
    # --- timeline ---
    start_year: int = 2026
    end_year: int = 2036
    singularity_year: int = 2027       # software singularity
    robotics_year: int = 2028          # robot production ramp begins

    # --- macro baseline (2026, $T) ---
    world_gdp: float = 115.0
    cognitive_wage_bill: float = 28.0   # global comp to predominantly-cognitive workers
    physical_wage_bill: float = 34.0    # global comp to predominantly-physical workers
    cognitive_workers_m: float = 950.0  # millions of cognitive workers globally
    physical_workers_m: float = 2400.0  # millions of physical workers globally

    # --- casualty revenue pools (2026, $T) ---
    it_services_pool: float = 1.55      # IT services / outsourcing global revenue
    bpo_pool: float = 0.36              # BPO / contact centers
    seat_saas_pool: float = 0.35        # seat-priced SaaS
    prof_info_pool: float = 0.45        # legal/tax/financial info & data services

    # --- compute supply chain ---
    ai_capex_2026: float = 0.65                 # $T/yr AI datacenter capex (2026)
    compute_deprec: float = 0.25                # annual depreciation of compute stock
    chip_capacity_growth_max: float = 0.55      # max yoy growth in accelerator output
    algo_eff_growth_pre: float = 2.5            # pre-singularity algo+arch efficiency x/yr
    singularity_boost: float = 2.0              # extra multiplier during recursive phase
    recursive_years: int = 3                    # how long the recursive burst lasts
    algo_eff_growth_post: float = 1.5           # steady-state efficiency gain after burst

    # --- power supply chain (GW dedicated to AI) ---
    ai_power_2026: float = 55.0                 # GW consumed by AI in 2026
    power_additions_2026: float = 24.0          # GW/yr addition rate in 2026
    power_additions_growth_max: float = 0.28    # turbine/transformer-limited ramp of additions
    power_lead_years: int = 2                   # order-to-energization lag beyond current pipeline
    gw_per_compute_unit: float = 55.0           # GW per unit of compute stock at 2026 efficiency
    power_efficiency_gain: float = 0.12         # yearly perf/W gain (reduces GW per compute)
    power_equip_cost_per_gw: float = 0.0035     # $T of power/electrical equipment per GW built

    # --- AI cognitive supply / demand ---
    ai_hew_2026_m: float = 12.0                 # AI human-equivalent cognitive workers (M), 2026
    adoption_halflife: float = 1.6              # years to 50% of addressable after singularity
    max_displacement_rate: float = 0.22         # max yoy shrink of human cognitive employment
    cognitive_demand_elasticity: float = 1.35   # |elasticity|: task demand expands as price falls
    ai_task_price_rel: float = 0.04             # AI cost per task vs human wage at singularity
    addressable_cognitive: float = 0.85         # share of cognitive work ultimately AI-doable

    # --- robotics ---
    robot_prod_2028_m: float = 0.12             # M units/yr in first ramp year
    robot_prod_growth_max: float = 1.10         # max yoy production growth (component-limited)
    component_capacity_growth: float = 0.90     # actuator/reducer/magnet supply growth cap
    robot_cost_2028_k: float = 50.0             # $k per robot in 2028
    robot_learning_rate: float = 0.22           # cost decline per doubling of cumulative units
    robot_hew: float = 1.4                      # physical human-equivalents per robot (multi-shift)
    robot_attrition: float = 0.08               # fleet attrition/yr
    physical_addressable: float = 0.65          # share of physical work robot-addressable by 2036
    max_physical_displacement_rate: float = 0.15

    # --- capital / margins ---
    capex_gdp_cap: float = 0.055                # max share of GDP devoted to AI+robot capex
    silicon_share_of_capex: float = 0.55        # accelerators+memory+network as share of DC capex
    bottleneck_margin: float = 0.55             # EBIT margin earned by the binding constraint
    nonbottleneck_margin: float = 0.22          # EBIT margin when not binding
    ai_services_margin: float = 0.35            # margin on AI-delivered cognitive services

    # --- macro feedback ---
    base_gdp_growth: float = 0.03           # trend real growth
    productivity_passthrough: float = 0.35  # share of AI output gains hitting GDP
    transition_drag: float = 0.5            # GDP drag per unit of *new* displacement
                                            # (displaced income not yet recycled)

    # --- casualty dynamics ---
    it_services_beta: float = 0.9    # sensitivity of IT-services revenue to displacement
    bpo_beta: float = 1.25           # BPO dies fastest (most commodity cognitive work)
    saas_beta: float = 0.75          # seat SaaS: seats shrink, some value re-captured via AI SKUs
    prof_info_beta: float = 0.35     # info providers partly protected by data moats/regulation


CONSTRAINTS = ("power", "chips", "capital", "demand")


# ----------------------------------------------------------------------------
# Simulation
# ----------------------------------------------------------------------------

@dataclass
class YearState:
    year: int
    # compute complex
    compute_stock: float
    algo_eff: float
    ai_capex: float
    binding: str
    scarcity: dict            # constraint -> utilization (1.0 = fully binding)
    ai_power_gw: float
    power_additions_gw: float
    # cognitive labor
    ai_hew_m: float           # AI human-equivalent cognitive workers deployed (M)
    human_cog_workers_m: float
    cog_displacement: float   # cumulative share of 2026 human cognitive work displaced
    cognitive_task_index: float  # total cognitive work consumed vs 2026 (Jevons expansion)
    # robotics
    robot_prod_m: float
    robot_fleet_m: float
    robot_cost_k: float
    phys_displacement: float
    # revenue pools ($T/yr)
    pools: dict
    # profit pools ($T/yr)
    profits: dict
    # macro
    gdp: float = 0.0


def logistic(x: float) -> float:
    return 1.0 / (1.0 + math.exp(-x))


def simulate(p: Params) -> list[YearState]:
    years = list(range(p.start_year, p.end_year + 1))

    # state init (2026)
    compute_stock = 1.0
    algo_eff = 1.0
    ai_power = p.ai_power_2026
    power_additions = p.power_additions_2026
    chip_output = p.ai_capex_2026 * p.silicon_share_of_capex  # $T/yr of silicon
    human_cog_m = p.cognitive_workers_m
    cognitive_wage_bill = p.cognitive_wage_bill
    robot_prod = 0.0
    robot_fleet = 0.0
    component_capacity = p.robot_prod_2028_m  # units/yr the component chain can support
    cum_robots = 0.02  # small seed so learning curve is defined
    robot_cost = p.robot_cost_2028_k
    phys_workers_m = p.physical_workers_m

    gw_per_unit = p.gw_per_compute_unit
    gdp = p.world_gdp
    prev_disp_for_macro = 0.0

    out: list[YearState] = []

    for year in years:
        t_sing = year - p.singularity_year

        # ---------------- algorithmic efficiency ----------------
        if year < p.singularity_year:
            algo_growth = p.algo_eff_growth_pre
        elif t_sing < p.recursive_years:
            algo_growth = p.algo_eff_growth_pre * p.singularity_boost
        else:
            algo_growth = p.algo_eff_growth_post
        if year > p.start_year:
            algo_eff *= algo_growth

        # ---------------- AI cognitive supply & demand ----------------
        # raw AI capability (human-equivalent workers, millions)
        ai_hew_raw = p.ai_hew_2026_m * compute_stock * algo_eff
        # adoption: logistic in time since singularity (integration friction)
        if t_sing < 0:
            # slow pre-singularity uptake, capped: without a singularity, AI
            # remains a strong-but-ordinary technology with limited displacement
            adopt = min(0.08 * (1.6 ** (year - p.start_year)), 0.28)
        else:
            k = math.log(3.0) / p.adoption_halflife  # logistic(k*halflife)=0.75-ish shape
            adopt = logistic(k * (t_sing - p.adoption_halflife) + math.log(1.0))
            adopt = max(adopt, 0.10)
        addressable_hew = p.cognitive_workers_m * p.addressable_cognitive
        # Jevons: as AI price << wages, total cognitive task demand expands
        price_ratio = p.ai_task_price_rel if t_sing >= 0 else 0.25
        task_expansion = min(price_ratio ** (-(p.cognitive_demand_elasticity - 1.0) * 0.35), 6.0)
        # expansion phases in with adoption
        cognitive_task_index = 1.0 + (task_expansion - 1.0) * adopt
        demand_hew = addressable_hew * adopt * cognitive_task_index
        ai_hew = min(ai_hew_raw, demand_hew)
        compute_demand_binding = ai_hew_raw > demand_hew * 1.05

        # human displacement: AI first absorbs task growth, then displaces
        displacement_target = min(ai_hew / max(cognitive_task_index, 1e-9), addressable_hew) \
            / p.cognitive_workers_m
        prev_disp = 1.0 - human_cog_m / p.cognitive_workers_m
        max_step = p.max_displacement_rate
        disp = min(displacement_target * min(adopt * 1.4, 1.0), prev_disp + max_step)
        disp = max(disp, prev_disp)  # ratchet: displaced work doesn't come back
        human_cog_m = p.cognitive_workers_m * (1.0 - disp)

        # ---------------- compute buildout & bottlenecks ----------------
        # desired capex: enough to close the gap to demand within ~2 years,
        # bounded by economic value of AI services
        gap_units = max(demand_hew - ai_hew_raw, 0.0) / max(p.ai_hew_2026_m * algo_eff, 1e-9)
        unit_cost = chip_output / max(compute_stock * p.chip_capacity_growth_max, 1e-9)
        desired_capex = p.ai_capex_2026 * (1.32 ** (year - p.start_year))
        if t_sing >= 0:
            desired_capex *= (1.0 + 2.2 * adopt)   # singularity demand shock
        desired_capex += gap_units * 0.0  # gap informs desire via adopt shock above

        # constraint 1: chips (silicon output can grow at most chip_capacity_growth_max)
        chip_output_next = chip_output * (1.0 + p.chip_capacity_growth_max)
        chips_cap = chip_output_next / p.silicon_share_of_capex  # $T of DC capex supportable

        # constraint 2: power. cost_per_unit: $T of capex per unit of compute stock
        # added (2026: $420B bought ~0.8 units gross of deprec => ~$0.52T/unit),
        # improving ~15%/yr on a hardware-$ basis.
        gw_per_unit *= (1.0 - p.power_efficiency_gain)
        hw_cost_index = 0.85 ** (year - p.start_year)
        cost_per_unit = (p.ai_capex_2026 /
                         (p.chip_capacity_growth_max + p.compute_deprec)) * hw_cost_index
        power_headroom = max(ai_power + power_additions - compute_stock * gw_per_unit, 0.0)
        power_cap = (power_headroom / gw_per_unit) * cost_per_unit

        # constraint 3: capital (share of GDP willing to fund AI capex)
        capital_cap = gdp * p.capex_gdp_cap

        caps = {"chips": chips_cap, "power": power_cap, "capital": capital_cap,
                "demand": desired_capex}
        ai_capex = min(caps.values())
        binding = min(caps, key=lambda k: caps[k])
        if compute_demand_binding and binding == "demand":
            binding = "demand"
        scarcity = {k: (ai_capex / v if v > 0 else 1.0) for k, v in caps.items()}

        # convert realized capex to compute-stock growth via cost_per_unit
        units_added = ai_capex / cost_per_unit
        compute_stock = compute_stock * (1.0 - p.compute_deprec) + units_added
        chip_output = min(chip_output_next, ai_capex * p.silicon_share_of_capex * 1.05)
        chip_output = max(chip_output, p.ai_capex_2026 * p.silicon_share_of_capex * 0.6)

        # power stock evolves; additions ramp is supply-constrained (turbines etc.)
        ai_power = min(ai_power + power_additions, ai_power + power_additions)
        used_power = compute_stock * gw_per_unit
        ai_power = max(ai_power, used_power)  # book actual consumption
        power_additions *= (1.0 + p.power_additions_growth_max)

        # ---------------- robotics ----------------
        if year >= p.robotics_year:
            if robot_prod == 0.0:
                robot_prod = p.robot_prod_2028_m
            else:
                robot_prod *= (1.0 + p.robot_prod_growth_max)
            component_capacity *= (1.0 + p.component_capacity_growth)
            robot_prod = min(robot_prod, component_capacity * 1.15)
            cum_prev = cum_robots
            cum_robots += robot_prod
            doublings = math.log2(max(cum_robots, 1e-9) / max(0.06, 1e-9))
            robot_cost = p.robot_cost_2028_k * ((1.0 - p.robot_learning_rate)
                                                ** max(doublings, 0.0))
            robot_cost = max(robot_cost, 8.0)  # BOM floor ($k)
            robot_fleet = robot_fleet * (1.0 - p.robot_attrition) + robot_prod

        phys_hew = robot_fleet * p.robot_hew
        phys_disp_target = min(phys_hew / p.physical_workers_m, p.physical_addressable)
        prev_pd = 1.0 - phys_workers_m / p.physical_workers_m
        pd = min(phys_disp_target, prev_pd + p.max_physical_displacement_rate)
        pd = max(pd, prev_pd)
        phys_workers_m = p.physical_workers_m * (1.0 - pd)

        # ---------------- revenue & profit pools ----------------
        avg_cog_wage = p.cognitive_wage_bill / p.cognitive_workers_m  # $T per M workers
        avg_phys_wage = p.physical_wage_bill / p.physical_workers_m

        # AI services revenue: displaced work billed at a discount to human cost,
        # plus expanded (Jevons) task volume at low AI prices
        displaced_value = disp * p.cognitive_workers_m * avg_cog_wage
        ai_services_rev = displaced_value * 0.45  # AI captures ~45% of replaced wage $
        expansion_rev = (cognitive_task_index - 1.0) * p.cognitive_wage_bill * 0.06
        ai_services_rev += expansion_rev

        silicon_rev = ai_capex * p.silicon_share_of_capex
        power_equip_rev = p.power_equip_cost_per_gw * power_additions
        dc_infra_rev = ai_capex * (1.0 - p.silicon_share_of_capex)
        electricity_rev = ai_power * 8760 * 0.055 / 1e6 * 1.0  # $T: GW*h*$/kWh
        robot_rev = robot_prod * robot_cost / 1e3           # M units * $k -> $B -> /1e3 $T
        robot_component_rev = robot_rev * 0.55
        robot_services_value = pd * p.physical_workers_m * avg_phys_wage * 0.35

        # casualties decay with displacement (beta-scaled), with baseline drift
        def casualty(pool0: float, beta: float, drift: float) -> float:
            organic = pool0 * ((1.0 + drift) ** (year - p.start_year))
            return organic * max(1.0 - beta * disp, 0.05)

        pools = {
            "ai_services": ai_services_rev,
            "silicon": silicon_rev,
            "dc_infra": dc_infra_rev,
            "power_equipment": power_equip_rev,
            "electricity": electricity_rev,
            "robots": robot_rev,
            "robot_components": robot_component_rev,
            "robot_services": robot_services_value,
            "it_services": casualty(p.it_services_pool, p.it_services_beta, 0.04),
            "bpo": casualty(p.bpo_pool, p.bpo_beta, 0.03),
            "seat_saas": casualty(p.seat_saas_pool, p.saas_beta, 0.08),
            "prof_info": casualty(p.prof_info_pool, p.prof_info_beta, 0.05),
            "human_cognitive_wages": human_cog_m * avg_cog_wage,
            "human_physical_wages": phys_workers_m * avg_phys_wage,
            # pseudo-pool: lets the valuation layer map GDP-linked businesses
            "gdp_index": gdp,
        }

        # margins: the binding constraint earns scarcity rents
        def margin_for(sector: str) -> float:
            bonus = {"chips": "silicon", "power": "power_equipment"}.get(binding)
            if sector == bonus:
                return p.bottleneck_margin
            if sector in ("silicon", "power_equipment", "robot_components"):
                return (p.bottleneck_margin + p.nonbottleneck_margin) / 2
            if sector == "ai_services":
                return p.ai_services_margin if binding == "demand" else p.ai_services_margin + 0.1
            return p.nonbottleneck_margin

        profits = {k: v * margin_for(k) for k, v in pools.items()
                   if not k.startswith("human_") and k != "gdp_index"}

        # ---------------- macro feedback ----------------
        # AI/robot output lifts GDP (productivity), but freshly displaced labor
        # income is a transition drag until recycled (fiscal transfers, new
        # jobs, capital income). Net effect can be negative in peak-displacement
        # years — the "transition recession" risk.
        new_disp = max(disp - prev_disp_for_macro, 0.0)
        prev_disp_for_macro = disp
        ai_output_share = (ai_services_rev + robot_services_value) / gdp
        gdp_growth = (p.base_gdp_growth
                      + p.productivity_passthrough * ai_output_share * 0.5
                      - p.transition_drag * new_disp
                      * (p.cognitive_wage_bill / gdp))
        gdp *= (1.0 + gdp_growth)

        out.append(YearState(
            year=year, compute_stock=compute_stock, algo_eff=algo_eff,
            ai_capex=ai_capex, binding=binding, scarcity=scarcity,
            ai_power_gw=used_power, power_additions_gw=power_additions,
            ai_hew_m=ai_hew, human_cog_workers_m=human_cog_m,
            cog_displacement=disp, cognitive_task_index=cognitive_task_index,
            robot_prod_m=robot_prod, robot_fleet_m=robot_fleet,
            robot_cost_k=robot_cost, phys_displacement=pd,
            pools=pools, profits=profits, gdp=gdp,
        ))

    return out


def summarize(states: list[YearState]) -> dict:
    """Condense a run into the investable facts."""
    bindings = {s.year: s.binding for s in states}
    def pool_path(name):
        return {s.year: round(s.pools[name], 3) for s in states}
    def profit_path(name):
        return {s.year: round(s.profits[name], 3) for s in states}
    last = states[-1]
    return {
        "binding_constraint_by_year": bindings,
        "ai_capex_path": {s.year: round(s.ai_capex, 3) for s in states},
        "cog_displacement_path": {s.year: round(s.cog_displacement, 3) for s in states},
        "phys_displacement_path": {s.year: round(s.phys_displacement, 3) for s in states},
        "robot_production_path_m": {s.year: round(s.robot_prod_m, 2) for s in states},
        "robot_cost_path_k": {s.year: round(s.robot_cost_k, 1) for s in states},
        "ai_power_gw_path": {s.year: round(s.ai_power_gw, 1) for s in states},
        "gdp_path": {s.year: round(s.gdp, 1) for s in states},
        "gdp_growth_path": {states[i].year: round(states[i].gdp / states[i - 1].gdp - 1.0, 4)
                            for i in range(1, len(states))},
        "pools": {k: pool_path(k) for k in states[0].pools},
        "profits": {k: profit_path(k) for k in states[0].profits},
        "terminal": {
            "year": last.year,
            "cognitive_displacement": round(last.cog_displacement, 3),
            "physical_displacement": round(last.phys_displacement, 3),
            "robot_fleet_m": round(last.robot_fleet_m, 1),
        },
    }


if __name__ == "__main__":
    import json
    s = simulate(Params())
    print(json.dumps(summarize(s), indent=2))
