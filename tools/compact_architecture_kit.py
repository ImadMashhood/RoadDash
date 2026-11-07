#!/usr/bin/env python3
"""Lossless exact-attribute vertex indexing for our derived storey GLBs."""
import json,struct,hashlib
from pathlib import Path
from verify_building_mesh_assets import read_glb,accessor

def compact(path):
    d,b=read_glb(path);binary=bytearray();views=[];acc=[]
    def write(values,width,kind,component):
        binary.extend(b'\0'*(-len(binary)%4));start=len(binary)
        pack=struct.Struct('<'+('f'if component==5126 else'I')*width)
        for v in values:binary.extend(pack.pack(*v))
        views.append(dict(buffer=0,byteOffset=start,byteLength=len(binary)-start,target=34962 if component==5126 else 34963))
        a=dict(bufferView=len(views)-1,componentType=component,type=kind,count=len(values))
        if kind=='VEC3':a.update(min=[min(v[k]for v in values)for k in range(3)],max=[max(v[k]for v in values)for k in range(3)])
        acc.append(a);return len(acc)-1
    for mesh in d['meshes']:
        for p in mesh['primitives']:
            attrs=p['attributes'];position=list(accessor(d,b,attrs['POSITION']));normal=list(accessor(d,b,attrs['NORMAL']));uv=list(accessor(d,b,attrs['TEXCOORD_0']))
            unique=[];table={};remap=[]
            for row in zip(position,normal,uv):
                key=tuple(v for stream in row for v in stream)
                if key not in table:table[key]=len(unique);unique.append(row)
                remap.append(table[key])
            indices=[(remap[i[0]],)for i in accessor(d,b,p['indices'])]
            p['attributes']={'POSITION':write([v[0]for v in unique],3,'VEC3',5126),'NORMAL':write([v[1]for v in unique],3,'VEC3',5126),'TEXCOORD_0':write([v[2]for v in unique],2,'VEC2',5126)}
            p['indices']=write(indices,1,'SCALAR',5125)
    binary.extend(b'\0'*(-len(binary)%4));d['buffers']=[dict(byteLength=len(binary))];d['bufferViews']=views;d['accessors']=acc
    header=json.dumps(d,separators=(',',':')).encode();header+=b' '*(-len(header)%4)
    path.write_bytes(struct.pack('<4sII',b'glTF',2,28+len(header)+len(binary))+struct.pack('<II',len(header),0x4e4f534a)+header+struct.pack('<II',len(binary),0x004e4942)+binary)

if __name__=='__main__':
    folder=Path(__file__).resolve().parents[1]/'art/buildings/mesh_assets/storey_kit';manifest=folder/'MANIFEST.json';m=json.loads(manifest.read_text())
    for r in m['records']:
        for lod in r['lods']:
            p=folder/f"{r['id']}_lod{lod['lod']}.glb";compact(p);lod['bytes']=p.stat().st_size;lod['sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
        print('INDEXED',r['id'],flush=True)
    manifest.write_text(json.dumps(m,indent=2)+'\n')
