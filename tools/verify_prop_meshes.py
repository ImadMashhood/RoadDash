#!/usr/bin/env python3
"""Independent binary geometry, material, LOD and complete-ID validation."""
import hashlib
import json
import math
from pathlib import Path
import struct
from verify_building_mesh_assets import read_glb, accessor
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/props/mesh_assets'

def inspect(path):
 d,b=read_glb(path);triangles=0;vertices=0
 if d.get('skins')or d.get('animations'):raise ValueError('Draft static catalog must not claim a skin/animation')
 for view in d['bufferViews']:
  if view.get('byteOffset',0)+view['byteLength']>len(b):raise ValueError('GLB buffer overrun')
 all_positions=[]
 for mesh in d['meshes']:
  for p in mesh['primitives']:
   if p.get('mode',4)!=4:raise ValueError('Expected indexed triangles')
   pos=list(accessor(d,b,p['attributes']['POSITION']));vertices+=len(pos);all_positions.extend(pos)
   if not all(math.isfinite(v)for co in pos for v in co):raise ValueError('Nonfinite position')
   indices=[v[0]for v in accessor(d,b,p['indices'])]
   if not indices or len(indices)%3 or min(indices)<0 or max(indices)>=len(pos):raise ValueError('Invalid indices')
   triangles+=len(indices)//3
   for attr in ['NORMAL','TEXCOORD_0']:
    values=list(accessor(d,b,p['attributes'][attr]))
    if len(values)!=len(pos)or not all(math.isfinite(v)for row in values for v in row):raise ValueError('Invalid normals/UVs')
   if 'COLOR_0'in p['attributes']:
    colors=list(accessor(d,b,p['attributes']['COLOR_0']))
    if not all(0<=v<=1 for row in colors for v in row):raise ValueError('Invalid vertex color export')
 for node in d['nodes']:
  for key in ['matrix','translation','rotation','scale']:
   if not all(math.isfinite(v)for v in node.get(key,[])):raise ValueError('Invalid transform')
 for im in d.get('images',[]):
  if 'uri'in im:raise ValueError('External media dependency')
  view=d['bufferViews'][im['bufferView']];start=view.get('byteOffset',0);png=b[start:start+view['byteLength']]
  if png[:8]!=b'\x89PNG\r\n\x1a\n'or struct.unpack_from('>II',png,16)!=(128,128):raise ValueError('Only original small material tiles allowed; no concept sheets')
 if not d.get('images'):raise ValueError('Missing original material tiles')
 if triangles<12 or len(all_positions)<8:raise ValueError('Empty or image-card substitute')
 spans=[max(p[i]for p in all_positions)-min(p[i]for p in all_positions)for i in range(3)]
 if min(spans)<.001:raise ValueError('Planar asset instead of 3D model')
 return dict(triangles=triangles,vertices=vertices,mesh_nodes=len(d['meshes']),material_tiles=len(d.get('images',[])),sha256=hashlib.sha256(path.read_bytes()).hexdigest())

def verify():
 manifest=json.loads((OUT/'MANIFEST.json').read_text());designs=json.loads((OUT/'designs.json').read_text())['designs'];queue=json.loads((ROOT/'docs/research/props/rebuild-queue.json').read_text())
 ids=[d['id']for d in manifest['records']]
 assert len(ids)==len(set(ids))and set(ids)=={d['id']for d in designs}=={j['id']for j in queue['jobs']}
 output=[]
 for r in manifest['records']:
  lods=[inspect(OUT/l['file'])for l in r['lods']]
  for expected,actual in zip(r['lods'],lods):
   assert actual['sha256']==expected['sha256'],r['id']
   assert actual['triangles']==expected['triangles'],r['id']
  assert lods[0]['triangles']>lods[1]['triangles']>lods[2]['triangles'],r['name']
  assert not r['production_approved']
  if r['group']in {'animals','pedestrians_animals'}:assert r['rig_status']=='not_rigged'
  assert all(math.isfinite(v)and v>0 for v in r['collider']['size_y_up'])
  output.append(dict(id=r['id'],name=r['name'],lods=lods))
 for source in manifest['sources']:assert hashlib.sha256((OUT/source['file']).read_bytes()).hexdigest()==source['sha256']
 assert hashlib.sha256((ROOT/'tools/build_prop_meshes.py').read_bytes()).hexdigest()==manifest['authoring_script_sha256']
 rendered={c['id']for s in manifest['renders']for c in s['cells']}
 by_id={r['id']:r for r in manifest['records']}
 for s in manifest['renders']+manifest.get('clay_renders',[]):
  for c in s['cells']:assert c['sha256']==by_id[c['id']]['lods'][0]['sha256'], 'stale render: '+c['id']
  assert s['method'].startswith('Blender render of re-imported')
  assert hashlib.sha256((OUT/s['file']).read_bytes()).hexdigest()==s['sha256']
 return dict(models=len(ids),glbs=len(output)*3,source_scenes=len(manifest['sources']),rendered_ids=len(rendered),render_missing=sorted(set(ids)-rendered),records=output,
  scope='Geometry/bytes/LOD/material coverage, not art approval, skinning, manifold proof or live gameplay validation')
if __name__=='__main__':
 report=verify();p=ROOT/'docs/testing/prop-meshes/verification.json';p.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items()if k!='records'}))
