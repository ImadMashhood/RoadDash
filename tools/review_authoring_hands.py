"""Blender: verify editable hand library, export/reload GLB and render 18 views.
Output original media only. No timeline or gameplay integration is inferred.
"""
import bpy
import hashlib
import json
import math
import runpy
import pathlib
import sys
from mathutils import Matrix, Vector
from mathutils.kdtree import KDTree
ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
from authoring_hands import PROFILE, PROFILE_PATH, SCALE, apply_values, fold, wrist_frame
BASE = ROOT/'art/animation/context'
OUT = ROOT/'docs/testing/part02-authoring'
OUT.mkdir(parents=True, exist_ok=True)
BLEND = pathlib.Path.home()/'Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend'
bpy.ops.wm.open_mainfile(filepath=str(BLEND))
rig = next(o for o in bpy.data.objects if o.type == 'ARMATURE' and o.name != 'Cow_Review_Rig')
rider = next(o for o in bpy.data.objects if o.type == 'MESH' and any(m.type == 'ARMATURE' and m.object == rig for m in o.modifiers))
assembly = json.loads((BASE/'assembly.json').read_text())
profile_hash = hashlib.sha256(PROFILE_PATH.read_bytes()).hexdigest()
assert json.loads((BASE/'hand-authoring.json').read_text())['profile_sha256'] == profile_hash
# Every pose loads a complete four-key state, never leaks a previous attack.
for pose in assembly.values():
    apply_values(rider, pose['hand_values'])
    for side in ['L', 'R']:
        v = pose['hand_values'][side]
        assert set(v) == {'grip', 'fist'}
        assert all(0 <= w <= 1 for w in v.values()) and sum(v.values()) <= 1
for side in ['L', 'R']:
    assert assembly['2222' if side == 'L' else '2232']['hand_values'][side] == {'grip': 0.0, 'fist': 1.0}
# Exercise the actual supplied loader, including reset after a fist, props,
# supported-recovery draft and mount. Measure static calibrated socket clearance
# separately from unchanged original landmark contact errors.
pose_load_checks = []
original_argv = sys.argv[:]
contact_profile = json.loads((ROOT/'art/source/contact-profile.json').read_text())
manifest = {p['id']:p for p in json.loads((BASE/'manifest.json').read_text())['poses']}
for key in [2222, 2020, 2030, 2232, 2100, 2160, 2362, 2390]:
    sys.argv = ['blender', '--', str(key)]
    runpy.run_path(str(ROOT/'tools/load_context_pose.py'), run_name='__main__')
    item = assembly[str(key)]
    gaps = {}
    if any('grip.'+side in manifest[key]['held_contacts'] for side in ['L','R']):
        pivot = bpy.data.objects['Context_BikePivot']
        for side in ['L', 'R']:
            if 'grip.'+side not in manifest[key]['held_contacts']: continue
            target = pivot.matrix_world @ Vector(contact_profile['anchors']['grip.'+side]['neutral_bike_position'])
            actual = bpy.data.objects['Context_GripSocket.'+side].matrix_world.translation
            gaps[side] = (actual-target).length*SCALE
            assert gaps[side] < .001, (key,side,gaps[side])
    for side in ['L', 'R']:
        for shape in ['Grip', 'Fist']:
            assert rider.data.shape_keys.key_blocks['Context_'+shape+'.'+side].value == item['hand_values'][side][shape.lower()]
    prop_error = None
    if key == 2100:
        prop_error = (bpy.data.objects['Context chain link'].matrix_world.translation - bpy.data.objects['Context_WeaponSocket.L'].matrix_world.translation).length*SCALE
        assert prop_error < 1e-6, prop_error
    if key == 2160:
        handle = bpy.data.objects['Context club grip']
        socket = bpy.data.objects['Context_WeaponSocket.L']
        # Grip cylinder spans -.04..+.10 along socket +Y, center +.03.
        expected = socket.matrix_world.translation + socket.matrix_world.to_quaternion() @ Vector((0,.03,0))
        prop_error = (handle.matrix_world.translation-expected).length*SCALE
        assert prop_error < 1e-6, prop_error
    pose_load_checks.append(dict(pose=key,hand_values=item['hand_values'],held_grip_socket_gap_m=gaps,weapon_proxy_error_m=prop_error))
sys.argv = original_argv
# Neutral authoring frame: immutable bind geometry; no animation/keyframe writes.
rig.animation_data_clear()
rig.matrix_basis = Matrix.Identity(4)
for bone in rig.pose.bones:
    bone.matrix_basis = Matrix.Identity(4)
bpy.data.objects['Animation_Assembly'].matrix_basis = Matrix.Identity(4)
apply_values(rider, {s: {'grip': 0., 'fist': 0.} for s in ['L', 'R']})
bpy.context.view_layer.update()
socket_error = 0.
for side_index, side in enumerate(['L', 'R']):
    for kind in ['Grip', 'Weapon']:
        obj = bpy.data.objects['Context_'+kind+'Socket.'+side]
        expected = rig.matrix_world @ wrist_frame(rig, side) @ (Vector(PROFILE['grip_sockets_m'][side_index])/SCALE)
        socket_error = max(socket_error, (obj.matrix_world.translation-expected).length*SCALE)
        assert obj['profile_sha256'] == profile_hash
assert socket_error < 1e-6, socket_error
# Cross-language fixture: boundaries, thumb side/sign and fully curled fingers.
samples = []
for p in [(0, 0, 0), (0, .014, 0), (0, .015, .01), (.02, .071, -.01),
          (0, .075, 0), (.02, .12, .01), (-.02, .17, -.01),
          (.063, .06, .01), (-.063, .06, .01), (.055, .09, -.01)]:
    for shape in ['grip', 'fist']:
        result, rotation = fold(p, PROFILE[shape+'_angles_rad'])
        samples.append(dict(point=p, shape=shape, folded=list(result), normal=list(rotation @ Vector((0, 0, 1)))))
(OUT/'fold-fixtures.json').write_text(json.dumps({'profile_sha256':profile_hash,'samples':samples}, indent=2)+'\n')
# Real glTF morph/skin/UV export and import, in a separate scene. Source remains.
bpy.ops.object.select_all(action='DESELECT')
for obj in [rig, rider] + [o for o in bpy.data.objects if o.name.startswith(('Context_GripSocket.', 'Context_WeaponSocket.'))]:
    obj.hide_set(False)
    obj.select_set(True)
bpy.context.view_layer.objects.active = rider
source_sockets = {o.name:o.matrix_world.copy() for o in bpy.context.selected_objects if 'Socket.' in o.name}
export = BLEND.parent/'RoadDash_Hands_Roundtrip.glb'
bpy.ops.export_scene.gltf(filepath=str(export), use_selection=True, export_animations=False,
                         export_morph=True, export_morph_normal=True, export_skins=True)
source_keys = {k.name: len(k.data) for k in rider.data.shape_keys.key_blocks}
source_uvs = len(rider.data.uv_layers)
original_scene = bpy.context.scene
reload_scene = bpy.data.scenes.new('Hand export reload verification')
bpy.context.window.scene = reload_scene
bpy.ops.import_scene.gltf(filepath=str(export))
imported = [o for o in reload_scene.objects if o.type == 'MESH' and o.data.shape_keys]
assert imported
imported_key_names = set().union(*(set(k.name for k in o.data.shape_keys.key_blocks) for o in imported))
assert set(source_keys) <= imported_key_names, imported_key_names
assert all(len(o.data.uv_layers) == source_uvs for o in imported)
assert all(math.isfinite(c) for o in imported for v in o.data.vertices for c in v.co)
assert all(math.isfinite(c) for o in imported for n in o.data.corner_normals for c in n.vector)
assert all(math.isfinite(c) for o in imported for k in o.data.shape_keys.key_blocks for v in k.data for c in v.co)
assert sum(o.type == 'ARMATURE' for o in reload_scene.objects) == 1
imported_sockets = [o for o in reload_scene.objects if 'Socket.' in o.name]
assert len(imported_sockets) == 4
bpy.context.view_layer.update()
glb_socket_error = max((o.matrix_world.translation - source_sockets[next(n for n in source_sockets if o.name.startswith(n))].translation).length*SCALE for o in imported_sockets)
glb_socket_rotation_error = max(abs(o.matrix_world[i][j]-source_sockets[next(n for n in source_sockets if o.name.startswith(n))][i][j]) for o in imported_sockets for i in range(3) for j in range(3))
assert glb_socket_error < 1e-5 and glb_socket_rotation_error < 1e-5, (glb_socket_error, glb_socket_rotation_error)
# UV values survive the export as well as the layer name/count. glTF can split
# coincident seam vertices, so compare against all coincident source corners.
uv_tree = KDTree(len(rider.data.vertices))
for v in rider.data.vertices: uv_tree.insert(rider.matrix_world @ v.co, v.index)
uv_tree.balance()
uvs = {}
for loop in rider.data.loops: uvs.setdefault(loop.vertex_index, []).append(rider.data.uv_layers.active.data[loop.index].uv.copy())
uv_error = 0.
for obj in imported:
    for loop in obj.data.loops:
        p = obj.matrix_world @ obj.data.vertices[loop.vertex_index].co
        candidates = [uv for _, index, _ in uv_tree.find_range(p, 1e-5) for uv in uvs.get(index, [])]
        assert candidates
        actual = obj.data.uv_layers.active.data[loop.index].uv
        uv_error = max(uv_error, min((actual-uv).length for uv in candidates))
assert uv_error < 1e-6, uv_error
# Compare evaluated source/imported surfaces for each independent target. glTF
# may split vertices at material/UV seams; nearest-surface matching handles that.
geometry_errors = {}
for target in ['Basis', 'Context_Grip.L', 'Context_Fist.L', 'Context_Grip.R', 'Context_Fist.R']:
    bpy.context.window.scene = original_scene
    for key in rider.data.shape_keys.key_blocks:
        if key.name != 'Basis': key.value = float(key.name == target)
    bpy.context.view_layer.update()
    evaluated = rider.evaluated_get(bpy.context.evaluated_depsgraph_get())
    source = [evaluated.matrix_world @ v.co for v in evaluated.data.vertices]
    tree = KDTree(len(source))
    for index, point in enumerate(source): tree.insert(point, index)
    tree.balance()
    bpy.context.window.scene = reload_scene
    for obj in imported:
        for key in obj.data.shape_keys.key_blocks:
            if key.name != 'Basis': key.value = float(key.name == target)
    bpy.context.view_layer.update()
    worst = 0.
    for obj in imported:
        evaluated = obj.evaluated_get(bpy.context.evaluated_depsgraph_get())
        worst = max(worst, max(tree.find(evaluated.matrix_world @ v.co)[2] for v in evaluated.data.vertices))
    geometry_errors[target] = worst * SCALE
assert max(geometry_errors.values()) < 1e-5, geometry_errors
bpy.context.window.scene = original_scene
# Isolated original hand views: all non-rider meshes hidden; pose is neutral.
for o in original_scene.objects:
    if o.type == 'MESH':
        o.hide_render = o != rider
scene = original_scene
scene.render.resolution_x = 560
scene.render.resolution_y = 560
scene.render.resolution_percentage = 100
scene.render.image_settings.file_format = 'PNG'
scene.cycles.samples = 16
scene.camera.data.type = 'ORTHO'
scene.camera.data.ortho_scale = .28
captures = []
for side_index, side in enumerate(['L', 'R']):
    frame = rig.matrix_world @ wrist_frame(rig, side)
    center = frame @ Vector((0, .085/SCALE, 0))
    for shape in ['Open', 'Grip', 'Fist']:
        apply_values(rider, {s: {'grip': float(s == side and shape == 'Grip'), 'fist': float(s == side and shape == 'Fist')} for s in ['L', 'R']})
        for view, direction in [('Palm', (0, 0, -1)), ('Back', (0, 0, 1)), ('Side', (1 if side == 'L' else -1, 0, 0))]:
            scene.camera.location = center + frame.to_quaternion() @ Vector(direction)*2
            scene.camera.rotation_euler = (center-scene.camera.location).to_track_quat('-Z','Y').to_euler()
            name = f'{side}-{shape}-{view}.png'
            scene.render.filepath = str(OUT/name)
            bpy.ops.render.render(write_still=True)
            captures.append(dict(side=side,shape=shape,view=view,file=name,sha256=hashlib.sha256((OUT/name).read_bytes()).hexdigest()))
sources = ['content/hand-contacts.json','tools/authoring_hands.py','tools/build_context_poses.py','tools/load_context_pose.py','tools/review_authoring_hands.py','art/source/rider.glb','art/source/seated-0003.json','art/animation/context/assembly.json','art/animation/context/hand-authoring.json']
report = dict(source_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources},blender=bpy.app.version_string,profile_sha256=profile_hash,assembly_states=len(assembly),pose_load_checks=pose_load_checks,
              editable_blend=str(BLEND),blend_sha256=hashlib.sha256(BLEND.read_bytes()).hexdigest(),roundtrip_glb=str(export),
              shape_keys=source_keys,source_socket_frame_error_m=socket_error,glb_socket_error_m=glb_socket_error,glb_socket_rotation_error=glb_socket_rotation_error,glb_uv_error=uv_error,glb_surface_error_m=geometry_errors,glb_sha256=hashlib.sha256(export.read_bytes()).hexdigest(),
              imported_meshes=len(imported),imported_morph_names=sorted(imported_key_names),uv_layers=source_uvs,
              no_animation_export=True,physics_changed=False,user_approved=False,captures=captures,
              limits=['Blender subdivision differs from runtime tessellation; exact source/runtime silhouettes remain unapproved.',
                      'Morph normals are recalculated by Blender export; source hard-normal identity remains an open gate.',
                      'Static source rig and physical-driver runtime retarget are distinct; GLB reload is not runtime rig replacement.',
                      'Held source socket-to-bike gaps use calibrated authoring IK and are separate from native retarget/art clearance acceptance.'])
(OUT/'roundtrip.json').write_text(json.dumps(report,indent=2)+'\n')
(OUT/'index.html').write_text('<!doctype html><meta charset="utf-8"><title>Part02 hand authoring</title><style>body{background:#161b22;color:#eee;font:16px sans-serif;margin:24px}.grid{display:grid;grid-template-columns:repeat(3,1fr);gap:12px}img{width:100%}figure{margin:0}</style><h1>Part02 original authoring hands</h1><p>Draft; same curl profile as runtime. Neutral source rig. Palm/back/side views; no pose approval or gameplay timing inferred. See roundtrip.json for export limits.</p><div class="grid">'+''.join(f'<figure><a href="{c["file"]}"><img src="{c["file"]}"></a><figcaption>{c["side"]} · {c["shape"]} · {c["view"]}</figcaption></figure>' for c in captures)+'</div>\n')
print('HAND_AUTHORING_VERIFIED',json.dumps({k:v for k,v in report.items() if k!='captures'}))
