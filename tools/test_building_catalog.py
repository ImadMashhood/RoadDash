import json
import unittest
from collections import Counter
from pathlib import Path
from collect_building_catalog import OUT,ROOT,collect


class CatalogTests(unittest.TestCase):
    def test_every_reference_has_one_typed_design_and_reviewed_sheet(self):
        plan=json.loads((OUT/'catalog_designs.json').read_text())['designs']
        reference=json.loads((ROOT/'art/buildings/concept_3d/MANIFEST.json').read_text())
        # Manifest schema is the authoritative ID ordering, not filename guesses.
        sheets=reference['records']
        expected={identity for sheet in sheets for identity in sheet['ids_in_row_major_order']}
        self.assertEqual(len(plan),146)
        self.assertEqual({d['id']for d in plan},expected)
        self.assertEqual(Counter(d['asset_type']for d in plan),dict(facade=100,structure=28,surface_module=18))
        for d in plan:
            self.assertTrue((ROOT/'art/buildings/concept_3d'/d['reference_sheet']).is_file())
            self.assertGreater(min(d[k]for k in ['width','height','depth']),0)

    def test_complete_exports_preserve_accepted_models_and_all_types(self):
        catalog=collect()
        self.assertEqual(catalog['missing_ids'],[])
        self.assertEqual(catalog['glb_files'],438)
        by_id={r['id']:r for r in catalog['records']}
        first=json.loads((OUT/'MANIFEST.json').read_text())['records']
        for r in first:self.assertEqual(r['lods'],by_id[r['id']]['lods'])
        for identity in ['CHURCH2','BARN12','GASA2','WATRTNK2']:
            self.assertEqual(by_id[identity]['asset_type'],'structure')
        for identity in ['METL21CC','TUNN31RW','TOP']:
            self.assertEqual(by_id[identity]['asset_type'],'surface_module')


if __name__=='__main__':unittest.main()
