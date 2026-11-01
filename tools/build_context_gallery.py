#!/usr/bin/env python3
"""Action-grouped review, optional private retail comparisons outside repository."""
import argparse,json,html,math,shutil
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont,ImageOps,ImageChops
ROOT=Path(__file__).resolve().parents[1];BASE=ROOT/'art/animation/context'
FONT=next(p for p in [Path('/usr/share/fonts/google-noto/NotoSans-Regular.ttf'),Path('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')] if p.exists())
def sprites(root):
    raw=(root/'DATA/BIKERS/HIBOB.DAT').read_bytes();exe=(root/'ROADRASH.EXE').read_bytes();pal=(root/'DATA/PALETTE.RAW').read_bytes();remap=exe[0x61600+0x46e158-0x463000+4*256:][:256];colors=[tuple(pal[i*4:i*4+3]) for i in remap];frames={};offset=0
    for id in range(326):
        w,h=raw[offset:offset+2];offset+=2;px=raw[offset:offset+w*h];offset+=w*h
        if w*h and any(px):
            im=Image.new('RGBA',(w,h));im.putdata([colors[p]+(255 if p else 0,) for p in px]);frames[id]=im
    if offset!=len(raw):raise ValueError('Invalid BOB extent')
    return frames

def build(out,retail=None):
    out.mkdir(parents=True,exist_ok=True)
    if retail and (out==ROOT or ROOT in out.parents):raise ValueError('Retail comparison output must remain outside repository')
    catalog=json.loads((ROOT/'docs/animation/context/catalog.json').read_text());frames=sprites(retail) if retail else {};sections=[];missing=[];index=[]
    f=lambda n:ImageFont.truetype(str(FONT),n)
    for a in catalog['actions']:
        if a['bank']!='rider':continue
        category='combat' if a['selectors'] or 10<=a['code']<=27 else 'riding' if a['code']<10 else 'recovery' if 36<=a['code']<=42 else 'wreck' if a['code']>=43 else 'crash'
        cells=[];tiles=[]
        for p in a['phases']:
            id=2000+a['code']*10+p['phase']-1;source=BASE/f'gallery/{id:04d}.jpg';image=''
            if source.exists():
                if out==BASE/'gallery':image=f'{id:04d}.jpg'
                else:
                    (out/'renders').mkdir(exist_ok=True);shutil.copy2(source,out/f'renders/{id:04d}.jpg');image=f'renders/{id:04d}.jpg'
            elif a['code']!=43:missing.append(id)
            label=p['label'];mirror=' mirrored' if p['rear']['mirror_x'] else ''
            cells.append(f'<article><h3>{html.escape(label)}</h3>'+ (f'<img src="{image}" alt="{html.escape(label)} rear view">' if image else '<p>Empty source art; no invented recreation.</p>')+f'<p>Game slot {p["rear"]["slot"]}{mirror}<br>{p["duration_ticks"]} logical ticks · control {p["control"]}<br>{html.escape(p["role"])}<br>{html.escape(p["visual_description"])}</p></article>')
            if retail:
                tile=Image.new('RGB',(400,350),'#1b2834');d=ImageDraw.Draw(tile);d.text((8,5),label,font=f(13),fill='white');d.text((8,24),p['visual_description'][:54],font=f(12),fill='#b9ccdb')
                if image:
                    model=Image.open(source).convert('RGB');background=Image.open(BASE/'gallery/background.jpg').convert('RGB');mask=ImageChops.difference(model,background).convert('L').point(lambda v:255 if v>30 else 0);bounds=mask.getbbox()
                    if bounds:
                        x0,y0,x1,y1=bounds;model=model.crop((max(0,x0-8),max(0,y0-8),min(model.width,x1+8),min(model.height,y1+8)))
                    model.thumbnail((220,240));tile.paste(model,((220-model.width)//2,43+(250-model.height)//2))
                else:d.text((8,120),'NO SOURCE ART',font=f(15),fill='white')
                original=frames.get(p['rear']['slot'])
                if original:
                    original=ImageOps.mirror(original) if p['rear']['mirror_x'] else original.copy();scale=min(170/original.width,235/original.height);original=original.resize((max(1,int(original.width*scale)),max(1,int(original.height*scale))),Image.Resampling.NEAREST);tile.paste(original,(220+(175-original.width)//2,45+(250-original.height)//2),original)
                d.text((8,300),'3D RECREATION / DRAFT',font=f(11),fill='#b9ccdb');d.text((230,300),f'SLOT {p["rear"]["slot"]}'+(' M' if mirror else ''),font=f(12),fill='#efb765');d.text((8,324),f'Phase {p["phase"]} / ticks {p["duration_ticks"]} / control {p["control"]}',font=f(12),fill='#b9ccdb');tiles.append(tile)
            index.append({'label':label,'id':id,'code':a['code'],'phase':p['phase'],'slot':p['rear']['slot'],'mirror_x':p['rear']['mirror_x'],'render':image or None})
        comparison=''
        if retail and tiles:
            strip=Image.new('RGB',(max(1200,400*len(tiles)),440),'#1b2834');d=ImageDraw.Draw(strip);d.text((12,10),a['id']+f'  / SOURCE CODE {a["code"]}',font=f(24),fill='white');d.text((12,45),f'Ordered phases; mirror flags applied. Final phase control {a["phases"][-1]["control_action"]}. Draft art, source-backed sequence identity.',font=f(15),fill='#b9ccdb')
            for n,t in enumerate(tiles):strip.paste(t,(n*400,90))
            (out/'comparisons').mkdir(exist_ok=True);dest=f'comparisons/{a["id"]}.jpg';strip.save(out/dest,quality=95,subsampling=0);comparison=f'<a href="{dest}"><img class="strip" src="{dest}" alt="{html.escape(a["id"])} ordered original and recreated phases"></a>'
        phases=''.join(cells)
        fallback='' if a['available'] else f'<p>Unavailable original action. Fallback animation code {a["fallback_code"]}; no invented frames.</p>'
        sections.append(f'<section data-category="{category}" data-name="{html.escape(a["id"].lower())}"><h2>{html.escape(a["id"])} · code {a["code"]}</h2><p>{html.escape(a["context"])} <a href="{a["source_evidence"]}">Source selector</a></p>{fallback}{comparison}<div class="phases">{phases}</div></section>')
    if missing:raise ValueError('Missing context renders '+str(missing))
    title='Context-first sequence review'
    page='''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>RoadDash — context-first sequences</title><style>body{margin:0;background:#111e29;color:#eaf1f5;font:16px system-ui}header{padding:24px;background:#1c2d3d}h1{margin:0 0 12px}a{color:#9de0cf}label{margin-right:24px}input,select{font:inherit;padding:7px}main{padding:16px}section{padding:15px;background:#1b2b39;margin-bottom:20px}section[hidden]{display:none}.phases{display:flex;gap:12px;overflow-x:auto}article{min-width:240px;max-width:260px;background:#12212e;padding:10px}article img{width:240px}h3{font-size:13px;overflow-wrap:anywhere}article p{font-size:14px;color:#bdd0dc}.strip{max-width:100%;margin-bottom:12px}.note{color:#f2c18b}footer{padding:24px}</style><header><h1>RoadDash — context-first action sequences</h1><p class="note">V2 supersedes the speculative V1 action labels. Each row is an action; _01, _02, etc. are ordered temporal descriptors. Camera/lean variants and mirrored sprite reuse are separate metadata. All recreated art is DRAFT.</p><p>104 action slots across rider/police banks; 194 temporal descriptors. This review shows 138 human + 16 bike-only rider recreations. Code43 has empty original art. Police metadata is recorded; no approved police model is claimed. This static gallery now includes torso/support corrections; runtime playback is documented separately in docs/testing/ANIMATION_B.md.</p><label>Find action <input id="find" type="search" placeholder="punch, kick, chain, run..."></label><label>Category <select id="category"><option value="all">All</option><option>combat</option><option>riding</option><option>crash</option><option>recovery</option><option>wreck</option></select></label><span id="count" role="status"></span></header><main>'''+''.join(sections)+'''</main><footer>Timings are reference logical counters, not a Bevy clip rate. Combat flags and post-advance hit gating are distinct from a guaranteed visual contact instant. Source reconstruction was cross-checked; native original execution has not been rerun. Accepted passive ragdoll physics remain unchanged.</footer><script>const find=document.querySelector('#find'),category=document.querySelector('#category'),rows=[...document.querySelectorAll('section')];function filter(){let shown=0;for(const row of rows){row.hidden=!(row.dataset.name.includes(find.value.toLowerCase())&&(category.value==='all'||row.dataset.category===category.value));if(!row.hidden)shown++;}document.querySelector('#count').textContent=shown+(shown===1?' action shown':' actions shown');}find.addEventListener('input',filter);category.addEventListener('change',filter);filter();</script></html>'''
    (out/'index.html').write_text(page);(out/'pose-index.json').write_text(json.dumps(index,indent=2)+'\n');print('Context gallery',len(index),'phase entries; original media',bool(retail))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path,default=BASE/'gallery');p.add_argument('--retail-root',type=Path);args=p.parse_args();build(args.output,args.retail_root)
