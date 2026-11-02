#!/usr/bin/env python3
"""Bounded proof renders of the exported GLBs, grouped into readable 3x2 sheets."""
import argparse
import concurrent.futures
import json
import subprocess
from pathlib import Path
from run_prop_meshes import DEFAULT, ROOT

def main():
 p=argparse.ArgumentParser();p.add_argument('--blender',default=DEFAULT);p.add_argument('--workers',type=int,default=2);p.add_argument('--group');p.add_argument('--clay',action='store_true');a=p.parse_args()
 designs=json.loads((ROOT/'art/props/mesh_assets/designs.json').read_text())['designs'];groups=sorted({d['group']for d in designs if not a.group or d['group']==a.group})
 def run(group):
  path=ROOT/'docs/testing/prop-meshes/logs'/(group+('_clay'if a.clay else '')+'_renders.txt')
  with path.open('w')as f:r=subprocess.run([a.blender,'-b','--factory-startup','--python-exit-code','1','--python',str(ROOT/'tools/render_prop_meshes.py'),'--',group]+(['--clay']if a.clay else []),cwd=ROOT,stdout=f,stderr=subprocess.STDOUT)
  print(json.dumps(dict(group=group,exit_code=r.returncode)),flush=True);return r.returncode
 with concurrent.futures.ThreadPoolExecutor(max_workers=a.workers)as pool:results=list(pool.map(run,groups))
 if any(results):raise SystemExit('Failed renders; inspect group logs')
if __name__=='__main__':main()
