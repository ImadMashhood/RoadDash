#!/usr/bin/env python3
"""Selected generated art is real media, traceable, complete and kept out of native assets."""
import hashlib
import json
from pathlib import Path
import re
import tempfile
import unittest
from PIL import Image
import package_ui_screen_art as package

UI=package.UI


class GeneratedArtTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data=json.loads((UI/'generated/manifest.json').read_text())
        cls.plan=json.loads((UI/'plan.json').read_text())

    def test_every_draft_page_has_one_unique_selected_screen_image(self):
        actual=[x['id'] for x in self.data['screens']]
        self.assertEqual(actual,[x['id'] for x in self.plan['screens']])
        self.assertEqual(len(actual),28)
        self.assertEqual(len(set(x['sha256'] for x in self.data['screens'])),28)

    def test_every_selected_png_decodes_and_matches_recorded_hash_dimensions(self):
        for row in self.data['screens']:
            path=UI/row['src'];self.assertEqual(package.digest(path),row['sha256'])
            self.assertEqual(row['bytes'],path.stat().st_size)
            with Image.open(path) as im:
                im.load();self.assertEqual(im.format,'PNG')
                self.assertEqual(im.size,(row['width'],row['height']))
                self.assertLess(abs(im.width/im.height-16/9),0.01)
            self.assertRegex(row['tool_output'],r'^exec-[a-f0-9-]+\.png$')
            self.assertTrue(row['src'].startswith('generated/screens/'))
            self.assertNotIn('..',Path(row['src']).parts)

    def test_transparent_frames_have_real_alpha_and_open_viewing_areas(self):
        for row in self.data['screens']:
            with Image.open(UI/row['src']) as im:
                if row['id'] in package.OVERLAYS:
                    self.assertEqual(im.mode,'RGBA')
                    alpha=im.getchannel('A');self.assertEqual(alpha.getextrema(),(0,255))
                    self.assertGreater(alpha.histogram()[0]/(im.width*im.height),.45)
                    if row['id']=='hud': self.assertEqual(alpha.getpixel((im.width//2,im.height//2)),0)
                else:self.assertEqual(row['kind'],'background')

    def test_prompts_and_selected_job_origins_are_preserved(self):
        jobs={j['id']:j for j in json.loads((UI/'generated/generation-jobs.json').read_text())}
        for row in self.data['screens']:
            p=UI/row['prompt'];self.assertEqual(package.digest(p),row['prompt_sha256'])
            self.assertEqual(p.read_text().rstrip(),jobs[row['id']]['prompt'])
            self.assertEqual(Path(jobs[row['id']]['path']).name,row['tool_output'])

    def test_art_review_is_open_and_context_is_a_project_native_screenshot(self):
        review=(UI/'SCREEN_ART_REVIEW.md').read_text()
        self.assertEqual(len(re.findall(r'- \[[ x]\] \*\*UI-SCREEN-',review)),28)
        self.assertTrue(all(row['status']=='draft' for row in self.data['screens']))
        context=self.data['context'];self.assertEqual(package.digest(UI/context['src']),context['sha256'])
        self.assertEqual(package.digest(package.ROOT/context['source']),context['source_sha256'])
        self.assertEqual(context['sha256'],context['source_sha256'])

    def test_manifest_javascript_and_repackaging_are_deterministic(self):
        js=(UI/'generated/manifest.js').read_text()
        parsed=json.loads(js.removeprefix('window.UI_SCREEN_ART = ').removesuffix(';\n'))
        self.assertEqual(parsed,self.data)
        before=(UI/'generated/manifest.json').read_bytes()
        package.build()
        self.assertEqual(before,(UI/'generated/manifest.json').read_bytes())

    def test_repackaging_keeps_human_review_states_and_notes(self):
        with tempfile.TemporaryDirectory() as folder:
            path=Path(folder)/'review.md'
            path.write_text('- [x] **UI-SCREEN-MAIN** approved by artist\nArtist note: keep this.\n')
            package.update_review(path,['- [ ] **UI-SCREEN-MAIN** default',
                                        '- [ ] **UI-SCREEN-HUD** new'])
            text=path.read_text()
            self.assertIn('- [x] **UI-SCREEN-MAIN** approved by artist',text)
            self.assertIn('Artist note: keep this.',text)
            self.assertIn('- [ ] **UI-SCREEN-HUD** new',text)
            self.assertNotIn('default',text)

    def test_external_copy_rejects_repo_or_arbitrary_destinations(self):
        with self.assertRaises(ValueError):package.copy_to(UI)
        with tempfile.TemporaryDirectory() as folder:
            with self.assertRaises(ValueError):package.copy_to(folder)


if __name__=='__main__':unittest.main()
