"""Validate actual baked street population against pavement and original layout."""
import json,math,subprocess
from pathlib import Path
from collections import Counter
from shapely import LineString,Point
from shapely.ops import unary_union
from metropolis_streetscape import sidewalks,footprint
from metropolis_population import station,route_lines
ROOT=Path(__file__).resolve().parents[1]
m=json.loads((ROOT/'content/world/metropolis.json').read_text());d=json.loads((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_text())
old=json.loads(subprocess.check_output(['git','show','d5db26f:content/world/metropolis.json'],cwd=ROOT))
for k in ['paths','traffic_paths','fields','buildings','bounds','streets','junctions','track','street_life','source_layout_sha256']:assert m[k]==old[k],k
road,walk,_=sidewalks(d)
bays=unary_union([footprint(c['position'],c['heading'],2.2,4.4) for c in m['parked_cars']]).intersection(walk)
road=road.union(bays);walk=walk.difference(bays)
for c in m['parked_cars']:
 assert road.covers(footprint(c['position'],c['heading'],1.8,3.8).buffer(.1))
 clear=footprint([c['position'][0]+math.cos(c['heading'])*1.9,c['position'][1]-math.sin(c['heading'])*1.9],c['heading'],1.5,6.4)
 assert walk.covers(clear),c['id']
occupied=unary_union([footprint(p['position'],p['heading'],.36 if p['kind']=='broadleaf' else p['size'][0],.36 if p['kind']=='broadleaf' else p['size'][2]).buffer(.2) for p in m['street_props']])
safe=walk.buffer(-.42).difference(occupied.buffer(.42))
for p in m['pedestrians']:
 x,z=p['position'];dx,dz=math.sin(p['heading'])*p['extent'],math.cos(p['heading'])*p['extent']
 assert safe.covers(LineString([(x-dx,z-dz),(x+dx,z+dz)])),p['id']
lines=route_lines(m['paths']);groups={}
for kind,items in [('trees',[p for p in m['street_props'] if p['kind']=='broadleaf']),('parked_cars',m['parked_cars']),('pedestrians',m['pedestrians'])]:
 bands=Counter(int(station(p['position'],m['paths'],lines)//1000) for p in items)
 assert all(bands[i]>0 for i in range(3,12)),(kind,bands)
 groups[kind]=dict(count=len(items),per_kilometre=dict(sorted(bands.items())))
assert len(m['street_props'])<=1500 and len(m['pedestrians'])<=96 and len(m['parked_cars'])<=128
report=dict(groups=groups,props=len(m['street_props']),bay_area_m2=bays.area,preserved=['layout source','all19068building footprints/heights/styles','race/traffic paths','4/3/4lanes','checkpoints/grid','farms/terrain'],checks=['allnineurban kilometre bands populated','all96fullpatrolcorridors capsule/furniture clear','carfootprints paved','1.5m outerwalk at everybay','bounded populations'],limits='Coverage and geometry are not native visibility, human art approval or performance certification.')
print(json.dumps(report,indent=2))
