"""What-if: can a noise fence / berm make 6 Trowbridge Circle quieter?
Reuses the LiDAR + sources from trowbridge3d; adds a barrier to the surface
and recomputes Leq with fine path sampling (captures a thin fence)."""
import numpy as np, math, sys
import trowbridge3d as t
sm = t.sm

SX, SY, SLW, SZ = t.SX, t.SY, t.SLW, t.SZ
TERRl, gx, gy, LOC = t.TERRl, t.gx, t.gy, t.LOC_RES
NXl, NYl = t.NXl, t.NYl

def surf_sample(SURF, x, y):
    fx=np.clip((x-gx[0])/LOC,0,NXl-1.001); fy=np.clip((y-gy[0])/LOC,0,NYl-1.001)
    i=fy.astype(int); j=fx.astype(int); di=fy-i; dj=fx-j
    return (SURF[i,j]*(1-di)*(1-dj)+SURF[i+1,j]*di*(1-dj)+SURF[i,j+1]*(1-di)*dj+SURF[i+1,j+1]*di*dj)

def leq(rx, ry, SURF, cull=1600.0):
    d=np.hypot(SX-rx,SY-ry); sel=np.where(d<cull)[0]
    dd=d[sel]; rz=float(surf_sample(TERRl,np.array([rx]),np.array([ry]))[0])+sm.H_RCV
    sx,sy,sz=SX[sel],SY[sel],SZ[sel]
    step=2.0
    Kmax=int(cull/step)
    # adaptive: sample each path at ~2 m
    Abar=np.zeros(len(sel));
    K=60
    tt=np.linspace(0,1,K)[None,:]
    px=rx+(sx-rx)[:,None]*tt; py=ry+(sy-ry)[:,None]*tt
    surf=surf_sample(SURF,px.ravel(),py.ravel()).reshape(len(sel),K)
    losz=rz+(sz-rz)[:,None]*tt
    dr=dd[:,None]*tt
    intr=np.where((dr<6)|(dr>(dd[:,None]-6)),-1e9,surf-losz)
    over=intr.max(axis=1); m=over>0.05
    if m.any():
        km=intr[m].argmax(axis=1); tk=tt[0,km]
        ox=rx+(sx[m]-rx)*tk; oy=ry+(sy[m]-ry)*tk; oz=surf[m,km]
        dso=np.sqrt((ox-sx[m])**2+(oy-sy[m])**2+(oz-sz[m])**2)
        dor=np.sqrt((ox-rx)**2+(oy-ry)**2+(oz-rz)**2)
        dsr=np.sqrt((sx[m]-rx)**2+(sy[m]-ry)**2+(sz[m]-rz)**2)
        N=2*np.maximum(dso+dor-dsr,0)/sm.WAVELEN
        Abar[m]=np.clip(10*np.log10(3+20*N),0,sm.BAR_CAP)
    Adiv=20*np.log10(np.maximum(dd,sm.D_MIN))+8
    Aatm=sm.AIR_DB_PER_M*dd
    hm=np.maximum((rz+sz)/2-surf_sample(TERRl,sx,sy),0.5)
    Agr=sm.ground_atten(dd,hm)
    excess=np.where(Abar>0,Abar,Agr)
    lvl=SLW[sel]-Adiv-Aatm-excess
    return 10*math.log10(np.sum(10**(lvl/10))+10**(sm.AMBIENT_LEQ/10))

def add_fence(y0, height, x0=-45, x1=45, thick=4.0):
    """Return a SURF copy with a solid barrier wall just north of the house."""
    S=t.SURFl.copy()
    jx0=int((x0-gx[0])/LOC); jx1=int((x1-gx[0])/LOC)
    jy0=int((y0-gy[0])/LOC); jy1=int((y0+thick-gy[0])/LOC)
    S[jy0:jy1+1, jx0:jx1+1]=np.maximum(S[jy0:jy1+1, jx0:jx1+1],
                                       TERRl[jy0:jy1+1, jx0:jx1+1]+height)
    return S

# receivers: house pad, exposed north yard, sheltered back (south) yard
RX={'house (0,0)':(0,0),'front yard N (0,+10)':(0,10),'back yard S (0,-15)':(0,-15)}
base={k:leq(x,y,t.SURFl) for k,(x,y) in RX.items()}
print("baseline Leq dB(A):")
for k,v in base.items(): print(f"   {k:<22} {v:5.1f}")
print()
print("fence/berm 15 m north of house (spanning 90 m), reduction vs baseline:")
print(f"   {'scenario':<26}"+''.join(f"{k.split(' (')[0]:>16}" for k in RX))
for label,h in [('6 ft fence (1.8 m)',1.8),('8 ft fence (2.4 m)',2.4),
                ('10 ft fence (3.0 m)',3.0),('berm+fence (4.0 m)',4.0)]:
    S=add_fence(13.0,h)
    red={k:base[k]-leq(x,y,S) for k,(x,y) in RX.items()}
    print(f"   {label:<26}"+''.join(f"{red[k]:>13.1f} dB" for k in RX))
