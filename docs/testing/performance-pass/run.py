"""Isolated, matched native survey; never compile while measuring."""
import argparse,hashlib,json,os,subprocess,sys
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('binary',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
binary=a.binary.resolve();out=a.output.resolve();out.mkdir(parents=True,exist_ok=False)
processes=subprocess.check_output(['ps','-eo','pid,comm'],text=True)
assert not any(l.split()[1] in ('cargo','rustc','road-dash','blender') for l in processes.splitlines()[1:]),'compilation or another game would invalidate comparison'
(out/'processes.txt').write_text(processes)
(out/'tmp').mkdir()
env=dict(os.environ, TMPDIR=str(out/'tmp'),DISPLAY=':0',RUST_LOG='warn',ROADDASH_TRACK='metro',ROADDASH_CROSSING_SURVEY='1',ROADDASH_DHT='0',ROADDASH_UPNP='0',ROADDASH_SETTINGS=str(out/'settings.json'),ROADDASH_SAVE_DIR=str(out/'save'),ROADDASH_LOG_DIR=str(out/'performance'))
for k in ['ROADDASH_ONLINE_BOOT','ROADDASH_TEST_RELOAD_MAP']:env.pop(k,None)
with (out/'native.log').open('w') as f:r=subprocess.run([str(binary),'--metro-survey'],cwd=out,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=360)
logs=list((out/'performance').glob('*.jsonl'));assert r.returncode==0 and len(logs)==1
script=Path(__file__).resolve().parents[3]/'tools/summarize_performance.py'
report=json.loads(subprocess.check_output([sys.executable,str(script),str(logs[0]),'--skip-samples','10','--racing-only']))
(out/'summary.json').write_text(json.dumps(report,indent=2)+'\n')
(out/'receipt.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'exit':r.returncode,'command':[str(binary),'--metro-survey'],'environment':{k:v for k,v in env.items() if k.startswith('ROADDASH_') or k == 'RAYON_NUM_THREADS'},'limits':'Matched scripted native windows; manual60Hz one fixed step/frame, not a full-race/GPU/hardware certification.'},indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['samples','window_distributions_ms','weighted_fixed_step_ms']},indent=2))
