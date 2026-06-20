"""Classify aerial RGB (+ optional DEM roughness) into surface-block categories.

World Imagery over central MA is washed-out and strongly green-biased, with no
usable blue channel, so absolute color thresholds fail. Instead we use
PER-TILE RELATIVE statistics (brightness/greenness percentiles) plus DEM
flatness: open water is the flattest, darkest, least-green surface around."""
import numpy as np

GRASS, FOREST, WATER, ROAD, BUILDING, SAND, DIRT = range(7)

CLASS_BLOCK = {
    GRASS:    "minecraft:grass_block",
    FOREST:   "minecraft:moss_block",
    WATER:    ("minecraft:water", {"level": "0"}),
    ROAD:     "minecraft:gray_concrete",
    BUILDING: "minecraft:light_gray_concrete",
    SAND:     "minecraft:sand",
    DIRT:     "minecraft:coarse_dirt",
}


def classify(rgb, rough=None):
    r = rgb[..., 0].astype(np.float32)
    g = rgb[..., 1].astype(np.float32)
    b = rgb[..., 2].astype(np.float32)
    bright = (r + g + b) / 3.0
    green = g - (r + b) / 2.0          # vegetation index (no NIR available)
    warm = r - b                       # bare soil tends warm

    bp = lambda p: float(np.percentile(bright, p))
    b_dark, b_lo, b_mid, b_hi = bp(12), bp(35), bp(60), bp(82)

    out = np.full(r.shape, GRASS, dtype=np.uint8)

    veg = green >= 7.0
    forest = veg & (bright < b_mid)
    # non-green (impervious / bare / water) split by brightness
    nong = ~veg
    building = nong & (bright >= b_hi)
    road = nong & (bright < b_mid) & (bright >= b_dark)
    sand = nong & (bright >= b_mid) & (warm > 6)
    dirt = nong & (bright >= b_dark) & (bright < b_mid) & (warm > 4)

    # water: flat (if DEM available), dark, and not strongly green
    if rough is not None:
        water = (rough < 0.45) & (bright < b_lo) & (green < 13)
    else:
        water = (bright < b_dark) & (green < 10)

    out[veg] = GRASS
    out[forest] = FOREST
    out[road] = ROAD
    out[sand] = SAND
    out[dirt] = DIRT
    out[building] = BUILDING
    out[water] = WATER
    return out


def dem_roughness(elev):
    gy, gx = np.gradient(elev.astype(np.float32))
    return np.hypot(gx, gy)
