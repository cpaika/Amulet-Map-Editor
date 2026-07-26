# China Political Economy — Parameters for the AI/Robotics Transition Model (2026–2050)

**Purpose.** Structural + quantitative inputs to represent a "China bloc" as a distinct actor vs a US/EU baseline in the systems-dynamics model. Every number carries a source + year. The core thesis for the model: China's comparative advantage in the automation transition is **physical throughput** (energy, robots, grid, manufacturing), while its comparative *disadvantage* is **frontier compute** and its comparative *fragility* is **demographics + property/debt + a political system with no democratic release valve**. These pull in opposite directions and the model should let them.

---

## 1. State Capacity & Industrial Policy

**How fast can China direct capital.** State-owned enterprises account for ~44% of fixed-asset investment (private ~56%), and SOE investment is explicitly *counter-cyclical* — it expands during downturns as a policy tool (CKGSB / MacroMicro, 2024). This is the key structural fact: China can redirect a large investment bloc by administrative fiat on a 1–2 year cycle, without waiting for market price signals or legislative appropriation. The US/EU direct capital via subsidy + private response (IRA, CHIPS, EU Green Deal) on a 3–7 year lag.

**Guided-fund machinery.** ~800 government-guided industrial funds worth ~RMB 2.2 trillion (~$300bn) are tied to Made in China 2025 sectors (PIIE / ORCA, 2019–2025). "New quality productive forces" (新质生产力, Xi's 2023–24 framing) is the current umbrella — AI, robotics, EVs, batteries, biomanufacturing, commercial space — funded through the same guided-fund + SOE + local-industrial-park channel.

**Made in China 2025 — actual outcomes (mixed, not a clean win):**
- China-based firms captured ~25% of *global* export growth in the ten MIC2025 sectors, 2015–2023 (USCC, Nov 2025). Dominance in EVs, batteries, solar, drones, telecom.
- **But**: little statistical evidence of TFP improvement, and China lagged in IP-intensive, high-upfront-cost processes (leading-edge logic, aero engines, advanced materials) (NBER w30676; USCC 2025). Subsidies flowed to targeted listed firms without measurable productivity/R&D/patent gains.
- Interpretation for the model: China's industrial policy is **high-conversion for capital-goods scale-up (robots, solar, grid), low-conversion for frontier IP-frontier tech**. Model as a *sector-dependent* policy multiplier, not a scalar.

**Robot install base — the flagship number:**
- 295,000 industrial robots installed in China in 2024 = **54% of the world's 542,000** (IFR World Robotics 2025).
- Operational stock: **2,027,000 units** (>50% of global stock).
- Domestic robot makers took **57% of the home market in 2024** (up from 47% in 2023) — import-substitution succeeding in real time.
- Asia = 74% of global installs vs Europe 16%, Americas 9% (IFR 2025).

---

## 2. Energy Buildout — the central advantage

This is the load-bearing parameter for an AI/robot transition, because both frontier training and a humanoid-robot fleet are electricity-bound.

**2024 additions (China):**
- Net new grid capacity: **429 GW in one year** (+21% YoY), of which wind+solar = 356.5 GW (83%) (Electrek / CEF, Jan 2025).
- Solar: **+277 GW** (+45.2%) → 887 GW total. Wind: +80 GW (+18%) → 521 GW.
- Coal: **94.5 GW of new coal construction *started* in 2024** (10-yr high) — China = 93% of global coal construction starts (Carbon Brief / CREA-GEM, 2025). Coal is the *firming/baseload* leg, not being retired.
- Nuclear: **36 reactors under construction, +38.9 GW**; capacity +76% (24 GW) 2016→2024; multiple roadmaps to ~200 GW by 2035 and ~400–500 GW by 2050 (EIA 2025; World Nuclear Association; Nuclear Business Platform).
- Grid transmission capex: RMB 608bn (~$84.7bn) in 2024, +15% YoY (CEF).

**Why ~5–10x faster than US/EU (absolute):**
- China added 429 GW of net capacity in 2024; the US added ~50 GW (EIA). Absolute buildout ratio ≈ **8x**. China wind+solar (356 GW) vs US wind+solar (~50 GW) ≈ **7x**.
- Structural reasons: two vertically-integrated grid SOEs (State Grid + Southern Grid) that can site, permit, and build transmission without NIMBY veto or multi-year FERC/interconnection queues; captive domestic supply chain (China makes ~80% of global solar modules, ~75% of batteries); permitting measured in months not years.
- Met Xi's 2030 target of 1,200 GW wind+solar **~6 years early** (in 2024). Total installed ~1.4 TW of a global ~4.5 TW (~⅓ of world).

**Electricity price (industrial, 2024):** China **€0.082/kWh**, US €0.075, EU **€0.199** (Prognos / BusinessEurope, 2024). China business rate rose to ~11.6 ¢/kWh by April 2026 from 8.7 ¢ in June 2024 (Statista) — still ~half the EU. For the model: **China industrial power ≈ 0.4–0.5x the EU price, roughly at par with US, but at ~7–8x the deployable *volume*.** Volume, not price, is the differentiator.

---

## 3. Demographics — why robots are a necessity, not a choice

- Working-age population: **984M (2024) → 745M (2050) = −239M (~−24%)** (UN / Visual Capitalist, 2024). Peaked in 2015.
- Total population: entered decline in 2022; UN median → **~1.26bn by 2050**.
- Aging: 60+ share **22% (2024) → 34.9% (2050)** (China NBS). ~40% over 60 and only ~10% under 15 by 2050.
- Care demand explosion: the 80+ cohort and a 10M+ eldercare-worker gap make China the earliest mass market for service/humanoid robots (couples directly to the model's `care_demand_m` / `care_gap` stock in `demography.rs`).

**Model consequence:** China's automation drive faces a *shrinking* labor denominator, so robots substitute for **vanishing** rather than **employed** workers over 2026–~2035 — displacement-to-backlash conversion is *lower* in the early window (robots fill vacancies) and only turns political later. This is the mirror image of the West's aging profile but far steeper. Treat China's `vacancy_gap(t)` as ~2–3x the EU's on a working-age-decline basis (China −5→−10M/yr working-age vs EU −2→−2.5M/yr).

---

## 4. Property / Debt — the fiscal constraint

- LGFV (local-government financing vehicle) debt ≈ **46% of GDP at end-2023**, estimated **>RMB 60 trillion by end-2024**; official local-government debt ~RMB 48 trillion (IMF Country Report 25/100; IW Köln 2025).
- Land-sale revenue historically ~40% of local government revenue; land transfer revenue fell ~23% in 2022 and stayed depressed — a structural hole in local fiscal capacity (IMF 2024).
- LGFVs as a group have **negative operating cash flow** — borrowing to pay interest ("extend and pretend"), a classic balance-sheet-recession signature (Atlantic Council, 2024).
- Property was ~25–30% of GDP at peak (incl. upstream/downstream); the drag from its contraction is the main reason 2023–2026 domestic demand is weak (Koo-style balance-sheet recession: private sector deleveraging, saving despite low rates).

**Fiscal space for AI/robot subsidy — nuanced:**
- *Local* fiscal space is largely exhausted (the debt above).
- *Central* government debt is comparatively low (~24% of GDP official; **augmented/general-government debt incl. LGFVs ≈ 110–120% of GDP**, IMF 2025) — so Beijing retains sovereign balance-sheet room to fund national champions and the grid, and is doing so (RMB 10tn 2024 debt-swap package to refinance hidden local debt).
- Model this as **bifurcated fiscal space**: high central capacity to fund *supply-side* industrial/energy pushes, low local capacity for *demand-side* transfers/UBI-type absorption of displaced workers. This asymmetry is important — China can build the robots faster than it can cushion the people the robots displace.

---

## 5. AI Strategy — the compute chokepoint + the open-weight/energy counter

**Compute under export controls (the binding constraint):**
- Frontier training gap is real. Huawei Ascend 910C (on SMIC's N+2 "7nm" DUV process) ≈ **⅓ of Nvidia B200 BF16 throughput**; ~60% of an H100 for *inference* per DeepSeek's own eval (2025). Training at scale still reverts to Nvidia (iFlytek reported a 3-month delay switching to Ascend 910B).
- SMIC stuck at 7nm-class without EUV; yields/volume constrained. Huawei targeted ~600k Ascend units in 2025.
- April 2025 H20 ban removed the last legal Nvidia part for China; June 2026 China mandated domestic chips in state-funded data centers (~$295bn program) — hard bifurcation of the compute stack.

**The counter-strategy (why the gap may matter less than headline):**
- **DeepSeek** R1 (Jan 2025): GPT-4-class at ~1 order of magnitude lower training cost, triggered a ~$600bn one-day Nvidia drawdown. Open-weight release → China exports *capability* globally without exporting chips, and undercuts closed-model margins.
- **Energy substitutes for chip efficiency**: if Ascend is 3x less efficient per chip but China has ~7–8x cheaper-to-deploy power at scale, China can partially brute-force the inference/deployment layer even while lagging at the training frontier. This is the key coupling — Section 2 partially offsets Section 5.
- **Data + surveillance feedback**: state access to population-scale data + a surveillance apparatus (facial recognition, social-credit-adjacent systems) gives a domestic RL/deployment loop the West lacks, and doubles as the *suppression* tooling in Section 6.

**Model consequence:** China compute = **~0.3–0.5x US frontier-training capacity through ~2030** (export-control-limited), but **~0.7–0.9x on deployment/inference** and **~1.0x+ on open-weight diffusion**. Frontier lead is a US/West edge; deployment/embodiment is a China edge.

---

## 6. Political Structure — durability vs brittleness, and the missing release valve

- Single-party durability: high state capacity for **suppression + narrative control** means short-run resilience to unrest is much higher than in a democracy — no electoral mechanism forces a policy reversal.
- **But the same feature removes the release valve.** In the model's terms (`society.rs`): the West routes displacement pressure through elections → transfers/UBI/restriction (a pressure-relief channel). China has **no transfer-legitimacy election channel**; the CCP's social contract is *growth-and-jobs for political quiescence*. Automation-driven unemployment therefore has only two exits: (a) **suppression** (feasible near-term, cheap given surveillance stack) or (b) a **legitimacy crisis** if growth+jobs both fail simultaneously. There is no gradual electoral pressure-release, so the political response is **more stable in the short run but more discontinuous (tail-heavy) in the long run** — model as low baseline backlash sensitivity with a rare high-severity regime-shift tail.
- Mitigants Beijing controls: nationalism as legitimacy substitute for growth; targeted repression; ability to *slow* automation deployment administratively if unrest rises (a lever the West lacks).
- **Taiwan risk timeline:** "Davidson window" = PLA *readiness* by 2027 (100th PLA anniversary); 2035 = modernization complete; 2049 = "world-class military." Crucially, ODNI's 2026 Annual Threat Assessment states China has **no fixed timeline** and is **not planning a 2027 invasion** — 2027 is a capability, not a decision, date (USNI/ODNI 2026). For the model: treat Taiwan as a **low-annual-probability, high-impact shock** whose hazard rate rises if (i) US frontier-AI/mil lead widens sharply, or (ii) domestic legitimacy crisis makes external diversion attractive. A Taiwan conflict is also the single largest tail risk to *global* compute (TSMC) — it couples China's political stress directly to the world's chip supply.

---

## 7. Model Parameters — "China bloc" vs US/EU baseline

Values are **relative multipliers vs a US/EU baseline = 1.0** unless noted. Ranges reflect genuine uncertainty; the point estimate is the suggested plug-in.

| # | Parameter | US/EU baseline | China bloc | Basis (source, year) |
|---|-----------|---------------|-----------|----------------------|
| P1 | **Industrial-policy intensity** (0–1) | 0.35 | **0.85** | SOE = 44% of FAI + RMB 2.2tn guided funds + counter-cyclical state investment (CKGSB/PIIE 2024–25). *Sector-split*: ~0.9 for capital goods/energy, ~0.5 for frontier-IP tech (MIC2025 mixed record, USCC 2025). |
| P2 | **Energy-buildout-speed multiplier** (vs US, absolute GW/yr) | 1.0 | **7x** (range 5–9x) | 429 GW net added 2024 vs US ~50 GW; wind+solar 356 vs ~50 GW; grid SOEs + captive supply chain (Electrek/EIA 2024–25). Per-capita ≈ 1.7x; use absolute for AI/robot power. |
| P3 | **Robot-adoption speed multiplier** | 1.0 | **2.5x** (range 2–3x) | 54% of global installs, 2.03M stock, 57% domestic-maker share, demographic pull (IFR 2025). |
| P4 | **Industrial electricity price** (relative cost) | 1.0 (US); EU ≈ 2.5 | **~1.0** (≈ US, ≈ 0.4x EU) | China €0.082 vs US €0.075 vs EU €0.199 /kWh, 2024 (Prognos). Advantage is *volume* (P2), not price. |
| P5 | **Frontier-compute access** (training) | 1.0 | **0.4x** (range 0.3–0.5x) | Ascend 910C ≈ ⅓ B200; SMIC 7nm ceiling; H20 ban 2025. *Inference/deployment* ≈ 0.8x; *open-weight diffusion* ≈ 1.0x+ (DeepSeek 2025). |
| P6 | **Fiscal space for displacement transfers** (demand-side cushion) | 1.0 | **0.4x** | Local fiscal exhausted (LGFV >60tn RMB, ~46%+ GDP); central room exists but is aimed supply-side, not at UBI (IMF 25/100). *Supply-side* fiscal space ≈ 1.3x. |
| P7 | **Political-backlash sensitivity** to displacement (short-run gain on unrest→policy reversal) | 1.0 | **0.3x baseline, with a fat regime-shift tail** | No electoral release valve; surveillance-state suppression capacity (Section 6). Low routine sensitivity, rare high-severity discontinuity. |
| P8 | **Working-age decline / vacancy-gap** (labor denominator shrink) | 1.0 (EU ≈1; US <1) | **2.5x** | China −5→−10M/yr working-age vs EU −2→−2.5M/yr; 984M→745M by 2050 (UN 2024). Robots fill vacancies first → *delays* early displacement backlash. |

**One-line encoding of the China bloc:** *builds physical capacity (energy P2≈7x, robots P3≈2.5x) far faster than it can either reach the compute frontier (P5≈0.4x) or cushion the displaced (P6≈0.4x, P7 low-but-tail-heavy), against a demographic clock (P8≈2.5x) that makes automation compulsory and a debt overhang that forces the whole push through the central-government/SOE balance sheet.*

---

## Sources
- IFR World Robotics 2025 — [ifr.org](https://ifr.org/worldrobotics/report-2025); [Xinhua](https://english.news.cn/20250926/0e0d8b4348b64c36951cc1c000498d38/c.html)
- China 2024 energy — [Electrek](https://electrek.co/2025/01/21/china-solar-wind-2024/); [Climate Energy Finance](https://climateenergyfinance.org/); [Carbon Brief coal](https://www.carbonbrief.org/chinas-construction-of-new-coal-power-plants-reached-10-year-high-in-2024); [EIA nuclear](https://www.eia.gov/todayinenergy/detail.php?id=67746)
- Electricity prices — [Prognos](https://www.prognos.com/en/projekt/energiepreise-industrie-internationaler-vergleich); [BusinessEurope](https://www.businesseurope.eu/media-room/data-hub/high-cost-of-energy/)
- Demographics — [Visual Capitalist / UN](https://www.visualcapitalist.com/charted-india-vs-china-working-age-populations-2024-2050/); China NBS
- Debt/property — [IMF Country Report 25/100](https://www.imf.org/-/media/files/publications/cr/2025/english/1chnea2025001-print-pdf.pdf); [IW Köln 2025](https://www.iwkoeln.de/); [Atlantic Council](https://www.atlanticcouncil.org/blogs/econographics/beijing-extends-and-pretends-to-deal-with-its-mountain-of-local-government-debt/)
- AI/chips — [SemiconductorX Huawei/SMIC](https://semiconductorx.com/spotlight-huawei-hisilicon.html); DeepSeek R1 coverage (2025)
- Industrial policy / MIC2025 — [USCC Nov 2025](https://www.uscc.gov/sites/default/files/2025-11/Made_in_China_2025--Evaluating_Chinas_Performance.pdf); [NBER w30676](https://www.nber.org/system/files/working_papers/w30676/w30676.pdf); [PIIE guided funds](https://www.piie.com/blogs/china-economic-watch/2019/government-guided-funds-china-financing-vehicles-state-industrial); China NBS FAI 2024
- Taiwan — [USNI / ODNI 2026](https://news.usni.org/2026/03/19/china-not-committed-to-2027-taiwan-invasion-u-s-intel-report-says); [Lowy Institute](https://www.lowyinstitute.org/the-interpreter/china-taiwan-pla-s-2027-milestones)
