#!/usr/bin/env python3
"""Package sanitized metadata/original wireframes and the user's local reference kit."""
import argparse
from collections import Counter
import csv
import html
import json
import re
from pathlib import Path
import shutil
from catalog_ui_references import PE, screens, menu_items, POPUPS, external_output, dialog_template
ROOT=Path(__file__).resolve().parents[1]

def svg(screen):
    title=html.escape(screen['name'])
    body=[f'<svg xmlns="http://www.w3.org/2000/svg" width="1280" height="960" viewBox="0 0 640 480"><rect width="640" height="480" fill="#20252b"/><text x="16" y="24" fill="#eaeaea" font-family="sans-serif" font-size="16">{screen["id"]} · {title}</text><text x="16" y="44" fill="#a2b0ba" font-family="sans-serif" font-size="10">Original source geometry; independent labels; no source artwork</text>']
    for item in screen['items']:
        x,y,w,h=item['rect']
        if w<=0 or h<=0:continue
        label=html.escape(f'{item["index"]+1}: text {item["caption_slot"]}')
        body.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="#35424e" fill-opacity=".65" stroke="#819bad" stroke-width="1"/><text x="{x+4}" y="{y+min(h-2,15)}" font-family="sans-serif" font-size="10" fill="white">{label}</text>')
    if not screen['items']:body.append('<text x="160" y="240" fill="#bac5cd" font-family="sans-serif" font-size="16">No stored menu controls in this slot</text>')
    return '\n'.join(body+['</svg>'])+'\n'

def package(kit, install):
    kit=external_output(kit)
    data=json.loads((kit/'catalog.json').read_text())
    # Refresh only menu metadata after decoder/label fixes; never copy text into repo.
    exe=PE((install/'ROADRASH.EXE').read_bytes());strings=PE((install/'TEXT/ENU.DLL').read_bytes()).strings()
    data['screens']=screens(exe,strings);data['popups']=[dict(id=f'RR-POP-{i+1:02}',name=name,items=menu_items(exe,a,n,strings)) for i,(name,a,n) in enumerate(POPUPS)]
    data['windows_dialogs']=[]
    for file in ['ROADRASH.EXE','RASHME.EXE']:
        for path,raw in PE((install/file).read_bytes()).resources():
            if path[0]!=5:continue
            import hashlib
            d=dialog_template(raw);d.update(id=f'RR-WIN-{Path(file).stem}-{path[1]}',source=file,resource_path=path,sha256=hashlib.sha256(raw).hexdigest(),name=('Windows Sound Mix300' if path[1]==300 else f'Windows dialog {path[1]}')+f' ({Path(file).stem})')
            data['windows_dialogs'].append(d)
    data['counts']['windows_dialog_resources']=len(data['windows_dialogs'])
    data['counts']['unique_windows_dialogs']=len({d['sha256'] for d in data['windows_dialogs']})
    data['counts']['populated_screen_entries']=sum(bool(s['items']) for s in data['screens'])
    data['counts']['unique_ui_source_hashes']=len({i['sha256'] for i in data['images'] if i['scope']=='ui'})
    data['counts']['stored_menu_items']=sum(len(s['items']) for s in data['screens'])
    plan=json.loads((ROOT/'docs/ui/plan.json').read_text())
    # Example original picture compositions. No retail text/font renderer is reused.
    from PIL import Image
    (kit/'menu-previews').mkdir(exist_ok=True)
    backgrounds={7:'CREDITS',8:'CHARS/AXLE-F',9:'MAIN',10:'POSTCARD/CITY',11:'CHARS/MANOMANO',12:'POSTCARD/CITY',14:'CHARS/AXLE-N',15:'STREET/HOME',16:'RESTROOM/REST1',17:'RESTROOM/REST1',18:'RESTROOM/REST1',19:'RESTROOM/REST1',20:'OLLEYS',21:'SHOWROOM/RAT/BIKES',22:'SHOWROOM/SPORT/BIKES',23:'SHOWROOM/SUPER/BIKES',24:'BIKES/RAT/BANZAI',25:'PANZER',26:'CHARS/AXLE-F',27:'POSTCARD/CITY',28:'RESULTS/WRECK1',29:'RESULTS/BUST1',30:'RESULTS/WIN1',31:'RESULTS/LOSE1',32:'RESULTS/LEVEL1',35:'CREDITS'}
    by_source={i['source']:i for i in data['images']}
    for screen in data['screens']:
        name=backgrounds.get(screen['original_index'])
        if not name:continue
        key='IMAGES/'+name+'.RRI';record=by_source[key];im=Image.open(kit/record['preview']).convert('RGB');out=Image.new('RGB',(640,480))
        sources=[key];x=y=0
        if name.startswith('POSTCARD/') or name.startswith('STREET/'):
            base='IMAGES/'+name.split('/')[0]+'.RRI';sources.insert(0,base);out.paste(Image.open(kit/by_source[base]['preview']).convert('RGB'),(0,0))
            x,y=(640-im.width,0) if name.startswith('POSTCARD/') else (146,230)
        out.paste(im,(x,y));preview=f'menu-previews/{screen["id"]}.png';out.save(kit/preview)
        screen.update(preview_reference=preview,composition_sources=sources,preview_note='Illustrative source composition/default focus, not a captured retail state. Dynamic text/fonts/handlers are not reproduced.')
    data['reference_pixels_bundled']=True
    for image in data['images']:
        source=image['source'];name=Path(source).stem;category=image['category']
        family=category+'-'+name.lower();variant='single';notes='Create original replacement art; keep live text separate.'
        if '/CHARS/' in source and '-' in name:
            character,variant=name.rsplit('-',1);family='rider-'+character.lower();notes='F/N/H are relationship-selected variants; preserve source context without guessing expression names.'
        elif '/BIKES/' in source and source.endswith('DASH.BOB'):
            family='dashboard-'+source.split('/')[2].lower();variant=('kmh' if name[0]=='K' else 'mph')+('-high' if name[1]=='H' else '-low');notes='One scalable dashboard master per family; labels/needles/meters remain dynamic. Generate units/resolution variants.'
        elif '/BIKES/' in source or '/SHOWROOM/' in source:
            family='bike-'+source.split('/')[2].lower()+'-'+name.lower();variant='detail' if '/BIKES/' in source else 'shop';notes='Pair shop/detail renders of the same bike; category BIKES images are overview art.'
        elif category=='results':
            match=re.fullmatch(r'([A-Z]+)([0-9]*)',name);family='result-'+match[1].lower();variant=match[2] or 'single';notes='Original illustration variants; text/rewards are overlays. A smaller approved initial set can share this family.'
        elif category=='options-backgrounds':family='options-background';variant=name
        elif category=='application-icons':family='application-brand-icon';variant=f"{image['width']}px-{source}";notes='One new RoadDash logo master; generate platform sizes. Original icons are reference-only.'
        elif name in ['BIGMETER','LILMETER']:family='hud-condition-stamina';variant=name;notes='Atlas reference, not a finished HUD panel. Rebuild scalable bars with fill/empty states.'
        elif category=='world-context':notes='Not a menu/UI replacement requirement; context-only horizon/cloud inventory.'
        image.update(master_family=family,variant=variant,art_notes=notes,used_by_screens=[p['id'] for p in plan['screens'] if image['category'] in p['categories'] and (not p.get('source_pattern') or re.search(p['source_pattern'],source))])
        if [image['width'],image['height']]==[640,480]:image['recommended_master']=[3840,2160]
    public=json.loads(json.dumps(data))
    for screen in public['screens']:
        screen.pop('title',None)
        for item in screen['items']:item['caption']=f'Text slot {item["caption_slot"]}' if item['caption_slot'] else ''
    for popup in public['popups']:
        for item in popup['items']:item['caption']=f'Text slot {item["caption_slot"]}' if item['caption_slot'] else ''
    for d in public['windows_dialogs']:
        d['title']=''
        for c in d['controls']:c['label']='Control '+str(c['control_id'])
    public['reference_pixels_bundled']=False
    (ROOT/'docs/ui/catalog.json').write_text(json.dumps(public,indent=2)+'\n')
    for folder,metadata in [(ROOT/'docs/ui',public),(kit,data)]:
        folder.mkdir(exist_ok=True)
        metadata['plan']=plan
        (folder/'catalog-data.js').write_text('window.UI_KIT_DATA = '+json.dumps(metadata,ensure_ascii=True,separators=(',',':'))+';\n')
        (folder/'wireframes').mkdir(exist_ok=True)
        for screen in public['screens']+public['popups']:(folder/'wireframes'/f'{screen["id"]}.svg').write_text(svg(screen))
        for d in metadata['windows_dialogs']:
            w,h=d['rect'][2:]
            body=[f'<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="{round(1000*h/w)}" viewBox="0 0 {w} {h}"><rect width="{w}" height="{h}" fill="#20252b"/>']
            for c in d['controls']:
                x,y,cw,ch=c['rect'];body.append(f'<rect x="{x}" y="{y}" width="{max(cw,0)}" height="{max(ch,0)}" fill="#35424e" stroke="#819bad" stroke-width=".3"/><text x="{x+1}" y="{y+min(ch-1,5)}" font-family="sans-serif" font-size="4" fill="white">{c["control_id"]}</text>')
            (folder/'wireframes'/f'{d["id"]}.svg').write_text('\n'.join(body+['</svg>'])+'\n')
        fields=['id','source','category','scope','width','height','replacement','master_family','variant','used_by_screens','recommended_master','art_notes','status','sha256','preview']
        with (folder/'IMAGE_LIST.csv').open('w',newline='') as f:
            writer=csv.DictWriter(f,fieldnames=fields,extrasaction='ignore',lineterminator='\n');writer.writeheader();writer.writerows(metadata['images'])
        with (folder/'VIDEO_LIST.csv').open('w',newline='') as f:
            fields=['id','source','width','height','duration_seconds','status','sha256','preview'];writer=csv.DictWriter(f,fieldnames=fields,extrasaction='ignore',lineterminator='\n');writer.writeheader();writer.writerows(metadata['videos'])
    (kit/'catalog.json').write_text(json.dumps(data,indent=2)+'\n')
    (kit/'english-menu-reference.json').write_text(json.dumps(dict(screens=data['screens'],popups=data['popups']),indent=2)+'\n')
    for name in ['index.html','app.js','style.css','plan.json','blank-progress.json','refresh_plan.py']:shutil.copy2(ROOT/'docs/ui'/name,kit/name)
    checklist=['# UI art replacement checklist','', 'Reference inventory, not a requirement to repaint every redundant bitmap. No original images are bundled in this repository.','', 'Use the local reference gallery to compare variants. Check only user-approved original replacements; low/high and unit variants can share one scalable master.','']
    for category in sorted({i['category'] for i in data['images'] if i['scope']=='ui'}):
        checklist += [f'## {category}','']
        for record in data['images']:
            if record['category']!=category:continue
            checklist.append(f'- [ ] **{record["id"]}** `{record["source"]}` — {record["width"]}×{record["height"]}; replacement `{record["replacement"]}`.')
        checklist.append('')
    checklist += ['## Reusable components (new RoadDash requirements)','']
    checklist += [f'- [ ] **{c["id"]}** {c["name"]} — {c["kind"]}; states: {", ".join(c["states"])}.' for c in plan['components']]
    checklist += ['','## Optional / context','','59 video clips are listed in VIDEO_LIST.csv. Rebuild original cutscenes or replace them with new static cards; they are not mandatory UI stills. Twelve horizon/cloud files are context-only, not UI tasks. Original fonts are inventoried but not copied or licensed for production.','']
    (ROOT/'docs/ui/ASSET_CHECKLIST.md').write_text('\n'.join(checklist))
    shutil.copy2(ROOT/'docs/ui/ASSET_CHECKLIST.md',kit/'ASSET_CHECKLIST.md')
    table=['# Original screen and popup index','', 'All36 stored screen slots are included;29 have stored items. Slots0–6 are bootstrap/race transitions, not seven extra menus. Dynamic handlers mutate several pages; this is not proof of every runtime state.','', '| ID | Original role | Stored controls | RoadDash draft |','| --- | --- | ---: | --- |']
    for screen in data['screens']:
        drafts=', '.join(p['id'] for p in plan['screens'] if screen['original_index'] in p['legacy']) or 'transition / reference only'
        table.append(f'| {screen["id"]} | {screen["name"]} | {len(screen["items"])} | {drafts} |')
    table += ['','## Original stored popups','']+[f'- {p["id"]}: {p["name"]} ({len(p["items"])} rows).' for p in data['popups']]
    table += ['','## Dynamic UI with no dedicated image file','','- Save filename list/name entry and overwrite / unsaved / failed-load messages.','- Purchase affordability, character/bike stats and contextual disabled entries.','- Fifteen-row results, penalty/game-over text, level and qualification state.','- Relationship-selected F/N/H portrait and conversation text.','- Countdown, announcements, needle and stamina/condition meters.','- Original Sound Mix opens native Windows dialog300, separately cataloged; modern fullscreen audio settings are a new proposal.','','## Reconstruction additions, not retail screens','','IDs0x24–0x2B add display, sound, pause, gamepad, touch, enhancements, online lobby and room browser in the inspected reconstruction. Those are not counted as original retail menu screens. RoadDash settings/pause/network proposals are explicitly marked in plan.json.','']
    (ROOT/'docs/ui/SCREEN_MAP.md').write_text('\n'.join(table));shutil.copy2(ROOT/'docs/ui/SCREEN_MAP.md',kit/'SCREEN_MAP.md')
    result=dict(counts=data['counts'],categories=dict(Counter(i['category'] for i in data['images'])),wireframes=len(data['screens'])+len(data['popups'])+len(data['windows_dialogs']),draft_screens=len(plan['screens']),components=len(plan['components']),reference_output=str(kit),original_pixels_in_repo=False,failures=data['failures'])
    masters=['# Master art families','', 'The142 UI source records reduce to66 proposed master families before further art-direction reuse. This is a planning grouping, not66 completed assets. Dashboard units/resolutions and application-icon sizes can be generated from one scalable master. Individual references remain in IMAGE_LIST.csv.','', 'For fullscreen backgrounds,3840×2160 is a proposed RoadDash master, not the original4:3 size. Preserve key subjects inside the central4:3 region until layout approval; keep the5% outer safe area free of text. Other pieces keep their source aspect as a suggested4× master. Use transparent PNG for cutouts, layers/masks for editable sources, and live/vector type and meters.','']
    for family in sorted({i['master_family'] for i in data['images'] if i['scope']=='ui'}):
        rows=[i for i in data['images'] if i['master_family']==family and i['scope']=='ui']
        masters += [f'## {family}', '', rows[0]['art_notes'], '', 'References: '+', '.join(i['id']+' ('+i['variant']+')' for i in rows)+'.', 'Used by: '+', '.join(sorted({s for i in rows for s in i['used_by_screens']}))+'.', '']
    masters += ['## Unresolved / inherited usage','', 'MATTE assets and several screen backgrounds are cache/inherited/legacy contexts; a dedicated RoadDash visual is not automatically required. File coverage is complete, but a full retail playthrough/state-by-state parity audit is not claimed. Cloud dimensions are explicitly inferred and context-only. Do not treat pictures of original logos/fonts as production permission.','']
    (ROOT/'docs/ui/MASTER_ART_BRIEFS.md').write_text('\n'.join(masters));shutil.copy2(ROOT/'docs/ui/MASTER_ART_BRIEFS.md',kit/'MASTER_ART_BRIEFS.md')
    (ROOT/'docs/ui/COVERAGE.json').write_text(json.dumps(result,indent=2)+'\n')
    from build_ui_prompts import build as build_prompts
    build_prompts()
    for name in ['SCREEN_GENERATION_PROMPTS.md', 'IMAGE_GENERATION_PROMPTS.md', 'art_prompts.json']:
        shutil.copy2(ROOT/'docs/ui'/name, kit/name)
    if (ROOT/'docs/ui/generated/manifest.json').is_file():
        from package_ui_screen_art import build as build_art, copy_to
        build_art()
        copy_to(kit)
    print(json.dumps(result,indent=2))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--kit',type=Path,required=True);p.add_argument('--install',type=Path,required=True);a=p.parse_args();package(a.kit.resolve(),a.install.resolve())
