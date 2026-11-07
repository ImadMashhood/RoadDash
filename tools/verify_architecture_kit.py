#!/usr/bin/env python3
"""Independently inspect derived geometry, shared media hashes and three-scene storey contracts."""
import json,hashlib,math,struct,argparse
from pathlib import Path
from verify_building_mesh_assets import read_glb,accessor
ROOT=Path(__file__).resolve().parents[1];KIT=ROOT/'art/buildings/mesh_assets';OUT=KIT/'storey_kit'
def verify():
    m=json.loads((OUT/'MANIFEST.json').read_text());ids=set();models=[];textures=set();total=0
    for r in m['records']:
        assert r['id']not in ids;ids.add(r['id']);actual=[]
        assert (KIT/'materials'/f"{r['masonry']}_albedo.png").is_file()
        for l in r['lods']:
            p=OUT/f"{r['id']}_lod{l['lod']}.glb";raw=p.read_bytes();assert hashlib.sha256(raw).hexdigest()==l['sha256'];d,b=read_glb(p)
            assert len(d['scenes'])==3 and 'skins'not in d and 'animations'not in d
            source=KIT/('front_modules'if r['asset_type']=='facade'else'models')/p.name
            assert hashlib.sha256(source.read_bytes()).hexdigest()==l['source_sha256']
            for image in d.get('images',[]):
                image_path=(OUT/image['uri']).resolve();assert image_path.is_relative_to(OUT.resolve());data=image_path.read_bytes()
                assert hashlib.sha256(data).hexdigest()==image_path.stem and data[:8]==b'\x89PNG\r\n\x1a\n';textures.add(image_path.name)
            counts=[]
            for band,scene in enumerate(d['scenes']):
                triangles=0; points=[]
                for node in scene['nodes']:
                    mesh=d['meshes'][d['nodes'][node]['mesh']]
                    for part in mesh['primitives']:
                        pos=list(accessor(d,b,part['attributes']['POSITION']));indices=[i[0]for i in accessor(d,b,part['indices'])]
                        points.extend(pos)
                        assert len(indices)%3==0 and all(i<len(pos)for i in indices);triangles+=len(indices)//3
                        for name in ['POSITION','NORMAL','TEXCOORD_0']:
                            rows=list(accessor(d,b,part['attributes'][name]));assert len(rows)==len(pos) and all(math.isfinite(v)for row in rows for v in row)
                        assert 0<=part['material']<len(d['materials'])
                actual_bounds={'min':[min((p[k]for p in points),default=0)for k in range(3)],'max':[max((p[k]for p in points),default=0)for k in range(3)]}
                assert all(abs(actual_bounds[side][k]-l['bounds'][band][side][k])<1e-5 for side in ['min','max']for k in range(3)),(r['id'],band)
                counts.append(triangles)
            assert counts==l['triangles'];actual.append(sum(counts));total+=len(raw)
        assert actual[0]>=actual[1]>=actual[2],(r['id'],actual)
        models.append({'id':r['id'],'triangles_per_lod':actual})
    assert len(ids)==128 and len(textures)>10
    return {'designs':len(ids),'glbs':len(ids)*3,'shared_textures':len(textures),'geometry_bytes':total,'source':'Own authored original models; no retail pixels','models':models}
if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,default=ROOT/'docs/testing/architecture-valley/kit-validation.json');a=ap.parse_args();result=verify();a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items()if k!='models'})
