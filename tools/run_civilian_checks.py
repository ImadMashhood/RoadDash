#!/usr/bin/env python3
"""Isolated issue27 native motion/recovery receipts; never measures GPU timing."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--binary', type=Path, required=True)
parser.add_argument('--timeout-seconds', type=int, default=600, help='Wall-clock bound per case; use 1800 for extended diagnostics under compilation load')
parser.add_argument('--output-root', type=Path, default=ROOT/'docs/testing/civilian-overtaking')
parser.add_argument('cases', nargs='+', choices=['hills', 'metro', 'fleet-hills', 'fleet-metro', 'active', 'baseline'])
args = parser.parse_args()
binary = args.binary.resolve()
digest = hashlib.sha256(binary.read_bytes()).hexdigest()
for name in args.cases:
    folder = args.output_root.resolve() / ('native-' + name)
    if folder.exists():
        raise SystemExit(f'Preserve existing or interrupted evidence: {folder}; choose a new --output-root')
    folder.mkdir(parents=True)
    smoke = name in ('active', 'baseline')
    track = 'metro' if name in ('metro', 'fleet-metro') else 'hills'
    flags = ['--smoke-test'] if smoke else ['--civilian-survey']
    if name == 'baseline':
        flags += ['--ragdoll-baseline']
    env = dict(os.environ, ROADDASH_TRACK=track, ROADDASH_VEHICLE='dynamic',
               ROADDASH_SMOKE_ROUTE='1' if name == 'baseline' else '0',
               ROADDASH_VERIFY_HILLS_RESCUE='1',
               ROADDASH_SAVE_DIR=str(folder/'save'), ROADDASH_SETTINGS=str(folder/'settings.json'),
               ROADDASH_LOG_DIR=str(folder/'performance'), ROADDASH_DHT='0', ROADDASH_UPNP='0',
               ROADDASH_CIVILIAN_FLEET='1' if name.startswith('fleet-') else '0')
    for key in ('ROADDASH_OFFLINE_BOOT', 'ROADDASH_OFFLINE_STATE', 'ROADDASH_ONLINE_BOOT'):
        env.pop(key, None)
    started = time.monotonic()
    with (folder/'native.txt').open('w') as log:
        try:
            result = subprocess.run([str(binary), *flags], cwd=folder, env=env,
                                    stdout=log, stderr=subprocess.STDOUT, timeout=max(1, args.timeout_seconds))
            code = result.returncode
        except subprocess.TimeoutExpired:
            code = -1
    log = (folder/'native.txt').read_text()
    errors = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
    report_name = 'civilian-fleet.json' if name.startswith('fleet-') else 'civilian-overtaking.json'
    complete = 'Render smoke test passed:' in log if smoke else False
    report_error = None
    if not smoke:
        try:
            report = json.loads((folder/'screenshots'/report_name).read_text())
            timing = report['planning_cpu']
            assert timing['ticks'] > 0
            if name.startswith('fleet-'):
                assert report['vehicles'] > 0 and report['race_elapsed_seconds'] >= 119
                assert report['initiated'] >= report['completed'] + report['aborted_returns']
                for actor in report.get('outstanding', []):
                    assert actor['age_seconds'] < 45, f"stranded claim: {actor}"
            else:
                assert len(report['cases']) == 4
                for case in report['cases']:
                    assert case['passed'] and case['max_blend'] > 0.95
                    assert case['phase'] == 'Follow' and case['final_blend'] == 0
                    assert case['collider_intersections'] == 0 and case['support_error_max_m'] < 0.02
            complete = True
        except (OSError, ValueError, KeyError, TypeError, AssertionError) as error:
            report_error = f'{type(error).__name__}: {error}'
    receipt = dict(case=name, binary_sha256=digest, exit_code=code, render_errors=errors,
                   verified=complete and not errors and code == 0, elapsed_seconds=time.monotonic()-started,
                   report_error=report_error, timeout_seconds=max(1, args.timeout_seconds),
                   limits='Scripted native functional motion/recovery evidence; no GPU/frame-budget or human/full-race acceptance.')
    (folder/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print(json.dumps(receipt), flush=True)
    if not receipt['verified']:
        raise SystemExit(1)
