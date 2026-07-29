# First-Principles v2 Book

The fullest first-principles configuration: the enhanced-realism dynamics (reflexive
spine + wealth effect + transmission lag + wage compression + JG tilt) PLUS the four
supply/cost/pricing mechanisms grounded from first principles this cycle:

- **Wright's-law compute cost** — unit cost falls with cumulative volume, not calendar.
- **Tobin's-q investment governor** — capex brakes when installed compute stops clearing
  its cost of capital (endogenous hurdle = sovereign rate + equity premium + depreciation).
- **Two-sided merchant power price** — power crashes in a glut, not just spikes in scarcity.
- **AI-services commoditization** — the AI provider's rent competes away as adoption saturates.

Run with `cargo run -- book-v2`. Still a review lens of hypotheses, not the shipped
default (baseline stays byte-identical). Scenario analysis, not investment advice.

## Three-way comparison (E[upside], %)

| Ticker | Thesis | Baseline | Enhanced | **v2** |
|---|---|---:|---:|---:|
| POWL | grid switchgear (bottleneck) | 450 | 411 | **372** |
| GEV | grid equipment | 256 | 233 | **209** |
| 002472.SZ | robot reducers | 251 | 242 | **159** |
| 300748.SZ | robot components | 264 | 254 | **154** |
| 6268.T | precision reducers | 188 | 181 | **104** |
| BESI.AS | hybrid-bonding tools | 270 | −30 | **136** |
| NVDA | GPUs | 146 | −32 | **65** |
| AVGO | custom silicon | 167 | −16 | **81** |
| TSM | foundry | 123 | −18 | **54** |
| MSFT | AI-services rent | 70 | 70 | **21** |
| GOOGL | AI-services rent | 45 | 15 | **9** |
| VST | merchant power | 137 | 41 | **25** |
| NRG | merchant power | 83 | −2 | **18** |
| CEG | contracted nuclear | 58 | −26 | **−5** |
| RHI / ADP / PAYX | wage-linked (short) | −86/−69/−68 | −88/−72/−71 | **−88/−72/−71** |

## Two findings the v2 composition surfaces

**1. Investment discipline makes the AI-capex bust SHALLOWER than pure animal spirits.**
Enhanced-realism (spine only) craters the silicon complex (NVDA −32, BESI −30, TSM −18)
because unrestrained reflexive capex overbuilds into a violent glut. Add the q-governor
and the bust is materially shallower (NVDA +65, BESI +136, TSM +54): the return-on-capital
brake cuts investment *before* the glut goes critical, so there is less overcapacity to
unwind. This is the q×spine interaction, now visible in the book — the same reason the
q-governor is kept OUT of the published enhanced set. The realistic AI-capex cycle is the
*balance* of animal spirits (spine) and investment discipline (q), not either alone.

**2. A new bear leg the prior books could not price: AI-services rent commoditization.**
Baseline and enhanced both hold MSFT's AI rent at ~70% and price GOOGL at 45/15%. v2 is the
first configuration where the AI PROVIDER'S pricing power is a variable, not an assumption —
and as adoption saturates the surplus competes away: MSFT 70→21, GOOGL 45→9. This is the
"do AI labs keep pricing power?" question made live. It hits the AI-services incumbents'
rent specifically and leaves the picks-and-shovels compute/infra layer (which sells
regardless of who wins the model race) comparatively intact.

## The v2 takeaway

Same structural spine as every prior book — **own the physical bottleneck (grid equipment
POWL/GEV), the robot-component chokepoint, and the compute leaders; short the wage-linked
services** — but v2 sharpens two edges the earlier books blurred:

- The silicon/AI-capex complex is a **cyclical**, not a permanent short: with investment
  discipline the bust is real but survivable (positive expected upside, deep worst-case).
- **Fade AI-provider pricing power** (MSFT/GOOGL AI rent) as a distinct short-the-rent leg —
  the commoditization thesis the frozen-margin books structurally could not express.

Merchant power (VST/NRG/CEG) stays the weakest long — two-sided pricing prices its glut
downside and the rent decays — while grid EQUIPMENT (the thing that gets built either way)
remains the highest-conviction physical long across all three books.
