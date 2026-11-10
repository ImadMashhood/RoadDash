#!/usr/bin/env python3
"""Recompile only material JSON from original model sources; preserve BIN/geometry.
Equivalent to extract_front_modules + build_architecture_kit, without redoing
unchanged clipping. Rebake facade LOD and Metropolis crops after running this.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path
from verify_building_mesh_assets import read_glb
from facade_material_policy import coordinate, FAMILIES, ROOT

KIT = ROOT/'art/buildings/mesh_assets'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()


def rewrite(path, materials):
    doc, binary = read_glb(path)
    old = sha(path)
    geometry = {k:v for k,v in doc.items() if k != 'materials'}
    doc['materials'] = [materials[m['name']] for m in doc['materials']]
    # Texture indices belong to the derived document, not the original model.
    original, _ = read_glb(path)
    for target, source in zip(doc['materials'], original['materials']):
        target = target['pbrMetallicRoughness']
        if 'baseColorTexture' in source['pbrMetallicRoughness']:
            target['baseColorTexture'] = source['pbrMetallicRoughness']['baseColorTexture']
    header = json.dumps(doc,separators=(',',':')).encode()
    header += b' '*(-len(header)%4)
    path.write_bytes(struct.pack('<4sII',b'glTF',2,28+len(header)+len(binary))+
                     struct.pack('<II',len(header),0x4e4f534a)+header+
                     struct.pack('<II',len(binary),0x004e4942)+binary)
    actual, actual_binary = read_glb(path)
    assert actual_binary == binary
    assert {k:v for k,v in actual.items() if k != 'materials'} == geometry
    return dict(path=str(path.relative_to(ROOT)),before_sha256=old,after_sha256=sha(path),
                unchanged_binary_sha256=hashlib.sha256(binary).hexdigest(),
                unchanged_geometry_json_sha256=hashlib.sha256(json.dumps(geometry,sort_keys=True).encode()).hexdigest())


def run(output):
    manifests = {name:json.loads((KIT/name/'MANIFEST.json').read_text()) for name in ('front_modules','storey_kit')}
    records = {name:{r['id']:r for r in m['records']} for name,m in manifests.items()}
    previous={r["path"]:r for r in json.loads(output.read_text())["records"]} if output.exists() else {}
    receipts=[]
    for identity in sorted(FAMILIES):
        for lod in range(3):
            filename=f'{identity}_lod{lod}.glb'
            source,_=read_glb(KIT/'models'/filename)
            for name in ('front_modules','storey_kit'):
                materials={m['name']:coordinate(identity,m) for m in source['materials']}
                path=KIT/name/filename
                receipt=rewrite(path,materials)
                if receipt['path'] in previous:
                    prior=previous[receipt['path']]
                    assert prior['unchanged_binary_sha256']==receipt['unchanged_binary_sha256']
                    assert prior['unchanged_geometry_json_sha256']==receipt['unchanged_geometry_json_sha256']
                    receipt['before_sha256']=prior['before_sha256']
                receipts.append(receipt)
                record=records[name][identity]['lods'][lod]
                record.update(sha256=sha(path),bytes=path.stat().st_size)
                if name=='storey_kit':record['source_sha256']=sha(KIT/'front_modules'/filename)
    for name,m in manifests.items():
        (KIT/name/'MANIFEST.json').write_text(json.dumps(m,indent=2)+'\n')
    output.write_text(json.dumps(dict(material_policy_sha256=sha(ROOT/'tools/facade_material_policy.py'),families=sorted(FAMILIES),geometry_preserved=True,records=receipts),indent=2)+'\n')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--receipt',type=Path,required=True)
    run(parser.parse_args().receipt)
