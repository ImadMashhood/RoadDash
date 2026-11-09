#!/usr/bin/env python3
"""Independent binary checks of the visual-only actor intake, without Blender."""
import hashlib
import json
import math
import struct
from pathlib import Path
from verify_building_mesh_assets import read_glb, accessor

ROOT = Path(__file__).resolve().parents[1]


def verify():
    catalog = json.loads((ROOT / 'content/props.json').read_text())
    receipt = json.loads((ROOT / 'docs/testing/props-in-game/runtime-intake.json').read_text())
    folder = ROOT / 'art/props/mesh_assets'
    assert hashlib.sha256((folder / 'MANIFEST.json').read_bytes()).hexdigest() == receipt['source_manifest_sha256']
    assert receipt['physics_data'] is False
    assert receipt['generator_sha256'] == hashlib.sha256((ROOT/'tools/prepare_runtime_props.py').read_bytes()).hexdigest()
    for record in receipt['files']:
        assert record['source_sha256'] == hashlib.sha256((ROOT/record['source_file']).read_bytes()).hexdigest()
    expected = {f['file']: f['sha256'] for f in receipt['files']}
    checked = set()
    for prop in catalog['records']:
        for file in prop.get('skin_files', []):
            path = folder / file
            relative = path.relative_to(ROOT).as_posix()
            assert hashlib.sha256(path.read_bytes()).hexdigest() == expected[relative], relative
            d, raw = read_glb(path)
            assert len(d['skins']) == 1
            skin = d['skins'][0]
            assert len(skin['joints']) == prop['bones']
            frames = receipt['cow_rest' if prop['bones'] == 10 else 'human_rest']
            # Independently decode inverse matrices and check bind-pose identity.
            acc = d['accessors'][skin['inverseBindMatrices']]
            view = d['bufferViews'][acc['bufferView']]
            start = view.get('byteOffset', 0) + acc.get('byteOffset', 0)
            assert acc['type'] == 'MAT4' and acc['componentType'] == 5126
            for i, (joint, frame) in enumerate(zip(skin['joints'], frames)):
                node = d['nodes'][joint]
                assert node['translation'] == frame
                matrix = struct.unpack_from('<16f', raw, start + i * 64)
                assert all(math.isfinite(x) for x in matrix)
                for axis in range(3):
                    assert abs(matrix[12 + axis] + frame[axis]) < 1e-6
                assert matrix[0] == matrix[5] == matrix[10] == matrix[15] == 1
            for node in d['nodes']:
                if 'mesh' not in node:
                    continue
                assert node['skin'] == 0
                assert not any(k in node for k in ('matrix', 'translation', 'rotation', 'scale'))
                for primitive in d['meshes'][node['mesh']]['primitives']:
                    a = primitive['attributes']
                    positions = list(accessor(d, raw, a['POSITION']))
                    weights = list(accessor(d, raw, a['WEIGHTS_0']))
                    joints = list(accessor(d, raw, a['JOINTS_0']))
                    assert len(positions) == len(weights) == len(joints)
                    for p, w, j in zip(positions, weights, joints):
                        assert all(math.isfinite(v) for v in (*p, *w))
                        assert abs(sum(w) - 1) < 1e-6 and min(w) >= 0
                        assert all(0 <= v < prop['bones'] for v in j)
            checked.add(relative)
    assert checked == set(expected) and len(checked) == 118
    print(f'Validated {len(checked)} actor GLBs: bind frames, inverse matrices, weights, indices and intake hashes')


if __name__ == '__main__':
    verify()
