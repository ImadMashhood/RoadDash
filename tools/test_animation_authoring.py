"""Static authoring/reference integrity, with synthetic reference bytes only."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('reference_inventory',ROOT/'tools/measure_reference_animation.py')
reader=importlib.util.module_from_spec(spec);spec.loader.exec_module(reader)
class AnimationAuthoringTests(unittest.TestCase):
    def test_reference_reader_preserves_empty_and_duplicate_slots_and_rejects_truncation(self):
        with tempfile.TemporaryDirectory() as folder:
            file=Path(folder)/'synthetic.dat';file.write_bytes(bytes([2,1,1,2,2,1,1,2,0,0]))
            data=reader.inventory(file,3)
            self.assertEqual(data['populated'],2);self.assertTrue(data['slots'][2]['empty'])
            self.assertEqual(data['slots'][1]['duplicate_of'],0)
            file.write_bytes(file.read_bytes()[:-1])
            with self.assertRaises(ValueError):reader.inventory(file,3)
    def test_all_reference_slots_have_confidence_without_claimed_timing_or_media(self):
        for name,total,populated in [('reference-slots',326,186),('police-slots',45,33)]:
            data=json.loads((ROOT/f'docs/animation/{name}.json').read_text())
            self.assertEqual(len(data['slots']),total);self.assertEqual(data['populated'],populated)
            self.assertFalse(data['contains_media'])
            for slot in data['slots']:
                self.assertTrue(slot['classification']);self.assertTrue(slot['confidence'])
                self.assertIsNone(slot['original_duration_ticks']);self.assertFalse(slot['sequence_order_verified'])
    def test_catalog_pose_renders_and_semantic_coverage_are_complete(self):
        base=ROOT/'art/animation';catalog=json.loads((base/'manifest.json').read_text())
        ids=[p['id'] for p in catalog['poses']]
        self.assertEqual(len(ids),len(set(ids)));self.assertGreaterEqual(len(ids),247)
        mapped={id for c in catalog['use_cases'] for id in c['pose_ids']}
        self.assertEqual(set(ids),mapped)
        for p in catalog['poses']:
            self.assertTrue((base/p['screenshot']).is_file(),p['id'])
            self.assertTrue((base/f"poses/{p['id']:04d}.json").is_file())
        self.assertFalse(catalog['runtime_connected']);self.assertFalse(catalog['physics_changed'])
        self.assertTrue((base/'legacy-mounted.json').exists())
if __name__=='__main__':unittest.main()
