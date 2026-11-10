#!/usr/bin/env python3
"""Refresh only pass07 changed provenance entries; preserve unrelated records."""
import csv,hashlib,subprocess
from pathlib import Path
root=Path(__file__).resolve().parents[4]
path=root/'docs/third_party/ASSETS.csv'
with path.open() as f:
 reader=csv.DictReader(f);fields=reader.fieldnames;rows=list(reader)
changed=set(subprocess.check_output(['git','diff','--name-only'],cwd=root,text=True).splitlines())
for row in rows:
 p=root/row['path']
 if row['path'] in changed and p.is_file():
  row['sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
  if 'Pass07' not in row['modifications']:
   row['modifications']+='; Pass07 original source material policy/effective roughness parity and model-derived rebake; geometry and original media preserved'
for name in ['facade_material_policy','refresh_facade_materials','extract_front_modules']:
 p=f'tools/{name}.py'
 if any(r['path']==p for r in rows):continue
 rows.append(dict(asset_id='original-polish07-'+name,path=p,creator='RoadDash contributors',source_url='local-original:'+p,license='MIT',license_copy='LICENSE',sha256=hashlib.sha256((root/p).read_bytes()).hexdigest(),modifications='Original offline material compiler/policy; no external code or media imported',attribution='Project MIT notice',redistribution='permitted',shipped='true'))
p='assets/metropolis/facades-packed.rgba'
if not any(r['path']==p for r in rows):
 rows.append(dict(asset_id='original-metro-facades-packed',path=p,creator='RoadDash contributors',source_url='local-original:tools/bake_metropolis.py',license='MIT',license_copy='LICENSE',sha256=hashlib.sha256((root/p).read_bytes()).hexdigest(),modifications='Original model-derived roughness/metallic crops with five mips; same 24 family/band mapping as albedo; no external media',attribution='Project MIT notice',redistribution='permitted',shipped='true'))
with path.open('w',newline='') as f:
 writer=csv.DictWriter(f,fieldnames=fields,lineterminator='\n');writer.writeheader();writer.writerows(rows)
