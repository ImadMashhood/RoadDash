#!/usr/bin/env python3
"""Serial native Hills polish/recovery evidence, isolated from user saves."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time
import sys
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'docs/testing/hills-polish'
BINARY = ROOT / 'target/debug/road-dash'
CASES = [
    ('active', 'hills', '0', ['--hills-polish-survey']),
    ('survey', 'hills', '0', ['--hills-survey']),
    ('west', 'hills', '0', ['--smoke-test']),
    ('east', 'hills', '1', ['--smoke-test']),
    ('baseline', 'hills', '1', ['--smoke-test', '--ragdoll-baseline']),
    ('metro-control', 'metro', '0', ['--smoke-test']),
]
OUT.mkdir(parents=True, exist_ok=True)
(OUT/'binary.json').write_text(json.dumps(dict(path=str(BINARY), sha256=hashlib.sha256(BINARY.read_bytes()).hexdigest(), profile='optimized debug'), indent=2)+'\n')
selected = set(sys.argv[1:])
unknown = selected - {case[0] for case in CASES}
if unknown:
    raise SystemExit(f'Unknown cases: {sorted(unknown)}')
rows = []
receipt = OUT / ('native-checks.json' if not selected else 'native-' + '-'.join(sorted(selected)) + '.json')
for name, track, route, args in CASES:
    if selected and name not in selected:
        continue
    folder = OUT/name
    folder.mkdir(exist_ok=True)
    binary_sha256 = hashlib.sha256(BINARY.read_bytes()).hexdigest()
    start = time.monotonic()
    with (folder/'native.txt').open('w') as log:
        result = subprocess.run([str(BINARY), *args], cwd=folder,
            env=dict(os.environ, ROADDASH_TRACK=track, ROADDASH_SMOKE_ROUTE=route,
                     ROADDASH_VERIFY_HILLS_RESCUE='1' if track == 'hills' and '--smoke-test' in args else '0'),
            stdout=log, stderr=subprocess.STDOUT, timeout=480)
    row = dict(extended_hills_rescue=track == 'hills' and '--smoke-test' in args, binary_sha256=binary_sha256, case=name, track=track, route=route, flags=args, exit_code=result.returncode, elapsed_seconds=time.monotonic()-start)
    rows.append(row)
    receipt.write_text(json.dumps(rows, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
