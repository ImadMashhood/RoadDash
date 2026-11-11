#!/usr/bin/env python3
"""Summarize bounded RoadDash JSONL windows; never infer GPU attribution from CPU utilization."""
import argparse
import json
import math
from pathlib import Path
import statistics

TIMINGS = ['window_wall_ms_per_frame', 'wall_average_ms', 'wall_p95_ms',
           'main_world_ms_per_frame', 'last_render_extract_ms', 'last_render_schedule_ms',
           'fixed_gameplay_ms_per_frame', 'physics_ms_per_frame', 'physics_sync_ms_per_frame',
           'physics_step_ms_per_frame', 'physics_writeback_ms_per_frame']
CONTEXT = ['racing', 'camera_xyz', 'elapsed_s', 'player_progress_m', 'fixed_steps', 'frames',
           'visible_mesh_entities', 'mesh_entities', 'dynamic_bodies', 'physics_backend',
           'mesh_assets', 'material_assets', 'linux_process_memory', 'linux_system_paging_counters']


def summarize(path, skip_samples=0, racing_only=False, skip_racing_samples=0, raw_skip_frames=0):
    if skip_samples < 0 or skip_racing_samples < 0 or raw_skip_frames < 0:
        raise ValueError('sample exclusions must be nonnegative')
    samples = []
    invalid = 0
    import gzip
    text = gzip.open(path, 'rt').read() if str(path).endswith('.gz') else Path(path).read_text()
    for line in text.splitlines():
        try:
            s = json.loads(line)
            if not isinstance(s, dict) or s.get('schema_version') != 1:
                invalid += 1
                continue
            if not all(isinstance(s.get(k), (int, float)) and math.isfinite(s[k])
                       for k in ['wall_p95_ms', 'frames', 'fixed_steps']):
                invalid += 1
                continue
            samples.append(s)
        except (json.JSONDecodeError, TypeError, ValueError):
            invalid += 1
    selected = [s for s in samples[skip_samples:] if not racing_only or s.get('racing')]
    if skip_racing_samples:
        selected = [s for s in selected if s.get('racing')][skip_racing_samples:]
    if not selected:
        raise ValueError('No complete performance samples match the selection')
    keys = CONTEXT + TIMINGS
    slow = sorted(selected, key=lambda s: s['wall_p95_ms'], reverse=True)[:10]
    def distribution(values):
        ordered = sorted(values)
        return dict(median=statistics.median(ordered), maximum=ordered[-1],
                    p95=ordered[math.ceil((len(ordered)-1)*0.95)])
    timing = {}
    for key in TIMINGS:
        values = [s[key] for s in selected if isinstance(s.get(key), (int, float)) and math.isfinite(s[key])]
        if values:
            timing[key] = distribution(values)
    # Catch-up can put several fixed steps in one render frame. Normalize aggregate
    # work by the actual number of steps, not by the display FPS or target 60 Hz.
    frames = sum(s['frames'] for s in selected)
    steps = sum(s['fixed_steps'] for s in selected)
    per_step = {}
    for key in ['fixed_gameplay_ms_per_frame', 'physics_ms_per_frame',
                'physics_sync_ms_per_frame', 'physics_step_ms_per_frame', 'physics_writeback_ms_per_frame']:
        covered = [s for s in selected if isinstance(s.get(key), (int, float)) and math.isfinite(s[key])]
        n = sum(s['fixed_steps'] for s in covered)
        if n:
            per_step[key.replace('_per_frame', '_per_fixed_step')] = sum(s[key]*s['frames'] for s in covered)/n
    raw_samples = samples if raw_skip_frames else selected
    raw = [v for sample in raw_samples
           for v in ((sample.get('raw_racing_frame_ms') if racing_only and 'raw_racing_frame_ms' in sample
                      else sample.get('raw_frame_ms') if not racing_only or sample.get('racing') else []) or [])
           if isinstance(v, (int, float)) and math.isfinite(v) and v >= 0][raw_skip_frames:]
    raw_summary = None
    if raw:
        ordered = sorted(raw)
        raw_summary = dict(samples=len(raw), dropped_samples=sum(s.get('raw_frame_samples_dropped') or 0 for s in selected), p50=statistics.median(ordered),
                           p95=ordered[math.ceil((len(ordered)-1)*.95)],
                           p99=ordered[math.ceil((len(ordered)-1)*.99)],
                           maximum=ordered[-1], stalls_over_50ms=sum(v > 50 for v in raw),
                           stalls_over_100ms=sum(v > 100 for v in raw))
    memory_peak_kib = {}
    for sample in samples:
        for key, value in (sample.get('linux_process_memory') or {}).items():
            if isinstance(value, str) and value.endswith(' kB'):
                try:
                    memory_peak_kib[key] = max(memory_peak_kib.get(key, 0), int(value.split()[0]))
                except ValueError:
                    pass
    render = {}
    for sample in selected:
        for key, value in sample.get('render_diagnostics_latest', {}).items():
            if isinstance(value, (int, float)) and math.isfinite(value):
                render.setdefault(key, []).append(value)
    return dict(raw_frame_distribution_ms=raw_summary, process_memory_peak_kib=memory_peak_kib,
                renderer_latest_sample_distributions={key: distribution(values) for key, values in render.items()},
                renderer_sampling='Latest available asynchronous render diagnostics once per logging window; not full-frame GPU percentiles; missing GPU metrics remain absent.',
                parallel_physics=selected[-1].get('parallel_physics'),
                available_cpu_threads=selected[-1].get('available_cpu_threads'),
                requested_rayon_threads=selected[-1].get('requested_rayon_threads'),
                total_valid_samples=len(samples), samples=len(selected), invalid_or_partial_lines=invalid,
                selection=dict(skip_samples=skip_samples, racing_only=racing_only, skip_racing_samples=skip_racing_samples, raw_skip_frames=raw_skip_frames),
                racing_samples=sum(bool(s.get('racing')) for s in selected),
                fixed_steps_per_render_frame=steps/frames if frames else None,
                window_distributions_ms=timing, weighted_fixed_step_ms=per_step,
                worst_windows=[{k:s.get(k) for k in keys} for s in slow],
                guidance='Distributions summarize logged windows, not raw per-frame percentiles; wall_p95_ms is an overlapping rolling 240-frame metric. window_wall_ms_per_frame is the independent logging-window mean. Compare matched scenes without compilation/other game instances. Physics includes sync/step/writeback CPU/wait spans, not just the solver. If steps/render frame rises, catch-up multiplies simulation cost. Large wall with small main needs GPU/presentation profiling; low utilization is not proof. Paging counters are system-wide; full swap alone does not prove active game paging.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('log')
    parser.add_argument('--skip-samples', type=int, default=0,
                        help='Discard the first N valid windows (e.g. startup/loading).')
    parser.add_argument('--racing-only', action='store_true')
    parser.add_argument('--skip-racing-samples', type=int, default=0, help='Discard racing windows after the state filter, independently of startup duration.')
    parser.add_argument('--raw-skip-frames', type=int, default=0, help='Select raw frames independently of window exclusions and omit this many initial racing frames.')
    args = parser.parse_args()
    try:
        print(json.dumps(summarize(args.log, args.skip_samples, args.racing_only, args.skip_racing_samples, args.raw_skip_frames), indent=2))
    except ValueError as error:
        parser.error(str(error))
