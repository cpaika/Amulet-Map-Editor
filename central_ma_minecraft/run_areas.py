"""Resumable sweep: cover all LAND in the Cape Cod + Martha's Vineyard + Waquoit
region with true 1:1 Minecraft tiles, one deliverable zip per tile.

For each tile: build -> zip -> git commit+push -> delete raw world (to free
disk). Tiles whose zip already exists in dist/ are skipped, so the job resumes
cleanly after any interruption. Named places are built first.
"""
import os
import sys
import json
import math
import time
import shutil
import zipfile
import subprocess
import numpy as np

import geo
import build_coastal as B

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)              # repo root (Amulet-Map-Editor)
DIST = os.path.join(HERE, "dist", "capecod")
WORK = os.path.join(HERE, "work", "capecod_build")
TILE = 6000                       # tile size (m); ~36 km^2 -> ~70 MB zip
LAND_MIN = 0.05                   # min land fraction to bother building

# whole-region bounding box (lon/lat)
LON0, LON1 = -70.96, -69.93
LAT0, LAT1 = 41.33, 42.10

# named priority centers (built first), lon, lat
PRIORITY = [
    ("waquoit_bay",        -70.513, 41.553),
    ("mv_aquinnah",        -70.835, 41.350),
    ("mv_chilmark",        -70.755, 41.355),
    ("mv_west_tisbury",    -70.690, 41.380),
    ("mv_vineyard_haven",  -70.605, 41.450),
    ("mv_oak_bluffs",      -70.560, 41.455),
    ("mv_edgartown",       -70.513, 41.389),
    ("mv_chappaquiddick",  -70.455, 41.385),
    ("ptown_racepoint",    -70.220, 42.060),
    ("truro_highlands",    -70.060, 41.990),
    ("chatham_elbow",      -69.960, 41.680),
    ("sandwich_canal",     -70.505, 41.760),
]


def land_mask():
    """One coarse elevation fetch over the whole region -> boolean land grid
    plus its geo transform."""
    cx = (LON0 + LON1) / 2; cy = (LAT0 + LAT1) / 2
    mx0, my0 = geo.lonlat_to_merc(LON0, LAT0)
    mx1, my1 = geo.lonlat_to_merc(LON1, LAT1)
    cmx, cmy = geo.lonlat_to_merc(cx, cy)
    mw = mx1 - mx0; mh = my1 - my0
    px = 700; py = int(px * mh / mw)
    elev = geo.fetch_mosaic(cmx, cmy, mw, mh, px, py, "elev")
    elev = np.nan_to_num(elev, nan=0.0)
    land = elev > 0.6
    return land, (mx0, my1, mw / px, mh / py)  # origin NW merc + px size


def tile_land_fraction(land, tf, lon, lat):
    mx0, my1, sx, sy = tf
    cmx, cmy = geo.lonlat_to_merc(lon, lat)
    half = TILE / 2 / B.COSLAT  # mercator half-extent
    c0 = int((cmx - half - mx0) / sx); c1 = int((cmx + half - mx0) / sx)
    r0 = int((my1 - (cmy + half)) / sy); r1 = int((my1 - (cmy - half)) / sy)
    H, W = land.shape
    c0, c1 = max(0, c0), min(W, c1); r0, r1 = max(0, r0), min(H, r1)
    if c1 <= c0 or r1 <= r0:
        return 0.0
    return float(land[r0:r1, c0:c1].mean())


def git_push(zip_path, name):
    rel = os.path.relpath(zip_path, REPO)
    subprocess.run(["git", "-C", REPO, "add", "-f", rel], check=True)
    subprocess.run(["git", "-C", REPO, "commit", "-q", "-m",
                    f"Add Cape Cod 1:1 tile: {name}"], check=True)
    for i in range(4):
        r = subprocess.run(["git", "-C", REPO, "push", "origin", "HEAD"])
        if r.returncode == 0:
            return True
        time.sleep(2 * (i + 1))
    return False


def zip_world(world_dir, zip_path):
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED, compresslevel=6) as z:
        for root, _, files in os.walk(world_dir):
            for f in files:
                fp = os.path.join(root, f)
                z.write(fp, os.path.relpath(fp, os.path.dirname(world_dir)))


def do_tile(name, lon, lat):
    zip_path = os.path.join(DIST, f"CapeCod_{name}.zip")
    if os.path.exists(zip_path):
        print(f"[skip] {name} (already delivered)", flush=True)
        return
    spec = {"name": f"CapeCod {name}", "tiles":
            [{"name": name, "lon": lon, "lat": lat, "w": TILE, "h": TILE}]}
    wd = os.path.join(WORK, name)
    shutil.rmtree(wd, ignore_errors=True)
    X0, Z0, *_ = B.tile_world_origin(lon, lat, TILE, TILE)
    t0 = time.time()
    regions = {}
    n, lf = B.build_tile(spec["tiles"][0], regions)
    os.makedirs(os.path.join(wd, "region"), exist_ok=True)
    for (rx, rz), rw in regions.items():
        rw.write(os.path.join(wd, "region", f"r.{rx}.{rz}.mca"))
    from mc_level import write_level_dat
    write_level_dat(wd, spec["name"], B.DATA_VERSION,
                    spawn=(X0 + TILE // 2, 60, Z0 + TILE // 2))
    os.makedirs(DIST, exist_ok=True)
    zip_world(wd, zip_path)
    mb = os.path.getsize(zip_path) / 1048576
    shutil.rmtree(wd, ignore_errors=True)
    ok = git_push(zip_path, name)
    print(f"[done] {name} land={lf:.2f} chunks={n} {mb:.0f}MB "
          f"{time.time()-t0:.0f}s pushed={ok}", flush=True)


def main():
    os.makedirs(DIST, exist_ok=True)
    print("computing land mask ...", flush=True)
    land, tf = land_mask()
    # build ordered tile list: priorities first, then grid by land fraction
    done = set()
    order = []
    for name, lon, lat in PRIORITY:
        order.append((name, lon, lat))
        done.add((round(lon, 3), round(lat, 3)))
    grid = []
    lat = LAT0
    step_lat = TILE / 111000.0
    while lat <= LAT1:
        step_lon = TILE / (111000.0 * math.cos(math.radians(lat)))
        lon = LON0
        while lon <= LON1:
            key = (round(lon, 3), round(lat, 3))
            lf = tile_land_fraction(land, tf, lon, lat)
            if lf >= LAND_MIN and key not in done:
                grid.append((lf, f"g_{lat:.3f}_{lon:.3f}".replace('.', 'p').replace('-', 'm'),
                             lon, lat))
            lon += step_lon
        lat += step_lat
    grid.sort(reverse=True)  # most land first
    run_grid = os.environ.get("RUN_GRID", "0") == "1"
    grid_limit = int(os.environ.get("GRID_LIMIT", "9999"))
    print(f"{len(order)} priority + {len(grid)} grid land-tiles "
          f"(run_grid={run_grid}, limit={grid_limit})", flush=True)
    for name, lon, lat in order:
        do_tile(name, lon, lat)
    if run_grid:
        for lf, name, lon, lat in grid[:grid_limit]:
            do_tile(name, lon, lat)
    print("SWEEP COMPLETE", flush=True)


if __name__ == "__main__":
    main()
