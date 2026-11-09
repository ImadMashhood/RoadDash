#!/usr/bin/env python3
"""Check recorded original hand export/capture provenance without Blender."""
import hashlib
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT/'docs/testing/part02-authoring'
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    report = json.loads((BASE/'roundtrip.json').read_text())
    assert report['source_sha256'] == {p: digest(ROOT/p) for p in report['source_sha256']}, 'stale source recipe'
    assert report['profile_sha256'] == digest(ROOT/'content/hand-contacts.json')
    assert report['assembly_states'] == 154 and len(report['captures']) == 18
    assert report['source_socket_frame_error_m'] < 1e-6
    assert max(report['glb_surface_error_m'].values()) < 1e-5
    assert report['glb_socket_error_m'] < 1e-5 and report['glb_socket_rotation_error'] < 1e-5
    assert report['glb_uv_error'] < 1e-6
    for capture in report['captures']:
        assert digest(BASE/capture['file']) == capture['sha256'], capture['file']
    assert not report['physics_changed'] and not report['user_approved']
    assert report['no_animation_export']
    fixture = json.loads((BASE/'fold-fixtures.json').read_text())
    assert fixture['profile_sha256'] == report['profile_sha256'] and len(fixture['samples']) == 20
    print('18 original authoring views,154 hand states, source hashes and GLB reload measurements verified')
if __name__ == '__main__':
    main()
