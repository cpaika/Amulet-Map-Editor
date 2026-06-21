"""Modeled lake bathymetry: distance-to-shore basin calibrated to a known max
depth. Felzenszwalb separable exact Euclidean distance transform (no scipy)."""
import numpy as np

def _edt1d_sq(f):
    n=len(f); d=np.empty(n); v=np.zeros(n,dtype=np.intp); z=np.empty(n+1)
    INF=1e20; k=0; v[0]=0; z[0]=-INF; z[1]=INF
    for q in range(1,n):
        s=((f[q]+q*q)-(f[v[k]]+v[k]*v[k]))/(2*q-2*v[k])
        while s<=z[k]:
            k-=1; s=((f[q]+q*q)-(f[v[k]]+v[k]*v[k]))/(2*q-2*v[k])
        k+=1; v[k]=q; z[k]=s; z[k+1]=INF
    k=0
    for q in range(n):
        while z[k+1]<q: k+=1
        d[q]=(q-v[k])*(q-v[k])+f[v[k]]
    return d

def edt(mask):
    """Euclidean distance (in cells) from each True cell to nearest False cell."""
    INF=1e20
    f=np.where(mask,INF,0.0)
    for j in range(f.shape[1]):
        f[:,j]=_edt1d_sq(f[:,j])
    for i in range(f.shape[0]):
        f[i,:]=_edt1d_sq(f[i,:])
    return np.sqrt(f)

def depth_map(mask, mpp, max_depth, power=0.8):
    dist=edt(mask)*mpp                  # meters from shore
    md=dist.max()
    if md<=0: return np.zeros_like(dist)
    dep=max_depth*np.power(dist/md,power)
    dep[~mask]=0
    return dep


def _shore_zero(cm):
    """Boundary cells of a coarse water mask (touch non-water) -> depth-0 seeds."""
    import numpy as np
    p = np.pad(cm, 1)
    nb = p[:-2,1:-1] & p[2:,1:-1] & p[1:-1,:-2] & p[1:-1,2:]
    return cm & ~nb


def laplace_depth(fmask, known_mask, known_val, iters=800):
    """Solve Laplace's equation for depth inside fmask with Dirichlet
    constraints (known_mask/known_val), Neumann at land edges."""
    import numpy as np
    d = np.zeros(fmask.shape, np.float32)
    if known_mask.any():
        d[fmask] = float(known_val[known_mask].mean())
    d[known_mask] = known_val[known_mask]
    fixed = known_mask & fmask
    fm = fmask.astype(np.float32)
    for _ in range(iters):
        s = np.zeros_like(d); c = np.zeros_like(d)
        s[1:,:]  += (d*fm)[:-1,:]; c[1:,:]  += fm[:-1,:]
        s[:-1,:] += (d*fm)[1:,:];  c[:-1,:] += fm[1:,:]
        s[:,1:]  += (d*fm)[:,:-1]; c[:,1:]  += fm[:,:-1]
        s[:,:-1] += (d*fm)[:,1:];  c[:,:-1] += fm[:,1:]
        avg = np.where(c > 0, s/np.maximum(c,1), d)
        d = np.where(fixed, known_val, avg)
        d[~fmask] = 0
    return d
