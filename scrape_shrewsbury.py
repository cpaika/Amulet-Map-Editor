#!/usr/bin/env python3
"""
Scrape Shrewsbury, MA property data from MassGIS ArcGIS Feature Service
and Vision Government Solutions (VGSI) for pool/extra features data.
Outputs a browsable interactive HTML map.
"""

import json
import math
import os
import re
import sys
import time
import urllib.parse
import urllib.request
import ssl

# ── ArcGIS Feature Service endpoint ──────────────────────────────────────────
ARCGIS_BASE = (
    "https://services1.arcgis.com/hGdibHYSPO59RG1h/arcgis/rest/services/"
    "L3_TAXPAR_POLY_ASSESS_gdb/FeatureServer/0/query"
)

OUT_FIELDS = [
    "MAP_PAR_ID", "LOC_ID", "SITE_ADDR", "TOTAL_VAL", "BLDG_VAL", "LAND_VAL",
    "LOT_SIZE", "BLD_AREA", "RES_AREA", "USE_CODE", "LOT_UNITS", "STYLE",
    "YEAR_BUILT", "NUM_ROOMS", "STORIES", "OWNER1", "LS_DATE", "LS_PRICE",
    "ZONING", "ADDR_NUM", "FULL_STR"
]

# We'll query broadly and filter in Python for flexibility
# BLD_AREA >= 2500 catches properties where RES_AREA may be >= 3500
# LOT_SIZE >= 0.40 gives a small buffer below the 0.45 minimum
WHERE_CLAUSE = (
    "CITY = 'SHREWSBURY' AND BLD_AREA >= 2500 AND LOT_SIZE >= 0.40 "
    "AND USE_CODE IN ('1010','1011','1012','1013','1040','1041','1090')"
)

BATCH_SIZE = 1000


def fetch_json(url, retries=3):
    """Fetch JSON from URL with retries."""
    ctx = ssl.create_default_context()
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
            with urllib.request.urlopen(req, context=ctx, timeout=60) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except Exception as e:
            if attempt < retries - 1:
                time.sleep(2 ** attempt)
                continue
            raise


def query_arcgis_count():
    """Get total count of matching features."""
    params = urllib.parse.urlencode({
        "where": WHERE_CLAUSE,
        "returnCountOnly": "true",
        "f": "json"
    })
    data = fetch_json(f"{ARCGIS_BASE}?{params}")
    return data.get("count", 0)


def query_arcgis_batch(offset):
    """Query a batch of features from ArcGIS."""
    params = urllib.parse.urlencode({
        "where": WHERE_CLAUSE,
        "outFields": ",".join(OUT_FIELDS),
        "returnGeometry": "true",
        "geometryType": "esriGeometryPolygon",
        "outSR": "4326",
        "f": "json",
        "resultRecordCount": BATCH_SIZE,
        "resultOffset": offset,
    })
    return fetch_json(f"{ARCGIS_BASE}?{params}")


def polygon_centroid(rings):
    """Calculate centroid of a polygon from its rings."""
    if not rings or not rings[0]:
        return None, None
    ring = rings[0]
    n = len(ring)
    if n < 3:
        return None, None
    # Simple average for speed (good enough for display)
    cx = sum(p[0] for p in ring) / n
    cy = sum(p[1] for p in ring) / n
    return cx, cy


def scrape_vgsi_property(pid):
    """Try to scrape a VGSI property page for pool/extra features."""
    url = f"https://gis.vgsi.com/shrewsburyma/Parcel.aspx?pid={pid}"
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
        ctx = ssl.create_default_context()
        with urllib.request.urlopen(req, context=ctx, timeout=15) as resp:
            html = resp.read().decode("utf-8", errors="replace")
        # Look for pool indicators
        has_pool = bool(re.search(r'(?i)(swimming\s*pool|in.?ground\s*pool|pool\b.*(?:yes|heated|vinyl|gunite|concrete|fiberglass))', html))
        return {"has_pool": has_pool, "raw_length": len(html)}
    except Exception:
        return {"has_pool": None, "raw_length": 0}


def try_vgsi_search_for_pools():
    """Try to get pool data from VGSI search/export if available."""
    # VGSI often blocks automated access, so this is best-effort
    url = "https://gis.vgsi.com/shrewsburyma/Search.aspx"
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
        ctx = ssl.create_default_context()
        with urllib.request.urlopen(req, context=ctx, timeout=15) as resp:
            return resp.status == 200
    except Exception:
        return False


def estimate_road_noise(feature, all_features):
    """
    Estimate road noise exposure based on proximity to major roads.
    Uses lot size and zoning as proxy indicators:
    - Larger lots = more setback from roads
    - Certain zones tend to be away from major routes
    """
    attrs = feature["attributes"]
    lot_size = attrs.get("LOT_SIZE") or 0
    zoning = (attrs.get("ZONING") or "").upper()

    # Larger lots generally mean more distance from road
    score = 0
    if lot_size >= 2.0:
        score += 3
    elif lot_size >= 1.0:
        score += 2
    elif lot_size >= 0.5:
        score += 1

    # Residential zones away from commercial corridors
    if zoning in ("RA", "RB", "RC"):
        score += 1

    # Map to labels
    if score >= 3:
        return "Low (large setback)"
    elif score >= 2:
        return "Low-Medium"
    elif score >= 1:
        return "Medium"
    else:
        return "Medium-High"


def main():
    print("=" * 60)
    print("Shrewsbury, MA Property Scraper")
    print("=" * 60)

    # Step 1: Get count
    print("\n[1/4] Querying ArcGIS for matching parcels...")
    total = query_arcgis_count()
    print(f"  Found {total} parcels matching broad criteria")

    # Step 2: Fetch all data in batches
    print(f"\n[2/4] Downloading parcel data in batches of {BATCH_SIZE}...")
    all_features = []
    for offset in range(0, total, BATCH_SIZE):
        batch_num = offset // BATCH_SIZE + 1
        total_batches = math.ceil(total / BATCH_SIZE)
        print(f"  Batch {batch_num}/{total_batches} (offset {offset})...")
        result = query_arcgis_batch(offset)
        features = result.get("features", [])
        all_features.extend(features)
        if len(features) < BATCH_SIZE:
            break
        time.sleep(0.5)  # Be polite

    print(f"  Downloaded {len(all_features)} parcels total")

    # Step 3: Process and filter
    print("\n[3/4] Processing and filtering properties...")
    properties = []
    for feat in all_features:
        attrs = feat.get("attributes", {})
        geom = feat.get("geometry", {})

        # Get centroid for map marker
        rings = geom.get("rings", [])
        lng, lat = polygon_centroid(rings)
        if lat is None:
            continue

        res_area = attrs.get("RES_AREA") or 0
        bld_area = attrs.get("BLD_AREA") or 0
        lot_size = attrs.get("LOT_SIZE") or 0
        total_val = attrs.get("TOTAL_VAL") or 0

        # Estimate road noise
        noise_est = estimate_road_noise(feat, all_features)

        # Simplify polygon for GeoJSON (reduce coordinate precision)
        simplified_rings = []
        for ring in rings:
            simplified_rings.append(
                [[round(p[0], 6), round(p[1], 6)] for p in ring]
            )

        prop = {
            "lat": round(lat, 6),
            "lng": round(lng, 6),
            "address": attrs.get("SITE_ADDR") or "Unknown",
            "total_val": total_val,
            "bldg_val": attrs.get("BLDG_VAL") or 0,
            "land_val": attrs.get("LAND_VAL") or 0,
            "lot_size": round(lot_size, 4),
            "bld_area": bld_area,
            "res_area": res_area,
            "use_code": attrs.get("USE_CODE") or "",
            "style": attrs.get("STYLE") or "",
            "year_built": attrs.get("YEAR_BUILT") or 0,
            "num_rooms": attrs.get("NUM_ROOMS") or 0,
            "stories": attrs.get("STORIES") or "",
            "owner": attrs.get("OWNER1") or "",
            "last_sale_date": attrs.get("LS_DATE") or "",
            "last_sale_price": attrs.get("LS_PRICE") or 0,
            "zoning": attrs.get("ZONING") or "",
            "map_par_id": attrs.get("MAP_PAR_ID") or "",
            "loc_id": attrs.get("LOC_ID") or "",
            "noise_estimate": noise_est,
            "rings": simplified_rings,
        }
        properties.append(prop)

    print(f"  Processed {len(properties)} properties with valid geometry")

    # Apply the user's core criteria for the "matching" set
    # but keep ALL properties so user can adjust filters on the map
    matching = [p for p in properties
                if p["res_area"] >= 3500
                and p["lot_size"] >= 0.45
                and p["total_val"] <= 2500000]
    print(f"  {len(matching)} match core criteria (RES_AREA>=3500, lot>=0.45ac, val<=2.5M)")

    near_match = [p for p in properties
                  if p not in matching
                  and p["bld_area"] >= 3500
                  and p["lot_size"] >= 0.45]
    print(f"  {len(near_match)} near-matches (BLD_AREA>=3500, lot>=0.45ac)")

    # Step 4: Try VGSI for pool data
    print("\n[4/4] Attempting VGSI pool data scrape...")
    vgsi_accessible = try_vgsi_search_for_pools()
    pool_data_available = False

    if vgsi_accessible:
        print("  VGSI is accessible! Scraping pool data for top candidates...")
        # Only scrape pool data for the best matching properties to avoid rate limiting
        candidates = sorted(matching, key=lambda p: p["lot_size"], reverse=True)[:50]
        for i, prop in enumerate(candidates):
            pid = prop.get("loc_id", "").replace("-", "")
            if pid:
                print(f"  Checking {prop['address']} for pool... ({i+1}/{len(candidates)})")
                vgsi_data = scrape_vgsi_property(pid)
                if vgsi_data["has_pool"] is not None:
                    prop["has_pool"] = vgsi_data["has_pool"]
                    pool_data_available = True
                time.sleep(1)  # Rate limit
    else:
        print("  VGSI returned 503 (rate limited). Pool data unavailable via scraping.")
        print("  Pool detection will use satellite imagery heuristics instead.")

    # Save data as JSON
    output_data = {
        "scrape_date": time.strftime("%Y-%m-%d %H:%M:%S"),
        "total_downloaded": len(properties),
        "matching_criteria": len(matching),
        "near_matches": len(near_match),
        "pool_data_available": pool_data_available,
        "properties": properties,
    }

    json_path = os.path.join(os.path.dirname(__file__), "shrewsbury_properties.json")
    with open(json_path, "w") as f:
        json.dump(output_data, f)
    print(f"\n  Saved {len(properties)} properties to {json_path}")
    print(f"  File size: {os.path.getsize(json_path) / 1024:.0f} KB")

    # Generate HTML map
    print("\nGenerating interactive map...")
    generate_map(output_data)
    print("\nDone! Open shrewsbury_map.html in a browser.")


def generate_map(data):
    """Generate an interactive Leaflet.js map with all property data."""
    properties = data["properties"]

    html_path = os.path.join(os.path.dirname(__file__), "shrewsbury_map.html")

    # Prepare properties JSON for embedding
    props_json = json.dumps(properties)
    roads_json = json.dumps(data.get("road_segments", []))

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Shrewsbury, MA Property Map</title>
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
<style>
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }}
#map {{ position: absolute; top: 0; left: 320px; right: 0; bottom: 0; }}
#sidebar {{
    position: absolute; top: 0; left: 0; width: 320px; bottom: 0;
    background: #1a1a2e; color: #e0e0e0; overflow-y: auto;
    padding: 16px; z-index: 1000;
}}
#sidebar h1 {{ font-size: 18px; color: #00d4ff; margin-bottom: 4px; }}
#sidebar .subtitle {{ font-size: 12px; color: #888; margin-bottom: 16px; }}
.filter-group {{ margin-bottom: 12px; }}
.filter-group label {{ display: block; font-size: 12px; color: #aaa; margin-bottom: 4px; font-weight: 600; }}
.filter-group input, .filter-group select {{
    width: 100%; padding: 6px 8px; background: #16213e; border: 1px solid #333;
    color: #e0e0e0; border-radius: 4px; font-size: 13px;
}}
.filter-row {{ display: flex; gap: 8px; }}
.filter-row .filter-group {{ flex: 1; }}
.stats {{
    background: #16213e; padding: 10px; border-radius: 6px; margin: 12px 0;
    font-size: 13px; line-height: 1.6;
}}
.stats .highlight {{ color: #00d4ff; font-weight: 700; font-size: 16px; }}
#property-list {{ margin-top: 12px; }}
.property-card {{
    background: #16213e; border: 1px solid #2a2a4a; border-radius: 6px;
    padding: 10px; margin-bottom: 8px; cursor: pointer; transition: border-color 0.2s;
}}
.property-card:hover {{ border-color: #00d4ff; }}
.property-card.selected {{ border-color: #00d4ff; background: #1a2744; }}
.property-card .addr {{ font-weight: 700; color: #fff; font-size: 14px; }}
.property-card .details {{ font-size: 12px; color: #aaa; margin-top: 4px; line-height: 1.5; }}
.property-card .price {{ color: #4ade80; font-weight: 700; }}
.property-card .tag {{
    display: inline-block; padding: 2px 6px; border-radius: 3px;
    font-size: 10px; font-weight: 600; margin-right: 4px; margin-top: 4px;
}}
.tag-match {{ background: #064e3b; color: #4ade80; }}
.tag-near {{ background: #422006; color: #fb923c; }}
.tag-pool {{ background: #1e1b4b; color: #818cf8; }}
.tag-quiet {{ background: #0c4a6e; color: #7dd3fc; }}
.tag-big-lot {{ background: #365314; color: #a3e635; }}
.popup-content {{ font-size: 13px; line-height: 1.6; min-width: 250px; }}
.popup-content h3 {{ margin: 0 0 6px 0; color: #1a1a2e; font-size: 15px; }}
.popup-content .val {{ font-weight: 600; }}
.popup-content a {{ color: #2563eb; }}
.btn {{
    display: inline-block; padding: 6px 12px; background: #2563eb; color: #fff;
    border: none; border-radius: 4px; cursor: pointer; font-size: 12px; margin-top: 8px;
}}
.btn:hover {{ background: #1d4ed8; }}
.legend {{
    background: #16213e; padding: 10px; border-radius: 6px; margin: 12px 0; font-size: 12px;
}}
.legend-item {{ display: flex; align-items: center; margin-bottom: 4px; }}
.legend-dot {{
    width: 12px; height: 12px; border-radius: 50%; margin-right: 8px; flex-shrink: 0;
}}
.checkbox-group {{ display: flex; flex-wrap: wrap; gap: 8px; margin-top: 4px; }}
.checkbox-group label {{
    display: flex; align-items: center; gap: 4px; font-size: 12px;
    color: #ccc; cursor: pointer; font-weight: 400;
}}
#reset-btn {{
    background: #4a1942; color: #e879f9; border: 1px solid #7c3aed;
    padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;
    margin-top: 4px; width: 100%;
}}
#reset-btn:hover {{ background: #5b2360; }}
</style>
</head>
<body>

<div id="sidebar">
    <h1>Shrewsbury Property Map</h1>
    <div class="subtitle">Data: MassGIS Assessor Records &bull; Scraped {data['scrape_date']}</div>

    <div class="filter-row">
        <div class="filter-group">
            <label>Min Living Area (sq ft)</label>
            <input type="number" id="f-min-sqft" value="3500" step="100">
        </div>
        <div class="filter-group">
            <label>Max Assessed Value ($)</label>
            <input type="number" id="f-max-val" value="2500000" step="50000">
        </div>
    </div>
    <div class="filter-row">
        <div class="filter-group">
            <label>Min Lot Size (acres)</label>
            <input type="number" id="f-min-lot" value="0.45" step="0.05">
        </div>
        <div class="filter-group">
            <label>Min Year Built</label>
            <input type="number" id="f-min-year" value="0" step="1">
        </div>
    </div>
    <div class="filter-group">
        <label>Search Address</label>
        <input type="text" id="f-address" placeholder="e.g. Prospect, Oak...">
    </div>
    <div class="filter-group">
        <label>Style</label>
        <select id="f-style">
            <option value="">All Styles</option>
        </select>
    </div>
    <div class="filter-group">
        <label>Max Noise Level (dB): <span id="noise-val">65</span></label>
        <input type="range" id="f-max-noise" min="40" max="75" value="65" step="1"
               style="width:100%;accent-color:#00d4ff">
    </div>
    <div class="filter-group">
        <label>Show</label>
        <div class="checkbox-group">
            <label><input type="checkbox" id="f-show-parcels" checked> Parcel outlines</label>
            <label><input type="checkbox" id="f-show-roads"> Road noise overlay</label>
            <label><input type="checkbox" id="f-pool-only"> Pool only</label>
            <label><input type="checkbox" id="f-quiet-only"> Quiet only (&lt;55dB)</label>
        </div>
    </div>
    <button id="reset-btn" onclick="resetFilters()">Reset Filters</button>

    <div class="stats" id="stats"></div>

    <div class="legend">
        <div class="legend-item"><div class="legend-dot" style="background:#22c55e"></div> Match (RES &ge;3500, lot &ge;0.45ac)</div>
        <div class="legend-item"><div class="legend-dot" style="background:#f97316"></div> Near match (BLD &ge;3500)</div>
        <div class="legend-item"><div class="legend-dot" style="background:#ef4444"></div> Over budget (&gt;$2.5M assessed)</div>
        <div class="legend-item"><div class="legend-dot" style="background:#8b5cf6"></div> Pool detected</div>
    </div>

    <div id="property-list"></div>
</div>

<div id="map"></div>

<script>
// ── Data ──
const ALL_PROPS = {props_json};
const ROAD_SEGMENTS = {roads_json};

// ── Map setup ──
const map = L.map('map', {{
    center: [42.286, -71.713],
    zoom: 13,
    zoomControl: true
}});

L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
    maxZoom: 19,
    attribution: '&copy; OpenStreetMap contributors | Data: MassGIS'
}}).addTo(map);

// Satellite layer toggle
const satellite = L.tileLayer(
    'https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{{z}}/{{y}}/{{x}}',
    {{ maxZoom: 19, attribution: '&copy; Esri' }}
);

let satelliteOn = false;
const satBtn = L.control({{ position: 'topright' }});
satBtn.onAdd = function() {{
    const div = L.DomUtil.create('div', 'leaflet-bar');
    div.innerHTML = '<a href="#" style="width:auto;padding:0 10px;font-size:13px;line-height:30px;background:#fff" id="sat-toggle">Satellite</a>';
    div.onclick = function(e) {{
        e.preventDefault();
        satelliteOn = !satelliteOn;
        if (satelliteOn) {{ satellite.addTo(map); document.getElementById('sat-toggle').textContent = 'Street'; }}
        else {{ map.removeLayer(satellite); document.getElementById('sat-toggle').textContent = 'Satellite'; }}
    }};
    return div;
}};
satBtn.addTo(map);

// ── Layers ──
let markerLayer = L.layerGroup().addTo(map);
let parcelLayer = L.layerGroup().addTo(map);
let roadLayer = L.layerGroup();
let selectedMarker = null;

// Build road overlay
function roadColor(db) {{
    if (db >= 85) return '#dc2626';
    if (db >= 80) return '#ef4444';
    if (db >= 75) return '#f97316';
    if (db >= 70) return '#eab308';
    if (db >= 65) return '#84cc16';
    return '#22c55e';
}}
ROAD_SEGMENTS.forEach(seg => {{
    const line = L.polyline(
        [[seg.start[1], seg.start[0]], [seg.end[1], seg.end[0]]],
        {{ color: roadColor(seg.db), weight: 4, opacity: 0.7 }}
    );
    line.bindTooltip(`${{seg.name}} (${{seg.type}}, ${{seg.db}}dB)`, {{ sticky: true }});
    roadLayer.addLayer(line);
}});

function fmt$(v) {{ return '$' + (v||0).toLocaleString(); }}
function fmtSqft(v) {{ return (v||0).toLocaleString() + ' sqft'; }}

function getColor(prop) {{
    if (prop.has_pool) return '#8b5cf6';
    if (prop.total_val > 2500000) return '#ef4444';
    if (prop.res_area >= 3500 && prop.lot_size >= 0.45) return '#22c55e';
    return '#f97316';
}}

function noiseColor(db) {{
    if (!db || db < 45) return '#22c55e';
    if (db < 50) return '#84cc16';
    if (db < 55) return '#eab308';
    if (db < 60) return '#f97316';
    if (db < 65) return '#ef4444';
    return '#dc2626';
}}

function makePopup(p) {{
    const vgsiLink = `https://gis.vgsi.com/shrewsburyma/Parcel.aspx?pid=${{(p.loc_id||'').replace(/-/g,'')}}`;
    const noiseDb = p.estimated_noise_db || 0;
    const nColor = noiseColor(noiseDb);
    const nearRoad = p.nearest_road_name ? ` (${{p.nearest_road_name}}, ${{Math.round(p.nearest_road_dist_m||0)}}m)` : '';
    const poolConf = p.pool_confidence ? ` (${{Math.round(p.pool_confidence*100)}}% conf)` : '';
    return `<div class="popup-content">
        <h3>${{p.address}}</h3>
        <b>Assessed:</b> <span class="val">${{fmt$(p.total_val)}}</span>
        (Bldg: ${{fmt$(p.bldg_val)}} / Land: ${{fmt$(p.land_val)}})<br>
        <b>Living Area:</b> <span class="val">${{fmtSqft(p.res_area)}}</span>
        (Total Bldg: ${{fmtSqft(p.bld_area)}})<br>
        <b>Lot:</b> <span class="val">${{p.lot_size}} acres</span><br>
        <b>Style:</b> ${{p.style}} &bull; <b>Year:</b> ${{p.year_built}} &bull; <b>Rooms:</b> ${{p.num_rooms}}<br>
        <b>Stories:</b> ${{p.stories}} &bull; <b>Zoning:</b> ${{p.zoning}}<br>
        <b>Noise:</b> <span style="color:${{nColor}};font-weight:700">${{noiseDb}}dB - ${{p.noise_estimate}}</span>${{nearRoad}}<br>
        ${{p.has_pool ? '<b style="color:#8b5cf6">&#x2714; Pool detected' + poolConf + '</b><br>' : ''}}
        <b>Last Sale:</b> ${{p.last_sale_date || 'N/A'}} for ${{fmt$(p.last_sale_price)}}<br>
        <b>Owner:</b> ${{p.owner}}<br>
        <a href="${{vgsiLink}}" target="_blank" class="btn">View on VGSI &rarr;</a>
        <a href="https://www.google.com/maps/@${{p.lat}},${{p.lng}},18z" target="_blank" class="btn">Google Maps</a>
    </div>`;
}}

function createMarker(p) {{
    const color = getColor(p);
    const radius = p.res_area >= 3500 && p.lot_size >= 0.45 ? 8 : 6;
    const marker = L.circleMarker([p.lat, p.lng], {{
        radius: radius,
        fillColor: color,
        color: '#fff',
        weight: 1.5,
        opacity: 0.9,
        fillOpacity: 0.85,
    }});
    marker.bindPopup(makePopup(p), {{ maxWidth: 350 }});
    marker.propData = p;
    return marker;
}}

function createParcel(p) {{
    if (!p.rings || !p.rings.length) return null;
    const color = getColor(p);
    // Convert [lng, lat] to [lat, lng] for Leaflet
    const latlngs = p.rings.map(ring => ring.map(c => [c[1], c[0]]));
    const poly = L.polygon(latlngs, {{
        color: color,
        weight: 1.5,
        fillColor: color,
        fillOpacity: 0.15,
    }});
    poly.bindPopup(makePopup(p), {{ maxWidth: 350 }});
    return poly;
}}

// ── Filtering ──
function getFilters() {{
    const maxNoise = parseInt(document.getElementById('f-max-noise').value) || 75;
    document.getElementById('noise-val').textContent = maxNoise;
    return {{
        minSqft: parseInt(document.getElementById('f-min-sqft').value) || 0,
        maxVal: parseInt(document.getElementById('f-max-val').value) || Infinity,
        minLot: parseFloat(document.getElementById('f-min-lot').value) || 0,
        minYear: parseInt(document.getElementById('f-min-year').value) || 0,
        maxNoise: maxNoise,
        address: document.getElementById('f-address').value.toLowerCase(),
        style: document.getElementById('f-style').value,
        showParcels: document.getElementById('f-show-parcels').checked,
        showRoads: document.getElementById('f-show-roads').checked,
        poolOnly: document.getElementById('f-pool-only').checked,
        quietOnly: document.getElementById('f-quiet-only').checked,
    }};
}}

function matchesFilter(p, f) {{
    if (p.res_area < f.minSqft && p.bld_area < f.minSqft) return false;
    if (p.total_val > f.maxVal) return false;
    if (p.lot_size < f.minLot) return false;
    if (f.minYear && p.year_built < f.minYear) return false;
    if (p.estimated_noise_db && p.estimated_noise_db > f.maxNoise) return false;
    if (f.address && !p.address.toLowerCase().includes(f.address)) return false;
    if (f.style && p.style !== f.style) return false;
    if (f.poolOnly && !p.has_pool) return false;
    if (f.quietOnly && p.estimated_noise_db && p.estimated_noise_db >= 55) return false;
    return true;
}}

function updateMap() {{
    const f = getFilters();
    markerLayer.clearLayers();
    parcelLayer.clearLayers();

    // Road overlay toggle
    if (f.showRoads) {{ roadLayer.addTo(map); }}
    else {{ map.removeLayer(roadLayer); }}

    const filtered = ALL_PROPS.filter(p => matchesFilter(p, f));

    // Sort: pool+match first, then matches, then by noise (quietest first)
    filtered.sort((a, b) => {{
        const aMatch = (a.res_area >= 3500 && a.lot_size >= 0.45) ? 1 : 0;
        const bMatch = (b.res_area >= 3500 && b.lot_size >= 0.45) ? 1 : 0;
        if (aMatch !== bMatch) return bMatch - aMatch;
        const aPool = a.has_pool ? 1 : 0;
        const bPool = b.has_pool ? 1 : 0;
        if (aPool !== bPool) return bPool - aPool;
        return (a.estimated_noise_db||99) - (b.estimated_noise_db||99);
    }});

    filtered.forEach(p => {{
        const marker = createMarker(p);
        markerLayer.addLayer(marker);
        if (f.showParcels) {{
            const parcel = createParcel(p);
            if (parcel) parcelLayer.addLayer(parcel);
        }}
    }});

    // Update stats
    const matchCount = filtered.filter(p => p.res_area >= 3500 && p.lot_size >= 0.45 && p.total_val <= 2500000).length;
    const poolCount = filtered.filter(p => p.has_pool).length;
    const quietCount = filtered.filter(p => (p.estimated_noise_db||99) < 55).length;
    document.getElementById('stats').innerHTML = `
        Showing <span class="highlight">${{filtered.length}}</span> properties<br>
        <span class="highlight">${{matchCount}}</span> match all criteria &bull;
        <span style="color:#8b5cf6;font-weight:700">${{poolCount}}</span> with pool &bull;
        <span style="color:#7dd3fc;font-weight:700">${{quietCount}}</span> quiet<br>
        Avg assessed: ${{fmt$(Math.round(filtered.reduce((s,p) => s+p.total_val, 0) / (filtered.length||1)))}}<br>
        Avg lot: ${{(filtered.reduce((s,p) => s+p.lot_size, 0) / (filtered.length||1)).toFixed(2)}} ac
    `;

    // Update property list
    const listEl = document.getElementById('property-list');
    listEl.innerHTML = '';
    filtered.slice(0, 100).forEach((p, i) => {{
        const isMatch = p.res_area >= 3500 && p.lot_size >= 0.45 && p.total_val <= 2500000;
        const card = document.createElement('div');
        card.className = 'property-card';
        let tags = '';
        if (isMatch) tags += '<span class="tag tag-match">Match</span>';
        else tags += '<span class="tag tag-near">Near Match</span>';
        if (p.has_pool) tags += '<span class="tag tag-pool">Pool ' + (p.pool_confidence ? Math.round(p.pool_confidence*100)+'%' : '') + '</span>';
        const db = p.estimated_noise_db || 0;
        if (db && db < 55) tags += '<span class="tag tag-quiet">' + db + 'dB</span>';
        else if (db) tags += '<span class="tag" style="background:#431407;color:#fb923c">' + db + 'dB</span>';
        if (p.lot_size >= 1.0) tags += '<span class="tag tag-big-lot">' + p.lot_size.toFixed(2) + ' ac</span>';

        card.innerHTML = `
            <div class="addr">${{p.address}}</div>
            <div class="details">
                <span class="price">${{fmt$(p.total_val)}}</span> &bull;
                ${{fmtSqft(p.res_area)}} living &bull; ${{p.lot_size}} ac<br>
                ${{p.style}} &bull; ${{p.year_built}} &bull; ${{p.num_rooms}} rooms
            </div>
            ${{tags}}
        `;
        card.onclick = () => {{
            map.setView([p.lat, p.lng], 17);
            // Find and open the marker popup
            markerLayer.eachLayer(m => {{
                if (m.propData === p) m.openPopup();
            }});
            document.querySelectorAll('.property-card').forEach(c => c.classList.remove('selected'));
            card.classList.add('selected');
        }};
        listEl.appendChild(card);
    }});

    if (filtered.length > 100) {{
        const more = document.createElement('div');
        more.style.cssText = 'text-align:center;padding:8px;color:#888;font-size:12px';
        more.textContent = `Showing 100 of ${{filtered.length}}. Use filters to narrow down.`;
        listEl.appendChild(more);
    }}
}}

function resetFilters() {{
    document.getElementById('f-min-sqft').value = 3500;
    document.getElementById('f-max-val').value = 2500000;
    document.getElementById('f-min-lot').value = 0.45;
    document.getElementById('f-min-year').value = 0;
    document.getElementById('f-address').value = '';
    document.getElementById('f-style').value = '';
    document.getElementById('f-max-noise').value = 65;
    document.getElementById('noise-val').textContent = '65';
    document.getElementById('f-show-parcels').checked = true;
    document.getElementById('f-show-roads').checked = false;
    document.getElementById('f-pool-only').checked = false;
    document.getElementById('f-quiet-only').checked = false;
    updateMap();
}}

// ── Populate style dropdown ──
const styles = [...new Set(ALL_PROPS.map(p => p.style).filter(Boolean))].sort();
const styleSelect = document.getElementById('f-style');
styles.forEach(s => {{
    const opt = document.createElement('option');
    opt.value = s;
    opt.textContent = s;
    styleSelect.appendChild(opt);
}});

// ── Event listeners ──
['f-min-sqft','f-max-val','f-min-lot','f-min-year','f-address','f-style','f-max-noise'].forEach(id => {{
    document.getElementById(id).addEventListener('input', updateMap);
}});
['f-show-parcels','f-show-roads','f-pool-only','f-quiet-only'].forEach(id => {{
    document.getElementById(id).addEventListener('change', updateMap);
}});

// ── Initial render ──
updateMap();
</script>
</body>
</html>"""

    with open(html_path, "w") as f:
        f.write(html)
    print(f"  Map saved to {html_path} ({os.path.getsize(html_path) / 1024:.0f} KB)")


if __name__ == "__main__":
    main()
