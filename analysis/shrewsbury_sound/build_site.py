"""Build an interactive website (web/) to browse the Shrewsbury sound model:
 - 2D: a Leaflet slippy map with the modelled noise field as a colour overlay
       plus clickable receiver points (real OSM basemap).
 - 3D: the Plotly LiDAR noise model of 6 Trowbridge Circle (embedded).
Run: python3 build_site.py   (recomputes the regional grid, ~1 min)
"""
import os, json, math, shutil, sys
import numpy as np
import matplotlib; matplotlib.use("Agg")
import matplotlib.cm as cm
import sound_model as sm

HERE = os.path.dirname(os.path.abspath(__file__))
WEB = os.path.join(HERE, "web"); os.makedirs(WEB, exist_ok=True)

# ---- regional noise grid (ground-propagation spray, like run_map) ----------
STEP = 30.0
xs = np.arange(sm.X0+150, sm.X0+(sm.NX-15)*sm.RES, STEP)
ys = np.arange(sm.Y0+150, sm.Y0+(sm.NY-15)*sm.RES, STEP)
W, H = len(xs), len(ys)
print(f"[site] noise grid {W}x{H}, spraying {len(sm.SX)} sub-sources ...", file=sys.stderr)
energy = np.full((H, W), 10**(sm.AMBIENT_LEQ/10))
cellterr = sm.sample_grid(sm.TERR, *np.meshgrid(xs, ys)).astype(np.float32)
sterr = sm.sample_grid(sm.TERR, sm.SX, sm.SY)
rr = int(2200/STEP)
for i in range(len(sm.SX)):
    cx = int((sm.SX[i]-xs[0])/STEP); cy = int((sm.SY[i]-ys[0])/STEP)
    x0=max(cx-rr,0); x1=min(cx+rr,W-1); y0=max(cy-rr,0); y1=min(cy+rr,H-1)
    if x1<x0 or y1<y0: continue
    sx=xs[x0:x1+1][None,:]; sy=ys[y0:y1+1][:,None]
    d=np.hypot(sx-sm.SX[i], sy-sm.SY[i])
    Adiv=20*np.log10(np.maximum(d,sm.D_MIN))+8.0; Aatm=sm.AIR_DB_PER_M*d
    rz=cellterr[y0:y1+1,x0:x1+1]+sm.H_RCV
    hm=np.maximum((rz+sm.SZ[i])/2-sterr[i],0.5); Agr=sm.ground_atten(d,hm)
    energy[y0:y1+1,x0:x1+1]+=10**((sm.SLW[i]-Adiv-Aatm-Agr)/10)
    if i%20000==0: print(f"   {i}/{len(sm.SX)}", file=sys.stderr)
grid = 10*np.log10(energy)

# ---- clean georeferenced overlay PNG (turbo, alpha by loudness) -------------
VMIN, VMAX = 40.0, 75.0
norm = np.clip((grid-VMIN)/(VMAX-VMIN), 0, 1)
rgba = cm.turbo(norm)
rgba[..., 3] = np.clip((grid-43)/(70-43), 0.12, 0.82)   # quiet = more transparent
rgba = (rgba[::-1]*255).astype(np.uint8)                # row 0 = north (top)
from matplotlib import image as mpimg
mpimg.imsave(os.path.join(WEB, "noise_overlay.png"), rgba)

south = sm.LAT0 + ys[0]/sm.MN;  north = sm.LAT0 + ys[-1]/sm.MN
west  = sm.LON0 + xs[0]/sm.ME;  east  = sm.LON0 + xs[-1]/sm.ME
bounds = [[south, west], [north, east]]

# ---- receiver markers (full model) -----------------------------------------
target = sm.total_leq(*sm.ll_to_m(42.2940843,-71.7007366))[0]
markers=[]
for label,lat,lon in sm.RECEIVERS:
    leq = sm.total_leq(*sm.ll_to_m(lat,lon))[0]
    markers.append(dict(name=label, lat=lat, lon=lon, leq=round(leq,1),
                        delta=round(leq-target,1),
                        target=label.startswith("6 Trowbridge")))

# ---- copy the 3D model ------------------------------------------------------
shutil.copy(os.path.join(HERE,"trowbridge_noise_3d.html"), os.path.join(WEB,"noise_3d.html"))

# ---- index.html -------------------------------------------------------------
DATA = json.dumps(dict(bounds=bounds, markers=markers, vmin=VMIN, vmax=VMAX,
                       center=[42.286,-71.715]))
HTML = """<!doctype html><html lang=en><head><meta charset=utf-8>
<meta name=viewport content="width=device-width,initial-scale=1">
<title>Shrewsbury Sound — 6 Trowbridge Circle</title>
<link rel=stylesheet href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css">
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
<style>
 :root{--bg:#0f1115;--panel:#1a1d24;--ink:#e8eaed;--mut:#9aa0aa;--acc:#4fc3f7}
 *{box-sizing:border-box} body{margin:0;font:15px/1.5 system-ui,Segoe UI,Roboto,sans-serif;background:var(--bg);color:var(--ink)}
 header{padding:14px 20px;background:var(--panel);border-bottom:1px solid #2a2f3a}
 h1{margin:0;font-size:19px} .sub{color:var(--mut);font-size:13px;margin-top:2px}
 nav{display:flex;gap:8px;padding:10px 20px;background:var(--panel);border-bottom:1px solid #2a2f3a}
 nav button{background:#262b35;color:var(--ink);border:1px solid #333a47;border-radius:8px;padding:7px 14px;cursor:pointer;font-size:14px}
 nav button.on{background:var(--acc);color:#06222e;border-color:var(--acc);font-weight:600}
 .view{display:none} .view.on{display:block}
 #map{height:calc(100vh - 132px)} #d3{height:calc(100vh - 132px);width:100%;border:0;background:#fff}
 .legend{background:rgba(20,23,29,.9);padding:10px 12px;border-radius:8px;color:var(--ink);font-size:12px;line-height:1.4}
 .bar{height:12px;width:200px;border-radius:3px;margin:4px 0;
   background:linear-gradient(90deg,#30123b,#4145ab,#26bce1,#7cf24f,#f5e636,#fb8022,#d23105)}
 .row{display:flex;justify-content:space-between;color:var(--mut)}
 .ctl{position:absolute;z-index:1000;top:12px;right:12px}
 .about{padding:16px 22px;max-width:820px;color:var(--ink)} .about h2{font-size:16px}
 .about p,.about li{color:#c7ccd4} a{color:var(--acc)}
 .leaflet-popup-content{font:13px system-ui} .pin{font-weight:700}
</style></head><body>
<header><h1>Shrewsbury, MA — modelled environmental sound</h1>
<div class=sub>6 Trowbridge Circle · A-weighted daytime Leq · measured MassDOT AADT + 1&nbsp;m LiDAR + ISO&nbsp;9613-2</div></header>
<nav>
 <button id=b2 class=on onclick="show('map')">2D sound map</button>
 <button id=b3 onclick="show('d3')">3D model (the house)</button>
 <button id=ba onclick="show('about')">About</button>
</nav>
<div id="v-map" class="view on"><div id=map></div></div>
<div id="v-d3" class="view"><iframe id=d3 src="noise_3d.html"></iframe></div>
<div id="v-about" class="view about">
 <h2>What this shows</h2>
 <p>Colours are the modelled outdoor <b>A-weighted equivalent sound level (Leq)</b> from road
 traffic, on a typical daytime. Red corridors are I-290, Route&nbsp;9/20 and MA-140; the cool
 blue/green pockets are quiet residential areas. <b>6 Trowbridge Circle</b> (★) sits in a ~50&nbsp;dB
 pocket, quieter than ~79% of homes in town.</p>
 <ul>
 <li><b>2D map</b>: the town-wide field (distance + ground propagation; click a point for its level).</li>
 <li><b>3D model</b>: the immediate area on 1&nbsp;m LiDAR terrain with buildings and full
 barrier/diffraction physics — rotate/zoom to see where sound concentrates.</li>
 </ul>
 <p>Every +10&nbsp;dB ≈ twice as loud. Physically-grounded estimate (~±3&nbsp;dB on absolute level);
 the relative pattern is robust. Built from open data: MassDOT, USGS/MassGIS LiDAR, OpenStreetMap.</p>
</div>
<script>
const D=__DATA__;
function show(v){for(const k of['map','d3','about']){
 document.getElementById('v-'+k).classList.toggle('on',k===v);}
 document.getElementById('b2').classList.toggle('on',v==='map');
 document.getElementById('b3').classList.toggle('on',v==='d3');
 document.getElementById('ba').classList.toggle('on',v==='about');
 if(v==='map')setTimeout(()=>map.invalidateSize(),60);}
const map=L.map('map').setView(D.center,13);
L.tileLayer('https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png',
 {attribution:'© OpenStreetMap, © CARTO',maxZoom:19}).addTo(map);
const overlay=L.imageOverlay('noise_overlay.png',D.bounds,{opacity:0.7}).addTo(map);
map.fitBounds(D.bounds);
function col(v){const t=Math.max(0,Math.min(1,(v-D.vmin)/(D.vmax-D.vmin)));
 const s=['#30123b','#4145ab','#26bce1','#7cf24f','#f5e636','#fb8022','#d23105'];
 const i=Math.min(s.length-2,Math.floor(t*(s.length-1)));return s[i+1];}
for(const m of D.markers){
 const r=L.circleMarker([m.lat,m.lon],{radius:m.target?9:6,color:'#fff',weight:m.target?3:1.5,
   fillColor:col(m.leq),fillOpacity:0.95}).addTo(map);
 const d=m.delta>0?'+'+m.delta:(m.delta===0?'reference':m.delta);
 r.bindPopup('<div class=pin>'+(m.target?'★ ':'')+m.name+'</div>'+m.leq+' dB(A)'+
   (m.target?'':'<br>'+d+' dB vs 6 Trowbridge'));
 if(m.target)r.bindTooltip('6 Trowbridge Circle',{permanent:true,direction:'top',offset:[0,-8]});}
const lg=L.control({position:'bottomright'});
lg.onAdd=function(){const d=L.DomUtil.create('div','legend');
 d.innerHTML='<b>Leq dB(A)</b><div class=bar></div>'+
  '<div class=row><span>40</span><span>quiet&nbsp;→&nbsp;loud</span><span>75</span></div>'+
  '<label style="display:block;margin-top:6px">overlay '+
  '<input type=range min=0 max=100 value=70 oninput="overlay.setOpacity(this.value/100)"></label>';
 L.DomEvent.disableClickPropagation(d);return d;};
lg.addTo(map);
</script></body></html>"""
open(os.path.join(WEB,"index.html"),"w").write(HTML.replace("__DATA__", DATA))
print("[site] wrote", os.path.join(WEB,"index.html"))
print("[site] receiver levels:", {m['name'][:18]:m['leq'] for m in markers})
