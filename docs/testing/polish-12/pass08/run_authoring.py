#!/usr/bin/env python3
"""Serial district/source/clearance authoring receipts; never overlap a native run."""
import json, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent; root=out.parents[3]
commands=[(name,['/tmp/roaddash-layout-venv/bin/python','-m','unittest','discover','-s','tools','-p',f'test_metropolis_{name}.py']) for name in ['districts','facades','street_life','streetscape','population','elevation']]
commands.append(('atlas',['python3','tools/verify_facade_lod.py']))
results={}
for name,command in commands:
 with (out/(name+'.txt')).open('w') as log:r=subprocess.run(command,cwd=root,stdout=log,stderr=subprocess.STDOUT)
 results[name]=dict(command=command,exit=r.returncode)
 (out/'authoring-checks.json').write_text(json.dumps(results,indent=2)+'\n')
 print(name,r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
