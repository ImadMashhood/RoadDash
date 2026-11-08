"""Independent real-pavement vehicle fit and unchanged-layout audit; authoring venv."""
import sys,json,math,bisect,subprocess
from pathlib import Path
from collections import Counter
from metropolis_streetscape import sidewalks,footprint
import shapely
ROOT=Path(__file__).resolve().parents[1]
m=json.loads((ROOT/'content/world/metropolis.json').read_text());d=json.loads((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_text());road,_,_=sidewalks(d)
clearance=road.buffer(.05);shapely.prepare(clearance)
fail=[];n=0
for route,path in enumerate(m['traffic_paths']):
 ss=[s for s in m['street_life']['lane_sections'] if s['route']==route]
 for s in range(3250,11950,20):
  k=next(i for i,x in enumerate(ss) if x['start']<=s<x['end']);b=ss[k];a=ss[max(0,k-1)];t=min(1,max(0,(s-b['start'])/100));t=t*t*(3-2*t)
  i=min(len(path['points'])-2,bisect.bisect_right(path['progress'],s)-1);p,q=path['points'][i:i+2];u=(s-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=q[0]-p[0],q[2]-p[2];dist=math.hypot(dx,dz);dx,dz=dx/dist,dz/dist;x,z=p[0]+(q[0]-p[0])*u,p[2]+(q[2]-p[2])*u
  for sign in [-1,1]:
   for slot in range(2):
    def center(v):
     count=v['outbound'] if sign<0 else v['inbound'];return (v['outbound']-v['inbound'])*v['lane_width']/2+sign*(min(slot,count-1)+.5)*v['lane_width']
    lane=center(a)+(center(b)-center(a))*t;pt=[x-dz*lane,z+dx*lane];shape=footprint(pt,math.atan2(dx,dz),1.9,3.9);n+=1
    if not clearance.covers(shape):fail.append(dict(route=route,station=s,lane=lane))
old=json.loads(subprocess.check_output(['git','show','8f83245:content/world/metropolis.json'],cwd=ROOT))
for key in ['paths','fields','bounds','junctions','streets','source_layout_sha256']:assert old[key]==m[key],key
# User-authorized farm shoulder widening may add pavement, never remove the old road.
from shapely import Polygon
old_road=shapely.union_all([Polygon(p['shape']['rings'][0],p['shape']['rings'][1:]) for p in old['surfaces'] if p['kind']==0])
new_road=shapely.union_all([Polygon(p['shape']['rings'][0],p['shape']['rings'][1:]) for p in m['surfaces'] if p['kind']==0])
assert old_road.difference(new_road).area<.01
assert [(p['kind'],p['shape']) for p in old['surfaces'] if p['kind']!=0]==[(p['kind'],p['shape']) for p in m['surfaces'] if p['kind']!=0]
assert [(b['id'],b['shape'],b['style']) for b in old['buildings']]==[(b['id'],b['shape'],b['style']) for b in m['buildings']]
for key in ['spawns','checkpoints']:assert old['track'][key]==m['track'][key]
report=dict(samples=n,body_outside_carriageway=fail,body_width_m=1.9,body_length_m=3.9,tolerance_m=.05,
            preserved=['race paths','fields','bounds','junctions','streets','city sidewalks; old asphalt contained by widened farm pavement','building IDs/footprints/styles','spawns','gates'],
            floor_histogram=dict(sorted(Counter(b['floors'] for b in m['buildings']).items())),street_furniture=len(m['street_props']),parked=len(m['parked_cars']))
out=ROOT/'docs/testing/startup-world-diagnostics/lane-clearance.json';out.parent.mkdir(exist_ok=True);out.write_text(json.dumps(report,indent=2)+'\n')
print('samples',n,'failures',len(fail),fail[:8]);assert not fail
