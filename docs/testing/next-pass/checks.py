#!/usr/bin/env python3
"""Run serialized integrated checks; fails at the first unmet gate."""
import json, os, subprocess, time
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=Path(__file__).resolve().parent
scratch=root.parent/'RoadDash-worktrees/.next-pass-build'
scratch.mkdir(parents=True, exist_ok=True)
env={**os.environ,'CARGO_TARGET_DIR':str(root/'target'),'TMPDIR':str(scratch)}
checks=[
 ('fmt',['cargo','fmt','--all','--check']),
 ('clippy',['cargo','clippy','--workspace','--locked','--all-targets','--','-D','warnings']),
 ('tests',['cargo','test','--workspace','--locked']),
 ('build',['cargo','build','--locked']),
 ('release',['cargo','build','--release','--locked']),
 ('audit',['python3','tools/audit.py']),
 ('progress',['python3','tools/sync_progress.py','--check']),
]
receipts=[]
for name, command in checks:
 print('START '+name,flush=True)
 start=time.monotonic()
 with (out/(name+'.txt')).open('w') as log:
  r=subprocess.run(command,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
 receipts.append({'name':name,'command':command,'exit':r.returncode,'elapsed_seconds':round(time.monotonic()-start,3)})
 (out/'checks.json').write_text(json.dumps(receipts,indent=2)+'\n')
 print(f'END {name}: {r.returncode}',flush=True)
 if r.returncode: raise SystemExit(r.returncode)
