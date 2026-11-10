#!/usr/bin/env python3
"""Serial isolated native lighting/recovery regression receipts (not benchmarks)."""
import json, os, subprocess, time, sys
from pathlib import Path
root = Path(__file__).resolve().parents[4]
out = Path(__file__).resolve().parent
binary = Path('/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash')
# A deterministic authoring rebake may still be finishing. Never measure with
# compiler/baker work in flight; snapshots below record the quiescent start.
for _ in range(120):
    processes=subprocess.check_output(['ps','-eo','comm,args'],text=True).splitlines()
    busy=any(line.split(None,1)[0] in ('rustc','cargo') or
             (line.split(None,1)[0].startswith('python') and ' tools/bake_metropolis.py' in line)
             for line in processes if line.strip())
    if not busy:break
    time.sleep(2)
else:raise RuntimeError('compiler/baker did not become idle')
results = json.loads((out/'native-results.json').read_text()) if '--resume-recovery' in sys.argv else {}
for name,args in ([] if '--resume-recovery' in sys.argv else [('after',['--metro-survey'])]):
    folder=out/name
    folder.mkdir(exist_ok=True)
    command=[str(binary),*args]
    (folder/'processes-before.txt').write_text(subprocess.check_output(['ps','-eo','pid,comm,args'],text=True))
    with (folder/'native.txt').open('w') as log:
        run=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','ROADDASH_TRACK':'metro' if name=='after' else 'valley','ROADDASH_SAVE_DIR':str(folder/'save')},stdout=log,stderr=subprocess.STDOUT,timeout=240)
    results[name]={'command':command,'exit':run.returncode}
    (out/'native-results.json').write_text(json.dumps(results,indent=2)+'\n')
    print(name,run.returncode,flush=True)
    if run.returncode:
        report_path=folder/'screenshots/world-survey.json'
        report=json.loads(report_path.read_text()) if report_path.exists() else {}
        if not (name=='legacy-world' and run.returncode==1 and report.get('sections_captured')==15 and report.get('logical_buildings')==657 and report.get('p95_target_met') is False):
            raise SystemExit(run.returncode)
        print('Legacy world geometry assertions complete; performance target remains failed.',flush=True)
run=subprocess.run([str(binary),'--inspect-world','--output',str(out/'inspect.json')],cwd=root)
if run.returncode:raise SystemExit(run.returncode)

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
