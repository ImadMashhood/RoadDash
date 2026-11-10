import json,os,subprocess,time
from pathlib import Path
out=Path(__file__).resolve().parent
root=out.parents[2]
checks=[('fmt',['cargo','fmt','--all','--check']),('clippy',['cargo','clippy','--workspace','--locked','--all-targets','--','-D','warnings']),('tests',['cargo','test','--workspace','--locked']),('build',['cargo','build','--locked']),('model-parity',['cargo','test','--locked','models::','--','--nocapture']),('audit',['python3','tools/audit.py']),('progress',['python3','tools/sync_progress.py','--check']),('roadmap',['python3','-m','unittest','tools.test_roadmap'])]
results=[]
for name,cmd in checks:
 start=time.monotonic()
 with (out/(name+'.txt')).open('w') as f:
  r=subprocess.run(cmd,cwd=root,env={**os.environ,'CARGO_TARGET_DIR':'/home/imad/Documents/ChatGPT/RoadRash/target'},stdout=f,stderr=subprocess.STDOUT)
 results.append({'name':name,'command':cmd,'exit':r.returncode,'seconds':round(time.monotonic()-start,3)})
 (out/'checks.json').write_text(json.dumps(results,indent=2)+'\n')
 print(name,r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
