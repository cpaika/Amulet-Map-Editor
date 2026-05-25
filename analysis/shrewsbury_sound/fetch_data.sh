#!/usr/bin/env bash
# Download the raw open datasets behind the Shrewsbury noise model and rebuild
# the compact committed grids.  Requires: curl, python3, numpy.
#
# Committed already (model runs without re-downloading):
#   ri_roads.json, terrain_grid.npz, building_grid.npz
# This script regenerates them from primary sources.
set -euo pipefail
cd "$(dirname "$0")"
BOX_OVERPASS="42.23,-71.78,42.34,-71.65"   # S,W,N,E
UA="shrewsbury-noise-model/1.0 (research)"

echo "1/3  MassDOT Road Inventory 2021 (per-segment AADT, speed, class) ..."
python3 fetch_ri.py            # writes ri_roads.json (auto-trimmed)

echo "2/3  SRTM 1-arcsec terrain tile (AWS Terrain Tiles / skadi) ..."
curl -s -o N42W072.hgt.gz \
  "https://s3.amazonaws.com/elevation-tiles-prod/skadi/N42/N42W072.hgt.gz"
gunzip -kf N42W072.hgt.gz

echo "3/3  OSM building footprints (Overpass) ..."
cat > buildings.ql <<EOF
[out:json][timeout:180];
( way["building"](${BOX_OVERPASS}); );
out geom;
EOF
curl -s -A "$UA" -H "Accept: application/json" \
  --data-urlencode "data@buildings.ql" \
  "https://overpass-api.de/api/interpreter" -o buildings.json

echo "4/4  OSM open water (Lake Quinsigamond + ponds) ..."
cat > water_all.ql <<EOF
[out:json][timeout:120];
( way["natural"="water"](${BOX_OVERPASS});
  relation["natural"="water"](${BOX_OVERPASS}); );
out geom;
EOF
curl -s -A "$UA" -H "Accept: application/json" \
  --data-urlencode "data@water_all.ql" \
  "https://overpass-api.de/api/interpreter" -o water_all.json

echo "Rebuilding compact grids (terrain, buildings, water) ..."
SOUND_RAW="$(pwd)" python3 build_dataset.py
SOUND_RAW="$(pwd)" python3 build_water.py
echo "Note: current AADT (Traffic Inventory 2024) and house LiDAR are fetched"
echo "      inline by the model build; see sound_model.py / trowbridge3d.py."
echo "Done."
