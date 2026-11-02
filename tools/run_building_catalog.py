#!/usr/bin/env python3
"""Bounded parallel Blender workers; success requires complete per-sheet artifacts."""
import argparse
import concurrent.futures
import json
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/buildings/mesh_assets'
BLENDER=Path('/tmp/roaddash-character-tools/blender-4.5.1-linux-x64/blender')
LOGS=ROOT/'docs/testing/building-catalog/logs'

def main():
    p=argparse.ArgumentParser();p.add_argument('--group',choices=['facades','structures','all'],default='all');p.add_argument('--workers',type=int,default=2);p.add_argument('--resume',action='store_true');a=p.parse_args()
    source=json.loads((OUT/'catalog_designs.json').read_text())['designs']
    sheets=sorted({d['reference_sheet']for d in source if d['builder']!='accepted_first_sheet'and(a.group=='all'or d['reference_sheet'].startswith(a.group))})
    LOGS.mkdir(parents=True,exist_ok=True)
    def run(sheet):
        manifest=OUT/'catalog_manifests'/(Path(sheet).stem+'.json')
        if a.resume and manifest.is_file():
            r=json.loads(manifest.read_text());expected={d['id']for d in source if d['reference_sheet']==sheet}
            if {d['id']for d in r['records']}==expected and all((OUT/'models'/l['file']).is_file()for d in r['records']for l in d['lods']):return sheet,'reused'
        log=LOGS/(Path(sheet).stem+'.txt')
        with log.open('w')as f:result=subprocess.run([str(BLENDER),'-b','--factory-startup','--python-exit-code','1','--python',str(ROOT/'tools/build_catalog_batch.py'),'--',sheet],cwd=ROOT,stdout=f,stderr=subprocess.STDOUT)
        if result.returncode or not manifest.is_file():return sheet,'FAILED: '+str(log)
        return sheet,'complete'
    failed=[]
    with concurrent.futures.ThreadPoolExecutor(max_workers=a.workers)as pool:
        for sheet,status in pool.map(run,sheets):
            print(json.dumps({'sheet':sheet,'status':status}),flush=True)
            if status.startswith('FAILED'):failed.append(sheet)
    if failed:raise SystemExit('Failed sheets: '+','.join(failed))
if __name__=='__main__':main()
