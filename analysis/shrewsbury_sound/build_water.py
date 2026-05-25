"""Rasterise OSM open-water polygons (Lake Quinsigamond + ponds) onto the model
grid -> water_grid.npz.  Water is acoustically hard/reflective, handled by the
propagation model. Input water_all.json is fetched by fetch_data.sh.
"""
import json, os, numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.environ.get("SOUND_RAW", "/tmp/sound")

T = np.load(os.path.join(HERE, "terrain_grid.npz"))
X0, Y0, RES = float(T["x0"]), float(T["y0"]), float(T["res"])
NX, NY = int(T["nx"]), int(T["ny"])
LAT0, LON0, MN, ME = float(T["lat0"]), float(T["lon0"]), float(T["mn"]), float(T["me"])

def ll(lat, lon): return ((lon-LON0)*ME, (lat-LAT0)*MN)

W = np.zeros((NY, NX), bool)
def fill(poly):
    pts = np.array([ll(p["lat"], p["lon"]) for p in poly])
    ix = (pts[:,0]-X0)/RES; iy = (pts[:,1]-Y0)/RES
    x0, x1 = max(int(ix.min()),0), min(int(ix.max())+1, NX-1)
    y0, y1 = max(int(iy.min()),0), min(int(iy.max())+1, NY-1)
    if x1 <= x0 or y1 <= y0: return
    gx, gy = np.meshgrid(np.arange(x0,x1+1), np.arange(y0,y1+1))
    px, py = gx+0.5, gy+0.5; inside = np.zeros(px.shape, bool); n = len(ix); j = n-1
    for i in range(n):
        cond = ((iy[i] > py) != (iy[j] > py))
        xint = ix[i] + (py-iy[i])*(ix[j]-ix[i])/(iy[j]-iy[i]+1e-12)
        inside ^= cond & (px < xint); j = i
    W[y0:y1+1, x0:x1+1] |= inside

def main():
    d = json.load(open(os.path.join(RAW, "water_all.json")))
    nb = 0
    for e in d["elements"]:
        g = e.get("geometry")
        if g and len(g) >= 3: fill(g); nb += 1
        for m in e.get("members", []):
            mg = m.get("geometry")
            if m.get("role") == "outer" and mg and len(mg) >= 3: fill(mg); nb += 1
    np.savez_compressed(os.path.join(HERE, "water_grid.npz"),
                        water=W, x0=X0, y0=Y0, res=RES, nx=NX, ny=NY)
    print(f"water polys {nb}, water cells {int(W.sum())} ({W.mean()*100:.1f}% of area)")

if __name__ == "__main__":
    main()
