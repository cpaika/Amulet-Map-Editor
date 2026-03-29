#!/usr/bin/env python3
"""
Pool detection via satellite imagery - memory-efficient version.
Processes tiles one at a time, saves results incrementally.
"""

import json
import math
import os
import sys
import time
import urllib.request
import ssl
import io

TILE_URL = "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}"
ZOOM = 19


def lat_lng_to_pixel(lat, lng, zoom):
    n = 2 ** zoom
    x_float = (lng + 180.0) / 360.0 * n
    lat_rad = math.radians(lat)
    y_float = (1.0 - math.log(math.tan(lat_rad) + 1.0 / math.cos(lat_rad)) / math.pi) / 2.0 * n
    tile_x = int(x_float)
    tile_y = int(y_float)
    pixel_x = int((x_float - tile_x) * 256)
    pixel_y = int((y_float - tile_y) * 256)
    return tile_x, tile_y, pixel_x, pixel_y


def fetch_tile(zoom, x, y, retries=3):
    url = TILE_URL.format(z=zoom, x=x, y=y)
    ctx = ssl.create_default_context()
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={
                "User-Agent": "Mozilla/5.0",
                "Referer": "https://www.arcgis.com/"
            })
            with urllib.request.urlopen(req, context=ctx, timeout=30) as resp:
                return resp.read()
        except Exception:
            if attempt < retries - 1:
                time.sleep(1)
    return None


def is_pool_blue(r, g, b):
    if b < 80:
        return False
    if b < r + 20:
        return False
    max_c = max(r, g, b)
    min_c = min(r, g, b)
    if max_c < 60:
        return False
    saturation = (max_c - min_c) / max_c if max_c > 0 else 0
    if saturation < 0.12:
        return False
    if b > 120 and b > r * 1.5 and b > g:
        return True
    if g > r and b > r + 20 and b > 80 and g > 80:
        return True
    if b > 140 and b > r + 30 and g > r:
        return True
    if b > 100 and r < 80 and b > g:
        return True
    return False


def analyze_tile_for_pools(tile_data, properties_on_tile):
    """Analyze one tile for all properties that fall on it. Returns pool results."""
    from PIL import Image

    try:
        img = Image.open(io.BytesIO(tile_data)).convert('RGB')
    except Exception:
        return {addr: (None, 0, 0) for addr in properties_on_tile}

    w, h = img.size
    px_array = img.load()
    results = {}

    for addr, (pixel_x, pixel_y, prop_idx) in properties_on_tile.items():
        search_radius = 50
        min_x = max(0, pixel_x - search_radius)
        max_x = min(w, pixel_x + search_radius)
        min_y = max(0, pixel_y - search_radius)
        max_y = min(h, pixel_y + search_radius)

        blue_pixels = set()
        for y in range(min_y, max_y):
            for x in range(min_x, max_x):
                r, g, b = px_array[x, y]
                if is_pool_blue(r, g, b):
                    blue_pixels.add((x, y))

        if len(blue_pixels) < 10:
            results[addr] = (False, 0, len(blue_pixels))
            continue

        # Find largest cluster
        visited = set()
        max_cluster = 0
        for px in blue_pixels:
            if px in visited:
                continue
            cluster = 0
            queue = [px]
            while queue:
                p = queue.pop()
                if p in visited:
                    continue
                visited.add(p)
                cluster += 1
                cx, cy = p
                for dx in (-2, -1, 0, 1, 2):
                    for dy in (-2, -1, 0, 1, 2):
                        if dx == 0 and dy == 0:
                            continue
                        nb = (cx + dx, cy + dy)
                        if nb in blue_pixels and nb not in visited:
                            queue.append(nb)
            max_cluster = max(max_cluster, cluster)

        if max_cluster >= 20:
            confidence = min(0.95, max_cluster / 200.0)
            results[addr] = (True, round(confidence, 2), max_cluster)
        else:
            results[addr] = (False, 0, len(blue_pixels))

    img.close()
    return results


def main():
    print("=" * 60)
    print("Pool Detection - Memory Efficient")
    print("=" * 60)

    json_path = os.path.join(os.path.dirname(__file__), "shrewsbury_properties.json")
    with open(json_path) as f:
        data = json.load(f)

    properties = data["properties"]

    # Group properties by tile
    candidates = [i for i, p in enumerate(properties)
                  if p["bld_area"] >= 2500 and p["lot_size"] >= 0.40]

    tile_groups = {}
    for idx in candidates:
        p = properties[idx]
        tile_x, tile_y, pixel_x, pixel_y = lat_lng_to_pixel(p["lat"], p["lng"], ZOOM)
        tile_key = (tile_x, tile_y)
        if tile_key not in tile_groups:
            tile_groups[tile_key] = {}
        tile_groups[tile_key][p["address"]] = (pixel_x, pixel_y, idx)

    print(f"Properties to analyze: {len(candidates)}")
    print(f"Unique tiles needed: {len(tile_groups)}")

    pools_found = 0
    tiles_processed = 0
    pool_results = {}  # address -> (has_pool, confidence, cluster)

    for tile_key, props_on_tile in tile_groups.items():
        tile_data = fetch_tile(ZOOM, tile_key[0], tile_key[1])
        if tile_data:
            results = analyze_tile_for_pools(tile_data, props_on_tile)
            for addr, (has_pool, conf, cluster) in results.items():
                pool_results[addr] = (has_pool, conf, cluster)
                if has_pool:
                    pools_found += 1
                    if conf > 0.3:
                        print(f"  [POOL] {addr} - conf: {conf:.0%}, cluster: {cluster}px")

        tiles_processed += 1
        if tiles_processed % 50 == 0:
            print(f"  ...{tiles_processed}/{len(tile_groups)} tiles, {pools_found} pools")

        # Save incrementally every 100 tiles
        if tiles_processed % 100 == 0:
            for p in properties:
                if p["address"] in pool_results:
                    has_pool, conf, cluster = pool_results[p["address"]]
                    p["has_pool"] = has_pool
                    p["pool_confidence"] = conf
                    p["pool_cluster_size"] = cluster
            data["pool_data_available"] = True
            data["pools_detected"] = pools_found
            with open(json_path, "w") as f:
                json.dump(data, f)

        time.sleep(0.03)
        del tile_data  # Free memory

    # Final save
    for p in properties:
        if p["address"] in pool_results:
            has_pool, conf, cluster = pool_results[p["address"]]
            p["has_pool"] = has_pool
            p["pool_confidence"] = conf
            p["pool_cluster_size"] = cluster

    data["pool_data_available"] = True
    data["pools_detected"] = pools_found

    with open(json_path, "w") as f:
        json.dump(data, f)

    print(f"\n{'='*40}")
    print(f"RESULTS: {pools_found} pools detected")
    print(f"Tiles processed: {tiles_processed}")

    # Regenerate map
    print("\nRegenerating map...")
    import scrape_shrewsbury
    scrape_shrewsbury.generate_map(data)

    # Print top matches with pools
    matching_with_pools = [p for p in properties
                           if p.get("has_pool") and p.get("pool_confidence", 0) > 0.3
                           and p["res_area"] >= 3500 and p["lot_size"] >= 0.45
                           and p["total_val"] <= 2500000]
    matching_with_pools.sort(key=lambda p: p["lot_size"], reverse=True)

    if matching_with_pools:
        print(f"\nTOP MATCHES WITH POOLS ({len(matching_with_pools)}):")
        for p in matching_with_pools[:20]:
            print(f"  {p['address']:30s} {p['res_area']:>5d}sqft  {p['lot_size']:.2f}ac  "
                  f"${p['total_val']:>10,}  pool:{p.get('pool_confidence',0):.0%}")

    print("\nDone!")


if __name__ == "__main__":
    main()
