#!/usr/bin/env python3
"""Package actual geometry, editable Blender sources and exported-mesh proof renders."""
import csv
import hashlib
import html
import json
from pathlib import Path
import zipfile
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/props/mesh_assets'

def package():
 m=json.loads((OUT/'MANIFEST.json').read_text());cards=[]
 by_id={r['id']:r for r in m['records']}
 for sheet in m['renders']:
  rows=[]
  for c in sheet['cells']:
   r=by_id[c['id']];links=' · '.join(f'<a href="{l["file"]}">LOD{l["lod"]} ({l["triangles"]:,} tris)</a>'for l in r['lods'])
   rows.append(f'<p><b>{c["slot"]:02} {html.escape(r["name"])}</b> · {r["id"]}<br>{links}</p>')
  search=' '.join(c['id']+' '+c['name']+' '+by_id[c['id']]['group']for c in sheet['cells'])
  cards.append(f'<article data-search="{html.escape(search,quote=True)}"><a href="{sheet["file"]}"><img loading="lazy" src="{sheet["file"]}"></a>'+''.join(rows)+'</article>')
 page='''<!doctype html><meta charset="utf-8"><title>RoadDash · actual 3D prop meshes</title><style>body{background:#171d23;color:#eee;font:16px system-ui;margin:24px}header{position:sticky;top:0;background:#171d23;padding:12px;z-index:1}input{padding:12px;width:min(600px,90%)}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(480px,1fr));gap:24px}article{background:#242c34;padding:16px}article[hidden]{display:none}img{width:100%}p{overflow-wrap:anywhere}a{color:#9dd}</style><header><h1>Actual 3D prop meshes · draft catalog</h1><p>369 source groups · 1,107 GLBs · 13 editable Blender scenes. These images are renders of re-imported GLBs, not AI concept renders. Art fidelity and character skinning remain unfinished; no gameplay replacement.</p><p>Inspect in 3D: <code>cargo run --locked --bin prop_preview</code> · arrows select, 1/2/3 LOD, Q/E orbit, +/- zoom.</p><input id="search" placeholder="Search source name, PROP ID or category"><b id="count"></b></header><main>CARDS</main><script>const q=document.querySelector('#search');function filter(){let n=0;for(const a of document.querySelectorAll('article')){a.hidden=!a.dataset.search.toLowerCase().includes(q.value.toLowerCase());n+=!a.hidden}document.querySelector('#count').textContent=' '+n+' sheets'}q.oninput=filter;filter();</script>'''.replace('CARDS','\n'.join(cards))
 (OUT/'index.html').write_text(page)
 assets=ROOT/'docs/third_party/ASSETS.csv'
 with assets.open(newline='')as f:reader=csv.DictReader(f);fields=reader.fieldnames;rows=list(reader)
 known={r['path']:r for r in rows}
 deliverables=[OUT/l['file']for r in m['records']for l in r['lods']]+[OUT/s['file']for s in m['sources']]+[OUT/s['file']for s in m['renders']+m.get('clay_renders',[])]
 for p in deliverables:
  relative=p.relative_to(ROOT).as_posix()
  row=dict(zip(fields,['prop-mesh-'+p.relative_to(OUT).as_posix().replace('/','-'),relative,'RoadDash original procedural authoring',
   'project-generated:tools/build_prop_meshes.py','UNVERIFIED-reference-derived-visual-design','docs/DECISIONS.md',hashlib.sha256(p.read_bytes()).hexdigest(),
   'Original volumetric geometry and small authored material tiles; source concepts used for design reference only','ID/concept mapping in art/props/mesh_assets/MANIFEST.json','local-development-only; production art/release review pending','false']))
  if relative in known:known[relative].update(row)
  else:rows.append(row)
 with assets.open('w',newline='')as f:w=csv.DictWriter(f,fieldnames=fields);w.writeheader();w.writerows(rows)
 archive=Path.home()/'Downloads/RoadDash_3D_Prop_Drafts.zip'
 files=deliverables+[OUT/'index.html',OUT/'MANIFEST.json',OUT/'designs.json',OUT/'README.md']
 with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=3)as z:
  for p in sorted(set(files)):z.write(p,p.relative_to(OUT))
 with zipfile.ZipFile(archive)as z:assert z.testzip()is None
 receipt=dict(models=len(m['records']),glbs=len(m['records'])*3,render_sheets=len(m['renders']),
  archive=str(archive),bytes=archive.stat().st_size,sha256=hashlib.sha256(archive.read_bytes()).hexdigest())
 (ROOT/'docs/testing/prop-meshes/package.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt))
if __name__=='__main__':package()
