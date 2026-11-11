#!/usr/bin/env python3
"""Summarize sampled decision traces; never infer unsampled 60Hz events."""
import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
parser = argparse.ArgumentParser()
parser.add_argument('receipt', type=Path)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
source = json.loads(args.receipt.read_text())
groups = defaultdict(list)
for sample in source['samples']:
    groups[sample['id']].append(sample)
rows = []
for identity, samples in sorted(groups.items()):
    samples.sort(key=lambda s: s['frame'])
    longest = duration = 0.
    current_kind = None
    longest_kind = None
    changes = 0
    for previous, sample in zip(samples, samples[1:]):
        changes += sample['target'] != previous['target']
        stopped = (sample['recovery'] == previous['recovery'] == 'Riding'
                   and sample['speed'] < 1. and previous['speed'] < 1.
                   and abs(sample['progress'] - previous['progress']) < 0.5
                   and sample['finished'] is None)
        plan = sample['plan'] or {}
        kind = 'avoidance' if plan.get('avoiding') and plan.get('speed', 0.) < 1. else 'physical_or_other'
        if stopped:
            duration += sample['elapsed'] - previous['elapsed']
            current_kind = kind if current_kind is None else current_kind if current_kind == kind else 'mixed'
        else:
            duration = 0.
            current_kind = None
        if duration > longest:
            longest = duration
            longest_kind = current_kind
    last = samples[-1]
    rows.append(dict(id=identity, initial_progress=samples[0]['progress'], last_progress=last['progress'],
                     last_speed=last['speed'], finished=last['finished'], sampled_longest_stop_seconds=longest,
                     longest_stop_kind=longest_kind, sampled_target_changes=changes,
                     decisions=dict(Counter(s['decision'] for s in samples)),
                     recovery=dict(Counter(s['recovery'] for s in samples))))
result = dict(source=str(args.receipt), frames=source['frames'], elapsed=source['elapsed'],
              synthetic_segment=source.get('synthetic_segment', False), segment_passed=source.get('segment_passed'),
              recorded_finished_ids=source['finished_ids'], slot_zero_finished=0 in source['finished_ids'],
              riders=rows, limits='Three-second samples; stop durations and target changes are sampled evidence, not continuous 60Hz guarantees. Police id15 is separate from race riders.')
args.output.write_text(json.dumps(result, indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k != 'riders'}, indent=2))
