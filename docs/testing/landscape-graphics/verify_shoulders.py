#!/usr/bin/env python3
"""Independent planar footprint audit of actual source road/building polygons."""
import bisect,json,math
from pathlib import Path
from shapely.geometry import Polygon
from shapely.strtree import STRtree
out=Path(__file__).resolve().parent; root=out.parents[2]
m=json.loads((root/'content/world/metropolis.json').read_text());path=m['paths'][0]
def sample(s,lane):
 clamped=max(0.,s);i=min(len(path['points'])-2,max(0,bisect.bisect_right(path['progress'],clamped)-1))
 a,b=path['points'][i:i+2];t=(clamped-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);f=[b[j]-a[j] for j in range(3)];length=math.sqrt(sum(v*v for v in f));f=[v/length for v in f];n=math.hypot(f[0],f[2]);side=[f[2]/n,-f[0]/n]
 return (a[0]+(b[0]-a[0])*t+f[0]*min(s,0)-side[0]*lane,a[2]+(b[2]-a[2])*t+f[2]*min(s,0)-side[1]*lane)
def point(s,band,side):
 width=[.08,.65+.25*math.sin(s*.19),2.7+.65*math.sin(s*.13)+.3*math.sin(s*.7)][band]
 return sample(s,side*(6.8+width))
solids=[Polygon(s['shape']['rings'][0],s['shape']['rings'][1:]) for s in m['surfaces']+m['buildings']];tree=STRtree(solids)
area=0.;worst=0.;count=0;rejected=0;kept=0
for s in range(-334,2998,2):
 for side in [-1,1]:
  for band in [0,1]:
   poly=Polygon([point(s,band,side),point(s+2,band,side),point(s+2,band+1,side),point(s,band+1,side)])
   overlaps=[poly.intersection(solids[idx]).area for idx in tree.query(poly)]
   a=sum(overlaps);area+=a;worst=max(worst,max(overlaps,default=0.))
   if a>1e-8: rejected+=1
   else: kept+=1
   count+=1
result={'candidate_quads':count,'rejected_overlapping_quads':rejected,'retained_zero_overlap_quads':kept,'summed_candidate_overlap_area_m2':area,'maximum_candidate_quad_overlap_m2':worst,'retained_overlap_area_m2':0.0,'method':'Independent source-path planar reconstruction and Shapely polygon intersections against all actual paved/sidewalk/other surfaces and building envelopes; not collider authority.'}
native=out/'after/screenshots/metro-survey.json'
if native.exists():
 result['native_final_shoulder_quads']=json.loads(native.read_text())['shoulder_triangles']//2
 result['native_matches_independent_count']=result['native_final_shoulder_quads']==kept
 assert result['native_matches_independent_count']
(out/'shoulder-footprints.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
assert rejected>0 and kept>6000
