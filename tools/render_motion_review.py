"""Original full-body animation previews and editable Actions from runtime keys.
Authoring preview, not captured gameplay. Run in Blender 4.5.1.
"""
import bpy,json,pathlib,math,sys
from mathutils import Matrix,Vector,Quaternion
ROOT=pathlib.Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
from animation_rig_math import solve_two_link
DATA=json.loads((ROOT/'assets/models/motion.json').read_text());OUT=ROOT/'art/animation/motion-review';OUT.mkdir(exist_ok=True)
bpy.ops.wm.open_mainfile(filepath=str(pathlib.Path.home()/'Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend'))
rig=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');rider=next(o for o in bpy.data.objects if o.type=='MESH' and any(m.type=='ARMATURE' and m.object==rig for m in o.modifiers));root=bpy.data.objects['Animation_Assembly'];bike=bpy.data.objects['Context_BikePivot'];scene=bpy.context.scene
for o in [rig,root,bike]:o.animation_data_clear()
for a in list(bpy.data.actions):bpy.data.actions.remove(a)
for o in bpy.data.objects:
 if o.name.startswith(('ReviewCow','Cow ')):o.hide_render=True
scene.render.resolution_x=400;scene.render.resolution_y=400;scene.cycles.samples=8;scene.render.image_settings.file_format='PNG'
scene.camera.location=(2.9,5.3,2.4);scene.camera.rotation_euler=(Vector((0,0,.9))-scene.camera.location).to_track_quat('-Z','Y').to_euler();scene.camera.data.ortho_scale=3.2
C=Matrix(((1,0,0),(0,0,1),(0,-1,0)));fps=12
keys={k['id']:k for k in DATA['keys']};clips={c['name']:c for c in DATA['clips']}
def smooth(t):t=max(0,min(1,t));return t*t*(3-2*t)
def pose(id):
 k=keys[id];return {'root':Vector(k['root']),'q':[Quaternion((q[3],*q[:3])) for q in k['rotations']],'held':k['held']}
def blend(a,b,t):
 t=max(0,min(1,t));return {'root':a['root'].lerp(b['root'],t),'q':[x.slerp(y,t) for x,y in zip(a['q'],b['q'])],'held':a['held'] if t<=0 else b['held'] if t>=1 else a['held']&b['held']}
def sample(name,t):
 c=clips[name]
 for i,d in enumerate(c['durations']):
  if t<d or i==len(c['keys'])-1:return blend(pose(c['keys'][i]),pose(c['keys'][min(i+1,len(c['keys'])-1)]),smooth(t/d))
  t-=d

def apply(p):
 root.matrix_basis=Matrix.Identity(4);bike.matrix_basis=Matrix.Identity(4);rig.matrix_basis=Matrix.Identity(4);world=[]
 for i,name in enumerate(DATA['names']):
  local=p['q'][i].to_matrix().to_4x4();local.translation=p['root'] if i==0 else Vector(DATA['offsets'][i]);parent=DATA['parents'][i];m=local if parent<0 else world[parent]@local;world.append(m)
  q=(C.inverted()@m.to_3x3()).to_quaternion();t=m.translation;target=q.to_matrix().to_4x4();target.translation=Vector((t.x/1.2,-t.z/1.2,(t.y-.03)/1.2));rig.pose.bones[name].matrix=target;bpy.context.view_layer.update()
 for chain in DATA['limbs']:
  if not p['held']&(1<<chain['bit']):continue
  names=[DATA['names'][chain[n]] for n in ['upper','lower','end']];end=rig.pose.bones[names[2]];q=end.matrix.to_quaternion();t=chain['target'];target=Vector((t[0]/1.2,-t[2]/1.2,(t[1]-.03)/1.2))-q@(Vector(chain['offset'])/1.2)
  shoulder=rig.pose.bones[names[0]].head;elbow=rig.pose.bones[names[1]].head;wrist=solve_two_link(rig,names[0],names[1],target,elbow,chain['max_flex']);end.matrix=Matrix.Translation(wrist)@q.to_matrix().to_4x4();bpy.context.view_layer.update()
for bone in rig.pose.bones:bone.rotation_mode='QUATERNION'
rider.data.shape_keys.animation_data_clear()
review=[]
for name in ['normal-tuck-normal','left-turn-normal','right-turn-tuck','punch-left-normal','punch-right-tuck','kick-left-normal','kick-right-tuck','pickup-normal','pickup-tuck']:
 base=pose(2030 if name.endswith('tuck') else 2020);action_name=name.split('-')[0];duration=2.0 if 'turn' in name or name=='normal-tuck-normal' else 1.3 if 'pickup' in name else 1.0
 action=bpy.data.actions.new('Motion | '+name);action.use_fake_user=True;rig.animation_data_create();rig.animation_data.action=action;root_action=bpy.data.actions.new('Bank | '+name);root_action.use_fake_user=True;root.animation_data_create();root.animation_data.action=root_action;bike_action=bpy.data.actions.new('Lift | '+name);bike_action.use_fake_user=True;bike.animation_data_create();bike.animation_data.action=bike_action
 fist_action=bpy.data.actions.new('Fist | '+name);fist_action.use_fake_user=True;rider.data.shape_keys.animation_data_create();rider.data.shape_keys.animation_data.action=fist_action
 folder=pathlib.Path.home()/'Downloads/RoadDash_Animation_Interpolation_v1/renders'/name;folder.mkdir(parents=True,exist_ok=True);count=round(duration*fps)+1
 for frame in range(count):
  scene.frame_set(frame+1);t=frame/fps;bank=0;lift=0;p=base
  if name=='normal-tuck-normal':
   amount=smooth((t-.15)/.55) if t<1.0 else 1-smooth((t-1.1)/.55);p=blend(pose(2020),pose(2030),amount)
  elif 'turn' in name:
   amount=smooth(t/.5) if t<1 else 1-smooth((t-1.2)/.5);sign=-1 if name.startswith('left') else 1;bank=-sign*.65*amount
   turn=pose(2900 if sign<0 else 2901);neutral=pose(2020)
   p={'root':base['root'],'q':[q@Quaternion().slerp(n.inverted()@v,amount) for q,n,v in zip(base['q'],neutral['q'],turn['q'])],'held':base['held']}
  elif name.startswith('pickup'):
   fraction=min(1,t/1.1);progress=max(0,min(1,(fraction-.16)/.69));p=sample('pickup',progress*1.6)
   if fraction<.16:p=blend(pose(2364),p,smooth(fraction/.16))
   if fraction>.85:p=blend(p,base,smooth((fraction-.85)/.15))
   angles=[-48,-28,-12,-5,0];f=progress*4;i=min(3,int(f));lift=math.radians(angles[i]+(angles[i+1]-angles[i])*smooth(f-i))
  else:
   side='left' if '-left-' in name else 'right';clip=action_name+'_'+side;length=sum(clips[clip]['durations']);age=max(0,t-.1)
   if age<.10:p=blend(base,sample(clip,0),smooth(age/.10))
   elif age<.10+length:p=sample(clip,age-.10)
   else:p=blend(sample(clip,length),base,smooth((age-.10-length)/.12))
  apply(p);root.rotation_euler.y=bank;bike.rotation_euler.y=lift
  for side in ['L','R']:rider.data.shape_keys.key_blocks['Context_Fist.'+side].value=0
  if name.startswith('punch') and .1<t<.75:rider.data.shape_keys.key_blocks['Context_Fist.'+('L' if '-left-' in name else 'R')].value=1
  for side in ['L','R']:rider.data.shape_keys.key_blocks['Context_Fist.'+side].keyframe_insert(data_path='value',frame=frame+1)
  for b in rig.pose.bones:
   for prop in ['location','rotation_quaternion','scale']:b.keyframe_insert(data_path=prop,frame=frame+1)
  root.keyframe_insert(data_path='rotation_euler',frame=frame+1);bike.keyframe_insert(data_path='rotation_euler',frame=frame+1)
  scene.render.filepath=str(folder/f'{frame:03d}.png');bpy.ops.render.render(write_still=True)
 review.append({'name':name,'fist_action':fist_action.name,'rig_action':action.name,'bank_action':root_action.name,'lift_action':bike_action.name,'frames':count,'fps':fps,'kind':'authoring preview, not native capture'})
 rig.animation_data_clear();root.animation_data_clear();bike.animation_data_clear();rider.data.shape_keys.animation_data_clear();print('ANIMATION_REVIEW',name,count,flush=True)
for a in bpy.data.actions:
 for layer in a.layers:
  for strip in layer.strips:
   for slot in a.slots:
    bag=strip.channelbag(slot)
    if bag:
     for curve in bag.fcurves:
      for key in curve.keyframe_points:key.interpolation='LINEAR'
scene.render.fps=fps;scene.frame_start=1;scene.frame_end=25;scene.frame_set(1);out=pathlib.Path.home()/'Downloads/RoadDash_Animation_Interpolation_v1';out.mkdir(exist_ok=True);bpy.ops.file.pack_all();bpy.ops.wm.save_as_mainfile(filepath=str(out/'RoadDash_Animations.blend'));(OUT/'clips.json').write_text(json.dumps(review,indent=2)+'\n');print('MOTION_REVIEW_COMPLETE',len(review))
