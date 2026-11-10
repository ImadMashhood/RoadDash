"""Geometry/clearance regressions for the sidewalk overlay (Shapely authoring venv)."""
import unittest
from shapely import Point,box
from street_first_layout import encode
from metropolis_streetscape import sidewalks,furniture,footprint

def fixture():
    road=box(-6,-100,6,9000)
    walk=box(-8.4,-100,-6,9000).union(box(6,-100,8.4,9000))
    return dict(road_surface=encode(road),sidewalks=encode(walk),city_envelope=encode(box(-100,0,100,9000)),
                race_first=dict(primary_junctions=[dict(point=[0,500])]))

class Sidewalks(unittest.TestCase):
    def test_width_reservation_and_farm_are_preserved(self):
        d=fixture();r,w,g=sidewalks(d)
        self.assertTrue(r.covers(Point(4.99,300)))
        self.assertFalse(r.covers(Point(5.01,300)))
        self.assertTrue(w.covers(box(5,200,8.4,400)))
        self.assertTrue(r.covers(Point(5.9,-50))) # farm is not narrowed
        self.assertAlmostEqual(r.union(w).area,16.8*9100)
        self.assertLess(r.intersection(w).area,1e-6)
        self.assertGreater(g.area,1000)
    def test_repeatable_furniture_has_open_walk_and_junction_setbacks(self):
        d=fixture();_,w,_=sidewalks(d)
        path=dict(points=[[0,12,0],[0,12,8850]],progress=[3150,12000])
        p=furniture(d,[path],w)
        self.assertGreater(len(p),200)
        self.assertEqual(p,furniture(d,[path],w))
        self.assertEqual(len(p),len({a['id'] for a in p}))
        self.assertEqual({a['kind'] for a in p},{'street_lamp','broadleaf','flower_planter','bench','street_bin','hydrant','news_boxes','parking_meter','bus_sign','callbox','mailboxes','recycling_pair','utility_box'})
        for a in p:
            x,z=a['position'];side=1 if x>0 else -1
            self.assertTrue(w.covers(footprint([x+side*1.1,z],0,1.5,2)))
            sx,sz=(.36,.36) if a['kind']=='broadleaf' else (a['size'][0],a['size'][2])
            shape=footprint([x,z],a['heading'],sx,sz).buffer(.12)
            self.assertTrue(w.covers(shape))
            self.assertGreaterEqual(shape.distance(Point(0,500)),20)

if __name__=='__main__':unittest.main()
