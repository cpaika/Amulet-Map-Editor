"""Cross-model robustness: conclusions that must hold on BOTH v1 and v2.

If one of these fails after a model change, the trade book's foundations
moved — that is a review event, not a test to silence.
"""

import unittest

from companies import COMPANIES
from scenarios_v2 import SCENARIOS_V2, scenario_states_v2
from valuation import evaluate


class TestCrossModelRobustness(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.states = scenario_states_v2()
        comps = {c.ticker: c for c in COMPANIES if c.ntm_earnings_b > 0}
        cls.rows = {t: evaluate(c, cls.states) for t, c in comps.items()}

    def test_scenarios_present(self):
        self.assertEqual(set(SCENARIOS_V2),
                         {"baseline", "fast_takeoff", "delayed", "friction",
                          "fizzle"})

    def test_core_semi_longs_positive(self):
        for t in ("TSM", "AVGO", "NVDA"):
            self.assertGreater(self.rows[t]["expected_upside"], 0.5, t)

    def test_power_complex_positive(self):
        for t in ("VST", "NRG", "CEG", "GEV"):
            self.assertGreater(self.rows[t]["expected_upside"], 0.5, t)

    def test_wage_linked_shorts_negative(self):
        for t in ("RHI", "ADP", "PAYX", "MAN", "CHRW", "LSTR", "TCS.NS"):
            self.assertLess(self.rows[t]["expected_upside"], -0.3, t)

    def test_robotics_longs_do_not_clear_hurdle_on_v2(self):
        """v2's endogenous robot ramp kills the robotics longs the red team
        killed — the two must keep agreeing."""
        for t in ("6268.T", "SYM", "MP"):
            self.assertLess(self.rows[t]["expected_upside"], 0.2, t)

    def test_fizzle_is_worst_for_beneficiaries(self):
        r = self.rows["TSM"]["per_scenario"]
        worst = min(r, key=lambda k: r[k]["upside"])
        self.assertEqual(worst, "fizzle")


if __name__ == "__main__":
    unittest.main(verbosity=2)
