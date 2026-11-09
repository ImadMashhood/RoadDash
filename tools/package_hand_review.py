#!/usr/bin/env python3
"""Package native glove screenshots; no image manipulation or proprietary references."""
import argparse
import hashlib
import html
import json
import shutil
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'docs/testing/hand-contacts/gallery'
MODES = [(0,'NORMAL',2020),(1,'FAST-TUCK',2030),(2,'LEAN-LEFT',2900),(3,'LEAN-RIGHT',2901),
         (4,'PUNCH-LEFT',2222),(5,'PUNCH-RIGHT',2232),(6,'KICK-LEFT',None),(7,'KICK-RIGHT',None),
         (8,'STANDING',2364),(9,'WALKING',None),(10,'RUNNING',None)]
SOURCES = ['content/hand-contacts.json','src/models/hands.rs','src/animation.rs','src/animation/preview.rs','assets/models/motion.json','src/models.rs','assets/models/riding.rdmesh','assets/models/rider-bindings.json','art/animation/context/manifest.json']
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true');args=parser.parse_args()
    if args.check:
        data=json.loads((OUT/'PROVENANCE.json').read_text())
        assert data['source_sha256']=={p:digest(ROOT/p) for p in SOURCES},'stale hand sources'
        assert len(data['captures'])==33
        for r in data['captures']:assert digest(OUT/r['file'])==r['sha256']
        print('33 native hand captures and source hashes verified');return
    OUT.mkdir(parents=True,exist_ok=True);captures=[];cards=[]
    poses={p['id']:p for p in json.loads((ROOT/'art/animation/context/manifest.json').read_text())['poses']}
    for mode,name,key in MODES:
        for view,label in enumerate(['Rear','Front','Side']):
            file=f'{mode:02}-{name}-{view}.png';source=ROOT/'screenshots/hand-contacts'/file
            if not source.is_file():raise SystemExit(f'Missing native capture: {file}')
            shutil.copyfile(source,OUT/file)
            capture=dict(id=f'H{mode:02}-{view}',mode=name,view=label,file=file,sha256=digest(source),
                         exact_held_key=key if mode in [1,4,5,8] else None,
                         related_pose=key,source_slots=poses.get(key,{}).get('source_slots',[]),kind='held authored key' if mode in [1,4,5,8] else 'runtime snapshot')
            captures.append(capture)
            title=f"{capture['id']} · {name} · {label}"
            cards.append(f'<figure><a href="{file}"><img loading="lazy" src="{file}" alt="{html.escape(title)}"></a><figcaption>{html.escape(title)}<br>{capture["kind"]}; '+(f'pose {key}' if key else 'procedural/clip motion')+'</figcaption></figure>')
    (OUT/'PROVENANCE.json').write_text(json.dumps(dict(source_sha256={p:digest(ROOT/p) for p in SOURCES},captures=captures,
        approval='draft; technical review only',physics_changed=False,contains_retail_media=False),indent=2)+'\n')
    (OUT/'index.html').write_text('<!doctype html><meta charset="utf-8"><title>RoadDash glove contact review</title><style>body{background:#171b20;color:#eee;font:16px sans-serif;margin:24px}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(320px,1fr));gap:16px}figure{margin:0;background:#252c33;padding:8px}img{width:100%}figcaption{padding:8px}</style><h1>Glove / grip / fist review</h1><p>33 original native captures. Draft art; exact held punch/tuck/idle keys are identified separately from runtime snapshots. Physical ragdoll unchanged. Click to inspect full resolution.</p><div class="grid">'+''.join(cards)+'</div>\n')
    print('Packaged 33 native glove captures')
if __name__=='__main__':main()
