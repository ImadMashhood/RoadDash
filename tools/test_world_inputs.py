#!/usr/bin/env python3
"""Verify imported design inputs and runtime quotas stay in agreement (stdlib)."""
import csv
import hashlib
import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'docs/world/source'


def load(path):
    return json.loads(path.read_text(encoding='utf-8'))


class WorldInputs(unittest.TestCase):
    def setUp(self):
        self.source = load(SOURCE / 'RoadDash_Base_World_Spec_v1.json')
        self.world = load(ROOT / 'content/world/valley_world_v1.json')
        self.catalog = load(ROOT / 'content/world/building_archetypes_v1.json')

    def test_source_snapshots_match_manifest(self):
        manifest = load(SOURCE / 'MANIFEST.json')
        for record in manifest['files']:
            data = (SOURCE / record['path']).read_bytes()
            self.assertEqual(len(data), record['bytes'])
            self.assertEqual(hashlib.sha256(data).hexdigest(), record['sha256'])

    def test_csv_matches_all_source_section_fields(self):
        with (SOURCE / 'RoadDash_Base_World_Sections_v1.csv').open(encoding='utf-8', newline='') as stream:
            rows = list(csv.DictReader(stream))
        self.assertEqual(len(rows), 15)
        for row, section in zip(rows, self.source['sections']):
            self.assertEqual(set(row), set(section))
            for key, value in section.items():
                self.assertEqual(float(row[key]) if isinstance(value, (int, float)) else row[key], value)

    def test_runtime_section_conversion_preserves_every_quota_and_station(self):
        districts = {'Farms': 'farms', 'Transition': 'rural_edge', 'Suburban': 'suburban',
                     'Inner city': 'inner_city', 'City core': 'city_core',
                     'City to hills': 'city_to_hills', 'Hills': 'hills'}
        self.assertEqual(len(self.world['sections']), 15)
        for original, runtime in zip(self.source['sections'], self.world['sections']):
            for key, value in original.items():
                self.assertEqual(runtime['district'] if key == 'zone' else runtime[key],
                                 districts[value] if key == 'zone' else value)
        for key in ['building_target_total', 'near_road', 'secondary_row', 'archetype_counts', 'road_length_m']:
            self.assertEqual(self.source[key], self.world[key])
        self.assertEqual(self.world['seed'], self.source['world_seed'])
        self.assertEqual(self.world['track_id'], self.source['source_course'])

    def test_archetypes_preserve_identity_budget_and_height_guidance(self):
        families = {'Agricultural': 'farm', 'Residential': 'residential',
                    'Commercial': 'commercial', 'Urban': 'urban', 'Hillside': 'hill'}
        self.assertEqual(len(self.catalog['archetypes']), 19)
        for original, runtime in zip(self.source['archetypes'], self.catalog['archetypes']):
            for key in ['id', 'name', 'quantity', 'brief']:
                self.assertEqual(original[key], runtime[key])
            self.assertEqual(runtime['family'], families[original['group']])
            heights = [float(n) for n in re.findall(r'\d+', original['height_guidance'])]
            self.assertEqual(heights, [runtime['height_min_m'], runtime['height_max_m']])


if __name__ == '__main__':
    unittest.main()
