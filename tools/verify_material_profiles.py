#!/usr/bin/env python3
"""Verify original material source provenance and decoded pixel identifiers."""
from pathlib import Path
from PIL import Image
import hashlib,json
root=Path(__file__).resolve().parents[1]
config=json.loads((root/'content/world/material_profiles.json').read_text());seen=set()
for p in config['profiles']:
    path=(root/p['source']).resolve();assert path.is_relative_to(root/'art/buildings/mesh_assets/materials')
    assert hashlib.sha256(path.read_bytes()).hexdigest()==p['source_sha256']
    h=14695981039346656037
    for b in Image.open(path).convert('RGBA').tobytes():h=((h^b)*1099511628211)&0xffffffffffffffff
    assert f'{h:016x}'==p['pixel_fnv64']and h not in seen;seen.add(h)
print('Verified',len(seen),'original palette profiles; no external media')
