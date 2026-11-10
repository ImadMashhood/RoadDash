#!/usr/bin/env python3
"""Serial combined release views and behavior checks; no concurrent builds."""
import json, os, subprocess, time, sys
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=Path(__file__).resolve().parent
binary=root.parent/'RoadDash-worktrees/.next-pass-binary/road-dash'
env={**os.environ,'ROADDASH_TEST_BINARY':str(binary),'ROADDASH_BINARY':str(binary)}
checks=[(name,['python3',str(out/'run_native.py'),name]) for name in ['after','route','recovery','baseline-recovery','actors','police','police-baseline','poses','legacy-city-recovery','legacy-valley-recovery']]
checks += [('crops-'+mode,['python3',str(out/'run_crop_native.py'),mode]) for mode in ['off','on']]
checks += [('ragdoll-lab',[str(binary),'--ragdoll-lab','--output',str(out/'ragdoll-lab.json')])]
receipt_path=out/'native-checks.json'
receipts=json.loads(receipt_path.read_text()) if receipt_path.exists() else []
selected=set(sys.argv[1:])
for name,command in checks:
 if selected and name not in selected:continue
 if any(r['name']==name and r['exit']==0 for r in receipts):continue
 print('START '+name,flush=True)
 start=time.monotonic()
 with (out/(name+'-run.txt')).open('w') as log:
  r=subprocess.run(command,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
 receipts.append({'name':name,'command':command,'exit':r.returncode,'elapsed_seconds':round(time.monotonic()-start,3)})
 (out/'native-checks.json').write_text(json.dumps(receipts,indent=2)+'\n')
 print(f'END {name}: {r.returncode}',flush=True)
 if r.returncode:raise SystemExit(r.returncode)
