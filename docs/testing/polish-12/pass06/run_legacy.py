#!/usr/bin/env python3
"""Serial legacy material survey and default structural inspection."""
import json, os, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
folder=out/'valley'
folder.mkdir(exist_ok=True)
processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True)
assert not any(line.split()[1] in ('cargo','rustc','road-dash') for line in processes.splitlines()[1:])
(folder/'processes-before.txt').write_text(processes)
binary='/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash'
command=[binary,'--world-survey']
with (folder/'native.txt').open('w') as log:
 result=subprocess.run(command,cwd=folder,env={**os.environ,'DISPLAY':':0','ROADDASH_TRACK':'valley','ROADDASH_SAVE_DIR':str(folder/'save')},stdout=log,stderr=subprocess.STDOUT,timeout=240)
(folder/'receipt.json').write_text(json.dumps({'command':command,'exit':result.returncode},indent=2)+'\n')
print('valley',result.returncode)
with (out/'inspect-world.json').open('w') as log:
 result=subprocess.run([binary,'--inspect-world'],env={**os.environ,'ROADDASH_TRACK':'metro'},stdout=log,stderr=subprocess.STDOUT)
print('inspect-world',result.returncode)
