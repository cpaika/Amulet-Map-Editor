"""Interactive 3D noise map of 6 Trowbridge Circle and its immediate area.

Drapes the modelled A-weighted daytime Leq (at 1.5 m, ear height) over 1 m
MassGIS LiDAR terrain, extrudes nearby buildings, and writes a standalone
HTML (Plotly) you can rotate/zoom to see where traffic noise concentrates and
where a barrier / berm / planting would help most.

Terrain : MassGIS LiDAR 2013-2021 (1 m)  -> house_dem.npz
Sources : MassDOT Road Inventory AADT via sound_model (CoRTN + ISO 9613-2),
          but barrier diffraction here is recomputed against the 1 m LiDAR
          (+ OSM building footprints) for near-field accuracy.
"""
import json, math, os, sys
import numpy as np
import sound_model as sm     # reuses calibrated sub-sources SX,SY,SLW,SZ and constants

HERE = os.path.dirname(os.path.abspath(__file__))
HOUSE = (42.2940843, -71.7007366)
RENDER_R = 300.0     # render/compute half-width, m
GRID_RES = 5.0       # noise grid resolution, m
LOC_RES  = 2.0       # local terrain/building raster resolution, m
NEAR_TILE = 820.0    # sources within this of house use LiDAR barriers

# ---- local 1 m LiDAR terrain, in sound_model's metric frame (origin = house) ----
_D = np.load(os.path.join(HERE, "house_dem.npz"))
DEM = _D["dem"].astype(np.float32)
LXMIN,LXMAX,LYMIN,LYMAX = (float(_D["xmin"]),float(_D["xmax"]),float(_D["ymin"]),float(_D["ymax"]))
DH, DW = DEM.shape

def lidar_at(lat, lon):
    fx=(lon-LXMIN)/(LXMAX-LXMIN)*DW-0.5; fy=(LYMAX-lat)/(LYMAX-LYMIN)*DH-0.5
    i=np.clip(fy.astype(int),0,DH-2); j=np.clip(fx.astype(int),0,DW-2)
    di=fy-i; dj=fx-j
    return (DEM[i,j]*(1-di)*(1-dj)+DEM[i+1,j]*di*(1-dj)+DEM[i,j+1]*(1-di)*dj+DEM[i+1,j+1]*di*dj)

# build a local metric raster (terrain + buildings) covering the tile
gx = np.arange(-NEAR_TILE, NEAR_TILE+LOC_RES, LOC_RES)
gy = np.arange(-NEAR_TILE, NEAR_TILE+LOC_RES, LOC_RES)
NXl, NYl = len(gx), len(gy)
# metric -> lat/lon (origin house); sm.ME, sm.MN are m/deg
LAT0, LON0 = sm.LAT0, sm.LON0
GX, GY = np.meshgrid(gx, gy)
lat_g = LAT0 + GY/sm.MN; lon_g = LON0 + GX/sm.ME
TERRl = lidar_at(lat_g, lon_g).astype(np.float32)         # (NYl,NXl) bare-earth
BLDGl = np.zeros_like(TERRl)

def _rasterize_buildings():
    d = json.load(open(os.path.join(HERE, "house_buildings.json")))
    cnt=0
    for el in d["elements"]:
        g=el.get("geometry");
        if not g or len(g)<3: continue
        pts=np.array([sm.ll_to_m(p["lat"],p["lon"]) for p in g])
        if pts[:,0].max()< -NEAR_TILE or pts[:,0].min()>NEAR_TILE or \
           pts[:,1].max()< -NEAR_TILE or pts[:,1].min()>NEAR_TILE: continue
        h=_bh(el.get("tags",{}))
        ix=(pts[:,0]-gx[0])/LOC_RES; iy=(pts[:,1]-gy[0])/LOC_RES
        x0,x1=max(int(ix.min()),0),min(int(ix.max())+1,NXl-1)
        y0,y1=max(int(iy.min()),0),min(int(iy.max())+1,NYl-1)
        if x1<x0 or y1<y0: continue
        mx,my=np.meshgrid(np.arange(x0,x1+1),np.arange(y0,y1+1))
        inside=_pip(mx+0.5,my+0.5,ix,iy)
        sub=BLDGl[y0:y1+1,x0:x1+1]; np.maximum(sub,np.where(inside,h,0),out=sub)
        cnt+=1
    return cnt

def _bh(t):
    h=t.get("height")
    if h:
        try: return max(2.5,float(str(h).split()[0].replace("m","")))
        except: pass
    lv=t.get("building:levels")
    if lv:
        try: return max(2.5,float(lv)*3.1+1)
        except: pass
    bt=(t.get("building") or "").lower()
    return {"commercial":8,"retail":8,"industrial":8,"church":10,"school":10,"apartments":12}.get(bt,6.5)

def _pip(px,py,vx,vy):
    inside=np.zeros(px.shape,bool); n=len(vx); j=n-1
    for i in range(n):
        cond=((vy[i]>py)!=(vy[j]>py))
        xint=vx[i]+(py-vy[i])*(vx[j]-vx[i])/(vy[j]-vy[i]+1e-12)
        inside^=cond&(px<xint); j=i
    return inside

nb=_rasterize_buildings()
SURFl = TERRl + BLDGl
print(f"[3d] local raster {NXl}x{NYl} @ {LOC_RES} m, {nb} buildings", file=sys.stderr)

def surf_sample(x, y):
    fx=np.clip((x-gx[0])/LOC_RES,0,NXl-1.001); fy=np.clip((y-gy[0])/LOC_RES,0,NYl-1.001)
    i=fy.astype(int); j=fx.astype(int); di=fy-i; dj=fx-j
    return (SURFl[i,j]*(1-di)*(1-dj)+SURFl[i+1,j]*di*(1-dj)+SURFl[i,j+1]*(1-di)*dj+SURFl[i+1,j+1]*di*dj)
def terr_sample(x,y):
    fx=np.clip((x-gx[0])/LOC_RES,0,NXl-1.001); fy=np.clip((y-gy[0])/LOC_RES,0,NYl-1.001)
    i=fy.astype(int); j=fx.astype(int); di=fy-i; dj=fx-j
    return (TERRl[i,j]*(1-di)*(1-dj)+TERRl[i+1,j]*di*(1-dj)+TERRl[i,j+1]*(1-di)*dj+TERRl[i+1,j+1]*di*dj)

# ---- sub-sources, with z from local LiDAR where in-tile ----
SX,SY,SLW,SZ=sm.SX,sm.SY,sm.SLW,sm.SZ.copy()
intile=(np.abs(SX)<NEAR_TILE)&(np.abs(SY)<NEAR_TILE)
SZ[intile]=terr_sample(SX[intile],SY[intile])+sm.H_SRC

def leq_cell(rx,ry):
    d=np.hypot(SX-rx,SY-ry)
    sel=np.where(d<2000.0)[0]
    if len(sel)==0: return sm.AMBIENT_LEQ
    dd=d[sel]; rz=float(terr_sample(np.array([rx]),np.array([ry]))[0])+sm.H_RCV
    Adiv=20*np.log10(np.maximum(dd,sm.D_MIN))+8.0
    Aatm=sm.AIR_DB_PER_M*dd
    hm=np.maximum((rz+SZ[sel])/2-terr_sample(SX[sel],SY[sel]),0.5)
    Agr=sm.ground_atten(dd,hm)
    excess=Agr.copy()
    near=(np.abs(SX[sel])<NEAR_TILE)&(np.abs(SY[sel])<NEAR_TILE)&(np.abs(rx)<NEAR_TILE)&(np.abs(ry)<NEAR_TILE)
    ni=np.where(near)[0]
    if len(ni):
        K=18; t=np.linspace(0,1,K)[None,:]
        sx=SX[sel][ni]; sy=SY[sel][ni]; sz=SZ[sel][ni]
        px=rx+(sx-rx)[:,None]*t; py=ry+(sy-ry)[:,None]*t
        surf=surf_sample(px.ravel(),py.ravel()).reshape(len(ni),K)
        losz=rz+(sz-rz)[:,None]*t
        dr=dd[ni][:,None]*t
        intr=np.where((dr<sm.BAR_EDGE)|(dr>(dd[ni][:,None]-sm.BAR_EDGE)),-1e9,surf-losz)
        over=intr.max(axis=1); m=over>0.05
        if m.any():
            km=intr[m].argmax(axis=1); tk=t[0,km]
            ox=rx+(sx[m]-rx)*tk; oy=ry+(sy[m]-ry)*tk; oz=surf[m,km]
            dso=np.sqrt((ox-sx[m])**2+(oy-sy[m])**2+(oz-sz[m])**2)
            dor=np.sqrt((ox-rx)**2+(oy-ry)**2+(oz-rz)**2)
            dsr=np.sqrt((sx[m]-rx)**2+(sy[m]-ry)**2+(sz[m]-rz)**2)
            N=2*np.maximum(dso+dor-dsr,0)/sm.WAVELEN
            ab=np.clip(10*np.log10(3+20*N),0,sm.BAR_CAP)
            idx_near=ni[m]; excess[idx_near]=np.maximum(excess[idx_near],ab)
    lvl=SLW[sel]-Adiv-Aatm-excess
    return 10*math.log10(np.sum(10**(lvl/10))+10**(sm.AMBIENT_LEQ/10))

def compute_grid():
    xs=np.arange(-RENDER_R,RENDER_R+GRID_RES,GRID_RES)
    ys=np.arange(-RENDER_R,RENDER_R+GRID_RES,GRID_RES)
    Z=np.zeros((len(ys),len(xs))); L=np.zeros_like(Z)
    print(f"[3d] computing {len(xs)*len(ys)} noise cells ...",file=sys.stderr)
    for jy,yy in enumerate(ys):
        for ix,xx in enumerate(xs):
            L[jy,ix]=leq_cell(xx,yy)
        Z[jy,:]=terr_sample(xs,np.full_like(xs,yy))
        if jy%15==0: print(f"   row {jy}/{len(ys)}",file=sys.stderr)
    np.savez_compressed(os.path.join(HERE,"house_noisegrid.npz"),xs=xs,ys=ys,Z=Z,L=L)
    print("[3d] saved house_noisegrid.npz",file=sys.stderr)
    return xs,ys,Z,L

def render(xs,ys,Z,L):
    import plotly.graph_objects as go
    fig=go.Figure()
    fig.add_surface(x=xs,y=ys,z=Z,surfacecolor=L,colorscale="Turbo",cmin=42,cmax=62,
                    colorbar=dict(title="Leq dB(A)"),
                    contours={"z":{"show":False}},
                    hovertemplate="x %{x:.0f} m, y %{y:.0f} m<br>%{surfacecolor:.1f} dB(A)<extra></extra>")
    # buildings as extruded prisms
    d=json.load(open(os.path.join(HERE,"house_buildings.json")))
    vx=[];vy=[];vz=[];ii=[];jj=[];kk=[];off=0
    for el in d["elements"]:
        g=el.get("geometry")
        if not g or len(g)<3: continue
        pm=np.array([sm.ll_to_m(p["lat"],p["lon"]) for p in g])
        if pm[:,0].min()<-RENDER_R or pm[:,0].max()>RENDER_R or pm[:,1].min()<-RENDER_R or pm[:,1].max()>RENDER_R: continue
        h=_bh(el.get("tags",{})); base=float(terr_sample(np.array([pm[:,0].mean()]),np.array([pm[:,1].mean()]))[0])
        n=len(pm)-1
        for p in pm[:-1]: vx.append(p[0]);vy.append(p[1]);vz.append(base)
        for p in pm[:-1]: vx.append(p[0]);vy.append(p[1]);vz.append(base+h)
        for a in range(n):
            b=(a+1)%n
            ii+=[off+a,off+a]; jj+=[off+b,off+n+b]; kk+=[off+n+b,off+n+a]
        cx,cy=pm[:-1,0].mean(),pm[:-1,1].mean(); ci=off+2*n
        vx.append(cx);vy.append(cy);vz.append(base+h)
        for a in range(n):
            b=(a+1)%n; ii+=[ci]; jj+=[off+n+a]; kk+=[off+n+b]
        off+=2*n+1
    if vx:
        fig.add_mesh3d(x=vx,y=vy,z=vz,i=ii,j=jj,k=kk,color="lightgray",opacity=1.0,
                       flatshading=True,name="buildings",hoverinfo="skip")
    hz=float(terr_sample(np.array([0.0]),np.array([0.0]))[0])
    fig.add_scatter3d(x=[0],y=[0],z=[hz+12],mode="markers+text",
                      marker=dict(size=6,color="black",symbol="diamond"),
                      text=["6 Trowbridge Circle"],textposition="top center",name="house")
    fig.update_layout(title="6 Trowbridge Circle — modelled daytime traffic noise on 1 m LiDAR terrain<br>"
                      "<sub>color = A-weighted Leq at ear height; gray = buildings. Lower (blue) = quieter.</sub>",
                      scene=dict(xaxis_title="m East",yaxis_title="m North",zaxis_title="elev (m)",
                                 aspectmode="manual",aspectratio=dict(x=1,y=1,z=0.35),
                                 camera=dict(eye=dict(x=1.4,y=-1.4,z=1.0))),
                      width=1100,height=850,margin=dict(l=0,r=0,t=70,b=0))
    out=os.path.join(HERE,"trowbridge_noise_3d.html")
    fig.write_html(out,include_plotlyjs="inline")
    print("[3d] wrote",out,file=sys.stderr)

if __name__=="__main__":
    if "--render-only" in sys.argv:
        z=np.load(os.path.join(HERE,"house_noisegrid.npz")); render(z["xs"],z["ys"],z["Z"],z["L"])
    else:
        xs,ys,Z,L=compute_grid(); render(xs,ys,Z,L)
