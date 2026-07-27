# Enhanced-Realism Book (the reflexive-bust lens)

**What this is.** The book repriced with the six gated satellites turned on *together*
— the coherent "enhanced-realism" enable set from `gated_dynamics_menu.md`, applied to
every scenario via `scenarios::enhanced_realism()`:

```
equity_sentiment_gain = 1.0   # R9 reflexive AI-capex bubble/bust (Minsky)
wealth_effect_gain    = 1.0   # ΔEquity-sentiment → consumption (cyclical GDP)
transmission_gain     = 0.5   # transformer/HVDC delivery lag caps usable power
wage_compression_*    = 0.4   # displaced-labor reserve compresses remaining wages
society.jg_share      = 0.3   # partial job-guarantee → smaller sovereign snowball
```

This is the **review lens, not the shipped default.** The baseline and the golden
snapshot stay byte-identical with all six off. Run it with `cargo run -- book-enhanced`.
Every gain is a hypothesis; nothing here is investment advice — it is scenario analysis.

## The headline: the top of the book flips

The baseline book is led by the **silicon + materials + merchant-power** complex.
Turn on the reflexive spine and that leadership inverts. What survives the Minsky bust
is the **physical bottleneck (grid equipment) and the robot-component chokepoint** —
the capex-reflexive names take a fat left tail.

| Ticker | Thesis | Baseline E[up] | Enhanced E[up] | Δ |
|---|---|---:|---:|---:|
| POWL | grid switchgear (bottleneck) | 450% | **411%** | −39 |
| GEV | grid equipment/turbines | 256% | **233%** | −23 |
| 300748.SZ | robot battery/components | 264% | **254%** | −10 |
| 002472.SZ | robot reducers (Shuanghuan) | 251% | **242%** | −9 |
| 6268.T | precision reducers (Nabtesco) | 188% | **181%** | −7 |
| MU | HBM memory | 198% | 65% | −133 |
| TECK | copper | 277% | 33% | −244 |
| FCX | copper | 187% | 5% | −182 |
| VST | merchant power (IPP) | 137% | **−11%** | −148 |
| NRG | merchant power | 83% | **−2%** | −85 |
| CEG | contracted nuclear | 58% | **−26%** | −84 |
| AVGO | custom silicon | 167% | **−16%** | −183 |
| TSM | foundry | 123% | **−18%** | −141 |
| NVDA | GPUs | 146% | **−32%** | −178 |
| BESI.AS | hybrid-bonding tools | 270% | **−30%** | −300 |
| ASML | EUV litho | 20% | **−62%** | −82 |
| ADP/PAYX/RHI | wage-linked (short) | −68/−68/−86% | **−72/−71/−88%** | deeper |

## Why it re-ranks this way

1. **The reflexive spine is the dominant term.** With `equity_sentiment_gain = 1.0`
   the AI-capex bubble cracks asymmetrically once the capacity glut passes the Minsky
   trigger — sentiment runs ~1.3 → ~0.4 and 2036 AI-capex prints ~2.9 vs ~7.6 baseline.
   Everything whose earnings *are* that capex — GPUs, foundry, HBM, back-end tools,
   copper — gets marked to the bust, not the boom. That is the whole 150–300pt haircut
   on NVDA/AVGO/TSM/MU/BESI/TECK/FCX.

2. **Grid equipment is the exception because it is the bottleneck, not the bubble.**
   `transmission_gain = 0.5` keeps delivered power scarce even as generation is built,
   so switchgear/turbine order books (POWL, GEV) stay full through the down-leg. They
   lead the enhanced book — the constraint is their *product*, so they de-rate least.

3. **Merchant power cracks; the bust kills the marginal-price thesis.** VST/NRG go
   negative and even contracted CEG turns down: in the reflexive bust the data-center
   demand that underwrites merchant power rents evaporates, and the C2 rent-decay term
   (already shipped) compounds it. The power *long* migrates decisively from the
   generators to the grid-gear that has to be built regardless.

4. **Robot components barely move.** The R2 robot bootstrap is far less capex-reflexive
   than the R1 compute flywheel — the fleet is materials/component-gated, not
   sentiment-gated — so 300748/002472/6268 hold ~250/242/181% and become the highest-
   conviction longs after grid gear.

5. **The short sleeve strengthens.** `wage_compression_* = 0.4` means the wage bill
   falls faster than headcount, so the wage-linked staffing/payroll/logistics shorts
   (RHI, ADP, PAYX, MAN, CHRW, LSTR) all deepen. The JG tilt (`jg_share = 0.3`) shaves
   the sovereign snowball and the long rate, a small offset that helps the surviving
   duration-heavy longs (POWL, GEV) but does not rescue the capex complex.

## The one-line takeaway

Under a realistic reflexive boom-bust, the book is no longer "own the AI-capex build."
It is **own the physical bottleneck (grid equipment) and the robot-component chokepoint,
short the wage-linked services, and treat the silicon/foundry/merchant-power complex as
a boom-only trade with a fat left tail.** The baseline book prices the boom; the
enhanced book prices what is left standing after the bust. Both are on the menu — the
gap between them *is* the reflexivity risk premium the model is trying to make visible.
