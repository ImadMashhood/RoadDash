#!/usr/bin/env python3
"""Locate unchanged facade pixels on the AI sheets. Reads pixels; never edits images.
UVs select body rectangles, excluding contact-sheet labels/background margins.
Bounds are cached and editable; source hashes prevent unnoticed art replacement.
"""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT/'art/buildings/city_preview/facade_assets.json'


def catalog():
    source = ROOT/'art/buildings/realistic_sheets'
    manifest = json.loads((source/'MANIFEST.json').read_text())
    assets = []
    for record in manifest['records']:
        if not record['file'].startswith('facades_'):
            continue
        path = source/record['file']
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != record['sha256']:
            raise ValueError(f'Art changed since reference manifest: {path}')
        with Image.open(path) as original:
            image = original.convert('RGB')
            width,height = image.size
            for i,identity in enumerate(record['ids']):
                col,row = i%3,i//3
                # Sheet headings occupy the first 10% of each half-row.
                left,right = int((col+.18)*width/3),int((col+.82)*width/3)
                top,bottom = int((row+.12)*height/2),int((row+.95)*height/2)
                points = [(x,y) for y in range(top,bottom) for x in range(left,right)
                          if max(image.getpixel((x,y))) > 85]
                if not points:
                    raise ValueError(f'No artwork found for {identity}')
                x0,x1=min(x for x,y in points),max(x for x,y in points)+1
                y0,y1=min(y for x,y in points),max(y for x,y in points)+1
                if x1-x0 < 65 or y1-y0 < 200:
                    raise ValueError(f'Inspect suspicious bounds for {identity}')
                # A masonry sample for hidden sides; no repainting or new image.
                samples=[image.getpixel((x,y)) for y in range(y0+10,min(y0+120,y1),5)
                         for x in range(x0+3,min(x0+15,x1),3)]
                color=[sorted(c[k] for c in samples)[len(samples)//2] for k in range(3)]
                assets.append(dict(id=identity,sheet=path.relative_to(ROOT).as_posix(),
                                   source_sha256=digest,pixels=[width,height],
                                   rect_px=[x0,y0,x1,y1],side_color=color,
                                   height_m=9.2 if identity.startswith(('BUSS','CHINA','WRHS','WHSE')) else 11.5))
    return dict(schema_version=1,source='Unchanged new AI-enhanced facade sheets; not original retail pixels',
                scope='Local authoring preview only; production media clearance remains unverified',
                uv_convention='Top-left origin, body bounds in source pixels; images remain byte-identical',
                assets=assets)


if __name__=='__main__':
    result=catalog();DEST.write_text(json.dumps(result,indent=2)+'\n')
    print(f"Cataloged {len(result['assets'])} actual facades across {len({a['sheet'] for a in result['assets']})} unchanged images")
