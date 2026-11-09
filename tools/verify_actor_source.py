#!/usr/bin/env python3
"""Independent neutral-surface, midpoint topology and derivative-cost audit."""
import hashlib
import json
import math
from pathlib import Path
import subprocess
from verify_building_mesh_assets import read_glb, accessor

ROOT=Path(__file__).resolve().parents[1]

def convert(v,node):
    x,y,z,w=node.get('rotation',[0,0,0,1]);a,b,c=v
    t=(2*(y*c-z*b),2*(z*a-x*c),2*(x*b-y*a))
    q=(a+w*t[0]+y*t[2]-z*t[1],b+w*t[1]+z*t[0]-x*t[2],c+w*t[2]+x*t[1]-y*t[0])
    return tuple((q[i]+node.get('translation',[0,0,0])[i])*1.23 for i in range(3))

def main():
    intake=json.loads((ROOT/'docs/testing/props-in-game/runtime-intake.json').read_text());records=[]
    for row in intake['files']:
        path=ROOT/row['file'];d,raw=read_glb(path);source,original=read_glb(ROOT/row['source_file'])
        before=subprocess.check_output(['git','show',f"96be522:{row['file']}"],cwd=ROOT)
        if len(d['skins'][0]['joints'])==10:
            assert path.read_bytes()==before,'cow bytes changed'
            continue
        for key in ['materials','images','textures','samplers']:assert d.get(key)==source.get(key),(path,key)
        assert raw[:len(original)]==original,'original embedded buffer changed'
        vertices=triangles=old_vertices=old_triangles=refined=0
        for old,node in zip(source['nodes'],d['nodes']):
            if 'mesh'not in old:continue
            for op,np in zip(source['meshes'][old['mesh']]['primitives'],d['meshes'][node['mesh']]['primitives']):
                oldp=[convert(v,old)for v in accessor(source,original,op['attributes']['POSITION'])]
                newp=list(accessor(d,raw,np['attributes']['POSITION']));n=len(oldp)
                olduv=list(accessor(source,original,op['attributes']['TEXCOORD_0']));newuv=list(accessor(d,raw,np['attributes']['TEXCOORD_0']))
                oi=[x[0]for x in accessor(source,original,op['indices'])];ni=[x[0]for x in accessor(d,raw,np['indices'])]
                expected=list(oldp);expecteduv=list(olduv);expectedindices=[];edges={}
                if len(newp)>n:
                    refined+=1
                    for start in range(0,len(oi),3):
                        a,b,c=oi[start:start+3];mid=[]
                        for edge in [(a,b),(b,c),(c,a)]:
                            key=tuple(sorted(edge))
                            if key not in edges:
                                edges[key]=len(expected)
                                expected.append(tuple((oldp[edge[0]][i]+oldp[edge[1]][i])*.5 for i in range(3)))
                                expecteduv.append(tuple((olduv[edge[0]][i]+olduv[edge[1]][i])*.5 for i in range(2)))
                            mid.append(edges[key])
                        ab,bc,ca=mid
                        expectedindices.extend([a,ab,ca,ab,b,bc,ca,bc,c,ab,bc,ca])
                    assert ni==expectedindices,'nonconforming topology or changed winding'
                else:assert ni==oi
                assert len(newp)==len(expected) and len(newuv)==len(expecteduv)
                assert max(abs(a-b)for p,q in zip(newp,expected)for a,b in zip(p,q))<5e-7
                assert max(abs(a-b)for p,q in zip(newuv,expecteduv)for a,b in zip(p,q))<1e-7
                assert len(ni)<=4*len(oi) and len(newp)<=n+len(oi),'one-level budget exceeded'
                normals=list(accessor(d,raw,np['attributes']['NORMAL']))
                assert len(normals)==len(newp)
                assert all(abs(sum(c*c for c in normal)-1)<1e-5 for normal in normals)
                vertices+=len(newp);old_vertices+=n;triangles+=len(ni)//3;old_triangles+=len(oi)//3
        records.append(dict(file=row['file'],bytes_before=len(before),bytes_after=path.stat().st_size,vertices_before=old_vertices,vertices_after=vertices,triangles_before=old_triangles,triangles_after=triangles,refined_primitives=refined))
    summary={key:sum(row[key]for row in records)for key in ['bytes_before','bytes_after','vertices_before','vertices_after','triangles_before','triangles_after']}
    summary.update(human_files=len(records),cow_files_byte_identical=8,max_actor_triangles=max(r['triangles_after']for r in records),max_actor_vertices=max(r['vertices_after']for r in records),max_triangle_growth=max(r['triangles_after']/r['triangles_before']for r in records))
    assert len(records)==110
    output=dict(summary=summary,records=records,neutral_geometry='Original vertices and shared-edge midpoints only; exact source-triangle subdivision, winding and UV interpolation verified; all original embedded bytes/material definitions retained.',baseline='96be522')
    (ROOT/'docs/testing/part06-npcs/source-and-cost.json').write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps(summary,indent=2))

if __name__=='__main__':main()
