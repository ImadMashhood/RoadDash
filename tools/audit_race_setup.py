"""Independent current race width/grid/traffic clearance and baseline geometry audit."""
import bisect
import json
import math
from pathlib import Path
import subprocess
from shapely import Polygon, union_all
from metropolis_streetscape import footprint

ROOT=Path(__file__).resolve().parents[1]
m=json.loads((ROOT/'content/world/metropolis.json').read_text())
old=json.loads(subprocess.check_output(['git','show','432ab33:content/world/metropolis.json'],cwd=ROOT))
for k in ['paths','traffic_paths','fields','bounds','junctions','streets','buildings','source_layout_sha256']:
    assert m[k]==old[k],k
assert m['track']['checkpoints']==old['track']['checkpoints']
for route in range(2):
    sections=[s for s in m['street_life']['lane_sections'] if s['route']==route]
    assert [(s['start'],s['end'],s['outbound'],s['inbound'],s['lane_width']) for s in sections]==[(0,3350,2,2,3.),(3350,5362.5,2,1,3.),(5362.5,12000,2,2,3.)]
spawns=m['track']['spawns'];distances=sorted(p['distance'] for p in spawns)
assert len(spawns)==15 and spawns[0]['distance']==min(distances)
assert all(b-a>=20 for a,b in zip(distances,distances[1:]))
assert {s['lane'] for s in spawns}=={-4.5,-1.5}
road=union_all([Polygon(s['shape']['rings'][0],s['shape']['rings'][1:]) for s in m['surfaces'] if s['kind']==0]).buffer(.05)
import shapely
shapely.prepare(road)
def pose(path,s,lane):
    i=max(0,min(len(path['points'])-2,bisect.bisect_right(path['progress'],s)-1))
    a,b=path['points'][i:i+2];t=(s-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz)
    return [a[0]+(b[0]-a[0])*t-dz/n*lane,a[2]+(b[2]-a[2])*t+dx/n*lane],math.atan2(dx,dz)
samples=0
for route,path in enumerate(m['traffic_paths']):
    ss=[s for s in m['street_life']['lane_sections'] if s['route']==route]
    for s in range(20,11981,20):
        k=next(i for i,p in enumerate(ss) if p['start']<=s<p['end']);b=ss[k];a=ss[max(0,k-1)];t=min(1,max(0,(s-b['start'])/100));t=t*t*(3-2*t)
        for sign in [-1,1]:
            for slot in range(2):
                def center(v):
                    count=v['outbound'] if sign<0 else v['inbound']
                    return (v['outbound']-v['inbound'])*v['lane_width']/2+sign*(min(slot,count-1)+.5)*v['lane_width']
                lane=center(a)+(center(b)-center(a))*t;p,yaw=pose(path,s,lane)
                assert road.covers(footprint(p,yaw,1.9,3.9)),(route,s,lane)
                samples+=1
for spawn in spawns:
    p,yaw=pose(m['paths'][0],spawn['distance'],spawn['lane'])
    assert road.covers(footprint(p,yaw,.72,2.1)),spawn
for p in m['parked_cars']:
    assert road.covers(footprint(p['position'],p['heading'],1.8,3.8).buffer(.1)),p['id']
assert 0<len(m['parked_cars'])<=65
assert len({p['kind'] for p in m['street_props']})==13
report=dict(vehicle_samples=samples,vehicle_clearance_failures=0,grid_bikes=15,minimum_station_gap_m=20,player_start_m=spawns[0]['distance'],grid_lanes_m=[-4.5,-1.5],lane_width_m=3,route_lane_counts=[4,3,4],split_m=3350,rejoin_m=5362.5,parked_cars=len(m['parked_cars']),street_props=len(m['street_props']),prop_types=13,buildings_unchanged=len(m['buildings']),preserved=['race paths','elevations','checkpoints','building IDs, footprints, heights and styles','field polygons','secondary street network'],limits='Geometry audit, not a full race balance or performance certificate')
(ROOT/'docs/testing/race-setup/geometry.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
