#!/usr/bin/env python3
"""Compare matched native race samples without inferring unsampled contacts."""
import argparse
import json
from pathlib import Path
from statistics import median


def read(path):
    return [row for row in json.loads(path.read_text())['samples'] if 1 <= row['id'] <= 14]


def counters(rows, field):
    if not rows or any(field not in row for row in rows):
        return None
    # Maxima are lower bounds if an identity respawns/resets during the survey.
    return {str(identity): max(row[field] for row in rows if row['id'] == identity)
            for identity in sorted({row['id'] for row in rows})}


def summarize(rows, frame):
    at_frame = [row for row in rows if row['frame'] == frame]
    attempts = counters(rows, 'attack_serial')
    contacts = counters(rows, 'confirmed_incidents')
    active_frames = {row['frame'] for row in rows if row['elapsed'] > 5.}
    peak_standing = max((sum(row['recovery'] == 'Riding' and row['speed'] < 1.
                             for row in rows if row['frame'] == sample)
                         for sample in active_frames), default=0)
    opening = [row for row in rows if 2. < row['elapsed'] < 4.]
    return {
        'matched_frame': frame,
        'elapsed': at_frame[0]['elapsed'],
        'rider_count': len(at_frame),
        'peak_sampled_standing_riders_after_launch': peak_standing,
        'opening_median_speed_kmh': median(row['speed'] * 3.6 for row in opening) if opening else None,
        'median_speed_kmh': median(row['speed'] * 3.6 for row in at_frame),
        'median_progress_m': median(row['progress'] for row in at_frame),
        'standing_riders': [row['id'] for row in at_frame
                            if row['recovery'] == 'Riding' and row['speed'] < 1.],
        'recovering_riders': [row['id'] for row in at_frame if row['recovery'] != 'Riding'],
        'max_attack_serial_by_rider': attempts,
        'max_confirmed_incidents_by_recipient': contacts,
        'attack_lower_bound': sum(attempts.values()) if attempts is not None else None,
        'contact_lower_bound': sum(contacts.values()) if contacts is not None else None,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('control', type=Path)
    parser.add_argument('candidate', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    control, candidate = read(args.control), read(args.candidate)
    common = {row['frame'] for row in control} & {row['frame'] for row in candidate}
    if not common:
        parser.error('Surveys have no shared sample frame')
    frame = max(common)
    matched_control = [row for row in control if row['frame'] <= frame]
    matched_candidate = [row for row in candidate if row['frame'] <= frame]
    report = {
        'control': summarize(matched_control, frame),
        'candidate': summarize(matched_candidate, frame),
        'candidate_extended': summarize(candidate, max(row['frame'] for row in candidate)),
        'limits': 'Periodic scripted samples, not full-race/human balance or a frame-time benchmark. '
                  'Missing counters remain null. Counter maxima are lower bounds; contacts are '
                  'counted by recipient, not attributed to an attacker. Police/slot-zero excluded.',
    }
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
