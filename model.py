#!/usr/bin/env python3
"""Anthropic equity — MA vs NH relocation state-tax model (v2).

Grounded in primary sources gathered 2026-08-10:
  * Offer letter (Docusign, 2026-03-23): $3,795,000 equity value / $259.1364
    Series G preferred price -> 14,645 RSUs. Vesting Commencement Date
    2026-05-20 (start 2026-04-21 falls in the Apr 6 - Jul 5 range).
    1/8 vests at the 6-month cliff (2026-11-20), then 1/16 each quarter.
    Event-based condition: IPO or Change of Control within 7 years of grant.
  * Anthropic "Equity Comp Illustrator" (Calvin, 2026-03-23): ~1,467M fully
    diluted shares at the $380B Series G valuation. Donation match 1:1 up to
    25% of grant.
  * Public reporting: Series H-1 May 2026 $965B post (~$65B raised);
    confidential S-1 filed 2026-06-01, targeting Oct 2026 Nasdaq IPO
    (~$60B primary assumed).
  * Google Calendar Apr-Aug 2026: ~15% of workdays spent in SF (onboarding
    2 weeks + ~1 week/quarter cadence) -> those are NOT MA workdays for
    nonresident apportionment purposes.
  * Closed on 6 Trowbridge Cir, Shrewsbury MA 2026-08-03 (permanent place
    of abode -> statutory residency trap if >183 MA days in any year).

Outputs STATE tax deltas only (federal is invariant across scenarios,
except the DAF module which estimates the federal deduction).
NOT tax advice — pressure-test with a SALT CPA/attorney.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass, field, replace
from datetime import date, timedelta

# ----------------------------------------------------------------------------
# Grant facts (offer letter + illustrator)
# ----------------------------------------------------------------------------

GRANT_VALUE = 3_795_000.0
SERIES_G_PREFERRED = 259.1364
TOTAL_RSUS = 14_645
GRANT_DATE = date(2026, 4, 21)          # ~start date; board approval may differ
VESTING_COMMENCEMENT = date(2026, 5, 20)
BASE_SALARY = 405_000.0

# Capitalization (dilution modeled from real share counts)
FD_SHARES_G = 380e9 / SERIES_G_PREFERRED          # ~1,466.4M (illustrator: 1,467M)
SERIES_H1_DATE = date(2026, 5, 15)
SERIES_H1_POST = 965e9
SERIES_H1_PRIMARY = 65e9

# MA tax
MA_RATE = 0.09          # 5% Part B + 4% Fair Share surtax; every scenario year
                        # the marginal equity income sits above the ~$1.11M
                        # (inflation-indexed) surtax threshold.

WORKDAYS_PER_YEAR = 250.0

# Fraction of pre-move workdays actually worked outside MA (SF trips).
# Calendar-derived: Apr 21 - Aug 10 2026 shows 14 of ~78 workdays in SF (~18%);
# ongoing cadence looks like ~1 week per 6-7 weeks -> default 15%.
SF_WORKDAY_FRACTION = 0.15


@dataclass
class ValuationPath:
    """Post-money valuation waypoints, linearly interpolated, flat outside."""
    name: str
    ipo_date: date
    ipo_post: float           # post-money at IPO, includes primary
    ipo_primary: float
    waypoints: list           # [(date, valuation)] after IPO
    post_ipo_dilution: float = 0.02   # annual employee-pool dilution

    def valuation(self, d: date) -> float:
        pts = [(SERIES_H1_DATE, SERIES_H1_POST), (self.ipo_date, self.ipo_post)]
        pts += self.waypoints
        if d <= pts[0][0]:
            return pts[0][1]
        for (d0, v0), (d1, v1) in zip(pts, pts[1:]):
            if d <= d1:
                f = (d - d0).days / (d1 - d0).days
                return v0 + f * (v1 - v0)
        return pts[-1][1]

    def fd_shares(self, d: date) -> float:
        """Fully diluted shares, stepping up at each primary issuance."""
        shares = FD_SHARES_G
        if d >= SERIES_H1_DATE:
            h1_price = (SERIES_H1_POST - SERIES_H1_PRIMARY) / shares
            shares += SERIES_H1_PRIMARY / h1_price
        if d >= self.ipo_date:
            ipo_price = (self.ipo_post - self.ipo_primary) / shares
            shares += self.ipo_primary / ipo_price
            years = (d - self.ipo_date).days / 365.25
            shares *= (1 + self.post_ipo_dilution) ** years
        return shares

    def price(self, d: date) -> float:
        return self.valuation(d) / self.fd_shares(d)


BASE_PATH = ValuationPath(
    name="base ($1T IPO -> $10T mid-27 -> $30T end-28)",
    ipo_date=date(2026, 10, 15), ipo_post=1_000e9, ipo_primary=60e9,
    waypoints=[(date(2027, 7, 1), 10_000e9), (date(2028, 12, 31), 30_000e9)],
)
BEAR_PATH = ValuationPath(
    name="bear ($800B IPO -> $1.5T -> $3T)",
    ipo_date=date(2026, 10, 15), ipo_post=800e9, ipo_primary=60e9,
    waypoints=[(date(2027, 7, 1), 1_500e9), (date(2028, 12, 31), 3_000e9)],
)
BULL_PATH = ValuationPath(
    name="bull ($1.2T IPO -> $15T -> $50T)",
    ipo_date=date(2026, 10, 15), ipo_post=1_200e9, ipo_primary=60e9,
    waypoints=[(date(2027, 7, 1), 15_000e9), (date(2028, 12, 31), 50_000e9)],
)
PATHS = {"base": BASE_PATH, "bear": BEAR_PATH, "bull": BULL_PATH}


# ----------------------------------------------------------------------------
# Vesting schedule: 2/16 cliff 2026-11-20, then 1/16 quarterly to 2030-05-20
# ----------------------------------------------------------------------------

def vest_schedule() -> list:
    """[(vest_date, shares)] — fractional shares kept for accuracy."""
    per_16 = TOTAL_RSUS / 16.0
    months = [(11, 2026), (2, 2027), (5, 2027), (8, 2027), (11, 2027),
              (2, 2028), (5, 2028), (8, 2028), (11, 2028),
              (2, 2029), (5, 2029), (8, 2029), (11, 2029),
              (2, 2030), (5, 2030)]
    sched = [(date(2026, 11, 20), 2 * per_16)]
    sched += [(date(y, m, 20), per_16) for (m, y) in months[1:]]
    assert abs(sum(s for _, s in sched) - TOTAL_RSUS) < 1e-6
    return sched


def weekdays_between(d0: date, d1: date) -> int:
    """Weekday count in [d0, d1) — holiday-free approximation."""
    if d1 <= d0:
        return 0
    days = (d1 - d0).days
    full_weeks, rem = divmod(days, 7)
    count = full_weeks * 5
    for i in range(rem):
        if (d0 + timedelta(days=full_weeks * 7 + i)).weekday() < 5:
            count += 1
    return count


# ----------------------------------------------------------------------------
# Scenario engine
# ----------------------------------------------------------------------------

@dataclass
class Scenario:
    name: str
    move_date: date | None                 # None = never leaves MA
    ma_workdays_per_year: float = 0.0      # MA workdays after the move
    sf_fraction: float = SF_WORKDAY_FRACTION
    path: ValuationPath = field(default_factory=lambda: BASE_PATH)


@dataclass
class TrancheResult:
    vest: date
    recog: date
    shares: float
    price: float
    income: float
    ma_fraction: float
    ma_tax: float


def ma_source_fraction(sc: Scenario, vest: date) -> float:
    """Nonresident apportionment: MA workdays / total workdays over
    [grant date, tranche vest date].  Pre-move workdays are MA except the
    SF-travel share; post-move MA workdays come from deliberate visits."""
    total = weekdays_between(GRANT_DATE, vest)
    if total == 0:
        return 1.0
    boundary = min(vest, sc.move_date) if sc.move_date else vest
    pre = weekdays_between(GRANT_DATE, boundary)
    post = total - pre
    ma_rate_post = min(sc.ma_workdays_per_year / WORKDAYS_PER_YEAR, 1.0)
    ma_days = pre * (1 - sc.sf_fraction) + post * ma_rate_post
    return ma_days / total


def run_scenario(sc: Scenario) -> list:
    results = []
    for vest, shares in vest_schedule():
        recog = max(vest, sc.path.ipo_date)   # double-trigger settlement
        px = sc.path.price(recog)
        income = shares * px
        resident = sc.move_date is None or recog < sc.move_date
        frac = 1.0 if resident else ma_source_fraction(sc, vest)
        results.append(TrancheResult(vest, recog, shares, px, income, frac,
                                     income * frac * MA_RATE))
    return results


def vest_tax(sc: Scenario) -> float:
    return sum(t.ma_tax for t in run_scenario(sc))


# ----------------------------------------------------------------------------
# Capital-gains layer: post-vest appreciation is intangible income taxed by
# the state of residence at sale.  NH resident at sale -> 0% state.
# ----------------------------------------------------------------------------

def capital_gains_layer(sc: Scenario, sale_date: date,
                        daf_shares: float = 0.0) -> tuple:
    """(total_gain, ma_tax_if_MA_resident_at_sale).  Assumes all shares vested
    on/before sale_date are held to sale_date (no sell-to-cover modeled) minus
    shares earmarked for the DAF."""
    sale_px = sc.path.price(sale_date)
    gain = 0.0
    remaining_daf = daf_shares
    for t in run_scenario(sc):
        if t.recog > sale_date:
            continue
        sh = t.shares
        donate = min(remaining_daf, sh)      # donate lowest-basis (earliest) lots
        remaining_daf -= donate
        sh -= donate
        gain += sh * max(sale_px - t.price, 0.0)
    return gain, gain * MA_RATE


# ----------------------------------------------------------------------------
# DAF module: 5% pledge (~732 shares), 1:1 company match.
# Deduction = FMV at transfer if held >1yr post-vest (30%-of-AGI limit,
# 5-yr carryforward).  Federal value only; MA charitable deduction is at the
# 5% rate and moot if NH resident.
# ----------------------------------------------------------------------------

def daf_estimate(sc: Scenario, transfer_date: date, fed_rate: float = 0.37):
    shares = 0.05 * TOTAL_RSUS
    px = sc.path.price(transfer_date)
    fmv = shares * px
    return {"shares": shares, "price": px, "fmv": fmv,
            "fed_deduction_value": fmv * fed_rate,
            "match_charitable_value": fmv}   # 1:1 match, not a deduction


# ----------------------------------------------------------------------------
# Report
# ----------------------------------------------------------------------------

def money(x: float) -> str:
    return f"${x:,.0f}"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--path", choices=PATHS, default="base")
    ap.add_argument("--sale-date", default="2029-06-30",
                    help="cap-gains sale date (post-lockup), YYYY-MM-DD")
    ap.add_argument("--detail", action="store_true",
                    help="print per-tranche table for each scenario")
    args = ap.parse_args()
    path = PATHS[args.path]
    sale_date = date.fromisoformat(args.sale_date)

    scenarios = [
        Scenario("Stay in MA", None, path=path),
        Scenario("Move Oct 1 2026", date(2026, 10, 1), path=path),
        Scenario("Move Oct 1 2026 + 20 MA wd/yr", date(2026, 10, 1), 20, path=path),
        Scenario("Move Jan 1 2027", date(2027, 1, 1), path=path),
        Scenario("Move Apr 1 2027", date(2027, 4, 1), path=path),
        Scenario("Move Jul 1 2027 (family pref)", date(2027, 7, 1), path=path),
        Scenario("Move Jul 1 2027 + 20 MA wd/yr", date(2027, 7, 1), 20, path=path),
    ]

    print(f"Valuation path: {path.name}")
    print(f"IPO {path.ipo_date}  |  cliff recognition "
          f"{max(date(2026, 11, 20), path.ipo_date)}  |  "
          f"price at cliff {money(path.price(date(2026, 11, 20)))}")
    print()
    print(f"{'Scenario':38s} {'MA tax on vests':>16s} {'vs stay':>14s}")
    stay = vest_tax(scenarios[0])
    for sc in scenarios:
        tax = vest_tax(sc)
        print(f"{sc.name:38s} {money(tax):>16s} {money(stay - tax):>14s}")
        if args.detail:
            for t in run_scenario(sc):
                print(f"    vest {t.vest}  recog {t.recog}  "
                      f"{t.shares:8.1f} sh @ {money(t.price):>10s}  "
                      f"income {money(t.income):>13s}  "
                      f"MA frac {t.ma_fraction:5.1%}  MA tax {money(t.ma_tax)}")

    # Capital-gains layer
    print(f"\nCapital-gains layer (sale {sale_date}, "
          f"price {money(path.price(sale_date))}, DAF shares excluded):")
    for sc in (scenarios[0], scenarios[5]):
        gain, ma_cg = capital_gains_layer(sc, sale_date,
                                          daf_shares=0.05 * TOTAL_RSUS)
        print(f"  {sc.name:38s} gain {money(gain):>15s}  "
              f"MA tax if MA-resident at sale {money(ma_cg):>13s}  (NH: $0)")

    # DAF
    d = daf_estimate(scenarios[5], date(2027, 12, 15))
    print(f"\nDAF (5% pledge, transfer 2027-12-15 @ {money(d['price'])}/sh): "
          f"FMV {money(d['fmv'])}, federal deduction value "
          f"{money(d['fed_deduction_value'])}, matched giving {money(d['match_charitable_value'])}")

    # Sensitivity: move date x valuation path
    print("\nSensitivity — MA tax on vests (move date x path):")
    moves = [None, date(2026, 10, 1), date(2027, 1, 1), date(2027, 4, 1),
             date(2027, 7, 1), date(2028, 1, 1)]
    header = f"{'move date':>12s}" + "".join(f"{n:>14s}" for n in PATHS)
    print(header)
    for mv in moves:
        label = "stay" if mv is None else str(mv)
        row = f"{label:>12s}"
        for p in PATHS.values():
            row += f"{money(vest_tax(Scenario('x', mv, path=p))):>14s}"
        print(row)

    print("""
Flags (not modeled — verify with SALT counsel):
  * Statutory residency: Shrewsbury home (closed 2026-08-03) is a permanent
    place of abode; >183 MA days in a year = full MA resident regardless of
    domicile.  Track every part-day.
  * Move-year part-year residency: vests recognized before the move date are
    taxed 100% by MA here; MA may also assert full-year residency for the
    move year if the day count or abode test fails.
  * Lockup: cliff (2026-11-20) is inside the IPO lockup; income is still
    recognized at vest-date FMV even though shares can't be sold — a large
    dry tax charge.  Verify sell-to-cover/net-settlement in the RSU agreement.
  * Refresh grants made after the move are 0% MA if no MA workdays.
  * Domicile change must be genuine; MA DOR audits large pre-liquidity moves.
""")


if __name__ == "__main__":
    main()
