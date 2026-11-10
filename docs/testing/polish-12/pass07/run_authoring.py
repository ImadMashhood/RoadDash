#!/usr/bin/env python3
"""Run source/geometry/effective-material authoring checks serially."""
import json,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent;root=out.parents[3]
python='/tmp/roaddash-layout-venv/bin/python'
commands=[
 ('material-policy',['python3','-m','unittest','discover','-s','tools','-p','test_facade_materials.py']),
 ('roughness-parity',['/tmp/roaddash-blender-run/blender-4.5.9-linux-x64/blender','-b','--factory-startup','--python-exit-code','1','--python','tools/test_facade_bake_materials.py']),
 ('atlas-validation',['python3','tools/verify_facade_lod.py']),
 ('profiles',['python3','tools/verify_material_profiles.py']),
 ('kit',['python3','tools/verify_architecture_kit.py','--output',str(out/'kit-validation.json')]),
 ('city-source',[python,'-m','unittest','discover','-s','tools','-p','test_city_buildings.py']),
 ('metro-source',[python,'-m','unittest','discover','-s','tools','-p','test_metropolis_facades.py']),
]
results={}
for name,command in commands:
 with (out/(name+'.txt')).open('w') as log:
  r=subprocess.run(command,cwd=root,stdout=log,stderr=subprocess.STDOUT)
 results[name]=dict(command=command,exit=r.returncode)
 print(name,r.returncode,flush=True)
 (out/'authoring-checks.json').write_text(json.dumps(results,indent=2)+'\n')
 if r.returncode:raise SystemExit(r.returncode)
