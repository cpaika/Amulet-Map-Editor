"""Render a hillshaded top-down image from the ACTUAL generated world (reads the
.mca files back), and fetch the matching satellite image, for visual QA."""
import os
import sys
import io
import json
import struct
import zlib
import math
import numpy as np
import nbtlib
from PIL import Image

import geo
from build import tile_world_origin, COSLAT

BLOCK_RGB = {
    "minecraft:grass_block": (95, 140, 60),
    "minecraft:moss_block": (64, 96, 44),
    "minecraft:water": (54, 92, 140),
    "minecraft:gray_concrete": (92, 92, 96),
    "minecraft:light_gray_concrete": (165, 165, 165),
    "minecraft:sand": (205, 193, 150),
    "minecraft:coarse_dirt": (120, 92, 66),
    "minecraft:dirt": (130, 100, 75),
    "minecraft:stone": (130, 130, 130),
    "minecraft:air": (0, 0, 0),
}


def _name(p):
    return str(p["Name"])


_PAL_RGB_CACHE = {}


def decode_section(sec):
    """Return (16,16,16) array of RGB-ish palette color indices is overkill;
    return (grid_air_mask, grid_rgb) where grid is (16y,16z,16x,3)."""
    pal = [_name(p) for p in sec["block_states"]["palette"]]
    rgb = np.array([BLOCK_RGB.get(nm, (255, 0, 255)) for nm in pal], dtype=np.uint8)
    air = np.array([nm == "minecraft:air" for nm in pal], dtype=bool)
    if "data" not in sec["block_states"]:
        idx = np.zeros(4096, dtype=np.int64)
    else:
        bits = max(4, (len(pal) - 1).bit_length())
        epl = 64 // bits
        longs = np.array(sec["block_states"]["data"], dtype=np.int64).astype(np.uint64)
        mask = np.uint64((1 << bits) - 1)
        vals = np.empty((longs.shape[0], epl), dtype=np.uint64)
        for k in range(epl):
            vals[:, k] = (longs >> np.uint64(bits * k)) & mask
        idx = vals.reshape(-1)[:4096].astype(np.int64)
    return idx.reshape(16, 16, 16), rgb, air  # (y,z,x)


def read_world_surface(world_dir, X0, Z0, W, H):
    """Vectorized: top surface color + height for world bbox, reading .mca."""
    color = np.zeros((H, W, 3), dtype=np.uint8)
    height = np.full((H, W), np.nan, dtype=np.float32)
    cx_lo, cx_hi = X0 >> 4, (X0 + W - 1) >> 4
    cz_lo, cz_hi = Z0 >> 4, (Z0 + H - 1) >> 4
    region_cache = {}
    for cz in range(cz_lo, cz_hi + 1):
        for cx in range(cx_lo, cx_hi + 1):
            rx, rz = cx >> 5, cz >> 5
            if (rx, rz) not in region_cache:
                p = os.path.join(world_dir, "region", f"r.{rx}.{rz}.mca")
                region_cache[(rx, rz)] = open(p, "rb").read() if os.path.exists(p) else None
            data = region_cache[(rx, rz)]
            if data is None:
                continue
            i = (cx & 31) + (cz & 31) * 32
            loc = struct.unpack(">I", b"\x00" + data[i * 4:i * 4 + 3])[0]
            if loc == 0:
                continue
            start = loc * 4096
            length = struct.unpack(">I", data[start:start + 4])[0]
            raw = zlib.decompress(data[start + 5:start + 5 + length - 1])
            nbt = nbtlib.File.parse(io.BytesIO(raw))
            # top surface for the 16x16 columns of this chunk
            top_rgb = np.zeros((16, 16, 3), dtype=np.uint8)   # (z,x)
            top_y = np.full((16, 16), np.nan, dtype=np.float32)
            for sec in sorted(nbt["sections"], key=lambda s: int(s["Y"]), reverse=True):
                grid, rgb, air = decode_section(sec)            # (y,z,x)
                y0 = int(sec["Y"]) * 16
                # scan layers top-down
                for ly in range(15, -1, -1):
                    layer = grid[ly]                            # (z,x)
                    isair = air[layer]
                    need = np.isnan(top_y) & (~isair)
                    if need.any():
                        top_rgb[need] = rgb[layer][need]
                        top_y[need] = y0 + ly
                if not np.isnan(top_y).any():
                    break
            # blit into output
            for lz in range(16):
                wz = cz * 16 + lz; row = wz - Z0
                if row < 0 or row >= H:
                    continue
                for lx in range(16):
                    wx = cx * 16 + lx; col = wx - X0
                    if col < 0 or col >= W:
                        continue
                    if not np.isnan(top_y[lz, lx]):
                        color[row, col] = top_rgb[lz, lx]
                        height[row, col] = top_y[lz, lx]
    return color, height


def hillshade(h, az=315, alt=45):
    h = np.nan_to_num(h, nan=np.nanmin(h[np.isfinite(h)]) if np.isfinite(h).any() else 0)
    gy, gx = np.gradient(h)
    slope = np.pi / 2 - np.arctan(np.hypot(gx, gy))
    aspect = np.arctan2(-gx, gy)
    az_r = math.radians(az); alt_r = math.radians(alt)
    shade = (np.sin(alt_r) * np.sin(slope) +
             np.cos(alt_r) * np.cos(slope) * np.cos(az_r - aspect))
    return np.clip(shade, 0, 1)


def render_tile(world_dir, spec, out_png, max_px=1100):
    X0, Z0, cmx, cmy, merc_w, merc_h = tile_world_origin(
        spec["lon"], spec["lat"], spec["w"], spec["h"])
    W, H = int(spec["w"]), int(spec["h"])
    color, height = read_world_surface(world_dir, X0, Z0, W, H)
    sh = hillshade(height)[..., None]
    shaded = np.clip(color.astype(np.float32) * (0.55 + 0.75 * sh), 0, 255).astype(np.uint8)
    mine = Image.fromarray(shaded)
    # satellite for same extent
    sat = geo.fetch_mosaic(cmx, cmy, merc_w, merc_h,
                           min(W, max_px), int(min(W, max_px) * H / W), "img")
    sat = Image.fromarray(sat)
    # scale both to same display width
    dw = min(W, max_px); dh = int(dw * H / W)
    mine = mine.resize((dw, dh)); sat = sat.resize((dw, dh))
    combo = Image.new("RGB", (dw, dh * 2 + 10), (20, 20, 20))
    combo.paste(sat, (0, 0)); combo.paste(mine, (0, dh + 10))
    combo.save(out_png)
    print("wrote", out_png, "(top=satellite, bottom=minecraft)")


if __name__ == "__main__":
    world_dir = sys.argv[1]; spec_path = sys.argv[2]; idx = int(sys.argv[3])
    out = sys.argv[4]
    specs = json.load(open(spec_path))
    render_tile(world_dir, specs["tiles"][idx], out)
