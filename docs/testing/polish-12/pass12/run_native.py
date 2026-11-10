#!/usr/bin/env python3
"""Serial native character presentation/recovery evidence; frozen assets and isolated saves."""
import hashlib, json, os, subprocess, sys
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent
name=sys.argv[1]
folder=out/name
folder.mkdir(exist_ok=True)
processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:])
(folder/'processes-before.txt').write_text(processes)
command=['/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash', *(['--animation-lab-view','--animation-lab-smoke'] if 'poses' in name else ['--actor-skin-survey'] if 'skin' in name else ['--actor-smoke-test'] if 'actors' in name else ['--smoke-test']), *(['--ragdoll-baseline'] if 'baseline' in name else [])]
root=out.parents[3]
frozen=[root/'content/world/art.json', root/'content/world/metropolis.json', root/'assets/shaders/metropolis_facade.wgsl', root/'assets/models/riding.rdmesh', root/'assets/models/rider-bindings.json', root/'assets/models/motion.json', root/'content/hand-contacts.json', Path(command[0])]
hashes=lambda: {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen}
before_hashes=hashes()
with (folder/'native.txt').open('w') as log:
 r=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn,road_dash::sky=info','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_VERIFY_REACTIONS':'1' if 'reactions' in name else '0','ROADDASH_VERIFY_REPAIRS':'0' if 'reactions' in name else '1','ROADDASH_VERIFY_REVERSE':'0' if 'reactions' in name else '1','ROADDASH_SMOKE_ROUTE':'1' if 'baseline' in name else '0'},stdout=log,stderr=subprocess.STDOUT,timeout=240)
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
