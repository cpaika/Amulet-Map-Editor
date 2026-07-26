# Raw Materials & Mines Behind Humanoid Robots and the AI/Robotics Buildout

**Purpose:** Hard-number reference for modeling a robot self-replication crunch. Focus on raw-material chokepoints for scaling humanoid robots + AI infrastructure 100–1000x. All figures carry source + year. Where sources conflict, ranges are given.

**Headline framing (Adamas Intelligence, 2025):** Building 10 billion humanoid robots would require ~600 million tonnes of input materials (mostly metals) and would demand roughly **186x** current NdFeB magnet output, **14x** lithium, **13x** graphite, **8x** cobalt, **4x** nickel, and **4x** copper. Adamas calls the full 10B scenario "almost certainly impossible." Even scaling 100–1000x (0.1–1B robots) blows through several material ceilings — magnets first.

---

## 1. RARE EARTHS — NdFeB Permanent Magnets (Nd, Pr, Dy, Tb)

**Use in robots/AI:** NdFeB (neodymium-iron-boron) sintered permanent magnets are the core of torque-dense electric actuators/motors in every humanoid joint (20–40 actuators/robot), plus voice-coil sensors, haptics, and cooling fans in AI servers. NdPr (neodymium-praseodymium) is the primary magnetic driver; **dysprosium (Dy)** and **terbium (Tb)** are added at grain boundaries to raise coercivity / high-temperature resistance. Note: humanoid motors are small, intermittently loaded, actively cooled, and run near ambient — so many designs use high-remanence NdPr grades with little/no Dy-Tb, but high-torque or high-temp designs still need heavy REEs.

**kg per humanoid robot:**
- NdFeB magnet content: **~2–4 kg** (multiple analysts); one widely cited figure ~3.5–4 kg (roughly 2x an EV traction motor).
- Contained NdPr: **~1.3 kg** (Morgan Stanley estimate across motors/actuators).
- Dy/Tb: tens of grams if used (grade-dependent).

**Global mine production 2024 (USGS, REO equivalent):** ~**390,000 t** (up ~4% YoY).
- **China: 270,000 t (69.2%)** — production quota-controlled.
- **USA: ~45,000 t (11.5%)** — entirely MP Materials **Mountain Pass, California** (bastnäsite).
- **Australia:** Lynas **Mt Weld** (WA) — one of the highest-grade REE deposits; feeds Lynas processing in Malaysia + new Kalgoorlie plant.
- Myanmar, Thailand, Nigeria — rising heavy-REE / ionic-clay feed (much trucked into China).
- **Bayan Obo (Inner Mongolia, China):** the single largest REE mine on Earth; backbone of China's light-REE supply.
- China reserves: ~44 Mt (≈49% of world).

**Refining/processing — the real chokepoint:**
- **China ≈ 90% of refined/separated REE output** (separation + purification).
- **China ≈ 85–94% of finished NdFeB sintered magnets** (IEA: 94% in 2024; up from ~50% in 2005). ~91% share for defense-grade magnets.
- Outside China, near-zero heavy-REE separation and very little magnet-making capacity exists today (MP Materials, Lynas, Neo/Estonia, Vacuumschmelze scaling but tiny by comparison).

**Prices (2025–2026):**
- Neodymium oxide: ~**$101/kg** (Dec 2025, NE Asia) → **~$146/kg** (Jul 2026, +21%).
- Praseodymium oxide: ~**$108/kg** (Dec 2025).
- Dysprosium oxide: **$353/kg (Jan 2025) → $780/kg (Sep 2025)**, +121% YTD.
- Terbium: **$1,396/kg (Jan 2025) → ~$3,485/kg** resale peak.

**Export controls (escalating — the "China turns off the tap" lever):**
- Jul 2023: Ga/Ge licensing (see §4).
- Aug 2024: antimony dual-use controls.
- Dec 2024: outright ban of Ga/Ge/Sb exports to the US.
- **Apr 2025:** license requirement on 7 REEs — **samarium, gadolinium, terbium, dysprosium, lutetium, scandium, yttrium** — plus NdFeB magnets containing Tb/Dy and SmCo magnets.
- **Nov–Dec 2025:** expanded regime — 0.1% Chinese-origin content threshold, a "50% rule," extraterritorial jurisdiction, and discretionary end-use licensing (Beijing decides who gets supply and how much).

**Supply elasticity:** Very low. New REE mine: ~5+ yrs minimum; US permitting 7–10 yrs. Separation/magnet plants are the harder bottleneck — Western heavy-REE separation and sintered-magnet capacity take years to qualify and scale even after mines open.

---

## 2. COPPER — Motor Windings, Wiring, Grid & Datacenters

**Use:** Motor windings, busbars, battery current collectors, and wiring in robots; power distribution, busways, and cabling in AI datacenters and the grid feeding them.

**kg/tonnes per unit:**
- Per humanoid robot: **~8–15 kg** (8.5 kg in 2025 rising toward 15 kg by 2030; ~4–8 kg in actuators/wiring alone). Robotics copper demand: ~30,000 t (2025) → 80,000 t (2030), some forecasts 380,000–420,000 t by 2030.
- Per AI datacenter: **~50,000 t of copper per 1 GW** (≈27–33 t per MW). Datacenter copper demand: **1.1 Mt (2025) → 2.5 Mt (2040)**.

**Global mine production 2024:** ~**22–23 Mt** (copper content). Chile #1, then Peru, DRC, China.
- **Escondida (Chile, BHP/Rio):** largest mine, ~1.35 Mt/yr capacity.
- **Grasberg (Indonesia, Freeport):** 816,466 t (2024), ~1.1% Cu grade + gold byproduct.
- **Kamoa-Kakula (DRC, Ivanhoe/Zijin):** 437,061 t (2024, +11%); Phase 3 commercial Aug 2024.

**Grade decline:** Structural. Escondida requires a $10.8B desalination + concentrator program to keep processing falling ore grades — emblematic of industry-wide grade erosion driving capex up.

**Price:** ~**$9,000–10,000/t** through 2025; spiked to ~**$13,300–13,550/t** by mid-2026. 2025 refined deficit ~304,000 t (Wood Mackenzie); 2026 deficit ~150,000 t (ICSG).

**Supply elasticity:** Very low. Discovery-to-production averages **15.7–17.9 yrs** (S&P Global) for mines started 2020–23, up from 12.7 yrs a generation ago; non-operating mines now approaching ~30 yrs including permitting. Fundamental mismatch with AI-driven demand curve.

---

## 3. BATTERY METALS — Li, Ni, Co, Mn, Graphite

**Use:** Onboard robot battery packs (Li-ion / LFP / NMC); grid storage for datacenters.

| Metal | 2024 mine production | Top mining | China refining share | Price (2025–26) |
|---|---|---|---|---|
| **Lithium** | **240,000 t** LCE-content (record, +18%) | Australia 88,000 t (**Greenbushes**, hard-rock spodumene), Chile 49,000 t (**Salar de Atacama**), China 41,000 t | China ~65–70% of Li **chemical** refining (most Australian spodumene ships to China) | Depressed vs 2022 peak; oversupplied |
| **Nickel** | **~3.7 Mt** | Indonesia (~52% of mined; **Sulawesi/Morowali** laterite) | China ~firms control >75% of Indonesian refining; Indonesia ~60–70% of refined nickel | ~$17,200/t (Feb 2026); 2025 surplus ~198,000 t |
| **Cobalt** | **~290,000–300,000 t** | **DRC ~76%** (Cu-Co, e.g. CMOC Tenke Fungurume, Kamoto), Indonesia ~10% | **China ~76–77% of refined cobalt** | Co hydroxide $4,012/t (Q4'24) → $14,560/t (Q4'25), +263% (DRC export quota) |
| **Graphite (natural)** | China ~**1.27 Mt** (largest) | China dominant; Mozambique, Madagascar | **China ~96% of refined/spherical graphite** (anode) | China Dec 2023 export licensing |
| **Manganese** | — | South Africa, Gabon, Australia | **China ~95% of refined battery-grade Mn** | — |

**Supply elasticity:** Lithium can scale fastest (2–4 yr brine/spodumene ramps) but chemical refining is China-gated. Nickel/cobalt tied to Indonesia + DRC concentration and Chinese-controlled refining. Graphite anode + battery-grade manganese refining are near-total China monopolies.

---

## 4. GALLIUM & GERMANIUM — Power Electronics (GaN/SiGe)

**Use:** Gallium → **GaN** power semiconductors (efficient motor drivers/inverters in robots, datacenter power supplies) and GaAs RF chips. Germanium → SiGe chips, fiber optics, IR optics, and datacenter interconnects.

**Production 2024:**
- **Gallium:** primary low-purity ~**760 t; China ~750 t (≈99%)**. High-purity refined ~320,000 kg globally. Byproduct of alumina/zinc refining.
- **Germanium:** ~**220 t/yr** globally; market ~$330M (2024). China dominant (byproduct of zinc/coal).

**China dominance:** Gallium **~98–99%**, germanium **~60%+** of primary output, higher in refining.

**Export controls:** Jul 2023 licensing on Ga/Ge; **Dec 2024 outright ban of exports to the US**. Prompted Western emergency stockpiling.

**Prices:** Gallium ~$420/kg (Oct 2025, China low-purity). Germanium >$5,000/kg (2025; ~$4,120→$5,814/kg), reaching $7,000–9,000/kg into 2026.

**Supply elasticity:** Moderate in theory — both are byproducts, so idle Western recovery capacity (e.g. from bauxite/zinc streams) could restart in ~1–3 yrs, but economics depend on China's price behavior. This is a pure geopolitical chokepoint, not a geological one.

---

## 5. SILICON — Polysilicon, High-Purity Quartz, Wafers

**Use:** Silicon = substrate of all logic/memory/power chips and AI accelerators; polysilicon → wafers; high-purity quartz → **crucibles** for Czochralski single-crystal ingot pulling.

**High-purity quartz — the hidden single point of failure:**
- **Spruce Pine, North Carolina** supplies **80–90% of the world's high-purity quartz** (BloombergNEF 2024: ~80% of PV + semiconductor-grade HPQ). Output ~180,000–200,000 t/yr, purity to 99.9999999%.
- Only two operators: **Sibelco** (Belgian) and **The Quartz Corp** (Norwegian-French JV). Hurricane Helene (Sep 2024) flooded the town and briefly halted mining — a genuine near-miss for global chip supply.

**Polysilicon:** **China ~93.5% of global production (2024)**. Top makers: Tongwei (910 kt), GCL (480 kt), Daqo (350 kt), Xinte (300 kt); 9 of top 10 are Chinese.

**Silicon metal:** China ~**70%** of global output. Price ~$2,000–3,060/t (2025).

**Supply elasticity:** Quartz deposits of Spruce Pine quality are extremely rare; alternatives exist (synthetic quartz, other deposits) but requalification is slow. Polysilicon capacity is abundant but Chinese-concentrated. Wafer/fab capacity is the separate TSMC/ASML bottleneck (not covered here).

---

## 6. PGMs, TUNGSTEN, TANTALUM, MAGNET STEELS

**Use:** PGMs (Pt, Pd, Ir, Ru) → sensors, electrical contacts, catalytic/coating, fuel cells; **tungsten** → carbide cutting tools for machining actuator/gearbox parts, ballast, high-density counterweights; **tantalum** → compact high-capacitance capacitors in robot control boards & AI servers; electrical/magnet steels → stator/rotor laminations.

- **PGMs:** South Africa ~**250 t/yr (2024)**, holds world's dominant reserves (Bushveld Complex). Russia (Norilsk) dominant in palladium. Concentration risk: South Africa + Russia.
- **Tungsten:** global **82,000 t (2024)**; **China ~80%+** of mine + processing. On multiple critical-materials control lists.
- **Tantalum:** **DRC ~980 t (2023, ~41%)**, Rwanda ~520 t; DRC+Rwanda+Brazil >50%. Conflict-mineral + supply-security risk (e.g. Rubaya mine).
- **Magnet/electrical steel:** China dominant in grain-oriented electrical steel capacity.

**Supply elasticity:** PGM mines have long lead times + deep-level SA constraints; tungsten and tantalum are China/Central-Africa concentrated with limited near-term Western substitution.

---

## TOP 5 RAW-MATERIAL CHOKEPOINTS FOR SCALING ROBOTS 100–1000x

Ranked by severity (demand multiple × concentration × lead time × substitutability):

1. **NdFeB magnets / heavy rare earths (Dy, Tb) + NdPr.** THE binding constraint. Needs ~186x for 10B robots; China holds ~70% mining, **~90% refining, ~90–94% magnet-making**, and has active export licenses covering exactly Dy/Tb/Sm + finished magnets. *Single points of failure:* Chinese separation/magnet plants; Bayan Obo. *Geopolitical: highest — controls already in force (Apr–Dec 2025).*

2. **Gallium (GaN power electronics) + Germanium.** ~99% / ~60%+ China; **already banned to the US (Dec 2024)**. Small tonnages but no fast Western substitute qualified. *SPOF:* Chinese alumina/zinc byproduct recovery. *Geopolitical: highest — tap already turned off for the US.*

3. **Copper.** Not concentration-limited but **volume + lead-time limited**: needs ~4x for 10B robots on top of a 50,000 t/GW datacenter draw, against 16–18 yr (up to 30 yr) mine lead times and structural grade decline. *SPOF:* Chile (Escondida) + a handful of mega-mines; ~40% of refining is in China.

4. **Battery-metal refining (graphite ~96%, manganese ~95%, cobalt ~77%, lithium ~65%, nickel via Indonesia).** Mining is diversified; **refining is a Chinese monopoly**, plus DRC (cobalt) and Indonesia (nickel) mine concentration. *SPOF:* Chinese anode/refining plants; DRC Cu-Co belt; Indonesian nickel laterites (Chinese-controlled). *Geopolitical: high.*

5. **High-purity quartz (Spruce Pine) + polysilicon.** A ~2-town, 2-company monopoly (80–90% HPQ) for crucible-grade quartz, plus China's ~93% polysilicon share. Low tonnage, near-zero redundancy. *SPOF:* Spruce Pine, NC (single geographic point — nearly proven by Hurricane Helene 2024); Chinese polysilicon fleet.

**Runner-up:** Tungsten (China ~80%, tooling to *build* robots) and tantalum (DRC/Rwanda capacitors).

## "CHINA TURNS OFF THE TAP" — geopolitical single points of failure
- **Already active controls:** Ga/Ge/Sb ban to US (Dec 2024); 7-REE + Dy/Tb/Sm magnet licensing (Apr 2025); extraterritorial 0.1%/50% rules + discretionary end-use licensing (Nov–Dec 2025).
- **Refining monopolies China could weaponize next:** rare-earth separation (~90%), NdFeB magnets (~90–94%), graphite anode (~96%), manganese (~95%), cobalt refining (~77%), lithium chemicals (~65%), tungsten (~80%), gallium (~99%).
- **Non-China physical SPOFs:** Spruce Pine HPQ (NC, USA), Escondida (Chile), DRC cobalt/tantalum belt, Indonesian nickel (Chinese-owned), South Africa/Russia PGMs.

---

### Key sources
- USGS Mineral Commodity Summaries 2025 (rare earths, gallium, germanium, cobalt, nickel, graphite, tungsten, PGM).
- IEA Global Critical Minerals Outlook 2024/2025; IEA Rare Earth Elements report.
- Adamas Intelligence, "The mountains of metals needed for 10 billion humanoid robots" (2025).
- Morgan Stanley / CRU Group humanoid-robot material analyses (2025).
- S&P Global Market Intelligence — mine lead times (15.7–17.9 yr; up to 30 yr).
- MINING.COM top-20 copper mines 2025; Ivanhoe Kamoa-Kakula; Freeport Grasberg.
- China MOFCOM Notices (2023 Ga/Ge; 2024 Sb; 2025 No. 61 REE controls); White & Case, Clark Hill, CSET legal analyses.
- BloombergNEF / Grist / Construction Physics on Spruce Pine HPQ; TaiyangNews / Bernreuter on polysilicon (China 93.5%, 2024).
- Fastmarkets / Argus / Shanghai Metal Market / Strategic Metals Invest — 2025–26 prices.
