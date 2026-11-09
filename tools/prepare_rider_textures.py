#!/usr/bin/env python3
"""Extract only our authored GLB atlases; neutralize red zones for roster tinting."""
import io
import json
from pathlib import Path
import struct
from PIL import Image
ROOT = Path(__file__).resolve().parents[1]
raw = (ROOT / 'art/source/rider.glb').read_bytes()
length = struct.unpack_from('<I', raw, 12)[0]
model = json.loads(raw[20:20+length])
binary = raw[28+length:]
for descriptor in model['images']:
    name = descriptor['name']
    view = model['bufferViews'][descriptor['bufferView']]
    start = view.get('byteOffset', 0)
    data = binary[start:start+view['byteLength']]
    if name == 'Rider_BaseColor':
        im = Image.open(io.BytesIO(data)).convert('RGB')
        pixels = im.get_flattened_data() if hasattr(im, 'get_flattened_data') else im.getdata()
        im.putdata([(min(255, int(r*2.6)),)*3 if r > g*1.6 and r > b*1.15 else (r,g,b) for r,g,b in pixels])
        im.save(ROOT/'assets/textures/rider-base.png')
    elif name in ['Rider_Normal', 'Rider_Roughness']:
        suffix = 'normal' if name == 'Rider_Normal' else 'roughness'
        (ROOT/f'assets/textures/rider-{suffix}.png').write_bytes(data)
print('Prepared three original rider atlases from art/source/rider.glb')
