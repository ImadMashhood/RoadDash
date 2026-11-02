#!/usr/bin/env python3
"""Bind original draft actor GLBs to existing physics-body frames; never export physics."""
import hashlib
import json
import math
from pathlib import Path
import struct
import sys
sys.path.insert(0,str(Path(__file__).resolve().parent))
from verify_building_mesh_assets import read_glb, accessor
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/props/mesh_assets'
HUMAN=[(0,1.55,0),(0,2.1,0),(0,1.1,0),(-.45,1.55,0),(-.49,1.14,0),(.45,1.55,0),(.49,1.14,0),(-.17,.73,0),(-.17,.25,0),(.17,.73,0),(.17,.25,0)]
COW=[(0,1,0),(1.25,1.25,0)]+[(x,y,z)for x in [-.7,.7]for z in [-.32,.32]for y in [.55,.15]]

def transformed(v,node,normal=False):
 x,y,z,w=node.get('rotation',[0,0,0,1]);vx,vy,vz=v
 tx=2*(y*vz-z*vy);ty=2*(z*vx-x*vz);tz=2*(x*vy-y*vx)
 q=(vx+w*tx+y*tz-z*ty,vy+w*ty+z*tx-x*tz,vz+w*tz+x*ty-y*tx)
 return q if normal else tuple(q[i]+node.get('translation',[0,0,0])[i]for i in range(3))

def bone(name,p,cow):
 x,y,z=p
 if cow:
  if any(a in name for a in ['head','neck','muzzle','ears','eyes','horn']):return 1
  if name in {'upper_leg','lower_leg','hoof'}:return 2+(4 if x>0 else 0)+(2 if z>0 else 0)+(1 if name!='upper_leg'else 0)
  return 0
 if any(a in name for a in ['head','hair','nose','ear','eyes','hardhat']):return 1
 if name in {'pelvis','belt','skateboard_deck','skate_wheels'}:return 2
 if name in {'upper_arm','shoulder'}:return 3 if x<0 else 5
 if name in {'forearm','elbow','hand','fingers','elbow_protection'}:return 4 if x<0 else 6
 if name=='thigh':return 7 if x<0 else 9
 if name in {'calf','knee','shoe'}:return 8 if x<0 else 10
 if any(a in name for a in ['racket','surfboard','briefcase','flag','diving']):return 4 if x<0 else 6
 return 0

def skin(source,dest,cow):
 d,raw=read_glb(source);data=bytearray(raw);frames=COW if cow else HUMAN
 def add(values,kind,component=5126):
  while len(data)%4:data.append(0)
  start=len(data);code='f'if component==5126 else 'H';width={'VEC3':3,'VEC4':4,'MAT4':16}[kind]
  for v in values:data.extend(struct.pack('<'+code*width,*v))
  view=len(d['bufferViews']);d['bufferViews'].append(dict(buffer=0,byteOffset=start,byteLength=len(data)-start));idx=len(d['accessors'])
  rec=dict(bufferView=view,componentType=component,count=len(values),type=kind)
  if kind=='VEC3':rec.update(min=[min(v[i]for v in values)for i in range(3)],max=[max(v[i]for v in values)for i in range(3)])
  d['accessors'].append(rec);return idx
 for node in d['nodes']:
  if 'mesh'not in node:continue
  name=node.get('extras',{}).get('semantic_part',node.get('name',''))
  for p in d['meshes'][node['mesh']]['primitives']:
   a=p['attributes'];pos=[];normal=[]
   for v in accessor(d,raw,a['POSITION']):
    q=transformed(v,node)
    pos.append((q[2]*1.05,q[1]*1.15,-q[0]*1.05)if cow else tuple(c*1.23 for c in q))
   for v in accessor(d,raw,a['NORMAL']):
    q=transformed(v,node,True);normal.append((q[2],q[1],-q[0])if cow else q)
   a['POSITION']=add(pos,'VEC3');a['NORMAL']=add(normal,'VEC3')
   a['JOINTS_0']=add([(bone(name,v,cow),0,0,0)for v in pos],'VEC4',5123)
   a['WEIGHTS_0']=add([(1.,0.,0.,0.)for _ in pos],'VEC4')
  for key in ['matrix','rotation','translation','scale']:node.pop(key,None)
  node['skin']=0
 start=len(d['nodes']);joints=[]
 for i,frame in enumerate(frames):
  joints.append(start+i);d['nodes'].append(dict(name=f'prop_visual_joint_{i}',translation=list(frame)))
 d['scenes'][0]['nodes'].extend(joints)
 inv=[]
 for x,y,z in frames:inv.append([1,0,0,0,0,1,0,0,0,0,1,0,-x,-y,-z,1])
 d['skins']=[dict(name='Existing_body_frames_visual_only',joints=joints,inverseBindMatrices=add(inv,'MAT4'))]
 d['buffers'][0]['byteLength']=len(data);header=json.dumps(d,separators=(',',':')).encode();header+=b' '*(-len(header)%4);data+=b'\0'*(-len(data)%4)
 total=12+8+len(header)+8+len(data)
 dest.write_bytes(struct.pack('<4sII',b'glTF',2,total)+struct.pack('<II',len(header),0x4e4f534a)+header+struct.pack('<II',len(data),0x004e4942)+data)

def main():
 manifest=json.loads((OUT/'MANIFEST.json').read_text());runtime=OUT/'runtime';runtime.mkdir(exist_ok=True);records=[]
 for r in manifest['records']:
  row=dict(id=r['id'],name=r['name'],group=r['group'],builder=r['builder'],files=[l['file']for l in r['lods']],size=r['collider']['size_y_up'],center=r['collider']['center_y_up'])
  cow=r['builder']=='cow';human=r['builder']=='human'
  if cow or human:
   # Every pedestrian variant is available; source groups and signs never become actors.
   files=[]
   for lod in [1,2]:
    path=runtime/f"{r['id']}_skin_lod{lod}.glb";skin(OUT/r['lods'][lod]['file'],path,cow);files.append(path.relative_to(OUT).as_posix())
   row['skin_files']=files;row['bones']=len(COW if cow else HUMAN)
  records.append(row)
 result=dict(schema_version=1,records=records)
 (ROOT/'content/props.json').write_text(json.dumps(result,separators=(',',':'))+'\n')
 receipt=dict(files=[dict(file=p.relative_to(ROOT).as_posix(),sha256=hashlib.sha256(p.read_bytes()).hexdigest())for p in sorted(runtime.glob('*.glb'))],human_rest=HUMAN,cow_rest=COW,physics_data=False,source_manifest_sha256=hashlib.sha256((OUT/'MANIFEST.json').read_bytes()).hexdigest())
 (ROOT/'docs/testing/props-in-game/runtime-intake.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print('Prepared',len(records),'catalog entries;',len(receipt['files']),'visual-only actor skin GLBs')
if __name__=='__main__':main()
