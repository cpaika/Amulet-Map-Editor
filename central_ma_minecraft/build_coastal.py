"""Coastal 1:1 Minecraft builder (Cape Cod / Martha's Vineyard / Waquoit Bay).

Differs from the inland builder: uses a GLOBAL sea-level datum (NAVD88 0 m ->
fixed world Y) so coastlines and water line up everywhere, fills ocean/bay with
water to sea level over a shallow seabed, and keeps salt marsh (low but
vegetated) as land instead of flooding it.
"""
import os
import sys
import json
import time
import math
import numpy as np

import geo
import classify as C
from classify import CLASS_BLOCK
from mc_anvil import RegionWriter, build_chunk
from mc_level import write_level_dat

DATA_VERSION = 3700           # 1.20.4
SEA_Y = -50                   # world Y of sea level (NAVD88 0 m)
SEA_DEPTH = 3                 # shallow seabed below sea level for visible water
POND_DEPTH = 2
SEA_T = 0.5                   # elev (m) <= this is water
WORLD_BOTTOM = -64
TOP_Y = 319

GLOBAL_BLOCKS = ["minecraft:air", "minecraft:stone", "minecraft:dirt",
                 "minecraft:sand", "minecraft:gravel"]
AIR, STONE, SUBDIRT, SEABED_SAND, GRAVEL = 0, 1, 2, 3, 4
CLASS_TO_GID = {}
for cid in range(7):
    CLASS_TO_GID[cid] = len(GLOBAL_BLOCKS)
    GLOBAL_BLOCKS.append(CLASS_BLOCK[cid])
WATER_GID = CLASS_TO_GID[C.WATER]

# project origin = Cape Cod Canal area (true-meter reference, shared by all tiles)
ORIGIN_LON, ORIGIN_LAT = -70.50, 41.74
ORIGIN_MX, ORIGIN_MY = geo.lonlat_to_merc(ORIGIN_LON, ORIGIN_LAT)
COSLAT = math.cos(math.radians(ORIGIN_LAT))


def tile_world_origin(center_lon, center_lat, w_m, h_m):
    cmx, cmy = geo.lonlat_to_merc(center_lon, center_lat)
    merc_w = w_m / COSLAT
    merc_h = h_m / COSLAT
    nw_mx = cmx - merc_w / 2.0
    nw_my = cmy + merc_h / 2.0
    X0 = int(round((nw_mx - ORIGIN_MX) * COSLAT))
    Z0 = int(round(-(nw_my - ORIGIN_MY) * COSLAT))
    return X0, Z0, cmx, cmy, merc_w, merc_h


def column_fields(elev, img):
    """Return per-pixel arrays: hcol (solid top world Y), water_y (top of water
    column or -9999), topgid (block at hcol)."""
    cls = C.classify(img, C.dem_roughness(elev))
    # at/below sea level = ocean/bay/estuary. Marsh above 0.5 m stays land.
    is_ocean = elev <= SEA_T
    is_pond = (~is_ocean) & (cls == C.WATER)

    land_top = np.clip(np.round(elev).astype(np.int32) + SEA_Y, WORLD_BOTTOM, TOP_Y)
    topgid = np.array([CLASS_TO_GID[i] for i in range(7)], dtype=np.int16)[cls]

    hcol = land_top.copy()
    water_y = np.full(elev.shape, -9999, dtype=np.int32)

    # ocean / bay: seabed below sea level, water up to sea level, sand seabed
    hcol[is_ocean] = SEA_Y - SEA_DEPTH
    water_y[is_ocean] = SEA_Y
    topgid[is_ocean] = SEABED_SAND
    # inland pond: carve a couple blocks, water to local surface
    hcol[is_pond] = land_top[is_pond] - POND_DEPTH
    water_y[is_pond] = land_top[is_pond]
    topgid[is_pond] = SUBDIRT

    land_frac = float((~is_ocean).mean())
    return hcol, water_y, topgid, land_frac


def build_tile(spec, regions):
    w_m = int(spec["w"]); h_m = int(spec["h"])
    X0, Z0, cmx, cmy, mw, mh = tile_world_origin(spec["lon"], spec["lat"], w_m, h_m)
    elev = geo.fetch_mosaic(cmx, cmy, mw, mh, w_m, h_m, "elev")
    img = geo.fetch_mosaic(cmx, cmy, mw, mh, w_m, h_m, "img")
    elev = np.nan_to_num(elev, nan=0.0)
    hcol_f, watery_f, topgid_f, land_frac = column_fields(elev, img)
    H, W = hcol_f.shape

    ar = np.arange(16)
    cx_lo, cx_hi = X0 // 16, (X0 + W - 1) // 16
    cz_lo, cz_hi = Z0 // 16, (Z0 + H - 1) // 16
    nchunks = 0
    for cz in range(cz_lo, cz_hi + 1):
        for cx in range(cx_lo, cx_hi + 1):
            px = cx * 16 - X0 + ar
            pz = cz * 16 - Z0 + ar
            colmask = (px >= 0) & (px < W)
            rowmask = (pz >= 0) & (pz < H)
            if not colmask.any() or not rowmask.any():
                continue
            pxc = np.clip(px, 0, W - 1); pzc = np.clip(pz, 0, H - 1)
            sl = np.ix_(pzc, pxc)
            hcol = hcol_f[sl].copy()
            watery = watery_f[sl].copy()
            topg = topgid_f[sl].copy()
            valid = rowmask[:, None] & colmask[None, :]
            hcol[~valid] = -1000
            present = hcol > -900
            if not present.any():
                continue
            y_hi = int(max(hcol.max(), watery.max()))
            Hc = hcol[None, :, :]; Wy = watery[None, :, :]; Tg = topg[None, :, :]
            pres = present[None, :, :]
            sections = {}
            for s in range(WORLD_BOTTOM // 16, y_hi // 16 + 1):
                y0 = s * 16
                wy = (y0 + ar)[:, None, None]
                grid = np.zeros((16, 16, 16), dtype=np.int16)
                stone_m = pres & (wy <= Hc - 4)
                dirt_m = pres & (wy <= Hc - 1) & (wy > Hc - 4)
                top_m = pres & (wy == Hc)
                water_m = pres & (wy > Hc) & (wy <= Wy)
                grid[stone_m] = STONE
                grid[dirt_m] = SUBDIRT
                tgb = np.broadcast_to(Tg, (16, 16, 16))
                grid[top_m] = tgb[top_m]
                grid[water_m] = WATER_GID
                if not grid.any():
                    continue
                flat = grid.reshape(4096)
                uniq, inv = np.unique(flat, return_inverse=True)
                names = [GLOBAL_BLOCKS[i] for i in uniq.tolist()]
                if len(names) == 1:
                    sections[s] = (None, names)
                else:
                    sections[s] = (inv.astype(np.uint64), names)
            if not sections:
                continue
            root = build_chunk(cx, cz, sections, DATA_VERSION,
                               biome="minecraft:beach")
            rx, rz = cx >> 5, cz >> 5
            rw = regions.get((rx, rz))
            if rw is None:
                rw = RegionWriter(); regions[(rx, rz)] = rw
            rw.add(cx, cz, root)
            nchunks += 1
    return nchunks, land_frac


def build_world(specs, out_dir):
    regions = {}
    total = 0
    for spec in specs["tiles"]:
        n, lf = build_tile(spec, regions)
        total += n
        print(f"  tile {spec.get('name','?')} land={lf:.2f} chunks={n}", flush=True)
    os.makedirs(os.path.join(out_dir, "region"), exist_ok=True)
    nbytes = 0
    for (rx, rz), rw in regions.items():
        p = os.path.join(out_dir, "region", f"r.{rx}.{rz}.mca")
        rw.write(p); nbytes += os.path.getsize(p)
    write_level_dat(out_dir, specs.get("name", "Cape Cod 1:1"), DATA_VERSION,
                    spawn=(0, 40, 0))
    return total, nbytes


if __name__ == "__main__":
    specs = json.load(open(sys.argv[1]))
    t0 = time.time()
    n, b = build_world(specs, sys.argv[2])
    print(f"chunks={n} bytes={b} time={time.time()-t0:.0f}s")
