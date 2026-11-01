"""Render selected static poses from the editable authoring scene, not game state."""
import bpy,json,pathlib,sys
from mathutils import Matrix,Vector
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'art/animation'
bpy.ops.wm.open_mainfile(filepath=str(pathlib.Path.home()/'Downloads/RoadDash_Animation_Keyframes_v1/RoadDash_Animation_Keyframes.blend'))
ids=[3,4,11,45,52,60,66,293,1012,1051,1057,1081,1085,1086,1101,1104,1142,1203,1209]
if '--' in sys.argv:ids=[int(i) for i in sys.argv[sys.argv.index('--')+1].split(',')]
root=bpy.data.objects['Animation_Assembly'];human=next(o for o in bpy.data.objects if o.type=='ARMATURE' and o.name!='Cow_Review_Rig');cow=bpy.data.objects['Cow_Review_Rig'];scene=bpy.context.scene
(OUT/'gallery/views').mkdir(exist_ok=True)
for id in ids:
    p=json.loads((OUT/f'poses/{id:04d}.json').read_text());rig=cow if p['actor']=='cow' else human
    root.matrix_basis=Matrix(p['root_matrix']);rig.matrix_basis=Matrix(p['rig_matrix'])
    for name,m in p['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(m)
    for o in bpy.data.objects:
        if o.type!='MESH' or o.name=='Authoring stage':continue
        is_cow=o.name.startswith(('ReviewCow','Cow '))
        is_human=any(m.type=='ARMATURE' and m.object==human for m in o.modifiers)
        o.hide_render=(p['actor']!='cow') if is_cow else p['actor']=='cow' if is_human else p['family'] not in ['mounted','riding','crash']
    bpy.context.view_layer.update()
    for name,location in [('front',(0,-6,2.1)),('rear',(0,6,2.1)),('side',(6,0,1.9))]:
        camera=scene.camera;camera.location=location;camera.rotation_euler=(Vector((0,0,.9))-camera.location).to_track_quat('-Z','Y').to_euler()
        scene.render.filepath=str(OUT/f'gallery/views/{id:04d}-{name}.jpg');scene.render.image_settings.file_format='JPEG';scene.render.image_settings.quality=88
        bpy.ops.render.render(write_still=True)
    print('REVIEW_VIEWS',id,flush=True)
