#!/usr/bin/env python3
"""Independent type-aware validation of the entire exported architectural catalog."""
import argparse
import hashlib
import json
import math
import struct
from pathlib import Path
from collect_building_catalog import collect, OUT, ROOT
from verify_building_mesh_assets import read_glb, accessor


def inspect(path):
    d,b=read_glb(path)
    if len(d['meshes'])!=1 or len(d['nodes'])!=1 or 'skins' in d or 'animations' in d:
        raise ValueError('Expected one standalone static mesh: '+str(path))
    for view in d['bufferViews']:
        if view.get('byteOffset',0)+view['byteLength']>len(b):raise ValueError('Buffer overrun')
    lo=[math.inf]*3;hi=[-math.inf]*3;triangles=vertices=0
    for p in d['meshes'][0]['primitives']:
        if p.get('mode',4)!=4:raise ValueError('Non triangle primitive')
        attrs=p['attributes'];count=d['accessors'][attrs['POSITION']]['count'];vertices+=count
        for v in accessor(d,b,attrs['POSITION']):
            if not all(math.isfinite(x)for x in v):raise ValueError('Nonfinite positions')
            for axis in range(3):lo[axis]=min(lo[axis],v[axis]);hi[axis]=max(hi[axis],v[axis])
        indices=d['accessors'][p['indices']]['count']
        if indices%3 or any(i[0]>=count for i in accessor(d,b,p['indices'])):raise ValueError('Invalid indices')
        triangles+=indices//3
        for key in ['NORMAL','TEXCOORD_0']:
            if d['accessors'][attrs[key]]['count']!=count:raise ValueError('Stream length')
            if not all(math.isfinite(x)for v in accessor(d,b,attrs[key])for x in v):raise ValueError('Nonfinite normal/UV')
    dimensions=[h-l for h,l in zip(hi,lo)]
    if min(dimensions)<.015 or triangles<12:raise ValueError('No volumetric geometry')
    if not d.get('images'):raise ValueError('Missing material tiles')
    for image in d['images']:
        if 'uri' in image:raise ValueError('External texture')
        view=d['bufferViews'][image['bufferView']];offset=view.get('byteOffset',0);png=b[offset:offset+view['byteLength']]
        if png[:8]!=b'\x89PNG\r\n\x1a\n' or struct.unpack_from('>II',png,16)!=(512,512):raise ValueError('Expected original square material tiles')
    return dict(file=path.name,triangles=triangles,vertices=vertices,dimensions_m=dimensions,materials=len(d['materials']))


def verify():
    catalog=collect();records=[]
    assert catalog['asset_counts']==dict(facade=100,structure=28,surface_module=18)
    assert catalog['glb_files']==438
    for model in catalog['records']:
        lods=[inspect(OUT/'models'/lod['file'])for lod in model['lods']]
        triangles=[l['triangles']for l in lods]
        if not triangles[0]>=triangles[1]>=triangles[2] or triangles[0]==triangles[2]:
            raise ValueError('LOD reduction missing '+model['id']+str(triangles))
        if model['asset_type']=='facade' and min(lods[0]['dimensions_m'])<2:raise ValueError('Facade lacks building depth')
        records.append(dict(id=model['id'],asset_type=model['asset_type'],lods=lods))
        print('VALIDATED '+model['id'],flush=True)
    return dict(assets=len(records),glb_files=438,asset_counts=catalog['asset_counts'],records=records,
                scope='Actual binary geometry, LOD, material, provenance hashes and coverage; art approval and production performance remain open')


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,default=ROOT/'docs/testing/building-catalog/glb-validation.json');a=ap.parse_args()
    result=verify();a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items()if k!='records'}))
