"""Run in Blender Python: effective source roughness must equal runtime fixture.
Uses the actual baker/profile lookup and quantized map, not just manifest hashes.
"""
import sys,json
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent))
import bake_facade_lod as bake
from verify_building_mesh_assets import read_glb

samples=[]
for identity in ['BUSS1','CHINA12','OFFC1','VICT11','WHSE11']:
    d,_=read_glb(bake.KIT/f'{identity}_lod2.glb')
    for material in d['materials']:
        p=material['pbrMetallicRoughness']
        if 'baseColorTexture' not in p:continue
        ix=p['baseColorTexture']['index'];rough=bake.effective_roughness(d,ix)
        assert rough is not None,(identity,material['name'])
        uri=d['images'][d['textures'][ix]['source']]['uri']
        profile=bake.PROFILES[Path(uri).stem]
        coordinates=[(0,0),(1,0),(0,1),(37,91),(511,511)]
        values=[int(round(float(rough[y,x,1])*255)) for x,y in coordinates]
        samples.append(dict(roughness=profile['roughness'],coordinates=coordinates,values=values))
# Golden values from explicit integer hash/f32 arithmetic; shared with the Rust
# test to protect either language's generation against later divergence.
fixture=json.loads((bake.ROOT/'docs/testing/polish-12/pass07/roughness-fixture.json').read_text())
for row in samples:
    assert row['values']==next(r['values'] for r in fixture if r['roughness']==row['roughness']),row
print('PASS effective profile lookup and runtime roughness parity:',len(samples),'material instances')
