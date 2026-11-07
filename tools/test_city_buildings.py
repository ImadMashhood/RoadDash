import copy
import hashlib
import json
import math
from pathlib import Path
import unittest
from city_buildings import generate,validate_config,ROOT,KIT,pools
from verify_building_mesh_assets import read_glb,accessor


class CityBuildingTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.config=json.loads((ROOT/'content/world/city_settings.json').read_text());cls.scene=generate(cls.config)

    def test_full_route_is_deterministic_and_matches_compiled_playable_data(self):
        canonical=lambda value:hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':')).encode()).hexdigest()
        self.assertEqual(canonical(self.scene),canonical(json.loads((ROOT/'content/world/city5300.json').read_text())))
        self.assertEqual(self.scene,generate(self.config))
        self.assertEqual(self.scene['length_m'],5300)
        self.assertEqual(len({b['block']for b in self.scene['lots']}),53)
        changed=copy.deepcopy(self.config);changed['seed']+=1
        self.assertNotEqual(self.scene['lots'],generate(changed)['lots'])
        self.assertEqual(self.scene['facades'],[])

    def test_every_building_has_a_shared_roof_floor_rear_openings_and_canonical_district(self):
        parts=[b[0]for c in self.scene['chunks']for b in c['boxes']]
        n=len(self.scene['lots'])
        self.assertEqual(parts.count('shared_closed_roof'),n)
        self.assertEqual(parts.count('shared_floor_slab'),n)
        self.assertEqual(parts.count('rear_service_door'),n)
        self.assertGreater(parts.count('recessed_rear_glass'),n*2)
        available,_=pools('CITY')
        for lot in self.scene['lots']:
            self.assertIn(lot['family'],{d['id']for d in available[lot['district']]})
            self.assertTrue(math.isfinite(lot['height_m']))
            self.assertGreaterEqual(lot['bays'],4)
            self.assertGreaterEqual(lot['z_start']%100,9.2-1e-6)
            self.assertLessEqual(lot['z_end']%100,90.8+1e-6)
        self.assertEqual(len(self.scene['model_design_ids']),85)
        for chunk in self.scene['chunks']:
            for name,centre,size,color in chunk['boxes']:
                if name=='cross_street_sidewalk':
                    self.assertGreaterEqual(abs(centre[0])-size[0]/2,self.config['road_width_m']/2)
                    self.assertLessEqual(abs(centre[0])+size[0]/2,self.config['parallel_road_offset_m']-self.config['parallel_road_width_m']/2)


    def test_config_rejects_district_gaps_incompatible_pools_and_road_intrusions(self):
        for mutation in ['gap','unavailable','too_deep']:
            c=copy.deepcopy(self.config)
            if mutation=='gap':c['district_zones'][1]['start_m']+=100
            if mutation=='unavailable':c['district_zones'][0]['district']='farms_agriculture'
            if mutation=='too_deep':c['building_depth_m']=40
            with self.assertRaises(ValueError):validate_config(c)

    def test_all_derived_facades_have_actual_front_geometry_and_unchanged_sources(self):
        manifest=json.loads((KIT/'front_modules/MANIFEST.json').read_text())
        self.assertEqual(len(manifest['records']),100)
        self.assertEqual(manifest['glbs'],300)
        for record in manifest['records']:
            triangles=[]
            for lod in record['lods']:
                path=KIT/'front_modules'/lod['file'];self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(),lod['sha256'])
                self.assertEqual(hashlib.sha256((KIT/'models'/lod['file']).read_bytes()).hexdigest(),lod['source_sha256'])
                d,b=read_glb(path)
                for p in d['meshes'][0]['primitives']:
                    bounds=d['accessors'][p['attributes']['POSITION']]
                    self.assertGreaterEqual(bounds['min'][2],-.601)
                    self.assertGreater(bounds['max'][2],bounds['min'][2]-.001)
                    count=d['accessors'][p['attributes']['POSITION']]['count']
                    self.assertTrue(all(i[0]<count for i in accessor(d,b,p['indices'])))
                self.assertGreater(lod['triangles'],30);triangles.append(lod['triangles'])
            self.assertGreaterEqual(triangles[0],triangles[1]);self.assertGreaterEqual(triangles[1],triangles[2])


if __name__=='__main__':unittest.main()
