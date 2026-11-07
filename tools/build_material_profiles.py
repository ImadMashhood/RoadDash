#!/usr/bin/env python3
"""Index original palette PNGs by exact source SHA and decoded-pixel fingerprint."""
from pathlib import Path
from PIL import Image
import hashlib,json
root=Path(__file__).resolve().parents[1]
records=[];seen=set()
for path in sorted((root/'art/buildings/mesh_assets/materials').glob('*.png')):
    h=14695981039346656037
    for b in Image.open(path).convert('RGBA').tobytes():h=((h^b)*1099511628211)&0xffffffffffffffff
    key=f'{h:016x}'
    if key in seen:continue
    seen.add(key);name=path.name
    kind=next((kind for word,kind in [('brick','brick'),('timber','wood'),('siding','wood'),('wood','wood'),('stone','stone'),('metal','metal'),('plastic','plastic')]if word in name),'plaster')
    records.append(dict(source=str(path.relative_to(root)),source_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),pixel_fnv64=key,kind=kind,normal_strength=.32 if kind in ['brick','stone']else .14,roughness=.90 if kind in ['brick','stone','plaster']else .42 if kind in ['metal','plastic']else .82))
(root/'content/world/material_profiles.json').write_text(json.dumps(dict(schema_version=1,profiles=records),indent=2)+'\n')
print('Original palette profiles',len(records))
