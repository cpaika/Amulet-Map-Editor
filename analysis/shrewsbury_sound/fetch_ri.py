"""Download MassDOT Road Inventory 2021 segments (with measured AADT, speed
limit, lanes, functional class) for the Shrewsbury study box and write a
trimmed ri_roads.json used by the model.

Layer: gis.massdot.state.ma.us .../Roads/RoadInventoryHistory/MapServer/17
"""
import urllib.parse, urllib.request, json, os, time

URL = "https://gis.massdot.state.ma.us/arcgis/rest/services/Roads/RoadInventoryHistory/MapServer/17/query"
ENV = "-71.78,42.23,-71.65,42.34"      # W,S,E,N
FIELDS = "St_Name,AADT,AADT_Year,Speed_Lim,Speed,Num_Lanes,F_Class,Truck_Rte,Route_Number"
KEEP = ["St_Name","AADT","Speed_Lim","Speed","F_Class","Truck_Rte","Route_Number"]
HERE = os.path.dirname(os.path.abspath(__file__))

def fetch(where):
    q = {"geometry": ENV, "geometryType": "esriGeometryEnvelope",
         "inSR": "4326", "outSR": "4326", "spatialRel": "esriSpatialRelIntersects",
         "where": where, "outFields": FIELDS, "returnGeometry": "true", "f": "json"}
    url = URL + "?" + urllib.parse.urlencode(q)
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=120) as r:
                return json.load(r)
        except Exception as e:
            print("  retry", attempt, e); time.sleep(2*(attempt+1))
    raise SystemExit("network failure")

def main():
    seen = {}
    def collect(where, lo=None, hi=None, depth=0):
        d = fetch(where)
        if "error" in d:                      # subdivide on server limit
            if lo is not None and hi-lo > 1 and depth < 24:
                mid = (lo+hi)//2
                collect(f"AADT>{lo} AND AADT<={mid}", lo, mid, depth+1)
                collect(f"AADT>{mid} AND AADT<={hi}", mid, hi, depth+1)
            return
        feats = d.get("features", [])
        if d.get("exceededTransferLimit") and lo is not None and hi-lo > 1 and depth < 24:
            mid = (lo+hi)//2
            collect(f"AADT>{lo} AND AADT<={mid}", lo, mid, depth+1)
            collect(f"AADT>{mid} AND AADT<={hi}", mid, hi, depth+1)
            return
        for f in feats:
            oid = (f["attributes"].get("St_Name"), f["attributes"].get("AADT"),
                   str(f["geometry"].get("paths"))[:60])
            seen[oid] = f
        time.sleep(0.1)

    collect("1=1")
    if not seen:                              # fall back to banded queries
        collect("AADT IS NULL"); collect("AADT>=0 AND AADT<=200000", 0, 200000)

    out = []
    for f in seen.values():
        a = f["attributes"]; paths = (f.get("geometry") or {}).get("paths") or []
        if not paths: continue
        rp = [[[round(x,6), round(y,6)] for x,y in path] for path in paths]
        out.append({"attributes": {k: a.get(k) for k in KEEP}, "geometry": {"paths": rp}})
    json.dump({"spatialReference": {"wkid": 4326}, "features": out},
              open(os.path.join(HERE, "ri_roads.json"), "w"), separators=(",",":"))
    nz = [f["attributes"]["AADT"] for f in out if f["attributes"].get("AADT")]
    print(f"saved {len(out)} segments, {len(nz)} with measured AADT, max {max(nz)}")

if __name__ == "__main__":
    main()
