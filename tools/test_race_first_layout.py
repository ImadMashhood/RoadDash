"""Regressions for secondary streets competing with the primary race avenue."""
import json
import unittest
from pathlib import Path
from shapely import LineString
from race_first_layout import parallel_run_m

RULES=json.loads((Path(__file__).resolve().parents[1]/'docs/planning/farms-long-city/RACE_LAYOUT_RULES.json').read_text())

class ParallelStreetTests(unittest.TestCase):
    def test_long_close_parallel_street_is_rejected(self):
        road=LineString([(0,0),(0,360)])
        self.assertGreaterEqual(parallel_run_m(LineString([(80,0),(80,360)]),[('race',road)],RULES),48)

    def test_short_local_run_is_caught_even_when_most_of_segment_is_far(self):
        # The previous 60%-of-segment rule missed this 120m parallel section.
        road=LineString([(80,0),(80,120),(400,260),(400,500)])
        self.assertGreaterEqual(parallel_run_m(LineString([(0,0),(0,360)]),[('race',road)],RULES),48)

    def test_perpendicular_crossing_and_well_spaced_avenue_remain(self):
        road=LineString([(0,-500),(0,500)])
        self.assertEqual(parallel_run_m(LineString([(-180,0),(180,0)]),[('race',road)],RULES),0)
        self.assertEqual(parallel_run_m(LineString([(180,0),(180,360)]),[('race',road)],RULES),0)

    def test_both_race_alternatives_are_protected(self):
        routes=[('west',LineString([(-300,0),(-300,360)])),('east',LineString([(300,0),(300,360)]))]
        for x in [-220,220]:
            self.assertGreaterEqual(parallel_run_m(LineString([(x,0),(x,360)]),routes,RULES),48)

if __name__=='__main__':unittest.main()
