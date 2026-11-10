#!/usr/bin/env python3
"""Serial final native verification after the fresh release build completes."""
import json, subprocess, sys
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[3]
results={}
for mode in ['after-poses','anatomy-recovery','baseline-recovery','anatomy-actors','baseline-actors','npc-skin']:
    command=[sys.executable,str(out/'run_native.py'),mode]
    result=subprocess.run(command,cwd=root)
    results[mode]={'command':command,'exit':result.returncode}
    (out/'native-results.json').write_text(json.dumps(results,indent=2)+'\n')
    if result.returncode: raise SystemExit(result.returncode)
command=['/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash','--ragdoll-lab','--output',str(out/'lab.json')]
with (out/'lab.txt').open('w') as log:
    result=subprocess.run(command,cwd=root,stdout=log,stderr=subprocess.STDOUT,timeout=240)
results['paired-lab']={'command':command,'exit':result.returncode}
(out/'native-results.json').write_text(json.dumps(results,indent=2)+'\n')
print('paired-lab',result.returncode,flush=True)
raise SystemExit(result.returncode)
