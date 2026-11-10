#!/usr/bin/env python3
"""Isolated revision evidence and same-binary collision-state A/B comparisons."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'docs/testing/hills-revision'
BINARY = ROOT / 'target/release/road-dash'
CASES = [
    ('active', 'hills', '0', ['--hills-revision-survey'], False, False),
    ('active-dynamic', 'hills', '0', ['--hills-revision-survey'], False, False),
    ('survey', 'hills', '0', ['--hills-survey'], False, False),
    ('west', 'hills', '0', ['--smoke-test'], False, False),
    ('east', 'hills', '1', ['--smoke-test'], False, False),
    ('baseline', 'hills', '1', ['--smoke-test', '--ragdoll-baseline'], False, False),
    ('metro-control', 'metro', '0', ['--smoke-test'], False, False),
    ('perf-fixed-a', 'hills', '0', ['--hills-revision-survey'], True, True),
    ('perf-static-a', 'hills', '0', ['--hills-revision-survey'], True, False),
    ('perf-nocloud-a', 'hills', '0', ['--hills-revision-survey'], True, False),
    ('perf-nocloud-b', 'hills', '0', ['--hills-revision-survey'], True, False),
    ('perf-static-b', 'hills', '0', ['--hills-revision-survey'], True, False),
    ('perf-fixed-b', 'hills', '0', ['--hills-revision-survey'], True, True),
]
selected = set(sys.argv[1:])
assert not selected - {c[0] for c in CASES}, 'unknown case'
OUT.mkdir(parents=True, exist_ok=True)
rows = []
receipt = OUT / ('native-checks.json' if not selected else 'native-' + '-'.join(sorted(selected)) + '.json')
for name, track, route, flags, perf, fixed in CASES:
    if selected and name not in selected:
        continue
    if perf:
        processes = subprocess.check_output(['ps', '-eo', 'comm'], text=True).splitlines()
        assert not any(p.strip() in ('cargo', 'rustc', 'road-dash') for p in processes[1:]), 'another compiler/game invalidates performance comparison'
    folder = OUT / name
    folder.mkdir(exist_ok=True)
    (folder / "screenshots").mkdir(exist_ok=True)
    env = dict(os.environ, ROADDASH_TRACK=track, ROADDASH_SMOKE_ROUTE=route,
               ROADDASH_VEHICLE='assisted' if name == 'active' else 'dynamic',
               ROADDASH_HILLS_PERF='1' if perf else '0',
               ROADDASH_HILLS_CLOUDS='0' if 'nocloud' in name else '1',
               ROADDASH_HILLS_FIXED_BODIES='1' if fixed else '0',
               ROADDASH_VERIFY_HILLS_RESCUE='1' if track == 'hills' else '0',
               ROADDASH_SETTINGS=str(folder/'settings.json'),
               ROADDASH_SAVE_DIR=str(folder/'save'),
               ROADDASH_LOG_DIR=str(folder/'performance'),
               ROADDASH_DHT='0', ROADDASH_UPNP='0')
    for key in ('ROADDASH_ONLINE_BOOT', 'ROADDASH_TEST_RELOAD_MAP'):
        env.pop(key, None)
    binary_hash = hashlib.sha256(BINARY.read_bytes()).hexdigest()
    started_wall = time.time()
    start = time.monotonic()
    with (folder/'native.txt').open('w') as log:
        result = subprocess.run([str(BINARY), *flags], cwd=folder, env=env,
                                stdout=log, stderr=subprocess.STDOUT, timeout=600)
    if '--hills-revision-survey' in flags:
        report_path = folder / 'screenshots/hills-revision.json'
        complete = report_path.exists() and report_path.stat().st_mtime >= started_wall
    elif '--hills-survey' in flags:
        report_path = folder / 'screenshots/hills-survey.json'
        complete = report_path.exists() and report_path.stat().st_mtime >= started_wall
    else:
        complete = 'Render smoke test passed:' in (folder / 'native.txt').read_text()
    row = dict(verification_complete=complete, case=name, track=track, route=route, flags=flags,
               vehicle_adapter=env["ROADDASH_VEHICLE"],
               performance=perf, fixed_body_control=fixed, volumetric_clouds='nocloud' not in name, binary_sha256=binary_hash,
               exit_code=result.returncode, elapsed_seconds=time.monotonic()-start)
    rows.append(row)
    receipt.write_text(json.dumps(rows, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    if result.returncode or not complete:
        raise SystemExit(result.returncode or 1)
    if perf:
        text = (folder/'native.txt').read_text()
        path = re.search(r'Performance log: (\S+)', text).group(1)
        report = subprocess.check_output([sys.executable, str(ROOT/'tools/summarize_performance.py'), path,
                                          '--skip-samples', '10', '--racing-only'], text=True)
        (folder/'summary.json').write_text(report)
