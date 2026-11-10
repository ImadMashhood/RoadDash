#!/usr/bin/env python3
"""Serial final native cases with frozen content and isolated saves/settings."""
import hashlib,json,os,subprocess
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent;root=out.parents[2]
binary=Path('/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash')
cases=[('after',['--metro-survey'],{}),('shoulder',['--metro-survey'],{'ROADDASH_LANDSCAPE_VIEWS':'1'}),('recovery',['--smoke-test'],{'ROADDASH_SMOKE_ROUTE':'0'}),('recovery-baseline',['--smoke-test','--ragdoll-baseline'],{'ROADDASH_SMOKE_ROUTE':'1'}),('lod-city',['--lod-survey'],{'ROADDASH_TRACK':'city'}),('lod-valley',['--lod-survey'],{'ROADDASH_TRACK':'valley'}),('world-valley',['--world-survey'],{'ROADDASH_TRACK':'valley'})]
results={}
for name,args,extra in cases:
 folder=out/name;folder.mkdir(exist_ok=True)
 procs=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
 assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in procs.splitlines()[1:])
 (folder/'processes-before.txt').write_text(procs)
 frozen=[binary,root/'content/world/art.json',root/'content/world/metropolis.json',root/'assets/models/rider-bindings.json',root/'assets/models/motion.json']
 hashes=lambda:{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen}
 before=hashes();cmd=[str(binary),*args]
 env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn,road_dash::world_gen::metropolis::geometry=info','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_LOG_DIR':str(folder/'performance'),'ROADDASH_VERIFY_REPAIRS':'1','ROADDASH_VERIFY_REVERSE':'1','ROADDASH_VERIFY_STRIKES':'0','ROADDASH_VERIFY_CLUB':'0',**extra}
 with (folder/'native.txt').open('w') as log:r=subprocess.run(cmd,cwd=folder,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=300)
 shots=list((folder/'screenshots').rglob('*.png'));blank=[str(p.relative_to(folder)) for p in shots if all(hi-lo<2 for lo,hi in Image.open(p).convert('RGB').getextrema())]
 errors=[l for l in (folder/'native.txt').read_text(errors='replace').splitlines() if 'ERROR' in l or 'panicked at' in l]
 results[name]=dict(command=cmd,exit=r.returncode,images=len(shots),blank_images=blank,error_lines=errors,frozen_inputs=before,inputs_unchanged=hashes()==before)
 (out/'native-results.json').write_text(json.dumps(results,indent=2)+'\n');print(name,r.returncode,len(shots),flush=True)
 if (r.returncode and name!='world-valley') or not shots or blank or errors or hashes()!=before:raise SystemExit(1)
for name,args,env in [('lab',['--ragdoll-lab','--output',str(out/'lab.json')],{}),('inspect-world',['--inspect-world'],{'ROADDASH_TRACK':'valley'})]:
 cmd=[str(binary),*args]
 with (out/(name+'.txt')).open('w') as log:r=subprocess.run(cmd,cwd=root,env={**os.environ,**env},stdout=log,stderr=subprocess.STDOUT,timeout=240)
 results[name]=dict(command=cmd,exit=r.returncode);(out/'native-results.json').write_text(json.dumps(results,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:raise SystemExit(1)
