#!/usr/bin/env python3
"""Portable, offline static review gallery and original-model contact sheets."""
from pathlib import Path
import json,html,math
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation';OUT=BASE/'gallery'
manifest=json.loads((BASE/'manifest.json').read_text())
poses=[json.loads((BASE/f"poses/{d['id']:04d}.json").read_text()) for d in manifest['poses']]
missing=[p['id'] for p in poses if not (OUT/f"{p['id']:04d}.jpg").exists()]
if missing:raise SystemExit(f'Missing renders: {missing}')
font_path=next((p for p in [Path('/usr/share/fonts/google-noto/NotoSans-Regular.ttf'),Path('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')] if p.exists()),None)
font=ImageFont.truetype(str(font_path),16) if font_path else ImageFont.load_default()
small=ImageFont.truetype(str(font_path),12) if font_path else font
pages=[]
for page,start in enumerate(range(0,len(poses),20),1):
    rows=poses[start:start+20];sheet=Image.new('RGB',(1400,75+math.ceil(len(rows)/4)*350),'#17212d');draw=ImageDraw.Draw(sheet)
    draw.text((20,18),f'ROADDASH / STATIC KEYFRAME REVIEW / {page:02} / DRAFT',font=font,fill='white')
    for i,p in enumerate(rows):
        x=i%4*350;y=75+i//4*350
        im=Image.open(OUT/f"{p['id']:04d}.jpg").convert('RGB');im.thumbnail((340,290));sheet.paste(im,(x+(350-im.width)//2,y))
        draw.text((x+10,y+290),f"{p['id']:04d} {p['name'][:34]}",font=small,fill='white')
        draw.text((x+10,y+311),p['family']+' / '+p['reference_confidence'],font=small,fill='#9edbd3')
        draw.text((x+10,y+330),'Held: '+(', '.join(p['held_contacts']) or 'released'),font=small,fill='#b9c7d4')
    dest=OUT/f'overview-{page:02d}.jpg';sheet.save(dest,quality=88);pages.append(dest.name)
# Compact selection spans motion families for a useful first review image.
selection=[3,4,45,60,1012,1051,1081,1085,1101,1142,1203,1209]
hero=Image.new('RGB',(1200,990),'#17212d');draw=ImageDraw.Draw(hero);draw.text((20,10),'ROADDASH — KEYFRAME CANDIDATES / NO RUNTIME ANIMATION YET',font=font,fill='white')
for i,id in enumerate(selection):
    p=next(p for p in poses if p['id']==id);im=Image.open(OUT/f'{id:04d}.jpg').convert('RGB');im.thumbnail((290,285));x=i%4*300;y=45+i//4*315;hero.paste(im,(x,y));draw.text((x+8,y+285),f'{id:04d} {p["name"][:28]}',font=small,fill='white')
hero.save(OUT/'review-overview.jpg',quality=92)
data=[]
for p in poses:
    data.append({k:p[k] for k in ['id','name','actor','family','source_slots','held_contacts','released_contacts','reference_confidence','review'] } | {'views':[n for n in ['front','rear','side'] if (OUT/f'views/{p["id"]:04d}-{n}.jpg').exists()]})
payload=json.dumps(dict(poses=data,use_cases=manifest['use_cases']),separators=(',',':')).replace('<','\\u003c')
page='''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>RoadDash keyframe review</title>
<style>body{margin:0;background:#101820;color:#edf4fa;font:16px system-ui}header{padding:28px;max-width:1100px}h1{margin:0 0 12px}a{color:#87e3d1}nav{position:sticky;top:0;background:#182631;padding:16px;display:flex;gap:12px;flex-wrap:wrap;z-index:1}input,select,button{font:inherit;padding:9px;background:#253744;color:white;border:1px solid #668090;border-radius:5px}#count{align-self:center}main{display:grid;grid-template-columns:repeat(auto-fill,minmax(260px,1fr));gap:18px;padding:20px}article{background:#1d2b36;padding:14px;border-radius:8px}article img{width:100%;cursor:zoom-in}article h2{font-size:17px}article p{font-size:13px;line-height:1.5;color:#c3d4df}dialog{background:#172630;color:white;max-width:900px;width:85vw;border:1px solid #648492;border-radius:8px}dialog::backdrop{background:#000b}dialog img{display:block;width:min(100%,650px);max-height:45vh;object-fit:contain;margin:auto}dialog button{margin:4px}#details{white-space:pre-line;font-size:14px;line-height:1.4}#help{max-width:1050px;line-height:1.6}</style>
<header><h1>RoadDash — V1 draft poses (semantic labels superseded)</h1><div id="help">V1 SEMANTIC LABELS SUPERSEDED: use the context-first catalog at docs/animation/context/README.md and art/animation/context/gallery/index.html. Static authored candidates, not animated clips. Filter by use case to compare its numbered poses. Reference slot numbers are preserved; empty/view variants are not timing samples. Open a pose for contact metadata and front/rear/side views where available. Fingers, cloth/clearance and ambiguous mount/reaction references still need review. Runtime controls and the physical ragdoll are unchanged in this authoring stage.</div><p><a href="review-overview.jpg">Review overview</a> · <a href="../manifest.json">Catalog</a> · <a href="../../../docs/animation/README.md">Scope and remaining work</a></p></header>
<nav><label>Find <input id="search" placeholder="Name or number"></label><label>Family <select id="family"><option value="">All</option></select></label><label>Use case <select id="case"><option value="">All</option></select></label><span id="count" aria-live="polite"></span></nav><main id="cards"></main>
<dialog id="modal"><button id="close">Close</button><h2 id="title"></h2><img id="preview" alt=""><div id="angles"></div><p id="details"></p><a id="json">Pose JSON</a></dialog>
<script id="data" type="application/json">PAYLOAD</script>
<script>
const data=JSON.parse(document.getElementById('data').textContent),byId=new Map(data.poses.map(p=>[p.id,p])),cards=document.getElementById('cards'),search=document.getElementById('search'),family=document.getElementById('family'),caseSelect=document.getElementById('case'),modal=document.getElementById('modal');
for(const name of [...new Set(data.poses.map(p=>p.family))].sort()){const o=new Option(name,name);family.add(o)}
for(const c of data.use_cases)caseSelect.add(new Option(c.id,c.id));
function imgpath(p,v){return v?`views/${String(p.id).padStart(4,'0')}-${v}.jpg`:`${String(p.id).padStart(4,'0')}.jpg`}
function show(p){document.getElementById('title').textContent=`${String(p.id).padStart(4,'0')} · ${p.name} · DRAFT`;const im=document.getElementById('preview');im.src=imgpath(p);im.alt=p.name;const angles=document.getElementById('angles');angles.replaceChildren();for(const v of ['',...p.views]){const b=document.createElement('button');b.textContent=v||'Primary view';b.onclick=()=>im.src=imgpath(p,v);angles.append(b)}document.getElementById('details').textContent=`Family: ${p.family}\nSource slots: ${p.source_slots.join(', ')||'Original design; no direct sprite'}\nReference confidence: ${p.reference_confidence}\nHeld: ${p.held_contacts.join(', ')||'none'}\nReleased: ${p.released_contacts.join(', ')||'none'}\nNo recovered timing or runtime interpolation.`;document.getElementById('json').href=`../poses/${String(p.id).padStart(4,'0')}.json`;modal.showModal()}
function draw(){cards.replaceChildren();const c=data.use_cases.find(c=>c.id===caseSelect.value);let poses=c?c.pose_ids.map(id=>byId.get(id)):data.poses;poses=poses.filter(p=>(!family.value||p.family===family.value)&&(`${p.id} ${p.name}`.toLowerCase().includes(search.value.toLowerCase())));document.getElementById('count').textContent=`${poses.length} shown / ${data.poses.length} static poses`;for(const p of poses){const a=document.createElement('article'),button=document.createElement('button'),im=document.createElement('img'),h=document.createElement('h2'),note=document.createElement('p');im.src=imgpath(p);im.loading='lazy';im.alt=p.name;button.style.padding='0';button.setAttribute('aria-label',`Review pose ${p.id} ${p.name}`);button.append(im);button.onclick=()=>show(p);h.textContent=`${String(p.id).padStart(4,'0')} · ${p.name}`;note.textContent=`${p.family} · ${p.reference_confidence} · ${p.held_contacts.length} held contacts`;a.append(button,h,note);cards.append(a)}}
search.addEventListener('input',draw);family.addEventListener('change',draw);caseSelect.addEventListener('change',draw);document.getElementById('close').onclick=()=>modal.close();draw();
</script></html>'''.replace('PAYLOAD',payload)
(OUT/'index.html').write_text(page)
print(len(poses),'gallery entries;',len(pages),'contact sheets')
