"""Issue #28 reproducible native fixtures; captures need separate visual review."""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--only', choices=['gait', 'reverse', 'recovery', 'lab'])
    args = parser.parse_args()
    binary = Path(args.binary).resolve()
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=False)
    cases = []
    for terrain in ['flat', 'up', 'down', 'cross', 'uneven']:
        cases.append(('gait', terrain, ['--animation-lab-view', '--animation-lab-smoke'],
                      dict(ROADDASH_GAIT_REVIEW='1', ROADDASH_GAIT_TERRAIN=terrain)))
    for adapter in ['dynamic', 'assisted']:
        cases.append(('reverse', adapter, ['--smoke-test'],
                      dict(ROADDASH_REVERSE_REVIEW='1', ROADDASH_VEHICLE=adapter)))
    for rig in ['anatomy', 'baseline']:
        cases.append(('recovery', rig, ['--smoke-test'] + (['--ragdoll-baseline'] if rig == 'baseline' else []),
                      dict(ROADDASH_VERIFY_WALK_CROSSINGS='1', ROADDASH_VERIFY_MANUAL_RETURN='1',
                           ROADDASH_SMOKE_ROUTE='0')))
    cases.append(('lab', 'paired', ['--ragdoll-lab', '--output', 'paired.json'], {}))
    receipt = dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), cases=[],
                   limits='Scripted Linux native fixtures; visual review is separate. No GPU/performance/hardware or human feel certificate.')
    for group, name, argv, extra in cases:
        if args.only and args.only != group:
            continue
        case = output / f'{group}-{name}'
        case.mkdir()
        env = {k: v for k, v in os.environ.items() if not k.startswith('ROADDASH_')}
        env.update(ROADDASH_TRACK='metro', ROADDASH_RACE_SEED='1234',
                   ROADDASH_SAVE_DIR=str(case / 'save'), ROADDASH_SETTINGS=str(case / 'settings.json'),
                   ROADDASH_LOG_DIR=str(case / 'logs'), RUST_LOG='warn', **extra)
        row = dict(group=group, name=name, args=argv, environment=extra, passed=False)
        with (case / 'native.log').open('w') as log:
            try:
                result = subprocess.run([str(binary), *argv], cwd=case, env=env,
                                        stdout=log, stderr=subprocess.STDOUT, timeout=600, check=False)
                row['exit_code'] = result.returncode
                if result.returncode:
                    raise ValueError(f'Exit {result.returncode}')
                log.flush()
                native_log = re.sub(r'\x1b\[[0-9;]*m', '', (case / 'native.log').read_text())
                assert not (re.search(r'\bERROR\b|panicked at', native_log) or
                            re.search(r'(?:shader|wgsl).*?(?:error|failed)', native_log, re.IGNORECASE)), \
                    'Native log contains error/panic/shader failure'
                if group == 'gait':
                    row['frames'] = len(list((case / 'screenshots').glob('gait-*.png')))
                    assert row['frames'] >= 145, 'Missing gait sequence'
                elif group == 'reverse':
                    data = json.loads((case / 'screenshots/reverse-receipt.json').read_text())
                    row['samples'] = len(data['samples'])
                    assert row['samples'] > 80, 'Missing reverse cases'
                    assert {s['key'] for s in data['samples']} == {'KeyA', 'KeyD'}
                elif group == 'recovery':
                    data = json.loads((case / 'screenshots/smoke-metrics.json').read_text())
                    assert data['manual_interruption_and_auto_resume'] == [True, True]
                    assert data['walk_crossing_setups'] == [True, True]
                    assert all(data['fixed_recovery_stages'])
                    row['walk_crossing_distances_m'] = data['walk_crossing_distances_m']
                else:
                    data = json.loads((case / 'paired.json').read_text())
                    assert len(data) == 18 and all(s['finite'] for s in data)
                    row['paired_cases'] = len(data)
                row['passed'] = True
            except (subprocess.TimeoutExpired, OSError, ValueError, KeyError, AssertionError) as error:
                row['error'] = str(error)
        receipt['cases'].append(row)
        (output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
        print(json.dumps(row), flush=True)
        if not row['passed']:
            return 1
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
