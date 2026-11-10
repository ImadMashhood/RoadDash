#!/usr/bin/env python3
"""Final serial integrated native regression; never modify user saves/settings."""
import hashlib,json,os,subprocess,shutil
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent;root=out.parents[3]
binary=out/'.run-binary'
shutil.copy2('/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash', binary)
cases=[('metro-repeat',['--metro-survey'],{})]
results={}
for name,args,extra in cases:
 folder=out/name;folder.mkdir(exist_ok=True)
 processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
 assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:]),'competing native/build process'
 (folder/'processes-before.txt').write_text(processes)
 files=[binary,root/'content/world/art.json',root/'content/world/metropolis.json',root/'assets/shaders/metropolis_facade.wgsl',root/'assets/models/rider-bindings.json',root/'assets/models/motion.json']
 hashes=lambda:{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
 before=hashes();command=[str(binary),*args]
 env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn,road_dash::sky=info','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_LOG_DIR':str(folder/'performance'),'ROADDASH_VERIFY_REPAIRS':'1','ROADDASH_VERIFY_REVERSE':'1','ROADDASH_VERIFY_STRIKES':'0','ROADDASH_VERIFY_CLUB':'0',**extra}
 with (folder/'native.txt').open('w') as log:r=subprocess.run(command,cwd=folder,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=300)
 shots=list((folder/'screenshots').rglob('*.png'));blank=[str(p.relative_to(folder)) for p in shots if all(hi-lo<2 for lo,hi in Image.open(p).convert('RGB').getextrema())]
 errors=[line for line in (folder/'native.txt').read_text(errors='replace').splitlines() if 'ERROR' in line or 'panicked at' in line]
 results[name]={'command':command,'exit':r.returncode,'captured_images':len(shots),'blank_images':blank,'error_lines':errors,'frozen_inputs':before,'inputs_unchanged':hashes()==before}
 (out/'native-repeat-results.json').write_text(json.dumps(results,indent=2)+'\n');print(name,r.returncode,len(shots),'images',flush=True)
 if r.returncode or not shots or blank or errors or hashes()!=before:raise SystemExit(1)
binary.unlink()
