"""Read-only, whole-block city reduction proposal from the approved source."""
import json, hashlib
from pathlib import Path
from shapely import Polygon, LineString, Point
from shapely.ops import unary_union
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon as Patch
from matplotlib.collections import PatchCollection
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'docs/testing/civic-greenery';OUT.mkdir(exist_ok=True)
source=ROOT/'docs/planning/farms-long-city/LAYOUT.json'
d=json.loads(source.read_text());m=json.loads((ROOT/'content/world/metropolis.json').read_text())
routes=[LineString([(p[0],p[2]) for p in e['points']]) for e in d['edges']]
race=unary_union(routes)
blocks={b['id']:Polygon(b['polygon'],b['holes']) for b in d['blocks']}
# Conservative 45m corridor protects carriageway, walks and immediate frontage.
front={i for i,p in blocks.items() if p.distance(race)<45}
protected=front|{b['id'] for b in d['blocks'] if b['role']=='park'}
nonfront=[i for i in blocks if i not in front]
eligible=[i for i in nonfront if i not in protected]
# Remove the farthest whole blocks first; retain the inner urban fabric.
removed=set(sorted(eligible,key=lambda i:blocks[i].centroid.distance(race),reverse=True)[:len(nonfront)//2])
kept=set(blocks)-removed
assert not removed.intersection(protected)
assert len(removed)==len(nonfront)//2
assert len(kept)+len(removed)==len(blocks)
lost=[b['id'] for b in m['buildings'] if any(b['id'].startswith(i+'-') for i in removed)]
report=dict(proposal_only=True,source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),total_blocks=len(blocks),race_frontage_blocks=len(front),non_frontage_blocks=len(nonfront),removed_blocks=len(removed),retained_blocks=len(kept),removed_buildings=len(lost),retained_buildings=len(m['buildings'])-len(lost),protected_park_blocks=[b['id'] for b in d['blocks'] if b['role']=='park'],removed_ids=sorted(removed),removed_building_ids=lost,frontage_rule='Within 45m of either main race-road centreline; conservative road + sidewalk protection',selection='Farthest whole non-frontage blocks first, retaining all six park sites',limits='Preview only; no runtime geometry removed. Local street pruning/edge treatment and performance require implementation and measurement.')
(OUT/'narrow-city-proposal.json').write_text(json.dumps(report,indent=2)+'\n')
fig,axes=plt.subplots(1,2,figsize=(15,17),facecolor='#edf1e8')
colors={'old_city':'#bbbcaa','market':'#d7b77d','mixed':'#a8b9c4','terraces':'#c0a899','north_city':'#95a9b5','tower':'#858f9e'}
for ax,proposed in zip(axes,[False,True]):
 ax.set_facecolor('#e0ebd7')
 for b in d['blocks']:
  i=b['id'];p=blocks[i]
  if proposed and i in removed:
   ax.add_patch(Patch(list(p.exterior.coords),facecolor='#d3e2c6',edgecolor='#c0d3b1',linewidth=.25));continue
  color='#72a86b' if b['role']=='park' else colors.get(b['district'],'#b8bcaa')
  ax.add_patch(Patch(list(p.exterior.coords),facecolor=color,edgecolor='#f5f5ee',linewidth=.65))
 # Small actual building footprints make retained urban density visible.
 polys=[Patch(b['shape']['rings'][0]) for b in m['buildings'] if not proposed or b['id'] not in lost]
 ax.add_collection(PatchCollection(polys,facecolor='#43515b',edgecolor='none',alpha=.45))
 for r in routes:
  x,z=r.xy;ax.plot(x,z,color='#fff9e8',lw=3);ax.plot(x,z,color='#ce633f',lw=1.5)
 for s,label in [(s,'CITY HALL' if s['kind']=='city_hall' else 'FOUNTAIN PARK') for s in m['civic_sites'] if s['kind']!='cafe']:
  x,z=s['position'];ax.scatter(x,z,s=38,c='#176d58',edgecolor='white',zorder=8)
  ax.annotate(label,(x,z),xytext=(10,7),textcoords='offset points',fontsize=9,fontweight='bold',color='#164c43',bbox=dict(facecolor='white',alpha=.9,edgecolor='none',pad=2))
 ax.set_xlim(-1300,2200);ax.set_ylim(-400,9450);ax.set_aspect('equal');ax.set_xlabel('East / west (metres)');ax.set_ylabel('North (metres)');ax.spines[['top','right']].set_visible(False)
 ax.set_title(('PROPOSED · outer blocks returned to green space' if proposed else 'CURRENT · full city footprint'),fontsize=12,pad=15,fontweight='bold')
fig.suptitle('RoadDash — narrower city proposal',fontsize=24,fontweight='bold',y=.96,color='#203d35')
fig.text(.5,.918,f"Keep every race-frontage block · remove {len(removed)} of {len(nonfront)} non-frontage blocks ({100*len(removed)/len(nonfront):.0f}%)",ha='center',fontsize=13)
fig.text(.5,.888,f"{len(blocks)} → {len(kept)} city blocks     |     {len(m['buildings']):,} → {len(m['buildings'])-len(lost):,} buildings     |     all park sites retained",ha='center',fontsize=12)
fig.text(.5,.06,'Orange = unchanged race roads / both branches     •     Pale green = proposed removed outer blocks',ha='center',fontsize=11)
fig.text(.5,.035,'Proposal only. Streets at new edges still need redesign. Building reduction is not a measured FPS improvement.',ha='center',fontsize=10,color='#43574e')
fig.subplots_adjust(top=.86,bottom=.10,wspace=.18)
fig.savefig(OUT/'narrow-city-proposal.png',dpi=150,facecolor=fig.get_facecolor())
print({k:v for k,v in report.items() if not k.endswith('_ids')})
