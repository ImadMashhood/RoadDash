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
parser.add_argument('--track', choices=['metro', 'hills', 'valley'], default='metro')
parser.add_argument('--frames', type=int, default=21600)
parser.add_argument('--segment', action='store_true', help='Synthetic final-segment planner diagnostic; not full-race completion')
parser.add_argument('--label', help='Distinct evidence folder; preserves earlier receipts')
parser.add_argument('--require-complete', action='store_true', help='Fail if the recorded race does not finish')
args = parser.parse_args()
if args.segment and args.require_complete:
    parser.error('--segment is diagnostic traversal, not --require-complete full-race acceptance')
if args.segment and args.case != 'race':
    parser.error('--segment requires --case race')
root = Path(__file__).resolve().parents[1]
folder = root / 'docs/testing/issue26-racer-ai' / (Path(args.label).name if args.label else 'native-' + args.case)
if folder.exists():
    parser.error(f'Evidence folder already exists; choose a unique --label: {folder}')
folder.mkdir(parents=True, exist_ok=False)
binary = args.binary.resolve()
env = dict(os.environ, ROADDASH_TRACK=args.track, ROADDASH_VEHICLE='dynamic',
           ROADDASH_AI_SURVEY_SEGMENT='1' if args.segment else '0',
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
                            stdout=log, stderr=subprocess.STDOUT, timeout=1800 if args.case == 'race' else 1200)
log = (folder/'native.txt').read_text()
errors = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
receipt_path = folder/'screenshots/racer-ai.json'
report = json.loads(receipt_path.read_text()) if args.case == 'race' and receipt_path.exists() else {}
# Earlier fixture binaries used Phase::Finished, which AI slot zero does not set.
# A real recorded slot-zero finish is authoritative even in those raw receipts.
completed = (report.get('completed', False) or 0 in report.get('finished_ids', [])) if args.case == 'race' else 'Render smoke test passed:' in log
segment_passed = report.get('segment_passed', False) if args.segment else None
survey_finished = bool(report) and (completed or segment_passed or report.get('frames', 0) >= args.frames)
receipt = dict(survey_finished=survey_finished, recorded_frames=report.get('frames'), case=args.case, track=args.track, synthetic_segment=args.segment, segment_passed=segment_passed, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
               exit_code=result.returncode, render_errors=errors, completed=completed,
               requested_frames=args.frames if args.case == 'race' else None,
               recorded_finished_ids=report.get('finished_ids', []),
               elapsed_seconds=time.time()-start,
               limits='Functional scripted Linux receipt, not an isolated frame-time benchmark or human approval.')
(folder/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps(receipt), flush=True)
required_result = segment_passed if args.segment else completed
if (args.case == 'race' and not survey_finished) or result.returncode or errors or ((args.segment or args.case != 'race' or args.require_complete) and not required_result):
    raise SystemExit(1)
