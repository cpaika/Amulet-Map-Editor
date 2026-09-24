# Wave 2 Results: the Power Complex (Sep 24, 2026)

Wave 2 of the structural re-analysis rebuilt the parts of the model behind its August
headline, "power binds 2028–34; POWL and GEV are the best names". Every dynamics change
is gated. At gate 0 the golden snapshot and the `--legacy` books are byte-identical,
and each gate was measured before it was switched on. Outputs are in `output/book_mc/*_wave2.txt`.

## What changed

| Item | Change | Where it is on |
|---|---|---|
| F2 demand engine | `dgb_decay_rate`: capex-desire growth converges to GDP. `perceived_growth_2026` pins the observed year. The named fizzle is now demand-bound (dgb 0.10, momentum 0.2). | fizzle; v2 lens (0.15) |
| F5 forward q (pulled forward from Wave 3) | The q-governor values compute on expected, not trailing, AI profit. The growth seed is sampled U(1.0, 2.5) in book-mc. | v2 lens |
| F3 GW per capex dollar | Explicit trend `gw_per_dollar_growth`. Legacy is +3.5%/yr (each dollar buys *more* power); the industry went the other way. | sampled U(−10%, +3.5%) in book-mc |
| F2 rents | `smooth_rents` (tanh saturation instead of hard clamps) and `electricity_margin_cap` | v2 lens |
| F7a vintage draw | Racked GPUs keep their build-year power draw. | v2 lens |
| F1 valuation | `ValuationParams`: firm capacity cap, steady-state flow terminal and power-margin routing, shipped together | **all book commands by default**; `--legacy` restores the old conventions |
| Measurement | `book-mc` prints the power-bound window as p10/p50/p90 | — |

## The headline, restated

**"Power binds 2028–34" was one point in a wide distribution.** Over the
2026-conditioned Monte Carlo prior:

| Lens | First power year | Last | Years bound | Never binds |
|---|---|---|---|---|
| Base | 2028 / 2030 / 2033 | 2031 / 2034 / 2036 | 0 / 2 / 7 | 42% |
| Enhanced | 2027 / 2028 / 2031 | 2028 / 2031 / 2036 | 0 / 2 / 6 | 33% |
| v2 | 2027 / 2028 / 2031 | 2028 / 2030 / 2034 | 0 / 0 / 2 | 76% |
| Mix | 2027 / 2029 / 2032 | 2029 / 2033 / 2036 | 0 / 0 / 6 | 50% |

(p10 / p50 / p90; the first- and last-year columns cover only the paths that bind.)
Power binding is roughly a coin flip, not the base case. The main swing factors are the
GW-per-dollar trend (−10%/yr: power never binds in the named baseline; legacy +3.5%:
2028–36) and the structural lens.

## What it does to the book

The named base book, first-principles valuation vs legacy:

| Name | Legacy | Now | Main cause |
|---|---|---|---|
| POWL | +293% | +43% | Flow terminal: 2036 grid-equipment spend was still growing ~48%/yr and was capitalized in perpetuity. The 20% capacity cap adds to this. |
| GEV | +184% | +32% | Flow terminal |
| 6501.T | +195% | +77% | Flow terminal |
| BESI | +379% | +118% | Capacity cap (20%/yr) |
| NVDA | +204% | +155% | Flow terminal (dollar-vintage steady state; unit-cost decline included) |
| SK hynix, 002472.SZ, VST, NRG, all shorts | — | within a few pp | No capex-pool exposure, or already capped |

Full MC (conditioned prior, mix lens, first-principles valuation):

- **Robust longs:** 002472.SZ +93% (P(loss) 0%, lens-invariant); 300748.SZ +54% (8%).
  SK hynix +142% (27%, but a lens bet); NRG +31% (0%).
- **Lens bets with fat right tails:** AVGO +117%, NVDA +77% (median +27%, P(loss) 45%),
  MU, CLS, BESI, TSM (median −2%).
- **Reversed:** POWL −37% (P(loss) 85%), GEV −53% (93%), 6501.T −13%, CEG −30%, VST
  +4%. In the base lens alone: POWL −25%, GEV −41%.
- **Shorts:** unchanged and still robust. RHI, MAN, CHRW, ADP, PAYX and TEAM lose on
  about 100% of paths in every lens.

## Why this might be wrong (read before acting)

1. **Observed power tightness runs hotter than the model.** PJM cleared at its cap in
   two auctions running, which is about the model's 2028 tightness level. The 2026
   conditioning rejects draws where *power* binds in 2026, because chips are the
   observed binding constraint. But observed power scarcity is real and rising. The
   grid verdict leans on that tension resolving toward chips.
2. **The GW-per-dollar prior is a judgment call.** U(−10%, +3.5%) centres on
   about −3%/yr. A prior centred on the legacy +3.5% would restore much of the grid
   upside (named base at 0%/yr: POWL +271% legacy valuation).
3. **The flow terminal's growth rate is tied to each name's terminal multiple.**
   It is internally consistent, but a secular AI-power share that keeps rising after
   2036 would justify a higher rate (POWL: g 4% → +32%, multiple-implied → +43%,
   life 20y → +57%).
4. **The capex level is still low.** The model has 2026 capex at $0.66T against
   ~$0.85–0.95T observed. That recalibration is not done, and it matters most for the
   grid names.

## Refuted or corrected this wave

- **"NVDA E[up] is non-decreasing in the forward-q growth seed"** (plan test): false.
  v2 has a regime switch near seed 1.35 (bust below, boom above) and is non-monotone
  within the boom. The lock now pins the switch.
- **Smooth rents unpin the power margin:** only slightly. It still sits at ~0.60
  against a 0.62 ceiling, so the pinning comes from utilization and is fixed
  upstream.
- **The first flow-terminal draft computed steady state in compute *units*.** That
  overstated the compute haircut (NVDA +99%, now +155%). It is now done in dollars
  with unit-cost decline.

*Scenario analysis, not investment advice.*
