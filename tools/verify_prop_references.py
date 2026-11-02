#!/usr/bin/env python3
"""Validate complete local review coverage without trusting allocation counts."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
from PIL import Image


def verify(folder):
    data = json.loads((folder / 'catalog.json').read_text())
    ids = [r['id'] for r in data['records']]
    if len(ids) != len(set(ids)) or len(ids) != data['asset_count']:
        raise ValueError('duplicate or missing asset IDs')
    if dict(Counter(r['group'] for r in data['records'])) != data['groups']:
        raise ValueError('group partition differs')
    source_keys = {(c['source'], c['offset']) for c in data['source_cells']}
    covered = set(); pixel_keys = {}; unresolved = 0; frames = 0
    for record in data['records']:
        if not record['aliases'] or not record['cells'] or not record['names']:
            raise ValueError('empty record')
        for c in record['cells']:
            pixel_keys[c['pixel_sha256']] = (c['width'], c['height'])
        for alias in record['aliases']:
            for leaf in alias['leaves']:
                for ref in leaf['references']:
                    covered.add((alias['source'], ref['cel_offset'])); frames += 1
                unresolved += len(leaf['non_cel_pixel_references'])
            if 'unresolved' in alias:
                covered.add((alias['source'], int(alias['source_identity'].rsplit(':', 1)[1])))
    if source_keys != covered:
        raise ValueError(f'unaccounted CELs: {len(source_keys - covered)}, foreign refs: {len(covered - source_keys)}')
    if len(source_keys) != data['scanned_cells'] or len(pixel_keys) != data['unique_images']:
        raise ValueError('source/image count mismatch')
    image_hashes = {}
    for digest, size in pixel_keys.items():
        path = folder / 'individual' / (digest + '.png')
        with Image.open(path) as im:
            im.verify()
        with Image.open(path) as im:
            if im.size != size or im.mode != 'RGBA':
                raise ValueError('image dimensions/mode mismatch')
        image_hashes[digest] = hashlib.sha256(path.read_bytes()).hexdigest()
    if set(p.stem for p in (folder / 'individual').glob('*.png')) != set(pixel_keys):
        raise ValueError('stale/missing extracted images')
    if 'sheets' in data:
        placed = [i for page in data['sheets'] for i in page['ids']]
        if Counter(placed) != Counter(ids):
            raise ValueError('sheet coverage differs')
        for page in data['sheets']:
            with Image.open(folder / page['file']) as im:
                if im.size != (1800, 940) or not 1 <= len(page['ids']) <= 6:
                    raise ValueError('invalid 3x2 sheet')
        for r in data['records']:
            sheet = next(p for p in data['sheets'] if p['file'] == r['sheet'])
            if sheet['ids'][r['sheet_slot'] - 1] != r['id']:
                raise ValueError('incorrect sheet slot')
    return dict(assets=len(ids), source_cells=len(source_keys), images=len(pixel_keys),
                decoded_frame_or_size_references=frames, non_cel_references_retained=unresolved,
                rejected_magic_candidates=len(data['rejected_magic_candidates']),
                sheets=len(data.get('sheets', [])), every_cel_accounted_for=True, image_sha256=image_hashes)


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('folder', type=Path); ap.add_argument('--output', type=Path); a = ap.parse_args()
    result = verify(a.folder)
    if a.output:
        a.output.parent.mkdir(parents=True, exist_ok=True); a.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({k: v for k, v in result.items() if k != 'image_sha256'}))
