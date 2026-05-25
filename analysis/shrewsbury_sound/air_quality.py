"""Air-quality summary for 6 Trowbridge Circle, Shrewsbury MA.

Pulls current conditions + ~1 year of hourly history from the Open-Meteo Air
Quality API (free, no key; based on the Copernicus CAMS model), computes
summary statistics and US-AQI category days, and writes a chart. Adds context
vs. US EPA standards.

Run: python3 air_quality.py
"""
import json, os, sys, urllib.request, datetime as dt
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
LAT, LON = 42.2940843, -71.7007366
API = "https://air-quality-api.open-meteo.com/v1/air-quality"

def fetch():
    end = dt.date.today() - dt.timedelta(days=2)
    start = end - dt.timedelta(days=365)
    hourly = "us_aqi,pm2_5,pm10,nitrogen_dioxide,ozone,sulphur_dioxide,carbon_monoxide"
    url = (f"{API}?latitude={LAT}&longitude={LON}"
           f"&current={hourly}"
           f"&hourly={hourly}"
           f"&start_date={start}&end_date={end}&timezone=America/New_York")
    return json.load(urllib.request.urlopen(url, timeout=60))

import math
# --- local source increments (added on top of the CAMS regional background) ---
# Calibrated decay model for the near-road pollution increment (HEI 2010: road
# pollutants are elevated within ~150-500 m, ~exponential falloff).
#   dC(d) = A * AADT * exp(-d/L)
# A chosen so a 100k-AADT highway at a 25 m kerb gives a literature-typical
# increment (NO2 ~ +18 ug/m3, PM2.5 ~ +4 ug/m3).
L_NO2, L_PM = 150.0, 120.0
A_NO2 = 18.0/(1e5*math.exp(-25/L_NO2))
A_PM  = 4.0 /(1e5*math.exp(-25/L_PM))

def _house_road_distances():
    """Nearest distance (m) and AADT for each road, from 6 Trowbridge Circle."""
    d=json.load(open(os.path.join(HERE,"ri_roads.json")))
    MN=111132.0; ME=111320.0*math.cos(math.radians(LAT))
    def xy(lat,lon): return ((lon-LON)*ME,(lat-LAT)*MN)
    def pseg(px,py,ax,ay,bx,by):
        dx,dy=bx-ax,by-ay
        if dx==0 and dy==0: return math.hypot(px-ax,py-ay)
        t=max(0,min(1,((px-ax)*dx+(py-ay)*dy)/(dx*dx+dy*dy)))
        return math.hypot(px-(ax+t*dx),py-(ay+t*dy))
    roads={}
    for f in d["features"]:
        a=f["attributes"]; aadt=a.get("AADT") or 0
        if aadt<250: continue
        nm=a.get("St_Name") or (f"Route {a.get('Route_Number')}")
        best=1e18
        for path in f["geometry"]["paths"]:
            pm=[xy(p[1],p[0]) for p in path]
            for i in range(len(pm)-1):
                best=min(best,pseg(0,0,pm[i][0],pm[i][1],pm[i+1][0],pm[i+1][1]))
        r=roads.setdefault(nm,{"d":best,"aadt":aadt})
        r["d"]=min(r["d"],best); r["aadt"]=max(r["aadt"],aadt)
    return roads

def road_increment():
    roads=_house_road_distances()
    no2=pm=0.0; contrib=[]
    for nm,r in roads.items():
        n=A_NO2*r["aadt"]*math.exp(-r["d"]/L_NO2)
        p=A_PM *r["aadt"]*math.exp(-r["d"]/L_PM)
        no2+=n; pm+=p
        if n>0.05: contrib.append((n,nm,r["d"],r["aadt"]))
    contrib.sort(reverse=True)
    return no2,pm,contrib[:5]

def airplane_increment():
    """Worcester Regional Airport (ORH) ~9.7 km WSW. LTO (landing/takeoff) NOx/PM
    emissions are concentrated at the airfield; ground-level increment this far
    downwind is negligible. Order-of-magnitude Gaussian-tail estimate."""
    dist_km=9.7
    # ~a few hundred annual jet ops; LTO NOx source tiny vs a highway, and at ~10
    # km a ground-level plume is diluted to <~0.3 ug/m3 NO2 / <~0.05 ug/m3 PM2.5.
    return 0.2*math.exp(-(dist_km-2)/6), 0.03*math.exp(-(dist_km-2)/6), dist_km

US_AQI_CATS = [(0,50,"Good"),(51,100,"Moderate"),(101,150,"USG"),
               (151,200,"Unhealthy"),(201,300,"Very Unhealthy"),(301,500,"Hazardous")]
def cat(aqi):
    for lo,hi,nm in US_AQI_CATS:
        if aqi<=hi: return nm
    return "Hazardous"

def main():
    d = fetch()
    c = d["current"]; u = d["current_units"]
    h = d["hourly"]
    t = np.array(h["time"])
    def arr(k): return np.array(h[k], dtype=float)
    aqi=arr("us_aqi"); pm25=arr("pm2_5"); pm10=arr("pm10")
    no2=arr("nitrogen_dioxide"); o3=arr("ozone")
    ok=~np.isnan(aqi)
    print("="*64)
    print("AIR QUALITY — 6 Trowbridge Circle (Open-Meteo / CAMS)")
    print("="*64)
    print(f"CURRENT ({c['time']}):  US AQI {c['us_aqi']} [{cat(c['us_aqi'])}]")
    for k,lab in [("pm2_5","PM2.5"),("pm10","PM10"),("nitrogen_dioxide","NO2"),
                  ("ozone","O3"),("sulphur_dioxide","SO2"),("carbon_monoxide","CO")]:
        print(f"   {lab:6s} {c[k]:6.1f} {u[k]}")
    print("-"*64)
    n=ok.sum()
    print(f"PAST 12 MONTHS ({n} hourly obs):")
    print(f"   US AQI    mean {np.nanmean(aqi):4.0f}   95th pct {np.nanpercentile(aqi,95):4.0f}   max {np.nanmax(aqi):4.0f}")
    print(f"   PM2.5     mean {np.nanmean(pm25):4.1f}   95th pct {np.nanpercentile(pm25,95):4.1f}   max {np.nanmax(pm25):5.1f}  ug/m3")
    print(f"   NO2       mean {np.nanmean(no2):4.1f}   95th pct {np.nanpercentile(no2,95):4.1f}   max {np.nanmax(no2):5.1f}  ug/m3")
    print(f"   O3        mean {np.nanmean(o3):4.1f}   95th pct {np.nanpercentile(o3,95):4.1f}   max {np.nanmax(o3):5.1f}  ug/m3")
    # AQI category days (by daily max AQI)
    days={}
    for ti,a in zip(t[ok],aqi[ok]):
        day=ti[:10]; days[day]=max(days.get(day,0),a)
    from collections import Counter
    cc=Counter(cat(a) for a in days.values()); nd=len(days)
    print(f"   Daily-max AQI category over {nd} days:")
    for _,_,nm in US_AQI_CATS:
        if cc.get(nm): print(f"      {nm:14s} {cc[nm]:3d} days ({100*cc[nm]/nd:.0f}%)")
    print("-"*64)
    print("US EPA annual PM2.5 standard = 9.0 ug/m3 (2024); 24-h = 35 ug/m3.")
    annual_pm=np.nanmean(pm25)
    print(f"   Trowbridge annual-mean PM2.5 ~ {annual_pm:.1f} ug/m3 "
          f"({'BELOW' if annual_pm<9 else 'ABOVE'} the 9.0 standard).")
    print("-"*64)
    print("LOCAL SOURCE BREAKDOWN (increment ABOVE regional background):")
    rno2,rpm,rc=road_increment(); ano2,apm,adist=airplane_increment()
    print(f"   Road traffic   NO2 +{rno2:4.1f}  PM2.5 +{rpm:4.2f}  ug/m3")
    for n,nm,dd,aadt in rc:
        print(f"       {nm:<20} {dd:5.0f} m  AADT {aadt:>6}  -> NO2 +{n:.2f}")
    print(f"   Aircraft (ORH, {adist:.0f} km)   NO2 +{ano2:4.2f}  PM2.5 +{apm:4.3f}  ug/m3  (negligible)")
    print(f"   => Local traffic adds ~{rno2:.0f} ug/m3 NO2 to the ~{np.nanmean(no2):.0f} regional mean;")
    print(f"      right by I-290/Route 9 this local term would be far larger.")

    # chart: daily PM2.5 and US AQI over the year
    try:
        import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
        dts=np.array([dt.datetime.fromisoformat(x) for x in t[ok]])
        fig,ax=plt.subplots(2,1,figsize=(11,6),sharex=True)
        ax[0].plot(dts,pm25[ok],lw=0.4,color="tab:red"); ax[0].axhline(9,ls="--",c="k",lw=0.8,label="EPA annual 9.0")
        ax[0].axhline(35,ls=":",c="gray",lw=0.8,label="EPA 24-h 35"); ax[0].set_ylabel("PM2.5 ug/m3"); ax[0].legend(fontsize=7); ax[0].set_ylim(0,None)
        ax[0].set_title("6 Trowbridge Circle — air quality, past 12 months (Open-Meteo / CAMS)")
        ax[1].plot(dts,aqi[ok],lw=0.4,color="tab:blue")
        for lo,hi,nm,col in [(0,50,"Good","#a8e05f"),(51,100,"Moderate","#fdd64b"),(101,150,"USG","#fe9b57")]:
            ax[1].axhspan(lo,hi,color=col,alpha=0.25)
        ax[1].set_ylabel("US AQI"); ax[1].set_ylim(0,max(120,np.nanmax(aqi)*1.05))
        fig.tight_layout(); fig.savefig(os.path.join(HERE,"trowbridge_air_quality.png"),dpi=130)
        print("saved trowbridge_air_quality.png")
    except Exception as e:
        print("chart skipped:",e)

if __name__=="__main__":
    main()
