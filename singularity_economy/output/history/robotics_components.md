# Robotics Component Supply Chain — Deep Dive (overnight research fleet)

Compiled from a ~15-agent research sweep, 2026-07-26. Figures tagged to source
where traced; sell-side/market-research numbers flagged as directional. This is
the empirical basis for `rust/singularity-econ/src/materials.rs`.

## Bottom line up front

**The binding physical chokepoints for scaling humanoids 100–1000× are precision
reducers and rare-earth (NdFeB) magnets — NOT batteries, compute, or structure.**
Both are dodgeable by superintelligent substitution (cycloidal/QDD transmissions;
RE-free motors), which is exactly why the model's aggressive-substitution baseline
shows materials going slack by the mid-2030s.

## BOM structure (McKinsey 2025; Morgan Stanley; 36Kr)

| Subsystem | % of BOM | Notes |
|---|---|---|
| Actuators (motor+reducer/screw+encoder+sensor+driver) | **40–60%** | Optimus ref 48.8% (linear 28.9 + rotary 19.9); BofA 2030 ~51% |
| — reducers (within actuator) | ~15% (~$3k at $20k target) | 30–50% of actuator cost |
| — roller/ball screws | ~18% (~$3.6k) | linear actuators |
| — motors | ~24% of BOM | frameless torque + coreless |
| Sensing / perception | 10–20% | 6-axis F/T is the expensive part |
| Compute / control | 10–15% (→21% by 2031) | leading-edge SoC island |
| Battery & power | 5–12% | NOT a bottleneck |
| Structure / housing | 5–10% | except flexspline steel |

Unit cost: **$250k (2022) → $150k (2023), −40% in one year** (Goldman); Tesla target
**$20k**, Unitree G1 **$13.5k** today. China BOM ~$46k vs ex-China ~$131k (MS).
Actuator count: 28 body (14 rotary + 14 linear) + ~50 hand ≈ 78; ~40 motors.

## Epoch AI component ceilings (VERIFIED — the core numbers)

Source: Epoch AI, "How Fast Could Robot Production Scale Up?" (2026).

| Component | Global capacity | Per robot | Humanoids/yr ceiling | Binding? |
|---|---|---|---|---|
| **Planetary reducers** | ~3M/yr | 6 | **500,000** | **YES** |
| **Cycloidal / RV reducers** | ~2M/yr | 4 | **500,000** | **YES** |
| Strain-wave (harmonic) | ~12M/yr | many small | ~1,000,000 | 6× headroom |
| Ball / roller screws | — | 14 | ~1.5–2.5M | secondary |
| Torque sensors / encoders | — | 4 / 40–60 | ~1.25M | secondary |
| Servo motors | — | ~40 | ~4.5M | no |
| Batteries | ~2 TWh/yr | 2.3 kWh | >800M | **no** |
| Cameras / MEMS / bearings | billions/yr | — | — | no |

Nuance: the popular "harmonic caps 500k" is imprecise — **planetary and RV** reducers
are the tightest links; harmonic has ~6× more headroom. Physical cap = precision gear
grinding (Reishauer/Kapp/Gleason machines), fatigue/maraging flexspline steel, and
skilled metrology labor — none scale faster than ~2×/2yr per tranche.

## Rare-earth magnets — the EXTREME long-run chokepoint

- **3.5–4 kg NdFeB per humanoid** (~2× an EV); ~1.3 kg NdPr.
- China: **~85–90% magnets, ~90% separation/refining, ~69% mining**.
- Global high-perf NdFeB ~100kt/yr (~300–345kt total). **10B robots ≈ 186× current
  NdFeB output** vs copper 3×, lithium 14× — by far the worst-scaling input.
- Export controls already live (Apr-2025 magnet + Dy/Tb/Sm licensing; Dy $700→$1,100/kg,
  Tb $2,000→$4,000/kg).
- **Relief:** RE-free motors (Tesla PMa-SynRM; BMW/Renault/ZF wound-rotor shipping),
  ferrite / iron-nitride (Niron Fe₁₆N₂ ~2026) can design out **70–90%** of RE-in-motor
  demand in 2–5 yr — this is how the 186× is dodged.

## Ranked chokepoints for 100–1000× scaling

1. **Precision reducers** (planetary + RV) — 500k/yr today, grinding-capacity gated.
2. **NdFeB magnets** — 90% China, 186× needed, export-controlled; substitutable.
3. **6-axis force/torque sensors** — ~$710–1,420 ea, APAC-concentrated, ~100–300×.
4. **Roller/ball screws** — narrow base (NSK/THK/Rollvis), internal-thread grinding.
5. **Leading-edge inference SoC** — TSMC 4nm/Samsung 7nm, ~0% China, Taiwan-concentrated.
6. Copper / bulk — elastic but 16–30yr mine lead times; the LATE binding input.

Batteries, cameras, MEMS, MCUs, structure = not bottlenecks.

## Model mapping (materials.rs Liebig inputs)

precision_reducers 0.5M · torque_sensors_encoders 1.25M · ball_screws 2.5M ·
rare_earth_magnets 12M (China 90% embargo) · inference_chips 5M · copper 8M.
Each relieved by ASI substitution (ceiling 40–85%, 2–5yr design-out) and threatened
by a China embargo (metals-shock → china_share cut). Robot output = Liebig minimum.
