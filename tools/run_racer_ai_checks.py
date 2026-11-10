#!/usr/bin/env python3
"""Issue26 isolated native receipts; explicit binary avoids shared-target replacement."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument('--binary', type=Path, required=True)
parser.add_argument('--case', choices=['race', 'recovery', 'baseline'], required=True)
parser.add_argument('--frames', type=int, default=21600)
parser.add_argument('--label', help='Distinct evidence folder; preserves earlier receipts')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
folder = root / 'docs/testing/issue26-racer-ai' / (Path(args.label).name if args.label else 'native-' + args.case)
folder.mkdir(parents=True, exist_ok=True)
binary = args.binary.resolve()
env = dict(os.environ, ROADDASH_TRACK='metro', ROADDASH_VEHICLE='dynamic',
           ROADDASH_AI_SURVEY_FRAMES=str(args.frames), ROADDASH_DHT='0', ROADDASH_UPNP='0',
           ROADDASH_SAVE_DIR=str(folder/'save'), ROADDASH_SETTINGS=str(folder/'settings.json'),
           ROADDASH_LOG_DIR=str(folder/'performance'), ROADDASH_SMOKE_ROUTE='0')
for key in ('ROADDASH_OFFLINE_BOOT', 'ROADDASH_OFFLINE_STATE', 'ROADDASH_ONLINE_BOOT'):
    env.pop(key, None)
flags = ['--racer-ai-survey'] if args.case == 'race' else ['--smoke-test']
if args.case == 'baseline':
    flags.append('--ragdoll-baseline')
start = time.time()
with (folder/'native.txt').open('w') as log:
    result = subprocess.run([str(binary), *flags], cwd=folder, env=env,
                            stdout=log, stderr=subprocess.STDOUT, timeout=1200)
log = (folder/'native.txt').read_text()
errors = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
receipt_path = folder/'screenshots/racer-ai.json'
report = json.loads(receipt_path.read_text()) if args.case == 'race' and receipt_path.exists() else {}
# Earlier fixture binaries used Phase::Finished, which AI slot zero does not set.
# A real recorded slot-zero finish is authoritative even in those raw receipts.
completed = (report.get('completed', False) or 0 in report.get('finished_ids', [])) if args.case == 'race' else 'Render smoke test passed:' in log
receipt = dict(case=args.case, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
               exit_code=result.returncode, render_errors=errors, completed=completed,
               requested_frames=args.frames if args.case == 'race' else None,
               recorded_finished_ids=report.get('finished_ids', []),
               elapsed_seconds=time.time()-start,
               limits='Functional scripted Linux receipt, not an isolated frame-time benchmark or human approval.')
(folder/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps(receipt), flush=True)
if result.returncode or errors or (args.case != 'race' and not completed):
    raise SystemExit(1)
