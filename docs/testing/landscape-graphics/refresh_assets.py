#!/usr/bin/env python3
import csv,hashlib
from pathlib import Path
root=Path(__file__).resolve().parents[3];p=root/'docs/third_party/ASSETS.csv'
with p.open() as f:r=csv.DictReader(f);fields=r.fieldnames;rows=list(r)
paths=['src/world_gen/metropolis/'+n for n in ['mod.rs','geometry.rs','survey.rs','geometry/landscape.rs']]
for path in paths:
 row=next((r for r in rows if r['path']==path),None)
 if row is None:
  row=dict(asset_id='original-landscape-grass',path=path,creator='RoadDash contributors',source_url='local-original:'+path,license='MIT',license_copy='LICENSE',modifications='',attribution='Project MIT notice',redistribution='permitted',shipped='true');rows.append(row)
 row['sha256']=hashlib.sha256((root/path).read_bytes()).hexdigest()
 if 'ADR117' not in row['modifications']:row['modifications']+='; ADR117 original bounded grass/ground/soil rendering and native survey counters; no imported source/media or physical changes'
with p.open('w',newline='') as f:w=csv.DictWriter(f,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows)
