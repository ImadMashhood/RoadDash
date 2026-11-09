#!/usr/bin/env python3
"""Bake bounded UI derivatives of the user's sixteen approved portraits."""
from pathlib import Path
import csv
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'output/portraits/approved-roster'
DEST = ROOT / 'assets/ui/portraits'

def main():
    manifest = json.loads((SOURCE / 'MANIFEST.json').read_text())
    entries = manifest['portraits']
    if len(entries) != 16 or len({p['character'] for p in entries}) != 16:
        raise ValueError('Expected sixteen unique approved portraits')
    DEST.mkdir(parents=True, exist_ok=True)
    intake = ROOT / 'docs/third_party/ASSETS.csv'
    with intake.open(newline='') as f:
        reader = csv.DictReader(f)
        fields = reader.fieldnames
        records = list(reader)
    records = [r for r in records if not r['asset_id'].startswith('ui-approved-portrait-')]
    packaged = []
    for entry in entries:
        source = SOURCE / entry['file']
        target = DEST / entry['file']
        with Image.open(source) as im:
            im = im.convert('RGB')
            # Contain the whole approved composition; do not stretch/crop square originals.
            im.thumbnail((768, 576), Image.Resampling.LANCZOS)
            canvas = Image.new('RGB', (768, 576), (28, 21, 32))
            canvas.paste(im, ((768-im.width)//2, (576-im.height)//2))
            canvas.save(target, optimize=True)
        digest = hashlib.sha256(target.read_bytes()).hexdigest()
        packaged.append(dict(entry, runtime=target.relative_to(ROOT).as_posix(), sha256=digest,
                             source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(), size=[768,576]))
        records.append(dict(asset_id='ui-approved-portrait-'+entry['character'],
            path=target.relative_to(ROOT).as_posix(), creator='RoadDash user / OpenAI image generation',
            source_url=source.relative_to(ROOT).as_posix(), license='MIT', license_copy='LICENSE',
            sha256=digest, modifications='User-approved concept; contained 768x576 UI derivative; no original retail pixels imported',
            attribution='Project-generated artwork; user-selected source; no exclusive-rights or legal-review assertion',
            redistribution='permitted', shipped='true'))
    with intake.open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader(); writer.writerows(records)
    (ROOT/'docs/ui/APPROVED_ROSTER.json').write_text(json.dumps(dict(date='2026-11-08',
        method='Built-in imagegen; ten user-selected attachments and six approved remakes',
        scope='Biker Bar concept browser; not simulation identities or playable selection',
        portraits=packaged), indent=2)+'\n')
    print('Packaged sixteen 768x576 portraits (27 MiB decoded RGB/RGBA runtime budget: 27 MiB RGBA).')

if __name__ == '__main__':
    main()
