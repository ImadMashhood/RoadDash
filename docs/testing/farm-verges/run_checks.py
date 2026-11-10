#!/usr/bin/env python3
import json,os,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[2]
commands=[('fmt',['cargo','fmt','--all','--check']),('clippy',['cargo','clippy','--workspace','--locked','--all-targets','--','-D','warnings']),('tests',['cargo','test','--workspace','--locked']),('build',['cargo','build','--locked']),('audit',['python3','tools/audit.py']),('progress',['python3','tools/sync_progress.py','--check']),('roadmap',['python3','-m','unittest','discover','-s','tools','-p','test_roadmap.py'])]
results={}
for name,cmd in commands:
 with (out/(name+'.txt')).open('w') as log:r=subprocess.run(cmd,cwd=root,env={**os.environ,'CARGO_TARGET_DIR':'/home/imad/Documents/ChatGPT/RoadRash/target'},stdout=log,stderr=subprocess.STDOUT)
 results[name]={'command':cmd,'exit':r.returncode};(out/'checks.json').write_text(json.dumps(results,indent=2)+'\n');print(name,r.returncode,flush=True)
raise SystemExit(any(v['exit'] for v in results.values()))
