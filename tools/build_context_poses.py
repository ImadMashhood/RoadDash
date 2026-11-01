"""Context-first authored poses. Blender only; no game/runtime/physics connection.
blender -b --python tools/build_context_poses.py -- --render all
"""
import bpy,json,pathlib,sys,math,argparse
from mathutils import Matrix,Vector,Quaternion
ROOT=pathlib.Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
from animation_rig_math import solve_two_link
parser=argparse.ArgumentParser();parser.add_argument('--render',default='all');args=parser.parse_args(sys.argv[sys.argv.index('--')+1:] if '--' in sys.argv else [])
BASE=ROOT/'art/animation/context';BASE.mkdir(exist_ok=True)
for n in ['poses','gallery']:(BASE/n).mkdir(exist_ok=True)
bpy.ops.wm.open_mainfile(filepath=str(pathlib.Path.home()/'Downloads/RoadDash_Animation_Keyframes_v1/RoadDash_Animation_Keyframes.blend'))
scene=bpy.context.scene;root=bpy.data.objects['Animation_Assembly'];rig=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');rider=next(o for o in bpy.data.objects if o.type=='MESH' and any(m.type=='ARMATURE' and m.object==rig for m in o.modifiers))
for o in bpy.data.objects:
 if o.type=='ARMATURE':o.animation_data_clear()
for action in list(bpy.data.actions):bpy.data.actions.remove(action)
for o in bpy.data.objects:
 if o.name.startswith(('ReviewCow','Cow ')):o.hide_render=True;o.hide_set(True)
# Bike-only assembly transforms are explicit, separate from rider pelvis/root.
bikepivot=bpy.data.objects.new('Context_BikePivot',None);bpy.context.collection.objects.link(bikepivot);bikepivot.parent=root
for o in list(root.children):
 if o not in [rig,bikepivot] and not o.name.startswith(('ReviewCow','Cow ')):o.parent=bikepivot
bike_meshes=[o for o in bpy.data.objects if o.type=='MESH' and o!=rider and not o.name.startswith(('ReviewCow','Cow ')) and o.name!='Authoring stage']
seat=json.loads((ROOT/'art/source/seated-0003.json').read_text());profile=json.loads((ROOT/'art/source/contact-profile.json').read_text());catalog=json.loads((ROOT/'docs/animation/context/catalog.json').read_text());slots=json.loads((ROOT/'docs/animation/reference-slots.json').read_text())['slots']
scene.render.resolution_x=440;scene.render.resolution_y=440;scene.render.resolution_percentage=100;scene.cycles.samples=12;scene.cycles.use_denoising=True;scene.render.image_settings.file_format='JPEG';scene.render.image_settings.quality=93
props=[]
def material(name,color):
 m=bpy.data.materials.new(name);m.diffuse_color=(*color,1);m.use_nodes=True;bs=m.node_tree.nodes.get('Principled BSDF');bs.inputs['Base Color'].default_value=(*color,1);bs.inputs['Metallic'].default_value=.75;bs.inputs['Roughness'].default_value=.3;return m
steel=material('Context original weapon steel',(.18,.20,.22));gripmat=material('Context original weapon grip',(.025,.022,.02));steel.use_fake_user=True;gripmat.use_fake_user=True
def rod(name,a,b,radius,mat):
 bpy.ops.mesh.primitive_cylinder_add(vertices=12,radius=radius,depth=(b-a).length,location=(a+b)*.5);o=bpy.context.object;o.name=name;o.rotation_euler=(b-a).to_track_quat('Z','Y').to_euler();o.data.materials.append(mat);o['context_recipe']=json.dumps({'kind':'cylinder','radius':radius,'depth':(b-a).length});props.append(o);return o
# Shape keys fold the rigid glove digits into a fist in authoring only.
if not rider.data.shape_keys:rider.shape_key_add(name='Basis')
for side in ['L','R']:
 key=rider.shape_key_add(name='Context_Fist.'+side);group=rider.vertex_groups.get('hand.'+side)
 if group:
  inv=rig.data.bones['hand.'+side].matrix_local.inverted()
  for v in rider.data.vertices:
   if any(g.group==group.index and g.weight>.65 for g in v.groups):
    p=inv@v.co
    if p.y>.055:p.z+=.024*min(1,(p.y-.055)/.06);p.y=.055+(p.y-.055)*.18;key.data[v.index].co=rig.data.bones['hand.'+side].matrix_local@p

def update():bpy.context.view_layer.update()
def head(n):return rig.matrix_world@rig.pose.bones[n].head
def orient(n,q,p):rig.pose.bones[n].matrix=rig.matrix_world.inverted()@(Matrix.Translation(p)@q.to_matrix().to_4x4());update()
def rotate(n,axis,deg):
 b=rig.pose.bones[n];rest=rig.data.bones[n].matrix_local.to_quaternion();b.rotation_mode='QUATERNION';b.rotation_quaternion=b.matrix_basis.to_quaternion()@rest.inverted()@Quaternion(Vector(axis),math.radians(deg))@rest;update()
def solve(a,b,target,pole,limit):return solve_two_link(rig,a,b,Vector(target),Vector(pole),limit)
def limb(side,target,pole,leg=False):
 a,b,end=('thigh.','shin.','foot.') if leg else ('upper_arm.','forearm.','hand.')
 target=solve(a+side,b+side,target,pole,2.45 if leg else 2.35)
 q=rig.pose.bones[b+side].matrix.to_quaternion()@rig.data.bones[b+side].matrix_local.to_quaternion().inverted()@rig.data.bones[end+side].matrix_local.to_quaternion()
 orient(end+side,q,target)
def reset(mounted=True):
 root.matrix_basis=Matrix.Identity(4);bikepivot.matrix_basis=Matrix.Identity(4);rig.matrix_basis=Matrix(seat['rig_matrix']) if mounted else Matrix.Identity(4)
 for b in rig.pose.bones:b.matrix_basis=Matrix(seat['bone_basis'][b.name]) if mounted else Matrix.Identity(4)
 for key in rider.data.shape_keys.key_blocks:
  if key.name.startswith('Context_Fist'):key.value=0
 for o in props:bpy.data.objects.remove(o,do_unlink=True)
 props.clear();rider.hide_render=False;rider.hide_set(False)
 for o in bike_meshes:o.hide_render=not mounted;o.hide_set(not mounted)
 update()
def landmark(n):
 d=profile['landmarks'][n];return rig.matrix_world@rig.pose.bones[d['bone']].matrix@rig.data.bones[d['bone']].matrix_local.inverted()@Vector(d['rest_position'])
def hold_grip(side):
 sign=1 if side=='L' else -1;desired=bikepivot.matrix_world@Vector(profile['anchors']['grip.'+side]['neutral_bike_position']);q=bikepivot.matrix_world.to_quaternion()@Matrix(((0,0,-sign),(-sign,0,0),(0,1,0))).to_quaternion();wrist=desired-q@Vector((sign*.052,-.031,0))
 clavicle=rig.pose.bones['clavicle.'+side];initial=clavicle.matrix_basis.copy();rest=rig.data.bones[clavicle.name].matrix_local.to_quaternion();best=None
 length=sum(rig.data.bones[n+side].length for n in ['upper_arm.','forearm.'])-.002
 for angle in [-45,-30,-15,0,15,30,45]:
  clavicle.matrix_basis=initial;clavicle.rotation_mode='QUATERNION';clavicle.rotation_quaternion=initial.to_quaternion()@rest.inverted()@Quaternion((0,0,1),math.radians(angle))@rest;update();error=max(0,(wrist-head('upper_arm.'+side)).length-length)+abs(angle)*1e-6
  if best is None or error<best[0]:best=(error,clavicle.matrix_basis.copy())
 clavicle.matrix_basis=best[1];update()
 wrist=solve('upper_arm.'+side,'forearm.'+side,wrist,(sign*.6,0,1.1),2.35);orient('hand.'+side,q@rig.data.bones['hand.'+side].matrix_local.to_quaternion(),wrist)
def weapon(kind,side,slot):
 p=head('hand.'+side);sgn=1 if side=='L' else -1
 # Directions are authored from the verified rear reference silhouettes.
 if kind=='club':
  direction=Vector((sgn*.95,-.05,.25)) if slot==142 else Vector((-sgn*.65,0,.9)) if slot in [140,141] else Vector((-sgn*.25,-.1,1))
  direction.normalize();rod('Context club grip',p-direction*.04,p+direction*.1,.023,gripmat);rod('Context club shaft',p+direction*.1,p+direction*.55,.019,steel)
 else:
  direction=Vector((sgn,0,.05)) if slot in [83,87] else Vector((-sgn*.6,-.05,.2)) if slot in [81,85,89] else Vector((sgn*.45,-.2,.45))
  end=p+direction.normalized()*.65
  for j in range(17):
   t=j/16;pos=p.lerp(end,t)+Vector((0,.05*math.sin(t*math.pi),-.10*math.sin(t*math.pi)))
   bpy.ops.mesh.primitive_torus_add(major_segments=8,minor_segments=4,major_radius=.026,minor_radius=.006,location=pos);o=bpy.context.object;o.name='Context chain link';o.rotation_euler=(math.pi/2,0,j%2*math.pi/2);o.data.materials.append(steel);o['context_recipe']=json.dumps({'kind':'torus'});props.append(o)

def mounted_pose(a,p):
 code=a['code'];slot=p['rear']['slot'];i=p['phase']-1;held=['seat','grip.L','grip.R','peg.L','peg.R'];released=[]
 if a['selectors'] and a['selectors'][0]['weapon']!='kick' or code in [14,15]:
  kind=a['selectors'][0]['weapon'] if a['selectors'] else 'chain';side='L' if ('_LEFT' in a['id'] or code==15) else 'R';sgn=1 if side=='L' else -1
  if kind=='punch':
   # 176 draw fist back,177 carry across chest,178 extend,179 backhand windup.
   targets={176:(sgn*.25,-.16,1.20),177:(-sgn*.13,-.12,1.32),178:(sgn*.78,-.04,1.20),179:(-sgn*.15,-.05,1.23)};t=targets[slot];rotate('chest',(0,0,1),sgn*(10 if slot in [177,179] else -7));rider.data.shape_keys.key_blocks['Context_Fist.'+side].value=1
  elif kind=='club':
   t={140:(sgn*.18,-.08,1.61),141:(sgn*.28,-.02,1.59),142:(sgn*.75,-.04,1.13),143:(-sgn*.14,-.10,1.34)}[slot];rotate('chest',(0,0,1),sgn*(8 if slot!=142 else -8));rider.data.shape_keys.key_blocks['Context_Fist.'+side].value=1
  else:
   t={81:(-.18,-.12,1.48),82:(sgn*.32,-.04,1.47),83:(sgn*.79,-.03,1.23),84:(sgn*.35,-.02,1.56),85:(-sgn*.12,-.08,1.57),86:(sgn*.29,.02,1.38),87:(sgn*.34,-.02,1.53),88:(-sgn*.10,-.12,1.42),89:(-sgn*.12,-.08,1.57)}[slot];rotate('chest',(0,0,1),sgn*(-8 if slot==83 else 5));rider.data.shape_keys.key_blocks['Context_Fist.'+side].value=1
  rotate('head',(0,0,1),sgn*40)
  limb(side,t,(sgn*.48,.18,1.2));other='R' if side=='L' else 'L';hold_grip(other);held.remove('grip.'+side)
  if kind in ['club','chain']:weapon(kind,side,slot)
 elif code in [26,27]:
  side='L' if code==26 else 'R';sgn=1 if side=='L' else -1;rotate('spine',(1,0,0),17);rotate('chest',(0,1,0),-sgn*9)
  limb(side,(sgn*(.53 if i==0 else .75),.28,.82 if i==0 else .91),(sgn*.5,-.25,1.12),True)
  hold_grip('L');hold_grip('R');held.remove('peg.'+side)
 elif code in [5,6,7]:
  active=['L','R'] if code==5 else ['R']
  for side in active:
   sign=1 if side=='L' else -1;limb(side,(sign*(.35 if i==0 else .48),.08,1.42 if i==0 else 1.65),(sign*.5,.1,1.45));held.remove('grip.'+side)
 elif code in [3,4,9]:
  rotate('spine',(1,0,0),22 if code==3 else -8 if code==4 else 5)
  if code==9:rotate('chest',(0,1,0),-12)
  for side,sign in [('L',1),('R',-1)]:
   if code==3:rotate('clavicle.'+side,(0,0,1),-sign*12)
   hold_grip(side)
 elif code==8:limb('L',(.25,.3,.15),(.35,-.3,.65),True);held.remove('peg.L')
 return held

def standing(a,p):
 code=a['code'];i=p['phase']-1;slot=p['rear']['slot'];rig.location.z=0
 if code==38:
  theta=i*math.tau/6;rotate('spine',(1,0,0),9)
  for side,sgn,off in [('L',1,0),('R',-1,math.pi)]:
   t=theta+off;limb(side,(sgn*.14,-.34*math.cos(t),.16+max(0,-math.sin(t))*.30),(sgn*.18,-.45,.7),True);limb(side,(sgn*.22,.22*math.cos(t),1.21),(sgn*.35,.10-.18*math.cos(t),1.32))
 elif code==37:
  for side,sgn in [('L',1),('R',-1)]:limb(side,(sgn*.35,-.06,1.48 if side=='R' else 1.30),(sgn*.45,.1,1.2))
 elif code==36 and i>0:
  # Supine -> seated hand-to-helmet -> supported half-kneel -> crouch -> stand.
  rig.location.z=[0,-.78,-.48,-.28,0][i];rotate('spine',(1,0,0),[0,15,65,35,0][i])
  if i in [1,2]:rotate('chest',(0,1,0),-18 if i==1 else 16);root.rotation_euler.z=math.radians(-25)
  update()
  for side,sgn in [('L',1),('R',-1)]:
   if i==1:ankle=(sgn*.2,-.48,.15);knee=(sgn*.25,-.8,.55)
   elif i==2 and side=='L':ankle=(sgn*.2,.52,.15);knee=(sgn*.25,-.1,.15)
   else:ankle=(sgn*.18,-.28 if i<4 else -.03,.15);knee=(sgn*.25,-.55,.4)
   limb(side,ankle,knee,True);shoulder=head('upper_arm.'+side)
   target=shoulder+Vector((sgn*.02,-.02,-.50));pole=shoulder+Vector((sgn*.10,.02,-.25))
   if i==1:
    target=Vector((sgn*.42,.15,.12)) if side=='L' else shoulder+Vector((.10,-.10,.10));pole=shoulder+Vector((sgn*.3,.2,-.1))
   elif i==2:
    target=Vector((.42,-.38,.15)) if side=='L' else shoulder+Vector((-.03,.05,-.42));pole=shoulder+Vector((sgn*.2,.05,-.15))
   elif i==3:target=shoulder+Vector((sgn*.05,-.13,-.4))
   limb(side,target,pole)

 else:
  # Pose orientation follows each recovered crash image; loop order comes only
  # from descriptors, never from this raw-slot geometric recipe.
  rotations={241:(75,5),242:(145,15),243:(210,15),244:(90,60),245:(-90,0),246:(-35,0),247:(40,15),256:(-85,0),257:(-120,40),258:(90,0),259:(-80,-50),260:(90,20),261:(130,40),262:(190,20),263:(240,-15),264:(-100,10),265:(-70,45),266:(-120,0),267:(-90,-35),268:(85,0),269:(125,0),270:(190,10),271:(245,0),272:(310,0),273:(150,10),274:(180,0),275:(215,15),276:(255,0),282:(-90,0)}
  x,y=rotations.get(slot,(0,0))
  for side,sgn in [('L',1),('R',-1)]:
   bent=(slot+i+(side=='R'))%2;limb(side,(sgn*.17,-.25 if bent else .12,.53 if bent else .16),(sgn*.25,-.4,.7),True);shoulder=head('upper_arm.'+side);limb(side,shoulder+Vector((sgn*(.35 if slot in [245,267,274,282] else .18),-.18,-.25)),shoulder+Vector((sgn*.4,.15,-.1)))
  root.rotation_euler=(math.radians(x),math.radians(y),0)
 update();deps=bpy.context.evaluated_depsgraph_get();low=min((rider.evaluated_get(deps).matrix_world@Vector(c)).z for c in rider.evaluated_get(deps).bound_box);root.location.z-=low
 if code in [33,34,35]:root.location.z+=.25
 update();return []
def mount(a,p):
 i=p['phase']-1;sgn=-1 if p['rear']['mirror_x'] else 1;rig.location=(sgn*[.6,.5,.3,.13,.04][i],.38,[0,.02,.09,.13,.14][i]);bikepivot.rotation_euler[1]=math.radians(-sgn*[48,28,12,5,0][i]);rotate('spine',(1,0,0),[48,43,40,35,32][i]);update()
 limb('L' if sgn==1 else 'R',(sgn*[.50,.45,.36,.20,.13][i],.35,.15),(sgn*.5,-.25,.7),True)
 limb('R' if sgn==1 else 'L',(sgn*[.95,.85,.50,-.12,-.25][i],.45,[.15,.23,.60,.87,.68][i]),(sgn*.4,-.35,1),True)
 for side in ['L','R']:hold_grip(side)
 return [] # Reach/ground are reviewed, not declared held until endpoint verification.
def capture(id,a,p,held):
 update();world={};basis={};bind={};attachment=0;length=0
 for b in rig.pose.bones:
  rest=rig.data.bones[b.name];world[b.name]=[list(r) for r in rig.matrix_world@b.matrix];basis[b.name]=[list(r) for r in b.matrix_basis];bind[b.name]={'parent':rest.parent.name if rest.parent else None,'head':list(rest.head_local),'tail':list(rest.tail_local),'matrix_local':[list(r) for r in rest.matrix_local]};length=max(length,abs(b.length-rest.length))
  if rest.parent:attachment=max(attachment,(head(b.name)-(rig.matrix_world@rig.pose.bones[rest.parent.name].matrix@rest.parent.matrix_local.inverted()@rest.head_local)).length)
 errors={n:(landmark(n)-bikepivot.matrix_world@Vector(profile['anchors'][n]['neutral_bike_position'])).length for n in held if n in profile['anchors']}
 record=dict(schema_version=1,id=id,frame_id=id,name=p['label'],actor='human',family='mounted' if a['code']<28 else 'locomotion' if a['code']==38 else 'recovery' if a['code']>=36 else 'ragdoll',source_slots=[p['rear']['slot']],interpretation=f"Context-first {a['id']} phase{p['phase']}; source code{a['code']}; source mirror={p['rear']['mirror_x']}; independent authored 3D, not native joint recovery",review='draft',held_contacts=held,released_contacts=[c for c in ['seat','grip.L','grip.R','peg.L','peg.R'] if c not in held] if a['code']<28 else [],contact_errors_m=errors,reference_confidence='executable-descriptor-and-source-selector',root_matrix=[list(r) for r in root.matrix_basis],rig_matrix=[list(r) for r in rig.matrix_basis],bone_basis=basis,bone_world=world,bind=bind,max_attachment_error_m=attachment,max_bone_length_error_m=length,screenshot=f'gallery/{id:04d}.jpg')
 (BASE/f'poses/{id:04d}.json').write_text(json.dumps(record,separators=(',',':'))+'\n');return record
manifest=[];cases=[];assembly={};bike_records=[];render=set(int(x) for x in args.render.split(',')) if args.render not in ['all','none'] else None
for a in catalog['actions']:
 if a['bank']!='rider' or not a['phases'] or a['code']==43:continue
 ids=[]
 for p in a['phases']:
  code=a['code'];id=2000+code*10+p['phase']-1;reset(code<28);bikeonly=code>=44;mounted=code<28 or code in [39,40,41,42]
  if bikeonly:
   rider.hide_render=True
   for o in bike_meshes:o.hide_render=False;o.hide_set(False)
   slot=p['rear']['slot'];root.rotation_euler=(0,math.radians(-85 if p['rear']['mirror_x'] else 85),math.radians({319:90,320:50,321:0,322:-45}[slot]));update()
   deps=bpy.context.evaluated_depsgraph_get();low=min((o.evaluated_get(deps).matrix_world@Vector(c)).z for o in bike_meshes for c in o.evaluated_get(deps).bound_box);root.location.z-=low;update();bike_records.append({'id':id,'label':p['label'],'code':code,'phase':p['phase'],'rear':p['rear'],'root_matrix':[list(r) for r in root.matrix_basis],'screenshot':f'gallery/{id:04d}.jpg','review':'draft'})
  else:
   if code<28:held=mounted_pose(a,p)
   elif code in [39,40,41,42]:
    for o in bike_meshes:o.hide_render=False;o.hide_set(False)
    held=mount(a,p)
   else:held=standing(a,p)
   record=capture(id,a,p,held);manifest.append({k:record[k] for k in ['id','name','actor','family','source_slots','review','held_contacts','screenshot']});ids.append(id)
   action=bpy.data.actions.new(p['label']);action.use_fake_user=True;rig.animation_data_create();rig.animation_data.action=action
   for bone in rig.pose.bones:
    for prop in ['location','rotation_quaternion','scale']:bone.keyframe_insert(data_path=prop,frame=1)
   for layer in action.layers:
    for strip in layer.strips:
     bag=strip.channelbag(action.slots[0])
     if bag:
      for curve in bag.fcurves:
       for point in curve.keyframe_points:point.interpolation='CONSTANT'
   action.asset_mark();rig.animation_data_clear()
  assembly[str(id)]={'action':a['id'],'code':code,'phase':p['phase'],'label':p['label'],'source':p['rear'],'duration_ticks':p['duration_ticks'],'control':p['control'],'bike_matrix':[list(r) for r in bikepivot.matrix_basis],'props':[{'name':o.name,'matrix':[list(r) for r in o.matrix_world],'recipe':json.loads(o['context_recipe']),'material':o.data.materials[0].name} for o in props],'fist_values':{side:rider.data.shape_keys.key_blocks['Context_Fist.'+side].value for side in ['L','R']}}
  if args.render!='none' and (render is None or id in render):
   camera=scene.camera;camera.location=(0,6,2.1);camera.rotation_euler=(Vector((0,0,.9))-camera.location).to_track_quat('-Z','Y').to_euler();camera.data.ortho_scale=3.2
   scene.render.filepath=str(BASE/f'gallery/{id:04d}.jpg');bpy.ops.render.render(write_still=True)
  print('CONTEXT_POSE',id,p['label'],flush=True)
 if ids:cases.append(dict(id=a['id'],family='source-action',actor='human',state=a['id'],pose_ids=ids,entry_pose=ids[0],exit_pose=ids[-1],phase_labels=[p['label'] for p in a['phases']],interrupts=['physics-impact'],timing_basis='unassigned',notes=a['context']+'; native logical timing/control in assembly.json, no Bevy clip timing assigned',review='draft'))
(BASE/'manifest.json').write_text(json.dumps(dict(schema_version=1,pose_kind='static-keyframes',runtime_connected=False,physics_changed=False,poses=manifest,use_cases=cases),indent=2)+'\n');(BASE/'assembly.json').write_text(json.dumps(assembly,indent=2)+'\n');(BASE/'bike-poses.json').write_text(json.dumps(bike_records,indent=2)+'\n')
# Editable pose actions are one-frame assets; JSON/assembly is canonical for props and bike transforms.
reset();scene.frame_start=1;scene.frame_end=1
if args.render=='all':
 rider.hide_render=True
 for o in bike_meshes:o.hide_render=True
 scene.render.filepath=str(BASE/'gallery/background.jpg');bpy.ops.render.render(write_still=True);reset()
bpy.ops.file.pack_all();out=pathlib.Path.home()/'Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend';bpy.ops.wm.save_as_mainfile(filepath=str(out));print('CONTEXT_COMPLETE',len(manifest),'human',len(bike_records),'bike',flush=True)
