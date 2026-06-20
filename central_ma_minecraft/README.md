# Central Massachusetts → Minecraft (1:1 scale)

A from-scratch pipeline that turns real, high-precision topographic + aerial
data into an openable Minecraft Java world at **1 block = 1 meter** scale.

Area of interest: **Worcester**, **Lake Quinsigamond**, **Shrewsbury**,
**Princeton**, and **Mount Wachusett** in central Massachusetts.

## Data sources (highest-precision public data for MA)

- **Elevation:** USGS **3DEP 1-meter** Digital Elevation Model, pulled live from
  the National Map `3DEPElevation` ImageServer as 32-bit float GeoTIFF tiles.
  This is the finest-resolution bare-earth/terrain data publicly available for
  Massachusetts (derived from statewide LiDAR).
- **Land cover / color:** Esri **World Imagery** (sub-meter aerial), used both to
  color the surface blocks and to verify the result against satellite imagery.

## How it works

1. `geo.py` — Web-Mercator transforms + tiled raster download. Mercator inflates
   distance by `1/cos(lat)` (~1.35× at 42°N), so the request box is scaled to
   sample **true 1 m/pixel** ground resolution.
2. `classify.py` — turns aerial RGB into surface classes (water, forest, grass,
   road, building, sand, bare soil) → Minecraft blocks.
3. `mc_nbt.py` / `mc_anvil.py` / `mc_level.py` — a from-scratch modern (1.18+)
   Anvil writer: NBT serializer, vectorized non-spanning bit-packed
   `block_states`, region (`.mca`) files, and a `level.dat` with a void flat
   preset so authored terrain is authoritative.
4. `build.py` — fetches each tile, computes a per-tile vertical datum, and writes
   blocks at their **true relative world coordinates** (1 block = 1 m east/north).
   Unauthored space between tiles costs no disk.
5. `render.py` — reads the generated `.mca` back, hillshades it, and stacks it
   under the satellite image for visual QA.

## Build it

```bash
pip install numpy pillow nbtlib
python3 build.py spec_central_ma.json out/central_ma_world
python3 render.py out/central_ma_world spec_central_ma.json 0 compare_0.png
```

Open `out/central_ma_world` as a Minecraft Java 1.20.4 save.

## Scale / scope notes

- The world is true 1:1 horizontally (1 block = 1 m) and 1:1 vertically within
  each tile (a per-tile datum keeps tall terrain like Mt Wachusett, 611 m, inside
  the −64..319 build range; tiles are separated by void so this is seamless).
- The **fully continuous** 707 km² circle covering everything between the towns
  is ~12 GB of region files (~1.4 GB zipped) — too large to ship through this
  channel. The same pipeline generates it: just widen the tiles in the spec.
  The shipped world authors the named towns + landmarks at full 1:1.
