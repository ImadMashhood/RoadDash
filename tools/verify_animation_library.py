"""Independent Blender reload of every saved candidate and static Action."""
import bpy,json,pathlib,sys
from mathutils import Matrix
ROOT=pathlib.Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation'
file=ROOT/'RoadDash_Animation_Keyframes.blend'
if not file.exists():file=pathlib.Path.home()/'Downloads/RoadDash_Animation_Keyframes_v1/RoadDash_Animation_Keyframes.blend'
bpy.ops.wm.open_mainfile(filepath=str(file))
root=bpy.data.objects['Animation_Assembly'];human=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');cow=bpy.data.objects['Cow_Review_Rig']
for rig in [human,cow]:rig.animation_data_clear()
max_error=0.0
catalog=json.loads((BASE/'manifest.json').read_text())
for d in catalog['poses']:
    p=json.loads((BASE/f"poses/{d['id']:04d}.json").read_text());rig=cow if p['actor']=='cow' else human
    root.matrix_basis=Matrix(p['root_matrix']);rig.matrix_basis=Matrix(p['rig_matrix'])
    for name,m in p['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(m)
    bpy.context.view_layer.update()
    for name,saved in p['bone_world'].items():
        actual=rig.matrix_world@rig.pose.bones[name].matrix
        max_error=max(max_error,max(abs(actual[i][j]-saved[i][j]) for i in range(4) for j in range(4)))
assert max_error<1e-5,max_error
curves=0
for action in bpy.data.actions:
    for layer in action.layers:
        for strip in layer.strips:
            bag=strip.channelbag(action.slots[0])
            if bag:
                for curve in bag.fcurves:
                    curves+=1
                    assert len(curve.keyframe_points)==1
                    point=curve.keyframe_points[0]
                    assert point.co.x==1 and point.interpolation=='CONSTANT'
assert len(bpy.data.actions)==len(catalog['poses'])
assert not any(o.rigid_body for o in bpy.data.objects)
report=dict(poses_reloaded=len(catalog['poses']),actions=len(bpy.data.actions),curves_checked=curves,max_matrix_error=max_error,all_keys_single_frame_constant=True,rigid_bodies=0)
(BASE/'round-trip-validation.json').write_text(json.dumps(report,indent=2)+'\n')
print('ANIMATION_ROUND_TRIP',json.dumps(report),flush=True)
