"""Authoring regressions for per-building skyline and curbside parking."""
import unittest,json
from pathlib import Path
from collections import Counter
from shapely import box,Point
from metropolis_street_life import varied_floors,parked_cars,traffic_paths
CFG=json.loads((Path(__file__).resolve().parents[1]/'content/world/metropolis_street_life.json').read_text())
class Life(unittest.TestCase):
 def test_seeded_frontages_mix_heights_and_preserve_interior_towers(self):
  b=dict(core_score=.5);items=[dict(id=f'lot-{i}',role='street_wall',floors=7) for i in range(100)]
  a=[varied_floors(x,b,CFG) for x in items]
  self.assertEqual(a,[varied_floors(x,b,CFG) for x in items]);self.assertGreater(len(set(a)),12);self.assertLessEqual(min(a),3);self.assertGreater(max(a),18)
  self.assertEqual(varied_floors(dict(id='tower',role='interior_tower',floors=31),b,CFG),31)
 def test_transition_caps_even_interior_towers_and_grows_smoothly(self):
  tower=dict(id='edge-tower',role='interior_tower',floors=31)
  heights=[varied_floors(tower,dict(core_score=.5,center=[0,z]),CFG) for z in range(-200,1801,100)]
  self.assertEqual(heights[0],2);self.assertEqual(heights[-1],31);self.assertEqual(heights,sorted(heights));self.assertLessEqual(max(b-a for a,b in zip(heights,heights[1:])),3)
 def test_parked_vehicles_are_fully_on_asphalt_and_outside_junctions(self):
  p=dict(points=[[0,12,0],[0,12,8850]],progress=[3150,12000]);d=dict(race_first=dict(primary_junctions=[dict(point=[0,500])]))
  cars=parked_cars(d,[p],box(-5,0,5,8850),CFG)
  self.assertGreater(len(cars),20);self.assertEqual(cars,parked_cars(d,[p],box(-5,0,5,8850),CFG))
  from metropolis_streetscape import footprint
  for c in cars:
   shape=footprint(c['position'],c['heading'],1.8,3.8).buffer(.1);self.assertTrue(box(-5,0,5,8850).covers(shape));self.assertGreaterEqual(shape.distance(Point(0,500)),25)
 def test_traffic_uses_reserved_street_centre_without_moving_race(self):
  p=dict(points=[[0,12,0],[1,12,100],[0,12,200]],progress=[0,3200,12000]);original=json.dumps(p)
  t=traffic_paths([p])[0];self.assertEqual(t['points'][1][0],0);self.assertEqual(json.dumps(p),original);self.assertEqual(t['progress'],p['progress'])
if __name__=='__main__':unittest.main()
