#!/usr/bin/env python3
"""Use the root-reviewed frozen combined binary; never build in this script."""
import hashlib,json,os,subprocess
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent
folder=out/'native';folder.mkdir(exist_ok=True)
binary=Path('/home/imad/roaddash-online-city-final/road-dash')
processes=subprocess.check_output(['ps','-eo','pid,comm'],text=True)
assert not any(l.split()[1] in ('cargo','rustc','road-dash','blender') for l in processes.splitlines()[1:]),'shared cargo/GPU slot busy'
(folder/'processes-before.txt').write_text(processes)
hash_before=hashlib.sha256(binary.read_bytes()).hexdigest()
cmd=[str(binary),'--metro-survey']
env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn','ROADDASH_TRACK':'metro','ROADDASH_CROSSING_SURVEY':'1','ROADDASH_UPNP':'0','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_LOG_DIR':str(folder/'performance')}
with (folder/'native.txt').open('w') as log:r=subprocess.run(cmd,cwd=folder,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=360)
shots=list((folder/'screenshots').glob('*.png'));blank=[p.name for p in shots if all(hi-lo<2 for lo,hi in Image.open(p).convert('RGB').getextrema())]
errors=[l for l in (folder/'native.txt').read_text(errors='replace').splitlines() if 'ERROR' in l or 'panicked' in l]
report=dict(command=cmd,exit=r.returncode,binary_sha256=hash_before,unchanged_binary=hashlib.sha256(binary.read_bytes()).hexdigest()==hash_before,images=len(shots),blank_images=blank,error_lines=errors,environment_overrides={k:v for k,v in env.items() if k.startswith('ROADDASH_')},limits='Native scripted census/actual motion, not user art approval, full route, GPU timestamps or network certification.')
(folder/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
print(report)
assert r.returncode==0 and shots and not blank and not errors and report['unchanged_binary']
for name in ['metro-street-life','pedestrian-crossings','metro-performance']:
 data=json.loads((folder/'screenshots'/f'{name}.json').read_text());print(name,data if name!='pedestrian-crossings' else {k:v for k,v in data.items() if k!='walkers'})

life=json.loads((folder/'screenshots/metro-street-life.json').read_text())
cross=json.loads((folder/'screenshots/pedestrian-crossings.json').read_text())
assert life['main_street_cars']==144 and life['walking_pedestrians']==384
assert cross['started']>=1 and cross['completed']>=1 and cross['active']<=cross['max_active']==2
for walker in cross['walkers']:
 if walker['completed']==1:
  assert sum((walker['position'][i]-walker['end'][i])**2 for i in (0,2))<0.001
