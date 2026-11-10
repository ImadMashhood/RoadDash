#!/usr/bin/env python3
"""Original patrol uniform, baked onto existing UVs; no mesh/rig edits or media intake."""
import hashlib
import json
from pathlib import Path
import struct
from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parents[1]
SIZE = 1024
raw = (ROOT / 'assets/models/riding.rdmesh').read_bytes()
n = struct.unpack_from('<I', raw, 8)[0]
header = json.loads(raw[12:12+n])
offset = 12+n
source = Image.open(ROOT / 'assets/textures/rider-base.png').convert('RGB').resize((SIZE, SIZE))
image = source.copy()
pixels = image.load()
for record in header['records']:
    vertices = [struct.unpack_from('<12f4H', raw, offset+i*56) for i in range(record['vertices'])]
    offset += record['vertices']*56
    indices = struct.unpack_from('<'+'I'*record['indices'], raw, offset)
    offset += record['indices']*4
    if record['group'] != 'standing':
        continue
    region = record['material']['region']
    for start in range(0, len(indices), 3):
        a,b,c = [vertices[i] for i in indices[start:start+3]]
        uv = [(p[6]*(SIZE-1), p[7]*(SIZE-1)) for p in (a,b,c)]
        (ax,ay),(bx,by),(cx,cy) = uv
        den = (by-cy)*(ax-cx)+(cx-bx)*(ay-cy)
        if abs(den) < 1e-8:
            continue
        for py in range(max(0,int(min(ay,by,cy))), min(SIZE,int(max(ay,by,cy))+2)):
            for px in range(max(0,int(min(ax,bx,cx))), min(SIZE,int(max(ax,bx,cx))+2)):
                u=((by-cy)*(px-cx)+(cx-bx)*(py-cy))/den
                v=((cy-ay)*(px-cx)+(ax-cx)*(py-cy))/den
                w=1-u-v
                if min(u,v,w) < -0.01:
                    continue
                x,y,z=[u*a[i]+v*b[i]+w*c[i] for i in range(3)]
                color=None
                if region=='shirt':
                    color=(28,47,69)
                    # Silver shoulder yoke and waist band wrap the front and back.
                    if 1.58 < y < 1.66 or 1.17 < y < 1.205:
                        color=(204,216,218)
                    # Original gold shield, no real department insignia.
                    if z>.09 and 1.40<y<1.51 and abs(x-.105)<(.035 if y>1.44 else (y-1.40)*.875):
                        color=(222,178,65)
                    if z>.10 and abs(x)<.008 and 1.22<y<1.57:
                        color=(120,139,150)
                elif region=='helmet':
                    color=(224,231,226)
                    if 1.94<y<1.99:
                        color=(24,62,105)
                    if abs(x)<.025 and z>.12:
                        color=(222,178,65)
                elif region=='body' and .50<y<1.18:
                    color=(25,36,49)
                if color:
                    grain=((px*17+py*31)%7-3)*.65
                    pixels[px,py]=tuple(max(0,min(255,int(q+grain))) for q in color)
# Smooth single-texel raster edges; existing source island padding is retained.
image=image.filter(ImageFilter.MedianFilter(3))
out=ROOT/'assets/textures/officer-base.png'
image.save(out)
manifest=dict(author='RoadDash original procedural artwork',source='tools/bake_police_uniform.py',license='MIT (project-original)',inputs={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [ROOT/'assets/models/riding.rdmesh',ROOT/'assets/textures/rider-base.png']},sha256=hashlib.sha256(out.read_bytes()).hexdigest(),notes='UV-only patrol navy cloth, silver yoke/belt, white striped helmet and generic gold shield. Existing normal/roughness/skin retained; no department logo or external media.')
(ROOT/'assets/textures/officer-provenance.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(out)
