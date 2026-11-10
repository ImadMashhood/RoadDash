#!/usr/bin/env python3
"""Observed wall-frame cost and actual ready detail census; no GPU/speedup inference."""
import json,re,shutil
from pathlib import Path
out=Path(__file__).resolve().parent
reports={};views={}
for name in ['before','after']:
 folder=out/name
 reports[name]=json.loads((folder/'screenshots/metro-performance.json').read_text())
 native=(folder/'native.txt').read_text()
 telemetry=Path(re.search(r'Performance log: (\S+)',native).group(1))
 shutil.copyfile(telemetry,folder/'performance.jsonl')
 rows=[json.loads(line) for line in telemetry.read_text().splitlines()]
 selected=[r for r in rows if -200<=r['camera_xyz'][2]<1800 and r['frames']>10]
 reports[name]['approach_window_observations']=[{k:r[k] for k in ['camera_xyz','frames','window_wall_ms_per_frame','visible_mesh_entities','mesh_entities']} for r in selected]
 detail=json.loads((folder/'screenshots/metro-authored-facades.json').read_text())
 views[name]=[{k:v[k] for k in ['view','route','chainage','kit','native_draw_audit']} for v in detail['views']]
comparison=dict(census_unchanged=reports['before']['census']==reports['after']['census'],before=reports['before'],after=reports['after'],native_views=views,limits='Single bounded wall samples. Approach windows can straddle a camera move; counts/CPU/wall timing only, not GPU cost, temporal/hardware approval or causal speedup. Static collision census equal; actual near detail and resident geometry are compared; no visibility policy changes.')
assert comparison['census_unchanged']
comparison['native_detail_audits_identical']=all(a['native_draw_audit']==b['native_draw_audit'] for a,b in zip(views['before'],views['after'],strict=True))
assert comparison['native_detail_audits_identical']
comparison['errors']={}
for name in ['before','after','after-recovery']:
 text=(out/name/'native.txt').read_text()
 comparison['errors'][name]=[line for line in text.splitlines() if re.search(r'\bERROR\b|panicked|Validation Error|shader error',line)]
 assert not comparison['errors'][name]
(out/'comparison.json').write_text(json.dumps(comparison,indent=2)+'\n')
for name in reports:print(name,reports[name]['metrics'])
