#!/usr/bin/env python3
"""Compare quiet same-binary AI-work controls; CPU timings are not GPU timestamps."""
import json
from pathlib import Path
from statistics import median
import sys


def summarize(folder, start=5., end=50.):
    files = list((folder / 'performance').glob('*.jsonl'))
    rows = [json.loads(line) for file in files for line in file.read_text().splitlines()]
    active = [r for r in rows if r['racing'] and start <= r.get('race_elapsed_s', -1.) <= end]
    if not active:
        raise ValueError('No opening race samples')
    ticks = sum(r['fixed_steps'] for r in active)
    frames = sum(r['frames'] for r in active)
    counters = {key: sum(r['ai_work_window'][key] for r in active) for key in active[0]['ai_work_window']}
    return dict(samples=len(active), fixed_ticks=ticks, render_frames=frames,
                mean_main_cpu_ms_per_frame=sum(r['main_world_ms_per_frame'] * r['frames'] for r in active) / frames,
                median_window_wall_ms=median(r['window_wall_ms_per_frame'] for r in active),
                median_window_main_ms=median(r['main_world_ms_per_frame'] for r in active),
                drive_ms_per_tick=counters['drive_ms'] / ticks,
                static_ms_per_tick=counters['static_ms'] / ticks,
                static_casts_per_tick=counters['static_casts'] / ticks,
                requests_per_tick=counters['static_requests'] / ticks,
                cache_hits_per_tick=counters['static_cache_hits'] / ticks,
                entity_range=[min(r['entities'] for r in active), max(r['entities'] for r in active)])


def states(folder):
    report = json.loads((folder / 'screenshots/racer-ai.json').read_text())
    return {(r['frame'], r['id']): r for r in report['samples']}


if __name__ == '__main__':
    a, b = [Path(p) for p in sys.argv[1:3]]
    start, end = map(float, sys.argv[3:5]) if len(sys.argv) > 3 else (5., 50.)
    old, new = states(a), states(b)
    common = sorted(i for i in old.keys() & new.keys() if start <= old[i]['elapsed'] <= end)
    result = dict(race_interval=[start, end], control=summarize(a, start, end), optimized=summarize(b, start, end),
                  matched_state_rows=len(common),
                  max_state_delta={key: max(abs(old[i][key] - new[i][key]) for i in common)
                                   for key in ['progress', 'lane', 'speed']},
                  different_plans=sum(old[i]['plan'] != new[i]['plan'] for i in common),
                  different_recovery_states=sum(old[i]['recovery'] != new[i]['recovery'] for i in common),
                  limits='One same-binary quiet Linux A/B opening pair at race_interval, '
                         'aggregated fixed-tick/window CPU measurements. Later trajectories may diverge; state equality is sampled, not per tick. No GPU, hardware60FPS '
                         'or cold-boot certification. Full raw streams retained locally.')
    print(json.dumps(result, indent=2))
