#!/usr/bin/env python3
"""Persist built-in image generation results without losing concurrent completions."""
import fcntl
import hashlib
import json
import shutil
import sys
from pathlib import Path
from PIL import Image
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'art/props/concept_3d'

def record(batch, source):
    with (OUT / '.generation.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        path = OUT / 'ALL_REMAINING.json'
        data = json.loads(path.read_text())
        sheet = next(s for s in data['sheets'] if s['batch'] == batch)
        if source == '--reviewed':
            sheet['output_reviewed'] = True
            sheet['output_review_notes'] = 'Visually inspected subject/slot correspondence, full silhouettes, labels and source-specific assemblies. Draft art; final model/material approval remains open.'
            path.write_text(json.dumps(data, indent=2) + '\n')
            return
        with Image.open(source) as im:
            im.verify()
        dest = OUT / (batch + '.png')
        if dest.exists():
            raise FileExistsError(dest)
        shutil.copy2(source, dest)
        sheet.update(status='draft_not_approved', file=str(dest.relative_to(ROOT)),
                     sha256=hashlib.sha256(dest.read_bytes()).hexdigest(),
                     generator='built-in image_gen', output_reviewed=False)
        temp = path.with_suffix('.tmp')
        temp.write_text(json.dumps(data, indent=2) + '\n')
        temp.replace(path)
        print(batch + ' saved')

if __name__ == '__main__':
    record(sys.argv[1], sys.argv[2])
