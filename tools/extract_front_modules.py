#!/usr/bin/env python3
"""Derive true geometric facade modules from our own GLBs; retain original files.
No bitmap projection, third-party model source or original game pixels involved.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import struct
from verify_building_mesh_assets import read_glb,accessor,FORMATS,WIDTH
ROOT=Path(__file__).resolve().parents[1];KIT=ROOT/'art/buildings/mesh_assets';OUT=KIT/'front_modules'


def derive(identity,depth,lod):
    source=KIT/'models'/f'{identity}_lod{lod}.glb';d,b=read_glb(source);plane=depth/2;threshold=plane-.6
    binary=bytearray();views=[];accessors=[];primitives=[];triangles=0
    def view(raw,target=None):
        while len(binary)%4:binary.append(0)
        start=len(binary);binary.extend(raw);v=dict(buffer=0,byteOffset=start,byteLength=len(raw))
        if target:v['target']=target
        views.append(v);return len(views)-1
    def array(raw,component,kind,count,limits=None):
        a=dict(bufferView=view(raw,34963 if kind=='SCALAR' else 34962),componentType=component,type=kind,count=count)
        if limits:a.update(min=limits[0],max=limits[1])
        accessors.append(a);return len(accessors)-1
    for primitive in d['meshes'][0]['primitives']:
        positions=list(accessor(d,b,primitive['attributes']['POSITION']));indices=[v[0]for v in accessor(d,b,primitive['indices'])]
        kept=[i for start in range(0,len(indices),3)if all(positions[j][2]>=threshold for j in indices[start:start+3])for i in indices[start:start+3]]
        if not kept:continue
        chosen=sorted(set(kept));mapping={old:new for new,old in enumerate(chosen)};attrs={}
        for name,idx in primitive['attributes'].items():
            a=d['accessors'][idx];values=list(accessor(d,b,idx));selected=[values[i]for i in chosen]
            if name=='POSITION':selected=[(v[0],v[1],v[2]-plane)for v in selected]
            code=FORMATS[a['componentType']][0];width=WIDTH[a['type']];packer=struct.Struct('<'+code*width)
            raw=b''.join(packer.pack(*v)for v in selected)
            limits=([min(v[k]for v in selected)for k in range(3)],[max(v[k]for v in selected)for k in range(3)])if name=='POSITION'else None
            attrs[name]=array(raw,a['componentType'],a['type'],len(selected),limits)
        raw=struct.pack('<'+'I'*len(kept),*(mapping[i]for i in kept));primitives.append(dict(attributes=attrs,indices=array(raw,5125,'SCALAR',len(kept)),material=primitive['material']));triangles+=len(kept)//3
    if not primitives:raise ValueError('No geometric facade '+identity)
    used_materials=sorted({p['material']for p in primitives});material_map={old:new for new,old in enumerate(used_materials)}
    materials=[d['materials'][i]for i in used_materials]
    def texture_refs(value):
        if isinstance(value,dict):
            for key,v in value.items():
                if key.endswith('Texture')and isinstance(v,dict)and 'index'in v:yield v
                else:yield from texture_refs(v)
        elif isinstance(value,list):
            for v in value:yield from texture_refs(v)
    refs=list(texture_refs(materials));texture_indices=sorted({v['index']for v in refs});texture_map={old:new for new,old in enumerate(texture_indices)}
    textures=[d['textures'][i]for i in texture_indices];image_indices=sorted({t['source']for t in textures});image_map={old:new for new,old in enumerate(image_indices)}
    images=[]
    for i in image_indices:
        image=dict(d['images'][i]);old=d['bufferViews'][image['bufferView']];start=old.get('byteOffset',0);image['bufferView']=view(b[start:start+old['byteLength']]);images.append(image)
    for ref in refs:ref['index']=texture_map[ref['index']]
    for t in textures:t['source']=image_map[t['source']]
    for p in primitives:p['material']=material_map[p['material']]
    while len(binary)%4:binary.append(0)
    result=dict(asset=dict(version='2.0',generator='RoadDash own-model geometric facade derivation'),scene=0,scenes=[dict(nodes=[0])],nodes=[dict(mesh=0,name=f'{identity}_FRONT_LOD{lod}')],
        meshes=[dict(primitives=primitives)],materials=materials,textures=textures,images=images,samplers=d.get('samplers',[]),accessors=accessors,bufferViews=views,buffers=[dict(byteLength=len(binary))])
    if 'extensionsUsed'in d:result['extensionsUsed']=d['extensionsUsed']
    header=json.dumps(result,separators=(',',':')).encode();header+=b' '*((-len(header))%4)
    output=OUT/f'{identity}_lod{lod}.glb';output.write_bytes(struct.pack('<4sII',b'glTF',2,28+len(header)+len(binary))+struct.pack('<II',len(header),0x4e4f534a)+header+struct.pack('<II',len(binary),0x004e4942)+binary)
    return dict(file=output.name,lod=lod,triangles=triangles,bytes=output.stat().st_size,sha256=hashlib.sha256(output.read_bytes()).hexdigest(),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),minimum_local_depth_m=-.6)


def main():
    ap=argparse.ArgumentParser();ap.add_argument('--workers',type=int,default=2);a=ap.parse_args();OUT.mkdir(exist_ok=True)
    designs=[d for d in json.loads((KIT/'catalog_designs.json').read_text())['designs']if d['asset_type']=='facade']
    def one(d):
        lods=[derive(d['id'],d['depth'],lod)for lod in range(3)];print('DERIVED '+d['id'],flush=True);return dict(id=d['id'],width=d['width'],height=d['height'],lods=lods)
    with ThreadPoolExecutor(max_workers=a.workers)as pool:records=list(pool.map(one,designs))
    (OUT/'MANIFEST.json').write_text(json.dumps(dict(schema_version=1,facade_designs=100,glbs=300,records=records,scope='Own modeled front geometry only; shared solid assembly supplies sides, rear and roof'),indent=2)+'\n')


if __name__=='__main__':main()
