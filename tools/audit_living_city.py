"""Independent whole-car, public-walk and civic authoring checks on baked geometry."""
import json,math,bisect,subprocess
from pathlib import Path
from shapely import Point,LineString,union_all
import shapely
from living_city import shape_polygon
from metropolis_streetscape import footprint
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'docs/testing/civic-greenery'
m=json.loads((ROOT/'content/world/metropolis.json').read_text())
old=json.loads(subprocess.check_output(['git','show','b4dd5e7:content/world/metropolis.json'],cwd=ROOT))
road=union_all([shape_polygon(s['shape']) for s in m['surfaces'] if s['kind']==0]);clearance=road.buffer(.03);shapely.prepare(clearance)
walk=union_all([shape_polygon(s['shape']) for s in m['surfaces'] if s['kind']==1]);shapely.prepare(walk)
failed=[];samples=0
for route,path in enumerate(m['traffic_paths']):
 sections=[s for s in m['street_life']['lane_sections'] if s['route']==route]
 for station in range(3250,11950,5):
  if route==1 and not 3350<=station<5362.5:continue
  k=next(i for i,s in enumerate(sections) if s['start']<=station<s['end']);b=sections[k];a=sections[max(0,k-1)];t=max(0,min(1,(station-b['start'])/100));t=t*t*(3-2*t)
  i=min(len(path['points'])-2,bisect.bisect_right(path['progress'],station)-1);p,q=path['points'][i:i+2];u=(station-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=q[0]-p[0],q[2]-p[2];dist=math.hypot(dx,dz);dx,dz=dx/dist,dz/dist;x,z=p[0]+(q[0]-p[0])*u,p[2]+(q[2]-p[2])*u
  for sign in [-1,1]:
   for slot in range(2):
    def center(v):
     count=v['outbound'] if sign<0 else v['inbound'];return (v['outbound']-v['inbound'])*v['lane_width']/2+sign*(min(slot,count-1)+.5)*v['lane_width']
    lane=center(a)+(center(b)-center(a))*t
    shape=footprint([x-dz*lane,z+dx*lane],math.atan2(dx,dz),1.9,3.9);samples+=1
    if not clearance.covers(shape):failed.append(dict(route=route,station=station,lane=lane))
for key in ['paths','fields','bounds','junctions','streets','source_layout_sha256']:assert old[key]==m[key],key
for key in ['spawns','checkpoints']:assert old['track'][key]==m['track'][key],key
ids={b['id']:b for b in m['buildings']}
for b in old['buildings']:
 assert b['id'] in ids
 assert shape_polygon(ids[b['id']]['shape']).area>=shape_polygon(b['shape']).area*.65
for p in m['pedestrians']:
 x,z=p['position'];dx,dz=math.sin(p['heading']),math.cos(p['heading']);e=p['extent'];sweep=LineString([(x-dx*e,z-dz*e),(x+dx*e,z+dz*e)]).buffer(.4)
 assert walk.buffer(.0001).covers(sweep),p['id']
 for s in m['civic_sites']:
  if s['kind']=='cafe':assert not sweep.intersects(footprint(s['position'],s['heading'],*s['size'])),(p['id'],s['id'])
source=json.loads((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_text())
config=json.loads((ROOT/'content/world/living_city.json').read_text())
for site in m['civic_sites'][:2]:
 size=(62*2.8,46*2.8) if site['kind']=='city_hall' else site['size']
 civic=footprint(site['position'],site['heading'],*size)
 block=next(b for b in source['blocks'] if b['id']==config['city_hall_site' if site['kind']=='city_hall' else 'park_site'])
 assert block['role']=='park'
 boundary=shapely.Polygon(block['polygon'],block['holes'])
 grounds=union_all([shape_polygon(patch) for patch in site['grounds']])
 expected=boundary.difference(road).difference(walk).difference(union_all([shape_polygon(b['shape']) for b in m['buildings'] if b['id'].startswith('civic-context-')]))
 assert grounds.symmetric_difference(expected).area < .05, (site['id'],'full block ground coverage')
 assert boundary.buffer(.0001).covers(grounds)
 paving=union_all([shape_polygon(patch) for patch in site['paving']])
 assert paving.intersection(road).area < .05
 for entrance in site['entrances']:
  assert walk.distance(Point(entrance))<.001
  assert paving.buffer(.001).covers(Point(entrance))
  assert paving.intersection(walk).intersection(Point(entrance).buffer(2)).area>.5
 if site['kind']=='park':
  assert len(site['entrances'])==6
  assert paving.geom_type=='Polygon', 'walkways must form a connected network around fountain'

 if site['kind']=='city_hall':
  assert boundary.covers(civic)
  race=LineString([(v[0],v[2]) for v in m['traffic_paths'][0]['points']]);p=Point(site['position']);q=race.interpolate(race.project(p))
  normal=(math.sin(site['heading']),math.cos(site['heading']));dx,dz=q.x-p.x,q.y-p.y
  assert (normal[0]*dx+normal[1]*dz)/math.hypot(dx,dz)>.999, 'arches must face actual driving street'

 for building in m['buildings']:
  assert civic.intersection(shape_polygon(building['shape'])).area < .0001, (site['id'],building['id'])
report=dict(civic_building_overlaps=0,samples=samples,body_outside_carriageway=failed,body_width=1.9,body_length=3.9,tolerance_m=.03,old_building_ids_retained=len(old['buildings']),new_buildings=len(ids)-len(old['buildings']),pedestrians=len(m['pedestrians']),civic_sites=m['civic_sites'],source_routes_gates_fields_unchanged=True)
(OUT/'geometry-audit.json').write_text(json.dumps(report,indent=2)+'\n')
print('car samples',samples,'failures',len(failed),failed[:8]);assert not failed
