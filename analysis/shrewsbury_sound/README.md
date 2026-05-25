# How loud is 6 Trowbridge Circle vs. the rest of Shrewsbury, MA?

A physics-based **estimate** of outdoor noise. It is not a measurement — it
models the dominant outdoor noise source in a suburb (road traffic) and
compares the target address to representative locations around town.

## TL;DR

6 Trowbridge Circle is **one of the quietest spots in Shrewsbury**, with a
modelled daytime traffic Leq of about **44 dB(A)** — essentially the suburban
ambient floor. It is a cul-de-sac whose only real nearby source is **Main
Street, ~154 m away**; every state highway is 1–3 km off.

| Location | Leq dB(A) | vs. Trowbridge |
|---|---:|---|
| Edgemere (residential, ~500 m off Rt 20) | 44.0 | about the same |
| **6 Trowbridge Circle (target)** | **44.4** | reference |
| Sherwood Ave (mid-town residential) | 45.2 | about the same |
| Jordan Rd (Fairlawn, near lake) | 48.8 | +4 dB (~1.4× louder) |
| Shrewsbury Town Hall / center | 51.3 | +7 dB (~1.6× louder) |
| Reservoir St (~280 m from I-290) | 51.9 | +7 dB (~1.7× louder) |
| Harrington Ave (~200 m off Route 9) | 64.3 | +20 dB (~4× louder) |
| Home fronting Route 9 (25 m) | 67.0 | +23 dB (~4.8× louder) |
| Quinsigamond Ave (lakeside, by Route 9) | 67.4 | +23 dB (~4.9× louder) |
| Home along I-290 (60 m) | 67.5 | +23 dB (~5× louder) |
| Grafton St (fronting MA-140) | 69.7 | +25 dB (~5.8× louder) |

Reading the scale: every **+10 dB ≈ twice as loud** to the ear. A home right
on Route 9, MA-140, or I-290 sounds roughly **4–6× louder** than Trowbridge
Circle. The town center is mildly louder (+7 dB). Other deep-residential
pockets (Edgemere, Sherwood Ave) are about the same as Trowbridge.

## Method

* **Geometry** — every road within 8 km is pulled from OpenStreetMap
  (`osm_roads.json`). Each source→receiver distance is the true nearest
  distance to that road's polyline, so the comparison is driven by real
  layout, not guesses. 6 Trowbridge Circle = 42.2940843, -71.7007366.
* **Source emission** — CoRTN basic noise level (UK DoT, *Calculation of Road
  Traffic Noise*, 1988): `L10(1h)@10m = 42.2 + 10·log10(q) + speed/heavy
  corrections`, converted to Leq (≈ L10 − 3).
* **Propagation** — soft-ground line-source spreading (~4.5 dB per distance
  doubling = `15·log10(d/10)`), air absorption (~5 dB/km, A-weighted), and a
  single 5 dB built-up screening allowance for roads > 150 m away (blocked by
  intervening houses/trees).
* **Combination** — all roads energy-summed, plus a 42 dB(A) suburban ambient
  floor.

## Caveats (why the absolute numbers are ±3–5 dB)

* **Traffic volumes are typical-by-class, not measured.** Exact MassDOT AADT
  counts weren't pulled; values are typical for each road class (I-290 ≈ 78k,
  MA-9 ≈ 33k, US-20 ≈ 18k, MA-140 ≈ 16k AADT). This shifts absolute levels but
  the *ranking* is dominated by distance and is robust.
* Terrain, real barriers/walls, building reflections, wind/temperature, and
  non-road sources (rail, lawn equipment, the nearby commercial strips) are
  modelled only crudely or not at all.
* "Fronting" receivers that geocoded onto a road centerline slightly overstate
  the level for a set-back home.
* This is a daytime average. Nights are quieter everywhere; the *relative*
  picture holds.

## Run it

```bash
python3 sound_model.py
```

No dependencies beyond the Python 3 standard library. Edit `RECEIVERS` or the
traffic tables (`BY_REF` / `BY_NAME` / `BY_HIGHWAY`) in `sound_model.py` to
try other locations or plug in real AADT counts.

## Sources

* Road geometry: OpenStreetMap (© OpenStreetMap contributors, ODbL) via the
  Overpass API. Geocoding via Nominatim.
* Emission/propagation: CoRTN (UK Dept. of Transport, 1988); soft-ground
  line-source attenuation rule of thumb (~4.5 dB/doubling).
* Context on local highways (MA-9, US-20, I-290 Lake Quinsigamond crossing,
  MA-140) and the Trowbridge neighborhood: USDOT National Transportation Noise
  Map, MassDOT, and local listings.
