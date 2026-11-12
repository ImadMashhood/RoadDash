#!/usr/bin/env python3
"""Matched Hills opening functional comparison; no performance/difficulty claim."""
import json
import sys
from statistics import median


def summarize(path):
    report = json.load(open(path))
    rows = [r for r in report['samples'] if 1 <= r['id'] <= 14]
    bins = []
    for end in [15., 30., 45., 60., 90., 120.]:
        eligible = [r for r in rows if r['elapsed'] <= end]
        if not eligible:
            continue
        frame = max(r['frame'] for r in eligible)
        at = [r for r in eligible if r['frame'] == frame]
        bins.append(dict(elapsed=at[0]['elapsed'], frame=frame,
                         median_progress_m=median(r['progress'] for r in at),
                         median_kmh=median(r['speed'] * 3.6 for r in at),
                         stopped=[r['id'] for r in at if r['speed'] < 1. and r['recovery'] == 'Riding'],
                         recovering=[r['id'] for r in at if r['recovery'] != 'Riding']))
    early = [r for r in rows if 5. < r['elapsed'] <= 60.]
    return dict(bins=bins,
                stopped_rider_samples=sum(r['speed'] < 1. and r['recovery'] == 'Riding' for r in early),
                median_opening_kmh=median(r['speed'] * 3.6 for r in early),
                first_sample_past_city={str(i): next((r['elapsed'] for r in rows if r['id'] == i and r['progress'] >= 1500.), None) for i in range(1, 15)})


if __name__ == '__main__':
    result = dict(control=summarize(sys.argv[1]), candidate=summarize(sys.argv[2]),
                  limits='Matched scripted all-AI race samples. Not literal W human control, '
                         'full-race completion, human challenge or a performance benchmark.')
    print(json.dumps(result, indent=2))
