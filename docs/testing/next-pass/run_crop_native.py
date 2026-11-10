#!/usr/bin/env python3
"""Serial crop contact fixture. Invoke only after building/freezing its executable."""
import hashlib, json, os, subprocess, sys
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent
root=out.parents[2]
mode=sys.argv[1] if len(sys.argv)>1 else 'on'
assert mode in ('on','off')
folder=out/('native-'+mode)
folder.mkdir(exist_ok=True)
binary=Path(os.environ.get('ROADDASH_BINARY','/home/imad/Documents/ChatGPT/RoadDash-worktrees/.crop-binary/road-dash'))
processes=subprocess.check_output(['ps','-eo','pid,comm'],text=True)
assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:]), 'serial build/native slot required'
(folder/'processes-before.txt').write_text(processes)
inputs=[binary,root/'content/world/metropolis.json',root/'assets/shaders/crops.wgsl',root/'src/world_gen/metropolis/geometry/crops.rs',root/'crates/rr_core/src/crops.rs',root/'src/world_gen/metropolis/geometry/crops/gpu.rs',root/'src/world_gen/metropolis/geometry/crops/survey.rs',root/'src/world_gen/metropolis/geometry/landscape.rs']
hashes=lambda:{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}
before=hashes()
with (folder/'native.txt').open('w') as log:
    result=subprocess.run([str(binary)],cwd=folder,env={**os.environ,'DISPLAY':':0','ROADDASH_CROP_SURVEY':'1','ROADDASH_CROP_RENDER':mode,'ROADDASH_TRACK':'metro','ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_LOG_DIR':str(folder/'performance'),'RUST_LOG':'warn,road_dash::world_gen::metropolis::geometry::crops=info,bevy_render::renderer=info'},stdout=log,stderr=subprocess.STDOUT,timeout=300)
images=list((folder/'screenshots').glob('crops-*.png'))
blank=[p.name for p in images if all(hi-lo<2 for lo,hi in Image.open(p).convert('RGB').getextrema())]
receipt={'render_mode':mode,'exit':result.returncode,'input_hashes':before,'inputs_unchanged':before==hashes(),'captures':len(images),'blank_captures':blank,'fixture':'scripted physical shape contact; not a free-running crash or GPU benchmark'}
(folder/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
assert result.returncode==0 and len(images)==5 and not blank and receipt['inputs_unchanged']
