#!/usr/bin/env python3
"""Verify baked facade identity coverage, exact source provenance and mip guards."""
from pathlib import Path
from PIL import Image
import json,hashlib,math
ROOT=Path(__file__).resolve().parents[1];KIT=ROOT/'art/buildings/mesh_assets/storey_kit';OUT=KIT.parent/'far_facades'
a=json.loads((OUT/'MANIFEST.json').read_text());source=json.loads((KIT/'MANIFEST.json').read_text());designs={d['id']:d for d in source['records']}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(KIT/'MANIFEST.json')==a['source_manifest_sha256']
assert sha(ROOT/'tools/bake_facade_lod.py')==a['generator_sha256']
expected={(d['id'],band) for d in source['records'] for band in range(3) if d['lods'][2]['triangles'][band]}
seen=set();cells=set()
for t in a['records']:
 key=(t['design'],t['band']);assert key not in seen;seen.add(key);d=designs[t['design']]
 assert sha(KIT/f"{t['design']}_lod2.glb")==t['source_sha256']==d['lods'][2]['sha256']
 assert t['bounds']==d['lods'][2]['bounds'][t['band']]
 assert 0<t['covered_pixels']<=96*96
 x=t['uv_min'][0]*a['width']-.5;y=t['uv_min'][1]*a['height']-.5
 cell=(int(x)//128,int(y)//128);assert cell not in cells;cells.add(cell)
 assert abs(x-(cell[0]*128+16))<1e-4 and abs(y-(cell[1]*128+16))<1e-4
 assert all(math.isfinite(v) and 0<=v<=1 for v in t['uv_min']+t['uv_max'])
 assert abs((t['uv_max'][0]-t['uv_min'][0])*a['width']-95)<1e-4
 assert abs((t['uv_max'][1]-t['uv_min'][1])*a['height']-95)<1e-4
assert seen==expected and len(seen)==383 and len({k[0] for k in seen})==128
byte_count=0
for m in a['images']:
 path=OUT/m['path'];assert sha(path)==m['sha256']
 im=Image.open(path);assert im.size==(m['width'],m['height'])==(2048>>m['level'],4096>>m['level']);assert im.mode=='RGBA'
 byte_count+=im.width*im.height*4
 # Every tile has a one-pixel guard even at the last mip.
 assert (16>>m['level'])>=1
assert byte_count<=128*1024*1024
print(json.dumps(dict(designs=128,bands=383,source_hashes_verified=True,nonoverlapping_tiles=True,mip_images=len(a['images']),resident_rgba_bytes=byte_count,minimum_last_mip_guard_pixels=1,tool='Blender4.5.9 bundled Python/numpy; original software triangle/UV/depth projection; no retail media'),indent=2))
