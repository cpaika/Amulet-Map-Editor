"""Geo helpers: web-mercator transforms and tiled raster downloads from the
USGS 3DEP elevation ImageServer and the Esri World Imagery service."""
import math
import time
import io
import urllib.request
import numpy as np
from PIL import Image, ImageFile

ImageFile.LOAD_TRUNCATED_IMAGES = True

R = 20037508.342789244


def lonlat_to_merc(lon, lat):
    x = lon * R / 180.0
    y = math.log(math.tan((90 + lat) * math.pi / 360.0)) / (math.pi / 180.0)
    y = y * R / 180.0
    return x, y


def merc_to_lonlat(x, y):
    lon = x / R * 180.0
    lat = y / R * 180.0
    lat = 180.0 / math.pi * (2 * math.atan(math.exp(lat * math.pi / 180.0)) - math.pi / 2)
    return lon, lat


def _get(url, tries=5):
    last = None
    for i in range(tries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "central-ma-mc/1.0"})
            with urllib.request.urlopen(req, timeout=120) as r:
                return r.read()
        except Exception as e:  # noqa
            last = e
            time.sleep(2 * (i + 1))
    raise last


ELEV_URL = ("https://elevation.nationalmap.gov/arcgis/rest/services/3DEPElevation/"
            "ImageServer/exportImage?bbox={bbox}&bboxSR=3857&imageSR=3857&"
            "size={w},{h}&format=tiff&pixelType=F32&"
            "interpolation=RSP_BilinearInterpolation&f=image")

IMG_URL = ("https://services.arcgisonline.com/ArcGIS/rest/services/World_Imagery/"
           "MapServer/export?bbox={bbox}&bboxSR=3857&imageSR=3857&"
           "size={w},{h}&format=png&f=image")


def fetch_mosaic(cx_merc, cy_merc, merc_w, merc_h, px_w, px_h, kind, tile_px=2000):
    """Download a region given its mercator extent (merc_w x merc_h centered at
    cx,cy) sampled to px_w x px_h pixels, tiled. Row 0 = north edge.
    Separating mercator extent from pixel count lets the caller request true
    1 m/px ground resolution (mercator inflates distance by 1/cos(lat))."""
    W = int(px_w); H = int(px_h)
    mx0 = cx_merc - merc_w / 2.0
    my1 = cy_merc + merc_h / 2.0   # north/top
    sx = merc_w / W                # mercator units per pixel (x)
    sy = merc_h / H
    if kind == "elev":
        out = np.zeros((H, W), dtype=np.float32)
    else:
        out = np.zeros((H, W, 3), dtype=np.uint8)
    for ry in range(0, H, tile_px):
        th = min(tile_px, H - ry)
        for rx in range(0, W, tile_px):
            tw = min(tile_px, W - rx)
            bx0 = mx0 + rx * sx
            bx1 = mx0 + (rx + tw) * sx
            by1 = my1 - ry * sy
            by0 = my1 - (ry + th) * sy
            bbox = f"{bx0},{by0},{bx1},{by1}"
            try:
                if kind == "elev":
                    url = ELEV_URL.format(bbox=bbox, w=tw, h=th)
                    im = Image.open(io.BytesIO(_get(url)))
                    a = np.array(im, dtype=np.float32)
                    if a.shape[:2] != (th, tw):
                        raise ValueError("bad size")
                    out[ry:ry + th, rx:rx + tw] = a[:th, :tw]
                else:
                    url = IMG_URL.format(bbox=bbox, w=tw, h=th)
                    im = Image.open(io.BytesIO(_get(url))).convert("RGB")
                    a = np.array(im, dtype=np.uint8)
                    if a.shape[:2] != (th, tw):
                        raise ValueError("bad size")
                    out[ry:ry + th, rx:rx + tw] = a[:th, :tw]
            except Exception:
                # no coverage (open ocean) -> sea level / neutral water color
                if kind == "elev":
                    out[ry:ry + th, rx:rx + tw] = 0.0
                else:
                    out[ry:ry + th, rx:rx + tw] = (60, 90, 110)
    return out
