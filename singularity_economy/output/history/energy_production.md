# Energy Production & Scaling — Research for the AI/Robotics Transition Model (2026–2050)

**Framing:** electricity is modeled as the binding constraint on compute + a large robot fleet.
The central question: is energy a **hard ceiling** or a **self-relieving exponential** (solar learning curve + robots that build more solar)?

**Bottom line up front:** Generation (especially solar+battery) is on a Wright's-law exponential that can, in principle, outrun demand — global solar added ~600 GW in 2024 and ~700 GW in 2025, doubling roughly every 3 years. The *hard* near-term ceilings are (a) **firm/dispatchable capacity** for 24/7 datacenter load (gas turbines sold out to 2029–2030; nuclear 7–10 yr builds) and (b) the **grid** (US interconnection queues 4–9 yr; ~2.6 TW stuck in queue). China faces neither ceiling as hard. Model energy as a **soft, self-relieving ceiling with hard regional/firm-power sub-constraints**, not a single global cap.

---

## 1. SOLAR — the exponential

| Parameter | Value | Source / Year |
|---|---|---|
| Learning rate (module price, 4+ decades) | **~20% per doubling** of cumulative capacity (Swanson's Law) | Our World in Data, 2024 |
| Learning rate (LCOE, recent) | **24%**; most recent epoch (2014–2020) as high as **40–45%** | OWID / learning-curve literature, 2024 |
| Learning rate (capex, 2010–2020) | **~33% per doubling** | learning-curve studies, 2024 |
| Module spot price (2024 low) | **$0.096/W** (historic low, China overcapacity) | BloombergNEF, 2024 |
| Utility-scale LCOE (US) | **$38–78/MWh** | Lazard LCOE+ v18, 2025 |
| Global annual additions 2024 | **~602 GW** (record) | IEA-PVPS / REN21, 2024 |
| Global annual additions 2025 | **~698 GWp** (+16% YoY) | IEA-PVPS, 2025 |
| Cumulative installed (end 2024) | **~2,247 GW (2.2 TW)**; approaching 3 TW in 2025 | pv-magazine, 2025 |
| China share of new capacity | **~60%** (357 GW in 2024, ~415 GW in 2025) | IEA / Ember, 2025 |
| Capacity factor | **~15–25%** (US utility ~20–27% fixed-tilt/tracking; N. Europe ~10–12%; desert ~25–30%) | NREL/EIA range |
| Manufacturing capacity | **~1.8 TW/yr** module nameplate (2025) — supply is NOT the binding constraint | pv-tech, 2025 |

**Scaling dynamic:** cumulative capacity is doubling roughly every **~3 years** at current add rates. At a 20% learning rate, each doubling cuts module cost ~20%; at recent LCOE learning rates (24–40%), each doubling cuts delivered cost ~25–40%. Module is now a minority of system cost — **BOS, land, interconnection, and financing now dominate**, which flattens the *system* learning curve relative to the module curve. Manufacturing overcapacity (1.8 TW nameplate vs ~0.7 TW demand) means the near-term ceiling is **deployment/interconnection, not panels**.

**Modeling note:** treat module $/W with a 20–24% learning rate on cumulative GW; treat *installed system* $/W with a lower effective rate (~10–15%) because BOS/soft costs learn slower.

---

## 2. BATTERY STORAGE — making solar dispatchable

| Parameter | Value | Source / Year |
|---|---|---|
| Li-ion pack price (all segments, avg) | **$108/kWh** (record low, −8% YoY) | BloombergNEF survey, Dec 2025 |
| LFP pack price (avg) | **~$81/kWh** | BNEF, 2025 |
| **Stationary storage pack price** | **~$70/kWh** (−45% YoY; now cheapest segment) | BNEF, 2025 |
| Learning rate (Li-ion) | **~19–24% per doubling** (long-run ~19%; recent faster) | learning-curve lit. |
| Grid BESS deployment (2025, 3 quarters) | **~49.4 GW / 136.5 GWh** new (+36% YoY) | industry data, 2025 |
| 4-hr LCOS (utility) | **$115–254/MWh** (2025); rising to ~$210–292/MWh in early-2026 Lazard | Lazard LCOS, 2025–26 |
| Solar + storage combined LCOE | **~$61–156/MWh** | Lazard, 2025 |
| Round-trip efficiency (LFP) | **~85–92%** (AC-AC, incl. inverter losses) | industry standard |
| Cycle life (LFP) | **~4,000–8,000+ cycles** to 80% (10–20 yr daily cycling) | industry |

**Duration economics:** the sweet spot is **4-hour** storage — it captures daily solar-shifting arbitrage and most capacity-market value cheaply. Costs scale roughly linearly with duration (energy = $/kWh × hours), so **long-duration (8–100 hr) is expensive with Li-ion**; multi-day/seasonal firming needs a different chemistry (iron-air ~$20/kWh target, flow, thermal, or H2) or overbuild+gas. **Clean firm from solar+4hr battery works for the daily cycle but NOT for multi-day low-sun periods (winter, "dunkelflaute")** — that residual is what gas/nuclear fill.

**Solar + battery → dispatchable:** overbuild solar (2–4×) + 4-hr LFP converts an intermittent 20% CF resource into a high-availability daytime-to-evening firm block. At $70/kWh packs and $0.10/W modules, **solar+storage "solar peaker" and even ~16–24hr firm blocks are becoming cost-competitive with new gas in high-insolation regions**, but the last ~5–15% of hours (worst weeks) remains the expensive tail.

---

## 3. NATURAL GAS — the bridge for 24/7 load

| Parameter | Value | Source / Year |
|---|---|---|
| GE Vernova gas turbine backlog | **~80–83 GW** (jumped 62→83 GW in Q4 2025), stretches into **2029** | GE Vernova / Utility Dive, 2025 |
| New-order lead time | **~3–4 yr typical; up to 5–7 yr** for non-prioritized slots; delivery of a turbine ordered now → **late 2028+** | Power-Eng / NextBigFuture, 2025 |
| GE Vernova production capacity | **20 GW/yr by mid-2026**, stretch to **24 GW/yr by mid-2028** | GE Vernova, 2025 |
| CCGT efficiency (modern H-class) | **~60–64% LHV** (~54–58% HHV) | industry |
| Simple-cycle / peaker (aeroderivative) | **~35–42%** efficiency; fast-start bridge power | industry |
| Fuel cost (Henry Hub) | **~$3–4/MMBtu** (2024–25 range); ~$25–35/MWh fuel cost at CCGT heat rate | EIA |
| CCGT emissions | **~0.35–0.45 tCO₂/MWh** (vs coal ~0.9, vs 0 for solar/nuclear) | EPA/IEA |
| New-build capex | **~$1,290/kW** (cheapest firm capacity) | Lazard-referenced, 2025 |

**Why gas is the bridge:** datacenters want **24/7/365 firm power now**, and gas is the only dispatchable resource that can be permitted+built on a datacenter timeline and run at high capacity factor without weather risk. The **turbine shortage is a genuine hard constraint through ~2030** — the most efficient CCGTs are effectively sold out, forcing some load onto less-efficient simple-cycle/older units that burn 50–60% more gas per MWh (a structural gas-demand multiplier). Behind-the-meter gas ("bring your own power") is proliferating for datacenters to bypass interconnection queues.

**Modeling note:** gas capacity is **supply-chain-limited to ~20–40 GW/yr of new turbines globally** near-term — this is a real ceiling on how fast *firm fossil* capacity can grow, independent of fuel.

---

## 4. NUCLEAR / SMR — firm clean power, slow

| Parameter | Value | Source / Year |
|---|---|---|
| Conventional nuclear capex | **$6,400–12,700/kW** (US, incl. Vogtle overruns) | Lazard / industry, 2025 |
| Build time (conventional) | **~7–10 yr** (US); Vogtle 3&4 took ~15 yr incl. delays | industry |
| SMR timeline | **2030+ at earliest** for first commercial units | industry, 2025 |
| Hyperscaler nuclear commitments | **>13 deals, ~10 GW** total (MSFT/GOOG/AMZN/META) | 2025 |

**Key deals:**
- **Microsoft – Three Mile Island (Crane) restart:** 835 MW, 20-yr PPA, ~$16B, target accelerated to **H2 2027**.
- **Amazon – Talen/Susquehanna:** offtake expanded to ~2 GW through 2042; **$700M+ into X-energy** Xe-100 SMRs (up to 12 units).
- **Google – Kairos Power:** first US corporate SMR fleet deal, **500 MW by ~2035** (Hermes 2 ~2030).
- **Meta:** RFP for **1–4 GW** new nuclear.

**Role:** nuclear is the ideal 24/7 carbon-free firm resource for datacenters, but **build times (7–10 yr) and SMR immaturity (2030+) mean it contributes little before ~2030 and only meaningfully in the 2030s–40s.** Economics favor nuclear only under a carbon-free constraint ($6,400+/kW vs $1,290/kW gas). Restarts (TMI, Palisades) are the fastest nuclear MW available. Treat nuclear as a **slow, back-loaded firm-power wedge** in the model.

---

## 5. GRID — often the *real* US binding constraint

| Parameter | Value | Source / Year |
|---|---|---|
| US interconnection queue (total) | **~2.6 TW** stuck in queues (~2.5× system peak load; ~1 TW solar, ~1 TW storage) | LBNL "Queued Up," 2024 |
| US queue wait time | **4–9 years** to interconnect | LBNL / industry, 2025 |
| China UHV lines (end 2025) | **45 UHV lines, 52,300 km, ~300 GW** transfer capacity | Enerdata / Global Times, 2025 |
| China grid investment (15th FYP 2026–30) | **CNY 4 trillion (~$580B)**, +40% vs prior plan | State Grid, 2025 |
| China UHV approval→energization | **~2 years** (norm, not exception) | ChinaTalk, 2025 |
| China plan 2026–30 | +15 UHV lines, +35% cross-provincial capacity | Enerdata, 2025 |

**Why the grid, not generation, binds in the US:** you can manufacture panels and turbines faster than you can permit and build **transmission + interconnection**. Queue times (4–9 yr) exceed datacenter build timelines (~2–3 yr), which is *why* hyperscalers are going behind-the-meter (on-site gas, nuclear PPAs at existing plants). Transmission siting, NIMBY, and cost-allocation disputes make US grid expansion the slowest link.

**China's structural advantage:** state-directed UHV buildout moves power from western solar/wind bases to eastern load centers in ~2 yr, ~300 GW of transfer capacity already built, $580B more coming. **China's grid is not the binding constraint the way the US grid is** — this is the single biggest US-vs-China asymmetry in the model.

---

## 6. AI + ROBOT DEMAND

**Datacenter electricity (US):**
| Source | 2030 projection |
|---|---|
| LBNL | **325–580 TWh/yr = 6.7–12% of US electricity** |
| EPRI | **~6.8–9.1% (likely), up to 9–17%** of US electricity |
| Baseline | ~4–4.5% today (~200 TWh) → roughly **doubling** of share by 2030 |
| Regional | Virginia: 25% today → **41–59% by 2030** |

**Robot fleet draw:**
- Humanoid (Tesla Optimus): **~100 W idle, 250–500 W active**, **~1–3 kWh/day** per robot on an 8-hr shift; 2.3 kWh onboard pack.
- Fleet math: **1 billion humanoids × ~2 kWh/day ≈ 2 TWh/day ≈ 730 TWh/yr** — i.e., ~1 billion robots ≈ the entire current US datacenter+ load; **10 billion robots ≈ 7,300 TWh/yr ≈ ~1.7× total current US generation (~4,300 TWh)**. TW-scale continuous draw only appears at multi-billion-robot fleets running 24/7.
- Note: robot *manufacturing* + the compute training their policies may exceed the robots' own operating draw in the transition.

**Can solar+battery outrun demand?** Quantitatively plausible: global solar adds ~700 GW/yr × ~20% CF ≈ **~1,200 TWh/yr of new annual generation added per year**, and rising. That already exceeds projected *annual growth* in AI datacenter demand. The constraint is **not annual TWh of generation** — it's **(a) firm 24/7 delivery, (b) transmission, and (c) regional concentration** (load piles up faster than local grid/firm capacity in specific hubs).

---

## 7. KEY MODEL PARAMETERS (endogenous solar+battery+gas+nuclear mix)

### Learning / cost trajectories
| Parameter | Model value |
|---|---|
| Solar module learning rate | **20–24%** per doubling of cumulative GW |
| Solar *installed-system* effective LR | **~10–15%** (BOS/soft costs learn slower) |
| Battery (LFP) learning rate | **~19–24%** per doubling of cumulative GWh |
| Solar cumulative-capacity doubling time | **~3 yr** at current add rate (endogenous — accelerates if robots build) |
| Solar $/W (install, 2025 start) | utility ~$0.8–1.1/W (US higher, China ~$0.5–0.6/W) |
| Battery pack $/kWh (2025 start) | **$70 (stationary LFP)** → project to $40–50 by ~2030, $20–35 by 2040 |
| Solar LCOE 2025 → 2050 | $38–78/MWh (2025) → ~$15–30/MWh (2035) → **<$15/MWh (2050)** in good regions |
| Solar+4hr-storage LCOE 2025 → 2050 | ~$60–156/MWh → **~$40–70 (2035) → ~$25–45 (2050)** |

### Firm-power fractions & ceilings
| Parameter | Model value |
|---|---|
| Battery round-trip efficiency | **0.85–0.90** |
| Solar overbuild factor for daily-firm | **2–4×** nameplate vs average load |
| Residual firm fraction (multi-day/seasonal) that gas/nuclear must cover | **~10–25%** of energy, higher at high latitude / low overbuild |
| Gas CCGT efficiency | **0.60 LHV** (0.35–0.42 for peakers) |
| Gas fleet emissions | **0.35–0.45 tCO₂/MWh** |
| Nuclear/SMR contribution before 2030 | **near-zero new**; restarts only (~few GW) |
| Nuclear build time | **7–10 yr** (SMR first units 2030+) |

### Buildout-rate ceilings by region (the hard sub-constraints)
| Constraint | Ceiling |
|---|---|
| Global solar manufacturing | **~1.8 TW/yr** nameplate — NOT binding |
| Global new gas turbines | **~20–40 GW/yr** (GE Vernova ~24 GW/yr by 2028; +Siemens/Mitsubishi) — **binding for firm fossil** |
| US interconnection | **4–9 yr** lag; ~2.6 TW queue — **binding for US grid-connected** |
| China grid | **~2 yr** approval→energization; ~300 GW UHV + $580B — **NOT binding** |
| US new transmission | slowest link; treat as **hard US regional cap** |

### Structural recommendation for the model
Model energy as a **soft, self-relieving global ceiling with hard regional + firm-power sub-constraints**:

1. **Annual TWh (generation):** self-relieving exponential. Solar+battery learning curve + endogenous feedback (**robots and AI-optimized factories build solar/batteries → cumulative capacity doubles faster → cost falls → more deployment**). This is the "energy is NOT a hard ceiling" mechanism. Once robot labor is applied to panel/battery/BOS installation, the deployment-rate ceiling itself becomes endogenous and rises.
2. **Firm 24/7 delivery:** hard near-term constraint (2026–~2032) set by **gas turbine supply (~20–40 GW/yr) + nuclear lead times (7–10 yr)**. This is the binding constraint for AI datacenter load *specifically* because it wants flat 24/7 draw. Relieved over time as (a) cheap overbuilt solar + long-duration storage substitutes for firm, and (b) nuclear/SMR ramps in the 2030s.
3. **Grid/transmission:** hard **regional** constraint, severe in the US (4–9 yr queues), mild in China (~2 yr). Model as region-specific throughput caps; drives behind-the-meter gas and geographic concentration of compute where firm power is available.

**Verdict:** Energy is **not a hard global ceiling** on the transition — it's a **self-relieving exponential** (solar Wright's law + robot-labor feedback) **gated by three temporary hard constraints** (turbine supply, nuclear lead time, US grid) that bind hardest in **2026–2032** and relax thereafter. The US–China asymmetry (grid + firm buildout speed) is the dominant regional differentiator.

---

## Sources
- Solar deployment/prices: [pv-magazine 2026](https://www.pv-magazine.com/2026/05/14/solar-approaches-3-tw-but-the-industry-faces-new-challenges/), [IEA-PVPS Snapshot 2025](https://iea-pvps.org/trends_reports/trends-2025/), [Ember 2025](https://ember-energy.org/latest-updates/global-solar-installations-surge-64-in-first-half-of-2025/), [BloombergNEF via pv-tech](https://www.pv-tech.org/global-solar-module-manufacturing-capacity-to-reach-1-8tw-in-2025-report/)
- Solar learning rate: [Our World in Data — learning curve](https://ourworldindata.org/learning-curve), [OWID data insight (20%/doubling)](https://ourworldindata.org/data-insights/solar-panel-prices-have-fallen-by-around-20-every-time-global-capacity-doubled), [Swanson's Law (Wikipedia)](https://en.wikipedia.org/wiki/Swanson%27s_law)
- LCOE/LCOS: [Lazard LCOE+ v18 (June 2025)](https://www.lazard.com/media/eijnqja3/lazards-lcoeplus-june-2025.pdf), [PV Tech on Lazard solar](https://www.pv-tech.org/us-utility-scale-solar-pv-lcoe-tightens-to-us38-78-mwh-in-2025-lazard/)
- Battery prices: [BloombergNEF Dec 2025](https://about.bnef.com/insights/clean-transport/lithium-ion-battery-pack-prices-fall-to-108-per-kilowatt-hour-despite-rising-metal-prices-bloombergnef/), [ESS-News on BNEF](https://www.ess-news.com/2025/12/09/bnef-lithium-ion-battery-pack-prices-fall-to-108-kwh-stationary-storage-becomes-lowest-price-segment/)
- Gas turbines: [Utility Dive — GE Vernova backlog](https://www.utilitydive.com/news/ge-vernova-gas-turbine-investor/807662/), [Power-Eng](https://www.power-eng.com/gas/turbines/data-centers-drive-record-surge-in-ge-vernova-power-equipment-orders-as-turbine-slots-tighten-through-2030/), [NextBigFuture](https://www.nextbigfuture.com/2025/09/2028-2030-to-get-a-ge-290-430mw-natural-gas-turbine.html)
- Nuclear/SMR deals: [Introl blog](https://introl.com/blog/nuclear-power-ai-data-centers-microsoft-google-amazon-2025), [smrintel.com deal tracker](https://smrintel.com/nuclear-data-center-deals/), [Carnegie Endowment assessment](https://carnegieendowment.org/research/2026/06/beyond-the-hype-assessing-hyperscaler-nuclear-commitments-against-u-s-energy-realities)
- Datacenter demand: [LBNL / EPRI Powering Intelligence 2026](https://powering-intelligence.epri.com/executive-summary.html), [EPRI press](https://www.epri.com/about/media-resources/press-release/q5vu86fr8tkxatfx8ihf1u48vw4r1dzf), [WRI](https://www.wri.org/insights/us-data-centers-electricity-demand)
- Grid / China: [ChinaTalk — Transmission Dominance](https://www.chinatalk.media/p/transmission-dominance-with-chinese), [Enerdata UHV](https://www.enerdata.net/publications/daily-energy-news/china-plans-15-new-ultra-high-voltage-transmission-lines-2030.html), LBNL "Queued Up" 2024
- Robot power: [Teslarati Optimus specs](https://www.teslarati.com/tesla-production-optimus-bot-specs-price/), [ThinkRobotics](https://thinkrobotics.com/blogs/indepths/tesla-optimus-robot-price-2025-breakdown)
