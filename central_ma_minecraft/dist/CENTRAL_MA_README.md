# Worcester · Lake Quinsigamond · Shrewsbury — 1:1 Minecraft (extended)

One **continuous** Minecraft Java 1.20.4 world, 1 block = 1 meter.
Extent: **14.2 km (E–W) × 9.2 km (N–S) = 131 km²** (extended +6.4 km east,
+3.2 km south from the original).

## Download + reassemble (split to stay under GitHub's 100 MB limit)
Files: `CentralMA_WQS_extended.zip.part00`, `.part01`, `.part02`
```bash
cat CentralMA_WQS_extended.zip.part* > CentralMA_WQS_extended.zip
unzip CentralMA_WQS_extended.zip      # -> world folder, drop into saves/
```
(Windows PowerShell: `cmd /c copy /b CentralMA_WQS_extended.zip.part00+CentralMA_WQS_extended.zip.part01+CentralMA_WQS_extended.zip.part02 CentralMA_WQS_extended.zip`)

## Data
- Terrain: **USGS 3DEP 1‑meter** LiDAR elevation.
- Water: **USGS NHD** polygons (exact shorelines + islands) — 364 water bodies.
- **Real bathymetry**: MassWildlife (MassGIS) depth‑sounder contours; 58 surveyed
  water bodies use true measured depth (Lake Quinsigamond 0–85 ft / 0–26 m),
  interpolated between contours and merged with shore terrain. Others use a
  distance‑to‑shore model. Every lake has a flat surface + a real depth basin.

Validated against satellite imagery (MAP_overview_extended.jpg,
compare_extended.jpg: top = satellite, bottom = generated world).
