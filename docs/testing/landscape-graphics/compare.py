#!/usr/bin/env python3
"""Census and matching early-farm windows; no GPU/causal-speedup inference."""
import json,statistics
from pathlib import Path
out=Path(__file__).resolve().parent
reports={}
for name in ['before','after']:
 folder=out/name;report=json.loads((folder/'screenshots/metro-performance.json').read_text())
 logs=list((folder/'performance').glob('*.jsonl'));assert len(logs)==1
 rows=[json.loads(l) for l in logs[0].read_text().splitlines()]
 selected=[r for r in rows if r['camera_xyz'][2]<-1000 and r['frames']>10]
 report['early_farm_windows']=[{k:r[k] for k in ['camera_xyz','frames','window_wall_ms_per_frame','visible_mesh_entities','mesh_entities','linux_process_memory']} for r in selected]
 report['early_farm_weighted_wall_ms']=sum(r['frames']*r['window_wall_ms_per_frame'] for r in selected)/sum(r['frames'] for r in selected)
 reports[name]=report
keys=['buildings','chunks','collision_triangles','source_collision_batches','static_collision_batches','physical_routes','source_layout_sha256','crossing_cars','street_props','parked_cars','walking_pedestrians']
assert all(reports['before']['census'][k]==reports['after']['census'][k] for k in keys)
views=json.loads((out/'after/screenshots/metro-authored-facades.json').read_text())['views']
result={'before':reports['before'],'after':reports['after'],'preserved_census_keys':keys,'grass_views':[{k:v[k] for k in ['view','chainage','grass_visible_batches','grass_visible_triangles']} for v in views],'limits':'Single native samples. Early-farm one-second windows include asset warmup and can straddle camera moves; no isolated900m timing, GPU attribution, causal speedup or full-course/Windows/hardware certificate. Final240frame p95 is downtown after grass fades. Collision-soup regression and preservation.json supplement count equality.'}
(out/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
for n in reports:print(n,reports[n]['metrics']['wall_frame_average_ms'],reports[n]['metrics']['wall_frame_p95_ms'],reports[n]['early_farm_weighted_wall_ms'])
print(result['grass_views'][:2])
