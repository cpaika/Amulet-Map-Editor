# First-Principles v2 Book

The fullest first-principles configuration: the enhanced-realism dynamics (reflexive
spine + wealth effect + transmission lag + wage compression + JG tilt) PLUS the four
supply/cost/pricing mechanisms grounded from first principles this cycle:

- **Wright's-law compute cost** — unit cost falls with cumulative volume, not calendar.
- **Tobin's-q investment governor** — capex brakes when installed compute's OPERATING
  return (ai_services value / replacement cost) falls below its cost of capital +
  depreciation (endogenous hurdle = sovereign rate + equity premium + depreciation).
- **Two-sided merchant power price** — power crashes in a glut, not just spikes in scarcity.
- **AI-services commoditization** — the AI provider's rent competes away as adoption saturates.

Run with `cargo run -- book-v2`. Still a review lens of hypotheses, not the shipped
default (baseline stays byte-identical). Scenario analysis, not investment advice.

> **Re-audit correction (this cycle).** An adversarial re-audit found the q-governor's
> return numerator originally summed `ai_services + silicon + ip_tolls` — but silicon and
> ip_tolls are the *vendors'* revenue (∝ the ai_capex FLOW), i.e. the operator's *cost*,
> not the return on the installed *stock*. That made q ~94% capex-momentum. Fixed to use
> `ai_services` alone — the operating value the compute actually produces. The corrected
> governor brakes harder and earlier (q crosses 1 only where operating returns genuinely
> ramp/compress), which markedly changed v2 below and **reversed** the earlier "q dampens
> the bust" claim: on an operating-return basis the brake *deepens* the compute-complex
> bust rather than propping it up.

## Three-way comparison (E[upside], %)

| Ticker | Thesis | Baseline | Enhanced | **v2 (corrected)** |
|---|---|---:|---:|---:|
| 002472.SZ | robot reducers | 251 | 242 | **181** |
| 300748.SZ | robot components | 264 | 254 | **179** |
| 6268.T | precision reducers | 188 | 181 | **125** |
| POWL | grid switchgear (bottleneck) | 450 | 411 | **93** |
| GEV | grid equipment | 256 | 233 | **46** |
| MSFT | AI-services rent | 70 | 70 | **21** |
| GOOGL | AI-services rent | 45 | 15 | **−9** |
| NVDA | GPUs | 146 | −32 | **−49** |
| AVGO | custom silicon | 167 | −16 | **−28** |
| TSM | foundry | 123 | −18 | **−35** |
| BESI.AS | hybrid-bonding tools | 270 | −30 | **−36** |
| VST | merchant power | 137 | 41 | **−34** |
| NRG | merchant power | 83 | −2 | **−16** |
| CEG | contracted nuclear | 58 | −26 | **−37** |
| RHI / ADP / PAYX | wage-linked (short) | −86/−69/−68 | −88/−72/−71 | **−88/−72/−71** |

## What the corrected v2 says

The fully-disciplined first-principles model is the **most conservative** book. When
investment is gated on the *operating* return of installed compute — not on capex
momentum — only two theses survive as strong longs:

1. **The robot-component chokepoint leads** (002472 181%, 300748 179%, 6268 125%). The
   R2 robot bootstrap is materials/component-gated and largely independent of the compute
   q-brake, so it holds up while everything compute-reflexive is disciplined down.
2. **Grid switchgear (POWL 93%)** remains the strongest grid name — the physical
   bottleneck that must be built regardless — with GEV (46%) a clear but second-tier long.

Everything downstream of the compute-capex flywheel is now negative or weak:

- **The silicon/AI-capex complex goes negative** (NVDA −49, TSM −35, AVGO −28, BESI −36,
  ASML −68). Disciplined investment doesn't prop up the hardware vendors once the
  operating return on compute compresses in the glut — the corrected q-brake *deepens*
  this leg rather than cushioning it.
- **AI-services rent commoditizes** (MSFT 70→21, GOOGL 45→−9) — the "do AI labs keep
  pricing power?" leg, which the frozen-margin books couldn't express.
- **Merchant power turns negative** (VST −34, NRG −16, CEG −37) — two-sided pricing plus
  rent decay price the glut downside the one-sided model couldn't.
- **Wage-linked shorts** are unchanged and remain the highest-conviction short sleeve.

## The v2 takeaway

Across all three books the durable longs are the **physical bottleneck and the
robot-component chokepoint**, and the durable shorts are the **wage-linked services**.
What v2 sharpens, with a properly-specified return-on-capital brake, is that the
compute/silicon/merchant-power complex is a **boom-only** trade: on operating-return
discipline it does not survive the glut. Own the robot chokepoint and grid switchgear;
fade AI-provider pricing power; treat the silicon complex and merchant power as cyclical,
not structural. The gap between the baseline book (prices the boom) and v2 (prices what
survives disciplined investment through the bust) is the model's reflexivity-plus-
discipline risk premium.
