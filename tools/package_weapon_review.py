#!/usr/bin/env python3
"""Package intact native club captures and bind them to the actual source/binary."""
import argparse
import hashlib
import html
import json
import shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'docs/testing/part07-weapons'
SOURCES=['src/weapons.rs','src/combat.rs','src/controls.rs','src/game.rs','src/animation.rs','src/animation/preview.rs','src/models.rs','src/models/hands.rs','src/verification.rs','src/destruction/mod.rs','src/runtime.rs','src/settings.rs','content/weapons.json','content/hand-contacts.json','assets/models/motion.json','assets/models/riding.rdmesh','Cargo.toml','Cargo.lock','crates/rr_core/src/weapons.rs','crates/rr_core/src/motion.rs','crates/rr_content/src/weapons.rs','tools/append_club_motion.py','tools/package_weapon_review.py']+[f'art/animation/context/poses/{i}.json' for i in [2160,2161,2162,2170,2171,2172]]
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true');args=parser.parse_args()
    if args.check:
        data=json.loads((OUT/'PROVENANCE.json').read_text())
        assert data['source_sha256']=={p:digest(ROOT/p) for p in SOURCES},'stale weapon sources'
        for item in data['captures']:assert digest(OUT/item['file'])==item['sha256'],item['file']
        assert len(data['captures'])==17
        print('17 intact native club/game captures and source hashes verified');return
    (OUT/'gallery').mkdir(exist_ok=True);captures=[];cards=[]
    for kind,folder in [('body','animation-part03'),('hand','hand-contacts')]:
        for mode,name,key in [(11,'CLUB-LEFT',2162),(12,'CLUB-RIGHT',2172)]:
            for view,label in enumerate(['Rear','Front','Side']):
                source=ROOT/f'screenshots/{folder}/{mode:02}-{name}-{view}.png'
                file=f'gallery/{kind}-{name}-{view}.png';shutil.copyfile(source,OUT/file)
                title=f'{kind} / {name} / {label}'
                captures.append(dict(file=file,sha256=digest(source),kind=kind,side=name,view=label,held_contact_key=key if kind=='hand' else None,phase='held authored contact' if kind=='hand' else 'live clip snapshot; not an exact contact certificate'))
                cards.append(f'<figure><a href="{Path(file).name}"><img loading="lazy" src="{Path(file).name}"></a><figcaption>{html.escape(title)}</figcaption></figure>')
    for path in sorted((OUT/'native').glob('*.png')):
        captures.append(dict(file=path.relative_to(OUT).as_posix(),sha256=digest(path),kind='native Metro keyboard/recovery fixture'))
    data=dict(source_sha256={p:digest(ROOT/p) for p in SOURCES},binary_sha256=digest(ROOT/'target/release/road-dash'),captures=captures,physics_changed=False,contains_retail_media=False,approval='functional draft; grip anatomy / clearance / modeled fidelity / human feel unapproved',laboratory_note='39 captures per lab run / 13 modes; inherited console summary still says eleven motions')
    (OUT/'PROVENANCE.json').write_text(json.dumps(data,indent=2)+'\n')
    (OUT/'gallery/index.html').write_text('<!doctype html><meta charset="utf-8"><title>RoadDash first club native review</title><style>body{font:16px sans-serif;background:#171b20;color:#eee;margin:24px}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(340px,1fr));gap:16px}figure{margin:0}img{width:100%}</style><h1>Original first club — native draft</h1><p>Intact current-release captures. Full-body views play the live clip; hand views hold keys2162/2172. Shafts attach and grip weights are active; angular/protruding fingers and one occluded right-side view keep anatomy, wrap and clearance unapproved. No retail images or physical rig changes.</p><p><a href="../PROVENANCE.json">Source / image / binary hashes</a> · <a href="../README.md">Implementation and open gates</a></p><div class="grid">'+''.join(cards)+'</div>\n')
    print('Packaged12 club body/hand views plus5 native keyboard/recovery captures')
if __name__=='__main__':main()
