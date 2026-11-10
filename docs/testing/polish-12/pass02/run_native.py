#!/usr/bin/env python3
"""Serial isolated native contact/recovery regression receipts (not benchmarks)."""
import json, os, subprocess
from pathlib import Path
root = Path(__file__).resolve().parents[4]
out = Path(__file__).resolve().parent
binary = Path('/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash')
results = {}
for rig, flags in [('anatomy', []), ('baseline', ['--ragdoll-baseline'])]:
    for mode, args in [('recovery', ['--smoke-test']), ('actors', ['--actor-smoke-test'])]:
        name = rig + '-' + mode
        folder = out / name
        folder.mkdir(exist_ok=True)
        env = {**os.environ, 'DISPLAY': ':0', 'ROADDASH_TRACK': 'metro',
               'ROADDASH_SAVE_DIR': str(folder / 'save'),
               'ROADDASH_VERIFY_REPAIRS': '1', 'ROADDASH_VERIFY_REVERSE': '1',
               'ROADDASH_SMOKE_ROUTE': '0' if rig == 'anatomy' else '1'}
        (folder / 'processes-before.txt').write_text(subprocess.check_output(['ps','-eo','pid,comm,args'], text=True))
        command = [str(binary), *args, *flags]
        with (folder / 'native.txt').open('w') as log:
            run = subprocess.run(command, cwd=folder, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=240)
        results[name] = {'command':command, 'exit':run.returncode, 'route':env['ROADDASH_SMOKE_ROUTE']}
        (out / 'native-results.json').write_text(json.dumps(results, indent=2)+'\n')
        print(name, run.returncode, flush=True)
        if run.returncode:
            raise SystemExit(run.returncode)
with (out / 'lab.txt').open('w') as log:
    run = subprocess.run([str(binary), '--ragdoll-lab', '--output', str(out / 'lab.json')], cwd=root, stdout=log, stderr=subprocess.STDOUT, timeout=240)
print('paired lab', run.returncode, flush=True)
(out / 'lab-exit.txt').write_text(str(run.returncode)+'\n')
