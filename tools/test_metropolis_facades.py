"""District assignment and independent source-band atlas checks; original local assets only."""
import copy
import hashlib
import json
import unittest
from collections import Counter, defaultdict
from PIL import Image
from bake_metropolis import ROOT, facade_style, validate_palette
from metropolis_street_life import varied_floors

PALETTE = json.loads((ROOT/'content/world/metropolis_facades.json').read_text())

class Facades(unittest.TestCase):
    def test_rejects_wrong_types_missing_pools_and_unknown_families(self):
        validate_palette(PALETTE)
        for mutation in ['farm', 'missing', 'unknown', 'duplicate', 'empty']:
            p = copy.deepcopy(PALETTE)
            if mutation == 'farm': p['families'][0] = 'BARN12'
            if mutation == 'missing': del p['pools']['old_city']
            if mutation == 'unknown': p['pools']['old_city'].append('NO_SUCH_MODEL')
            if mutation == 'duplicate': p['families'][0] = p['families'][1]
            if mutation == 'empty': p['pools']['tower'] = []
            with self.assertRaises(AssertionError): validate_palette(p)
        with self.assertRaises(KeyError):
            facade_style(dict(id='x',role='street_wall'),dict(district='unknown'),4,PALETTE)

    def test_all_placed_buildings_use_district_or_tower_roles_deterministically(self):
        layout = json.loads((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_text())
        world = json.loads((ROOT/'content/world/metropolis.json').read_text())
        life = json.loads((ROOT/'content/world/metropolis_street_life.json').read_text())
        records = {b['id']: b for b in world['buildings']}
        coverage = defaultdict(Counter)
        for block in layout['blocks']:
            for b in block['buildings']:
                floors = varied_floors(b, block, life)
                style = facade_style(b, block, floors, PALETTE)
                self.assertEqual(style,facade_style(b, block, floors, PALETTE))
                self.assertEqual(records[b['id']]['style'],style)
                self.assertEqual(records[b['id']]['floors'],floors)
                role = 'tower' if floors >= 13 or b['role'] == 'interior_tower' else block['district']
                design = PALETTE['families'][style]
                self.assertIn(design,PALETTE['pools'][role])
                coverage[role][design] += 1
        self.assertEqual(set(x for pool in coverage.values() for x in pool),set(PALETTE['families']))
        self.assertEqual(sum(sum(x.values()) for x in coverage.values()),19068)
        print(json.dumps({k:dict(v) for k,v in coverage.items()},sort_keys=True))

    def test_all_three_bands_and_five_mips_are_actual_source_crops(self):
        provenance=json.loads((ROOT/'assets/metropolis/PROVENANCE.json').read_text())
        atlas_root=ROOT/'art/buildings/mesh_assets/far_facades'
        atlas=json.loads((atlas_root/'MANIFEST.json').read_text())
        raw=(ROOT/'assets/metropolis/facades.rgba').read_bytes()
        self.assertEqual(hashlib.sha256(raw).hexdigest(),provenance['sha256'])
        self.assertEqual(hashlib.sha256((ROOT/'content/world/metropolis_facades.json').read_bytes()).hexdigest(),provenance['palette_sha256'])
        self.assertEqual(len(provenance['sources']),72)
        offset=0
        images=[Image.open(atlas_root/f'albedo-mip{l}.png').convert('RGBA') for l in range(5)]
        for i,source in enumerate(provenance['sources']):
            self.assertEqual((source['design'],source['band']),(PALETTE['families'][i//3],i%3))
            design='CLSTGA11' if source['design']=='BUSS4' and i%3==1 else source['design']
            self.assertEqual(source['source_design'],design)
            tile=next(t for t in atlas['records'] if t['design']==design and t['band']==i%3)
            self.assertEqual(source['source_sha256'],tile['source_sha256'])
            for level,image in enumerate(images):
                side=96>>level
                x,y=(round(tile['uv_min'][n]*image.size[n]-.5) for n in range(2))
                crop=image.crop((x,y,x+side,y+side))
                expected=Image.new('RGBA',crop.size,(135,126,108,255));expected.alpha_composite(crop)
                data=expected.tobytes()
                self.assertEqual(raw[offset:offset+len(data)],data)
                offset+=len(data)
        self.assertEqual(offset,len(raw))
        shader=(ROOT/'assets/shaders/metropolis_facade.wgsl').read_text()
        self.assertIn('tile % 8u',shader);self.assertIn('tile / 8u',shader)
        self.assertIn('1024.0, 1152.0',shader)
        self.assertEqual((provenance['atlas_columns'],provenance['atlas_rows']),(8,9))

if __name__=='__main__': unittest.main()
