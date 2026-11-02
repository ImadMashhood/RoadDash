"""Authoring regressions: semantic exceptions, complete exports and hostile GLB inputs."""
import json
from pathlib import Path
import struct
import tempfile
import unittest
from plan_prop_meshes import recipe, ROOT
from verify_prop_meshes import inspect, verify

class SemanticRecipes(unittest.TestCase):
 def test_filename_category_must_not_override_visible_object(self):
  for name,group,expected in [('COWS2','animals','warning_sign'),('CARSLIP2','parked_vehicles_machinery','warning_sign'),('METERMD2','street_lighting_utilities','service_trike'),('SLATS22','fences_barriers','pallets'),('TRAILSN2','parked_vehicles_machinery','information_board'),('BKRGRP10','pedestrians_animals','biker_group')]:
   self.assertEqual(recipe(name,group),expected)
 def test_unknown_subject_does_not_silently_become_a_cube(self):
  with self.assertRaises(ValueError):recipe('UNKNOWN','unclassified_review')
 def test_every_queue_id_has_a_concept_mapped_recipe(self):
  d=json.loads((ROOT/'art/props/mesh_assets/designs.json').read_text())['designs'];q=json.loads((ROOT/'docs/research/props/rebuild-queue.json').read_text())['jobs']
  self.assertEqual({r['id']for r in d},{r['id']for r in q});self.assertEqual(len(d),len(q))
  self.assertTrue(all(r['concept']and r['concept_sha256']for r in d))

class DeliveredGeometry(unittest.TestCase):
 def test_all_delivered_mesh_bytes_lods_and_source_evidence(self):
  report=verify();self.assertEqual(report['models'],369);self.assertEqual(report['glbs'],1107);self.assertEqual(report['render_missing'],[])
 def test_truncated_glb_is_rejected(self):
  source=next((ROOT/'art/props/mesh_assets/models').glob('*.glb'))
  with tempfile.TemporaryDirectory()as td:
   path=Path(td)/'broken.glb';path.write_bytes(source.read_bytes()[:-12])
   with self.assertRaises(ValueError):inspect(path)
 def test_invalid_header_does_not_load(self):
  with tempfile.TemporaryDirectory()as td:
   path=Path(td)/'broken.glb';path.write_bytes(struct.pack('<4sII',b'nope',2,32)+bytes(20))
   with self.assertRaises(ValueError):inspect(path)
if __name__=='__main__':unittest.main()
