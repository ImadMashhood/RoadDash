import copy
import json
import math
import unittest
from build_city_preview import DEFAULT, ROOT, build, validate, load_assets
import hashlib
import tempfile


class CityPreviewTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.config = json.loads((DEFAULT/'config.legacy-1km.json').read_text())
        cls.scene = build(cls.config)

    def test_exact_kilometre_and_contiguous_frontage(self):
        self.assertEqual(self.scene['length_m'], 1000)
        lots = self.scene['lots']
        self.assertEqual(len({x['id'] for x in lots}), len(lots))
        for block in range(10):
            for side in [-1, 1]:
                for row in ['avenue', 'outer']:
                    group = sorted((x for x in lots if (x['block'], x['side'], x['row']) == (block, side, row)), key=lambda x:x['z_start'])
                    self.assertAlmostEqual(group[0]['z_start'],block*100+6)
                    self.assertAlmostEqual(group[-1]['z_end'],block*100+94)
                    for a,b in zip(group,group[1:]):
                        self.assertAlmostEqual(a['z_end'],b['z_start'])

    def test_clear_roads_and_rear_courts(self):
        for lot in self.scene['lots']:
            lo=abs(lot['center_x'])-lot['depth']/2
            hi=abs(lot['center_x'])+lot['depth']/2
            self.assertGreaterEqual(lo, 10.2-1e-6)
            self.assertLessEqual(hi, 59.8+1e-6)
            if lot['row']=='avenue': self.assertLessEqual(hi,28.2+1e-6)
            else: self.assertGreaterEqual(lo,41.8-1e-6)

    def test_seeded_layout_and_family_coverage(self):
        other=build(self.config)
        self.assertEqual(self.scene,other)
        changed=copy.deepcopy(self.config);changed['seed']+=1
        self.assertNotEqual(self.scene['lots'],build(changed)['lots'])
        self.assertEqual({x['family'] for x in self.scene['lots']},{x['id'] for x in load_assets(self.config)})

    def test_supported_finite_positive_geometry_and_batch_budget(self):
        self.assertLessEqual(len(self.scene['chunks']),65)
        self.assertLess(sum(len(c['boxes']) for c in self.scene['chunks']),160000)
        for chunk in self.scene['chunks']:
            for name,centre,size,color in chunk['boxes']:
                self.assertTrue(all(math.isfinite(v) for v in (*centre,*size)))
                self.assertGreater(min(size),0)
                self.assertTrue(all(0<=v<=255 for v in color))
                if chunk['name'].startswith('buildings'):
                    self.assertGreaterEqual(centre[1]-size[1]/2,-1e-6)

    def test_invalid_authoring_rejected(self):
        for key,value in [('length_m',999),('road_width_m',-1),('block_pitch_m',0),('building_depth_m',28),('tree_spacing_m',float('nan')),('tree_spacing_m',.5),('height_scale_range',[1.2,.8])]:
            c=copy.deepcopy(self.config);c[key]=value
            with self.assertRaises(ValueError):validate(c)

    def test_every_new_facade_is_used_without_changing_its_pixels(self):
        assets=load_assets(self.config)
        self.assertEqual(len(assets),100)
        self.assertEqual(len(self.scene['facades']),17)
        ids={a['id'] for a in assets}
        self.assertEqual({q['asset'] for b in self.scene['facades'] for q in b['quads']},ids)
        self.assertEqual({lot['family'] for lot in self.scene['lots']},ids)
        for a in assets:
            self.assertEqual(hashlib.sha256((ROOT/a['sheet']).read_bytes()).hexdigest(),a['source_sha256'])
        # Prevent the rejected generated-window approximation from returning.
        self.assertFalse(any('window' in box[0] or box[0]=='shop_sign'
                             for c in self.scene['chunks'] for box in c['boxes']))

    def test_uvs_stay_on_their_exact_facade_and_geometry_is_finite(self):
        assets={a['id']:a for a in load_assets(self.config)}
        for b in self.scene['facades']:
            for q in b['quads']:
                a=assets[q['asset']];x0,y0,x1,y1=a['rect_px'];w,h=a['pixels']
                self.assertEqual(b['sheet'],a['sheet'])
                for u,v in q['uvs']:
                    self.assertTrue(x0/w<=u<=x1/w and y0/h<=v<=y1/h)
                self.assertTrue(all(math.isfinite(v) for p in q['positions'] for v in p))
        proof=[q for b in self.scene['facades'] for q in b['quads'] if q['owner']=='proof:BUSS1']
        self.assertGreater(len({round(p[0],3) for q in proof for p in q['positions']}),2)

    def test_painted_signs_are_not_mirrored_on_either_street_side(self):
        lots={x['id']:x for x in self.scene['lots']}
        checked=0
        for batch in self.scene['facades']:
            for q in batch['quads']:
                if q['role']!='front':continue
                inward=1 if q['owner'].startswith('proof:') else lots[q['owner']]['side']*(1 if lots[q['owner']]['row']=='avenue' else -1)
                u0=q['uvs'][0][0];z0=q['positions'][0][2]
                for p,uv in zip(q['positions'][1:],q['uvs'][1:]):
                    if abs(uv[0]-u0)>1e-6:
                        self.assertGreater((p[2]-z0)*(uv[0]-u0)*inward,0)
                        checked+=1
        self.assertGreater(checked,1000)

    def test_changed_or_invalid_art_catalog_is_rejected(self):
        catalog=json.loads((ROOT/self.config['facade_catalog']).read_text())
        for field,value in [('source_sha256','invalid'),('rect_px',[-1,0,100,100])]:
            bad=copy.deepcopy(catalog);bad['assets'][0][field]=value
            with tempfile.TemporaryDirectory() as folder:
                path=ROOT/folder/'catalog.json';path.write_text(json.dumps(bad))
                c=copy.deepcopy(self.config);c['facade_catalog']=str(path)
                with self.assertRaises(ValueError):load_assets(c)


if __name__=='__main__':unittest.main()
