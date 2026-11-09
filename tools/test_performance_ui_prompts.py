#!/usr/bin/env python3
"""Regression tests for performance interpretation and complete artist prompt coverage."""
import json
import re
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch
import build_ui_prompts as prompts
from summarize_performance import summarize


class PerformanceSummaryTests(unittest.TestCase):
    def summary(self, rows, **kwargs):
        with tempfile.TemporaryDirectory() as folder:
            p=Path(folder)/'log.jsonl'
            p.write_text('\n'.join(json.dumps(r) if not isinstance(r,str) else r for r in rows))
            return summarize(p, **kwargs)

    def row(self, **kwargs):
        return dict(schema_version=1, frames=10, fixed_steps=30, racing=True,
                    wall_p95_ms=80, physics_ms_per_frame=30, **kwargs)

    def test_normalizes_catchup_by_actual_fixed_steps(self):
        result=self.summary([self.row()])
        self.assertEqual(result['fixed_steps_per_render_frame'],3)
        self.assertEqual(result['weighted_fixed_step_ms']['physics_ms_per_fixed_step'],10)
        self.assertNotIn('physics_sync_ms_per_frame',result['window_distributions_ms'])

    def test_startup_and_paused_windows_can_be_excluded(self):
        rows=[self.row(),dict(self.row(),racing=False,wall_p95_ms=1000), self.row()]
        result=self.summary(rows,skip_samples=1,racing_only=True)
        self.assertEqual(result['samples'],1)
        self.assertEqual(result['total_valid_samples'],3)
        self.assertEqual(result['worst_windows'][0]['wall_p95_ms'],80)

    def test_bad_lines_schemas_and_nonfinite_values_are_ignored(self):
        bad=dict(self.row(),wall_p95_ms=float('nan'))
        result=self.summary(['{broken','null',[],{'schema_version':99},bad,self.row()])
        self.assertEqual(result['invalid_or_partial_lines'],5)
        self.assertEqual(result['samples'],1)

    def test_empty_selections_and_negative_skip_fail_clearly(self):
        with self.assertRaises(ValueError): self.summary([self.row()],skip_samples=2)
        with self.assertRaises(ValueError): self.summary([self.row()],skip_samples=-1)

    def test_zero_tick_frames_never_create_infinite_step_cost(self):
        result=self.summary([dict(self.row(),fixed_steps=0)])
        self.assertEqual(result['fixed_steps_per_render_frame'],0)
        self.assertEqual(result['weighted_fixed_step_ms'],{})


class PromptTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data=json.loads((prompts.UI/'art_prompts.json').read_text())
        cls.catalog=json.loads((prompts.UI/'catalog.json').read_text())

    def test_all_28_screens_and_only_142_ui_resources_are_covered(self):
        plan=json.loads((prompts.UI/'plan.json').read_text())
        self.assertEqual({s['id'] for s in self.data['screens']},{s['id'] for s in plan['screens']})
        expected={i['id'] for i in self.catalog['images'] if i['scope']=='ui'}
        self.assertEqual({i['id'] for i in self.data['images']},expected)
        self.assertEqual(len(self.data['images']),142)
        self.assertEqual(len(self.data['families']),66)

    def test_each_resource_keeps_source_aspect_variant_and_export_path(self):
        lookup={i['id']:i for i in self.catalog['images']}
        for item in self.data['images']:
            source=lookup[item['id']]
            for text in [source['source'],source['variant'],source['replacement'],
                         f'{source["width"]}:{source["height"]}']:
                self.assertIn(text,item['prompt'])
            self.assertEqual(item['replacement'],source['replacement'])
            size=re.search(r'Suggested master (\d+)×(\d+)',item['prompt'])
            self.assertIsNotNone(size)
            w,h=map(int,size.groups())
            self.assertEqual(w*source['height'],h*source['width'])

    def test_transparent_overlays_and_live_text_remain_separate(self):
        screens={s['id']:s for s in self.data['screens']}
        for key in ['hud','pause','confirm']:
            self.assertIn('true transparency',screens[key]['output'])
        for s in self.data['screens']:
            self.assertIn('No baked-in lettering',s['prompt'])
            self.assertIn('5% safe margin',s['prompt'])
        for item in self.data['families']:
            if item['id'].startswith('rider-'):
                self.assertIn('do not guess their emotions',item['prompt'])

    def test_prompt_outputs_rebuild_deterministically_without_original_files(self):
        with tempfile.TemporaryDirectory() as folder:
            target=Path(folder)
            for name in ['plan.json','catalog.json']: shutil.copy(prompts.UI/name,target/name)
            with patch.object(prompts,'UI',target): prompts.build()
            for name in ['art_prompts.json','SCREEN_GENERATION_PROMPTS.md','IMAGE_GENERATION_PROMPTS.md']:
                self.assertEqual((target/name).read_bytes(),(prompts.UI/name).read_bytes())


if __name__=='__main__': unittest.main()
