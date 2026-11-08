#!/usr/bin/env python3
"""Summarize RoadDash's local JSONL samples; CPU/wall clues, never GPU attribution."""
import argparse,json
from pathlib import Path

def summarize(path):
    samples=[];invalid=0
    for line in Path(path).read_text().splitlines():
        try:s=json.loads(line)
        except json.JSONDecodeError:invalid+=1;continue
        if s.get('schema_version')==1:samples.append(s)
    if not samples:raise ValueError('No complete performance samples')
    slow=sorted(samples,key=lambda s:s['wall_p95_ms'],reverse=True)[:10]
    keys=['racing','camera_xyz','elapsed_s','player_progress_m','wall_average_ms','wall_p95_ms','main_world_ms_per_frame','last_render_extract_ms','last_render_schedule_ms','fixed_gameplay_ms_per_frame','physics_ms_per_frame','visible_mesh_entities','mesh_entities','dynamic_bodies','mesh_assets','material_assets','linux_process_memory','linux_system_paging_counters']
    return dict(parallel_physics=samples[-1].get("parallel_physics"),available_cpu_threads=samples[-1].get("available_cpu_threads"),requested_rayon_threads=samples[-1].get("requested_rayon_threads"),samples=len(samples),invalid_or_partial_lines=invalid,racing_samples=sum(bool(s.get('racing')) for s in samples),worst_windows=[{k:s.get(k) for k in keys} for s in slow],
        guidance='Compare main-world vs wall time first; large physics/gameplay shares identify simulation candidates. Low main with high wall requires render-extraction/GPU/presentation profiling, not a GPU-utilization guess. Compare paging-counter deltas between samples; system counters are global and a full swap alone does not prove active paging.')
if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('log');args=p.parse_args()
    print(json.dumps(summarize(args.log),indent=2))
