#!/usr/bin/env python3
"""Validate/publish the selected built-in imagegen drafts; never edits image pixels."""
import argparse
import hashlib
import json
import re
from pathlib import Path
import shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[1]
UI=ROOT/'docs/ui'
OVERLAYS={'hud','pause','confirm'}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build():
    plan=json.loads((UI/'plan.json').read_text())
    jobs=json.loads((UI/'generated/generation-jobs.json').read_text())
    mapping={j['id']:j for j in jobs}
    expected={s['id'] for s in plan['screens']}
    if len(mapping)!=len(jobs) or set(mapping)!=expected:
        raise ValueError('Need exactly one selected image for each of the28 proposed screens')
    records=[]
    for screen in plan['screens']:
        identity=screen['id']; image=UI/'generated/screens'/f'{identity}.png'
        prompt=UI/'generated/prompts'/f'{identity}.txt'
        if not image.is_file() or not prompt.is_file():
            raise ValueError('Missing selected image or prompt: '+identity)
        with Image.open(image) as im:
            im.load(); width,height=im.size
            if abs(width/height-16/9)>0.01:
                raise ValueError('Screen must preserve landscape16:9 composition: '+identity)
            alpha=im.getchannel('A') if im.mode=='RGBA' else None
            transparent=alpha is not None and alpha.getextrema()[0]==0
            transparent_fraction=(alpha.histogram()[0]/(width*height)) if alpha else 0.
            if identity in OVERLAYS and (not transparent or transparent_fraction<0.45):
                raise ValueError('Overlay needs real transparent alpha/open viewing area: '+identity)
        if prompt.read_text().rstrip()!=mapping[identity]['prompt']:
            raise ValueError('Saved generation prompt differs from selected job: '+identity)
        records.append(dict(id=identity,asset_id='UI-SCREEN-'+identity.upper(),title=screen['title'],
            src=f'generated/screens/{identity}.png',width=width,height=height,
            sha256=digest(image),bytes=image.stat().st_size,status='draft',
            transparent=transparent,transparent_fraction=round(transparent_fraction,6),
            prompt=f'generated/prompts/{identity}.txt',prompt_sha256=digest(prompt),
            tool_output=Path(mapping[identity]['path']).name,
            kind='overlay' if identity in OVERLAYS else 'background'))
    context_dir=UI/'generated/context';context_dir.mkdir(exist_ok=True)
    source=ROOT/'docs/testing/performance-ui-prompts/final/metro/metro-06.png'
    context=context_dir/'race.png'
    if not context.exists(): shutil.copy2(source,context)
    if digest(context)!=digest(source): raise ValueError('Preview context must match the project native screenshot')
    payload=dict(schema_version=1,method='ChatGPT built-in image_gen; model version not exposed',
        stage='generated draft / authoring preview only; user art approval and Bevy integration pending',
        context=dict(src='generated/context/race.png',sha256=digest(context),
                     source=source.relative_to(ROOT).as_posix(),source_sha256=digest(source)),
        screens=records)
    (UI/'generated/manifest.json').write_text(json.dumps(payload,indent=2)+'\n')
    (UI/'generated/manifest.js').write_text('window.UI_SCREEN_ART = '+json.dumps(payload,separators=(',',':'))+';\n')
    checklist=['# Generated screen art review','','All28 selected images exist and load in the preview. Each remains a draft pending user review; presence is not approval. These are screen-level compositions, not completed replacements for all142 source-resource IDs.','']
    for r in records:
        checklist += [f'- [ ] **{r["asset_id"]}** {r["title"]} — [{r["width"]}×{r["height"]} {r["kind"]}]({r["src"]}); check composition, model identity, anatomy, quiet text areas, alpha/edges and live-widget fit.']
    review=UI/'SCREEN_ART_REVIEW.md'
    update_review(review,checklist)
    return payload


def update_review(path, checklist):
    """Preserve human review statuses/notes; append new asset IDs without resetting rows."""
    if not path.exists():
        path.write_text('\n'.join(checklist)+'\n')
        return
    current=path.read_text()
    known=set(re.findall(r'\*\*(UI-SCREEN-[A-Z0-9_-]+)\*\*',current))
    additions=[line for line in checklist if line.startswith('- [ ]')
               and re.search(r'\*\*(UI-SCREEN-[A-Z0-9_-]+)\*\*',line)[1] not in known]
    if additions:
        path.write_text(current.rstrip()+'\n\n## Additional screen assets\n\n'+'\n'.join(additions)+'\n')


def copy_to(kit):
    kit=Path(kit).resolve()
    if kit==UI or ROOT in kit.parents: raise ValueError('External kit destination must be outside the repository')
    if not (kit/'index.html').is_file(): raise ValueError('Expected an existing UI kit, not an arbitrary destination')
    # External artist edits are not expendable. Validate before any copying.
    for source in (UI/'generated').rglob('*.png'):
        target=kit/source.relative_to(UI)
        if target.exists() and digest(target)!=digest(source):
            raise ValueError('External art differs; preserve it and explicitly version a replacement: '+str(target))
    for name in ['app.js','style.css','index.html']:
        shutil.copy2(UI/name,kit/name)
    if not (kit/'SCREEN_ART_REVIEW.md').exists():
        shutil.copy2(UI/'SCREEN_ART_REVIEW.md',kit/'SCREEN_ART_REVIEW.md')
    shutil.copytree(UI/'generated',kit/'generated',dirs_exist_ok=True)


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kit',type=Path)
    args=parser.parse_args();data=build()
    if args.kit: copy_to(args.kit)
    print(f'Validated{len(data["screens"])} generated drafts, including3 real-alpha overlays; no pixels modified.')
