"""
Singularity Economy Model v2 — systems-dynamics core (Meadows formalization).

Design: meadows_design.md. Differences from v1:
  * Supply growth is ENDOGENOUS: sector margins above normal accelerate
    capacity investment through explicit construction-delay pipelines (B1).
  * Capex follows perceived (lagged) demand with herding -> overshoot (R3).
  * Credit conditions respond to debt accumulation (B4).
  * Adoption slows under displacement backlash (B2) and bottleneck-price
    affordability (B3).
  * Recursive AI saturates (R1); robot bootstrap raises component-capacity
    ceilings as the fleet grows (R2).

Every loop has a gain in `Loops` so tests can ablate it and check the
predicted behavioral change. Pure stdlib; written to port 1:1 to Rust
(fixed-size state, no dynamic dict dynamics in the hot loop).
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field, replace


# ----------------------------------------------------------------------------
# Parameters
# ----------------------------------------------------------------------------

@dataclass
class Loops:
    """Feedback-loop gains. 1.0 = calibrated strength, 0.0 = ablated."""
    b1_supply_response: float = 1.0
    b2_backlash: float = 1.0
    b3_affordability: float = 1.0
    r1_recursive_ai: float = 1.0
    r2_robot_bootstrap: float = 1.0
    r3_capex_momentum: float = 1.0
    b4_credit: float = 1.0


@dataclass
class ParamsV2:
    # timeline
    start_year: int = 2026
    end_year: int = 2036
    singularity_year: int = 2027
    robotics_year: int = 2028

    # macro baseline (2026)
    world_gdp: float = 115.0
    cognitive_wage_bill: float = 28.0
    physical_wage_bill: float = 34.0
    cognitive_workers_m: float = 950.0
    physical_workers_m: float = 2400.0
    base_gdp_growth: float = 0.03
    productivity_passthrough: float = 0.35
    transition_drag: float = 0.5

    # casualty pools (2026, $T)
    it_services_pool: float = 1.55
    bpo_pool: float = 0.36
    seat_saas_pool: float = 0.35
    prof_info_pool: float = 0.45
    it_services_beta: float = 0.9
    bpo_beta: float = 1.25
    saas_beta: float = 0.75
    prof_info_beta: float = 0.35

    # --- compute / silicon ---
    ai_capex_2026: float = 0.65          # $T desired capex 2026
    compute_deprec: float = 0.25
    silicon_share_of_capex: float = 0.55
    chip_capacity_2026: float = 0.28     # $T/yr pre-2026-delivery silicon capacity
                                         # (post-delivery ~0.36 = sold out vs 2026 demand)
    chip_base_growth: float = 0.30       # organic capacity growth at normal margins
    chip_supply_gain: float = 1.6        # B1 gain: extra growth per unit of excess margin
    chip_growth_ceiling: float = 0.85    # physical max yoy capacity growth
    chip_pipeline_stages: int = 2        # ~2yr fab/packaging response delay
    hw_cost_decline: float = 0.15        # $/unit-of-compute improvement per year

    # --- power ---
    ai_power_2026: float = 58.0
    power_additions_2026: float = 30.0   # GW/yr entering service 2026
    power_base_growth: float = 0.08      # organic growth of the addition rate
    power_supply_gain: float = 0.55      # B1 gain (permitting/turbines cap response)
    power_growth_ceiling: float = 0.40
    power_pipeline_stages: int = 3       # ~3yr order->energization
    gw_per_compute_unit: float = 55.0
    power_efficiency_gain: float = 0.12
    power_equip_cost_per_gw: float = 0.0035
    electricity_price_normal: float = 0.055   # $/kWh at normal utilization

    # --- AI cognitive supply / demand ---
    ai_hew_2026_m: float = 12.0
    algo_eff_growth_pre: float = 2.5
    singularity_boost: float = 2.0
    recursive_years: int = 3
    algo_eff_growth_post: float = 1.5
    algo_eff_cap: float = 3000.0         # R1 saturation level (multiplier on 2026)
    adoption_halflife: float = 1.6
    max_displacement_rate: float = 0.22
    backlash_gain: float = 2.0           # B2: friction per unit of recent displacement
    afford_gain: float = 0.4             # B3: demand slowdown per unit bottleneck price
    cognitive_demand_elasticity: float = 1.35
    ai_task_price_rel: float = 0.04
    addressable_cognitive: float = 0.85

    # --- robotics ---
    robot_prod_2028_m: float = 0.12
    component_capacity_2028: float = 0.14     # M units/yr supportable at ramp start
    component_base_growth: float = 0.45
    component_supply_gain: float = 2.2        # B1: components respond fast (China entry)
    component_growth_ceiling: float = 1.4
    component_pipeline_stages: int = 2
    bootstrap_gain: float = 0.10              # R2: extra ceiling per M robots in mfg
    robot_cost_2028_k: float = 50.0
    robot_learning_rate: float = 0.22
    robot_cost_floor_k: float = 8.0
    robot_hew: float = 1.4
    robot_attrition: float = 0.08
    physical_addressable: float = 0.65
    max_physical_displacement_rate: float = 0.15

    # --- capital / margins / credit ---
    capex_gdp_cap: float = 0.055
    internal_funding_share: float = 0.65      # share of capex funded from cash flow
    credit_gain: float = 1.2                  # B4: capital-cap tightening per leverage unit
    debt_revenue_tolerance: float = 1.5       # leverage level where spreads bite
    debt_amortization: float = 0.90           # prior-stock survival rate per year
    price_adjustment: float = 0.6             # yearly speed of margin adjustment
                                              # toward the scarcity target
                                              # (contracts/LTAs smooth prices)
    normal_margin: float = 0.22
    rent_margin_slope: float = 0.35           # margin rise per unit queue-excess
    margin_ceiling: float = 0.62              # best-in-history sustained EBIT margin
    target_utilization: float = 0.85
    ai_services_margin: float = 0.35

    # --- capex behavior (R3) ---
    perception_smoothing: float = 0.5         # 1yr info delay (exponential)
    momentum_gain: float = 0.5                # herding on perceived growth
    demand_growth_base: float = 0.32          # desired-capex growth pre-shock

    loops: Loops = field(default_factory=Loops)


CONSTRAINTS = ("power", "chips", "capital", "demand")


# ----------------------------------------------------------------------------
# Pipeline: N-stage material delay (Meadows: stocks in transit)
# ----------------------------------------------------------------------------

class Pipeline:
    """N first-order stages in series; total mean delay = stages years when
    each stage drains 100%/yr at steady state. Conserves material."""

    def __init__(self, stages: int, initial_flow: float):
        # initialize at steady state carrying `initial_flow` per year
        self.stages = [initial_flow] * stages

    def step(self, inflow: float) -> float:
        """Advance one year; returns delivered outflow."""
        carry = inflow
        for i in range(len(self.stages)):
            out = self.stages[i]           # each stage empties over ~1 year
            self.stages[i] = carry
            carry = out
        return carry

    def in_transit(self) -> float:
        return sum(self.stages)


def logistic(x: float) -> float:
    return 1.0 / (1.0 + math.exp(-x))


# ----------------------------------------------------------------------------
# Year state
# ----------------------------------------------------------------------------

@dataclass
class YearV2:
    year: int
    binding: str
    ai_capex: float
    compute_stock: float
    algo_eff: float
    ai_power_gw: float
    power_utilization: float
    chip_utilization: float
    silicon_margin: float
    power_margin: float
    component_margin: float
    electricity_price: float
    ai_hew_m: float
    cog_displacement: float
    cognitive_task_index: float
    adoption_friction: float
    adoption_level: float
    robot_prod_m: float
    robot_fleet_m: float
    robot_cost_k: float
    component_capacity_m: float
    phys_displacement: float
    sector_debt: float
    credit_multiplier: float
    perceived_growth: float
    queue_ratio: float              # desired capex vs deliverable (scarcity depth)
    capacity_glut: float            # silicon capacity vs demand (>1 = glut)
    ip_toll_margin: float
    gdp: float
    pools: dict
    profits: dict


# ----------------------------------------------------------------------------
# Simulation
# ----------------------------------------------------------------------------

def simulate_v2(p: ParamsV2) -> list[YearV2]:
    L = p.loops
    years = range(p.start_year, p.end_year + 1)

    # stocks
    compute_stock = 1.0
    algo_eff = 1.0
    ai_power = p.ai_power_2026
    chip_capacity = p.chip_capacity_2026
    component_capacity = 0.0
    robot_fleet = 0.0
    cum_robots = 0.02
    robot_cost = p.robot_cost_2028_k
    human_cog_m = p.cognitive_workers_m
    phys_workers_m = p.physical_workers_m
    gdp = p.world_gdp
    sector_debt = 0.0
    perceived_growth = p.demand_growth_base
    last_capex = p.ai_capex_2026 * 0.8
    prev_disp = 0.0
    prev_disp_macro = 0.0
    gw_per_unit = p.gw_per_compute_unit

    # pipelines (material delays)
    power_pipe = Pipeline(p.power_pipeline_stages, p.power_additions_2026)
    chip_pipe = Pipeline(p.chip_pipeline_stages,
                         chip_capacity * p.chip_base_growth)
    ip_capacity = chip_capacity * 0.18   # litho/EDA/IP slice of silicon flow
    ip_pipe = Pipeline(p.chip_pipeline_stages, ip_capacity * p.chip_base_growth)
    comp_pipe = Pipeline(p.component_pipeline_stages, 0.0)
    prev_adopt = 0.08

    # margins start at normal
    silicon_margin = p.normal_margin + 0.10
    ip_toll_margin = p.normal_margin + 0.15
    power_margin = p.normal_margin + 0.08
    component_margin = p.normal_margin

    out: list[YearV2] = []

    for year in years:
        t_sing = year - p.singularity_year

        # ------------- R1: recursive AI (saturating) -------------
        if year > p.start_year:
            if year <= p.singularity_year:
                g = p.algo_eff_growth_pre
            elif t_sing <= p.recursive_years:
                boost = 1.0 + (p.singularity_boost - 1.0) * L.r1_recursive_ai
                g = p.algo_eff_growth_pre * boost
            else:
                g = p.algo_eff_growth_post
            # saturation: growth fades as algo_eff approaches cap
            headroom = max(1.0 - algo_eff / p.algo_eff_cap, 0.0)
            algo_eff *= 1.0 + (g - 1.0) * headroom

        # ------------- demand side: adoption with B2 + B3 -------------
        if len(out) >= 2:
            recent_disp_rate = max(out[-1].cog_displacement
                                   - out[-2].cog_displacement, 0.0)
        elif out:
            recent_disp_rate = out[-1].cog_displacement
        else:
            recent_disp_rate = 0.0
        friction = 1.0 + L.b2_backlash * p.backlash_gain * recent_disp_rate
        # bottleneck price index (from last year's utilizations)
        if out:
            btl_price = max(out[-1].power_utilization, out[-1].chip_utilization)
            btl_price = max(btl_price - p.target_utilization, 0.0) / \
                (1.0 - p.target_utilization)
        else:
            btl_price = 0.4
        afford = 1.0 + L.b3_affordability * p.afford_gain * btl_price
        pre_ramp = min(0.08 * (1.6 ** (year - p.start_year)), 0.28)
        if t_sing < 0:
            adopt = pre_ramp
        else:
            k = math.log(3.0) / (p.adoption_halflife * friction)
            adopt = max(logistic(k * (t_sing - p.adoption_halflife * friction)),
                        pre_ramp, 0.10)

        ai_hew_raw = p.ai_hew_2026_m * compute_stock * algo_eff
        price_ratio = (p.ai_task_price_rel * afford) if t_sing >= 0 else 0.25
        task_expansion = min(price_ratio ** (-(p.cognitive_demand_elasticity - 1.0)
                                             * 0.35), 6.0)
        cognitive_task_index = 1.0 + (task_expansion - 1.0) * adopt
        addressable = p.cognitive_workers_m * p.addressable_cognitive
        demand_hew = addressable * adopt * cognitive_task_index
        ai_hew = min(ai_hew_raw, demand_hew)

        disp_target = min(ai_hew / max(cognitive_task_index, 1e-9), addressable) \
            / p.cognitive_workers_m
        disp = min(disp_target * min(adopt * 1.4, 1.0),
                   prev_disp + p.max_displacement_rate / friction)
        disp = max(disp, prev_disp)
        prev_disp = disp
        human_cog_m = p.cognitive_workers_m * (1.0 - disp)

        # ------------- R3: capex desire from perceived demand -------------
        demand_signal_growth = p.demand_growth_base
        if t_sing >= 0:
            demand_signal_growth += 1.6 * max(adopt - prev_adopt, 0.0) \
                + 0.5 * adopt
        prev_adopt = adopt
        perceived_growth += p.perception_smoothing * \
            (demand_signal_growth - perceived_growth)
        herd = L.r3_capex_momentum * p.momentum_gain * max(perceived_growth, 0.0)
        # B3 closure (review fix 1): bottleneck prices raise the effective
        # cost of AI capacity and throttle desired capex growth.
        desired_capex = last_capex * (1.0 + (perceived_growth + herd) / afford)

        # ------------- B4: credit conditions -------------
        ai_revenue_proxy = max((out[-1].pools["ai_services"] if out else 0.02)
                               + 0.25 * last_capex, 0.05)
        leverage = sector_debt / max(ai_revenue_proxy, 1e-9)
        credit_mult = 1.0 / (1.0 + L.b4_credit * p.credit_gain *
                             max(leverage - p.debt_revenue_tolerance, 0.0))

        # ------------- constraints -------------
        hw_cost_index = (1.0 - p.hw_cost_decline) ** (year - p.start_year)
        cost_per_unit = (p.ai_capex_2026 / 0.80) * hw_cost_index

        # Fix 6 (review): deliver chip capacity at the START of the year so
        # chips and power use the same same-year-delivery convention.
        excess_margin = max(silicon_margin - p.normal_margin, 0.0)
        chip_growth = min(p.chip_base_growth
                          + L.b1_supply_response * p.chip_supply_gain * excess_margin,
                          p.chip_growth_ceiling)
        chip_capacity += chip_pipe.step(chip_capacity * chip_growth)
        # IP-moat toll capacity (Fix 12): same demand, NO supply response —
        # the monopoly's capacity grows at base rate only. Rent persistence
        # vs the silicon track is the design's headline contrast.
        ip_capacity += ip_pipe.step(ip_capacity * p.chip_base_growth)

        chips_cap = chip_capacity / p.silicon_share_of_capex
        if year > p.start_year:   # Fix 3: decay starts after the anchor year
            gw_per_unit *= (1.0 - p.power_efficiency_gain)
        power_additions = power_pipe.step(_power_orders(p, L, power_margin,
                                                        perceived_growth,
                                                        power_pipe))
        power_headroom = max(ai_power + power_additions
                             - compute_stock * gw_per_unit, 0.0)
        power_cap = (power_headroom / gw_per_unit) * cost_per_unit
        capital_cap = gdp * p.capex_gdp_cap * credit_mult

        caps = {"chips": chips_cap, "power": power_cap,
                "capital": capital_cap, "demand": desired_capex}
        ai_capex = min(caps.values())
        binding = min(caps, key=lambda k: caps[k])
        queue_ratio = desired_capex / max(min(chips_cap, power_cap, capital_cap),
                                          1e-9)
        # capacity overshoot per design: delivered silicon capacity vs the
        # demand actually flowing through it (>1 = glut, the "Cisco moment")
        capacity_glut = chip_capacity / max(desired_capex
                                            * p.silicon_share_of_capex, 1e-9)

        # debt accumulates on externally funded capex
        sector_debt = sector_debt * p.debt_amortization \
            + max(ai_capex * (1.0 - p.internal_funding_share), 0.0)

        # compute stock update
        pre_stock = compute_stock
        units_added = ai_capex / cost_per_unit
        compute_stock = compute_stock * (1.0 - p.compute_deprec) + units_added
        ai_power = ai_power + power_additions
        used_power = min(compute_stock * gw_per_unit, ai_power)

        # ------------- utilizations, prices, margins -------------
        # Scarcity is a queue phenomenon: measure desired demand against
        # capacity. Realized capex is capped by the constraint, so measuring
        # realized/capacity would hide the very scarcity that sets prices.
        chip_utilization = min(desired_capex * p.silicon_share_of_capex
                               / max(chip_capacity, 1e-9), 1.35)
        ip_utilization = min(desired_capex * p.silicon_share_of_capex
                             / max(ip_capacity, 1e-9), 1.35)
        # demand = surviving pre-update stock + FULL desired additions (queued
        # demand); supply = energized capacity after this year's additions.
        power_demand_gw = pre_stock * (1.0 - p.compute_deprec) * gw_per_unit \
            + (desired_capex / cost_per_unit) * gw_per_unit
        power_utilization = min(power_demand_gw / max(ai_power, 1e-9), 1.35)

        def margin_from(u: float, gain_class: float) -> float:
            excess = max(u - p.target_utilization, 0.0) / (1.0 - p.target_utilization)
            return min(p.normal_margin
                       + p.rent_margin_slope * min(excess, 2.0) * gain_class,
                       p.margin_ceiling)

        pa = p.price_adjustment
        silicon_margin += pa * (margin_from(chip_utilization, 0.9) - silicon_margin)
        ip_toll_margin += pa * (margin_from(ip_utilization, 0.9) - ip_toll_margin)
        power_margin += pa * (margin_from(power_utilization, 1.0) - power_margin)
        electricity_price = p.electricity_price_normal * \
            (1.0 + 1.2 * min(max(power_utilization - p.target_utilization, 0.0)
                             / (1.0 - p.target_utilization), 2.0))

        # ------------- robotics: components pipeline + R2 bootstrap -------------
        robot_prod = 0.0
        if year >= p.robotics_year:
            if component_capacity == 0.0:
                component_capacity = p.component_capacity_2028
            comp_excess = max(component_margin - p.normal_margin, 0.0)
            bootstrap = L.r2_robot_bootstrap * p.bootstrap_gain * \
                min(robot_fleet * 0.3, 10.0)   # share of fleet in manufacturing
            comp_ceiling = p.component_growth_ceiling + bootstrap
            comp_growth = min(p.component_base_growth
                              + L.b1_supply_response * p.component_supply_gain
                              * comp_excess + bootstrap * 0.2,
                              comp_ceiling)
            component_capacity += comp_pipe.step(component_capacity * comp_growth)

            # economic demand: desired fleet grows with payback quality and a
            # deployment-adoption ramp; production = min(demand, capacity).
            avg_phys_wage_k = p.physical_wage_bill / p.physical_workers_m * 1e3
            payback_years = robot_cost / max(avg_phys_wage_k * p.robot_hew, 1e-9)
            econ_pull = max(min(2.0 / max(payback_years, 0.25), 3.0), 0.0)
            t_rob = year - p.robotics_year
            deploy_ramp = logistic(0.9 * (t_rob - 4.0))  # integration takes years
            fleet_target = p.physical_workers_m * p.physical_addressable \
                * deploy_ramp * min(econ_pull / 2.0, 1.0)
            robot_demand = max(fleet_target - robot_fleet * (1.0 - p.robot_attrition),
                               p.robot_prod_2028_m * 0.5)
            robot_prod = min(robot_demand, component_capacity)
            comp_utilization = robot_demand / max(component_capacity, 1e-9)
            component_margin += p.price_adjustment * \
                (margin_from(min(comp_utilization, 1.35), 0.8) - component_margin)

            cum_robots += robot_prod
            doublings = math.log2(max(cum_robots / 0.06, 1.0))
            robot_cost = max(p.robot_cost_2028_k *
                             (1.0 - p.robot_learning_rate) ** doublings,
                             p.robot_cost_floor_k)
            robot_fleet = robot_fleet * (1.0 - p.robot_attrition) + robot_prod

        phys_hew = robot_fleet * p.robot_hew
        pd_target = min(phys_hew / p.physical_workers_m, p.physical_addressable)
        prev_pd = 1.0 - phys_workers_m / p.physical_workers_m
        pd = max(min(pd_target, prev_pd + p.max_physical_displacement_rate), prev_pd)
        phys_workers_m = p.physical_workers_m * (1.0 - pd)

        # ------------- pools & profits -------------
        avg_cog_wage = p.cognitive_wage_bill / p.cognitive_workers_m
        avg_phys_wage = p.physical_wage_bill / p.physical_workers_m
        displaced_value = disp * p.cognitive_workers_m * avg_cog_wage
        ai_services = displaced_value * 0.45 + \
            (cognitive_task_index - 1.0) * p.cognitive_wage_bill * 0.06

        def casualty(pool0, beta, drift):
            organic = pool0 * ((1.0 + drift) ** (year - p.start_year))
            return organic * max(1.0 - beta * disp, 0.05)

        pools = {
            "ai_services": ai_services,
            "silicon": ai_capex * p.silicon_share_of_capex,
            "ip_tolls": ai_capex * 0.10,
            "dc_infra": ai_capex * (1.0 - p.silicon_share_of_capex),
            "power_equipment": p.power_equip_cost_per_gw * power_additions,
            "electricity": used_power * 8760 * electricity_price / 1e6,
            "robots": robot_prod * robot_cost / 1e3,
            "robot_components": robot_prod * robot_cost / 1e3 * 0.55,
            "robot_services": pd * p.physical_workers_m * avg_phys_wage * 0.35,
            "it_services": casualty(p.it_services_pool, p.it_services_beta, 0.04),
            "bpo": casualty(p.bpo_pool, p.bpo_beta, 0.03),
            "seat_saas": casualty(p.seat_saas_pool, p.saas_beta, 0.08),
            "prof_info": casualty(p.prof_info_pool, p.prof_info_beta, 0.05),
            "human_cognitive_wages": human_cog_m * avg_cog_wage,
            "human_physical_wages": phys_workers_m * avg_phys_wage,
            "gdp_index": gdp,
        }
        margins = {"silicon": silicon_margin, "ip_tolls": ip_toll_margin,
                   "power_equipment": power_margin,
                   "robot_components": component_margin,
                   "electricity": min(0.30 + 0.5 * (electricity_price
                                                    / p.electricity_price_normal - 1.0),
                                      0.6),
                   "ai_services": p.ai_services_margin}
        profits = {k: v * margins.get(k, p.normal_margin)
                   for k, v in pools.items()
                   if not k.startswith("human_") and k != "gdp_index"}

        # ------------- macro -------------
        new_disp = max(disp - prev_disp_macro, 0.0)
        prev_disp_macro = disp
        ai_share = (ai_services + pools["robot_services"]) / gdp
        gdp *= 1.0 + (p.base_gdp_growth
                      + p.productivity_passthrough * ai_share * 0.5
                      - p.transition_drag * new_disp
                      * (p.cognitive_wage_bill / gdp))

        last_capex = max(ai_capex, 1e-6)

        out.append(YearV2(
            year=year, binding=binding, ai_capex=ai_capex,
            compute_stock=compute_stock, algo_eff=algo_eff,
            ai_power_gw=used_power, power_utilization=power_utilization,
            chip_utilization=chip_utilization, silicon_margin=silicon_margin,
            power_margin=power_margin, component_margin=component_margin,
            electricity_price=electricity_price, ai_hew_m=ai_hew,
            cog_displacement=disp, cognitive_task_index=cognitive_task_index,
            adoption_friction=friction, adoption_level=adopt, robot_prod_m=robot_prod,
            robot_fleet_m=robot_fleet, robot_cost_k=robot_cost,
            component_capacity_m=component_capacity, phys_displacement=pd,
            sector_debt=sector_debt, credit_multiplier=credit_mult,
            perceived_growth=perceived_growth, queue_ratio=queue_ratio, capacity_glut=capacity_glut,
            ip_toll_margin=ip_toll_margin,
            gdp=gdp, pools=pools, profits=profits,
        ))

    return out


def _power_orders(p: ParamsV2, L: Loops, power_margin: float,
                  perceived_growth: float, pipe: Pipeline) -> float:
    """GW/yr of new orders entering the power pipeline: organic growth of the
    ordering rate plus B1 response to power scarcity rents, capped by the
    turbine/transformer manufacturing ceiling."""
    current_rate = pipe.stages[0] if pipe.stages else p.power_additions_2026
    excess = max(power_margin - p.normal_margin, 0.0)
    growth = min(p.power_base_growth
                 + L.b1_supply_response * p.power_supply_gain * excess
                 + 0.3 * max(perceived_growth - 0.3, 0.0),
                 p.power_growth_ceiling)
    return current_rate * (1.0 + growth)


# ----------------------------------------------------------------------------
# Summaries
# ----------------------------------------------------------------------------

def summarize_v2(states: list[YearV2]) -> dict:
    def path(fn):
        return {s.year: round(fn(s), 3) for s in states}
    # rent peaks: year of max margin per rent sector
    def peak_year(fn):
        return max(states, key=fn).year
    return {
        "binding_by_year": {s.year: s.binding for s in states},
        "ai_capex_path": path(lambda s: s.ai_capex),
        "cog_displacement_path": path(lambda s: s.cog_displacement),
        "phys_displacement_path": path(lambda s: s.phys_displacement),
        "robot_prod_path_m": path(lambda s: s.robot_prod_m),
        "silicon_margin_path": path(lambda s: s.silicon_margin),
        "power_margin_path": path(lambda s: s.power_margin),
        "component_margin_path": path(lambda s: s.component_margin),
        "queue_ratio_path": path(lambda s: s.queue_ratio),
        "capacity_glut_path": path(lambda s: s.capacity_glut),
        "ip_toll_margin_path": path(lambda s: s.ip_toll_margin),
        "credit_multiplier_path": path(lambda s: s.credit_multiplier),
        "gdp_path": path(lambda s: s.gdp),
        "rent_peaks": {
            "silicon": peak_year(lambda s: s.silicon_margin),
            "power": peak_year(lambda s: s.power_margin),
            "components": peak_year(lambda s: s.component_margin),
        },
        "pools": {k: {s.year: round(s.pools[k], 3) for s in states}
                  for k in states[0].pools},
    }


if __name__ == "__main__":
    import json
    print(json.dumps(summarize_v2(simulate_v2(ParamsV2())), indent=2))
