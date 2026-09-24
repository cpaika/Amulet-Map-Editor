# Wave 1 Results: the Monte Carlo Book (Sep 24, 2026)

Wave 1 of the structural re-analysis (`structural_reanalysis_2026-09.md` §3) delivers the
measurement layer. Every name is now valued on thousands of sampled paths, not six
hand-picked scenarios. Raw outputs are in `output/book_mc/`.

## What was built

| Piece | What it does |
|---|---|
| `valuation::value_on_path` + `PathContext` | One valuation code path shared by the named book and the MC. The named books stay byte-identical. |
| `book-mc <n> <seed> [base\|enhanced\|v2\|mix]` | Values every name on every path. Reports mean, p10/p50/p90, P(loss), E[log], the gap to the named book, terminal share and ρ on λ. |
| Book prior (`sampler::BookSampler`) | The mc/sa prior plus the named book's 9.3% fizzle mass. The lens is fixed, or under `mix` λ ~ U(0,1) blends baseline structure toward v2. It uses a separate RNG stream, so mc/sa are unchanged. |
| 2026 conditioning (`base_year_consistent`, `ANCHORS_2026`) | Rejects paths whose 2026 contradicts the observed year (chips bind; mapped pools near anchors). It rejects about 50% of raw draws. |
| `book-sa` | Per-name Spearman drivers over the conditioned prior. |
| `tests/loop_dominance.rs` | Locks the loop signatures: B1 (−42%) and R4 (−26%) on 2029–36 capex, R1 (−39%) on 2026–28 displacement, all others within ±5%. |

## Findings

**1. Half of the raw MC prior contradicts 2026.** About 38% of raw draws have *power*
binding in 2026, where the observed binding constraint is chips (HBM and CoWoS sold out).
Another ~3% have a 2026 capex collapse. Any `mc` statistic that includes those draws is
partly describing a counterfactual year. Conditioning on 2026 trims the power names
(mix lens: POWL 126% → 117%, GEV 68% → 62%) and leaves everything else within noise.

**2. The compute complex is a bet on the model's structural hypotheses, not on the
paths.** For NVDA, TSM, AVGO, MU, SK hynix, BESI, ASML, LITE, CLS, VST, NRG and CEG, the
#1 driver of upside is the lens fraction λ (ρ ≈ −0.55 to −0.65). The #2 driver,
singularity timing, is about half as strong (ρ ≈ −0.27). Put plainly, how much you like
these names depends mainly on whether you believe the v2 mechanisms: Wright-law cost
learning, the q-governor, the power glut and AI commoditization. The dated path within a
lens matters less.

**3. What survives every lens (P(loss) ≈ 0–15%, |ρ_λ| < 0.1):**

| Name | Base E[up] | V2 E[up] | Mix E[up] | Mix P(loss) | Main driver |
|---|---|---|---|---|---|
| 002472.SZ (Shuanghuan) | 93% | 94% | 93% | 0% | singularity year |
| 300748.SZ (JL Mag) | ~53% | 55% | 54% | 9% | singularity year |
| POWL | 178% | 46% | 117% | 15% | singularity year, power base growth |
| WTKWY | ~12% | 13% | 12% | 8% | prof-info beta |

POWL is lens-sensitive in *level* but keeps a low loss probability in every lens. Only the
robotics-component names are truly lens-invariant.

**4. Median vs mean.** Under `mix`, NVDA's mean is +74% but its **median is −1%**, with
P(loss) 50% and a negative E[log]. TSM (median −17%), ASML (−27%) and LITE (−31%) have
the same shape. The upside is a right tail from the high-λ-off paths. A sizing rule
based on E[log] would hold these names small or not at all.

**5. The short book is robust.** RHI, MAN, CHRW, ADP, PAYX, TEAM, 6324.T and MMC lose on
98–100% of paths with |ρ_λ| < 0.15. The drivers are timing (singularity year, adoption
half-life) and task exposure, not the lens. That matches the report card, where the
displacement shorts were the part of the signal that worked. The exceptions are LSTR,
EQIX and MRVL, which *are* lens bets (ρ_λ ≈ −0.55).

**6. Named book vs MC.** The named book overstates the power complex relative to the MC:

| Lens | VST | CEG | NRG | GEV | POWL |
|---|---|---|---|---|---|
| Mix | −45pp | −18pp | −32pp | −45pp | −62pp |

The mix figures are gaps against the average of the named base and v2 books. This is the
F4 finding (named scenarios omit the MC's dominant capex-decline mode) now measured per
name.

**7. Taiwan weight.** Severe-Taiwan paths (a blockade or invasion by 2036) make up 26% of
the conditioned prior. The named book weights `taiwan_shock` at 7% (a 2027–31 window).
This is flagged, not changed.

**8. Inert loops.** R2 (robot bootstrap) and R5 (launch learning) move robot output by
less than 1% when knocked out. They are carried as structure with no measured effect.

## Known 2026 calibration gaps (not enforced yet)

The check anchors pools to the calibrated base, not to observed levels, because the
default misses two observations. Both are queued for Wave 2:

- AI capex: observed about $0.85–0.95T; the model has $0.66T.
- Electricity price: observed about $0.05–0.10/kWh; the model has $0.136/kWh.

*Scenario analysis, not investment advice.*
