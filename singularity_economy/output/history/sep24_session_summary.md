# Sep 24, 2026: Re-analysis Executed (Waves 1–4) and Reviewed

This follows the report card (`report_card_2026-09.md`) and the structural re-analysis
(`structural_reanalysis_2026-09.md`). Detailed notes: `wave1_book_mc_2026-09.md`,
`wave2_power_complex_2026-09.md`. Outputs: `output/book_mc/*_2026-09-24_final.txt`.

## What the model is now

**How the book is measured**
- Every name is valued on 6,000 Monte Carlo paths (`book-mc`), not six named scenarios.
- The prior carries the named book's 9.3% fizzle mass.
- Paths are conditioned on the observed 2026 (chips bind, pools near anchors). This
  rejects about 50% of raw draws, mostly ones where power binds in 2026.
- The structural lens is sampled: λ blends baseline structure toward v2.
- `book-sa` ranks what each call is a bet on.

**Valuation conventions (the CLI default; `--legacy` restores the old ones)**
- A cap on each firm's revenue-growth capacity.
- A terminal struck on steady-state replacement spend, not on a still-accelerating
  2036 build.
- Grid-equipment earnings routed through the model's power margin.
- Per-name capital structure (net debt, repricing, limited liability).
- A conv / drift / AI attribution on every name.

**New gated dynamics (all byte-identical at gate 0; golden unchanged)**

| Mechanism | Where it is on |
|---|---|
| Demand-growth decay; fizzle made demand-bound | fizzle; v2 |
| Forward q-governor | v2 |
| Explicit GW-per-capex-dollar trend | sampled |
| Saturating rents | v2 |
| Vintage power draw | v2 |
| Fab investment discipline | v2 |
| Reorganization lag on displacement (τ = 4) | enhanced (so v2) |
| Fiscal-dominance / inflation rates regime | enhanced (so v2) |
| Cash-flow capex funding | enhanced (so v2) |
| Inference market clearing | sampled ε |

## The book (MC, mix lens, first-principles valuation)

| | Name | E[up] | Median | P(loss) | Notes |
|---|---|---|---|---|---|
| **Robust longs** | 002472.SZ | +92% | +80% | 0% | Lens-invariant in every run |
| | NRG | +37% | +29% | 0% | |
| | 300748.SZ | +48% | +39% | 10% | |
| | WTKWY | +18% | +19% | 4% | |
| **Compute: right-skewed lens bets** | SK hynix | +161% | +155% | 24% | |
| | AVGO | +117% | +70% | 35% | |
| | MU | +87% | +79% | 32% | |
| | NVDA | +81% | +34% | 41% | E[log] < 0 |
| | TSM | +34% | +6% | 48% | |
| **Grid, reversed from August** | POWL | −24% | | 79% | Base lens alone −19% |
| | GEV | −39% | | 87% | Base lens alone −33% |
| | CEG | −29% | | | |
| | 6501.T | −2% | | | |
| **AI-services** | MSFT | −4% | | | ρ +0.80 on ε; ranges −31% to +83% across ε 0.7–1.35 |
| | PLTR | −52% | | | ρ +0.80 on ε; ranges −76% to +37% across ε 0.7–1.35 |
| **Shorts** | RHI, MAN, ADP, TCS.NS, PAYX | | | ~100% | Hold in every lens (AI-thesis shorts) |
| | MRVL, LSTR, EQIX, 6954.T | | | | Convention only: the AI economy *raises* them |

For MSFT and PLTR, the whole call is the inference demand elasticity ε.

**The August headline, restated.** "Power binds 2028–34" is now "power binds in about half
of paths". Over the conditioned prior, 54% of mix-lens paths (45% in the base lens) never
bind on power, and the median path is power-bound for 0–1 years.

## What the adversarial review caught (and was fixed)

A 13-agent review workflow checked today's diff: 4 dimension reviewers, then one skeptic
per finding. 7 of 9 verified findings were confirmed and fixed in c895190:

1. The grid flow terminal sustained the compute *draw*, not the installed grid. This
   understated grid names by 10–20pp in bust paths.
2. The silicon flow terminal assumed a fixed 15% unit-cost decline, inconsistent with
   Wright's-law v2 paths.
3. Spearman ranks did not average ties, so binary drivers were roughly halved. Fizzle
   now shows correctly as a top driver (ρ ≈ −0.45).
4. `book-sa` ranked post-fizzle parameters, which created fake drivers.
5. Fizzle's herding override changed the observed 2026, so conditioning thinned the
   fizzle mass.
6. The fiscal snowball used the marginal rate, so the inflation tax never worked.
7. A Fisher mismatch: the inflation premium was discounting real earnings.

Two claims were refuted (tau-lag double counting; the inflation target double counting
debt service).

**Known and not fixed**
- The book-mc prior turns on inference clearing and samples the GW-per-dollar trend in
  every lens, while the named books keep legacy values. So the "named vs MC gap" mixes
  prior differences with path diversity.
- The forward-q growth seed is sampled U(1.0, 2.5) but capped at 1.5 in use, so about
  2/3 of that prior sits at the cap.

## Bias check

Every structural addition today adds a constraint (bearish), while August's calibration
misses (capex, silicon rent, robot onset) were all *bullish-side* misses. The only
bullish channel added is inference clearing at ε > 1. Read the negative grid and
AI-services verdicts with that asymmetry in mind.

## Still queued

- Return-anchored demand outside v2. Only v2's q links AI revenue to capex; compute
  names are ε-invariant in the base and enhanced lenses.
- The philosophy wedge and capability-gated adoption.
- Capture shortlist #2–6: refinancing wall, electricity contract book, vendor financing,
  order backlog, endogenous algorithmic progress.
- The 2026 capex-level recalibration: $0.66T in the model vs ~$0.9T observed; measured
  book-neutral.

*Scenario analysis, not investment advice.*
