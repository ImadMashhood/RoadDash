#!/usr/bin/env python3
"""Isolated native guardrail/recovery evidence; functional checks, not timing."""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
CASES = {
    'gaps': ['--guardrail-survey'],
    'valley-gaps': ['--legacy-guardrail-survey'],
    'valley-recovery': ['--smoke-test'],
    'west-current': ['--smoke-test'],
    'east-current': ['--smoke-test'],
    'west-baseline': ['--smoke-test', '--ragdoll-baseline'],
    'east-baseline': ['--smoke-test', '--ragdoll-baseline'],
}
parser = argparse.ArgumentParser()
parser.add_argument('cases', nargs='*', choices=CASES)
parser.add_argument('--binary', type=Path, default=ROOT/'target/release/road-dash')
parser.add_argument('--out', type=Path, default=ROOT/'docs/testing/guardrails/native')
args = parser.parse_args()
binary = args.binary.resolve()
sha = hashlib.sha256(binary.read_bytes()).hexdigest()
for case in args.cases or CASES:
    folder = (args.out/case).resolve()
    (folder/'screenshots').mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, ROADDASH_TRACK='valley' if case.startswith('valley') else 'hills', ROADDASH_VEHICLE='dynamic',
               ROADDASH_SMOKE_ROUTE='1' if case.startswith('east') else '0',
               ROADDASH_VERIFY_HILLS_RESCUE='1', ROADDASH_DHT='0', ROADDASH_UPNP='0',
               ROADDASH_SETTINGS=str(folder/'settings.json'), ROADDASH_SAVE_DIR=str(folder/'save'),
               ROADDASH_LOG_DIR=str(folder/'performance'))
    for key in ('ROADDASH_ONLINE_BOOT', 'ROADDASH_TEST_RELOAD_MAP', 'ROADDASH_OFFLINE_BOOT', 'ROADDASH_OFFLINE_STATE'):
        env.pop(key, None)
    with open('/tmp/roaddash-native.lock', 'w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        started = time.time()
        with (folder/'native.txt').open('w') as log:
            result = subprocess.run([str(binary), *CASES[case]], cwd=folder, env=env,
                                    stdout=log, stderr=subprocess.STDOUT, timeout=600)
    log = (folder/'native.txt').read_text(errors='replace')
    error = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
    marker = 'GUARDRAIL_SURVEY_COMPLETE' if case.endswith('gaps') else 'Render smoke test passed:'
    complete = result.returncode == 0 and (case.endswith('gaps') or marker in log) and not error
    row = dict(case=case, completed=complete, exit_code=result.returncode, errors=error,
               binary_sha256=sha, functional_only=True, elapsed_seconds=time.time()-started)
    if complete and case.endswith('gaps'):
        data = json.loads((folder/'screenshots/guardrails-report.json').read_text())
        captures = list((folder/'screenshots').glob('guardrails-*.png'))
        complete = data['physical_foot_crossings'] == 2 and data['rescues'] == 0 and len(captures) == 4
        row.update(completed=complete, physical_foot_crossings=data['physical_foot_crossings'],
                   grounded_ticks=data['grounded_ticks'], screenshots=len(captures))
    elif complete:
        data = json.loads((folder/'screenshots/smoke-metrics.json').read_text())
        row['fixed_recovery_stages'] = data['fixed_recovery_stages']
        row['exceptional_recovery'] = data['exceptional_recovery']
        complete = all(data['fixed_recovery_stages'])
        row['completed'] = complete
    (folder/'receipt.json').write_text(json.dumps(row, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    if not complete:
        raise SystemExit(1)
