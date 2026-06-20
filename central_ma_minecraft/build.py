"""Build a 1:1 (1 block = 1 m) Minecraft world of central Massachusetts from
USGS 3DEP 1 m elevation + Esri World Imagery land cover.

Each "tile" (a named area such as Worcester or Mt Wachusett) is fetched at true
1 m/px ground resolution and written into one shared, correctly geo-referenced
world. Empty space between tiles costs no disk (unauthored chunks stay void).
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

DATA_VERSION = 3700      # Minecraft Java 1.20.4
MC_NAME = "1.20.4"
WORLD_BOTTOM = -64
BASE_Y = -60             # lowest terrain sits here
TOP_Y = 319

# global block table: 0 air,1 stone,2 subsurface dirt, then one per land class
GLOBAL_BLOCKS = ["minecraft:air", "minecraft:stone", "minecraft:dirt"]
CLASS_TO_GID = {}
for cid in range(7):
    CLASS_TO_GID[cid] = len(GLOBAL_BLOCKS)
    GLOBAL_BLOCKS.append(CLASS_BLOCK[cid])
AIR, STONE, SUBDIRT = 0, 1, 2

# project origin (true-meter reference) = Worcester City Hall area
ORIGIN_LON, ORIGIN_LAT = -71.8023, 42.2626
ORIGIN_MX, ORIGIN_MY = geo.lonlat_to_merc(ORIGIN_LON, ORIGIN_LAT)
COSLAT = math.cos(math.radians(ORIGIN_LAT))


def tile_world_origin(center_lon, center_lat, w_m, h_m):
    """Return (X0, Z0) world block coords of the tile's NW corner and the
    mercator center/extent needed to sample it at true 1 m/px."""
    cmx, cmy = geo.lonlat_to_merc(center_lon, center_lat)
    merc_w = w_m / COSLAT
    merc_h = h_m / COSLAT
    nw_mx = cmx - merc_w / 2.0
    nw_my = cmy + merc_h / 2.0
    east = (nw_mx - ORIGIN_MX) * COSLAT
    north = (nw_my - ORIGIN_MY) * COSLAT
    X0 = int(round(east))
    Z0 = int(round(-north))   # north -> negative Z
    return X0, Z0, cmx, cmy, merc_w, merc_h


def build_tile(spec, regions, stats):
    name = spec["name"]
    w_m = int(spec["w"]); h_m = int(spec["h"])
    print(f"[{name}] fetching {w_m}x{h_m} m elevation + imagery ...", flush=True)
    X0, Z0, cmx, cmy, merc_w, merc_h = tile_world_origin(
        spec["lon"], spec["lat"], w_m, h_m)
    elev = geo.fetch_mosaic(cmx, cmy, merc_w, merc_h, w_m, h_m, "elev")
    img = geo.fetch_mosaic(cmx, cmy, merc_w, merc_h, w_m, h_m, "img")
    # vertical datum: robust min so an outlier pixel doesn't waste height range
    valid = np.isfinite(elev)
    elev = np.where(valid, elev, np.nanmedian(elev[valid]))
    cls = C.classify(img, C.dem_roughness(elev))
    dmin = float(np.percentile(elev, 0.2))
    hcol_full = np.round(elev - dmin).astype(np.int32) + BASE_Y
    hcol_full = np.clip(hcol_full, WORLD_BOTTOM, TOP_Y)
    H, W = hcol_full.shape
    print(f"[{name}] world NW corner ({X0},{Z0}); elev {elev.min():.0f}-"
          f"{elev.max():.0f} m -> y {hcol_full.min()}..{hcol_full.max()}", flush=True)

    cls_to_gid = np.array([CLASS_TO_GID[i] for i in range(7)], dtype=np.int16)
    gid_top_full = cls_to_gid[cls]

    # iterate chunks covering [X0,X0+W) x [Z0,Z0+H)
    cx_lo = X0 // 16
    cx_hi = (X0 + W - 1) // 16
    cz_lo = Z0 // 16
    cz_hi = (Z0 + H - 1) // 16
    ar = np.arange(16)
    nchunks = 0
    for cz in range(cz_lo, cz_hi + 1):
        for cx in range(cx_lo, cx_hi + 1):
            # local pixel coords for this chunk's 16x16 footprint
            gx0 = cx * 16; gz0 = cz * 16
            px = gx0 - X0 + ar      # (16,) col indices into arrays
            pz = gz0 - Z0 + ar      # (16,) row indices
            colmask = (px >= 0) & (px < W)
            rowmask = (pz >= 0) & (pz < H)
            if not colmask.any() or not rowmask.any():
                continue
            pxc = np.clip(px, 0, W - 1)
            pzc = np.clip(pz, 0, H - 1)
            hcol = hcol_full[np.ix_(pzc, pxc)].copy()      # (16z,16x)
            topg = gid_top_full[np.ix_(pzc, pxc)].copy()
            valid2 = rowmask[:, None] & colmask[None, :]
            hcol[~valid2] = -1000                          # mark absent -> air
            if not (hcol > -900).any():
                continue
            y_hi = int(hcol.max())
            sections = {}
            s_lo = WORLD_BOTTOM // 16          # -4
            s_hi = y_hi // 16
            Hc = hcol[None, :, :]
            Tg = topg[None, :, :]
            present = Hc > -900
            for s in range(s_lo, s_hi + 1):
                y0 = s * 16
                wy = (y0 + ar)[:, None, None]          # (16y,1,1)
                grid = np.zeros((16, 16, 16), dtype=np.int16)  # air
                stone_m = present & (wy <= Hc - 4)
                dirt_m = present & (wy <= Hc - 1) & (wy > Hc - 4)
                top_m = present & (wy == Hc)
                grid[stone_m] = STONE
                grid[dirt_m] = SUBDIRT
                # broadcast top gid into top cells
                tg_b = np.broadcast_to(Tg, (16, 16, 16))
                grid[top_m] = tg_b[top_m]
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
            root = build_chunk(cx, cz, sections, DATA_VERSION)
            rx, rz = cx >> 5, cz >> 5
            rw = regions.get((rx, rz))
            if rw is None:
                rw = RegionWriter(); regions[(rx, rz)] = rw
            rw.add(cx, cz, root)
            nchunks += 1
    stats[name] = {"chunks": nchunks, "X0": X0, "Z0": Z0, "w": w_m, "h": h_m,
                   "elev_min": float(elev.min()), "elev_max": float(elev.max())}
    print(f"[{name}] wrote {nchunks} chunks", flush=True)


def main(spec_path, out_dir):
    with open(spec_path) as f:
        specs = json.load(f)
    regions = {}
    stats = {}
    t0 = time.time()
    for spec in specs["tiles"]:
        build_tile(spec, regions, stats)
    os.makedirs(os.path.join(out_dir, "region"), exist_ok=True)
    nbytes = 0
    for (rx, rz), rw in regions.items():
        p = os.path.join(out_dir, "region", f"r.{rx}.{rz}.mca")
        rw.write(p)
        nbytes += os.path.getsize(p)
    # spawn at Worcester center on the surface
    write_level_dat(out_dir, specs.get("name", "Central MA 1:1"), DATA_VERSION,
                    spawn=(0, 120, 0))
    stats["_summary"] = {"regions": len(regions),
                         "region_bytes": nbytes,
                         "seconds": round(time.time() - t0, 1)}
    with open(os.path.join(out_dir, "build_stats.json"), "w") as f:
        json.dump(stats, f, indent=2)
    print("TOTAL regions", len(regions), "bytes", nbytes,
          "time", stats["_summary"]["seconds"], "s")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
