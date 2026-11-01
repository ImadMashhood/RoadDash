"""Independent static FK/Action/assembly checks inside Blender; no playback."""
import bpy,json,pathlib,math
from mathutils import Matrix
ROOT=pathlib.Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation/context'
bpy.ops.wm.open_mainfile(filepath=str(pathlib.Path.home()/'Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend'))
rig=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');rig.animation_data_clear();root=bpy.data.objects['Animation_Assembly'];pivot=bpy.data.objects['Context_BikePivot'];manifest=json.loads((BASE/'manifest.json').read_text());assembly=json.loads((BASE/'assembly.json').read_text());error=0;curves=0
for d in manifest['poses']:
 p=json.loads((BASE/f'poses/{d["id"]:04d}.json').read_text());root.matrix_basis=Matrix(p['root_matrix']);rig.matrix_basis=Matrix(p['rig_matrix']);pivot.matrix_basis=Matrix(assembly[str(d['id'])]['bike_matrix'])
 for n,m in p['bone_basis'].items():rig.pose.bones[n].matrix_basis=Matrix(m)
 bpy.context.view_layer.update()
 for n,m in p['bone_world'].items():
  actual=rig.matrix_world@rig.pose.bones[n].matrix;error=max(error,max(abs(actual[i][j]-m[i][j]) for i in range(4) for j in range(4)))
 assert (BASE/d['screenshot']).is_file()
assert error<1e-5,error
assert len(bpy.data.actions)==138
for a in bpy.data.actions:
 assert a.name in {d['name'] for d in manifest['poses']}
 for l in a.layers:
  for strip in l.strips:
   bag=strip.channelbag(a.slots[0])
   for f in bag.fcurves:
    curves+=1;assert len(f.keyframe_points)==1 and f.keyframe_points[0].co.x==1 and f.keyframe_points[0].interpolation=='CONSTANT'
for item in assembly.values():
 for p in item['props']:
  assert p['recipe']['kind'] in ['torus','cylinder'];assert p['material'] in bpy.data.materials;assert all(math.isfinite(v) for r in p['matrix'] for v in r)
for p in json.loads((BASE/'bike-poses.json').read_text()):
 assert len(p['root_matrix'])==4 and (BASE/p['screenshot']).is_file()
assert not any(o.rigid_body for o in bpy.data.objects)
report={'human_poses_reloaded':138,'bike_only_poses':16,'actions':138,'single_constant_frame':True,'curves':curves,'max_matrix_error':error,'rigid_bodies':0,'native_playback_integrated':False}
(BASE/'validation.json').write_text(json.dumps(report,indent=2)+'\n');print('CONTEXT_VERIFIED',json.dumps(report))
