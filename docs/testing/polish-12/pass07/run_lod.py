#!/usr/bin/env python3
"""Serial native LOD sweeps, isolated saves, no concurrent game/compiler/baker."""
import json, os, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
binary='/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash'
for track in ['valley','city']:
    folder=out/('lod-'+track);folder.mkdir(exist_ok=True)
    processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
    assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:])
    (folder/'processes-before.txt').write_text(processes)
    command=[binary,'--lod-survey']
    with (folder/'native.txt').open('w') as log:
        result=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','ROADDASH_TRACK':track,'ROADDASH_SAVE_DIR':str(folder/'save')},stdout=log,stderr=subprocess.STDOUT,timeout=300)
    (folder/'receipt.json').write_text(json.dumps(dict(command=command,exit=result.returncode),indent=2)+'\n')
    print(track,result.returncode,flush=True)
    if result.returncode:raise SystemExit(result.returncode)
