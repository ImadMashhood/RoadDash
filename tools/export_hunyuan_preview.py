"""Bake a local visual preview using RoadDash's existing rider mesh format."""
import json
from pathlib import Path
import struct
import sys

import bpy
from mathutils import Matrix

sys.path.insert(0, str(Path(__file__).resolve().parent))
from export_riding_models import ROOT, SCALE, buffers, export_mesh, records

preview = ROOT / 'art/hunyuan-rider'
bpy.ops.wm.open_mainfile(filepath=str(preview / 'rider.blend'))
rig = next(obj for obj in bpy.context.scene.objects if obj.type == 'ARMATURE')
rider = next(obj for obj in rig.children if obj.type == 'MESH')
export_mesh(rider, 'standing', True)
pose = json.loads((ROOT / 'art/source/seated-0003.json').read_text())
rig.matrix_basis = Matrix(pose['rig_matrix'])
for name, basis in pose['bone_basis'].items():
    rig.pose.bones[name].matrix_basis = Matrix(basis)
bpy.context.view_layer.update()
export_mesh(rider, 'seated', True)

header = json.dumps(dict(version=1, scale=SCALE, visual_offset_y=.03,
                         vertex_stride_bytes=56, records=records, physics_data=False)).encode()
(preview / 'preview.rdmesh').write_bytes(b'RDMESH01' + struct.pack('<I',len(header)) +
                                        header + b''.join(buffers))
print('Preview records:', [(r['name'],r['indices']//3) for r in records])
