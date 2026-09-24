# Report Card — August 2026 Calls vs. September 24, 2026

Window: Aug 14 → Sep 23, 2026 (≈6 weeks). Benchmarks: S&P 500 −1.0%, Nikkei −5.4%,
CSI 300 −3.2%, KOSPI +1.5%, DAX −3.9% (Europe proxy), Nifty −3.8%. Returns are
excess over the local benchmark. Prices from Yahoo chart API / stockanalysis.com via
the grading workflow (all high confidence except 6954.T, medium).

**Framing caveat.** The model makes 10-year fair-value calls. Six weeks of price action
is mostly noise against that horizon: a rank correlation on n=41 has a standard error
of ≈0.16. The *macro nowcast* (section 1) is the fairer test of the model; the stock
grades (sections 2–3) are reported because they were asked for, not because they can
confirm or refute the thesis yet.

## 1. The model's 2026 macro predictions vs. observable data

| Prediction | Model | Observed (Sep 2026) | Grade |
|---|---|---|---|
| World GDP growth 2026 | ~3% | IMF Jul-2026: 3.0% | ✅ exact |
| Binding constraint 2026 | Chips | Chips — specifically HBM/memory + CoWoS packaging, sold out through 2026 | ✅ right, refine: memory+packaging, not logic |
| Chip scarcity vs glut | Scarcity | Scarcity, no glut; 40–52wk lead times | ✅ |
| Power scarcity direction | Rising | PJM cleared at cap two auctions running, record shortfall | ✅ direction; level too low (already ~model's 2028 tightness) |
| White-collar displacement 2026 | ~0.1% | Challenger 116k AI-cited cuts YTD ≈0.1%; Goldman net ~11–16k/mo | ✅ |
| Staffing industry | no collapse yet | Cyclical recovery (RHI Talent Solutions +3 sequential qtrs) | ✅ |
| 10y yield, 2026 avg | 4.49% | YTD avg 4.45% — but path +90bp Feb→Sep, **5.10% on Sep 23** | ⚠️ level-average right, **dynamics wrong**: market hit the model's *2036* level in 2026 |
| AI capex 2026 | $0.66T | Big-4 alone ~$0.73T; global ~$0.85–1.0T; every revision up | ❌ 25–40% low (partly nominal: memory-price inflation) |
| Silicon rent 2026 | ~50% | NVDA 75% GM; SK hynix ~76% op margin | ❌ ~25pts low; memory is the rent-taker |
| Humanoid production 2026 | ~0 until 2028 | ~19k shipped 1H26, ~60k FY26 (China-led) | ❌ onset 2+ years late |
| Displacement 2027→2028 | 1.3% → 11% | untestable; 2026 run-rate extrapolates to 0.2–0.4%/yr | ⚠️ ramp looks too steep (11% in one year exceeds Goldman's whole-transition 9%) |

**Macro verdict:** the model got the *qualitative structure* right — the constraint
sequence, GDP, the displacement level — and missed *magnitudes* in the direction
of underestimating the boom: capex, chip rents, and robot onset all run hotter than
modeled. Its largest miss is rates. The model only knows
duration-supply term premium; the 2026 selloff to 5.1% came from term premium + fiscal
+ inflation — exactly the fiscal-dominance/inflation mechanism flagged as missing in
August ("why don't rates go to 15%?").

## 2. The model's raw stock signals (rank-IC of E[upside] vs excess return)

| Lens | Spearman | Top-10 avg excess | Bottom-10 avg excess | Long/short spread |
|---|---|---|---|---|
| Baseline | +0.14 | −0.3% (6/10 beat) | −2.5% (7/10 lagged) | +2.2% |
| Enhanced | +0.11 | +1.2% (6/10 beat) | −2.3% (7/10 lagged) | **+3.5%** |
| v2 | +0.06 | −0.5% (5/10 beat) | −2.3% (7/10 lagged) | +1.8% |

**Signal verdict:** weakly positive in all three lenses and statistically
indistinguishable from zero at n=41 over 6 weeks. The **short side carried it**:
the displacement shorts worked (RHI −12.5%, PAYX −13.3%, LSTR −11.2%, TCS.NS −7.7%
excess; PAYX's Sep-23 print was the thesis on schedule). The long side was flat:
MU +11.3%, FCX +10.2%, CLS +9.0% against POWL −8.8%, GEV −9.5%, AVGO −8.6%, BESI −15.0%.

**The causal link to the macro miss:** the top longs were hit on Aug 18 and Sep 14 —
both "AI-power / rates" selloffs as the 10y ran to 19-year highs. Duration-heavy
bottleneck names (POWL, GEV, BESI) sold off on the exact rate mechanism the model
doesn't have. The model's biggest macro gap and its worst stock outcomes share a cause.

## 3. The August red-team overlays — graded separately

| Overlay call | Outcome | Grade |
|---|---|---|
| POWL: no adds until 22–25x + book-to-bill gate | −8.8% ex; gate never filled | ✅ avoided adding into drawdown |
| GEV: no add above $850–900 | −9.5% ex | ✅ |
| AVGO: half-size | −8.6% ex | ✅ halved the damage |
| BESI: trim to ⅓ stub | −15.0% ex | ✅ best call in the book |
| MP: trim into strength | −15.8% ex | ✅ |
| MRVL: gated pair short, don't enter | +18.5% ex (would have lost) | ✅ gate saved a loss the model would have taken |
| PLTR: options-only, gate closed | +11.2% ex (would have lost) | ✅ same |
| TECK: reclassify as merger event | +6.2% ex | ✅ |
| RHI: short via puts only | −12.5% ex | ✅ thesis + expression both worked |
| **Drop model shorts LSTR / TCS.NS / EQIX / 6954.T** | −11.2 / −7.7 / −4.9 / −1.0% ex | ❌ removed four winning shorts |
| **PAYX / ADP: no position, trigger-gated** | −13.3 / −2.4% ex | ❌ PAYX was the cleanest catalyst in the window |
| CLS: cut to tracker | +9.0% ex | ❌ |
| MU: begin indicator-gated scale-out | +11.3% ex | ⚠️ early; triggers only partly fired |
| CBZ: new short candidate | acquired at $55 (announced 7/28) | ❌ screening missed a public deal |
| NRG: hold (leverage flagged but unmodeled) | −19.2% ex | ❌ the leverage blindness the capture-shortlist #1 targets |
| LITE, SYM, 6268.T, HUBS, CHRW, CEG | within ±5% | — neutral |

**Overlay verdict:** the red team was **good at long-side risk management** (9 of 9
gating/trim/sizing calls on extended longs helped) and **bad at overriding the model's
short signals** (dropped 6 displacement/services shorts that went on to work). It also
saved the model from its two worst raw signals (MRVL, PLTR shorts). Lesson for the
process: keep the red team on longs' entry discipline; stop letting it veto the
displacement shorts without a quantified reason.

## 4. What this changes in the model (feeds the re-analysis)

1. **Rates regime** — add the fiscal-dominance / inflation / convex-term-premium
   mechanism; the rate path is now the dominant risk to the top of the book.
2. **Recalibrate 2026 anchors** — AI capex (~$0.85–0.95T), silicon rent level
   (split memory vs logic vs packaging), robot production onset (~60k units 2026).
3. **Nominal vs real capex** — memory-price inflation is inflating dollar capex without
   proportional capacity; the model conflates them.
4. **Displacement ramp** — the 1.3%→11% step (2027→28) needs a sensitivity check
   against observed adoption curves.
5. **Leverage** — NRG (−19%) is the first live cost of the per-name capital-structure
   blindness; move capture-shortlist #1 up the queue.

*Scenario analysis, not investment advice. Grading uses ~6 weeks of data; none of it
can confirm or refute 10-year calls.*
