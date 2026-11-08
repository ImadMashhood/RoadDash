#!/usr/bin/env python3
"""Check 2D parcel outlines, deterministic exports and the layout-only branch scope."""
import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FOLDER = ROOT / 'docs/planning/dense-city-layout'
BASELINE = '149e355'


def orientation(a, b, c):
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])


def proper_crossing(a, b, c, d):
    return (orientation(a, b, c) * orientation(a, b, d) < -1e-9
            and orientation(c, d, a) * orientation(c, d, b) < -1e-9)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report', type=Path, help='Same read-only inspection report as generation')
    args = parser.parse_args()
    plan = json.loads((FOLDER / 'LAYOUT.json').read_text())
    crossings = 0
    for parcel in plan['parcels']:
        for key in ['lot', 'front_building', 'private_court', 'rear_wing']:
            poly = parcel[key]
            edges = list(zip(poly, poly[1:] + poly[:1]))
            for i, (a, b) in enumerate(edges):
                for j in range(i + 2, len(edges)):
                    if i == 0 and j == len(edges) - 1:
                        continue
                    crossings += proper_crossing(a, b, *edges[j])
    assert crossings == 0, f'{crossings} self-crossing outline edges'
    outputs = ['LAYOUT.json', 'masterplan.png', 'masterplan.svg',
               'block-plan.png', 'block-plan.svg', 'street-widths.png', 'index.html']
    with tempfile.TemporaryDirectory() as temp:
        subprocess.run([sys.executable, str(ROOT / 'tools/plan_dense_city.py'),
                        str(args.report.resolve()), '--output', temp],
                       cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
        identical = {name: hashlib.sha256((FOLDER / name).read_bytes()).digest()
                     == hashlib.sha256((Path(temp) / name).read_bytes()).digest()
                     for name in outputs}
        assert all(identical.values()), identical
    def git(*args):
        return subprocess.check_output(['git', *args], cwd=ROOT, text=True).splitlines()
    changed = git('diff', '--name-only', BASELINE) + git('ls-files', '--others', '--exclude-standard')
    runtime = [p for p in changed if p.startswith(('src/', 'content/', 'assets/', 'art/', 'crates/', 'Cargo.'))]
    assert not runtime, f'Layout-only scope changed runtime files: {runtime}'
    receipt = dict(
        proposed_parcels=len(plan['parcels']),
        primary_frontage_parcels=plan['metrics']['primary_frontage_parcels'],
        geometry_checks=plan['validation'], proper_self_intersections=crossings,
        deterministic_outputs=identical,
        runtime_differences_from_149e355=runtime,
        limits='2D proposal, not approval or a playable bake; cross-parcel/corner/access/terrain/vehicle checks remain open',
    )
    (FOLDER / 'VALIDATION.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt, indent=2))


if __name__ == '__main__':
    main()
