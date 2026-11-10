#!/usr/bin/env python3
"""Native course handoff, landscape and matched city-shadow evidence (ADR138)."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'docs/testing/hills-delivery'
BINARY = ROOT / 'target/release/road-dash'
# case, track, route, arguments, shadow-distance control
CASES = [
    ('menu', 'metro', '0', ['--frontend-smoke'], 500),
    ('active', 'hills', '0', ['--hills-revision-survey'], 500),
    ('active-dynamic', 'hills', '0', ['--hills-revision-survey'], 500),
    ('west', 'hills', '0', ['--smoke-test'], 500),
    ('east', 'hills', '1', ['--smoke-test'], 500),
    ('baseline', 'hills', '1', ['--smoke-test', '--ragdoll-baseline'], 500),
    ('metro-control', 'metro', '0', ['--smoke-test'], 500),
    ('perf-near-a', 'hills', '0', ['--hills-revision-survey'], 150),
    ('perf-city-a', 'hills', '0', ['--hills-revision-survey'], 500),
    ('perf-city-b', 'hills', '0', ['--hills-revision-survey'], 500),
    ('perf-near-b', 'hills', '0', ['--hills-revision-survey'], 150),
]
selected = set(sys.argv[1:])
assert not selected - {c[0] for c in CASES}, 'unknown case'
OUT.mkdir(parents=True, exist_ok=True)
receipt = OUT / ('native-' + '-'.join(sorted(selected)) + '.json')
rows = []
for name, track, route, flags, distance in CASES:
    if selected and name not in selected:
        continue
    perf = name.startswith('perf-')
    if perf:
        processes = subprocess.check_output(['ps', '-eo', 'comm'], text=True).splitlines()
        assert not any(p.strip() in ('cargo', 'rustc', 'road-dash') for p in processes[1:]), 'another game/compiler invalidates timing'
    folder = OUT / name
    folder.mkdir(exist_ok=True)
    (folder / 'screenshots').mkdir(exist_ok=True)
    env = dict(os.environ, ROADDASH_TRACK=track, ROADDASH_SMOKE_ROUTE=route,
               ROADDASH_VEHICLE='assisted' if name == 'active' else 'dynamic',
               ROADDASH_HILLS_PERF='1' if perf else '0',
               ROADDASH_HILLS_DELIVERY='0' if perf else '1',
               ROADDASH_HILLS_CLOUDS='1', ROADDASH_HILLS_FIXED_BODIES='0',
               ROADDASH_SHADOW_DISTANCE_M=str(distance),
               ROADDASH_VERIFY_COURSES='1' if name == 'menu' else '0',
               ROADDASH_VERIFY_HILLS_RESCUE='1' if track == 'hills' else '0',
               ROADDASH_SETTINGS=str(folder / 'settings.json'),
               ROADDASH_SAVE_DIR=str(folder / 'save'),
               ROADDASH_LOG_DIR=str(folder / 'performance'),
               ROADDASH_DHT='0', ROADDASH_UPNP='0')
    for key in ('ROADDASH_ONLINE_BOOT', 'ROADDASH_TEST_RELOAD_MAP',
                'ROADDASH_OFFLINE_BOOT', 'ROADDASH_OFFLINE_STATE'):
        env.pop(key, None)
    sha = hashlib.sha256(BINARY.read_bytes()).hexdigest()
    started = time.time()
    with (folder / 'native.txt').open('w') as log:
        result = subprocess.run([str(BINARY), *flags], cwd=folder, env=env,
                                stdout=log, stderr=subprocess.STDOUT, timeout=600)
        if name == 'menu' and result.returncode == 0:
            report = folder / 'screenshots/course-selector-report.json'
            # The normal world-load boundary starts children and exits its parent.
            # Parent exit alone is not successful menu-to-race verification.
            deadline = time.monotonic() + 600
            while time.monotonic() < deadline:
                if report.exists() and report.stat().st_mtime >= started:
                    break
                if 'panicked at' in (folder / 'native.txt').read_text():
                    break
                time.sleep(1)
        elif '--hills-revision-survey' in flags:
            report = folder / 'screenshots/hills-revision.json'
        else:
            report = None
    complete = (report.exists() and report.stat().st_mtime >= started) if report else 'Render smoke test passed:' in (folder / 'native.txt').read_text()
    render_errors = bool(re.search(r"\bERROR\b|Shader compilation error|panicked at", (folder / "native.txt").read_text()))
    complete = complete and not render_errors
    row = dict(case=name, verification_complete=complete, render_errors=render_errors, exit_code=result.returncode,
               elapsed_seconds=time.time()-started, binary_sha256=sha,
               track=track, route=route, vehicle_adapter=env['ROADDASH_VEHICLE'],
               shadow_distance_m=distance, volumetric_clouds=True, performance=perf)
    rows.append(row)
    receipt.write_text(json.dumps(rows, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    if result.returncode or not complete:
        raise SystemExit(result.returncode or 1)
    if perf:
        log_path = re.search(r'Performance log: (\S+)', (folder / 'native.txt').read_text()).group(1)
        summary = subprocess.check_output([sys.executable, str(ROOT/'tools/summarize_performance.py'),
                                          log_path, '--skip-samples', '10', '--racing-only'], text=True)
        (folder / 'summary.json').write_text(summary)
