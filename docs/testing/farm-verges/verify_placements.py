"""Independent Shapely clearance check on native-emitted placements."""
import json
from pathlib import Path
from shapely.geometry import Polygon,Point
from shapely.strtree import STRtree
out=Path(__file__).resolve().parent;root=out.parents[2]
m=json.loads((root/'content/world/metropolis.json').read_text())
def polygonal(shape):
 # Authoring flattens all exterior and hole rings of MultiPolygons.
 # Symmetric difference reconstructs their even/odd fill without assuming first-only shell.
 result=Polygon()
 for ring in shape['rings']:
  result=result.symmetric_difference(Polygon(ring))
 return result
fields=[polygonal(f['shape']) for f in m['fields']]
solids=[polygonal(f['shape']) for f in m['surfaces']+m['buildings']]
tree=STRtree(solids)
j=json.loads((out/'after/screenshots/farm-placements.json').read_text()); ps=j['placements']
clearances=[]
for i,p in enumerate(ps):
 x,y,z=p['position']; center=Point(x,z); r=p['clearance_radius']
 containing=[f for f in fields if f.contains(center)]
 assert containing,p
 margin=max(f.boundary.distance(center) for f in containing)-r
 assert margin >= .999,p
 for index in tree.query(center.buffer(r+1.)):
  assert solids[index].is_valid, (p,index)
  assert solids[index].distance(center)>r+.999,p
 for other in ps[:i]:
  assert center.distance(Point(other['position'][0],other['position'][2])) > r+other['clearance_radius']+1.999
 clearances.append(margin)
report={'cows':sum(p['name']=='cow' for p in ps),'props':sum(p['name']!='cow' for p in ps),'ready_prop_scenes':j['ready_prop_scenes'],'minimum_body_to_field_boundary_m':min(clearances),'actual_paved_surfaces_clear':True,'pairwise_occupant_clear':True,'method':'Independent Shapely on native positions and source polygon rings, including holes; not collider or GPU performance certification.'}
(out/'placement-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(report)
