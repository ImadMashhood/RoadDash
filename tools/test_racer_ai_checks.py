#!/usr/bin/env python3
"""Exercise native receipt gates using isolated fake processes, never the game."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class ReceiptGateTests(unittest.TestCase):
    def run_fixture(self, report=None, *, segment=False, case='race'):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'tools').mkdir()
            runner = root / 'tools/run_racer_ai_checks.py'
            shutil.copy2(Path(__file__).with_name('run_racer_ai_checks.py'), runner)
            binary = root / 'fake_game'
            binary.write_text('#!' + sys.executable + '\nfrom pathlib import Path\n'
                              + ('Path("screenshots").mkdir()\nPath("screenshots/racer-ai.json").write_text('
                                 + repr(json.dumps(report)) + ')\n' if report is not None else ''))
            binary.chmod(0o755)
            command = [sys.executable, str(runner), '--binary', str(binary),
                       '--case', case, '--frames', '180']
            if segment:
                command.append('--segment')
            result = subprocess.run(command, capture_output=True, text=True, check=False)
            receipt = next(root.glob('docs/testing/issue26-racer-ai/*/receipt.json'))
            return result.returncode, json.loads(receipt.read_text())

    def test_clean_process_exit_without_survey_is_not_a_pass(self):
        code, receipt = self.run_fixture()
        self.assertNotEqual(code, 0)
        self.assertFalse(receipt['survey_finished'])
        self.assertEqual(receipt['exit_code'], 0)

    def test_short_survey_is_not_a_completed_sample_window(self):
        code, _ = self.run_fixture({'frames': 90, 'completed': False})
        self.assertNotEqual(code, 0)

    def test_complete_window_and_early_real_finish_are_accepted(self):
        for report in [{'frames': 180, 'completed': False},
                       {'frames': 90, 'completed': False, 'finished_ids': [0]}]:
            code, receipt = self.run_fixture(report)
            self.assertEqual(code, 0)
            self.assertTrue(receipt['survey_finished'])

    def test_early_segment_result_remains_separate_from_race_completion(self):
        code, receipt = self.run_fixture({'frames': 90, 'segment_passed': True}, segment=True)
        self.assertEqual(code, 0)
        self.assertTrue(receipt['segment_passed'])
        self.assertFalse(receipt['completed'])


if __name__ == '__main__':
    unittest.main()
