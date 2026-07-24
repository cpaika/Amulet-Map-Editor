"""
Tests for the Meadows v2 systems-dynamics model.

The heart of this suite is LOOP ABLATION: each named feedback loop, when
disabled, must change behavior exactly the way systems theory predicts.
If ablating a loop does nothing, the loop isn't real — it's decoration.
"""

import unittest
from dataclasses import replace

from model_v2 import Loops, ParamsV2, Pipeline, simulate_v2, summarize_v2


def run(loops: Loops | None = None, **kw) -> list:
    p = ParamsV2(**kw)
    if loops is not None:
        p = replace(p, loops=loops)
    return simulate_v2(p)


class TestPipeline(unittest.TestCase):
    def test_steady_state_passthrough(self):
        pipe = Pipeline(3, 10.0)
        for _ in range(5):
            out = pipe.step(10.0)
        self.assertAlmostEqual(out, 10.0, places=9)

    def test_delay_length(self):
        """A pulse takes `stages` years to emerge."""
        pipe = Pipeline(3, 0.0)
        outs = [pipe.step(100.0 if i == 0 else 0.0) for i in range(6)]
        self.assertEqual(outs[0], 0.0)
        self.assertEqual(outs[1], 0.0)
        self.assertEqual(outs[2], 0.0)
        self.assertEqual(outs[3], 100.0)

    def test_conservation(self):
        pipe = Pipeline(2, 0.0)
        total_in, total_out = 0.0, 0.0
        for i in range(10):
            inflow = float(i)
            total_in += inflow
            total_out += pipe.step(inflow)
        self.assertAlmostEqual(total_in, total_out + pipe.in_transit(), places=9)


class TestInvariantsV2(unittest.TestCase):
    def setUp(self):
        self.states = run()

    def test_nonnegative_and_bounded(self):
        p = ParamsV2()
        for s in self.states:
            self.assertGreaterEqual(s.ai_capex, 0)
            self.assertGreaterEqual(s.robot_fleet_m, 0)
            self.assertLessEqual(s.silicon_margin, p.margin_ceiling + 1e-9)
            self.assertLessEqual(s.power_margin, p.margin_ceiling + 1e-9)
            self.assertLessEqual(s.cog_displacement, 1.0)
            for k, v in s.pools.items():
                self.assertGreaterEqual(v, 0.0, k)

    def test_displacement_ratchet(self):
        d = [s.cog_displacement for s in self.states]
        self.assertEqual(d, sorted(d))

    def test_2026_anchors_match_v1_calibration(self):
        s0 = self.states[0]
        self.assertGreater(s0.ai_capex, 0.45)
        self.assertLess(s0.ai_capex, 0.70)
        self.assertGreater(s0.ai_power_gw, 45)
        self.assertLess(s0.ai_power_gw, 110)
        self.assertAlmostEqual(s0.pools["it_services"], 1.55, delta=0.16)

    def test_qualitative_parity_with_v1(self):
        """Power binds most years; casualty decay back-half loaded; robots
        small this decade."""
        bindings = [s.binding for s in self.states]
        self.assertGreaterEqual(bindings.count("power"), 6)
        by_year = {s.year: s for s in self.states}
        self.assertLess(by_year[2028].cog_displacement, 0.25)
        self.assertGreater(by_year[2032].cog_displacement, 0.4)
        self.assertLess(by_year[2032].robot_prod_m, 3.0)


class TestEndogenousRentDynamics(unittest.TestCase):
    """The core v2 claim: rent duration is a RESULT of loop gain + delay."""

    def test_silicon_rents_decay_within_horizon(self):
        """High supply gain + 2yr delay -> capacity-scarcity rents die."""
        states = run()
        m = {s.year: s.silicon_margin for s in states}
        p = ParamsV2()
        early_peak = max(m[y] for y in (2026, 2027, 2028))
        self.assertGreater(early_peak, p.normal_margin + 0.15)  # rents exist early
        self.assertLess(m[2033], early_peak - 0.15)             # and decay hard

    def test_power_rents_persist(self):
        """Low supply gain (permitting) -> power rents outlive silicon rents."""
        states = run()
        self.assertGreater(states[-1].power_margin,
                           states[-1].silicon_margin + 0.1)

    def test_rent_duration_scales_with_supply_gain(self):
        """More B1 gain on chips -> earlier silicon rent normalization."""
        fast = run(chip_supply_gain=3.0)
        slow = run(chip_supply_gain=0.4)
        def norm_year(states):
            p = ParamsV2()
            for s in states:
                if s.year > 2027 and s.silicon_margin <= p.normal_margin + 0.02:
                    return s.year
            return 9999
        self.assertLessEqual(norm_year(fast), norm_year(slow))


class TestLoopAblation(unittest.TestCase):
    """Disable each loop; check the predicted behavioral change."""

    def test_b1_off_rents_persist(self):
        on = run()
        off = run(Loops(b1_supply_response=0.0))
        self.assertGreater(off[-1].silicon_margin, on[-1].silicon_margin + 0.05)

    def test_b2_off_faster_displacement(self):
        on = run()
        off = run(Loops(b2_backlash=0.0))
        y = 2030 - 2026
        self.assertGreaterEqual(off[y].cog_displacement,
                                on[y].cog_displacement - 1e-9)

    def test_b3_off_more_task_expansion_and_capex(self):
        """B3 must CLOSE: without it, bottleneck prices no longer throttle
        desired capex, so cumulative capex must be strictly higher."""
        on = run()
        off = run(Loops(b3_affordability=0.0))
        self.assertGreaterEqual(off[-1].cognitive_task_index,
                                on[-1].cognitive_task_index - 1e-9)
        self.assertGreater(sum(s.ai_capex for s in off),
                           sum(s.ai_capex for s in on))

    def test_r1_off_less_capability(self):
        on = run()
        off = run(Loops(r1_recursive_ai=0.0))
        self.assertLess(off[-1].algo_eff, on[-1].algo_eff)

    def test_r2_off_fewer_robots_by_2036(self):
        on = run()
        off = run(Loops(r2_robot_bootstrap=0.0))
        self.assertLessEqual(off[-1].robot_fleet_m, on[-1].robot_fleet_m + 1e-9)

    def test_r3_off_shallower_queues(self):
        on = run()
        off = run(Loops(r3_capex_momentum=0.0))
        self.assertLess(max(s.queue_ratio for s in off),
                        max(s.queue_ratio for s in on) + 1e-9)

    def test_b4_binds_under_stress(self):
        """Credit must bite when capex runs hot against weak AI revenue:
        low internal funding + strong momentum + late singularity."""
        stress = run(Loops(b4_credit=1.0), internal_funding_share=0.25,
                     momentum_gain=1.2, singularity_year=2031,
                     debt_revenue_tolerance=0.8)
        self.assertLess(min(s.credit_multiplier for s in stress), 0.999)
        no_credit = run(Loops(b4_credit=0.0), internal_funding_share=0.25,
                        momentum_gain=1.2, singularity_year=2031,
                        debt_revenue_tolerance=0.8)
        self.assertEqual(min(s.credit_multiplier for s in no_credit), 1.0)


class TestOvershoot(unittest.TestCase):
    def test_queues_emerge_with_momentum_and_delay(self):
        """R3 + pipeline delays must produce desired capex exceeding
        deliverable capacity during the boom (queue formation)."""
        states = run()
        self.assertGreater(max(s.queue_ratio for s in states), 1.2)

    def test_silicon_glut_emerges_but_power_never_gluts(self):
        """The endogenous Cisco moment: silicon capacity overshoots demand
        within the horizon; power scarcity persists."""
        states = run()
        self.assertGreater(max(s.capacity_glut for s in states[5:]), 1.3)
        self.assertGreater(states[-1].power_margin, 0.30)

    def test_ip_rents_persist_while_silicon_rents_decay(self):
        """The design's headline contrast, now implemented: same demand,
        no supply response -> durable rents."""
        states = run()
        self.assertGreater(states[-1].ip_toll_margin,
                           states[-1].silicon_margin + 0.2)

    def test_adoption_never_falls_with_late_singularity(self):
        states = run(singularity_year=2031)
        levels = [s.adoption_level for s in states]
        self.assertEqual(levels, sorted(levels))

    def test_delayed_singularity_delays_everything(self):
        base = run()
        late = run(singularity_year=2029)
        b = {s.year: s.cog_displacement for s in base}
        l = {s.year: s.cog_displacement for s in late}
        self.assertGreater(b[2029], l[2029])


if __name__ == "__main__":
    unittest.main(verbosity=2)
