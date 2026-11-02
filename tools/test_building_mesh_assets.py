import json
import unittest
from build_city_preview import DEFAULT,build
from verify_building_mesh_assets import FOLDER,verify


class BuildingMeshTests(unittest.TestCase):
    def test_real_glbs_have_finite_volumetric_geometry_materials_and_lods(self):
        report=verify()
        self.assertEqual(report['models'],6)
        self.assertEqual(report['glb_files'],18)

    def test_each_reference_has_distinct_authored_architecture(self):
        records=json.loads((FOLDER/'MANIFEST.json').read_text())['records']
        by_id={r['id']:r for r in records}
        for r in records:
            self.assertGreater(r['geometry_parts']['individual_bricks_front'],100)
            for part in ['roof_deck','chimney_stack','hvac_box','shop_glass','door_handle','side_window_glass']:
                self.assertGreater(r['geometry_parts'][part],0)
        self.assertIn('arched_brick_lintel',by_id['BUSS1']['geometry_parts'])
        self.assertIn('sloped_fabric_canopy',by_id['BUSS2']['geometry_parts'])
        self.assertIn('sagged_motel_banner',by_id['BUSS4']['geometry_parts'])
        self.assertIn('bay_angled_glass',by_id['BUSS6']['geometry_parts'])

    def test_model_city_uses_only_actual_models_and_no_flat_facade_surfaces(self):
        config=json.loads((DEFAULT/'config.json').read_text())
        scene=build(config,modeled=True)
        self.assertEqual(scene['geometry_mode'],'assembled_glb_buildings')
        self.assertGreater(len(scene['model_design_ids']),70)
        self.assertEqual(scene['facades'],[])
        self.assertEqual(scene['texture_sources'],[])
        self.assertFalse(any(c['boxes'] for c in scene['chunks'] if c['name'].startswith('buildings_')))
        self.assertEqual({l['family'] for l in scene['lots']},set(scene['model_design_ids']))


if __name__=='__main__':unittest.main()
