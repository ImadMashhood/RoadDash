#!/usr/bin/env python3
"""Functional recovery checks for dispatch experiments; never use these timings as benchmarks."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--track', choices=['hills', 'metro', 'both'], default='both')
    parser.add_argument('--fixed-executor', choices=['single', 'multi'], default='single')
    parser.add_argument('--production-defaults', action='store_true', help='Leave dispatch, loading admission and terrain reuse overrides unset.')
    args = parser.parse_args()
    binary = args.binary.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    receipts = []
    for track in (['hills', 'metro'] if args.track == 'both' else [args.track]):
        for baseline in [False, True]:
            case = f'{track}-' + ('baseline' if baseline else 'biomechanical')
            folder = output/case
            folder.mkdir()
            (folder/'screenshots').mkdir()
            env = {k:v for k,v in os.environ.items() if not k.startswith('ROADDASH_')}
            env.update(ROADDASH_TRACK=track, ROADDASH_FIXED_EXECUTOR=args.fixed_executor,
                       ROADDASH_LOAD_JOBS='8', ROADDASH_TERRAIN_CACHE='1',
                       ROADDASH_VEHICLE='dynamic', ROADDASH_RACE_SEED='25',
                       ROADDASH_SMOKE_ROUTE='1' if baseline and track == 'hills' else '0',
                       ROADDASH_VERIFY_HILLS_RESCUE='1' if track == 'hills' else '0',
                       ROADDASH_SAVE_DIR=str(folder/'save'), ROADDASH_SETTINGS=str(folder/'settings.json'),
                       ROADDASH_LOG_DIR=str(folder/'logs'), ROADDASH_DHT='0', ROADDASH_UPNP='0')
            if args.production_defaults:
                for key in ['ROADDASH_FIXED_EXECUTOR', 'ROADDASH_LOAD_JOBS', 'ROADDASH_TERRAIN_CACHE']:
                    env.pop(key)
            flags = ['--smoke-test'] + (['--ragdoll-baseline'] if baseline else [])
            start = time.monotonic()
            with (folder/'native.txt').open('w') as log:
                result = subprocess.run([str(binary), *flags], cwd=folder, env=env,
                                        stdout=log, stderr=subprocess.STDOUT, timeout=900)
            log = (folder/'native.txt').read_text()
            errors = bool(re.search(r'\bERROR\b|panicked at|Shader compilation error', log))
            complete = 'Render smoke test passed:' in log
            receipt = dict(case=case, exit_code=result.returncode, binary_sha256=digest, production_defaults=args.production_defaults,
                           flags=flags, environment={k:v for k,v in env.items() if k.startswith('ROADDASH_')},
                           complete=complete, render_errors=errors, duration_s=time.monotonic()-start,
                           timing_claim=False)
            receipts.append(receipt)
            (output/'receipts.json').write_text(json.dumps(receipts, indent=2)+'\n')
            print(json.dumps(receipt), flush=True)
            if result.returncode or errors or not complete:
                raise SystemExit('Native recovery failed; evidence retained')


if __name__ == '__main__':
    main()
