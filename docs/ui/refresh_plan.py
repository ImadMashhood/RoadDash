#!/usr/bin/env python3
"""Refresh this standalone artist desk after editing plan.json; no game files needed."""
import json
from pathlib import Path
import re


def validate_plan(plan, catalog):
    if plan.get('schema_version') != 1:
        raise ValueError('unsupported plan schema')
    screens=plan.get('screens',[])
    if not isinstance(screens,list) or not 1<=len(screens)<=128:
        raise ValueError('need1..128 draft screens')
    ids=[s.get('id','') for s in screens]
    if len(set(ids))!=len(ids) or 'main' not in ids or any(not re.fullmatch(r'[a-z][a-z0-9_-]{0,50}',i) for i in ids):
        raise ValueError('unique simple screen IDs including main are required')
    sources={i['source'] for i in catalog['images']}
    for screen in screens:
        if screen.get('layout') not in ['hero','menu','cards','hub','dialogue','settings','hud','results','modal']:
            raise ValueError('unknown layout for '+screen['id'])
        if not isinstance(screen.get('title'),str) or not screen['title'].strip():
            raise ValueError('each screen needs a title')
        actions=screen.get('actions',[])
        if len(actions)>10 or any(not isinstance(a,list) or len(a)!=2 or a[1] not in ids for a in actions):
            raise ValueError('all action targets must be draft screen IDs')
        if screen.get('preferred_source') and screen['preferred_source'] not in sources:
            raise ValueError('unknown preferred reference source')
        re.compile(screen.get('source_pattern',''))
    canvas=plan.get('canvas')
    if not isinstance(canvas,list) or len(canvas)!=2 or any(not isinstance(v,int) or not 640<=v<=7680 for v in canvas):
        raise ValueError('canvas must contain two dimensions between640 and7680')
    if not isinstance(plan.get('components'),list):
        raise ValueError('components must be a list')


def refresh(folder):
    folder=Path(folder)
    plan=json.loads((folder/'plan.json').read_text())
    catalog=json.loads((folder/'catalog.json').read_text())
    validate_plan(plan,catalog)
    catalog['plan']=plan
    for image in catalog['images']:
        image['used_by_screens']=[s['id'] for s in plan['screens'] if image['category'] in s.get('categories',[]) and (not s.get('source_pattern') or re.search(s['source_pattern'],image['source']))]
    target=folder/'catalog-data.js';temporary=folder/'catalog-data.js.tmp'
    temporary.write_text('window.UI_KIT_DATA = '+json.dumps(catalog,ensure_ascii=True,separators=(',',':'))+';\n')
    temporary.replace(target)
    return len(plan['screens'])

if __name__=='__main__':
    count=refresh(Path(__file__).resolve().parent)
    print(f'Refreshed{count}draft screens. Reload index.html. Original images/artwork files were not changed.')
