"""Build compact gridded datasets for the Shrewsbury noise model.

Inputs (downloaded separately, not committed because they are large):
  - N42W072.hgt        SRTM 1-arcsec (~30 m) terrain tile
  - buildings.json     OSM building footprints (Overpass, ~29k polygons)
  - ri_roads.json      MassDOT Road Inventory 2021 segments (committed)

Outputs (committed, small):
  - terrain_grid.npz   ground elevation (m) on a local 8 m metric grid
  - building_grid.npz  max building height (m) on the same grid

The grid is a local east/north metric frame centred on 6 Trowbridge Circle.
"""
import json, math, os
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.environ.get("SOUND_RAW", "/tmp/sound")   # where big raw inputs live

# ---- local metric frame, centred on the target -----------------------------
LAT0, LON0 = 42.2940843, -71.7007366
def m_per_deg(lat):
    l = math.radians(lat)
    return (111132.92 - 559.82*math.cos(2*l) + 1.175*math.cos(4*l),
            111412.84*math.cos(l) - 93.5*math.cos(3*l))
MN, ME = m_per_deg(LAT0)   # metres per degree (north, east)

def ll_to_m(lat, lon):
    return ((lon-LON0)*ME, (lat-LAT0)*MN)   # x east, y north (metres)

# data box (a touch larger than the receivers' span)
LA0, LA1, LO0, LO1 = 42.225, 42.345, -71.785, -71.645
X0, Y0 = ll_to_m(LA0, LO0)
X1, Y1 = ll_to_m(LA1, LO1)
RES = 8.0  # metres
NX = int((X1-X0)/RES)+1
NY = int((Y1-Y0)/RES)+1
print(f"grid {NX} x {NY} cells @ {RES} m  ({NX*NY/1e6:.2f} M cells)")

def m_to_ix(x, y):
    return ((x-X0)/RES, (y-Y0)/RES)


# ---- terrain from SRTM ------------------------------------------------------
def build_terrain():
    a = np.fromfile(os.path.join(RAW, "N42W072.hgt"), dtype=">i2").astype(np.float32)
    n = int(round(len(a)**0.5))
    dem = a.reshape(n, n)
    dem[dem < -1000] = np.nan  # voids
    # tile N42W072: row 0 = lat 43 (north), col 0 = lon -72 (west), 1 arcsec
    xs = X0 + np.arange(NX)*RES
    ys = Y0 + np.arange(NY)*RES
    XX, YY = np.meshgrid(xs, ys)              # (NY, NX) metres
    lat = LAT0 + YY/MN
    lon = LON0 + XX/ME
    fi = (43.0 - lat)*3600.0
    fj = (lon + 72.0)*3600.0
    i0 = np.clip(fi.astype(int), 0, n-2); j0 = np.clip(fj.astype(int), 0, n-2)
    di = fi - i0; dj = fj - j0
    terr = (dem[i0, j0]*(1-di)*(1-dj) + dem[i0+1, j0]*di*(1-dj) +
            dem[i0, j0+1]*(1-di)*dj + dem[i0+1, j0+1]*di*dj).astype(np.float32)
    np.savez_compressed(os.path.join(HERE, "terrain_grid.npz"),
                        elev=terr, x0=X0, y0=Y0, res=RES, nx=NX, ny=NY,
                        lat0=LAT0, lon0=LON0, mn=MN, me=ME)
    print(f"terrain: min {np.nanmin(terr):.0f} max {np.nanmax(terr):.0f} m  "
          f"-> terrain_grid.npz {os.path.getsize(HERE+'/terrain_grid.npz')//1024} KB")


# ---- building heights from OSM ----------------------------------------------
DEFAULT_H = 6.5   # 2-storey residential
def bldg_height(tags):
    h = tags.get("height")
    if h:
        try: return max(2.5, float(str(h).split()[0].replace("m","")))
        except ValueError: pass
    lv = tags.get("building:levels")
    if lv:
        try: return max(2.5, float(lv)*3.1 + 1.0)
        except ValueError: pass
    bt = (tags.get("building") or "").lower()
    if bt in ("commercial","retail","industrial","warehouse","supermarket"): return 8.0
    if bt in ("church","school","hospital","civic","public"): return 10.0
    if bt in ("apartments","dormitory"): return 12.0
    return DEFAULT_H

def build_buildings():
    d = json.load(open(os.path.join(RAW, "buildings.json")))
    grid = np.zeros((NY, NX), dtype=np.float32)
    nb = 0
    for el in d["elements"]:
        g = el.get("geometry")
        if not g or len(g) < 3: continue
        h = bldg_height(el.get("tags", {}))
        pts = np.array([ll_to_m(p["lat"], p["lon"]) for p in g])  # (k,2) metres
        ix = (pts[:,0]-X0)/RES; iy = (pts[:,1]-Y0)/RES
        cminx, cmaxx = int(np.floor(ix.min())), int(np.ceil(ix.max()))
        cminy, cmaxy = int(np.floor(iy.min())), int(np.ceil(iy.max()))
        if cmaxx < 0 or cminx > NX-1 or cmaxy < 0 or cminy > NY-1: continue
        cminx=max(cminx,0); cminy=max(cminy,0); cmaxx=min(cmaxx,NX-1); cmaxy=min(cmaxy,NY-1)
        gx, gy = np.meshgrid(np.arange(cminx,cmaxx+1), np.arange(cminy,cmaxy+1))
        inside = _pip(gx+0.5, gy+0.5, ix, iy)
        sub = grid[cminy:cmaxy+1, cminx:cmaxx+1]
        np.maximum(sub, np.where(inside, h, 0.0), out=sub)
        nb += 1
    np.savez_compressed(os.path.join(HERE, "building_grid.npz"),
                        height=grid, x0=X0, y0=Y0, res=RES, nx=NX, ny=NY)
    cov = (grid > 0).sum()
    print(f"buildings: {nb} rasterised, {cov} cells covered ({cov*RES*RES/1e6:.2f} km2) "
          f"-> building_grid.npz {os.path.getsize(HERE+'/building_grid.npz')//1024} KB")

def _pip(px, py, vx, vy):
    """Vectorised even-odd point-in-polygon. px,py grids; vx,vy polygon verts."""
    inside = np.zeros(px.shape, dtype=bool)
    n = len(vx); j = n-1
    for i in range(n):
        cond = ((vy[i] > py) != (vy[j] > py))
        slope = (vx[j]-vx[i]) / (vy[j]-vy[i] + 1e-12)
        xint = vx[i] + (py - vy[i])*slope
        inside ^= cond & (px < xint)
        j = i
    return inside


if __name__ == "__main__":
    build_terrain()
    build_buildings()
