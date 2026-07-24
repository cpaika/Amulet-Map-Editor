"""Tests for the valuation engine."""

import unittest

from model import Params, simulate
from scenarios import SCENARIOS
from valuation import (Company, DISCOUNT_RATE, HORIZON, SCENARIO_PROBS,
                       earnings_path, evaluate, evaluate_all, implied_cagr, pv)


class TestPV(unittest.TestCase):
    def test_pv_positive_and_orders(self):
        flat = [10.0] * HORIZON
        growing = [10.0 * 1.2 ** i for i in range(HORIZON)]
        self.assertGreater(pv(growing, 15), pv(flat, 15))

    def test_pv_terminal_multiple_matters(self):
        path = [10.0] * HORIZON
        self.assertGreater(pv(path, 20), pv(path, 10))


class TestImpliedCagr(unittest.TestCase):
    def test_roundtrip(self):
        """A price built from growth g should imply ~g back."""
        g = 0.12
        e0 = 10.0
        path = [e0 * (1 + g) ** (i + 1) for i in range(HORIZON)]
        mcap = pv(path, 15)
        self.assertAlmostEqual(implied_cagr(mcap, e0, 15), g, delta=0.005)

    def test_higher_price_implies_higher_growth(self):
        self.assertGreater(implied_cagr(500, 10, 15), implied_cagr(200, 10, 15))


class TestEarningsPath(unittest.TestCase):
    def setUp(self):
        self.states = simulate(Params())

    def test_pool_growth_flows_through(self):
        c = Company("X", "x", 100, 5, pools={"silicon": 1.0})
        path = earnings_path(c, self.states)
        self.assertGreater(path[-1], path[0])  # silicon pool grows in baseline

    def test_casualty_pool_shrinks_earnings(self):
        c = Company("Y", "y", 100, 5, pools={"bpo": 1.0}, pool_beta=1.0)
        path = earnings_path(c, self.states)
        self.assertLess(path[-1], path[0])

    def test_beta_amplifies(self):
        lo = Company("A", "a", 100, 5, pools={"silicon": 1.0}, pool_beta=0.5)
        hi = Company("B", "b", 100, 5, pools={"silicon": 1.0}, pool_beta=1.5)
        self.assertGreater(earnings_path(hi, self.states)[-1],
                           earnings_path(lo, self.states)[-1])

    def test_share_drift_compounds(self):
        gain = Company("G", "g", 100, 5, pools={"silicon": 1.0}, share_drift=0.05)
        lose = Company("L", "l", 100, 5, pools={"silicon": 1.0}, share_drift=-0.05)
        self.assertGreater(earnings_path(gain, self.states)[-1],
                           earnings_path(lose, self.states)[-1])


class TestEvaluate(unittest.TestCase):
    def test_probs_sum_to_one(self):
        self.assertAlmostEqual(sum(SCENARIO_PROBS.values()), 1.0)

    def test_casualty_short_worst_case_is_fizzle_or_friction(self):
        """For a casualty short, the best world for the COMPANY (highest fair
        value) must be a no/slow-singularity scenario — that's the short's risk."""
        scenario_states = {n: simulate(p) for n, p in SCENARIOS.items()}
        c = Company("IT", "outsourcer", 200, 8, pools={"it_services": 1.0},
                    pool_beta=1.2, terminal_multiple=12, stance="short")
        r = evaluate(c, scenario_states)
        best = max(r["per_scenario"], key=lambda k: r["per_scenario"][k]["upside"])
        self.assertIn(best, ("fizzle", "friction", "delayed"))

    def test_beneficiary_long_worst_case_is_fizzle(self):
        scenario_states = {n: simulate(p) for n, p in SCENARIOS.items()}
        c = Company("SI", "silicon", 4000, 110, pools={"silicon": 1.0},
                    pool_beta=1.1, terminal_multiple=18, stance="long")
        r = evaluate(c, scenario_states)
        worst = min(r["per_scenario"], key=lambda k: r["per_scenario"][k]["upside"])
        self.assertEqual(worst, "fizzle")

    def test_evaluate_all_ranks_by_expected_upside(self):
        rows = evaluate_all([
            Company("SI", "silicon", 4000, 110, pools={"silicon": 1.0},
                    pool_beta=1.1, terminal_multiple=18),
            Company("IT", "outsourcer", 200, 8, pools={"it_services": 1.0},
                    pool_beta=1.2, terminal_multiple=12),
        ])
        ups = [r["expected_upside"] for r in rows]
        self.assertEqual(ups, sorted(ups, reverse=True))


if __name__ == "__main__":
    unittest.main(verbosity=2)
