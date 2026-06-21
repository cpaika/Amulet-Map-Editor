# Worcester · Lake Quinsigamond · Shrewsbury — 1:1 Minecraft

`CentralMA_Worcester_Quinsigamond_Shrewsbury.zip` — one **continuous** Minecraft
Java 1.20.4 world (no gaps), 1 block = 1 meter. Spawn is mid-map.

Data:
- Terrain: **USGS 3DEP 1‑meter** LiDAR elevation.
- Water bodies: **USGS NHD** polygons (exact shorelines + islands) — Lake
  Quinsigamond's 8 islands included.
- **Real bathymetry**: MassWildlife (MassGIS) GPS/depth-sounder contour lines.
  Lake Quinsigamond is modeled to its true **0–85 ft (0–26 m)** surveyed depth,
  interpolated between contours (Laplace) and merged with the shore terrain.
  51 surveyed water bodies in-frame use real depth; the rest use a
  distance-to-shore model.

Each lake has a single flat water surface with a real depth basin, so it reads
as actual water rather than a skin on the noisy DEM. Validated against satellite
imagery (compare_lake.jpg, compare_lake_islands.jpg: left = satellite,
right/bottom = the generated world).

Supersedes the earlier two-patch CentralMassachusetts_1to1.zip (which had a gap
between Worcester and Princeton and no real lake).
