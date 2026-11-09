"""Load a complete context pose, including fist shape keys, bike and weapon props.
Run in Blender Text Editor (set POSE_ID) or append -- 2222 to Blender CLI.
"""
import bpy,json,pathlib,sys
sys.path.insert(0,str(pathlib.Path(__file__).resolve().parent))
from authoring_hands import apply_values
from mathutils import Matrix
ROOT=pathlib.Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation/context';POSE_ID=2222
if '--' in sys.argv:POSE_ID=int(sys.argv[sys.argv.index('--')+1])
assembly=json.loads((BASE/'assembly.json').read_text())[str(POSE_ID)];code=assembly['code'];rig=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');root=bpy.data.objects['Animation_Assembly'];pivot=bpy.data.objects['Context_BikePivot'];rig.animation_data_clear()
for o in list(bpy.data.objects):
 if o.name.startswith('Context ') and o.type=='MESH':bpy.data.objects.remove(o,do_unlink=True)
file=BASE/f'poses/{POSE_ID:04d}.json'
if file.exists():
 pose=json.loads(file.read_text());root.matrix_basis=Matrix(pose['root_matrix']);rig.matrix_basis=Matrix(pose['rig_matrix'])
 for n,m in pose['bone_basis'].items():rig.pose.bones[n].matrix_basis=Matrix(m)
else:
 pose=next(p for p in json.loads((BASE/'bike-poses.json').read_text()) if p['id']==POSE_ID);root.matrix_basis=Matrix(pose['root_matrix'])
pivot.matrix_basis=Matrix(assembly['bike_matrix']);mounted=code<28 or 39<=code<=42 or code>=44
rider=next(o for o in bpy.data.objects if o.type=='MESH' and any(m.type=='ARMATURE' and m.object==rig for m in o.modifiers))
for o in bpy.data.objects:
 if o.type!='MESH' or o.name=='Authoring stage':continue
 cow=o.name.startswith(('ReviewCow','Cow '));human=any(m.type=='ARMATURE' and m.object==rig for m in o.modifiers);hidden=True if cow else code>=44 if human else not mounted;o.hide_render=hidden;o.hide_set(hidden)
apply_values(rider,assembly['hand_values'])
for p in assembly['props']:
 recipe=p['recipe']
 if recipe['kind']=='cylinder':bpy.ops.mesh.primitive_cylinder_add(vertices=12,radius=recipe['radius'],depth=recipe['depth'])
 else:bpy.ops.mesh.primitive_torus_add(major_segments=8,minor_segments=4,major_radius=.026,minor_radius=.006)
 o=bpy.context.object;o.name=p['name'];o.matrix_world=Matrix(p['matrix']);o.data.materials.append(bpy.data.materials[p['material']])
bpy.context.view_layer.update();print('CONTEXT_LOADED',assembly['label'],'original code',code,'phase',assembly['phase'])
