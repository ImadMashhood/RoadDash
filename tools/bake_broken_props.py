"""Original-source crumpled furniture, GLB2 offline bake; no downloaded art.

The per-part recipes operate on the existing original low-LOD meshes, preserving
indices, UVs and embedded material/image bytes. No runtime mesh mutation/debris.
Run with --check to verify exact derivatives and their source/output hashes.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'art/props/mesh_assets'
ROLES = {'street_bin', 'recycling_pair', 'flower_planter', 'mailboxes', 'utility_box', 'news_boxes'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def bake(prop):
    source = (ASSETS / prop['files'][2]).read_bytes()
    length = struct.unpack_from('<I', source, 12)[0]
    gltf = json.loads(source[20:20 + length])
    binary = bytearray(source[28 + length:])
    size, center = prop['size'], prop['center']
    lo = [center[i] - size[i] / 2 for i in range(3)]
    hi = [center[i] + size[i] / 2 for i in range(3)]

    def accessor(index):
        a = gltf['accessors'][index]
        v = gltf['bufferViews'][a['bufferView']]
        return a, v.get('byteOffset', 0) + a.get('byteOffset', 0)

    bounds = [[float('inf')] * 3, [-float('inf')] * 3]
    parts = []
    for node in gltf['nodes']:
        if 'mesh' not in node:
            continue
        name = node.get('name', '')
        parts.append(name)
        assert not any(k in node for k in ('matrix', 'translation', 'rotation', 'scale')), name
        for primitive in gltf['meshes'][node['mesh']]['primitives']:
            a, offset = accessor(primitive['attributes']['POSITION'])
            points = []
            for i in range(a['count']):
                x, y, z = struct.unpack_from('<3f', binary, offset + 12 * i)
                h = max(0, min(1, (y - lo[1]) / size[1]))
                # Accordion-fold containers and sag their panels. Part-specific lids,
                # doors, roof and foliage remain recognisable original geometry.
                fold = math.sin(h * math.pi * 3) ** 2
                cx, cz = center[0], center[2]
                x = cx + (x - cx) * (0.80 - 0.20 * fold)
                z = cz + (z - cz) * (0.81 - 0.22 * fold)
                y *= 0.38
                if 'lid' in name:
                    y -= size[1] * 0.065 * (x - lo[0]) / size[0]
                    z += size[2] * 0.08
                elif 'door' in name or 'recessed' in name:
                    z = cz + (z - cz) * 0.55
                    y -= size[1] * 0.035 * h
                elif 'roof' in name or 'foliage' in name:
                    y -= size[1] * 0.075 * math.sin(math.pi * (x - lo[0]) / size[0]) ** 2
                # Keep the original supported crush collider; zero detached bodies.
                p = [max(lo[0], min(hi[0], x)), max(lo[1] * 0.38, min(hi[1] * 0.38, y)), max(lo[2], min(hi[2], z))]
                points.append(p)
                struct.pack_into('<3f', binary, offset + 12 * i, *p)
                for axis in range(3):
                    bounds[0][axis] = min(bounds[0][axis], p[axis])
                    bounds[1][axis] = max(bounds[1][axis], p[axis])
            a['min'] = [min(p[i] for p in points) for i in range(3)]
            a['max'] = [max(p[i] for p in points) for i in range(3)]
            # Recompute normals from the unchanged authored topology after deformation.
            ia, io = accessor(primitive['indices'])
            fmt = {5123: 'H', 5125: 'I'}[ia['componentType']]
            indices = struct.unpack_from('<' + fmt * ia['count'], binary, io)
            normals = [[0., 0., 0.] for _ in points]
            for t in range(0, len(indices), 3):
                ids = indices[t:t + 3]
                p, q, r = [points[j] for j in ids]
                u, v = [[b - a for a, b in zip(p, s)] for s in (q, r)]
                n = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]]
                for j in ids:
                    normals[j] = [a + b for a, b in zip(normals[j], n)]
            _, no = accessor(primitive['attributes']['NORMAL'])
            for i, n in enumerate(normals):
                length = math.sqrt(sum(x*x for x in n))
                struct.pack_into('<3f', binary, no + 12*i, *([x/length for x in n] if length else [0., 1., 0.]))
    gltf['asset']['extras'] = {'recipe': 'RoadDash original crumpled furniture v1', 'source_sha256': digest(source)}
    encoded = json.dumps(gltf, separators=(',', ':')).encode()
    encoded += b' ' * (-len(encoded) % 4)
    output = struct.pack('<III', 0x46546C67, 2, 28 + len(encoded) + len(binary)) + struct.pack('<II', len(encoded), 0x4E4F534A) + encoded + struct.pack('<II', len(binary), 0x004E4942) + binary
    return output, {'id': prop['id'], 'role': prop['builder'], 'source': prop['files'][2], 'source_sha256': digest(source), 'file': f"broken/{prop['id']}.glb", 'sha256': digest(output), 'bounds': bounds, 'parts': parts}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    records = []
    for prop in json.loads((ROOT / 'content/props.json').read_text())['records']:
        if prop['builder'] not in ROLES:
            continue
        data, record = bake(prop)
        path = ASSETS / record['file']
        if args.check:
            assert path.read_bytes() == data, path
        else:
            path.parent.mkdir(exist_ok=True)
            path.write_bytes(data)
        records.append(record)
    manifest = json.dumps({'schema_version': 1, 'recipe_sha256': digest(Path(__file__).read_bytes()), 'records': records}, indent=2) + '\n'
    path = ASSETS / 'broken/manifest.json'
    if args.check:
        assert path.read_text() == manifest, path
    else:
        path.write_text(manifest)
    print(f'{len(records)} original broken meshes; six roles; source/output/recipe hashes verified' if args.check else f'Baked {len(records)} original broken meshes')


if __name__ == '__main__':
    main()
