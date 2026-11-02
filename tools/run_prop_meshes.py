#!/usr/bin/env python3
"""Run bounded local Blender exports; failed groups never receive completion status."""
import argparse
import concurrent.futures
import json
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
DEFAULT='/tmp/roaddash-character-tools/blender-4.5.1-linux-x64/blender'

def main():
 p=argparse.ArgumentParser();p.add_argument('--blender',default=DEFAULT);p.add_argument('--workers',type=int,default=2);p.add_argument('--group');a=p.parse_args()
 designs=json.loads((ROOT/'art/props/mesh_assets/designs.json').read_text())['designs']
 groups=sorted({d['group']for d in designs if not a.group or d['group']==a.group})
 logs=ROOT/'docs/testing/prop-meshes/logs';logs.mkdir(parents=True,exist_ok=True)
 def run(group):
  with (logs/(group+'.txt')).open('w')as f:
   r=subprocess.run([a.blender,'-b','--factory-startup','--python-exit-code','1','--python',str(ROOT/'tools/build_prop_meshes.py'),'--',group],stdout=f,stderr=subprocess.STDOUT,cwd=ROOT)
  print(json.dumps(dict(group=group,exit_code=r.returncode)),flush=True)
  return r.returncode
 with concurrent.futures.ThreadPoolExecutor(max_workers=a.workers)as pool:results=list(pool.map(run,groups))
 if any(results):raise SystemExit('One or more prop groups failed; inspect logs')
if __name__=='__main__':main()
