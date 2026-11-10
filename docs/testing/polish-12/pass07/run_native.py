#!/usr/bin/env python3
"""Serial native paired facade material evidence; run before/after separately."""
import json, os, subprocess, sys
from pathlib import Path
out=Path(__file__).resolve().parent
name=sys.argv[1]
folder=out/name
folder.mkdir(exist_ok=True)
processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
assert not any(line.split()[1] in ('cargo','rustc','road-dash','blender') for line in processes.splitlines()[1:])
(folder/'processes-before.txt').write_text(processes)
command=['/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash', '--smoke-test' if 'recovery' in name else '--metro-survey']
with (folder/'native.txt').open('w') as log:
 r=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_VERIFY_REPAIRS':'1','ROADDASH_VERIFY_REVERSE':'1'},stdout=log,stderr=subprocess.STDOUT,timeout=240)
(folder/'receipt.json').write_text(json.dumps({'command':command,'exit':r.returncode},indent=2)+'\n')
print(name,r.returncode)
raise SystemExit(r.returncode)
