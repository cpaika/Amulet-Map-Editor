# Food Production & Agriculture — Systems-Dynamics Reference for the AI/Robotics Transition (2026–2050)

**Purpose:** Hard-number reference for modeling food as (a) a production system that AI/robots make cheaper and (b) a political-stability variable where food-price spikes drive unrest. Two coupling directions matter: food ← energy (fertilizer via natural gas) and food → political tension. All figures carry source + year; where sources conflict, ranges are given. Compiled 2026-07-26.

**Headline framing:**
- Food is a low-automation, high-labor, energy-and-nitrogen-dependent system. AI/robots attack the ~12c farm share and the labor line; they do NOT touch the physics of nitrogen or water fast.
- The tightest exogenous linkage is **fertilizer ← natural gas** (Haber-Bosch): a gas-price shock passes through to N-fertilizer with elasticity ~0.8, and N-fertilizer underwrites ~half of world food. This wires the food layer to the energy layer.
- The tightest endogenous social linkage is **food price → unrest**: the FAO Food Price Index has a widely-cited disruption threshold near **210** (2004=100 basis), above which riot probability jumps. This wires food to the political-stability/tension layer.

---

## 1. AG AUTOMATION — Robots, Autonomy, and the Cost/Labor Attack Surface

**Where the money is (US food dollar, USDA ERS 2024):**
- **Farm share = 11.8 cents** of every consumer food dollar; **marketing share = 88.2 cents** (processing, transport, wholesale, retail, energy, and — the largest single line — **labor**). [USDA ERS Food Dollar, 2024]
- Implication for the model: **automating the farm gate alone can only compress ~12% of retail food cost.** The bigger automatable pool is post-farm labor across the 88c marketing bill. Labor is "the largest single element in the food marketing bill" (USDA ERS).
- Farm labor is itself a minority of on-farm cost but is the binding constraint operationally: **US average producer age >58**, unmet demand ~**2.4M farm workers/yr** (industry, 2025). Labor scarcity — not labor cost per se — is the automation driver.

**Ag robotics market (directional, market-research):**
- Global agricultural robots market: **$7.3–17.7B (2024–25) → $26–56B (2030–32)**, CAGR **~18–26%** (MarketsandMarkets, GM Insights, 2025). Small vs. the ~$3–4T global farm-gate economy — automation is early.
- At CES 2025: second-generation fully autonomous tractors, orchard sprayers, remote dump trucks (camera + LiDAR + AI, no cab operator). Autonomous combines and picking robots in field trials.

**Quantified input/labor savings (precision ag + autonomy, 2025):**
- **Herbicide: up to −60%** via see-and-spray / targeted weeding (vs. broadcast).
- **Nitrogen fertilizer: −10 to −30%** via variable-rate application (VRA), typically **−12 to −25%** while holding/improving yield; fertilizer *cost* down **15–40%** with AI soil-nutrient prediction (Omdena, Farmonaut, 2025–26).
- **Water: −20 to −35%** via AI irrigation scheduling; **yield +10 to +20%** claimed for full precision-ag stacks (directional vendor numbers, treat as optimistic ceilings).
- Autonomous machinery cuts labor dependency, optimizes fuel, reduces mechanical wear (smoother control).

**Model takeaway (automation cost lever):** AI/robots plausibly cut *farm-gate production cost* by an order of **1–3%/yr** compounding over 2026–2040 as autonomy diffuses (precision inputs + labor substitution), with a larger one-time step available in the post-farm labor bill (88c pool) as general-purpose robots enter processing/logistics. This is a **slow, ceiling-limited** decline vs. compute/robotics — food does NOT get a Wright's-law cliff because land, water, and nitrogen are the binding physics, not manufacturing.

**Vertical farming — do NOT treat as a food-supply solution in the base case:**
- **Capex: $1–2M per acre-equivalent of growing capacity vs. $5–10k/acre** for field lettuce — **100–200x** the capital per unit output (2025).
- Energy: optimized leafy-green systems ~**11–15 kWh/kg** (new viability threshold) down from 150–350 kWh/kg legacy; energy cost alone ~**$0.50/kg**, roughly the *entire* farm-gate price of field lettuce — before capex, labor, refrigeration.
- **Bankruptcy wave: ≥28 CEA/vertical-farm companies failed 2024; 14 more in 2025.** Bowery ($2.3B peak valuation), Plenty (~$1B raised, Bezos/Schmidt-backed, Chapter 11 Mar 2025), AeroFarms (>$300M, Chapter 11 2023). [foodlore.blog, CEAg World, 2024–25]
- **Verdict:** vertical farming stays a **niche** (high-value leafy/herbs, import-substitution for water-scarce/cold regions) through 2050 unless electricity goes very cheap AND general-purpose robots kill the labor line. It is an *energy-price-elastic* niche, not a staple-calorie source. Useful only as a hedge parameter that switches on if power prices collapse.

---

## 2. FERTILIZER–ENERGY LINKAGE — The Haber-Bosch / Natural-Gas Wire (KEY CROSS-LAYER COUPLING)

**The physics:** Haber-Bosch fixes atmospheric N₂ into ammonia (NH₃) using hydrogen stripped from **natural gas** (steam methane reforming). Gas is both feedstock AND process heat.

**Energy footprint:**
- Haber-Bosch consumes **~1–2% of total global energy** and **~3–5% of global natural gas** production; ammonia ~**1.7–2.5% of global gas consumption/yr**. ~**80%** of fertilizer-industry gas goes to ammonia via Haber-Bosch. [multiple, 2020–25]
- Global ammonia output ~**170–185 Mt/yr**, ~**80% into fertilizer**. Natural gas is **~70–90% of ammonia production cost** (the reason for the tight price wire).

**Food dependence on synthetic N:**
- Synthetic nitrogen fertilizer feeds **~48% of the world population** (≈half); without Haber-Bosch, ~half the world lacks enough food (Smil; Nature Geoscience 2008; Our World in Data). This is the single largest fragility multiplier in the food system.
- Nitrogen is the yield-limiting input: modern cereal yields (see §3) are structurally impossible without ~110 Mt N/yr of applied fertilizer.

**Price transmission gas → fertilizer (THE elasticity to hard-code):**
- **Long-run ammonia price elasticity w.r.t. natural gas price ≈ 0.8**; gas–ammonia price correlation **~0.7–0.8** (USDA ERS; farmdoc/Illinois, 2022). Use **ε(NH₃/gas) ≈ 0.7–0.8**.
- 2020–22 shock as calibration: anhydrous ammonia **$290/ton (Jun 2020) → $1,350–1,430/ton (Jan–Nov 2022)**, a **~4.7x** rise tracking the European gas spike (Ukraine war shut ~70% of EU N-fertilizer capacity). Urea and DAP moved similarly.
- Pass-through chain: **gas price → ammonia → urea/DAP/UAN → farm input cost → planted area & yield decisions → grain supply → food price.** Each hop damps and lags (months to a season).

**Model wire:** `fertilizer_cost = f(gas_price^0.8)`; `N_applied = g(fertilizer_cost, crop_price)` with own-price elasticity of N demand roughly **−0.2 to −0.4** short-run (farmers cut N when it gets expensive, hitting yield next season). A gas shock therefore hits food supply with a **1–2 season lag**. Green ammonia (electrolytic H₂) can eventually **break this wire** — model as a switch that decouples fertilizer from gas once electrolyzer H₂ < ~$1–2/kg, plausibly 2035–2045 in cheap-power scenarios; until then, food stays gas-coupled.

---

## 3. YIELDS, LAND, WATER & AI OPTIMIZATION

**The ~4-tonnes-grain-per-capita system (production side):**
- Global cereal production **~2.85–2.96 Bt (2024/25)** on **~736 Mha** harvested. Maize **1,226–1,289 Mt**, wheat **~800 Mt**, rice **~541–555 Mt** (FAO/USDA 2024/25).
- Average global cereal yield **~3.4 t/ha (2024)**, up from ~1.4 t/ha in 1961 — roughly **+1.3%/yr** trend, but **decelerating** in mature regions.
- Per-capita: ~2.9 Bt cereals / ~8.1B people ≈ **~350–360 kg grain/capita/yr** (the "4 tonnes" framing usually refers to total primary grain-equivalent incl. feed; direct cereal is ~0.35 t/cap). Roughly **35–40% of cereals go to animal feed**, the lever alt-protein attacks (§4).

**Land & water constraints (slow/hard limits):**
- **Agriculture = ~70% of global freshwater withdrawals** (UNESCO WWDR 2024; note a 2025 critique puts the empirically-defensible range at **45–90%**). Irrigation is ~20% of cropland but ~40% of output. Groundwater ~25% of irrigation water — depleting in India/N. China/US High Plains.
- ~3,000 L water/person/day of food. Cropland ~**1.6 Bha**, near-static; expansion drives deforestation. Land and water are **inelastic constraints** — model as near-fixed ceilings that AI optimizes *within*, not expands.

**How AI raises yield (the optimization lever):**
- **Precision inputs / VRA:** −10 to −30% N with equal/better yield; ~+12% yield at −30% excess N in trials (§1).
- **Gene editing (CRISPR/Cas) + AI:** AI designs guide-RNAs, predicts off-targets, prioritizes trait targets; accelerates breeding cycles for yield, drought/heat tolerance, disease resistance. Real deployments exist (soybean architecture, staple-crop quality) but yield gains compound **slowly** (breeding cycles + regulatory), not step-changes.
- **GNoME-style design:** DeepMind's GNoME (materials) is the analogy — AI foundation models for genomics (protoplast MPRA, multispecies genomic models, AlphaFold-lineage protein design) are moving toward *de novo* trait design. Treat as a **yield-trend accelerator** that lifts the ~1.3%/yr yield trend modestly (add ~0.2–0.5 pp/yr in optimistic AI-bio scenarios post-2035), NOT a discontinuity — biology iterates on seasons.

**Model takeaway:** yield is a **slow-moving, constraint-bounded** variable. AI bends the trend up by tenths of a percent per year and reduces input intensity (helping the fertilizer-gas wire), but land/water/nitrogen physics cap it. Climate (§5) pushes the other way.

---

## 4. SYNTHETIC / ALTERNATIVE PROTEIN — The Land/Animal Decoupling Option

**Cost curves (2025–26):**
- **Cultured meat:** Gourmey **€7/kg** at commercial scale (May 2025); cultivated chicken ~**£10.9/kg** retail (2025); Mosa Meat claims **−99.999%** vs. 2013 prototype (which was ~$1M/burger). GFI projects **cost-competitiveness with conventional meat ~2030** (~£5/kg / ~£2.31/lb).
- **Precision fermentation** (recombinant proteins via engineered microbes): ReThinkX projected **~$10/kg by 2025**; GFI June-2025 meta-analysis (55 techno-economic models) finds **biomass fermentation (mycoprotein) approaching parity with beef/pork**, while **precision-fermentation proteins & microbial oils face a steeper climb**. Viability discussed around **~$25/kg** thresholds today.
- Three cost levers: **feedstock, capex (bioreactors), fermentation efficiency**.

**Land decoupling potential (the upside case):**
- ReThinkX (aggressive): by **2035, ~60% of livestock+feed land freed** (~485M acres US) as precision fermentation collapses dairy/beef; US cow numbers −50% by 2030, beef/dairy revenue −90% by 2035. **Note: ReThinkX's earlier (2019) "bankrupt by 2030" call has proven too optimistic** — treat as a fast-scenario bound, not baseline.
- Market-research (ResearchAndMarkets 2023): ~**60% of meat cell-grown by 2040** — also aggressive.
- Realistic baseline: alt-protein reaches **low-single-digit % of protein market by 2030**, **~10–20% by 2040** in favorable cost/regulatory paths. The decoupling matters because **~35–40% of cereals are feed** and livestock uses most agricultural land — even partial substitution relaxes land/water/feed-grain demand.

**Model takeaway:** treat alt-protein as an **S-curve adoption variable** (logistic) gated on a cost-parity crossover (~2030 for premium, later for commodity), whose payoff is **relaxing the land/feed constraint and the emissions line**, not near-term calorie security. It is a *land-liberation and price-ceiling* lever, most impactful post-2035.

---

## 5. CLIMATE IMPACT — Yield Stress and Regional Shifts (Downward Pressure)

**Per-degree sensitivity (hard-code as a warming penalty):**
- **Wheat: −6.1% yield per +1°C** below +2.38°C warming; **−8.2% per +1°C** above that (nonlinear worsening) [Nature Sci. Reports 2025].
- Maize is more vulnerable than wheat. Business-as-usual (SSP5-8.5): **maize −22%, wheat −14%** by 2080–2100 vs. 2015.

**2050 horizon (the model window):**
- Median grid-cell yield decline **~3–12%** by ~2050, with hotspots **−16 to −30%**.
- **Sub-Saharan Africa: staple yields −10 to −20% by 2050** (Ethiopia maize ~−15%). **Southern Europe wheat up to −49%**, EU maize −1 to −22%. **Egypt cereal yields −6 to −15%.**
- Regional **shift poleward**: gains at high latitudes (Canada, Russia, N. Europe) partly offset losses in the tropics/subtropics — but the losers are disproportionately the **already food-insecure importers** (§6), amplifying the stability risk.

**Model takeaway:** apply a **regional climate yield drag** growing to roughly **−0.2 to −0.5%/yr** in vulnerable low-latitude regions through 2050, partially offsetting AI yield gains, and — critically — **concentrated in import-dependent, politically fragile regions**, so it feeds §6 more than it dents global supply.

---

## 6. FOOD → POLITICAL STABILITY — The Coupling to Tension/Unrest

**The threshold (calibration anchor):**
- **FAO Food Price Index (FFPI) ≥ 210** (on the older 2002–04=100 basis) marks a widely-cited **"disruption threshold"**: food riots become markedly more likely above it (Lagi/Bar-Yam, New England Complex Systems Institute, 2011). The 2008 and 2011 spikes both breached ~210–240 and coincided with riots in **30+ countries** and the **Arab Spring** onset.
- **Basis caveat for the model:** FAO **rebased to 2014–16 = 100** in 2020. The old-basis 210 ≈ a *real-terms* level; current index runs **~120–130** on the new basis (2025 avg **127.2**, +4.3% vs 2024; Feb–Mar 2026 **125.3**). To use the 210 rule, convert to the 2014–16 basis or use **real/deflated** FFPI and detect **rapid rises**, not just levels.

**Econometric coupling (what to hard-code):**
- Direction of causation is robust: multiple panel studies (Arezki & Brückner 2011; Smith 2014; **Bellemare 2015, AJAE** — the cleanest) find **higher food price *levels* → higher probability of protests/riots/unrest**; interestingly **price *volatility* is NOT significantly linked** (Bellemare) — it's the **level/trend** that matters. Use *level and rate-of-rise*, not variance.
- MENA/Arab-Spring test: probability the unrest coincided with high food prices by chance **p < 0.06** (statistically significant).
- Pass-through is **not 1:1** and varies hugely by country (subsidies, controls). Tunisia held food-price controls even during its uprising — so **domestic pass-through / subsidy buffering is a country-level modifier** on the coupling.

**Which regions are food-insecure / import-dependent (where the coupling fires):**
- **MENA = largest net cereal importer and most water-scarce region on Earth**; **North Africa = world's largest wheat-importing region.**
- **Egypt: world's largest wheat importer**, ~60% of cereal imports, pop. >110M (bread subsidy is a regime-stability keystone).
- **Algeria imports ~75% of food; Libya up to ~90% of cereals; Tunisia ~70% of wheat; Morocco >50% of cereals.** 2024/25 = 3rd consecutive drought year (worst since 2003), forcing higher imports.
- Also structurally exposed: Sub-Saharan Africa (climate + low buffers), parts of South Asia, import-dependent poor states generally. These are the **model's high-gain nodes** for food→tension.

**Model wire (food → political-stability):**
- Define a **food-stress index** per region = f( real food price level relative to threshold, rate-of-rise, import-dependence share, subsidy buffer, income share spent on food ). Poor importers spend 40–60% of income on food → high gain; rich food-exporters → near-zero gain.
- **Unrest probability rises sharply once real food prices breach the ~210-equivalent (real) threshold or rise fast**; couple this into the geopolitics/tension layer (see sibling `geopolitics_sweep`, `tension_supplement`). A **gas shock (§2) → fertilizer → food price → unrest in importers** is the full cross-layer transmission chain the model should carry.

---

## 7. KEY PARAMETERS (for the Rust model)

| Parameter | Point value | Range | Basis / source |
|---|---|---|---|
| **Farm share of retail food $** | 0.118 | 0.10–0.14 | USDA ERS Food Dollar 2024 |
| **Post-farm (marketing) share** | 0.882 | — | USDA ERS 2024 |
| **Food-production cost decline under AI/robots** | **−1.5%/yr** | −1 to −3%/yr | Precision-ag + autonomy diffusion (derived); ceiling-limited |
| **Precision N-fertilizer reduction** | −20% | −10 to −30% | VRA studies 2025 |
| **Herbicide reduction (see-and-spray)** | −50% | −40 to −60% | 2025 |
| **AI irrigation water saving** | −25% | −20 to −35% | vendor 2025 (optimistic) |
| **Yield uplift trend (baseline)** | +1.3%/yr | +1.0 to +1.5 | FAO 1961–2024 (decelerating) |
| **AI yield-trend accelerator (post-2035)** | +0.3 pp/yr | +0.2 to +0.5 | CRISPR+AI, GNoME-style (speculative) |
| **Ammonia price elasticity w.r.t. gas price** | **0.8** | 0.7–0.8 | USDA ERS / farmdoc 2022 |
| **Gas–ammonia price correlation** | 0.75 | 0.7–0.8 | 2022 |
| **Gas cost share of ammonia production** | 0.80 | 0.70–0.90 | 2022–25 |
| **N-fertilizer demand own-price elasticity (SR)** | −0.3 | −0.2 to −0.4 | derived |
| **Fertilizer→food-supply lag** | 1–2 seasons | — | agronomic |
| **Share of population fed by synthetic N** | 0.48 | ~0.5 | Smil / OWID |
| **Haber-Bosch share of global energy** | 1.5% | 1–2% | multiple |
| **Haber-Bosch share of global gas** | 4% | 3–5% | multiple |
| **Agriculture share of freshwater withdrawal** | 0.70 | 0.45–0.90 | UNESCO 2024 / 2025 critique |
| **Cereals to animal feed** | 0.37 | 0.35–0.40 | FAO |
| **Grain per capita (direct cereal)** | ~0.35 t/cap/yr | 0.33–0.36 | FAO 2024/25 ÷ pop |
| **Climate yield drag, vulnerable regions** | −0.35%/yr | −0.2 to −0.5 | to 2050, SSA/S.Europe/MENA |
| **Wheat yield sensitivity** | −6.1%/°C | to −8.2%/°C >2.38°C | Nature 2025 |
| **Maize / wheat loss by 2080–2100 (BAU)** | −22% / −14% | — | SSP5-8.5 |
| **Alt-protein cost-parity crossover** | ~2030 (premium) | 2030–2040 commodity | GFI 2025 |
| **Alt-protein market share** | ~2% (2030) → 10–20% (2040) | up to 60% (aggressive) | GFI baseline / ReThinkX bound |
| **Land freed if alt-protein scales** | up to 60% livestock land by 2035 | — | ReThinkX (fast bound) |
| **Vertical-farm capex penalty** | 100–200x field | — | 2025 |
| **Vertical-farm leafy energy** | 11–15 kWh/kg | up to 350 legacy | 2025 |
| **FAO disruption threshold (real, old basis)** | **210** (2002–04=100) | 200–220 | NECSI 2011 |
| **Current FFPI (2014–16=100)** | 125 (2026) | 124–130 | FAO 2025–26 |
| **Food price → unrest signal** | **level & rate-of-rise** (NOT volatility) | — | Bellemare 2015 |
| **Food-stress gain, poor importers** | high (food = 40–60% of income) | — | derived |
| **Food-stress gain, rich exporters** | ~0 | — | derived |
| **MENA import-dependence (unrest nodes)** | Egypt ~60% cereals, Algeria ~75% food, Libya ~90%, Tunisia ~70% wheat, Morocco >50% | — | 2021–24 |

### Suggested coupling equations (compact)

```
# Energy → food (per season, lagged)
fertilizer_cost_t   = fertilizer_cost_0 * (gas_price_t / gas_price_0)^0.8
N_applied_t         = N_base * (fertilizer_cost_t / fertilizer_cost_0)^(-0.3)
yield_penalty_t+1   = k_N * (1 - N_applied_t / N_optimal)         # nitrogen shortfall hits next season

# AI/robot cost & yield
food_cost_t         = food_cost_0 * (1 - 0.015)^(t-2026)          # farm-gate; -1..-3%/yr
yield_t             = yield_0 * (1.013 + AI_accel_t)^(t) * (1 - climate_drag_region_t)

# Food → political stability (per region)
real_ffpi_t         = ffpi_t / cpi_deflator_t
food_stress_r,t     = w_import_r * w_incomeshare_r * relu( (real_ffpi_t - 210)/210 + β * dFFPI/dt ) / subsidy_buffer_r
tension_r,t        += λ * food_stress_r,t          # feed into geopolitics/tension layer
```
Where poor net-importers (MENA, SSA) carry high `w_import * w_incomeshare` and low `subsidy_buffer`, and food-exporters carry ~0. `λ` is the food→tension coupling coefficient to be swept.

---

## Cross-layer summary (how food plugs into the rest of the model)

1. **Energy layer → Food:** natural-gas price is the dominant exogenous shock path, via Haber-Bosch (ε≈0.8), lagged 1–2 seasons. Green ammonia can sever this wire post-2035 in cheap-power scenarios.
2. **AI/Robotics layer → Food:** slow cost decline (−1 to −3%/yr, ceiling-limited by land/water/N physics), modest yield-trend acceleration, big potential in post-farm labor (88c bill) as general-purpose robots arrive. Vertical farming stays niche.
3. **Bio/AI layer → Food:** alt-protein S-curve (parity ~2030+) relaxes land/feed/emissions constraints post-2035.
4. **Climate layer → Food:** downward yield drag concentrated in fragile importers (SSA, S. Europe, MENA).
5. **Food → Political-Stability layer:** real food-price level & rate-of-rise (threshold ~210 real) drive unrest, gained up by import-dependence and food-income-share; feeds the tension/geopolitics layer. **The marquee transmission chain: gas shock → fertilizer → food price → unrest in MENA/SSA importers.**

---

## Sources (selected)
- USDA ERS, *Food Dollar Series* (2024) — farm share 11.8c.
- Our World in Data, *How many people does synthetic fertilizer feed?*; Smil; Nature Geoscience (2008) — ~half world fed by synthetic N.
- USDA ERS *Impact of Rising Natural Gas Prices on U.S. Ammonia Supply*; farmdoc daily / U. Illinois (2022); EIA Today in Energy #52358 — gas→ammonia elasticity ~0.8, 2022 price shock.
- FAO/USDA WASDE (2024/25) — cereal production/yield/area.
- UNESCO WWDR (2024); PMC 2025 critique — agriculture 70% (45–90%) of freshwater.
- Nature *Scientific Reports* (2025) — wheat −6.1%/°C; OWID crop-yields-climate — maize −22%/wheat −14% BAU.
- EU JRC PESETA IV (2020) — EU 2050 maize/wheat projections.
- GFI (June 2025 meta-analysis; cultivated-meat cost analyses); Gourmey (2025); Mosa Meat; ReThinkX *Food & Agriculture* (2019/2023).
- Lagi, Bertrand & Bar-Yam / NECSI (2011), *The Food Crises and Political Instability in North Africa and the Middle East* (arXiv:1108.2455) — FFPI 210 threshold.
- Bellemare (2015), *Rising Food Prices, Food Price Volatility, and Social Unrest*, AJAE — level (not volatility) drives unrest.
- ISPI / Middle East Council (2024–26) — MENA import dependence; FAO Food Price Index (2025–26 levels).
- foodlore.blog / CEAg World (2024–25) — vertical-farm bankruptcies & capex.
- MarketsandMarkets, GM Insights, Omdena, Farmonaut (2025–26) — ag-robotics market & precision-ag input savings.
