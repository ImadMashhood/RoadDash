"""Independent damaged furniture GLB/material/collider-envelope checks."""
import json
import math
from pathlib import Path
from verify_building_mesh_assets import read_glb, accessor

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'art/props/mesh_assets'


def main():
    catalog = {p['id']: p for p in json.loads((ROOT / 'content/props.json').read_text())['records']}
    records = json.loads((BASE / 'broken/manifest.json').read_text())['records']
    vertices = triangles = 0
    for record in records:
        prop = catalog[record['id']]
        source, original = read_glb(BASE / record['source'])
        model, binary = read_glb(BASE / record['file'])
        assert model['materials'] == source['materials']
        assert model['textures'] == source['textures']
        for image in model['images']:
            v = model['bufferViews'][image['bufferView']]
            offset = v.get('byteOffset', 0)
            assert binary[offset:offset+v['byteLength']] == original[offset:offset+v['byteLength']]
        changed = 0
        for mesh, intact in zip(model['meshes'], source['meshes'], strict=True):
            for primitive, src in zip(mesh['primitives'], intact['primitives'], strict=True):
                positions = list(accessor(model, binary, primitive['attributes']['POSITION']))
                normals = accessor(model, binary, primitive['attributes']['NORMAL'])
                before = accessor(source, original, src['attributes']['POSITION'])
                indices = [i[0] for i in accessor(model, binary, primitive['indices'])]
                assert indices == [i[0] for i in accessor(source, original, src['indices'])]
                assert list(accessor(model, binary, primitive['attributes']['TEXCOORD_0'])) == list(accessor(source, original, src['attributes']['TEXCOORD_0']))
                for p, n, old in zip(positions, normals, before, strict=True):
                    assert all(math.isfinite(x) for x in (*p, *n))
                    assert abs(sum(x*x for x in n)-1) < 1e-4
                    # Apply the runtime fitted origin/scale: solid collider local
                    # bounds are +/- half width/depth and [0, .38*height].
                    normalized = [(p[i]-prop['center'][i]*(.38 if i==1 else 1))/prop['size'][i] for i in range(3)]
                    assert abs(normalized[0]) <= .50001 and abs(normalized[2]) <= .50001
                    assert -.19001 <= normalized[1] <= .19001
                    changed += int(abs(p[0]-old[0]) + abs(p[2]-old[2]) > 1e-5)
                for i in range(0, len(indices), 3):
                    p, q, r = [positions[j] for j in indices[i:i+3]]
                    a, b = [[v-u for u,v in zip(p, s)] for s in (q,r)]
                    cross = (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
                    assert sum(x*x for x in cross) > 1e-16, (record['id'], mesh['name'], i)
                vertices += len(positions)
                triangles += len(indices)//3
        assert changed > 0, 'must differ from old root-only squash'
    print(f'{len(records)} variants, {vertices} vertices, {triangles} finite nondegenerate triangles; original UV/material/image identity and crush collider envelopes verified')


if __name__ == '__main__':
    main()
