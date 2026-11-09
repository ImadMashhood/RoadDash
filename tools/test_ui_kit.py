"""Portable parser/coverage/artist-handoff tests; no retail bytes required."""
import json
from pathlib import Path
import struct
import unittest
import xml.etree.ElementTree as ET
from catalog_ui_references import PE, ROOT, bob_size, external_output, menu_items, resolve_caption, dialog_template

def synthetic_pe(resource=None):
    b=bytearray(2048);b[:2]=b'MZ';struct.pack_into('<I',b,60,128);b[128:132]=b'PE\0\0';struct.pack_into('<H',b,134,1);struct.pack_into('<H',b,148,224);struct.pack_into('<H',b,152,0x10b);struct.pack_into('<I',b,180,0x400000);struct.pack_into('<I',b,264,0x1000 if resource else 0);section=376;struct.pack_into('<III',b,section+12,0x1000,1024,512)
    if resource:b[512:512+len(resource)]=resource
    return bytes(b)

class ReaderTests(unittest.TestCase):
    def test_pe_bounds_and_mapping(self):
        p=PE(synthetic_pe());self.assertEqual(p.va(0x401000,2),b'\0\0');self.assertEqual(p.resources(),[])
        for data in [b'',b'MZ',b'NO'+bytes(2000)]:
            with self.assertRaises(ValueError):PE(data)
        with self.assertRaises(ValueError):p.rva(0x1fff,32)
    def test_recursive_resource_directory_is_rejected(self):
        r=bytearray(24);struct.pack_into('<H',r,14,1);struct.pack_into('<II',r,16,6,0x80000000)
        with self.assertRaisesRegex(ValueError,'cycle'):PE(synthetic_pe(r)).resources()
    def test_short_string_resource_is_rejected(self):
        r=bytearray(180)
        for off,key,child in [(0,6,24),(24,1,48)]:struct.pack_into('<H',r,off+14,1);struct.pack_into('<II',r,off+16,key,0x80000000|child)
        struct.pack_into('<H',r,62,1);struct.pack_into('<II',r,64,1033,80);struct.pack_into('<IIII',r,80,0x1000+128,3,0,0);r[128:131]=b'\x03\x00x'
        with self.assertRaisesRegex(ValueError,'truncated string'):PE(synthetic_pe(r)).strings()
    def test_menu_zero_dynamic_and_handler_fields(self):
        raw=bytearray(56);struct.pack_into('<iiiiii',raw,4,9,2,10,20,100,30);raw[40]=1;struct.pack_into('<I',raw,44,0x1234)
        class Fake:
            def va(self,a,n):return bytes(raw)
        x=menu_items(Fake(),0,1,{2:'Select'})[0];self.assertEqual(x['caption'],'Select');self.assertEqual(x['rect'],[10,20,100,30]);self.assertEqual(x['handler'],'0x00001234');self.assertTrue(x['enabled'])
        with self.assertRaises(ValueError):menu_items(Fake(),0,65,{})
        self.assertEqual(resolve_caption({0:'Header'},0),'');self.assertEqual(resolve_caption({},1),'Dynamic runtime text')
    def test_string_continuation_is_bounded(self):
        self.assertEqual(resolve_caption({2:'Hello '+chr(255)+chr(255),2999:'world'},2),'Hello world')
        with self.assertRaises(ValueError):resolve_caption({2:chr(255)+chr(255),2999:chr(255)+chr(255)},2)
    def test_bob_stride_and_size_rules(self):
        self.assertEqual(bob_size('IMAGES/BIKES/RAT/KHIDASH.BOB',640*119),(640,119));self.assertEqual(bob_size('IMAGES/BIKES/SPORT/MLODASH.BOB',320*61),(320,61));self.assertEqual(bob_size('IMAGES/BIGMETER.BOB',190*64),(190,64))
        for path,size in [('unknown.BOB',100),('MATTE.BOB',639),('MATTE.BOB',0)]:
            with self.assertRaises(ValueError):bob_size(path,size)
    def test_standard_dialog_units_and_malformed_templates(self):
        raw=struct.pack('<IIHhhhh',0,0,0,0,0,100,60)+bytes(6)
        d=dialog_template(raw);self.assertEqual(d['rect'],[0,0,100,60]);self.assertEqual(d['controls'],[]);self.assertEqual(d['units'],'dialog units, not pixels')
        for raw in [b'',bytes(18),raw[:20]]:
            with self.assertRaises(ValueError):dialog_template(raw)
    def test_extended_dialog_header(self):
        raw=struct.pack('<HHIIIHhhhh',1,65535,0,0,0,0,0,0,100,60)+bytes(6)
        d=dialog_template(raw);self.assertEqual(d['format'],'extended');self.assertEqual(d['rect'],[0,0,100,60]);self.assertEqual(d['controls'],[])
    def test_original_media_cannot_be_exported_inside_repo(self):
        for p in [ROOT,ROOT/'assets',ROOT/'docs/ui/references']:
            with self.assertRaises(ValueError):external_output(p)
        self.assertEqual(external_output(Path('/tmp/roaddash-ui-external')),Path('/tmp/roaddash-ui-external'))

class HandoffTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.path=ROOT/'docs/ui';cls.data=json.loads((cls.path/'catalog.json').read_text());cls.plan=json.loads((cls.path/'plan.json').read_text())
    def test_source_coverage_and_no_failures(self):
        d=self.data;self.assertEqual(len(d['images']),154);self.assertEqual(sum(i['scope']=='ui' for i in d['images']),142);self.assertEqual(sum(i['scope']=='context-only' for i in d['images']),12);self.assertEqual(len(d['videos']),59);self.assertEqual(len(d['popups']),13);self.assertEqual(d['failures'],[]);self.assertEqual(len({i['id'] for i in d['images']}),154)
    def test_every_menu_slot_and_popup_has_editable_geometry(self):
        d=self.data;self.assertEqual([s['original_index'] for s in d['screens']],list(range(36)));self.assertEqual(sum(bool(s['items']) for s in d['screens']),29)
        for screen in d['screens']+d['popups']:
            r=ET.parse(self.path/'wireframes'/f'{screen["id"]}.svg').getroot();self.assertEqual(r.attrib['viewBox'],'0 0 640 480');self.assertFalse(r.findall('.//{http://www.w3.org/2000/svg}image'))
    def test_prototype_actions_resolve_and_defaults_match_screen_context(self):
        import re
        ids={s['id'] for s in self.plan['screens']};self.assertEqual(len(ids),28)
        for s in self.plan['screens']:
            for _,target in s['actions']:self.assertIn(target,ids)
            if 'preferred_source' in s:
                matches=[i for i in self.data['images'] if i['category'] in s['categories'] and re.search(s['source_pattern'],i['source'])]
                self.assertTrue(any(i['source']==s['preferred_source'] for i in matches),s['id'])
    def test_all_image_tasks_and_components_have_artist_identifiers(self):
        checklist=(self.path/'ASSET_CHECKLIST.md').read_text()
        for i in self.data['images']:
            if i['scope']=='ui':self.assertIn('**'+i['id']+'**',checklist)
        for c in self.plan['components']:self.assertIn('**'+c['id']+'**',checklist)
    def test_public_pack_contains_no_retail_pixels_or_font_binaries(self):
        self.assertFalse(self.data['reference_pixels_bundled'])
        # User-authorized generated RoadDash drafts are distinct from retail references.
        manifest=json.loads((self.path/'generated/manifest.json').read_text())
        candidate=json.loads((self.path/'generated/candidates/manifest.json').read_text())
        allowed={r['src'] for r in manifest['screens']}|{manifest['context']['src'],candidate['src']}
        variations=json.loads((self.path/'generated/candidates/variations.json').read_text())
        allowed|={r['src'] for r in variations}
        actual={p.relative_to(self.path).as_posix() for p in self.path.rglob('*') if p.suffix.lower() in ['.png','.jpg','.jpeg']}
        self.assertEqual(actual,allowed)
        self.assertFalse([p for p in self.path.rglob('*') if p.suffix.lower() in ['.avi','.ttf','.dll','.rri','.bob']])
        for screen in self.data['screens']:
            self.assertNotIn('title',screen)
            for item in screen['items']:self.assertIn(item['caption'],['',f'Text slot {item["caption_slot"]}'])
    def test_frozen_source_ids_and_master_families_are_complete(self):
        registry=json.loads((ROOT/'tools/ui/source_ids.json').read_text())
        self.assertEqual(len(registry['images']),146);self.assertEqual(len(registry['embedded']),8);self.assertEqual(len(registry['videos']),59)
        self.assertEqual(self.data['input_sha256'],registry['input_sha256'])
        for r in self.data['images']:
            key=r['source'] if r['id'].startswith('UI-IMG') else r['source']+'#'+'/'.join(map(str,r['resource_path']))
            self.assertEqual(registry['images' if r['id'].startswith('UI-IMG') else 'embedded'][key],r['id'])
            self.assertEqual(r['replacement'],'replacements/'+r['id']+'.png')
        self.assertEqual(len({i['master_family'] for i in self.data['images'] if i['scope']=='ui'}),66)
        self.assertEqual(len(list((self.path/'wireframes').glob('*.svg'))),63)
        self.assertEqual(len(self.data['windows_dialogs']),14)
        self.assertEqual(len({d['sha256'] for d in self.data['windows_dialogs']}),7)
    def test_plan_edits_refresh_without_original_game_files(self):
        import importlib.util,tempfile,copy
        spec=importlib.util.spec_from_file_location('ui_refresh',self.path/'refresh_plan.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
        plan=copy.deepcopy(self.plan);plan['screens'][0]['title']='Custom artist title'
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp);(p/'plan.json').write_text(json.dumps(plan));(p/'catalog.json').write_text(json.dumps(self.data));self.assertEqual(module.refresh(p),28)
            result=json.loads((p/'catalog-data.js').read_text().removeprefix('window.UI_KIT_DATA = ').removesuffix(';\n'));self.assertEqual(result['plan']['screens'][0]['title'],'Custom artist title')
        plan['screens'][0]['actions'][0][1]='not-a-screen'
        with self.assertRaises(ValueError):module.validate_plan(plan,self.data)
    def test_checklists_are_not_preticked_as_finished_art(self):
        self.assertNotIn('- [x]',(self.path/'ASSET_CHECKLIST.md').read_text());self.assertEqual(len(self.plan['components']),15)

if __name__=='__main__':unittest.main()
