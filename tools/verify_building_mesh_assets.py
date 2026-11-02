#!/usr/bin/env python3
"""Independent GLB binary, geometry and embedded-material validation; no Blender needed."""
import argparse
import hashlib
import json
import math
import struct
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FOLDER=ROOT/'art/buildings/mesh_assets'
FORMATS={5121:('B',1),5123:('H',2),5125:('I',4),5126:('f',4)}
WIDTH={'SCALAR':1,'VEC2':2,'VEC3':3,'VEC4':4}


def read_glb(path):
    b=path.read_bytes()
    if len(b)<28 or struct.unpack_from('<4sII',b)!=(b'glTF',2,len(b)):raise ValueError('Invalid GLB header/length')
    offset=12;chunks=[]
    while offset<len(b):
        size,kind=struct.unpack_from('<II',b,offset);offset+=8
        if size%4 or offset+size>len(b):raise ValueError('Invalid GLB chunk')
        chunks.append((kind,b[offset:offset+size]));offset+=size
    if len(chunks)!=2 or chunks[0][0]!=0x4e4f534a or chunks[1][0]!=0x004e4942:raise ValueError('Expected embedded GLB JSON/BIN')
    return json.loads(chunks[0][1]),chunks[1][1]


def accessor(document,binary,index):
    a=document['accessors'][index];view=document['bufferViews'][a['bufferView']]
    code,size=FORMATS[a['componentType']];width=WIDTH[a['type']]
    start=view.get('byteOffset',0)+a.get('byteOffset',0);stride=view.get('byteStride',width*size)
    if start+(a['count']-1)*stride+width*size>len(binary):raise ValueError('Accessor overrun')
    for i in range(a['count']):yield struct.unpack_from('<'+code*width,binary,start+i*stride)


def inspect(path):
    d,b=read_glb(path)
    if len(d['meshes'])!=1 or len(d['nodes'])!=1 or 'skins' in d or 'animations' in d:raise ValueError('Expected standalone static building, not a rig/image card scene')
    mins=[math.inf]*3;maxs=[-math.inf]*3;triangles=0;vertices=0;material_fronts={}
    for v in d['bufferViews']:
        if v.get('byteOffset',0)+v['byteLength']>len(b):raise ValueError('Buffer view overrun')
    for p in d['meshes'][0]['primitives']:
        if p.get('mode',4)!=4:raise ValueError('Expected triangles')
        attr=p['attributes'];positions=list(accessor(d,b,attr['POSITION']));vertices+=len(positions)
        for values in positions:
            if not all(math.isfinite(v) for v in values):raise ValueError('Nonfinite mesh position')
            mins=[min(a,v) for a,v in zip(mins,values)];maxs=[max(a,v) for a,v in zip(maxs,values)]
        material_name=d['materials'][p['material']]['name']
        material_fronts[material_name]=max(v[2] for v in positions)
        indices=list(accessor(d,b,p['indices']))
        if len(indices)%3 or max(i[0] for i in indices)>=len(positions):raise ValueError('Bad triangle indices')
        triangles+=len(indices)//3
        for key in ['NORMAL','TEXCOORD_0']:
            values=list(accessor(d,b,attr[key]))
            if len(values)!=len(positions) or not all(math.isfinite(x) for v in values for x in v):raise ValueError('Invalid normal/UV stream')
    dimensions=[b-a for a,b in zip(mins,maxs)]
    if not (3<=dimensions[0]<=4.5 and 9<=dimensions[1]<=11 and 6<=dimensions[2]<=8 and mins[1]>=-.02):raise ValueError(f'Unexpected physical bounds {mins} {maxs}')
    for image in d.get('images',[]):
        if 'uri' in image:raise ValueError('GLB not self-contained')
        view=d['bufferViews'][image['bufferView']];start=view.get('byteOffset',0);png=b[start:start+view['byteLength']]
        if png[:8]!=b'\x89PNG\r\n\x1a\n' or struct.unpack_from('>II',png,16)!=(512,512):raise ValueError('Expected authored square material tiles, never facade/render imagery')
    relief=None
    if '_lod0' in path.stem:
        tiles=[name for name in material_fronts if name.endswith('_masonry_tile')]
        if not tiles:raise ValueError('Missing geometric masonry backing material')
        tile=tiles[0];base=tile.removesuffix('_masonry_tile')
        relief=max(v for name,v in material_fronts.items() if name in [base,base+'_dark',base+'_light'])-material_fronts[tile]
        if relief<.02:raise ValueError('Individual bricks are hidden behind wall backing')
    if not d.get('images') or triangles<1500:raise ValueError('Missing real modeled detail or materials')
    return dict(file=path.name,vertices=vertices,triangles=triangles,dimensions_m=dimensions,
                min_m=mins,max_m=maxs,masonry_projection_m=relief,materials=len(d['materials']),embedded_material_tiles=len(d.get('images',[])),
                sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def verify():
    manifest=json.loads((FOLDER/'MANIFEST.json').read_text());records=[]
    for model in manifest['records']:
        lods=[]
        for lod in model['lods']:
            path=FOLDER/'models'/lod['file'];record=inspect(path)
            if record['sha256']!=lod['sha256']:raise ValueError('Changed GLB after authoring export')
            lods.append(record);records.append(record)
        if not lods[0]['triangles']>lods[1]['triangles']>lods[2]['triangles']:raise ValueError('LOD geometry reduction missing')
        if lods[0]['triangles']<50000 or lods[2]['triangles']>4000:raise ValueError('Detailed/coarse geometry budgets violated')
    return dict(models=len(manifest['records']),glb_files=len(records),records=records,
                scope='Binary/geometry/material validation; visual fidelity and game integration acceptance separate')


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,default=ROOT/'docs/testing/building-meshes/glb-validation.json');a=ap.parse_args()
    report=verify();a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'models':report['models'],'glb_files':report['glb_files'],'passed':True}))
