"""Bake corrected full-body authoring FK to a runtime-only visual clip library."""
import json,pathlib,math
from mathutils import Matrix,Vector,Quaternion
ROOT=pathlib.Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation/context';C=Matrix(((1,0,0),(0,0,1),(0,-1,0)))
base=json.loads((BASE/'poses/2020.json').read_text());names=[]
while len(names)<20:
 for n,b in base['bind'].items():
  if n not in names and (b['parent'] is None or b['parent'] in names):names.append(n)
parents=[names.index(base['bind'][n]['parent']) if base['bind'][n]['parent'] else -1 for n in names]
def frames(p):
 out=[]
 for n in names:
  m=Matrix(p['bone_world'][n]);q=(C@m.to_3x3()).to_quaternion().normalized();t=m.translation;mt=q.to_matrix().to_4x4();mt.translation=Vector((t.x*1.2,t.z*1.2+.03,-t.y*1.2));out.append(mt)
 return [out[i] if p<0 else out[p].inverted()@out[i] for i,p in enumerate(parents)]
rest=frames(base);keys=[]
ids=[2020,2030,2040]+list(range(2220,2223))+list(range(2230,2233))+list(range(2260,2262))+list(range(2270,2272))+list(range(2410,2415))+list(range(2420,2425))+list(range(2360,2365))+list(range(2380,2386))
max_offset=0
for id in ids:
 p=json.loads((BASE/f'poses/{id}.json').read_text());local=frames(p)
 for i in range(1,20):max_offset=max(max_offset,(local[i].translation-rest[i].translation).length)
 keys.append(dict(id=id,held=sum(1<<i for i,n in enumerate(['grip.L','grip.R','peg.L','peg.R']) if n in p['held_contacts']),root=list(local[0].translation),rotations=[list(m.to_quaternion().normalized()) for m in local])) # Blender quaternion is w,x,y,z; reorder below.
for k in keys:k['rotations']=[q[1:]+q[:1] for q in k['rotations']]
# Small input-driven upper-body turn offsets complement the existing whole-bike
# bank animation. They do not apply road curvature or double the chassis roll.
for id,sign in [(2900,-1),(2901,1)]:
 k=json.loads(json.dumps(keys[0]));k['id']=id
 for bone,axis,angle in [('spine',(0,1,0),sign*.07),('head',(0,0,1),-sign*.16)]:
  i=names.index(bone);q=k['rotations'][i];quat=Quaternion((q[3],*q[:3]))@Quaternion(axis,angle);k['rotations'][i]=[quat.x,quat.y,quat.z,quat.w]
 keys.append(k)
clips=[dict(name='normal',keys=[2020],durations=[1.0]),dict(name='tuck',keys=[2030],durations=[1.0]),dict(name='accelerate',keys=[2040],durations=[1.0]),dict(name='turn_left',keys=[2900],durations=[1.0]),dict(name='turn_right',keys=[2901],durations=[1.0])]
for name,ids,ds in [('punch_left',list(range(2220,2223)),[.14,.12,.14]),('punch_right',list(range(2230,2233)),[.14,.12,.14]),('kick_left',[2260,2261],[.24,.24]),('kick_right',[2270,2271],[.24,.24]),('pickup',list(range(2410,2415)),[.32]*5),('pickup_mirrored',list(range(2420,2425)),[.32]*5),('getup',list(range(2360,2365)),[.60,.30,.30,.30,.30]),('run',list(range(2380,2386)),[.12]*6)]:clips.append(dict(name=name,keys=ids,durations=ds))
assert max_offset<.00002,max_offset
profile=json.loads((ROOT/'art/source/contact-profile.json').read_text());limbs=[]
for side in ['L','R']:
 for leg in [False,True]:
  contact=('peg.' if leg else 'grip.')+side;end=('foot.' if leg else 'hand.')+side;offset=Matrix(base['bind'][end]['matrix_local']).inverted()@Vector(profile['landmarks'][contact]['rest_position']);target=Vector(profile['anchors'][contact]['neutral_bike_position'])
  limbs.append(dict(bit=['grip.L','grip.R','peg.L','peg.R'].index(contact),upper=names.index(('thigh.' if leg else 'upper_arm.')+side),lower=names.index(('shin.' if leg else 'forearm.')+side),end=names.index(end),offset=list(offset*1.2),target=[target.x*1.2,target.z*1.2+.03,-target.y*1.2],max_flex=2.45 if leg else 2.35))
out=dict(version=1,limbs=limbs,names=names,parents=parents,offsets=[list(m.translation) for m in rest],render_joints=[names.index(n) for n in ['chest','head','pelvis','upper_arm.R','forearm.R','upper_arm.L','forearm.L','thigh.R','shin.R','thigh.L','shin.L']],keys=keys,clips=clips)
(ROOT/'assets/models/motion.json').write_text(json.dumps(out,separators=(',',':'))+'\n');print('MOTION_EXPORTED',len(keys),'keys',len(clips),'clips; max non-root offset drift',max_offset)
