#!/usr/bin/env python3
"""Synchronize the canonical completed-first README checklist without losing IDs."""
import argparse
from pathlib import Path
import re
ROOT=Path(__file__).resolve().parents[1]
MARK='<!-- GENERATED CHECKLIST INDEX -->'
SOURCES=[ROOT/'README.md']
TASK=re.compile(r'- \[([ x])\] \*\*([A-Z0-9-]+)\*\* ')
def render(text,source_dir=None):
    source_dir=source_dir or ROOT
    result=[]
    for line in text.splitlines():
        if line.startswith('### '):result.append(line)
        elif TASK.match(line):
            def link(match):
                target=match.group(1)
                if target.startswith(('http://','https://','#')):return match.group(0)
                import os
                return ']('+Path(os.path.relpath(source_dir/target,ROOT/'docs')).as_posix()+')'
            result.append(re.sub(r'\]\(([^)]+)\)',link,line))
    return '\n\n'.join(result)+'\n'
def checklist():
    blocks=[];seen=set()
    for path in SOURCES:
        content=path.read_text(encoding='utf-8')
        for line in content.splitlines():
            match=TASK.match(line)
            if match:
                if match[2] in seen:raise ValueError('duplicate roadmap ID: '+match[2])
                seen.add(match[2])
        blocks.append(render(content,path.parent))
    return '\n\n'.join(blocks)
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    progress=ROOT/'docs/PROGRESS.md'
    header=progress.read_text(encoding='utf-8').split(MARK)[0].rstrip() if progress.exists() else '# Development progress'
    output=header+'\n\n'+MARK+'\n\n'+checklist()
    if args.check:
        if not progress.exists() or progress.read_text(encoding='utf-8')!=output:raise SystemExit('Progress index is stale; run python3 tools/sync_progress.py')
        print('Canonical README / progress IDs and states match')
    else:progress.write_text(output,encoding='utf-8')
if __name__=='__main__':main()
