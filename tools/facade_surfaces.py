"""UV-preserving relief meshes. Texture bytes come directly from the new facade sheets."""
from collections import defaultdict

# Normalized rectangles (left, top, right, bottom, outward depth in metres).
# These profiles move the actual painted windows/signs, not substitute geometry.
RELIEF = {
    'BUSS1':[(.12,.18,.36,.35,-.10),(.61,.18,.85,.35,-.10),(.02,.46,.98,.51,.13),(.1,.57,.9,.97,-.06)],
    'BUSS10':[(.12,.2,.42,.47,-.10),(.60,.2,.90,.47,-.10),(.04,.08,.96,.15,.12),(.12,.66,.92,.98,-.07)],
    'BUSS2':[(0,.37,1,.48,.55),(.15,.56,.88,.98,-.09)],
    'BUSS4':[(.06,.15,.94,.3,.07),(.02,.40,.98,.53,.15),(.10,.60,.90,.98,-.08)],
    'BUSS5':[(.03,.22,.97,.45,.08),(.47,.51,.94,.98,-.09),(.08,.58,.41,.90,-.07)],
    'BUSS6':[(.12,.12,.47,.45,.18),(.53,.12,.88,.45,.18),(.46,.63,.94,.98,-.09)]}


class Surfaces:
    def __init__(self):
        self.batches=defaultdict(list)

    def panel(self, asset, centre, normal, width, height, owner, role='front'):
        regions=[(0,0,1,.045,.10),(0,.975,1,1,.03)]
        if role=='front': regions+=RELIEF.get(asset['id'],[])
        us=sorted({0,1,*[r[k] for r in regions for k in [0,2]]})
        vs=sorted({0,1,*[r[k] for r in regions for k in [1,3]]})
        depths={}
        for j in range(len(vs)-1):
            for i in range(len(us)-1):
                u,v=(us[i]+us[i+1])/2,(vs[j]+vs[j+1])/2
                d=.015
                for x0,y0,x1,y1,value in regions:
                    if x0<=u<=x1 and y0<=v<=y1:d=value+.015
                depths[i,j]=d
        # Right cross up points toward the exterior normal; source text stays readable.
        nx,_,nz=normal;right=(nz,0,-nx)
        def point(u,v,d):
            return [centre[k]+right[k]*(u-.5)*width+(height*(1-v) if k==1 else 0)+normal[k]*d for k in range(3)]
        def uv(u,v):
            x0,y0,x1,y1=asset['rect_px'];w,h=asset['pixels']
            # Half-pixel inset prevents label/background bleed under filtering.
            return [(x0+.5+u*(x1-x0-1))/w,(y0+.5+v*(y1-y0-1))/h]
        def quad(values):
            self.batches[asset['sheet']].append(dict(owner=owner,asset=asset['id'],role=role,
                positions=[point(*x) for x in values],uvs=[uv(x[0],x[1]) for x in values]))
        for (i,j),d in depths.items():
            u0,u1=us[i:i+2];v0,v1=vs[j:j+2]
            quad([(u0,v1,d),(u1,v1,d),(u1,v0,d),(u0,v0,d)])
            if i+1<len(us)-1 and depths[i+1,j]!=d:
                other=depths[i+1,j]
                quad([(u1,v1,d),(u1,v0,d),(u1,v0,other),(u1,v1,other)])
            if j+1<len(vs)-1 and depths[i,j+1]!=d:
                other=depths[i,j+1]
                quad([(u0,v1,d),(u1,v1,d),(u1,v1,other),(u0,v1,other)])

    def records(self):
        return [dict(sheet=sheet,quads=quads) for sheet,quads in sorted(self.batches.items())]
