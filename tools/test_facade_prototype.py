import json, tempfile, unittest
from pathlib import Path
from build_facade_prototype import ROOT, generate
class FacadeTests(unittest.TestCase):
    def setUp(self):self.families=json.loads((ROOT/'art/buildings/prototype/grammar.json').read_text())['families']
    def test_bay_width_preserves_window_sizes_and_recesses(self):
        for f in self.families:
            for bays in [2,4,8]:
                m=generate(f,bays,3)
                windows=[b for b in m.boxes if b[0]=='recessed_glass']
                self.assertEqual(len(windows),bays*3-1)
                self.assertTrue(all(abs(b[2][0]-f['window_width'])<1e-8 for b in windows))
                self.assertTrue(all(b[1][2]>-4.5 for b in windows))
    def test_corner_adds_second_facade_and_keeps_ground_support(self):
        for f in self.families:
            plain=generate(f,3,3);corner=generate(f,3,3,corner=True)
            self.assertGreater(len(corner.boxes),len(plain.boxes))
            for box in corner.boxes:self.assertGreaterEqual(min(v[1] for v in corner.vertices(box)),-1e-8)
    def test_obj_indices_and_materials_resolve(self):
        with tempfile.TemporaryDirectory() as folder:
            p=Path(folder)/'building.obj';m=generate(self.families[0],3,3);m.export(p)
            lines=p.read_text().splitlines();vertices=[x for x in lines if x.startswith('v ')]
            self.assertEqual(len(vertices),len(m.boxes)*8)
            for face in [x for x in lines if x.startswith('f ')]:
                self.assertTrue(all(1<=int(x)<=len(vertices) for x in face.split()[1:]))
            self.assertTrue(p.with_suffix('.mtl').exists())
    def test_invalid_dimensions_rejected(self):
        for bays,floors in [(0,3),(3,0),(20,3)]:
            with self.assertRaises(ValueError):generate(self.families[0],bays,floors)
if __name__=='__main__':unittest.main()
