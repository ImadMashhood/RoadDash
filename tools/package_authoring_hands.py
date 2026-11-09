#!/usr/bin/env python3
"""Package only original current hand/pose authoring; leave private references alone."""
import argparse
import hashlib
import shutil
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=Path.home()/'Downloads/RoadDash_Hand_Authoring_Part02')
    args = parser.parse_args()
    out = args.output.resolve()
    if out == ROOT or ROOT in out.parents:
        raise ValueError('Large editable outputs belong outside the repository')
    out.mkdir(parents=True, exist_ok=True)
    for source in ['art/animation/context', 'docs/testing/part02-authoring', 'docs/testing/hand-contacts/gallery']:
        shutil.copytree(ROOT/source, out/source, dirs_exist_ok=True)
    for source in ['LICENSE', 'content/hand-contacts.json', 'tools/load_context_pose.py', 'tools/authoring_hands.py']:
        target = out/source
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT/source, target)
    for name in ['RoadDash_Context_Poses.blend', 'RoadDash_Hands_Roundtrip.glb']:
        shutil.copyfile(Path.home()/'Downloads/RoadDash_Context_Review_v2'/name, out/name)
    (out/'README.txt').write_text('''RoadDash Part02 original hand authoring development pass — draft, not approved.
Open RoadDash_Context_Poses.blend. Run tools/load_context_pose.py in Blender's
Text Editor (set POSE_ID), or use Blender -b FILE --python SCRIPT -- 2222.
The loader restores body, bike, props and all four grip/fist values.

Current full pose gallery: art/animation/context/gallery/index.html
18 source-hand views: docs/testing/part02-authoring/index.html
Technical report/limits: docs/testing/part02-authoring/README.md and roundtrip.json
GLB is an actual exported/reloaded original visual asset, not a physics rig.
No retail media or third-party character/animation source is packaged here.
Original project assets/code: MIT, see LICENSE and art/animation/context/PROVENANCE.json.
All Part02 approval/production gates remain open. No new runtime clips or physics.
''')
    files = sorted(p for p in out.rglob('*') if p.is_file() and p.name != 'PACKAGE_SHA256SUMS')
    (out/'PACKAGE_SHA256SUMS').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+p.relative_to(out).as_posix()+'\n' for p in files))
    print(out)
if __name__ == '__main__':
    main()
