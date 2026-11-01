"""Blender 4.5: original static pose assets, JSON and review renders.

blender --background --python tools/build_animation_library.py -- --render 3,1010,1080,1200
No physics or runtime animation is changed. Source art is project-authored.
"""
import argparse,json,math,pathlib,sys
import bpy
from mathutils import Matrix,Vector,Quaternion
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'art/animation'
args=argparse.ArgumentParser();args.add_argument('--render',default='none');args.add_argument('--blend',default=str(pathlib.Path.home()/'Downloads/RoadDash_Animation_Keyframes_v1/RoadDash_Animation_Keyframes.blend'))
opts=args.parse_args(sys.argv[sys.argv.index('--')+1:] if '--' in sys.argv else [])
for folder in ['poses','gallery']:(OUT/folder).mkdir(parents=True,exist_ok=True)
legacy_source=json.loads((OUT/'legacy-mounted.json').read_text())
plan=json.loads((OUT/'pose-plan.json').read_text());profile=json.loads((ROOT/'art/source/contact-profile.json').read_text())
bpy.ops.wm.read_factory_settings(use_empty=True)
bpy.ops.import_scene.gltf(filepath=str(ROOT/'art/source/rider.glb'))
rig=next(o for o in bpy.context.scene.objects if o.type=='ARMATURE');rider=next(o for o in bpy.context.scene.objects if o.type=='MESH')
bpy.ops.import_scene.gltf(filepath=str(ROOT/'art/source/motorcycle.glb'))
bike_objects=[o for o in bpy.context.scene.objects if o!=rider and o.type=='MESH' and not any(c.name=='glTF_not_exported' for c in o.users_collection)]
asset_objects=[o for o in bpy.context.scene.objects if not any(c.name=='glTF_not_exported' for c in o.users_collection)]
root=bpy.data.objects.new('Animation_Assembly',None);bpy.context.collection.objects.link(root)
for o in asset_objects:
    if not o.parent:o.parent=root
human_rig=rig

def update():bpy.context.view_layer.update()
def reset():
    root.matrix_basis=Matrix.Identity(4);rig.matrix_basis=Matrix.Identity(4)
    for b in rig.pose.bones:b.matrix_basis=Matrix.Identity(4)
    update()
def head(name):return rig.matrix_world@rig.pose.bones[name].head
def orient(name,q,pos):
    rig.pose.bones[name].matrix=rig.matrix_world.inverted()@(Matrix.Translation(pos)@q.to_matrix().to_4x4());update()
def direction(name,a,b):
    rest=rig.data.bones[name]
    q=(rest.tail_local-rest.head_local).rotation_difference(b-a)@rest.matrix_local.to_quaternion()
    orient(name,q,a)
def solve(a_name,b_name,target,pole,max_flex):
    a=head(a_name);l1=rig.data.bones[a_name].length;l2=rig.data.bones[b_name].length
    delta=target-a
    minimum=math.sqrt(l1*l1+l2*l2+2*l1*l2*math.cos(max_flex))+.001
    d=max(minimum,min(delta.length,l1+l2-.002));axis=delta.normalized()
    target=a+axis*d;along=(l1*l1-l2*l2+d*d)/(2*d)
    perp=pole-a-axis*(pole-a).dot(axis)
    if perp.length<1e-5:perp=axis.cross(Vector((1,0,0)))
    perp.normalize();joint=a+axis*along+perp*math.sqrt(max(0,l1*l1-along*along))
    direction(a_name,a,joint);direction(b_name,joint,target)
    return target

def rotate(name,axis,degrees):
    rest=rig.data.bones[name].matrix_local.to_quaternion()
    pb=rig.pose.bones[name];pb.rotation_mode='QUATERNION';pb.rotation_quaternion=rest.inverted()@Quaternion(Vector(axis),math.radians(degrees))@rest;update()
def landmark(name):
    d=profile['landmarks'][name];bone=d['bone']
    return rig.matrix_world@rig.pose.bones[bone].matrix@rig.data.bones[bone].matrix_local.inverted()@Vector(d['rest_position'])

def human_ground(spec):
    p=spec['params'];phase=p.get('phase',0);stride=p.get('stride',0);gait=p.get('gait','walk');theta=phase*2*math.pi
    running=gait in ['run','sprint']
    if running and math.cos(theta*2)<-.4:p['air']=True
    crouch=min(.30,.035+abs(stride)*.30+p.get('crouch',0));rig.location.z=-crouch;update()
    rotate('spine',(1,0,0),p.get('torso',8 if gait in ['run','sprint'] else 0));rotate('chest',(0,0,1),p.get('turn',0)*.35)
    for side,sign,offset in [('L',1,0),('R',-1,math.pi)]:
        swing=math.sin(theta+offset);y=-stride*math.cos(theta+offset)
        lift=max(0,-swing)*(.22 if gait in ['run','sprint'] else .11)
        if p.get('step')==side:lift+=.17;y=-.18
        z=.14+lift-math.tan(math.radians(p.get('slope',0)))*y+sign*.14*math.tan(math.radians(p.get('cross_slope',0)))
        target=Vector((sign*.14+p.get('strafe',0),y,z))
        ankle=solve('thigh.'+side,'shin.'+side,target,Vector((sign*.2,-.45,.65)),2.45)
        orient('foot.'+side,rig.data.bones['foot.'+side].matrix_local.to_quaternion(),ankle)
        shoulder=head('upper_arm.'+side);arms=p.get('arms','relaxed')
        delta=Vector((sign*.055,stride*.8*math.cos(theta+offset),-.43))
        if gait in ['run','sprint']:delta=Vector((sign*.07,stride*.65*math.cos(theta+offset),-.27))
        if arms=='brace':delta=Vector((sign*.12,-.32,-.09))
        if arms=='protect':delta=Vector((sign*.01,-.20,.15))
        if arms=='reach-'+('left' if side=='L' else 'right'):delta=Vector((sign*.41,-.14,.02))
        wrist=solve('upper_arm.'+side,'forearm.'+side,shoulder+delta,shoulder+Vector((sign*.45,.1,-.1)),2.35)
        # Wrist follows the forearm: no arbitrary disconnected hand translation.
        q=rig.pose.bones['forearm.'+side].matrix.to_quaternion()
        rest=rig.data.bones['forearm.'+side].matrix_local.to_quaternion().inverted()@rig.data.bones['hand.'+side].matrix_local.to_quaternion()
        orient('hand.'+side,q@rest,wrist)
    root.rotation_euler=(math.radians(p.get('lying',0)),math.radians(p.get('roll',0)),math.radians(p.get('turn',0)))
    update()
    held=[]
    if not p.get('lying') and not p.get('air'):
        held=['ground.'+('L' if math.cos(theta)>=0 else 'R')] if running else ['ground.'+side for side,offset in [('L',0),('R',math.pi)] if math.sin(theta+offset)>=-1e-4]
    return held

def human_seated(spec):
    base=json.loads((ROOT/'art/source/seated-0003.json').read_text());rig.matrix_basis=Matrix(base['rig_matrix'])
    for name,rows in base['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(rows)
    update();p=spec['params'];held=['seat','grip.L','grip.R','peg.L','peg.R']
    if p.get('torso'):
        pb=rig.pose.bones['spine'];pb.rotation_mode='QUATERNION';pb.rotation_quaternion=pb.matrix_basis.to_quaternion()@Quaternion((1,0,0),math.radians(p['torso']));update()
        for side,sign in [('L',1),('R',-1)]:
            # Preserve the source fitted palm surface with a two-bone solve.
            if p['torso']<0:rotate('clavicle.'+side,(0,0,1),-sign*20)
            desired=Vector(profile['anchors']['grip.'+side]['neutral_bike_position'])
            hand_q=Matrix(((0,0,-sign),(-sign,0,0),(0,1,0))).to_quaternion()
            wrist=desired-hand_q@Vector((sign*.052,-.031,0))
            wrist=solve('upper_arm.'+side,'forearm.'+side,wrist,Vector((sign*.53,.02,1.15)),2.35)
            orient('hand.'+side,hand_q@rig.data.bones['hand.'+side].matrix_local.to_quaternion(),wrist)
    if p.get('release')=='both':
        held=[];rig.location.y-=.15;rig.location.z+=.15
    elif p.get('release') in held:held.remove(p['release'])
    root.rotation_euler=(math.radians(p.get('pitch',0)),math.radians(p.get('bank',0)),0);update()
    return held

# Original quadruped authoring proxy, with separate anatomy and named joints.
cow_data=bpy.data.armatures.new('Cow_Review_Skeleton');cow=bpy.data.objects.new('Cow_Review_Rig',cow_data);bpy.context.collection.objects.link(cow);cow.parent=root
bpy.context.view_layer.objects.active=cow;cow.select_set(True);bpy.ops.object.mode_set(mode='EDIT')
specs=[('body',(0,0,1.0),(0,-.8,1.0),None,(.9,2.1,1.0)),('head',(0,-1.25,1.25),(0,-1.7,1.25),'body',(.65,.7,.65))]
for end,y in [('front',-.7),('hind',.7)]:
    for side,sign in [('L',1),('R',-1)]:
        n=end+'.'+side;specs.extend([(n,(sign*.32,y,.55),(sign*.32,y,.3),'body',(.22,.22,.65)),(n+'.lower',(sign*.32,y,.15),(sign*.32,y,.02),n,(.2,.2,.4))])
for name,h,t,parent,size in specs:
    b=cow_data.edit_bones.new(name);b.head=h;b.tail=t
    if parent:b.parent=cow_data.edit_bones[parent]
bpy.ops.object.mode_set(mode='OBJECT');cow.select_set(False)
cow_objects=[]
white=bpy.data.materials.new('Original cow chalk');white.diffuse_color=(.76,.75,.68,1)
for name,h,t,parent,size in specs:
    bpy.ops.mesh.primitive_cube_add(size=1,location=h);o=bpy.context.object;o.name='ReviewCow_'+name;o.scale=size;bpy.ops.object.transform_apply(location=False,rotation=False,scale=True)
    o.data.materials.append(white);group=o.vertex_groups.new(name=name);group.add(list(range(len(o.data.vertices))),1,'REPLACE');modifier=o.modifiers.new('Cow skin','ARMATURE');modifier.object=cow;o.parent=cow
    cow_objects.append(o)
# Simple original markings make the quadruped review proxy readable as a cow.
black=bpy.data.materials.new('Original cow charcoal');black.diffuse_color=(.035,.028,.025,1)
def cow_detail(name,position,size,bone,material):
    bpy.ops.mesh.primitive_cube_add(size=1,location=position);o=bpy.context.object;o.name=name;o.scale=size;bpy.ops.object.transform_apply(location=False,rotation=False,scale=True)
    o.data.materials.append(material);g=o.vertex_groups.new(name=bone);g.add(list(range(len(o.data.vertices))),1,'REPLACE');m=o.modifiers.new('Cow skin','ARMATURE');m.object=cow;o.parent=cow;cow_objects.append(o)
cow_detail('Cow muzzle',(0,-1.57,1.12),(.55,.32,.28),'head',black)
for sign in [-1,1]:
    cow_detail('Cow ear',(sign*.42,-1.15,1.4),(.28,.22,.1),'head',white)
    cow_detail('Cow eye',(sign*.33,-1.46,1.36),(.045,.09,.07),'head',black)
    cow_detail('Cow marking',(sign*.456,.25,1.12),(.012,.67,.49),'body',black)
for name,h,t,parent,size in specs:
    if name.endswith('.lower'):cow_detail('Cow hoof',(h[0],h[1],-.005),(.23,.29,.12),name,black)
for o in cow_objects:
    bevel=o.modifiers.new('Small bevel','BEVEL');bevel.width=.035;bevel.segments=2
    normals=o.modifiers.new('Face normals','WEIGHTED_NORMAL')

def cow_pose(spec):
    p=spec['params'];action=p['action'];phase=p['phase']*math.pi*2
    if action=='graze':rotate('head',(1,0,0),-35)
    if action=='startle':rotate('head',(1,0,0),20)
    for end in ['front','hind']:
        for side,sign in [('L',1),('R',-1)]:
            value=math.sin(phase+(math.pi if side=='R' else 0)+(math.pi if end=='hind' else 0))*25 if 'walk' in action or 'flight' in action else 0
            rotate(end+'.'+side,(1,0,0),value);rotate(end+'.'+side+'.lower',(1,0,0),abs(value)*(.65 if end=='front' else -.65))
    if action=='impact-brace':rotate('body',(1,0,0),8)
    if action=='fallen-side':root.rotation_euler[1]=math.radians(85)
    if action=='righting-attempt':root.rotation_euler[1]=math.radians(45)
    update();return [] if action in ['fallen-side','righting-attempt','run-flight'] else ['ground.'+n for n in ['front.L','front.R','hind.L','hind.R']]

scene=bpy.context.scene;scene.render.engine='CYCLES';scene.cycles.samples=8;scene.cycles.use_denoising=True
scene.render.resolution_x=560;scene.render.resolution_y=560;scene.render.resolution_percentage=100
scene.world=bpy.data.worlds.new('Pose review world');scene.world.color=(.18,.18,.18);scene.view_settings.view_transform='AgX'
bpy.ops.mesh.primitive_plane_add(size=200,location=(0,0,-.008));floor=bpy.context.object;floor.name='Authoring stage'
mat=bpy.data.materials.new('Studio');mat.diffuse_color=(.035,.045,.06,1);floor.data.materials.append(mat)
for pos,power in [((-3,-4,5),800),((3,1,4),700),((1,4,3),500)]:
    light=bpy.data.objects.new('Pose light',bpy.data.lights.new('Area','AREA'));bpy.context.collection.objects.link(light);light.data.energy=power;light.data.shape='DISK';light.data.size=4;light.location=pos;light.rotation_euler=(Vector((0,0,1))-light.location).to_track_quat('-Z','Y').to_euler()
bpy.ops.object.camera_add();camera=bpy.context.object;camera.data.type='ORTHO';camera.data.ortho_scale=3.1;scene.camera=camera
render_ids=None if opts.render=='all' else set(int(v) for v in opts.render.split(',')) if opts.render!='none' else set()
manifest=[];errors=[]
for spec in plan['poses']:
    rig=cow if spec['actor']=='cow' else human_rig;reset()
    mounted=spec['mode'] in ['legacy','seated']
    for o in bike_objects:o.hide_render=not mounted
    rider.hide_render=spec['actor']=='cow'
    for o in cow_objects:o.hide_render=spec['actor']!='cow'
    floor.rotation_euler=(0,0,0)
    if spec['mode']=='legacy':
        old=legacy_source[str(spec['id'])]
        root.matrix_basis=Matrix(old['root_matrix']);rig.matrix_basis=Matrix(old['rig_matrix'])
        for name,rows in old['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(rows)
        held=old['held_contacts'];update()
        if spec['id'] in [309,310,311]:
            # The inherited mount candidates over-flexed the released right
            # knee. Refit its ankle within the same anatomical flex budget.
            target=head('foot.R');pole=Vector((-.3,-.25,.65))
            target=solve('thigh.R','shin.R',target,pole,2.45)
            orient('foot.R',rig.data.bones['foot.R'].matrix_local.to_quaternion(),target)
    elif spec['mode']=='seated':held=human_seated(spec)
    elif spec['mode']=='cow':held=cow_pose(spec)
    else:held=human_ground(spec)
    update()
    # Place released/lying layouts on the stage; never stretch limbs to do so.
    if not mounted:
        deps=bpy.context.evaluated_depsgraph_get();objects=cow_objects if spec['actor']=='cow' else [rider]
        lowest=min((o.evaluated_get(deps).matrix_world@Vector(c)).z for o in objects for c in o.evaluated_get(deps).bound_box)
        if spec['params'].get('air'):root.location.z+=({'run':.12,'sprint':.18}.get(spec['params'].get('gait'),.3))-lowest
        else:root.location.z-=lowest
        update()
    attachment=0.0;length_error=0.0;world={};basis={};bind={}
    for b in rig.pose.bones:
        rest=rig.data.bones[b.name]
        world[b.name]=[list(row) for row in rig.matrix_world@b.matrix]
        basis[b.name]=[list(row) for row in b.matrix_basis]
        bind[b.name]=dict(parent=rest.parent.name if rest.parent else None,head=list(rest.head_local),tail=list(rest.tail_local),matrix_local=[list(row) for row in rest.matrix_local])
        length_error=max(length_error,abs(b.length-rest.length))
        if rest.parent:
            expected=rig.matrix_world@rig.pose.bones[rest.parent.name].matrix@rest.parent.matrix_local.inverted()@rest.head_local
            attachment=max(attachment,(head(b.name)-expected).length)
    contacts=['seat','grip.L','grip.R','peg.L','peg.R'] if mounted else ['ground.L','ground.R'] if spec['actor']=='human' else ['ground.'+n for n in ['front.L','front.R','hind.L','hind.R']]
    contact_errors={}
    if mounted:
        for name in held:
            target=root.matrix_world@Vector(profile['anchors'][name]['neutral_bike_position'])
            contact_errors[name]=(landmark(name)-target).length
    confidence='documented-helper-anchor' if spec['id'] in [3,4,5,6,8,9,10,11,44,45,51,52,60,66,101,102,105,106,140,141,142,143] else 'visual-inspection' if spec['source_slots'] else 'new-design'
    record=dict(schema_version=1,id=spec['id'],frame_id=spec['id'],name=spec['name'],actor=spec['actor'],family=spec['family'],
        source_slots=spec['source_slots'],interpretation='authored static candidate; not recovered original motion',review='draft',
        held_contacts=held,released_contacts=[c for c in contacts if c not in held],contact_errors_m=contact_errors,reference_confidence=confidence,root_matrix=[list(row) for row in root.matrix_basis],rig_matrix=[list(row) for row in rig.matrix_basis],
        bone_basis=basis,bone_world=world,bind=bind,max_attachment_error_m=attachment,max_bone_length_error_m=length_error,
        screenshot=f"gallery/{spec['id']:04d}.jpg")
    (OUT/f"poses/{spec['id']:04d}.json").write_text(json.dumps(record,separators=(',',':'))+'\n')
    # Each Action is ONE static keyframe with constant interpolation, not a clip.
    action=bpy.data.actions.new(f"{spec['id']:04d} | {spec['name']}");action.use_fake_user=True;rig.animation_data_create();rig.animation_data.action=action
    for bone in rig.pose.bones:
        bone.keyframe_insert(data_path='location',frame=1);bone.keyframe_insert(data_path='rotation_quaternion',frame=1);bone.keyframe_insert(data_path='scale',frame=1)
    rig.keyframe_insert(data_path='location',frame=1);rig.keyframe_insert(data_path='rotation_quaternion',frame=1);rig.keyframe_insert(data_path='scale',frame=1)
    for layer in action.layers:
        for strip in layer.strips:
            bag=strip.channelbag(action.slots[0])
            if bag:
                for curve in bag.fcurves:
                    for point in curve.keyframe_points:point.interpolation='CONSTANT'
    action.asset_mark();rig.animation_data.action=None
    manifest.append({k:record[k] for k in ['id','name','actor','family','source_slots','review','held_contacts','screenshot']})
    if attachment>.005 or length_error>.001:errors.append(dict(id=spec['id'],attachment=attachment,length_error=length_error))
    if render_ids is None or spec['id'] in render_ids:
        dest=OUT/record['screenshot']
        if not dest.exists():
            camera.location=(3.6,4.8,2.8) if mounted else (3.5,-5,2.5)
            if 291<=spec['id']<=296:camera.location=(0,-6,2.1)
            elif 297<=spec['id']<=302:camera.location=(0,6,2.1)
            elif 303<=spec['id']<=308:camera.location=(6,0,1.9)
            camera.rotation_euler=(Vector((0,0,.9))-camera.location).to_track_quat('-Z','Y').to_euler()
            scene.render.image_settings.file_format='JPEG';scene.render.image_settings.quality=88;scene.render.filepath=str(dest)
            bpy.ops.render.render(write_still=True)
    print('STATIC_POSE',spec['id'],f'attachment={attachment:.6f}',flush=True)
manifest=dict(schema_version=1,pose_kind='static-keyframes',runtime_connected=False,physics_changed=False,poses=manifest,use_cases=plan['use_cases'])
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
(OUT/'authoring-validation.json').write_text(json.dumps(dict(poses=len(manifest['poses']),actions=len(bpy.data.actions),frames_per_action=1,errors=errors,physics_changed=False),indent=2)+'\n')
# Keep an editable package outside runtime assets; source script rebuilds it.
bpy.context.view_layer.objects.active=human_rig;rig=human_rig;reset();human_seated(dict(params={}))
for o in bike_objects:o.hide_render=False
rider.hide_render=False
for o in cow_objects:o.hide_render=True;o.hide_set(True)
for o in bike_objects:o.hide_set(False)
rider.hide_set(False)
camera.location=(3.6,4.8,2.8);camera.rotation_euler=(Vector((0,0,.9))-camera.location).to_track_quat('-Z','Y').to_euler()
scene.frame_start=1;scene.frame_end=1
bpy.ops.file.pack_all()
path=pathlib.Path(opts.blend);path.parent.mkdir(parents=True,exist_ok=True);bpy.ops.wm.save_as_mainfile(filepath=str(path))
print('AUTHORING_COMPLETE',len(manifest['poses']),'errors',len(errors),flush=True)
