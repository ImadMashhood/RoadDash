"""Original Blender hand adapter for the runtime's metre-based visual profile.

No physical rig, clip timing or source GLB is changed. See ADR080. The optional
edge sampling is conforming Blender subdivision; runtime triangle tessellation
is retained independently, so this is not a bitwise mesh-export replacement.
"""
import hashlib
import json
from pathlib import Path

import bpy
import bmesh
from mathutils import Matrix, Quaternion, Vector

ROOT = Path(__file__).resolve().parents[1]
SCALE = 1.2  # Same original source -> renderer conversion as export_riding_models.
PROFILE_PATH = ROOT / 'content/hand-contacts.json'
PROFILE = json.loads(PROFILE_PATH.read_text())


def clamp(value):
    return min(1.0, max(0.0, value))


def fold(point, angles):
    """Port of models::hands::fold, in renderer metres and wrist coordinates."""
    p = Vector(point)
    if p.y < .015:
        return p, Quaternion()
    out = p.copy()
    rotation = Quaternion()
    thumb = clamp((abs(p.x) - .048) / .015) * clamp((.10 - p.y) / .025)
    for hinge, angle in reversed(list(zip(PROFILE['hinges_m'], angles))):
        influence = clamp((p.y - hinge + .004) / .008) * (1 - thumb)
        q = Quaternion((1, 0, 0), angle * influence)
        anchor = Vector((0, hinge, 0))
        out = anchor + q @ (out - anchor)
        rotation = q @ rotation
    sign = 1 if p.x > 0 else -1 if p.x < 0 else 0
    q = Quaternion((0, 0, 1), sign * angles[0] * thumb) @ Quaternion((0, 1, 0), -sign * angles[0] * .8 * thumb)
    anchor = Vector((sign * .035, .025, 0))
    return anchor + q @ (out - anchor), q @ rotation


def wrist_frame(rig, side):
    # Runtime visual distal origin inherits the forearm, .27 renderer metres
    # along its local Y. The author's neutral hand basis matches that frame.
    forearm = rig.data.bones['forearm.' + side].matrix_local
    frame = forearm.copy()
    frame.translation = forearm @ Vector((0, .27 / SCALE, 0))
    return frame


def hand_weight(rider, vertex, side, frame):
    group_names = {'forearm.' + side, 'hand.' + side}
    weight = sum(g.weight for g in vertex.groups if rider.vertex_groups[g.group].name in group_names)
    along = (frame.inverted() @ vertex.co).y * SCALE + .27
    return weight * clamp((along - .22) / .07)


def install(rider, rig, refine=True):
    """Replace historical compression keys; add independent open/grip/fist keys.

    Original per-corner UVs/materials and skin groups follow Blender's edge
    interpolation. Subdivision adds no surface displacement. Source files remain
    untouched. Existing key values must be restored through assembly metadata.
    """
    if rider.data.shape_keys:
        rider.shape_key_clear()
    original_vertices = len(rider.data.vertices)
    if refine:
        bm = bmesh.new()
        bm.from_mesh(rider.data)
        bmesh.ops.triangulate(bm, faces=list(bm.faces))
        deform = bm.verts.layers.deform.active
        frames = {side: wrist_frame(rig, side).inverted() for side in ['L', 'R']}
        groups = {side: {g.index for g in rider.vertex_groups if g.name in ['hand.'+side, 'forearm.'+side]} for side in ['L', 'R']}
        def distal_weight(vertex):
            return sum(sum(w for g, w in vertex[deform].items() if g in groups[side]) *
                       clamp((((frames[side] @ vertex.co).y * SCALE + .27) - .22) / .07)
                       for side in ['L', 'R'])
        for _ in range(3):
            edges = set()
            for face in bm.faces:
                edge = max(face.edges, key=lambda e: e.calc_length())
                if edge.calc_length() * SCALE > .035 and all(distal_weight(v) > .95 for v in edge.verts):
                    edges.add(edge)
            if edges:
                bmesh.ops.subdivide_edges(bm, edges=list(edges), cuts=1, use_grid_fill=False)
                bmesh.ops.triangulate(bm, faces=[f for f in bm.faces if len(f.verts) != 3])
        bm.to_mesh(rider.data)
        bm.free()
        rider.data.update()
    rider.shape_key_add(name='Basis')
    for side in ['L', 'R']:
        frame = wrist_frame(rig, side)
        for shape in ['Grip', 'Fist']:
            key = rider.shape_key_add(name='Context_' + shape + '.' + side)
            angles = PROFILE[shape.lower() + '_angles_rad']
            for vertex in rider.data.vertices:
                weight = hand_weight(rider, vertex, side, frame)
                if weight < .001:
                    continue
                local = (frame.inverted() @ vertex.co) * SCALE
                folded, _ = fold(local, angles)
                key.data[vertex.index].co = vertex.co + (frame @ (folded / SCALE) - vertex.co) * weight
    sockets = {}
    for side_index, side in enumerate(['L', 'R']):
        # Parent to the authoring hand so sockets inherit every loaded pose.
        bone = rig.data.bones['hand.' + side]
        frame = wrist_frame(rig, side)
        sockets[side] = {}
        for kind in ['Grip', 'Weapon']:
            name = 'Context_' + kind + 'Socket.' + side
            old = bpy.data.objects.get(name)
            if old:
                bpy.data.objects.remove(old, do_unlink=True)
            socket = bpy.data.objects.new(name, None)
            bpy.context.collection.objects.link(socket)
            socket.empty_display_type = 'ARROWS'
            socket.empty_display_size = .06
            socket.parent = rig
            socket.parent_type = 'BONE'
            socket.parent_bone = bone.name
            # Bone parenting includes the tail translation; cancel it explicitly.
            local = bone.matrix_local.inverted() @ frame
            local.translation += local.to_quaternion() @ (Vector(PROFILE['grip_sockets_m'][side_index]) / SCALE)
            if kind == 'Weapon':
                x, y, z, w = PROFILE['weapon_rotation_xyzw'][side_index]
                local = local @ Quaternion((w, x, y, z)).to_matrix().to_4x4()
            socket.matrix_basis = Matrix.Translation((0, -bone.length, 0)) @ local
            socket['profile_sha256'] = hashlib.sha256(PROFILE_PATH.read_bytes()).hexdigest()
            socket['side'] = side
            socket['kind'] = kind.lower()
            sockets[side][kind.lower()] = [list(row) for row in local]
    return dict(profile_sha256=hashlib.sha256(PROFILE_PATH.read_bytes()).hexdigest(),
                source_scale=SCALE, vertices_before=original_vertices,
                vertices_after=len(rider.data.vertices), sockets=sockets,
                refinement='three conforming Blender long-edge passes; runtime tessellation independent',
                review='draft; silhouette/clearance/user approval open')


def values(rider):
    return {side: {shape.lower(): rider.data.shape_keys.key_blocks['Context_' + shape + '.' + side].value
                   for shape in ['Grip', 'Fist']} for side in ['L', 'R']}


def apply_values(rider, hand_values):
    for side in ['L', 'R']:
        for shape in ['Grip', 'Fist']:
            rider.data.shape_keys.key_blocks['Context_' + shape + '.' + side].value = hand_values[side][shape.lower()]
