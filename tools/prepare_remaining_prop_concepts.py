#!/usr/bin/env python3
"""Prepare a resumable ID-complete concept batch, without duplicating finished IDs."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/props/concept_3d'
LOCAL=Path.home()/'Downloads/RoadDash_Prop_References'
MANIFEST=OUT/'ALL_REMAINING.json'
STYLE=ROOT/'art/buildings/concept_3d/facades_01.png'
COMMON='''Create one high-quality 3-by-2 concept-art sheet for RoadDash, with exactly the supplied subjects in exactly the same row-major slots as image1. Image1 is subject/silhouette/color reference ONLY; image2 is STYLE reference ONLY (our building concepts). Rebuild the visible source subjects in that detailed stylized-realistic game-asset style: rich restrained colors, legible slightly faceted geometry, believable worn materials, crisp surface detail. Neutral charcoal studio backdrop, coherent soft lighting/contact shadows, complete three-quarter orthographic assets. Match the source shapes, proportions, colors and constituent parts carefully. Each object fills its own cell at an independent useful review scale, ample margins, no cropping. A cluster/composite stays a cluster/composite. A sign depicting a person/animal stays a sign, never invent a live actor from a filename. A flat ground patch stays a detailed flat material patch on a thin review slab. Don't infer identity from abbreviated filenames. Keep structures or object assemblies if that is what the source visibly shows. No sprite thumbnails, image cards, added scenery, brands/corporate logos, watermarks, property hashes, metadata or descriptions. Neutral human/animal poses, full adult anatomy and feet visible, matching source clothes/colors; do not copy flailing/crash poses into a neutral character design. No gore. Do not make an unrelated generic substitute. One original reconstruction per source slot. Only include the short source labels given below and a restrained category title. These are bitmap design concepts, not exported3D meshes. Large landscape sheet.'''

def prepare():
 if MANIFEST.exists():
  frozen=json.loads(MANIFEST.read_text())
  print(json.dumps(dict(already_covered=len(frozen['already_covered_ids']), remaining=len(frozen['remaining_ids']), sheets=len(frozen['sheets']), frozen=True)))
  return
 queue=json.loads((ROOT/'docs/research/props/rebuild-queue.json').read_text())
 catalog=json.loads((LOCAL/'catalog.json').read_text());records={r['id']:r for r in catalog['records']}
 old=json.loads(MANIFEST.read_text()) if MANIFEST.exists() else dict(sheets=[])
 by_name={j['batch']:j for j in old['sheets']};groups={};done=[]
 for job in queue['jobs']:
  if job['concept_status']=='draft_not_approved' and job.get('concept') and (ROOT/job['concept']['file']).exists():done.append(job['id']);continue
  groups.setdefault(job['group'],[]).append(records[job['id']])
 source_dir=LOCAL/'remaining_sources';source_dir.mkdir(exist_ok=True)
 font=ImageFont.truetype('/usr/share/fonts/google-noto/NotoSans-Regular.ttf',23)
 sheets=[]
 for group,entries in sorted(groups.items()):
  for start in range(0,len(entries),6):
   name=f'all_{group}_{start//6+1:02}';selected=entries[start:start+6];cells=[]
   im=Image.new('RGB',(1800,1080),(30,34,40));draw=ImageDraw.Draw(im)
   for i,r in enumerate(selected):
    x=i%3*600;y=i//3*540;label=r['names'][0]
    draw.text((x+300,y+10),f'{i+1:02} {label}',font=font,fill='white',anchor='mt')
    previews=r['cells'][:3] if len(r['cells'])>1 else r['cells'][:1]
    for k,c in enumerate(previews):
     pic=Image.open(LOCAL/'individual'/(c['pixel_sha256']+'.png')).convert('RGBA')
     width=560//len(previews);scale=min((width-12)/pic.width,435/pic.height)
     pic=pic.resize((max(1,round(pic.width*scale)),max(1,round(pic.height*scale))),Image.Resampling.NEAREST)
     im.paste(pic,(x+20+k*width+(width-pic.width)//2,y+65+(435-pic.height)//2),pic)
    cells.append(dict(slot=i+1,id=r['id'],label=label,names=r['names']))
   for i in range(len(selected),6):
    draw.text((i%3*600+300,i//3*540+240),'EMPTY — leave blank',font=font,fill=(150,150,150),anchor='mm')
   source=source_dir/(name+'.png');im.save(source)
   prompt=COMMON+' Category title: ROADDASH — '+group.replace('_',' ').upper()+'. Labels row-major: '+', '.join(c['label'] for c in cells)+'.'
   if len(selected)<6:prompt+=f' Only {len(selected)} occupied cells; remaining cells must be empty background, no invented extra objects.'
   if group=='pedestrians_animals':prompt+=' Source arrays may show multiple poses of the same outfit; produce ONE neutral standing character/animal per occupied cell, retain species and clothing rather than duplicating the poses.'
   record=dict(batch=name,group=group,source_sheet=str(source),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),cells=cells,prompt=prompt,status='pending',source_reviewed=False,output_reviewed=False)
   previous=by_name.get(name)
   if previous:
    if previous['cells']!=cells:raise ValueError('changed batch cells; preserve old manifest before rebuilding')
    record=previous
   sheets.append(record)
 result=dict(schema_version=1,scope='Every remaining prop/actor source group, including alternate view/color/size banks and unresolved subjects. No architectural/terrain/global cross-reference banks added.',already_covered_ids=sorted(done),remaining_ids=sorted(c['id']for s in sheets for c in s['cells']),
             style_reference=str(STYLE.relative_to(ROOT)),style_sha256=hashlib.sha256(STYLE.read_bytes()).hexdigest(),sheets=sheets)
 if len(result['remaining_ids'])!=len(set(result['remaining_ids'])):raise ValueError('duplicate ID')
 MANIFEST.write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps(dict(already_covered=len(done),remaining=len(result['remaining_ids']),sheets=len(sheets),by_group={k:len(v)for k,v in groups.items()})))
if __name__=='__main__':prepare()
