"""Malformed source protection and real catalog review completeness."""
import json
from pathlib import Path
import struct
import tempfile
import unittest
import catalog_prop_references as C
import plan_prop_catalog as P
from verify_prop_references import verify


def put(data, offset, *values):
    struct.pack_into('<' + 'I' * len(values), data, offset, *values)


def cel():
    b = bytearray(96); put(b, 0, 0xbc614e); b[16:20] = b'TREE'
    put(b, 32, 4, 4, 1); put(b, 56, 40); b[68:84] = bytes(range(16)); return b


class SourceSafety(unittest.TestCase):
    def test_retail_output_cannot_enter_checkout(self):
        with self.assertRaises(ValueError):
            C.catalog(Path('/not-read'), Path('/not-imported'), C.ROOT / 'assets')
    def test_corrupt_dimensions_and_pixel_pointer_are_rejected(self):
        for offset, value in [(32, 3), (36, 513), (40, 0), (56, 1000)]:
            b = cel(); put(b, offset, value)
            with self.assertRaises(ValueError): C.texture(b, 0)
    def test_scan_keeps_valid_cells_and_reports_bad_magic(self):
        b = cel() + bytearray(64); put(b, 96, 0xbc614e)
        found, rejected = C.cells(b)
        self.assertEqual(list(found), [0]); self.assertEqual(len(rejected), 1)
    def test_aliases_keep_paths_and_cycles_fail(self):
        b = bytearray(40); put(b, 0, 2, 16, 16); put(b, 16, 0x5252414e)
        self.assertEqual([p for p, _, _ in C.leaves(b)], [[0], [1]])
        put(b, 16, 0x4f465354, 0)
        with self.assertRaises(ValueError): C.leaves(b)
    def test_frame_order_and_repeated_views_are_retained(self):
        b = bytearray(200); put(b, 0, 0x5252414e, 20, 150); put(b, 166, 3)
        for i in range(3): put(b, 32 + i * 28, 100)
        refs = C.references(b, 0, 0x5252414e)
        self.assertEqual([r['frame'] for r in refs], [0, 1, 2])
        self.assertEqual([r['cel_offset'] for r in refs], [108] * 3)
    def test_geometry_groups_keep_all_rows_and_columns(self):
        b = bytearray(200); put(b, 0, 0x434c4750); b[21] = 8; b[24:26] = bytes([1, 1])
        put(b, 32, 48, 56); put(b, 48, 64, 72); put(b, 56, 80, 88)
        self.assertEqual([r['cel_offset'] for r in C.references(b, 0, 0x434c4750)], [72, 80, 88, 96])
    def test_pedestrian_category_overrides_vehicle_like_name(self):
        self.assertEqual(C.classify(2, ['BUSM100']), 'pedestrians_animals')
        self.assertEqual(C.classify(None, ['TREOAKA2']), 'trees')
        self.assertEqual(C.classify(None, ['UNKNOWN']), 'unclassified_review')
        self.assertEqual(C.classify(None, ['DEER2']), 'signs_billboards')
    def test_missing_review_images_and_duplicate_ids_fail(self):
        base = dict(records=[dict(id='A', group='x', aliases=[], cells=[], names=[])], asset_count=1,
                    groups={'x': 1}, source_cells=[], scanned_cells=0, unique_images=0, rejected_magic_candidates=[])
        with tempfile.TemporaryDirectory() as t:
            p = Path(t); (p/'catalog.json').write_text(json.dumps(base))
            with self.assertRaises(ValueError): verify(p)
            base['records'] *= 2; base['asset_count'] = 2
            (p/'catalog.json').write_text(json.dumps(base))
            with self.assertRaises(ValueError): verify(p)


class QueueEvidence(unittest.TestCase):
    def test_all_prop_banks_have_open_geometry_and_verified_stage_separation(self):
        source = json.loads((C.ROOT/'docs/research/props/source-inventory.json').read_text())
        queue = json.loads((C.ROOT/'docs/research/props/rebuild-queue.json').read_text())
        expected = {r['id'] for r in source['records'] if r['group'] not in P.EXCLUDED}
        self.assertEqual({j['id'] for j in queue['jobs']}, expected)
        self.assertEqual(len(queue['jobs']), len(expected))
        drafts = [j for j in queue['jobs'] if j['concept_status'] == 'draft_not_approved']
        manifest = json.loads((C.ROOT/'art/props/concept_3d/MANIFEST.json').read_text())
        concept_ids = {c['id'] for c in manifest['cells']}
        batch_file = C.ROOT/'art/props/concept_3d/BATCH_02.json'
        if batch_file.exists():
            for sheet in json.loads(batch_file.read_text()):
                if sheet['status'] == 'draft_not_approved':
                    self.assertEqual(sorted(c['slot'] for c in sheet['cells']), [1,2,3,4,5,6])
                    concept_ids.update(c['id'] for c in sheet['cells'])
        remaining = C.ROOT/'art/props/concept_3d/ALL_REMAINING.json'
        if remaining.exists():
            data = json.loads(remaining.read_text())
            ids = [c['id'] for sheet in data['sheets'] for c in sheet['cells']]
            self.assertEqual(len(ids), len(set(ids)))
            self.assertEqual(set(ids) | set(data['already_covered_ids']), expected)
            for sheet in data['sheets']:
                self.assertTrue(sheet['source_reviewed'])
                if sheet['status'] == 'draft_not_approved':
                    concept_ids.update(c['id'] for c in sheet['cells'])
                    path = C.ROOT/sheet['file']
                    self.assertEqual(__import__('hashlib').sha256(path.read_bytes()).hexdigest(), sheet['sha256'])
        self.assertEqual({j['id'] for j in drafts}, concept_ids)
        for j in drafts:
            self.assertTrue((C.ROOT/j['concept']['file']).is_file())
        for j in queue['jobs']:
            self.assertFalse(j['approved'])
            if j.get('mesh_files'):
                self.assertEqual(j['model_status'], 'draft_volumetric_mesh_not_approved')
                self.assertEqual(len(j['mesh_files']), 3)
                for mesh in j['mesh_files']:self.assertTrue((C.ROOT/mesh).is_file())
            else:self.assertEqual(j['model_status'], 'not_started')
            receipts = (C.ROOT/'docs/testing/props-in-game').glob('native-*.json')
            selected = {i for p in receipts for i in json.loads(p.read_text())['selected_ids']}
            self.assertEqual(j['native_status'], 'development_selected_in_native_scene' if j['id'] in selected else 'not_integrated')
            self.assertIsNone(j['dimensions_m'])


if __name__ == '__main__': unittest.main()
