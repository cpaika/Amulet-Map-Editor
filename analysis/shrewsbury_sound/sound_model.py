"""Environmental road-traffic noise model for Shrewsbury, MA.

Estimates the A-weighted equivalent sound level (Leq, daytime) at 6 Trowbridge
Circle and at a set of comparison locations around town, so the relative
quietness of each spot can be compared.

This is a physics/engineering ESTIMATE, not a measurement. It models the
dominant outdoor noise source in a suburb -- road traffic -- using:

  * Source emission: CoRTN basic noise level (UK DoT "Calculation of Road
    Traffic Noise", 1988), driven by hourly flow, mean speed and % heavy.
  * Propagation: soft-ground line-source spreading (~4.5 dB per distance
    doubling), air absorption (~5 dB/km, A-weighted), and a single built-up
    screening allowance for roads that are blocked by intervening houses.
  * All roads energy-summed, plus a constant suburban ambient floor.

Road geometry (and therefore every source->receiver distance) comes from real
OpenStreetMap data in osm_roads.json, so the comparison is driven by true
distances. Traffic volumes (AADT) are typical values for each road class
(exact MassDOT counts were not available); they shift absolute dB but the
*relative* ranking between locations is dominated by distance and is robust.

Run: python3 sound_model.py
"""

from __future__ import annotations

import json
import math
import os
from dataclasses import dataclass, field

HERE = os.path.dirname(os.path.abspath(__file__))
ROADS_JSON = os.path.join(HERE, "osm_roads.json")

TARGET = (42.2940843, -71.7007366)  # 6 Trowbridge Circle, Shrewsbury MA 01545

# --- suburban ambient floor (no road can make a place quieter than this) ----
AMBIENT_LEQ_DAY = 42.0  # dB(A): birds, HVAC, wind, distant unmodelled sources

# Built-up screening: roads farther than this from a receiver are assumed to be
# blocked by at least one row of houses/trees. A single conservative allowance.
SCREEN_START_M = 150.0
SCREEN_DB = 5.0

AIR_ABSORPTION_DB_PER_M = 0.005  # ~5 dB/km, A-weighted, mild atmosphere

# Fraction of AADT carried in a representative daytime hour (CoRTN 18h basis).
HOURLY_FRACTION = 0.05


# ---------------------------------------------------------------------------
# Traffic parameters per road. Keyed by an identifier we can match from OSM
# tags (ref like "MA 9" / "US 20" / "I 290", else by road name).
# aadt = annual avg daily traffic (veh/day), v = mean speed km/h, p = % heavy.
# ---------------------------------------------------------------------------
@dataclass
class RoadClass:
    aadt: float
    v: float       # km/h
    p: float       # percent heavy vehicles

BY_REF = {
    "I 290":  RoadClass(aadt=78000, v=105, p=8.0),
    "MA 9":   RoadClass(aadt=33000, v=80,  p=5.0),
    "US 20":  RoadClass(aadt=18000, v=70,  p=6.0),
    "MA 140": RoadClass(aadt=16000, v=60,  p=4.0),
    "MA 135": RoadClass(aadt=9000,  v=55,  p=3.0),
    "MA 70":  RoadClass(aadt=9000,  v=55,  p=3.0),
}

# Named local roads that carry real traffic but have no state route number.
BY_NAME = {
    "Main Street":   RoadClass(aadt=9000,  v=45, p=3.0),
    "Maple Avenue":  RoadClass(aadt=12000, v=50, p=4.0),
}

# Defaults by OSM highway class for everything else.
BY_HIGHWAY = {
    "motorway":      RoadClass(aadt=78000, v=105, p=8.0),
    "motorway_link": RoadClass(aadt=8000,  v=60,  p=6.0),
    "trunk":         RoadClass(aadt=30000, v=80,  p=5.0),
    "trunk_link":    RoadClass(aadt=6000,  v=50,  p=4.0),
    "primary":       RoadClass(aadt=15000, v=60,  p=4.0),
    "primary_link":  RoadClass(aadt=4000,  v=45,  p=3.0),
    "secondary":     RoadClass(aadt=6000,  v=50,  p=3.0),
}


# ---------------------------------------------------------------------------
# Geometry helpers (local equirectangular projection centred on TARGET).
# ---------------------------------------------------------------------------
def _m_per_deg(lat_deg: float) -> tuple[float, float]:
    lat = math.radians(lat_deg)
    lat_m = 111132.92 - 559.82 * math.cos(2 * lat) + 1.175 * math.cos(4 * lat)
    lon_m = 111412.84 * math.cos(lat) - 93.5 * math.cos(3 * lat)
    return lat_m, lon_m

_LATM, _LONM = _m_per_deg(TARGET[0])

def _to_xy(lat: float, lon: float) -> tuple[float, float]:
    return ((lon - TARGET[1]) * _LONM, (lat - TARGET[0]) * _LATM)

def _pt_seg_dist(p, a, b) -> float:
    px, py = p; ax, ay = a; bx, by = b
    dx, dy = bx - ax, by - ay
    if dx == 0 and dy == 0:
        return math.hypot(px - ax, py - ay)
    t = max(0.0, min(1.0, ((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)))
    cx, cy = ax + t * dx, ay + t * dy
    return math.hypot(px - cx, py - cy)


# ---------------------------------------------------------------------------
# Road model built from OSM ways.
# ---------------------------------------------------------------------------
@dataclass
class Road:
    name: str
    ref: str
    highway: str
    polylines: list = field(default_factory=list)  # list of [(x,y), ...]

    def traffic(self) -> RoadClass:
        if self.ref in BY_REF:
            return BY_REF[self.ref]
        if self.name in BY_NAME:
            return BY_NAME[self.name]
        return BY_HIGHWAY.get(self.highway, RoadClass(aadt=4000, v=45, p=3.0))

    def distance_to(self, rx: float, ry: float) -> float:
        best = math.inf
        for pl in self.polylines:
            for i in range(len(pl) - 1):
                best = min(best, _pt_seg_dist((rx, ry), pl[i], pl[i + 1]))
            if len(pl) == 1:
                best = min(best, math.hypot(pl[0][0] - rx, pl[0][1] - ry))
        return best


def load_roads() -> dict[str, Road]:
    data = json.load(open(ROADS_JSON))
    roads: dict[str, Road] = {}
    for el in data["elements"]:
        if el.get("type") != "way" or "geometry" not in el:
            continue
        tags = el.get("tags", {})
        hw = tags.get("highway") or tags.get("railway")
        if hw not in BY_HIGHWAY:
            continue  # skip rail and anything we don't price (rail handled as note)
        ref = tags.get("ref", "")
        name = tags.get("name") or ref or "(unnamed)"
        key = ref or name
        if key not in roads:
            roads[key] = Road(name=name, ref=ref, highway=hw)
        roads[key].polylines.append([_to_xy(g["lat"], g["lon"]) for g in el["geometry"]])
        # keep the most major highway class seen for this key
        if BY_HIGHWAY.get(hw, RoadClass(0, 0, 0)).aadt > \
           BY_HIGHWAY.get(roads[key].highway, RoadClass(0, 0, 0)).aadt:
            roads[key].highway = hw
    return roads


# ---------------------------------------------------------------------------
# Acoustics.
# ---------------------------------------------------------------------------
def cortn_leq_at_10m(rc: RoadClass) -> float:
    """CoRTN basic noise level L10(1h) at 10 m, converted to Leq."""
    q = max(1.0, rc.aadt * HOURLY_FRACTION)        # veh/hour
    v = rc.v
    speed_term = 33.0 * math.log10(v + 40.0 + 500.0 / v)
    heavy_term = 10.0 * math.log10(1.0 + 5.0 * rc.p / v)
    l10_10 = 42.2 + 10.0 * math.log10(q) + speed_term + heavy_term - 68.8
    return l10_10 - 3.0  # free-flow Leq is ~3 dB below L10


def level_at_receiver(rc: RoadClass, dist_m: float) -> float:
    d = max(7.5, dist_m)  # don't evaluate inside the carriageway
    src = cortn_leq_at_10m(rc)
    a_dist = 15.0 * math.log10(d / 10.0)           # soft-ground line source (~4.5 dB/doubling)
    a_air = AIR_ABSORPTION_DB_PER_M * d
    a_screen = SCREEN_DB if d > SCREEN_START_M else 0.0
    return src - a_dist - a_air - a_screen


def energy_sum(levels: list[float]) -> float:
    return 10.0 * math.log10(sum(10.0 ** (l / 10.0) for l in levels))


# ---------------------------------------------------------------------------
# Receivers. Either a (lat, lon) coordinate, or explicit per-source distances
# for a clearly-synthetic anchor case.
# ---------------------------------------------------------------------------
@dataclass
class Receiver:
    label: str
    latlon: tuple | None = None
    forced: dict | None = None  # {road_key: distance_m} synthetic override

RECEIVERS = [
    Receiver("6 Trowbridge Circle (TARGET)", latlon=(42.2940843, -71.7007366)),
    Receiver("Edgemere (residential, ~500 m off Rt 20)", latlon=(42.2487048, -71.7411810)),
    Receiver("Sherwood Ave (mid-town residential)", latlon=(42.2840550, -71.7270729)),
    Receiver("Jordan Rd (Fairlawn, near lake)", latlon=(42.2670815, -71.7496331)),
    Receiver("Shrewsbury Town Hall / center", latlon=(42.2915147, -71.7223002)),
    Receiver("Reservoir St (~280 m from I-290)", latlon=(42.3250694, -71.6970145)),
    Receiver("Harrington Ave (~200 m off Route 9)", latlon=(42.2770477, -71.7443035)),
    Receiver("Quinsigamond Ave (lakeside, by Route 9)", latlon=(42.2738390, -71.7517402)),
    Receiver("Grafton St area (fronting MA-140)", latlon=(42.2938516, -71.7128624)),
    Receiver("Typical home fronting Route 9 (25 m)",
             forced={"MA 9": 25.0}),
    Receiver("Typical home along I-290 (60 m)",
             forced={"I 290": 60.0}),
]


def evaluate(receiver: Receiver, roads: dict[str, Road]):
    if receiver.latlon is not None:
        rx, ry = _to_xy(*receiver.latlon)
    contributions = []  # (road_key, dist_m, level_db)
    for key, road in roads.items():
        if receiver.forced is not None:
            if key not in receiver.forced:
                continue
            dist = receiver.forced[key]
        else:
            dist = road.distance_to(rx, ry)
        contributions.append((key, dist, level_at_receiver(road.traffic(), dist)))
    contributions.sort(key=lambda c: -c[2])
    road_levels = [c[2] for c in contributions]
    total = energy_sum(road_levels + [AMBIENT_LEQ_DAY])
    return total, contributions


def perception(delta_db: float) -> str:
    if abs(delta_db) < 1.5:
        return "about the same loudness"
    louder = "louder" if delta_db > 0 else "quieter"
    factor = 2.0 ** (abs(delta_db) / 10.0)  # +10 dB ~ 2x perceived loudness
    return f"~{factor:.1f}x {louder}"


def main():
    roads = load_roads()
    results = []
    for r in RECEIVERS:
        total, contribs = evaluate(r, roads)
        results.append((r, total, contribs))

    target_total = next(t for r, t, _ in results if r.label.startswith("6 Trowbridge"))

    results.sort(key=lambda x: x[1])

    print("=" * 78)
    print("Modelled daytime road-traffic noise, Shrewsbury MA  [A-weighted Leq, dB]")
    print("=" * 78)
    print(f"{'location':<46}{'Leq dB(A)':>10}{'vs Trowbridge':>22}")
    print("-" * 78)
    for r, total, _ in results:
        if r.label.startswith("6 Trowbridge"):
            note = "(reference)"
        else:
            d = total - target_total
            note = f"{d:+.0f} dB, {perception(d)}"
        print(f"{r.label:<46}{total:>10.1f}{note:>22}")
    print("-" * 78)
    print(f"(suburban ambient floor assumed = {AMBIENT_LEQ_DAY:.0f} dB(A))")

    print("\nDominant sources at 6 Trowbridge Circle:")
    target = next((r, t, c) for r, t, c in results if r.label.startswith("6 Trowbridge"))
    for key, dist, lvl in target[2][:6]:
        if lvl < 1:
            continue
        print(f"   {key:<16} {dist:7.0f} m -> {lvl:5.1f} dB(A)")


if __name__ == "__main__":
    main()
