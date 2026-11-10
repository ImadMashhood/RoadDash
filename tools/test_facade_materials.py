"""Delivered source policy and unchanged geometry, independently checked at all LODs."""
import copy
import hashlib
import json
import unittest
from facade_material_policy import coordinate, FAMILIES, ROOT
from verify_building_mesh_assets import read_glb

KIT=ROOT/'art/buildings/mesh_assets'

class FacadeMaterials(unittest.TestCase):
    def test_delivered_materials_match_source_policy_at_every_lod(self):
        self.assertEqual(len(FAMILIES),24)
        for identity in sorted(FAMILIES):
            for lod in range(3):
                name=f'{identity}_lod{lod}.glb'
                original,_=read_glb(KIT/'models'/name)
                expected={m['name']:coordinate(identity,m) for m in original['materials']}
                for folder in ('front_modules','storey_kit'):
                    actual,_=read_glb(KIT/folder/name)
                    for material in actual['materials']:
                        want=copy.deepcopy(expected[material['name']])
                        # Texture index remapping is specific to each compilation.
                        if 'baseColorTexture' in want['pbrMetallicRoughness']:
                            want['pbrMetallicRoughness']['baseColorTexture']=material['pbrMetallicRoughness']['baseColorTexture']
                        self.assertEqual(material,want,(identity,lod,folder,material['name']))
                        pbr=material['pbrMetallicRoughness']
                        self.assertTrue(all(0<=v<=1 for v in pbr['baseColorFactor']))

    def test_material_only_binary_and_geometry_receipts(self):
        receipt=json.loads((ROOT/'docs/testing/polish-12/pass07/geometry-parity.json').read_text())
        self.assertEqual(len(receipt['records']),144)
        for r in receipt['records']:
            doc,binary=read_glb(ROOT/r['path'])
            geometry={k:v for k,v in doc.items() if k!='materials'}
            self.assertEqual(hashlib.sha256(binary).hexdigest(),r['unchanged_binary_sha256'])
            self.assertEqual(hashlib.sha256(json.dumps(geometry,sort_keys=True).encode()).hexdigest(),r['unchanged_geometry_json_sha256'])

    def test_unplaced_families_preserve_source_and_variant_contrast(self):
        source={'name':'red_brick_dark','pbrMetallicRoughness':{'baseColorFactor':[.82,.82,.82,1], 'baseColorTexture':{'index':0},'metallicFactor':.05}}
        self.assertEqual(coordinate('UNPLACED',source),source)
        result=coordinate('BUSS1',source)['pbrMetallicRoughness']
        self.assertAlmostEqual(result['baseColorFactor'][0],.82*.94)
        self.assertEqual(result['metallicFactor'],0)
        glass=coordinate('BUSS1',{'name':'glass','pbrMetallicRoughness':{}})['pbrMetallicRoughness']
        self.assertEqual(glass['roughnessFactor'],.28)
        self.assertEqual(glass['metallicFactor'],0)
        self.assertGreater(glass['baseColorFactor'][0],.018)

if __name__=='__main__':unittest.main()
