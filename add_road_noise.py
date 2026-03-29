#!/usr/bin/env python3
"""
Enhance road noise estimates using actual road data from OpenStreetMap.
Queries Overpass API for major roads in Shrewsbury and calculates
distance from each property to the nearest major road.
"""

import json
import math
import os
import time
import urllib.request
import urllib.parse
import ssl


# Major road types that generate significant noise
NOISY_ROAD_TYPES = {
    "motorway": 85,      # ~85 dB at 50ft
    "motorway_link": 80,
    "trunk": 80,          # ~80 dB
    "trunk_link": 75,
    "primary": 75,        # ~75 dB (e.g., Route 9, Route 20)
    "primary_link": 70,
    "secondary": 70,      # ~70 dB
    "secondary_link": 65,
    "tertiary": 60,       # ~60 dB
}


def haversine_meters(lat1, lng1, lat2, lng2):
    """Distance between two lat/lng points in meters."""
    R = 6371000
    dlat = math.radians(lat2 - lat1)
    dlng = math.radians(lng2 - lng1)
    a = (math.sin(dlat/2)**2 +
         math.cos(math.radians(lat1)) * math.cos(math.radians(lat2)) *
         math.sin(dlng/2)**2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1-a))


def point_to_segment_distance(px, py, ax, ay, bx, by):
    """Distance from point (px,py) to line segment (ax,ay)-(bx,by) in coordinate space."""
    dx = bx - ax
    dy = by - ay
    if dx == 0 and dy == 0:
        return math.sqrt((px-ax)**2 + (py-ay)**2)
    t = max(0, min(1, ((px-ax)*dx + (py-ay)*dy) / (dx*dx + dy*dy)))
    proj_x = ax + t * dx
    proj_y = ay + t * dy
    return haversine_meters(py, px, proj_y, proj_x)


def fetch_overpass_roads():
    """Fetch major roads in Shrewsbury from OpenStreetMap Overpass API."""
    # Bounding box for Shrewsbury, MA (approximate)
    bbox = "42.24,-71.78,42.34,-71.64"

    query = f"""
    [out:json][timeout:60];
    (
      way["highway"~"motorway|trunk|primary|secondary|tertiary"]({bbox});
    );
    out body;
    >;
    out skel qt;
    """

    url = "https://overpass-api.de/api/interpreter"
    data = urllib.parse.urlencode({"data": query}).encode()

    ctx = ssl.create_default_context()
    req = urllib.request.Request(url, data=data, headers={"User-Agent": "Mozilla/5.0"})

    for attempt in range(3):
        try:
            with urllib.request.urlopen(req, context=ctx, timeout=60) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except Exception as e:
            print(f"  Overpass attempt {attempt+1} failed: {e}")
            time.sleep(5)
    return None


def parse_road_segments(osm_data):
    """Parse OSM data into road segments with noise levels."""
    if not osm_data or "elements" not in osm_data:
        return []

    # Build node lookup
    nodes = {}
    for elem in osm_data["elements"]:
        if elem["type"] == "node":
            nodes[elem["id"]] = (elem["lon"], elem["lat"])

    # Build road segments
    segments = []
    for elem in osm_data["elements"]:
        if elem["type"] != "way":
            continue
        highway_type = elem.get("tags", {}).get("highway", "")
        if highway_type not in NOISY_ROAD_TYPES:
            continue

        noise_db = NOISY_ROAD_TYPES[highway_type]
        name = elem.get("tags", {}).get("name", highway_type)
        node_ids = elem.get("nodes", [])

        for i in range(len(node_ids) - 1):
            if node_ids[i] in nodes and node_ids[i+1] in nodes:
                segments.append({
                    "start": nodes[node_ids[i]],
                    "end": nodes[node_ids[i+1]],
                    "noise_db": noise_db,
                    "road_type": highway_type,
                    "name": name,
                })

    return segments


def estimate_noise_at_distance(source_db, distance_m):
    """
    Estimate noise level at a given distance from a road.
    Sound drops ~6dB per doubling of distance from a line source,
    with additional atmospheric absorption.
    """
    if distance_m < 15:
        distance_m = 15  # Minimum reference distance

    # Reference distance is 15m (50ft) for road noise measurements
    ref_distance = 15
    # Line source: -3dB per doubling (for traffic as a line source)
    # Point source would be -6dB per doubling
    reduction = 10 * math.log10(distance_m / ref_distance) if distance_m > ref_distance else 0
    # Additional atmospheric absorption (~0.005 dB/m for mid frequencies)
    atm_absorption = 0.005 * max(0, distance_m - ref_distance)

    return source_db - reduction - atm_absorption


def main():
    print("=" * 60)
    print("Road Noise Analysis - OpenStreetMap Data")
    print("=" * 60)

    json_path = os.path.join(os.path.dirname(__file__), "shrewsbury_properties.json")
    with open(json_path) as f:
        data = json.load(f)

    properties = data["properties"]

    # Fetch road data
    print("\n[1/3] Fetching road data from OpenStreetMap...")
    osm_data = fetch_overpass_roads()
    if not osm_data:
        print("  ERROR: Could not fetch road data. Using fallback estimates.")
        return

    segments = parse_road_segments(osm_data)
    print(f"  Found {len(segments)} road segments")

    # Count by type
    type_counts = {}
    for s in segments:
        t = s["road_type"]
        type_counts[t] = type_counts.get(t, 0) + 1
    for t, c in sorted(type_counts.items()):
        print(f"    {t}: {c} segments ({NOISY_ROAD_TYPES[t]} dB)")

    # Calculate noise for each property
    print(f"\n[2/3] Calculating noise exposure for {len(properties)} properties...")

    for i, prop in enumerate(properties):
        plng, plat = prop["lng"], prop["lat"]

        min_distance = float("inf")
        nearest_road = ""
        nearest_type = ""
        worst_noise = 0

        for seg in segments:
            dist = point_to_segment_distance(
                plng, plat,
                seg["start"][0], seg["start"][1],
                seg["end"][0], seg["end"][1]
            )

            noise_at_prop = estimate_noise_at_distance(seg["noise_db"], dist)

            if noise_at_prop > worst_noise:
                worst_noise = noise_at_prop
                min_distance = dist
                nearest_road = seg["name"]
                nearest_type = seg["road_type"]

        prop["nearest_road_dist_m"] = round(min_distance, 1)
        prop["nearest_road_name"] = nearest_road
        prop["nearest_road_type"] = nearest_type
        prop["estimated_noise_db"] = round(worst_noise, 1)

        # Classify
        if worst_noise < 45:
            prop["noise_estimate"] = "Very Quiet (<45 dB)"
        elif worst_noise < 50:
            prop["noise_estimate"] = "Quiet (45-50 dB)"
        elif worst_noise < 55:
            prop["noise_estimate"] = "Low (50-55 dB)"
        elif worst_noise < 60:
            prop["noise_estimate"] = "Moderate (55-60 dB)"
        elif worst_noise < 65:
            prop["noise_estimate"] = "Moderate-High (60-65 dB)"
        else:
            prop["noise_estimate"] = "High (65+ dB)"

        if (i + 1) % 500 == 0:
            print(f"  ...{i+1}/{len(properties)}")

    # Save
    print(f"\n[3/3] Saving updated data...")

    # Store road segments for map visualization
    road_data = []
    for seg in segments:
        road_data.append({
            "start": list(seg["start"]),
            "end": list(seg["end"]),
            "type": seg["road_type"],
            "name": seg["name"],
            "db": seg["noise_db"]
        })
    data["road_segments"] = road_data

    with open(json_path, "w") as f:
        json.dump(data, f)

    # Stats
    matching = [p for p in properties
                if p["res_area"] >= 3500 and p["lot_size"] >= 0.45 and p["total_val"] <= 2500000]

    quiet_matches = [p for p in matching if p.get("estimated_noise_db", 99) < 50]
    pool_quiet = [p for p in quiet_matches if p.get("has_pool")]

    print(f"\n  Matching properties: {len(matching)}")
    print(f"  Quiet matches (<50 dB): {len(quiet_matches)}")
    print(f"  Quiet + Pool: {len(pool_quiet)}")

    # Regenerate map
    print("\nRegenerating map with noise data...")
    import scrape_shrewsbury
    scrape_shrewsbury.generate_map(data)

    # Print quietest matches
    matching.sort(key=lambda p: p.get("estimated_noise_db", 99))
    print(f"\nQUIETEST MATCHING PROPERTIES:")
    for p in matching[:15]:
        pool = "POOL" if p.get("has_pool") else "    "
        print(f"  {p['address']:30s} {p['res_area']:>5d}sqft  {p['lot_size']:.2f}ac  "
              f"${p['total_val']:>10,}  {p.get('estimated_noise_db',0):.0f}dB  "
              f"{p.get('nearest_road_name','')}  {pool}")


if __name__ == "__main__":
    main()
