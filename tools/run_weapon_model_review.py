#!/usr/bin/env python3
"""Serial native GLB/actual rider socket receipts; requires a graphical session."""
import argparse,hashlib,json,os,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('--binary',type=Path,required=True);a=p.parse_args()
binary=a.binary.resolve();out=ROOT/'docs/testing/new-weapon-models/native';out.mkdir(parents=True,exist_ok=True)
binary_hash=hashlib.sha256(binary.read_bytes()).hexdigest()
results=[]
for model in ['chain','whip','rolling_pin','baseball_bat','club_baseline','invalid_model']:
 for mode in (['equipped'] if model in ['club_baseline','invalid_model'] else ['lod0','lod1','lod2','equipped']):
  stem=f'{model}-{mode}';image=out/f'{stem}.png';log=out/f'{stem}.log'
  command=[str(binary),'--id',model,'--capture',str(image)]
  command += ['--equipped']if mode=='equipped'else ['--lod',mode[-1]]
  env=os.environ.copy();env.pop('ROADDASH_WEAPON_MODEL',None)
  if model!='club_baseline':env['ROADDASH_WEAPON_MODEL']=model
  env['RUST_LOG']='warn,road_dash::weapon_models=info'
  env['TMPDIR']='/home/imad/Documents/ChatGPT/RoadRash/output/testing/parallel-fixes/tmp'
  start=time.monotonic()
  with log.open('w')as f:r=subprocess.run(command,cwd=ROOT,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=180)
  ready='Original weapon model scene ready; hiding 3 fallback parts' in log.read_text()
  record=dict(model=model,mode=mode,command=command,exit=r.returncode,seconds=time.monotonic()-start,image_sha256=hashlib.sha256(image.read_bytes()).hexdigest()if image.exists()else None,equipped_scene_ready=ready)
  results.append(record);(out/'receipts.json').write_text(json.dumps(dict(binary_sha256=binary_hash,results=results),indent=2)+'\n')
  assert r.returncode==0 and image.is_file(),record
  if mode=='equipped':
   assert ready==(model not in ['club_baseline','invalid_model']),record
   if model=='invalid_model':assert 'retaining original club' in log.read_text()
  print(stem,'PASS',flush=True)
