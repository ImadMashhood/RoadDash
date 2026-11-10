#!/usr/bin/env python3
"""Run isolated serial Hills/legacy native evidence; no user saves are written."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'docs/testing/hills'
BINARY = ROOT / 'target/debug/road-dash'
CASES = [
    ('survey', 'hills', '0', ['--hills-survey']),
    ('recovery-west', 'hills', '0', ['--smoke-test']),
    ('recovery-east', 'hills', '1', ['--smoke-test']),
    ('recovery-baseline', 'hills', '1', ['--smoke-test', '--ragdoll-baseline']),
    ('legacy-metro', 'metro', '0', ['--smoke-test']),
]
(OUTPUT / 'native-binary.json').write_text(json.dumps(dict(
    binary=str(BINARY), sha256=hashlib.sha256(BINARY.read_bytes()).hexdigest(),
    profile='debug optimized; no target-performance certification'), indent=2)+'\n')
results = []
for name, track, route, flags in CASES:
    folder = OUTPUT / name
    folder.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, ROADDASH_TRACK=track, ROADDASH_SMOKE_ROUTE=route)
    started = time.monotonic()
    with (folder / 'native.log').open('w') as log:
        result = subprocess.run([str(BINARY), *flags], cwd=folder, env=env,
                                stdout=log, stderr=subprocess.STDOUT, timeout=240)
    row = dict(case=name, track=track, route=route, flags=flags,
               exit_code=result.returncode, elapsed_seconds=time.monotonic()-started)
    results.append(row)
    (OUTPUT / 'native-checks.json').write_text(json.dumps(results, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
