"""Street-life distribution and safe sidewalk patrol regression fixtures."""
import unittest
from shapely import box,LineString
from metropolis_population import balanced,pedestrians
from metropolis_street_life import parked_cars

class Population(unittest.TestCase):
    def setUp(self):
        self.path=dict(points=[[0,0,0],[0,0,8850]],progress=[3150,12000])
        self.data=dict(race_first=dict(primary_junctions=[]),streets=[])
    def test_budget_is_distributed_before_first_district_exhausts_it(self):
        items=[(x,z) for z in range(0,8000,20) for x in [10,20]]
        first=list(balanced(items,[self.path],lambda x:x))[:32]
        self.assertGreater(len({int(z//500) for _,z in first}),12)
        self.assertEqual(first,list(balanced(items,[self.path],lambda x:x))[:32])
    def test_full_patrol_stays_on_sidewalk_and_avoids_obstacles(self):
        walk=box(-8.4,0,-6,8850).union(box(6,0,8.4,8850))
        # Deliberately block one entire stretch, not just candidate points.
        blocked=dict(position=[7.2,4000],heading=0,kind='bench',size=[2.4,1,200])
        result=pedestrians(self.data,[self.path],walk,[blocked])
        self.assertEqual(len(result),384)
        self.assertEqual(result,pedestrians(self.data,[self.path],walk,[blocked]))
        safe=walk.buffer(-.42).difference(box(6,3900,8.4,4100).buffer(.62))
        for p in result:
            x,z=p['position'];e=p['extent']
            self.assertTrue(safe.covers(LineString([(x,z-e),(x,z+e)])))
        self.assertGreater(len({int(p['station']//1000) for p in result}),7)
    def test_too_narrow_walk_has_no_unsafe_patrols(self):
        self.assertEqual(pedestrians(self.data,[self.path],box(6,0,6.7,8850),[]),[])
    def test_narrow_secondary_parking_keeps_crossing_sweeps_clear(self):
        from street_first_layout import encode
        self.data['streets']=[dict(id='side',points=[[-130,4000],[130,4000]],width_m=8)]
        self.data['race_first']['primary_junctions']=[dict(point=[0,4000])]
        cfg=dict(lane_sections=[dict(route=0,start=0,end=12000,outbound=2,inbound=2)])
        road=box(-130,3996,130,4004)
        cars=parked_cars(self.data,[self.path],road,cfg)
        self.assertTrue(cars)
        for p in cars:
            self.assertGreaterEqual(abs(p['position'][0]),82)
            self.assertAlmostEqual(abs(p['position'][1]-4000),2.95)

if __name__=='__main__':unittest.main()

class Crossings(unittest.TestCase):
    def test_crossings_need_both_sidewalks_clear_complete_corridor_and_spacing(self):
        from metropolis_population import crossing_routes
        walk=box(-12,0,-7,1200).union(box(7,0,12,1200));road=box(-7,0,7,1200)
        walkers=[dict(position=[x,z],station=3300+z,route=0) for z in range(50,1150,50) for x in [-9,9]]
        selected=crossing_routes(walkers,road,walk,[],24)
        self.assertGreaterEqual(len(selected),4)
        self.assertTrue(all(b-a>=240 for a,b in zip(selected,selected[1:])))
        self.assertTrue(all(LineString([p['position'],p['crossing_end']]).length==18 for p in walkers if 'crossing_end' in p))
        blocked=[dict(position=[0,600],heading=0,size=[26,2,1200])]
        fresh=[{k:v for k,v in p.items() if k!='crossing_end'} for p in walkers]
        self.assertEqual(crossing_routes(fresh,road,walk,blocked),[])
        self.assertEqual(crossing_routes(fresh,road.difference(box(-1,0,1,1200)),walk,[]),[])
