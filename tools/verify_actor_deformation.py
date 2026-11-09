#!/usr/bin/env python3
"""Decoded-GLB attachment stress test. Synthetic poses, not gameplay proof.

Uses point-to-triangle distances, not allocation counts or weight sums. Numpy
is an offline measurement dependency only (the game/intake do not import it).
"""
import argparse
import hashlib
import json
from pathlib import Path
import numpy as np
from verify_building_mesh_assets import read_glb, accessor

ROOT = Path(__file__).resolve().parents[1]
FRAMES = np.array([(0,1.55,0),(0,2.1,0),(0,1.1,0),(-.45,1.55,0),(-.49,1.14,0),(.45,1.55,0),(.49,1.14,0),(-.17,.73,0),(-.17,.25,0),(.17,.73,0),(.17,.25,0)])
PARENTS = [None,0,0,0,3,0,5,2,7,2,9]
ANCHORS = np.array([(0,0,0),(0,1.87,0),(0,1.26,0),(-.29,1.69,0),(-.47,1.34,0),(.29,1.69,0),(.47,1.34,0),(-.17,.96,0),(-.17,.49,0),(.17,.96,0),(.17,.49,0)])
PAIRS = [('torso','shoulder'),('shoulder','upper_arm'),('upper_arm','elbow'),('elbow','forearm'),('forearm','hand'),('torso','pelvis'),('pelvis','thigh'),('thigh','knee'),('knee','calf'),('torso','neck'),('neck','head'),('torso','vest_reflective_tape'),('head','hair_cap'),('head','hardhat')]


def distances(points, triangles):
    """Exact unsigned point/triangle distances, including edges and corners."""
    a,b,c = triangles[:,0],triangles[:,1],triangles[:,2]
    ab=b-a;ac=c-a;n=np.cross(ab,ac);nn=np.sum(n*n,axis=1)
    result=[]
    for p in points:
        ap=p-a; signed=np.sum(ap*n,axis=1)
        projected=p-n*(signed/np.maximum(nn,1e-20))[:,None]
        v=projected-a
        aa=np.sum(ab*ab,axis=1);bb=np.sum(ab*ac,axis=1);cc=np.sum(ac*ac,axis=1)
        av=np.sum(ab*v,axis=1);cv=np.sum(ac*v,axis=1);den=np.maximum(aa*cc-bb*bb,1e-20)
        u=(cc*av-bb*cv)/den;w=(aa*cv-bb*av)/den
        ds=np.where((u>=0)&(w>=0)&(u+w<=1)&(nn>1e-20),signed*signed/np.maximum(nn,1e-20),np.inf)
        for start,end in [(a,b),(b,c),(c,a)]:
            edge=end-start;t=np.clip(np.sum((p-start)*edge,axis=1)/np.maximum(np.sum(edge*edge,axis=1),1e-20),0,1)
            ds=np.minimum(ds,np.sum((p-start-t[:,None]*edge)**2,axis=1))
        result.append(float(np.sqrt(np.min(ds))))
    return np.array(result)


def pose(angles):
    matrices=[]
    for i,parent in enumerate(PARENTS):
        angle=angles.get(i,0.)
        x,y,z=angle if isinstance(angle,tuple)else (angle,0.,0.)
        cx,sx=np.cos(x),np.sin(x);cy,sy=np.cos(y),np.sin(y);cz,sz=np.cos(z),np.sin(z)
        rx=np.array([[1,0,0],[0,cx,-sx],[0,sx,cx]]);ry=np.array([[cy,0,sy],[0,1,0],[-sy,0,cy]]);rz=np.array([[cz,-sz,0],[sz,cz,0],[0,0,1]])
        r=np.eye(4);r[:3,:3]=rz@ry@rx
        r[:3,3]=ANCHORS[i]-r[:3,:3]@ANCHORS[i]
        matrices.append(r if parent is None else matrices[parent]@r)
    return np.array(matrices)


def decoded(path):
    d,raw=read_glb(path);groups={}
    for node in d['nodes']:
        if 'mesh' not in node:continue
        name=node.get('extras',{}).get('semantic_part',node.get('name',''))
        for prim in d['meshes'][node['mesh']]['primitives']:
            a=prim['attributes'];p=np.array(list(accessor(d,raw,a['POSITION'])));j=np.array(list(accessor(d,raw,a['JOINTS_0'])),dtype=int);w=np.array(list(accessor(d,raw,a['WEIGHTS_0'])))
            indices=np.array(list(accessor(d,raw,prim['indices'])),dtype=int).reshape(-1,3)
            groups.setdefault(name,[]).append((p,j,w,indices))
    return groups


def deform(group,matrices,rigid=None):
    positions=[];triangles=[]
    for p,j,w,idx in group:
        if rigid is not None:
            # Historical primitive-wide bone: decoded field's semantic owner.
            # Assigned by caller, independently of the continuous field.
            q=(np.c_[p,np.ones(len(p))]@matrices[rigid].T)[:,:3]
        else:
            q=np.zeros_like(p)
            for k in range(4):q+=np.einsum('nij,nj->ni',matrices[j[:,k]],np.c_[p,np.ones(len(p))])[:,:3]*w[:,k,None]
        positions.extend(q);triangles.extend(q[idx])
    return np.array(positions),np.array(triangles)


def split_groups(groups,side):
    selected={}
    for name,parts in groups.items():
        selected[name]=[]
        for p,j,w,idx in parts:
            idx=idx[np.mean(p[idx][:,:,0],axis=1)*side>0]
            if not len(idx):continue
            used=np.unique(idx);remap=np.full(len(p),-1,dtype=int);remap[used]=np.arange(len(used))
            selected[name].append((p[used],j[used],w[used],remap[idx]))
    return selected


def owners_for(side):
    return {'torso':0,'neck':0,'head':1,'pelvis':2,'upper_arm':3 if side<0 else 5,'forearm':4 if side<0 else 6,'thigh':7 if side<0 else 9,'calf':8 if side<0 else 10,'hand':4 if side<0 else 6,'fingers':4 if side<0 else 6,'shoulder':3 if side<0 else 5,'elbow':4 if side<0 else 6,'knee':8 if side<0 else 10,'vest_reflective_tape':0,'hair_cap':1,'hardhat':1}


def native_receipt(path,output):
    records=[];geometry={};samples={};ident=np.array([np.eye(4)]*11)
    for line in path.read_text().splitlines():
        frame=json.loads(line)
        for actor in frame['actors']:
            file=actor['file']
            if file not in geometry:geometry[file]=decoded(ROOT/'art/props/mesh_assets'/file)
            matrices=np.array(actor['driver_skin_matrices_column_major']).reshape(11,4,4).transpose(0,2,1)
            for side in [-1,1]:
                selected=split_groups(geometry[file],side);owners=owners_for(side)
                for left,right in PAIRS:
                    if not selected.get(left) or not selected.get(right):continue
                    key=(file,side,left,right)
                    if key not in samples:
                        p,_=deform(selected[left],ident);_,tri=deform(selected[right],ident)
                        rest=distances(p,tri);ids=np.where(rest<.035)[0]
                        samples[key]=ids[np.argsort(rest[ids])[:24]]
                    ids=samples[key]
                    if not len(ids):continue
                    weighted,_=deform(selected[left],matrices);_,tt=deform(selected[right],matrices)
                    rigid,_=deform(selected[left],matrices,owners[left]);_,rt=deform(selected[right],matrices,owners[right])
                    before=distances(rigid[ids],rt);after=distances(weighted[ids],tt)
                    records.append(dict(frame=frame['frame'],phase=frame['phase'],family=actor['family'],file=file,side=side,attachment=f'{left}/{right}',samples=len(ids),rigid_mean_m=float(np.mean(before)),weighted_mean_m=float(np.mean(after)),rigid_max_m=float(np.max(before)),weighted_max_m=float(np.max(after))))
    summary={}
    for phase in ['walk','impact','ragdoll']:
        rows=[r for r in records if r['phase']==phase]
        assert rows,phase
        summary[phase]=dict(cases=len(rows),rigid_mean_m=float(np.mean([r['rigid_mean_m']for r in rows])),weighted_mean_m=float(np.mean([r['weighted_mean_m']for r in rows])),max_m=max(r['weighted_max_m']for r in rows),worsened_mean_over_1mm=sum(r['weighted_mean_m']>r['rigid_mean_m']+.001 for r in rows))
    result=dict(method='Actual native final propagated driver matrices; same source-overlap sample/nearest-triangle method as synthetic audit. Rigid control uses equivalent affine old primitive surfaces on the same neutral geometry. Not full race/pose coverage.',source=str(path),summary=summary,records=records)
    output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(summary,indent=2))

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=Path);parser.add_argument('--native',type=Path);args=parser.parse_args()
    if args.native:
        native_receipt(args.native,args.output);return
    catalog=json.loads((ROOT/'content/props.json').read_text())
    records=[];cache={};ident=np.array([np.eye(4)]*11)
    # Split sides before measuring so opposite limbs cannot mask a gap.
    for row in catalog['records']:
        if row['builder']!='human':continue
        for file in row['skin_files']:
            groups=decoded(ROOT/'art/props/mesh_assets'/file)
            for side in [-1,1]:
                selected=split_groups(groups,side);owners=owners_for(side)
                for left,right in PAIRS:
                    if not selected.get(left) or not selected.get(right):continue
                    key=hashlib.sha256(b''.join(a.tobytes() for name in [left,right] for part in selected[name] for a in part)).digest()
                    key=(key,owners[left],owners[right])
                    if key in cache:
                        for item in cache[key]:records.append(dict(item,id=row['id'],file=file,side=side,attachment=f'{left}/{right}'))
                        continue
                    start=len(records)
                    p,_=deform(selected[left],ident);_,tri=deform(selected[right],ident)
                    rest=distances(p,tri);ids=np.where(rest<.035)[0]
                    if not len(ids):continue
                    ids=ids[np.argsort(rest[ids])[:24]]
                    for label,angles in [('compound',{0:(.2,.15,.1),1:(.2,-.3,.1),2:(-.2,.1,.1),3:(.5,.35,-.45),4:1.3,5:(-.3,-.35,.45),6:1.,7:(-.8,.15,-.25),8:1.5,9:(.4,-.15,.25),10:1.}),('rest',{}),('walk',{3:.45,4:.45,5:-.45,6:-.45,7:-.45,8:-.45,9:.45,10:.45}),('impact',{0:.2,1:.5,2:-.3,3:1.1,4:1.3,5:-.8,6:1.2,7:-1.,8:1.5,9:.8,10:1.5}),('ragdoll',{0:1.,1:-.6,2:-.4,3:-1.2,4:1.5,5:.9,6:-1.,7:1.1,8:1.6,9:-.8,10:1.4})]:
                        matrices=pose(angles)
                        weighted,t=deform(selected[left],matrices);_,tt=deform(selected[right],matrices)
                        legacy,_=deform(selected[left],matrices,owners[left]);_,lt=deform(selected[right],matrices,owners[right])
                        before=distances(legacy[ids],lt);after=distances(weighted[ids],tt)
                        records.append(dict(id=row['id'],file=file,side=side,attachment=f'{left}/{right}',pose=label,samples=len(ids),rigid_mean_m=float(np.mean(before)),rigid_max_m=float(np.max(before)),weighted_mean_m=float(np.mean(after)),weighted_max_m=float(np.max(after))))
                    cache[key]=records[start:]
    result=dict(method='Synthetic hierarchical driver rotations around unchanged physical anchors; nearest actual decoded triangle distances for up to24 source-overlap vertices per side/attachment. Not native gameplay poses or art approval.',records=records,summary={})
    for label in ['rest','walk','impact','ragdoll','compound']:
        rows=[r for r in records if r['pose']==label]
        result['summary'][label]=dict(cases=len(rows),rigid_mean_m=float(np.mean([r['rigid_mean_m']for r in rows])),weighted_mean_m=float(np.mean([r['weighted_mean_m']for r in rows])),weighted_max_m=max(r['weighted_max_m']for r in rows))
        if label!='rest':assert result['summary'][label]['weighted_mean_m']<result['summary'][label]['rigid_mean_m'],label
    assert len({r['id']for r in records})==55
    if args.output:args.output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result['summary'],indent=2))

if __name__=='__main__':main()
