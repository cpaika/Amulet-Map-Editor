"""Continuous 1:1 Worcester / Lake Quinsigamond / Shrewsbury world.

Water comes from authoritative USGS NHD waterbody polygons (exact shorelines +
islands), each given a FLAT surface (single Y) and a modeled depth basin
(distance-to-shore, calibrated to known max depth) so lakes read as real water
with bathymetry instead of a fragmented skin on the noisy DEM.
"""
import os, sys, json, time, math, urllib.request, urllib.parse
import numpy as np
from PIL import Image, ImageDraw

import geo
import classify as C
from classify import CLASS_BLOCK
import lake_depth
from mc_anvil import RegionWriter, build_chunk
from mc_level import write_level_dat

DATA_VERSION = 3700
REF_Y = -36          # world Y of the main lake surface (room for 26 m depth)
WORLD_BOTTOM, TOP_Y = -64, 319

GLOBAL_BLOCKS = ["minecraft:air", "minecraft:stone", "minecraft:dirt",
                 "minecraft:sand", "minecraft:gravel"]
AIR, STONE, SUBDIRT, SAND, GRAVEL = 0, 1, 2, 3, 4
CLASS_TO_GID = {}
for cid in range(7):
    CLASS_TO_GID[cid] = len(GLOBAL_BLOCKS); GLOBAL_BLOCKS.append(CLASS_BLOCK[cid])
WATER_GID = CLASS_TO_GID[C.WATER]

ORIGIN_LON, ORIGIN_LAT = -71.8023, 42.2626
ORIGIN_MX, ORIGIN_MY = geo.lonlat_to_merc(ORIGIN_LON, ORIGIN_LAT)
COSLAT = math.cos(math.radians(ORIGIN_LAT))

QUIN_MAX_DEPTH = 15.2   # Lake Quinsigamond, real max depth (50 ft)


def _arcgis_query(url, p):
    for i in range(4):
        try:
            return json.load(urllib.request.urlopen(url + "?" + urllib.parse.urlencode(p),
                                                     timeout=120)).get("features", [])
        except Exception:
            time.sleep(2 * (i + 1))
    return []


def fetch_waterbodies(xmin, ymin, xmax, ymax):
    url = "https://hydro.nationalmap.gov/arcgis/rest/services/nhd/MapServer/12/query"
    return _arcgis_query(url, {"geometry": f"{xmin},{ymin},{xmax},{ymax}",
        "geometryType": "esriGeometryEnvelope", "inSR": "4326",
        "spatialRel": "esriSpatialRelIntersects", "outFields": "gnis_name,areasqkm",
        "returnGeometry": "true", "outSR": "4326", "f": "json"})


def fetch_contours(xmin, ymin, xmax, ymax):
    """Real MassWildlife bathymetric contour lines (DEPTH in feet)."""
    url = ("https://services1.arcgis.com/7iJyYTjCtKsZS1LR/arcgis/rest/services/"
           "PondBathmetry/FeatureServer/0/query")
    feats = []; off = 0
    while True:
        fs = _arcgis_query(url, {"geometry": f"{xmin},{ymin},{xmax},{ymax}",
            "geometryType": "esriGeometryEnvelope", "inSR": "4326",
            "spatialRel": "esriSpatialRelIntersects", "outFields": "NAME,DEPTH",
            "returnGeometry": "true", "outSR": "4326", "f": "json",
            "resultOffset": off, "resultRecordCount": 1000})
        feats += fs
        if len(fs) < 1000:
            break
        off += 1000
    # precompute lon/lat bbox per contour
    out = []
    for f in feats:
        pts = [p for path in f["geometry"]["paths"] for p in path]
        xs = [p[0] for p in pts]; ys = [p[1] for p in pts]
        out.append((f["attributes"]["DEPTH"], f["geometry"]["paths"],
                    (min(xs), min(ys), max(xs), max(ys))))
    return out


def _ring_area(r):
    x = [p[0] for p in r]; y = [p[1] for p in r]
    return abs(sum(x[i]*y[i+1]-x[i+1]*y[i] for i in range(len(r)-1))) / 2


# a point inside Lake Quinsigamond's open water (vertical datum anchor)
ANCHOR_LON, ANCHOR_LAT = -71.7470, 42.2700


def _pt_in_ring(lon, lat, ring):
    inside = False; j = len(ring) - 1
    for i in range(len(ring)):
        xi, yi = ring[i][0], ring[i][1]; xj, yj = ring[j][0], ring[j][1]
        if ((yi > lat) != (yj > lat)) and \
           (lon < (xj - xi) * (lat - yi) / (yj - yi) + xi):
            inside = not inside
        j = i
    return inside


def _largest_waterbody_surface(feats, elev, xmin, xmax, ymin, ymax, W, H):
    sx = W / (xmax - xmin); sy = H / (ymax - ymin)
    # prefer the waterbody whose outer ring contains the Quinsigamond anchor
    best = None; best_a = -1
    for f in feats:
        rings = f["geometry"]["rings"]
        outer = max(rings, key=_ring_area)
        if _pt_in_ring(ANCHOR_LON, ANCHOR_LAT, outer):
            best = f; break
        a = sum(_ring_area(r) for r in rings)
        if a > best_a:
            best_a = a; best = f
    rings = best["geometry"]["rings"]
    xs = [(p[0]-xmin)*sx for r in rings for p in r]
    ys = [(ymax-p[1])*sy for r in rings for p in r]
    cmin = max(0, int(min(xs))); cmax = min(W, int(max(xs)) + 1)
    rmin = max(0, int(min(ys))); rmax = min(H, int(max(ys)) + 1)
    im = Image.new("1", (cmax-cmin, rmax-rmin), 0); dr = ImageDraw.Draw(im)
    order = sorted(rings, key=_ring_area, reverse=True)
    for j, r in enumerate(order):
        dr.polygon([((p[0]-xmin)*sx - cmin, (ymax-p[1])*sy - rmin) for p in r],
                   fill=1 if j == 0 else 0)
    m = np.array(im, bool)
    return float(np.median(elev[rmin:rmax, cmin:cmax][m]))


def rasterize_water(feats, contours, xmin, xmax, ymin, ymax, W, H, elev, world_y):
    """Return water_mask, surf_y, floor_y. Uses real bathymetric contours where
    they fall inside a waterbody, otherwise a distance-to-shore depth model."""
    water = np.zeros((H, W), bool)
    surf_y = np.full((H, W), -9999, np.int32)
    floor_y = np.full((H, W), -9999, np.int32)
    sx = W / (xmax - xmin); sy = H / (ymax - ymin)
    def to_px(r, c0, r0):
        return [((p[0]-xmin)*sx - c0, (ymax-p[1])*sy - r0) for p in r]
    areas = [sum(_ring_area(r) for r in f["geometry"]["rings"]) for f in feats]
    used = 0
    for i, f in enumerate(feats):
        rings = f["geometry"]["rings"]
        if not rings:
            continue
        xs = [(p[0]-xmin)*sx for r in rings for p in r]
        ys = [(ymax-p[1])*sy for r in rings for p in r]
        cmin = max(0, int(min(xs))); cmax = min(W, int(max(xs)) + 1)
        rmin = max(0, int(min(ys))); rmax = min(H, int(max(ys)) + 1)
        if cmax <= cmin or rmax <= rmin:
            continue
        lw, lh = cmax - cmin, rmax - rmin
        im = Image.new("1", (lw, lh), 0); dr = ImageDraw.Draw(im)
        order = sorted(range(len(rings)), key=lambda k: _ring_area(rings[k]), reverse=True)
        for j, k in enumerate(order):
            dr.polygon(to_px(rings[k], cmin, rmin), fill=1 if j == 0 else 0)
        fmask = np.array(im, bool)
        if not fmask.any():
            continue
        surf_elev = float(np.median(elev[rmin:rmax, cmin:cmax][fmask]))
        sy_world = int(world_y(surf_elev))
        # contours overlapping this feature's lon/lat bbox
        flon0 = xmin + cmin/sx; flon1 = xmin + cmax/sx
        flat1 = ymax - rmin/sy; flat0 = ymax - rmax/sy
        local = [(d, paths) for (d, paths, (bx0, by0, bx1, by1)) in contours
                 if bx1 >= flon0 and bx0 <= flon1 and by1 >= flat0 and by0 <= flat1]
        step = max(1, int(round(min(lw, lh) / 240)))
        cm = fmask[::step, ::step]
        if local:
            known = np.full(cm.shape, np.nan, np.float32)
            known[lake_depth._shore_zero(cm)] = 0.0
            ci = Image.new("I", (cm.shape[1], cm.shape[0]), 0); dc = ImageDraw.Draw(ci)
            for d, paths in local:
                for path in paths:
                    pts = [(((p[0]-xmin)*sx - cmin)/step, ((ymax-p[1])*sy - rmin)/step) for p in path]
                    if len(pts) > 1:
                        dc.line(pts, fill=int(d) + 1, width=1)
            cv = np.array(ci, np.int32)
            seed = (cv > 0) & cm
            known[seed] = (cv[seed] - 1) * 0.3048          # ft -> m
            dep_c = lake_depth.laplace_depth(cm, ~np.isnan(known),
                                             np.nan_to_num(known), iters=900)
            used += 1
        else:
            area_m2 = areas[i] * 1e6
            maxd = min(9.0, 1.5 + 0.05 * math.sqrt(area_m2))
            dep_c = lake_depth.depth_map(cm, step, maxd)
        dep = np.repeat(np.repeat(dep_c, step, 0), step, 1)[:lh, :lw]
        dep_blocks = np.clip(np.round(dep), 1, sy_world - (WORLD_BOTTOM + 1)).astype(np.int32)
        rr, cc = np.where(fmask)
        water[rr+rmin, cc+cmin] = True
        surf_y[rr+rmin, cc+cmin] = sy_world
        floor_y[rr+rmin, cc+cmin] = sy_world - dep_blocks[rr, cc]
    print(f"  real-bathymetry waterbodies: {used}", flush=True)
    return water, surf_y, floor_y


def build(spec, out_dir):
    clon, clat = spec["lon"], spec["lat"]; W, H = int(spec["w"]), int(spec["h"])
    cmx, cmy = geo.lonlat_to_merc(clon, clat)
    mw = W / COSLAT; mh = H / COSLAT
    nw_mx, nw_my = cmx - mw / 2, cmy + mh / 2
    X0 = int(round((nw_mx - ORIGIN_MX) * COSLAT))
    Z0 = int(round(-(nw_my - ORIGIN_MY) * COSLAT))
    lon_half = W / 2 / (111000 * math.cos(math.radians(clat))); lat_half = H / 2 / 111000
    xmin, xmax = clon - lon_half, clon + lon_half
    ymin, ymax = clat - lat_half, clat + lat_half

    print("fetch elevation + imagery ...", flush=True)
    elev = np.nan_to_num(geo.fetch_mosaic(cmx, cmy, mw, mh, W, H, "elev"), nan=0.0)
    elev = np.clip(elev, -50, 2000)
    img = geo.fetch_mosaic(cmx, cmy, mw, mh, W, H, "img")

    print("fetch NHD waterbodies + bathymetric contours ...", flush=True)
    feats = fetch_waterbodies(xmin, ymin, xmax, ymax)
    contours = fetch_contours(xmin, ymin, xmax, ymax)
    print(f"  {len(feats)} waterbodies, {len(contours)} contour lines", flush=True)

    # anchor vertical datum to the main lake's surface elevation
    ref_elev = _largest_waterbody_surface(feats, elev, xmin, xmax, ymin, ymax, W, H)
    def world_y(e):
        return float(np.clip(round(e - ref_elev) + REF_Y, WORLD_BOTTOM, TOP_Y))
    print(f"  lake surface elev {ref_elev:.1f} m -> world y {REF_Y}", flush=True)

    water, surf_y, floor_y = rasterize_water(feats, contours, xmin, xmax, ymin, ymax,
                                             W, H, elev, world_y)
    print(f"  water px frac {water.mean():.3f}", flush=True)

    cls = C.classify(img, C.dem_roughness(elev))
    land_top = np.clip(np.round(elev - ref_elev).astype(np.int32) + REF_Y, WORLD_BOTTOM, TOP_Y)
    topgid = np.array([CLASS_TO_GID[i] for i in range(7)], np.int16)[cls]
    # NHD water overrides classification water/land
    hcol = land_top.copy()
    watery = np.full((H, W), -9999, np.int32)
    hcol[water] = floor_y[water]
    watery[water] = surf_y[water]
    topgid[water] = GRAVEL

    ar = np.arange(16)
    cx_lo, cx_hi = X0 // 16, (X0 + W - 1) // 16
    cz_lo, cz_hi = Z0 // 16, (Z0 + H - 1) // 16
    regions = {}; nch = 0
    for cz in range(cz_lo, cz_hi + 1):
        for cx in range(cx_lo, cx_hi + 1):
            px = cx*16 - X0 + ar; pz = cz*16 - Z0 + ar
            cm = (px >= 0) & (px < W); rm = (pz >= 0) & (pz < H)
            if not cm.any() or not rm.any():
                continue
            sl = np.ix_(np.clip(pz, 0, H-1), np.clip(px, 0, W-1))
            Hc = hcol[sl].copy(); Wy = watery[sl].copy(); Tg = topgid[sl].copy()
            valid = rm[:, None] & cm[None, :]
            Hc[~valid] = -1000
            pres = Hc > -900
            if not pres.any():
                continue
            y_hi = int(max(Hc.max(), Wy.max()))
            Hc3 = Hc[None]; Wy3 = Wy[None]; Tg3 = Tg[None]; pres3 = pres[None]
            sections = {}
            for s in range(WORLD_BOTTOM // 16, y_hi // 16 + 1):
                y0 = s * 16; wy = (y0 + ar)[:, None, None]
                grid = np.zeros((16, 16, 16), np.int16)
                grid[pres3 & (wy <= Hc3 - 4)] = STONE
                grid[pres3 & (wy <= Hc3 - 1) & (wy > Hc3 - 4)] = SUBDIRT
                tm = pres3 & (wy == Hc3)
                grid[tm] = np.broadcast_to(Tg3, (16, 16, 16))[tm]
                grid[pres3 & (wy > Hc3) & (wy <= Wy3)] = WATER_GID
                if not grid.any():
                    continue
                flat = grid.reshape(4096); uniq, inv = np.unique(flat, return_inverse=True)
                names = [GLOBAL_BLOCKS[i] for i in uniq.tolist()]
                sections[s] = (None, names) if len(names) == 1 else (inv.astype(np.uint64), names)
            if not sections:
                continue
            root = build_chunk(cx, cz, sections, DATA_VERSION, biome="minecraft:plains")
            rx, rz = cx >> 5, cz >> 5
            regions.setdefault((rx, rz), RegionWriter()).add(cx, cz, root); nch += 1
    os.makedirs(os.path.join(out_dir, "region"), exist_ok=True)
    nbytes = 0
    for (rx, rz), rw in regions.items():
        p = os.path.join(out_dir, "region", f"r.{rx}.{rz}.mca"); rw.write(p)
        nbytes += os.path.getsize(p)
    write_level_dat(out_dir, spec.get("name", "Worcester Quinsigamond Shrewsbury"),
                    DATA_VERSION, spawn=(X0 + W // 2, 80, Z0 + H // 2))
    return nch, nbytes, X0, Z0


if __name__ == "__main__":
    spec = json.load(open(sys.argv[1]))
    t0 = time.time()
    nch, nb, X0, Z0 = build(spec, sys.argv[2])
    print(f"chunks={nch} bytes={nb} X0={X0} Z0={Z0} time={time.time()-t0:.0f}s")
