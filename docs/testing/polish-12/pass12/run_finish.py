#!/usr/bin/env python3
"""Run only after mandatory checks complete; no compile/native overlap."""
import json,os,subprocess,sys,time
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[3]
env={**os.environ,'CARGO_TARGET_DIR':'/home/imad/Documents/ChatGPT/RoadRash/target'}
results={}
for name,command in [
 ('models',['cargo','test','--locked','models::tests','--','--nocapture']),
 ('release-build',['cargo','build','--release','--locked'])]:
 started=time.monotonic()
 with (out/(name+'.txt')).open('w') as log:
  result=subprocess.run(command,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
 results[name]={'command':command,'exit':result.returncode,'wall_seconds':time.monotonic()-started}
 (out/'finish-results.json').write_text(json.dumps(results,indent=2)+'\n')
 print(name,result.returncode,flush=True)
 if result.returncode:raise SystemExit(result.returncode)
raise SystemExit(subprocess.run([sys.executable,str(out/'run_remaining.py')],cwd=root).returncode)
