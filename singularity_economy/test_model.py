"""
Tests for the singularity economy model.

Three layers:
  1. Sanity/invariant tests — no negative pools, ratchets hold, constraints
     actually cap growth.
  2. Calibration tests — 2026 outputs land near known 2026 actuals.
  3. Behavioral tests — the model responds to parameter changes in the
     economically required direction (comparative statics).
"""

import unittest

from model import Params, simulate, summarize
from scenarios import SCENARIOS, monte_carlo, _draw
import random


class TestInvariants(unittest.TestCase):
    def setUp(self):
        self.states = simulate(Params())

    def test_no_negative_values(self):
        for s in self.states:
            for k, v in s.pools.items():
                self.assertGreaterEqual(v, 0.0, f"{k} negative in {s.year}")
            self.assertGreaterEqual(s.ai_capex, 0.0)
            self.assertGreaterEqual(s.robot_fleet_m, 0.0)

    def test_displacement_is_monotonic_ratchet(self):
        prev = -1.0
        for s in self.states:
            self.assertGreaterEqual(s.cog_displacement, prev)
            prev = s.cog_displacement
        self.assertLessEqual(self.states[-1].cog_displacement, 1.0)

    def test_displacement_rate_capped(self):
        p = Params()
        prev = 0.0
        for s in self.states:
            self.assertLessEqual(s.cog_displacement - prev,
                                 p.max_displacement_rate + 1e-9)
            prev = s.cog_displacement

    def test_capex_respects_capital_cap(self):
        p = Params()
        prev_gdp = p.world_gdp
        for s in self.states:
            self.assertLessEqual(s.ai_capex, prev_gdp * p.capex_gdp_cap + 1e-9)
            prev_gdp = s.gdp

    def test_gdp_stays_positive_and_reasonable(self):
        for s in self.states:
            self.assertGreater(s.gdp, 50.0)
            self.assertLess(s.gdp, 400.0)

    def test_transition_drag_can_slow_gdp(self):
        calm = simulate(Params(transition_drag=0.0))
        drag = simulate(Params(transition_drag=1.0, max_displacement_rate=0.30))
        g_calm = min(calm[i].gdp / calm[i - 1].gdp for i in range(1, len(calm)))
        g_drag = min(drag[i].gdp / drag[i - 1].gdp for i in range(1, len(drag)))
        self.assertLess(g_drag, g_calm)

    def test_binding_constraint_is_reported(self):
        for s in self.states:
            self.assertIn(s.binding, ("power", "chips", "capital", "demand"))

    def test_robots_start_at_robotics_year(self):
        p = Params()
        for s in self.states:
            if s.year < p.robotics_year:
                self.assertEqual(s.robot_prod_m, 0.0)
            if s.year >= p.robotics_year:
                self.assertGreater(s.robot_prod_m, 0.0)

    def test_robot_cost_declines_to_floor(self):
        costs = [s.robot_cost_k for s in self.states if s.robot_prod_m > 0]
        self.assertTrue(all(b <= a + 1e-9 for a, b in zip(costs, costs[1:])))
        self.assertGreaterEqual(min(costs), 8.0)


class TestCalibration(unittest.TestCase):
    """2026 outputs should be within shouting distance of known actuals."""

    def setUp(self):
        self.s2026 = simulate(Params())[0]

    def test_2026_ai_capex_near_actual(self):
        # 2026 AI datacenter capex consensus ~ $0.35-0.50T
        self.assertGreater(self.s2026.ai_capex, 0.30)
        self.assertLess(self.s2026.ai_capex, 0.55)

    def test_2026_ai_power_plausible(self):
        # AI power consumption 2026 estimates ~ 50-90 GW
        self.assertGreater(self.s2026.ai_power_gw, 40)
        self.assertLess(self.s2026.ai_power_gw, 100)

    def test_2026_displacement_small(self):
        # visible-but-small labor impact in 2026
        self.assertLess(self.s2026.cog_displacement, 0.05)

    def test_2026_it_services_pool_near_actual(self):
        self.assertAlmostEqual(self.s2026.pools["it_services"], 1.55, delta=0.15)


class TestComparativeStatics(unittest.TestCase):
    """The model must respond in the economically required direction."""

    def test_delayed_singularity_delays_displacement(self):
        base = summarize(simulate(Params()))
        late = summarize(simulate(Params(singularity_year=2029)))
        self.assertGreater(base["cog_displacement_path"][2029],
                           late["cog_displacement_path"][2029])

    def test_no_singularity_means_little_displacement(self):
        fizzle = summarize(simulate(SCENARIOS["fizzle"]))
        self.assertLess(fizzle["cog_displacement_path"][2032], 0.15)

    def test_tighter_power_lowers_capex(self):
        base = simulate(Params())
        tight = simulate(Params(power_additions_growth_max=0.10))
        self.assertLess(sum(s.ai_capex for s in tight),
                        sum(s.ai_capex for s in base))

    def test_tight_power_binds_more_often(self):
        tight = simulate(Params(power_additions_growth_max=0.10,
                                chip_capacity_growth_max=0.75))
        bindings = [s.binding for s in tight]
        self.assertGreaterEqual(bindings.count("power"), 6)

    def test_tight_chips_bind_when_power_abundant(self):
        s = simulate(Params(power_additions_growth_max=0.60,
                            ai_power_2026=200.0, power_additions_2026=80.0,
                            chip_capacity_growth_max=0.25))
        bindings = [x.binding for x in s]
        self.assertIn("chips", bindings)

    def test_higher_beta_kills_casualty_faster(self):
        soft = simulate(Params(bpo_beta=0.5))
        hard = simulate(Params(bpo_beta=1.5))
        self.assertLess(hard[-1].pools["bpo"], soft[-1].pools["bpo"])

    def test_faster_components_mean_more_robots(self):
        slow = simulate(Params(component_capacity_growth=0.4))
        fast = simulate(Params(component_capacity_growth=1.3))
        self.assertGreater(fast[-1].robot_fleet_m, slow[-1].robot_fleet_m)

    def test_learning_rate_drives_cost_down(self):
        lo = simulate(Params(robot_learning_rate=0.10))
        hi = simulate(Params(robot_learning_rate=0.30))
        self.assertLess(hi[-1].robot_cost_k, lo[-1].robot_cost_k + 1e-9)


class TestMonteCarlo(unittest.TestCase):
    def test_runs_and_is_deterministic_by_seed(self):
        a = monte_carlo(n=40, seed=3)
        b = monte_carlo(n=40, seed=3)
        self.assertEqual(a, b)

    def test_draw_produces_valid_params(self):
        rng = random.Random(1)
        for _ in range(50):
            p = _draw(rng)
            states = simulate(p)
            self.assertEqual(len(states), p.end_year - p.start_year + 1)

    def test_binding_frequencies_sum_to_one(self):
        mc = monte_carlo(n=60, seed=5)
        for year, freqs in mc["binding_constraint_frequency"].items():
            self.assertAlmostEqual(sum(freqs.values()), 1.0, delta=0.02)


if __name__ == "__main__":
    unittest.main(verbosity=2)
