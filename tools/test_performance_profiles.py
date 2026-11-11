#!/usr/bin/env python3
"""Regression checks for genuine raw-frame statistics and legacy log compatibility."""
import json
from pathlib import Path
import tempfile
import unittest
from summarize_performance import summarize


class FrameEvidence(unittest.TestCase):
    def report(self, raw=None):
        sample = dict(schema_version=1, frames=100, fixed_steps=100,
                      wall_p95_ms=20., racing=True)
        if raw is not None:
            sample['raw_frame_ms'] = raw
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder)/'sample.jsonl'
            path.write_text(json.dumps(sample)+'\n'+'incomplete')
            return summarize(path)

    def test_legacy_window_summary_never_invents_raw_or_gpu_measurements(self):
        result = self.report()
        self.assertIsNone(result['raw_frame_distribution_ms'])
        self.assertEqual(result['renderer_latest_sample_distributions'], {})
        self.assertEqual(result['invalid_or_partial_lines'], 1)

    def test_raw_tail_and_stalls_are_frames_not_window_means(self):
        result = self.report([10.]*98+[75., 125.])['raw_frame_distribution_ms']
        self.assertEqual(result['samples'], 100)
        self.assertEqual(result['p50'], 10.)
        self.assertEqual(result['p95'], 10.)
        self.assertEqual(result['p99'], 125.)
        self.assertEqual(result['maximum'], 125.)
        self.assertEqual(result['stalls_over_50ms'], 2)
        self.assertEqual(result['stalls_over_100ms'], 1)

    def test_racing_raw_warmup_is_independent_of_window_selection(self):
        samples = [dict(schema_version=1, frames=200, fixed_steps=200,
                        wall_p95_ms=99., racing=True,
                        raw_frame_ms=[999.] * 10,
                        raw_racing_frame_ms=[90.] * 180 + [12.] * 20),
                   dict(schema_version=1, frames=20, fixed_steps=20,
                        wall_p95_ms=12., racing=True,
                        raw_frame_ms=[888.] * 20, raw_racing_frame_ms=[14.] * 20)]
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder)/'sample.jsonl'
            path.write_text('\n'.join(map(json.dumps, samples)))
            result = summarize(path, racing_only=True, skip_racing_samples=1, raw_skip_frames=180)
        raw = result['raw_frame_distribution_ms']
        self.assertEqual(raw['samples'], 40)
        self.assertEqual(raw['p50'], 13.)
        self.assertEqual(raw['maximum'], 14.)


if __name__ == '__main__':
    unittest.main()
