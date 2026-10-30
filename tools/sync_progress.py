#!/usr/bin/env python3
"""Keep the complete task index in PROGRESS aligned with README; preserve notes."""
import argparse
from pathlib import Path
import re
ROOT=Path(__file__).resolve().parents[1]
MARK='<!-- GENERATED CHECKLIST INDEX -->'
def render(readme):
    result=[]
    for line in readme.splitlines():
        if line.startswith('### '): result.append(line)
        elif re.match(r'- \[[ x]\] \*\*[A-Z0-9-]+\*\* ',line):
            # README file references are relative to repository root.
            line=re.sub(r'\]\((?!https?://|#)([^)]+)\)',lambda m:'](../'+m.group(1)+')',line)
            result.append(line)
    return '\n\n'.join(result)+'\n'
def main():
    p=argparse.ArgumentParser();p.add_argument('--check',action='store_true');args=p.parse_args()
    readme=(ROOT/'README.md').read_text(encoding="utf-8"); progress=ROOT/'docs/PROGRESS.md'
    header=progress.read_text(encoding="utf-8").split(MARK)[0].rstrip() if progress.exists() else '# Development progress'
    output=header+'\n\n'+MARK+'\n\n'+render(readme)
    if args.check:
        if not progress.exists() or progress.read_text(encoding="utf-8")!=output: raise SystemExit('Progress index is stale; run python3 tools/sync_progress.py')
        print('README / progress task IDs and checkbox states match')
    else: progress.write_text(output,encoding="utf-8")
if __name__=='__main__': main()
