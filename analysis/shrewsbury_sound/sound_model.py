"""Comprehensive environmental noise model for Shrewsbury, MA.

Estimates the A-weighted daytime equivalent sound level (Leq) produced by road
traffic at 6 Trowbridge Circle and at comparison locations, using measured
inputs and a physically grounded propagation model.

DATA (all real, downloaded by the fetch scripts):
  * Road network + MEASURED traffic volumes: MassDOT Road Inventory 2021
    (per-segment AADT, speed limit, lanes, functional class).  ri_roads.json
  * Terrain: SRTM 1-arcsec (~30 m) elevation.                   terrain_grid.npz
  * Buildings: OSM footprints, rasterised to max height.        building_grid.npz

MODEL (per road, summed incoherently):
  * Each road is cut into ~15 m segments treated as point sub-sources whose
    A-weighted sound power comes from a CoRTN emission level (flow from AADT,
    plus speed and heavy-vehicle corrections), calibrated so an infinite line
    reproduces the CoRTN level at 10 m.
  * Propagation per sub-source (ISO 9613-2 style):
        Leq = Lw - Adiv - Aatm - {Abar  if the sight line is blocked
                                   Agr   otherwise}
    - Adiv : geometric divergence over hard ground near the source (20·log d).
    - Aatm : atmospheric absorption (~2.5 dB/km, A-weighted, ~10C/70% RH).
    - Agr  : ISO 9613-2 soft-ground attenuation (heights above local terrain).
    - Abar : barrier diffraction (Maekawa) from the REAL terrain+building
             profile sampled along each sight line; capped at 20 dB.
  * All sub-sources energy-summed, plus a suburban ambient floor.

Heights/terrain are taken from the DEM so a receiver downhill of / screened by
terrain or rows of houses is correctly quieter.  Facade reflections, lateral
diffraction and meteorological focusing are not modelled (notes in README).

Run:  python3 sound_model.py            # comparison table
      python3 sound_model.py --map      # also render a town-wide noise map
"""
import json, math, os, sys
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))

# --------------------------------------------------------------------------- #
#  Grids (terrain elevation + building heights) in a local metric frame.
# --------------------------------------------------------------------------- #
_T = np.load(os.path.join(HERE, "terrain_grid.npz"))
_B = np.load(os.path.join(HERE, "building_grid.npz"))
X0, Y0, RES = float(_T["x0"]), float(_T["y0"]), float(_T["res"])
NX, NY = int(_T["nx"]), int(_T["ny"])
LAT0, LON0 = float(_T["lat0"]), float(_T["lon0"])
MN, ME = float(_T["mn"]), float(_T["me"])
TERR = _T["elev"].astype(np.float32)                 # (NY, NX) ground elevation
BLDG = _B["height"].astype(np.float32)               # (NY, NX) building height
SURF = TERR + BLDG                                    # top-of-obstruction surface

def ll_to_m(lat, lon):
    return ((lon-LON0)*ME, (lat-LAT0)*MN)

def sample_grid(arr, x, y):
    """Bilinear sample of a grid at metric (x,y) arrays."""
    fx = np.clip((x-X0)/RES, 0, NX-1.001)
    fy = np.clip((y-Y0)/RES, 0, NY-1.001)
    ix = fx.astype(int); iy = fy.astype(int)
    dx = fx-ix; dy = fy-iy
    return (arr[iy, ix]*(1-dx)*(1-dy) + arr[iy, ix+1]*dx*(1-dy) +
            arr[iy+1, ix]*(1-dx)*dy + arr[iy+1, ix+1]*dx*dy)

def terr_at(x, y):
    return float(sample_grid(TERR, np.array([x]), np.array([y]))[0])


# --------------------------------------------------------------------------- #
#  Acoustic / traffic constants.
# --------------------------------------------------------------------------- #
AMBIENT_LEQ = 40.0          # suburban daytime ambient floor, dB(A)
HOURLY_FRAC = 0.0556        # representative daytime hour = AADT / 18
AIR_DB_PER_M = 0.0025       # atmospheric absorption, A-weighted (~2.5 dB/km)
H_SRC = 0.5                 # effective source height above road, m
H_RCV = 1.5                 # receiver (ears, outdoors at grade), m
D_MIN = 10.0                # min source-receiver distance (road edge), m
BAR_EDGE = 12.0             # ignore obstructions within this of source/receiver, m
SEG = 15.0                  # sub-source spacing along roads, m
MAX_RANGE = 4000.0          # ignore sub-sources beyond this (negligible), m
BAR_CAP = 20.0              # max barrier insertion loss, dB
WAVELEN = 343.0/550.0       # nominal wavelength for diffraction (~550 Hz)

# %heavy and fallback speed (mph) by MassDOT functional class code.
FCLASS_HEAVY = {1:10,2:8,3:6,4:4,5:3,6:3,7:2,0:3}
FCLASS_MPH   = {1:65,2:60,3:45,4:40,5:35,6:30,7:25,0:30}
FCLASS_DEF_AADT = {1:40000,2:25000,3:12000,4:7000,5:3000,6:1200,7:400,0:600}


# --------------------------------------------------------------------------- #
#  Build point sub-sources from the Road Inventory.
# --------------------------------------------------------------------------- #
def cortn_leq10(q, v_kmh, p_heavy):
    """CoRTN basic noise level L10(1h) at 10 m, converted to Leq, dB(A)."""
    q = max(q, 1.0); v = max(v_kmh, 8.0)
    l10 = (42.2 + 10*math.log10(q)
           + 33*math.log10(v + 40 + 500/v)
           + 10*math.log10(1 + 5*p_heavy/v) - 68.8)
    return l10 - 3.0

def build_sources():
    d = json.load(open(os.path.join(HERE, "ri_roads.json")))
    sx, sy, slw, sroad = [], [], [], []
    roadmeta = {}
    for f in d["features"]:
        a = f["attributes"]; geom = f.get("geometry", {})
        paths = geom.get("paths") or []
        if not paths: continue
        fclass = a.get("F_Class") or 0
        aadt = a.get("AADT") or FCLASS_DEF_AADT.get(fclass, 800)
        if aadt < 250: continue
        mph = a.get("Speed_Lim") or a.get("Speed") or FCLASS_MPH.get(fclass, 30)
        v = max(15.0, float(mph)) * 1.609344
        p = FCLASS_HEAVY.get(fclass, 3)
        if a.get("Truck_Rte") in (1, "1", "Y", "yes"): p += 2
        leq10 = cortn_leq10(aadt*HOURLY_FRAC, v, p)
        name = a.get("St_Name") or (f"Route {a.get('Route_Number')}" if a.get("Route_Number") else "(local)")
        rt = a.get("Route_Number")
        key = f"Route {rt}" if rt and str(rt).isdigit() and int(rt) < 1000 else name
        roadmeta.setdefault(key, {"name": name, "aadt": 0})
        roadmeta[key]["aadt"] = max(roadmeta[key]["aadt"], aadt)
        # cut each path into ~SEG-spaced sub-sources, each carrying SEG of road
        for path in paths:
            pm = [ll_to_m(lat=p2[1], lon=p2[0]) for p2 in path]
            for i in range(len(pm)-1):
                (x1,y1),(x2,y2) = pm[i], pm[i+1]
                seglen = math.hypot(x2-x1, y2-y1)
                n = max(1, int(round(seglen/SEG)))
                for k in range(n):
                    t = (k+0.5)/n
                    sx.append(x1+(x2-x1)*t); sy.append(y1+(y2-y1)*t)
                    slw.append(leq10); sroad.append(key)
    sx=np.array(sx); sy=np.array(sy); slw=np.array(slw)
    sz = sample_grid(TERR, sx, sy) + H_SRC          # source elevation (m, ASL)
    return sx, sy, slw, sz, np.array(sroad), roadmeta

SX, SY, SLW10, SZ, SROAD, ROADMETA = build_sources()
print(f"[model] {len(SX)} sub-sources from {len(ROADMETA)} roads", file=sys.stderr)

# Calibrate: sub-source point power so an infinite line at spacing SEG
# reproduces the CoRTN Leq at 10 m (no excess attenuation).
def _calibrate():
    xs = np.arange(-6000, 6000, SEG)
    r = np.maximum(np.hypot(xs, 10.0), D_MIN)   # same clamp as propagation
    s = np.sum(10**(-(20*np.log10(r)+8)/10))
    return -10*math.log10(s)        # OFFSET: Lw_pt = Leq10 + OFFSET
OFFSET = _calibrate()
SLW = SLW10 + OFFSET                # per-sub-source A-weighted sound power


# --------------------------------------------------------------------------- #
#  Propagation.
# --------------------------------------------------------------------------- #
def ground_atten(d, hm):
    """ISO 9613-2 alternative soft-ground attenuation (A-weighted)."""
    return np.clip(4.8 - (2*hm/np.maximum(d,1.0))*(17 + 300/np.maximum(d,1.0)), 0, None)

def barrier_atten(rx, ry, rz, idx):
    """Maekawa barrier loss from the real terrain+building profile along each
    sight line receiver->sub-source[idx].  Returns (Abar, blocked_mask)."""
    sx, sy, sz = SX[idx], SY[idx], SZ[idx]
    d = np.hypot(sx-rx, sy-ry)
    nseg = len(idx)
    K = 24                                   # profile samples per sight line
    t = np.linspace(0.0, 1.0, K)[None, :]    # (1,K)
    px = rx + (sx-rx)[:,None]*t              # (n,K)
    py = ry + (sy-ry)[:,None]*t
    surf = sample_grid(SURF, px.ravel(), py.ravel()).reshape(nseg, K)
    losz = rz + (sz-rz)[:,None]*t            # straight line height
    intr = surf - losz                       # >0 where terrain/buildings block
    # ignore obstructions hugging the receiver or source (own house / curb)
    dist_r = d[:,None]*t
    edge = (dist_r < BAR_EDGE) | (dist_r > (d[:,None]-BAR_EDGE))
    intr = np.where(edge, -1e9, intr)
    Abar = np.zeros(nseg); blocked = np.zeros(nseg, bool)
    over = intr.max(axis=1)
    m = over > 0.05
    if m.any():
        kmax = intr[m].argmax(axis=1)
        tk = t[0, kmax]
        ox = (rx + (sx[m]-rx)*tk); oy = (ry + (sy[m]-ry)*tk)
        oz = surf[m, kmax]
        dso = np.sqrt((ox-sx[m])**2 + (oy-sy[m])**2 + (oz-sz[m])**2)
        dor = np.sqrt((ox-rx)**2 + (oy-ry)**2 + (oz-rz)**2)
        dsr = np.sqrt((sx[m]-rx)**2 + (sy[m]-ry)**2 + (sz[m]-rz)**2)
        delta = np.maximum(dso+dor-dsr, 0.0)          # path-length difference
        N = 2*delta/WAVELEN                            # Fresnel number
        a = np.clip(10*np.log10(3 + 20*N), 0, BAR_CAP)
        Abar[m] = a; blocked[m] = True
    return Abar, blocked, d

def leq_at(rx, ry, rz=None, cull=MAX_RANGE):
    if rz is None:
        rz = terr_at(rx, ry) + H_RCV
    d0 = np.hypot(SX-rx, SY-ry)
    idx = np.where(d0 <= cull)[0]
    Abar, blocked, d = barrier_atten(rx, ry, rz, idx)
    Adiv = 20*np.log10(np.maximum(d, D_MIN)) + 8.0
    Aatm = AIR_DB_PER_M * d
    hm = np.maximum((rz + SZ[idx])/2 - (sample_grid(TERR, SX[idx], SY[idx])), 0.5)
    Agr = ground_atten(d, hm)
    excess = np.where(blocked, Abar, Agr)
    lvl = SLW[idx] - Adiv - Aatm - excess
    return lvl, idx

def total_leq(rx, ry, rz=None):
    lvl, idx = leq_at(rx, ry, rz)
    e = np.sum(10**(lvl/10)) + 10**(AMBIENT_LEQ/10)
    return 10*math.log10(e), lvl, idx

def dominant_roads(lvl, idx, n=6):
    agg = {}
    for L, k in zip(lvl, SROAD[idx]):
        agg[k] = agg.get(k, 0.0) + 10**(L/10)
    out = sorted(((10*math.log10(v), k) for k, v in agg.items()), reverse=True)
    return out[:n]


# --------------------------------------------------------------------------- #
#  Receivers.
# --------------------------------------------------------------------------- #
RECEIVERS = [
    ("6 Trowbridge Circle (TARGET)",            42.2940843, -71.7007366),
    ("Edgemere (residential, off Rt 20)",        42.2487048, -71.7411810),
    ("Sherwood Ave (mid-town residential)",      42.2840550, -71.7270729),
    ("Jordan Rd (Fairlawn, near lake)",          42.2670815, -71.7496331),
    ("Shrewsbury Town Hall / center",            42.2915147, -71.7223002),
    ("Reservoir St (far north, near I-290)",     42.3250694, -71.6970145),
    ("Grafton St area (fronting MA-140)",        42.2938516, -71.7128624),
    ("Harrington Ave (off Route 9)",             42.2770477, -71.7443035),
    ("Quinsigamond Ave (lakeside, by Rt 9/290)", 42.2738390, -71.7517402),
]

def perception(delta):
    if abs(delta) < 1.5: return "about the same"
    f = 2.0**(abs(delta)/10.0)
    return f"~{f:.1f}x {'louder' if delta>0 else 'quieter'}"

def run_table():
    rows = []
    for label, lat, lon in RECEIVERS:
        x, y = ll_to_m(lat, lon)
        tot, lvl, idx = total_leq(x, y)
        rows.append((label, tot, dominant_roads(lvl, idx), terr_at(x,y)))
    target = next(t for l,t,_,_ in rows if l.startswith("6 Trowbridge"))
    rows.sort(key=lambda r: r[1])
    print("="*82)
    print("Modelled daytime road-traffic noise, Shrewsbury MA   [A-weighted Leq, dB]")
    print("  measured MassDOT AADT + SRTM terrain + OSM buildings (ISO 9613-2 style)")
    print("="*82)
    print(f"{'location':<44}{'Leq dB(A)':>10}{'elev':>6}{'vs Trowbridge':>22}")
    print("-"*82)
    for label, tot, _, elev in rows:
        if label.startswith("6 Trowbridge"):
            note = "(reference)"
        else:
            note = f"{tot-target:+.0f} dB, {perception(tot-target)}"
        print(f"{label:<44}{tot:>10.1f}{elev:>5.0f}m{note:>22}")
    print("-"*82)
    print(f"(suburban ambient floor = {AMBIENT_LEQ:.0f} dB(A))\n")
    tr = next(r for r in rows if r[0].startswith("6 Trowbridge"))
    print("Dominant sources at 6 Trowbridge Circle:")
    for L, k in tr[2]:
        aadt = ROADMETA.get(k,{}).get("aadt")
        print(f"   {k:<22} {L:5.1f} dB(A)   (AADT {aadt})")


def run_map(step=35.0, radius=2200.0):
    """Town-wide noise map via source energy 'spray' (ground propagation,
    all sources).  Fast: each sub-source adds energy to nearby cells only."""
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    import matplotlib.patheffects as pe
    xs = np.arange(X0+160, X0+(NX-20)*RES, step)
    ys = np.arange(Y0+160, Y0+(NY-20)*RES, step)
    W, H = len(xs), len(ys)
    print(f"[map] {W}x{H} cells, spraying {len(SX)} sub-sources ...", file=sys.stderr)
    energy = np.full((H, W), 10**(AMBIENT_LEQ/10))
    cellterr = sample_grid(TERR, *np.meshgrid(xs, ys)).astype(np.float32)  # (H,W)
    rr = int(radius/step)
    sterr = sample_grid(TERR, SX, SY)
    for i in range(len(SX)):
        cx = int((SX[i]-xs[0])/step); cy = int((SY[i]-ys[0])/step)
        x0=max(cx-rr,0); x1=min(cx+rr,W-1); y0=max(cy-rr,0); y1=min(cy+rr,H-1)
        if x1<x0 or y1<y0: continue
        sub_x = xs[x0:x1+1][None,:]; sub_y = ys[y0:y1+1][:,None]
        d = np.hypot(sub_x-SX[i], sub_y-SY[i])
        Adiv = 20*np.log10(np.maximum(d,D_MIN))+8.0
        Aatm = AIR_DB_PER_M*d
        rz = cellterr[y0:y1+1, x0:x1+1] + H_RCV
        hm = np.maximum((rz+SZ[i])/2 - sterr[i], 0.5)
        Agr = ground_atten(d, hm)
        energy[y0:y1+1, x0:x1+1] += 10**((SLW[i]-Adiv-Aatm-Agr)/10)
        if i % 15000 == 0: print(f"   {i}/{len(SX)}", file=sys.stderr)
    grid = 10*np.log10(energy)
    fig, ax = plt.subplots(figsize=(10,11))
    im = ax.imshow(grid, origin="lower", extent=[xs[0],xs[-1],ys[0],ys[-1]],
                   cmap="turbo", vmin=40, vmax=72, aspect="equal")
    cs = ax.contour(xs, ys, grid, levels=[45,50,55,60,65,70], colors="k", linewidths=0.4, alpha=0.5)
    ax.clabel(cs, fmt="%d", fontsize=7)
    for label, lat, lon in RECEIVERS:
        x,y = ll_to_m(lat,lon)
        if label.startswith("6 Trowbridge"):
            ax.annotate("6 Trowbridge Circle", xy=(x,y), xytext=(x+1250, y+1050),
                        color="white", fontsize=9, weight="bold",
                        path_effects=[pe.withStroke(linewidth=2.5, foreground="k")],
                        arrowprops=dict(arrowstyle="-|>", color="white", lw=1.6,
                                        mutation_scale=14,
                                        path_effects=[pe.withStroke(linewidth=3, foreground="k")]))
        else:
            ax.plot(x,y,"o",color="white",ms=4,mec="k",mew=0.6)
    fig.colorbar(im, ax=ax, label="A-weighted Leq, dB", shrink=0.8)
    ax.set_title("Shrewsbury, MA — modelled daytime road-traffic noise (A-weighted Leq)\n"
                 "measured MassDOT AADT; distance+ground propagation "
                 "(barrier shielding applied only at the labelled sites)")
    ax.set_xlabel("metres east of 6 Trowbridge Circle"); ax.set_ylabel("metres north")
    fig.tight_layout()
    out = os.path.join(HERE, "noise_map.png")
    fig.savefig(out, dpi=130)
    print(f"[map] saved {out}", file=sys.stderr)


if __name__ == "__main__":
    run_table()
    if "--map" in sys.argv:
        run_map()
