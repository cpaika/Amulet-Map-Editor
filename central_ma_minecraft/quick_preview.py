"""Fast classification preview: fetch imagery+elevation for a tile at reduced
resolution, classify, and stack satellite over the would-be Minecraft surface
(same top-block + hillshade logic as build.py). For tuning without rebuilding."""
import sys, json, numpy as np
from PIL import Image
import geo, classify as C
from classify import CLASS_BLOCK
from build import tile_world_origin
from render import BLOCK_RGB, hillshade

def preview(spec, out_png, px=900):
    X0, Z0, cmx, cmy, mw, mh = tile_world_origin(spec["lon"], spec["lat"], spec["w"], spec["h"])
    W = px; H = int(px * spec["h"] / spec["w"])
    img = geo.fetch_mosaic(cmx, cmy, mw, mh, W, H, "img")
    elev = geo.fetch_mosaic(cmx, cmy, mw, mh, W, H, "elev")
    cls = C.classify(img, C.dem_roughness(elev))
    rgb = np.zeros((H, W, 3), np.uint8)
    for cid in range(7):
        nm = CLASS_BLOCK[cid]
        rgb[cls == cid] = BLOCK_RGB[nm if isinstance(nm, str) else nm[0]]
    sh = hillshade(elev)[..., None]
    shaded = np.clip(rgb * (0.55 + 0.75 * sh), 0, 255).astype(np.uint8)
    combo = Image.new("RGB", (W, H * 2 + 8), (20, 20, 20))
    combo.paste(Image.fromarray(img), (0, 0))
    combo.paste(Image.fromarray(shaded), (0, H + 8))
    combo.save(out_png)
    frac = {C.GRASS:"grass",C.FOREST:"forest",C.WATER:"water",C.ROAD:"road",
            C.BUILDING:"bldg",C.SAND:"sand",C.DIRT:"dirt"}
    tot = cls.size
    print(out_png, {frac[k]: round(float((cls==k).mean()),3) for k in frac})

if __name__ == "__main__":
    specs = json.load(open(sys.argv[1]))
    preview(specs["tiles"][int(sys.argv[2])], sys.argv[3])
