"""Profile parity, support tessellation and preserved approved 2D authoring."""
import json, math, unittest
from pathlib import Path
from metropolis_elevation import PROFILE, height
ROOT=Path(__file__).resolve().parents[1]
class ElevationTests(unittest.TestCase):
    def test_baked_paths_and_smooth_grades(self):
        d=json.loads((ROOT/'content/world/metropolis.json').read_text())
        self.assertEqual(d['elevation_profile'],PROFILE)
        for p in d['paths']+d['traffic_paths']:
            for x,y,z in p['points']:self.assertAlmostEqual(y,height(x,z),places=7)
        for a,b in zip(PROFILE['knots'],PROFILE['knots'][1:]):
            self.assertLessEqual(1.875*abs(b[1]-a[1])/(b[0]-a[0]),.04)
        for z,y in PROFILE['knots']:
            self.assertAlmostEqual(height(0,z),y)
            self.assertLess(abs(height(0,z+.1)-height(0,z-.1)),1e-5)
    def test_field_coverage_without_new_boundaries(self):
        from shapely import Polygon, union_all
        d=json.loads((ROOT/'content/world/metropolis.json').read_text())
        for f in d['fields']:
            def faces(shape):
                vertices=[p for ring in shape['rings'] for p in ring]
                return [Polygon([vertices[i] for i in t]) for t in shape['triangles']]
            boundary=union_all(faces(f['shape']))
            patches=[p for s in f['patches'] for p in faces(s)]
            covered=union_all(patches)
            self.assertLess(boundary.symmetric_difference(covered).area,.01)
            self.assertLess(abs(sum(p.area for p in patches)-covered.area),.01)
    def test_actual_triangle_support_error(self):
        d=json.loads((ROOT/'content/world/metropolis.json').read_text())
        groups={'pavement':[s['shape'] for s in d['surfaces']],
                'fields':[s for f in d['fields'] for s in f['patches']]}
        for name,shapes in groups.items():
            worst=0
            for shape in shapes:
                v=[p for ring in shape['rings'] for p in ring]
                for triangle in shape['triangles']:
                    ps=[v[i] for i in triangle]
                    for weights in [(1/3,1/3,1/3),(.5,.5,0),(.5,0,.5),(0,.5,.5)]:
                        x,z=[sum(w*p[k] for w,p in zip(weights,ps)) for k in [0,1]]
                        y=sum(w*height(*p) for w,p in zip(weights,ps))
                        worst=max(worst,abs(y-height(x,z)))
            print(name,'maximum sampled triangle interpolation error',worst,flush=True)
            self.assertLess(worst,.012)
class ElevationOnlyBakeTests(unittest.TestCase):
    def fixture(self):
        import copy
        old_profile = copy.deepcopy(PROFILE)
        for knot in old_profile['knots']:
            knot[1] = 10 + (knot[1] - 10) / 1.25
        old = dict(elevation_profile=old_profile,
                   nodes={'start': [0, height(0, -3000, old_profile), -3000]},
                   edges=[dict(id='farm', points=[[0, height(0, -3000, old_profile), -3000]],
                               progress_fractions=[0], measured_3d_m=1)],
                   validation={})
        new = copy.deepcopy(old)
        new['elevation_profile'] = PROFILE
        new['nodes']['start'][1] = height(0, -3000)
        new['edges'][0]['points'][0][1] = height(0, -3000)
        manifest = dict(elevation_profile=old_profile, paths=[dict(points=[[0, 12, -3000]],progress=[500])],
                        traffic_paths=[dict(points=[[10, 12, -3000]])],
                        track=dict(control_points=[dict(position=[0, 12, -3000])], checkpoints=[.5]),
                        buildings=[dict(id='retained', shape={'rings': [[[1, 2], [3, 4], [5, 6]]]})],
                        street_life=dict(lane_sections=[dict(start=2800, end=3350, outbound=1)]))
        return manifest, old, new

    def test_reheight_preserves_all_other_manifest_data(self):
        import copy
        from bake_metropolis import reheight_manifest
        manifest, old, new = self.fixture()
        before = copy.deepcopy(manifest)
        result = reheight_manifest(manifest, old, new)
        self.assertEqual(manifest, before)
        self.assertEqual(result['paths'][0]['points'][0][1], height(0, -3000))
        self.assertEqual(result['traffic_paths'][0]['points'][0][1], height(10, -3000))
        self.assertEqual(result['track']['control_points'][0]['position'][1], height(0, -3000))
        result['elevation_profile'] = before['elevation_profile']
        for group in ['paths', 'traffic_paths']:
            for path in result[group]:
                for point in path['points']: point[1] = 12
        result['track']['control_points'][0]['position'][1] = 12
        self.assertEqual(result, before)

    def test_rejects_planar_progress_and_metadata_drift(self):
        from bake_metropolis import reheight_manifest
        for case in ['node_x', 'path_z', 'progress', 'id', 'metadata', 'stale_height']:
            manifest, old, new = self.fixture()
            if case == 'node_x': new['nodes']['start'][0] += 1
            elif case == 'path_z': new['edges'][0]['points'][0][2] += 1
            elif case == 'progress': new['edges'][0]['progress_fractions'][0] = .1
            elif case == 'id': new['edges'][0]['id'] = 'different'
            elif case == 'metadata': new['new_layout_field'] = 'changed'
            else: new['edges'][0]['points'][0][1] += 1
            with self.subTest(case=case), self.assertRaises(ValueError):
                reheight_manifest(manifest, old, new)

if __name__=='__main__':unittest.main()
