import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import run_slice_checks as checks


class SliceChecks(unittest.TestCase):
    def report(self):
        return {"stages":22,"ended_in_garage":True,"race_seed":1234,"armed_rivals":list(range(1,8)),
                "layouts":[{"root":[1440,900],"window":[1440,900],"camera":[1440,900]}]*5}

    def test_partial_or_wrong_seed_weapon_and_layout_reports_fail(self):
        checks.validate_entry(self.report(),1234)
        for key,value in [("stages",21),("race_seed",9),("armed_rivals",[0,1,2,3,4,5,6]),("layouts",[])]:
            report=self.report();report[key]=value
            with self.assertRaises(ValueError):checks.validate_entry(report,1234)

    def test_runner_isolates_saves_and_preserves_existing_receipts(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);binary=root/'binary';binary.write_bytes(b'fixture')
            def invoke(command,**kwargs):
                case=kwargs['cwd'];self.assertEqual(kwargs['env']['ROADDASH_SAVE_DIR'],str(case/'test-save'))
                path=case/'screenshots';path.mkdir();(path/'frontend-report.json').write_text(json.dumps(self.report()))
                return subprocess.CompletedProcess(command,0)
            with patch.object(checks.subprocess,'run',side_effect=invoke):
                self.assertTrue(checks.run(binary,root/'out')['passed'])
            with self.assertRaises(FileExistsError):checks.run(binary,root/'out')

    def test_native_failure_and_timeout_are_reported_as_failures(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);binary=root/'binary';binary.write_bytes(b'fixture')
            with patch.object(checks.subprocess,'run',return_value=subprocess.CompletedProcess([],101)):
                self.assertFalse(checks.run(binary,root/'failed')['passed'])
            with patch.object(checks.subprocess,'run',side_effect=subprocess.TimeoutExpired([],1)):
                self.assertFalse(checks.run(binary,root/'timeout')['passed'])


if __name__=='__main__':unittest.main()
