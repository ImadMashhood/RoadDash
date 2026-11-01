"""Apply one static candidate to the authoring scene; never runs interpolation.
Open the supplied .blend, then run this script in Blender's Text Editor.
CLI: blender library.blend -b --python tools/load_animation_pose.py -- 1012
"""
import bpy,json,pathlib,sys
from mathutils import Matrix
POSE_ID=3
if '--' in sys.argv:POSE_ID=int(sys.argv[sys.argv.index('--')+1])
BASE=pathlib.Path(__file__).resolve().parents[1]/'art/animation'
def load_pose(id):
    p=json.loads((BASE/f'poses/{id:04d}.json').read_text())
    root=bpy.data.objects['Animation_Assembly'];human=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');cow=bpy.data.objects['Cow_Review_Rig']
    rig=cow if p['actor']=='cow' else human
    rig.animation_data_clear();root.matrix_basis=Matrix(p['root_matrix']);rig.matrix_basis=Matrix(p['rig_matrix'])
    for name,m in p['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(m)
    for o in bpy.data.objects:
        if o.type!='MESH' or o.name=='Authoring stage':continue
        is_cow=o.name.startswith(('ReviewCow','Cow '))
        is_human=any(m.type=='ARMATURE' and m.object==human for m in o.modifiers)
        hidden=(p['actor']!='cow') if is_cow else p['actor']=='cow' if is_human else p['family'] not in ['mounted','riding','crash']
        o.hide_render=hidden;o.hide_set(hidden)
    bpy.context.view_layer.update()
    return p
p=load_pose(POSE_ID)
print('Loaded static keyframe',p['id'],p['name'],'held',p['held_contacts'])
