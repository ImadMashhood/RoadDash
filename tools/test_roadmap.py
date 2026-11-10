"""Protect preserved milestone IDs/statuses and relative evidence links."""
import importlib.util
import json
import re
from pathlib import Path
import unittest
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('roadmap_sync',ROOT/'tools/sync_progress.py')
sync=importlib.util.module_from_spec(spec);spec.loader.exec_module(sync)
class RoadmapTests(unittest.TestCase):
    def test_historical_tasks_and_evidence_are_not_lost(self):
        original=json.loads((ROOT/'docs/roadmap/MIGRATION_STATUS.json').read_text(encoding='utf-8'))['preserved_ids']
        current={}
        for source in sync.SOURCES:
            for line in source.read_text(encoding='utf-8').splitlines():
                match=sync.TASK.match(line)
                if match:
                    self.assertNotIn(match[2],current)
                    current[match[2]]=match[1]
        self.assertTrue(set(original).issubset(current))
        for id,status in original.items():
            if status=='x':self.assertEqual(current[id],'x',id)
        self.assertGreaterEqual(sum(id.startswith('V2-') for id in current),54)
        introduced=json.loads((ROOT/'docs/roadmap/MIGRATION_STATUS.json').read_text(encoding='utf-8'))['new_v2_ids']
        self.assertTrue(set(introduced).issubset(current))
    def test_archive_and_active_evidence_resolve_to_the_same_source(self):
        a=sync.render('- [ ] **X-01** proof [test](src/game.rs)',ROOT)
        b=sync.render('- [ ] **X-01** proof [test](../../src/game.rs)',ROOT/'docs/roadmap')
        self.assertEqual(a,b)
        self.assertIn('(../src/game.rs)',a)
    def test_index_has_both_current_and_historical_requirements(self):
        index=sync.checklist()
        self.assertIn('**V2-01-001**',index)
        self.assertIn('**P01-001**',index)
        self.assertIn('**P09-007**',index)
    def test_all_tasks_have_current_placement_and_completed_tasks_come_first(self):
        text=(ROOT/'README.md').read_text(encoding='utf-8')
        placement=json.loads((ROOT/'docs/roadmap/TASK_PLACEMENT.json').read_text(encoding='utf-8'))['parts']
        actual={};part=None;seen_open=False
        for line in text.splitlines():
            section=re.match(r'### Part (\d+) ',line)
            if section:part=section[1]
            match=sync.TASK.match(line)
            if not match:continue
            if match[1]==' ':
                seen_open=True
                self.assertNotEqual(part,'00')
            else:
                self.assertFalse(seen_open,match[2])
                self.assertEqual(part,'00')
            actual[match[2]]=part
        expected={id:part for part,value in placement.items() for id in value['task_ids']}
        self.assertEqual(actual,expected)
        before=json.loads((ROOT/'docs/roadmap/README_REORGANIZATION_STATUS.json').read_text(encoding='utf-8'))['statuses']
        self.assertTrue(set(before).issubset(actual))
        self.assertEqual(sync.SOURCES,[ROOT/'README.md'])

    def test_summary_counts_match_checkboxes(self):
        text=(ROOT/'README.md').read_text(encoding='utf-8')
        tasks=[sync.TASK.match(line) for line in text.splitlines() if sync.TASK.match(line)]
        done=sum(t[1]=='x' for t in tasks)
        self.assertIn(f'**{done} verified tasks complete; {len(tasks)-done} tasks remain open.**',text)
        placement=json.loads((ROOT/'docs/roadmap/TASK_PLACEMENT.json').read_text(encoding='utf-8'))['parts']
        for part,value in placement.items():
            status=f"{len(value['task_ids'])} "+('verified complete' if part=='00' else 'open')
            self.assertIn(f"| [{part}](#part-{part}) | {value['title']} | {status} |",text)

if __name__=='__main__':unittest.main()
