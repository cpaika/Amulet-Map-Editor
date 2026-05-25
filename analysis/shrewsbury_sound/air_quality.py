"""Air-quality summary + comparison: 6 Trowbridge Circle vs 17A EK Court.

Pulls current conditions + ~1 year of hourly history from the Open-Meteo Air
Quality API (free, no key; Copernicus CAMS model), computes summary stats and
US-AQI category days, and adds LOCAL source increments on top of the regional
background: a calibrated near-road NO2/PM2.5 term and an aircraft term for
Worcester Regional Airport (ORH). Writes per-site and comparison charts.

Run: python3 air_quality.py
"""
import json, os, sys, math, urllib.request, datetime as dt
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
API = "https://air-quality-api.open-meteo.com/v1/air-quality"
ORH = (42.2673, -71.8757)   # Worcester Regional Airport

SITES = [
    ("6 Trowbridge Circle", 42.2940843, -71.7007366),
    ("17A EK Court",        42.2443170, -71.7437820),
]

HOURLY = "us_aqi,pm2_5,pm10,nitrogen_dioxide,ozone,sulphur_dioxide,carbon_monoxide"
def fetch(lat, lon):
    end = dt.date.today() - dt.timedelta(days=2)
    start = end - dt.timedelta(days=365)
    url = (f"{API}?latitude={lat}&longitude={lon}&current={HOURLY}&hourly={HOURLY}"
           f"&start_date={start}&end_date={end}&timezone=America/New_York")
    return json.load(urllib.request.urlopen(url, timeout=60))

# --- near-road increment: dC = A*AADT*exp(-d/L), calibrated to HEI near-road data
L_NO2, L_PM = 150.0, 120.0
A_NO2 = 18.0/(1e5*math.exp(-25/L_NO2))
A_PM  = 4.0 /(1e5*math.exp(-25/L_PM))

def _road_distances(lat, lon):
    d = json.load(open(os.path.join(HERE, "ri_roads.json")))
    MN = 111132.0; ME = 111320.0*math.cos(math.radians(lat))
    def xy(la, lo): return ((lo-lon)*ME, (la-lat)*MN)
    def pseg(ax, ay, bx, by):
        dx, dy = bx-ax, by-ay
        if dx == 0 and dy == 0: return math.hypot(ax, ay)
        t = max(0, min(1, (-(ax)*dx - (ay)*dy)/(dx*dx+dy*dy)))
        return math.hypot(ax+t*dx, ay+t*dy)
    roads = {}
    for f in d["features"]:
        a = f["attributes"]; aadt = a.get("AADT") or 0
        if aadt < 250: continue
        nm = a.get("St_Name") or f"Route {a.get('Route_Number')}"
        best = 1e18
        for path in f["geometry"]["paths"]:
            pm = [xy(p[1], p[0]) for p in path]
            for i in range(len(pm)-1):
                best = min(best, pseg(pm[i][0], pm[i][1], pm[i+1][0], pm[i+1][1]))
        r = roads.setdefault(nm, {"d": best, "aadt": aadt})
        r["d"] = min(r["d"], best); r["aadt"] = max(r["aadt"], aadt)
    return roads

def road_increment(lat, lon):
    roads = _road_distances(lat, lon)
    no2 = pm = 0.0; contrib = []
    for nm, r in roads.items():
        n = A_NO2*r["aadt"]*math.exp(-r["d"]/L_NO2)
        p = A_PM*r["aadt"]*math.exp(-r["d"]/L_PM)
        no2 += n; pm += p
        if n > 0.05: contrib.append((n, nm, r["d"], r["aadt"]))
    contrib.sort(reverse=True)
    return no2, pm, contrib[:4]

def airplane_increment(lat, lon):
    dkm = math.hypot((lat-ORH[0])*111.132, (lon-ORH[1])*111.320*math.cos(math.radians(lat)))
    return 0.2*math.exp(-(dkm-2)/6), 0.03*math.exp(-(dkm-2)/6), dkm

US_AQI_CATS = [(0,50,"Good"),(51,100,"Moderate"),(101,150,"USG"),
               (151,200,"Unhealthy"),(201,300,"Very Unhealthy"),(301,500,"Hazardous")]
def cat(a):
    for lo,hi,nm in US_AQI_CATS:
        if a <= hi: return nm
    return "Hazardous"

def analyze(name, lat, lon):
    d = fetch(lat, lon); c = d["current"]; h = d["hourly"]
    A = lambda k: np.array(h[k], dtype=float)
    aqi, pm25, no2, o3 = A("us_aqi"), A("pm2_5"), A("nitrogen_dioxide"), A("ozone")
    ok = ~np.isnan(aqi)
    t = np.array(h["time"])[ok]
    days = {}
    for ti, a in zip(t, aqi[ok]): days[ti[:10]] = max(days.get(ti[:10], 0), a)
    from collections import Counter
    cc = Counter(cat(a) for a in days.values()); nd = len(days)
    rno2, rpm, rc = road_increment(lat, lon); ano2, apm, adist = airplane_increment(lat, lon)
    return dict(name=name, cur=c, t=t, aqi=aqi[ok], pm25=pm25[ok], no2=no2[ok], o3=o3[ok],
                aqi_mean=np.nanmean(aqi), pm_mean=np.nanmean(pm25), no2_mean=np.nanmean(no2),
                o3_mean=np.nanmean(o3), pm_p95=np.nanpercentile(pm25,95), pm_max=np.nanmax(pm25),
                good=100*cc.get("Good",0)/nd, mod=100*cc.get("Moderate",0)/nd,
                usg=100*(cc.get("USG",0)+cc.get("Unhealthy",0))/nd,
                rno2=rno2, rpm=rpm, rc=rc, ano2=ano2, apm=apm, adist=adist)

def main():
    R = [analyze(*s) for s in SITES]
    print("="*78)
    print("AIR QUALITY COMPARISON  (Open-Meteo / CAMS regional + local increments)")
    print("="*78)
    hdr = f"{'metric':<34}" + "".join(f"{r['name']:>22}" for r in R)
    print(hdr); print("-"*78)
    rows = [
        ("Current US AQI",        lambda r: f"{r['cur']['us_aqi']:.0f} ({cat(r['cur']['us_aqi'])})"),
        ("12-mo US AQI mean",     lambda r: f"{r['aqi_mean']:.0f}"),
        ("12-mo PM2.5 mean ug/m3",lambda r: f"{r['pm_mean']:.1f}"),
        ("   PM2.5 95th / max",   lambda r: f"{r['pm_p95']:.0f} / {r['pm_max']:.0f}"),
        ("12-mo NO2 mean (regional)", lambda r: f"{r['no2_mean']:.1f}"),
        ("12-mo O3 mean ug/m3",   lambda r: f"{r['o3_mean']:.0f}"),
        ("% days Good",           lambda r: f"{r['good']:.0f}%"),
        ("% days Moderate",       lambda r: f"{r['mod']:.0f}%"),
        ("% days USG+",           lambda r: f"{r['usg']:.0f}%"),
        ("LOCAL road NO2 increment", lambda r: f"+{r['rno2']:.1f}"),
        ("LOCAL road PM2.5 increment", lambda r: f"+{r['rpm']:.2f}"),
        ("Aircraft NO2 (ORH)",    lambda r: f"+{r['ano2']:.2f} ({r['adist']:.0f} km)"),
    ]
    for lab, fn in rows:
        print(f"{lab:<34}" + "".join(f"{fn(r):>22}" for r in R))
    print("-"*78)
    for r in R:
        tops = ", ".join(f"{nm} {dd:.0f}m +{n:.1f}" for n,nm,dd,aadt in r["rc"][:3])
        print(f"{r['name']}: top road sources -> {tops}")

    try:
        import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
        # comparison bar chart
        labels = ["PM2.5\nann mean","NO2 regional\nmean","NO2 road\nincrement","O3 mean/10"]
        vals = {r['name']: [r['pm_mean'], r['no2_mean'], r['rno2'], r['o3_mean']/10] for r in R}
        x = np.arange(len(labels)); w = 0.38
        fig, ax = plt.subplots(figsize=(9,5))
        for i,(nm,v) in enumerate(vals.items()):
            ax.bar(x+(i-0.5)*w, v, w, label=nm)
        ax.set_xticks(x); ax.set_xticklabels(labels); ax.set_ylabel("ug/m3 (O3 shown /10)")
        ax.set_title("Air quality: 6 Trowbridge Circle vs 17A EK Court\n(CAMS regional background + modelled local road increment)")
        ax.legend(); ax.axhline(9,ls="--",c="gray",lw=0.8)
        ax.annotate("EPA PM2.5 annual 9.0",(3.3,9.2),fontsize=7,color="gray")
        fig.tight_layout(); fig.savefig(os.path.join(HERE,"air_quality_compare.png"),dpi=130)
        print("saved air_quality_compare.png")
        # keep the 12-month series chart for Trowbridge
        r=R[0]; dts=np.array([dt.datetime.fromisoformat(x) for x in r['t']])
        fig,ax=plt.subplots(2,1,figsize=(11,6),sharex=True)
        ax[0].plot(dts,r['pm25'],lw=0.4,color="tab:red"); ax[0].axhline(9,ls="--",c="k",lw=0.8,label="EPA annual 9.0")
        ax[0].axhline(35,ls=":",c="gray",lw=0.8,label="EPA 24-h 35"); ax[0].set_ylabel("PM2.5 ug/m3"); ax[0].legend(fontsize=7); ax[0].set_ylim(0,None)
        ax[0].set_title("6 Trowbridge Circle — air quality, past 12 months (Open-Meteo / CAMS)")
        ax[1].plot(dts,r['aqi'],lw=0.4,color="tab:blue")
        for lo,hi,col in [(0,50,"#a8e05f"),(51,100,"#fdd64b"),(101,150,"#fe9b57")]:
            ax[1].axhspan(lo,hi,color=col,alpha=0.25)
        ax[1].set_ylabel("US AQI"); ax[1].set_ylim(0,max(120,np.nanmax(r['aqi'])*1.05))
        fig.tight_layout(); fig.savefig(os.path.join(HERE,"trowbridge_air_quality.png"),dpi=130)
        print("saved trowbridge_air_quality.png")
    except Exception as e:
        print("chart skipped:", e)

if __name__ == "__main__":
    main()
