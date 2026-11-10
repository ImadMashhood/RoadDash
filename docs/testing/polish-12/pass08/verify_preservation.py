#!/usr/bin/env python3
"""Compare all baked content with committed pass07; record exact scoped changes."""
import hashlib,json,subprocess
from pathlib import Path
from collections import Counter
root=Path(__file__).resolve().parents[4];out=Path(__file__).resolve().parent
base='669cb094f8d1930b81f783c350275f65bbeabac1'
def old(path):return subprocess.check_output(['git','show',f'{base}:{path}'],cwd=root)
world=json.loads((root/'content/world/metropolis.json').read_text());before=json.loads(old('content/world/metropolis.json'))
layout=json.loads((root/'docs/planning/farms-long-city/LAYOUT.json').read_text())
blocks={b['id']:block for block in layout['blocks'] for b in block['buildings']}
changed=[]
for a,b in zip(before['buildings'],world['buildings']):
 assert a['id']==b['id']
 if a['style']!=b['style']:
  block=blocks[b['id']];z=block['center'][1]
  assert z<1800 and b['floors']<=5
  changed.append(dict(id=b['id'],z=z,floors=b['floors'],before=a['style'],after=b['style']))
 assert {k:v for k,v in a.items() if k!='style'}=={k:v for k,v in b.items() if k!='style'}
assert len(world['buildings'])==len(before['buildings'])==19068
assert {k:v for k,v in before.items() if k!='buildings'}=={k:v for k,v in world.items() if k!='buildings'}
paths=['docs/planning/farms-long-city/LAYOUT.json','assets/metropolis/facades.rgba','assets/metropolis/facades-packed.rgba','art/buildings/mesh_assets/far_facades/MANIFEST.json']
hashes={}
for path in paths:
 raw=(root/path).read_bytes();assert raw==old(path);hashes[path]=hashlib.sha256(raw).hexdigest()
assert changed
report=dict(base_commit=base,total_buildings=19068,changed_count=len(changed),changed_by_floors=dict(Counter(x['floors'] for x in changed)),changed_z_bounds=[min(x['z'] for x in changed),max(x['z'] for x in changed)],all_nonstyle_world_content_equal=True,unchanged_source_and_pixels_sha256=hashes,changed=changed)
(out/'preservation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='changed'},indent=2))
