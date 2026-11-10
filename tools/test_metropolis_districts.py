"""Source-layout transition evidence without changing resident geometry or access."""
import hashlib
import copy
import json
import unittest
from collections import Counter
from bake_metropolis import ROOT, facade_role, facade_style, validate_palette

PALETTE=json.loads((ROOT/'content/world/metropolis_facades.json').read_text())

class Districts(unittest.TestCase):
    def test_invalid_transition_weights_order_and_models_are_rejected(self):
        for mutation in ['weights','order','office','unknown','empty']:
            p=copy.deepcopy(PALETTE)
            a=p['approach']
            if mutation=='weights':a['stations'][1]['weights']['workshop']=.8
            if mutation=='order':a['stations'][1]['z']=-200
            if mutation=='office':a['pools']['roadside'][0]='OFFC1'
            if mutation=='unknown':a['pools']['roadside'][0]='missing'
            if mutation=='empty':a['pools']['roadside']=[]
            with self.assertRaises(AssertionError):validate_palette(p)

    def test_capped_tower_uses_lowrise_roles_but_actual_tower_stays_office(self):
        block=dict(id='edge',center=[0,0],district='old_city')
        b=dict(id='edge-tower',role='interior_tower',footprint_index=0)
        self.assertIn(facade_role(b,block,2,PALETTE),['roadside','workshop'])
        self.assertEqual(facade_role(b,block,13,PALETTE),'tower')
        self.assertEqual(facade_role(b,dict(block,center=[0,1800]),2,PALETTE),'tower')

    def test_all_outside_approach_or_taller_assignments_keep_original_identity(self):
        layout=json.loads((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_text())
        from metropolis_street_life import varied_floors
        life=json.loads((ROOT/'content/world/metropolis_street_life.json').read_text())
        checked=0
        for block in layout['blocks']:
            for b in block['buildings']:
                floors=varied_floors(b,block,life)
                if block['center'][1]<1800 and floors<=5:continue
                role='tower' if floors>=13 or b['role']=='interior_tower' else block['district']
                pool=PALETTE['pools'][role]
                seed=int.from_bytes(hashlib.blake2b(b['id'].encode(),digest_size=4).digest(),'little')
                self.assertEqual(facade_style(b,block,floors,PALETTE),PALETTE['families'].index(pool[seed%len(pool)]))
                checked+=1
        self.assertGreater(checked,15000)

    def test_grouped_blend_has_progressive_use_without_uniform_family_rows(self):
        block=dict(id='sample-block',center=[0,0],district='old_city')
        records=[dict(id=f'lot-{i}',role='street_wall',footprint_index=i) for i in range(900)]
        counts={}
        for z in [-200,250,650,1100,1800]:
            block['center'][1]=z
            roles=[facade_role(b,block,2,PALETTE) for b in records]
            counts[z]=Counter(roles)
            self.assertEqual(roles,[facade_role(b,block,2,PALETTE) for b in records])
            self.assertTrue(all(len(set(roles[i:i+3]))==1 for i in range(0,900,3)))
            styles=[facade_style(b,block,2,PALETTE) for b in records]
            self.assertGreater(len(set(styles)),2)
            self.assertGreater(sum(a!=b for a,b in zip(styles,styles[1:])),450)
        self.assertEqual(counts[-200],{'roadside':900})
        self.assertGreater(counts[250]['workshop'],350)
        self.assertGreater(counts[650]['neighborhood'],350)
        self.assertGreater(counts[1100]['neighborhood'],650)
        self.assertEqual(counts[1800],{'old_city':900})

if __name__=='__main__':unittest.main()
