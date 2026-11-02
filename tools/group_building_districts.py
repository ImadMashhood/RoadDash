#!/usr/bin/env python3
"""Group real assets without copying meshes or mixing district inference with disc facts."""
import argparse
import csv
import hashlib
import json
import math
import os
import re
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[1]
KIT=ROOT/'art/buildings/mesh_assets'
OUT=KIT/'districts'
# These are authored RoadDash district roles. Course associations below are measured.
GROUPS=[
 ('commercial_high_street','Commercial high street',['BUSS'],'Continuous shopfronts, awnings and signs; group related storefronts along the same block.'),
 ('office_centre','Office centre',['OFFC'],'Taller urban business blocks; concentrate rather than scatter among farms.'),
 ('chinatown','Chinatown',['CHINA','CHINGAT2'],'Chinese-influenced fronts with the gateway as a landmark; City-family association observed.'),
 ('victorian_residential','Victorian residential',['VICT'],'Residential rows with bays and stoops; keep neighbouring heights and roof language coherent.'),
 ('small_town_main_street','Small-town main street',['CLSTGA'],'Lower town-centre frontages; use in rural/coastal towns rather than treating all as downtown offices.'),
 ('run_down_quarter','Run-down quarter',['RUNDWN'],'Boarded and worn urban streets; transition from industrial or older residential blocks.'),
 ('industrial_warehouses','Industrial / warehouses',['WHSE','WRHS'],'Loading bays, large gridded windows and service yards; reserve access outside the race carriageway.'),
 ('farms_agriculture','Farms / agriculture',['BARN','GRNTNK2','HAYSHED2'],'Barns, grain storage and open hay shelters; standalone field-edge placement with yard clearance.'),
 ('rural_village_winery','Rural village / winery',['CHURCH2','HOUSEB2','RUSTICA2','RUSTICB2','SCHOOL2','WINNERY2'],'Detached homes, community landmarks and winery; space these as settlements, not touching urban facades.'),
 ('roadside_services','Roadside services',['BUSSTOP2','CARWASH2','GASA2','GASB2','PHONSHK2','SHOPD2','SHOPE2','SHOPICE2','SNCKSHP2'],'Shared settlement services; use course eligibility and site clearance, not every service in every district.'),
 ('utilities_service_yards','Utilities / service yards',['OUTHOUS2','PORTLET2','TANKSEP2','WATRTNK2','WHTTANK2'],'Ancillary tanks and facilities; site behind/alongside compatible buildings, not as frontage substitutes.'),
 ('retaining_tunnels','Retaining walls / tunnels',['METL','TUN','TOWERB2'],'Structural wall, tunnel and tower kit; follows terrain/engineering needs, never random building lots.'),
 ('shared_roof_edges','Shared roof / edge modules',['TOP'],'Reusable trim modules; select for architectural compatibility and dimensions, not standalone building placement.')]


def build():
    catalog=json.loads((KIT/'CATALOG_MANIFEST.json').read_text());designs=json.loads((KIT/'catalog_designs.json').read_text())['designs']
    source=json.loads((ROOT/'docs/research/building-districts/source-associations.json').read_text())
    observed={r['id']:r for r in source['assets']};records={r['id']:r for r in catalog['records']};members={key:[]for key,*_ in GROUPS};assets=[]
    for design in designs:
        identity=design['id'];matches=[key for key,label,prefixes,rule in GROUPS if any(identity==prefix or re.match('^'+prefix+r'\d',identity) or (prefix=='BARN' and identity.startswith('BARND')) or (prefix=='TUN' and identity.startswith('TUNN')) for prefix in prefixes)]
        if len(matches)!=1:raise ValueError('Ambiguous or missing district '+identity+str(matches))
        group=matches[0];members[group].append(identity);record=records[identity]
        assets.append(dict(id=identity,asset_type=record['asset_type'],primary_district=group,assignment_basis='RoadDash art-direction inference from original named asset family and modeled form',
            original_family_ids=observed[identity]['family_ids'],original_course_family_load_matches=observed[identity]['course_family_load_matches'],
            course_evidence=observed[identity]['evidence'],lods=[dict(file='../models/'+l['file'],sha256=l['sha256'])for l in record['lods']],render='../'+record['render']))
    result=dict(schema_version=1,scope='Catalog organization only; no changes to gameplay or existing map selection',district_labels_are_original=False,
        source_evidence='../../../../docs/research/building-districts/source-associations.json',
        assignment_rule='Exactly one primary district per ID; original course eligibility is many-to-many and never inferred from appearance',
        districts=[dict(id=key,label=label,asset_ids=members[key],placement_guidance=rule)for key,label,prefixes,rule in GROUPS],assets=assets,
        courses=[dict(code=code,label=c['label'],asset_ids=[a['id']for a in assets if code in a['original_course_family_load_matches']],
            evidence='Named assets in loaded families across stored section definitions; not exact placement, route order, frequency or availability at every point')for code,c in source['courses'].items()])
    if len(assets)!=146 or set(observed)!=set(records):raise ValueError('Incomplete source/model coverage')
    return result


def render_sheet(ids,assets,destination,title,subtitle):
    cols=min(6,len(ids));rows=math.ceil(len(ids)/cols);width=max(cols*240,900);height=rows*285+90
    canvas=Image.new('RGB',(width,height),'#252b30');draw=ImageDraw.Draw(canvas);font=ImageFont.load_default(size=19);small=ImageFont.load_default(size=14)
    draw.text((18,15),title,font=font,fill='white');draw.text((18,48),subtitle,font=small,fill='#bfc9d2')
    for i,identity in enumerate(ids):
        a=assets[identity];path=OUT/a['render'];im=Image.open(path).convert('RGB');im.thumbnail((236,238));x=(i%cols)*240;y=(i//cols)*285+85
        canvas.paste(im,(x+(240-im.width)//2,y));draw.text((x+8,y+241),identity+' | '+a['asset_type'],font=small,fill='white')
    canvas.save(destination)


def render_overview(result):
    by_id={a['id']:a for a in result['assets']};canvas=Image.new('RGB',(1500,1500),'#252b30');draw=ImageDraw.Draw(canvas)
    font=ImageFont.load_default(size=19);small=ImageFont.load_default(size=14)
    for i,group in enumerate(result['districts']):
        x=(i%3)*500;y=(i//3)*300;draw.text((x+10,y+8),group['label'],font=font,fill='white')
        ids=group['asset_ids'];draw.text((x+10,y+36),str(len(ids))+' assets | examples below',font=small,fill='#bfc9d2')
        examples=ids if len(ids)<=4 else [ids[j*len(ids)//4]for j in range(4)]
        for j,identity in enumerate(examples):
            im=Image.open(OUT/by_id[identity]['render']).convert('RGB');im.thumbnail((120,210));px=x+j*123
            canvas.paste(im,(px+(120-im.width)//2,y+58));draw.text((px+4,y+275),identity,font=small,fill='white')
    canvas.save(OUT/'district-overview.png')


def write(result):
    OUT.mkdir(parents=True,exist_ok=True);(OUT/'districts.json').write_text(json.dumps(result,indent=2)+'\n');by_id={a['id']:a for a in result['assets']}
    with (OUT/'asset_assignments.csv').open('w')as f:
        w=csv.writer(f);w.writerow(['id','asset_type','primary_district','original_family_ids','course_family_load_matches','assignment_basis'])
        for a in result['assets']:w.writerow([a['id'],a['asset_type'],a['primary_district'],';'.join(map(str,a['original_family_ids'])),';'.join(a['original_course_family_load_matches']),a['assignment_basis']])
    for group in result['districts']:
        folder=OUT/group['id'];folder.mkdir(exist_ok=True);links=folder/'models';links.mkdir(exist_ok=True)
        group_assets=json.loads(json.dumps([by_id[i]for i in group['asset_ids']]))
        for asset in group_assets:
            asset['render']='../'+asset['render']
            for lod in asset['lods']:lod['file']='../'+lod['file']
        (folder/'manifest.json').write_text(json.dumps(dict(**group,assets=group_assets,paths_relative_to_manifest=True),indent=2)+'\n')
        for asset in group_assets:
            for lod in asset['lods']:
                source=folder/lod['file'];target=links/source.name;relative=os.path.relpath(source,links)
                if target.is_symlink() and os.readlink(target)!=relative:target.unlink()
                if not target.exists():target.symlink_to(relative)
        render_sheet(group['asset_ids'],by_id,folder/'contact-sheet.png',group['label'],str(len(group_assets))+' assets | authored district grouping; course matches recorded separately')
    course_root=OUT/'original_courses';course_root.mkdir(exist_ok=True)
    for course in result['courses']:
        (course_root/(course['code']+'.json')).write_text(json.dumps(course,indent=2)+'\n')
        render_sheet(course['asset_ids'],by_id,course_root/(course['code']+'.png'),course['label']+' | '+course['code'],str(len(course['asset_ids']))+' assets in loaded families | eligibility, not a recreated course layout')
    render_overview(result)
    return dict(assets=len(result['assets']),districts=len(result['districts']),courses=len(result['courses']),links=sum(1 for p in OUT.glob('*/models/*.glb')),course_eligible_assets=sum(bool(a['original_course_family_load_matches'])for a in result['assets']))


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--check',action='store_true');a=ap.parse_args();result=build()
    if a.check:
        if json.loads((OUT/'districts.json').read_text())!=result:raise SystemExit('District catalog is stale')
        print('District metadata matches source evidence and all 146 models')
    else:print(json.dumps(write(result)))
