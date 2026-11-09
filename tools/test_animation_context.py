import importlib.util,json,struct,sys,tempfile,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
from read_animation_context import Bytes,pe_data,resources,cans_groups,sequence
from catalog_animation_context import catalog
class ContextTests(unittest.TestCase):
    def test_pe_ranges_reject_crossing_and_unmapped_addresses(self):
        b=bytearray(464);b[:2]=b'MZ';struct.pack_into('<I',b,60,64);b[64:68]=b'PE\0\0';struct.pack_into('<H',b,68,0x14c);struct.pack_into('<H',b,70,1);struct.pack_into('<H',b,84,224);struct.pack_into('<H',b,88,0x10b);struct.pack_into('<I',b,116,0x400000);struct.pack_into('<III',b,324,0x1000,64,400);b[400:404]=b'TEST'
        self.assertEqual(pe_data(b,0x401000,4),b'TEST')
        for address,size in [(0x40103f,2),(0x402000,1)]:
            with self.assertRaises(ValueError):pe_data(b,address,size)
        with self.assertRaises(ValueError):pe_data(b[:100],0x401000,4)
    def test_synthetic_groups_keep_cell_views_separate_from_phases(self):
        b=bytearray(128)
        def word(p,v):struct.pack_into('<I',b,p,v)
        word(0,0x43414e53);word(4,60);word(8,2);word(12,1);word(16,1);word(20,1);word(24,140);word(32,4<<16)
        word(64,8);word(72,24);word(80,1);word(84,0);word(96,8);word(104,28);word(112,42)
        groups=cans_groups(bytes(b));raw=struct.pack('<iBBBB',42,0,0,0,0)+struct.pack('<iBBBB',42,0,2,0,0)
        phases=sequence(raw,groups)
        self.assertEqual([p['rear']['slot'] for p in phases],[3,3]);self.assertEqual([p['control_action'] for p in phases],[0,2]);self.assertEqual([p['duration_ticks'] for p in phases],[7,7]);self.assertEqual(len(groups),1)
        with self.assertRaises(ValueError):cans_groups(bytes(b[:80]))
        with self.assertRaises(ValueError):sequence(raw[:-1],groups)
    def test_resources_reject_out_of_range_payloads(self):
        b=bytearray(73);b[:4]=b'CRSR';b[12:16]=b'LBTR';struct.pack_into('<I',b,16,24);b[24:28]=b'LBTR';struct.pack_into('<I',b,32,1);b[40:44]=b'SNAC';struct.pack_into('<III',b,44,1,72,1);b[72]=7
        self.assertEqual(resources(bytes(b))[('CANS',1)],bytes([7]))
        with self.assertRaises(ValueError):resources(bytes(b[:-1]))
    def test_recovered_combat_order_mirroring_and_repeated_phases(self):
        data=json.loads((ROOT/'docs/animation/context/catalog.json').read_text());actions={a['id']:a for a in data['actions']};self.assertEqual(len(actions),104)
        for name,slots in [('PUNCH_LEFT_FOREHAND',[176,177,178]),('PUNCH_LEFT_BACKHAND',[179,178,178]),('KICK_LEFT',[225,226]),('CHAIN_LEFT_FOREHAND',[81,82,83,84,85]),('CLUB_LEFT_FOREHAND',[140,141,142])]:
            self.assertEqual([p['rear']['slot'] for p in actions[name]['phases']],slots)
            self.assertEqual([p['label'] for p in actions[name]['phases']],[f'{name}_{i:02d}' for i in range(1,len(slots)+1)])
        self.assertTrue(all(p['rear']['mirror_x'] for p in actions['PUNCH_RIGHT_FOREHAND']['phases']))
        self.assertFalse(any(p['rear']['mirror_x'] for p in actions['PUNCH_LEFT_FOREHAND']['phases']))
        self.assertEqual([p['rear']['slot'] for p in actions['AI_POST_HIT_GESTURE']['phases']],[44,45])
        self.assertEqual([p['rear']['slot'] for p in actions['RUN_TO_BIKE']['phases']],list(range(297,303)))
        self.assertFalse(actions['CLUB_STEAL_UNAVAILABLE_LEFT']['available']);self.assertEqual(actions['CLUB_STEAL_UNAVAILABLE_LEFT']['fallback_code'],16)
        self.assertEqual(len(data['rider_slot_usage']),326);self.assertEqual(len(data['police_slot_usage']),45)
        self.assertEqual([r['slot'] for r in data['rider_slot_usage'] if r['status']=='unused-populated'],[64])
    def test_catalog_rebuild_matches_and_context_poses_have_exact_source_links(self):
        raw=json.loads((ROOT/'docs/animation/context/playback.json').read_text());expected=json.loads((ROOT/'docs/animation/context/catalog.json').read_text());self.assertEqual(catalog(raw),expected)
        self.assertEqual(sum(len(s['phases']) for s in raw['banks'][0]),159);self.assertEqual(sum(len(s['phases']) for s in raw['banks'][1]),35)
        base=ROOT/'art/animation/context';manifest=json.loads((base/'manifest.json').read_text());assembly=json.loads((base/'assembly.json').read_text());self.assertEqual(len(manifest['poses']),138)
        labels={p['label']:p for a in expected['actions'] if a['bank']=='rider' for p in a['phases']}
        for d in manifest['poses']:
            pose=json.loads((base/f'poses/{d["id"]:04d}.json').read_text());native=labels[d['name']];self.assertEqual(pose['source_slots'],[native['rear']['slot']]);self.assertEqual(assembly[str(d['id'])]['source'],native['rear']);self.assertEqual(d['review'],'draft')
            self.assertLessEqual(max(pose['contact_errors_m'].values(),default=0),.005)
        self.assertEqual(len(json.loads((base/'bike-poses.json').read_text())),16)
    def test_authoring_hand_states_share_runtime_profile_and_do_not_leak(self):
        import hashlib
        base=ROOT/'art/animation/context'
        source=json.loads((base/'hand-authoring.json').read_text())
        expected=hashlib.sha256((ROOT/'content/hand-contacts.json').read_bytes()).hexdigest()
        self.assertEqual(source['profile_sha256'],expected)
        self.assertEqual(source['source_scale'],1.2)
        assembly=json.loads((base/'assembly.json').read_text())
        self.assertEqual(len(assembly),154)
        for item in assembly.values():
            for side in ['L','R']:
                state=item['hand_values'][side]
                self.assertEqual(set(state),{'grip','fist'})
                self.assertTrue(all(0<=x<=1 for x in state.values()))
                self.assertLessEqual(sum(state.values()),1)
        for d in json.loads((base/'manifest.json').read_text())['poses']:
            for side in ['L','R']:
                if 'grip.'+side in d['held_contacts']:
                    self.assertEqual(assembly[str(d['id'])]['hand_values'][side],{'grip':1.,'fist':0.})
        for key,side in [(2222,'L'),(2232,'R')]:
            self.assertEqual(assembly[str(key)]['hand_values'][side],{'grip':0.,'fist':1.})
        for key,side in [(2100,'L'),(2110,'R'),(2160,'L'),(2170,'R')]:
            self.assertEqual(assembly[str(key)]['hand_values'][side],{'grip':1.,'fist':0.})
        fixture=json.loads((ROOT/'docs/testing/part02-authoring/fold-fixtures.json').read_text())
        self.assertEqual(fixture['profile_sha256'],expected)

    def test_whole_body_kicks_tuck_and_punches_keep_their_supports(self):
        import math
        base=ROOT/'art/animation/context/poses'
        def pose(id):return json.loads((base/f'{id}.json').read_text())
        neutral=pose(2020);tuck=pose(2030);kick=pose(2261);left=pose(2222);right=pose(2232)
        self.assertGreater(kick['bone_world']['pelvis'][2][3]-neutral['bone_world']['pelvis'][2][3],.14)
        self.assertNotIn('seat',kick['held_contacts']);self.assertIn('peg.R',kick['held_contacts']);self.assertIn('grip.L',kick['held_contacts']);self.assertIn('grip.R',kick['held_contacts'])
        self.assertGreater(neutral['bone_world']['head'][2][3]-tuck['bone_world']['head'][2][3],.18)
        self.assertLess(left['bone_world']['head'][0][3],-.08);self.assertGreater(right['bone_world']['head'][0][3],.08)
        self.assertAlmostEqual(left['bone_world']['pelvis'][0][3],-right['bone_world']['pelvis'][0][3],places=5)
        for p in [neutral,tuck,kick,left,right]:self.assertLess(max(p['contact_errors_m'].values()),.005)
if __name__=='__main__':unittest.main()
