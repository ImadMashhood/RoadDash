#!/usr/bin/env python3
"""Serial native cast AI and living city evidence; isolated saves and guarded process capture."""
import hashlib, json, os, subprocess, sys
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent
name=sys.argv[1]
folder=out/name
folder.mkdir(exist_ok=True)
processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:])
(folder/'processes-before.txt').write_text('\n'.join(' '.join(line.split()[:2]) for line in processes.splitlines())+'\n')
command=[os.environ.get('ROADDASH_TEST_BINARY','/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash'), '--actor-smoke-test' if name == 'actors' else '--smoke-test' if 'recovery' in name else '--metro-survey']
if 'baseline' in name: command.append('--ragdoll-baseline')
if name=='skins': command=[command[0],'--actor-skin-survey']
if name=='poses': command=[command[0],'--animation-lab-view','--animation-lab-smoke']
root=out.parents[2]
frozen=[root/'content/world/living_city.json', root/'content/catalog.json', root/'content/world/art.json', root/'content/world/metropolis.json', root/'assets/shaders/metropolis_facade.wgsl', Path(command[0])]
hashes=lambda: {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen}
before_hashes=hashes()
with (folder/'native.txt').open('w') as log:
 r=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn,road_dash::sky=info,road_dash::world_gen::metropolis::geometry=info','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_LOG_DIR':str(folder/'performance'),'ROADDASH_SMOKE_ROUTE':'1' if 'baseline' in name else '0','ROADDASH_VERIFY_REPAIRS':'1','ROADDASH_VERIFY_REVERSE':'1',**({'ROADDASH_HALL_VIEWS':'1'} if name in ('draft','final') else {'ROADDASH_CIVIC_VIEWS':'1'} if name=='civic' else {'ROADDASH_LANDSCAPE_VIEWS':'1'} if name=='shoulder' else {})},stdout=log,stderr=subprocess.STDOUT,timeout=240)
blank_images=[]
screenshots=list((folder/'screenshots').rglob('*.png'))
assert screenshots, 'native run captured no screenshots'
for screenshot in screenshots:
 extrema=Image.open(screenshot).convert('RGB').getextrema()
 if all(hi-lo < 2 for lo,hi in extrema): blank_images.append(screenshot.name)
(folder/'image-validation.json').write_text(json.dumps({'blank_images':blank_images,'accepted':not blank_images},indent=2)+'\n')
(folder/'receipt.json').write_text(json.dumps({'command':command,'exit':r.returncode,'frozen_inputs':before_hashes,'inputs_unchanged':hashes()==before_hashes},indent=2)+'\n')
print(name,r.returncode)
raise SystemExit(r.returncode or bool(blank_images))
