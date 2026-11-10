#!/usr/bin/env python3
"""ADR139 native grid recovery and Metropolis road-edge evidence."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'docs/testing/hills-grid'
BINARY = ROOT / 'target/release/road-dash'
CASES = {
    'hills': ('hills', ['--smoke-test']),
    'baseline': ('hills', ['--smoke-test', '--ragdoll-baseline']),
    'metro': ('metro', ['--metro-survey']),
}
for name in sys.argv[1:]:
    track, flags = CASES[name]
    folder = OUT / ('native-' + name)
    folder.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, ROADDASH_TRACK=track, ROADDASH_VEHICLE='dynamic',
               ROADDASH_SMOKE_ROUTE='1' if name == 'baseline' else '0',
               ROADDASH_ROAD_EDGE_VIEWS='1', ROADDASH_VERIFY_HILLS_RESCUE='1',
               ROADDASH_SAVE_DIR=str(folder/'save'), ROADDASH_SETTINGS=str(folder/'settings.json'),
               ROADDASH_LOG_DIR=str(folder/'performance'), ROADDASH_DHT='0', ROADDASH_UPNP='0')
    for key in ('ROADDASH_OFFLINE_BOOT', 'ROADDASH_OFFLINE_STATE', 'ROADDASH_ONLINE_BOOT'):
        env.pop(key, None)
    start = time.time()
    with (folder/'native.txt').open('w') as log:
        result = subprocess.run([str(BINARY), *flags], cwd=folder, env=env,
                                stdout=log, stderr=subprocess.STDOUT, timeout=420)
    log = (folder/'native.txt').read_text()
    errors = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
    reports = [p for p in folder.rglob('*.json') if p.stat().st_mtime >= start and ('survey' in p.name or 'metro' in p.name)]
    complete = ('Render smoke test passed:' in log) if name != 'metro' else any(p.name == 'metro-survey.json' for p in reports)
    receipt = dict(case=name, binary_sha256=hashlib.sha256(BINARY.read_bytes()).hexdigest(),
                   exit_code=result.returncode, render_errors=errors, verified=complete and not errors and result.returncode == 0,
                   reports=[str(p.relative_to(folder)) for p in reports], elapsed_seconds=time.time()-start)
    (OUT/('receipt-'+name+'.json')).write_text(json.dumps(receipt, indent=2)+'\n')
    print(json.dumps(receipt), flush=True)
    if not receipt['verified']:
        raise SystemExit(1)
