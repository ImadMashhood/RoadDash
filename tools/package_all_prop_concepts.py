#!/usr/bin/env python3
"""Build a source-ID searchable gallery and portable generated-concept archive."""
import csv
import hashlib
import html
import json
from pathlib import Path
import shutil
import zipfile
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'art/props/concept_3d'

def package():
    remaining = json.loads((OUT/'ALL_REMAINING.json').read_text())
    first = json.loads((OUT/'MANIFEST.json').read_text())
    first['batch'] = 'props_01'
    sheets = [first] + json.loads((OUT/'BATCH_02.json').read_text())
    sheets += [s for s in remaining['sheets'] if s['status'] == 'draft_not_approved']
    coverage = {c['id'] for s in sheets for c in s['cells']}
    expected = set(remaining['remaining_ids']) | set(remaining['already_covered_ids'])
    cards = []
    for s in sheets:
        file = Path(s['file']).name
        names = ', '.join(c.get('label', c.get('name', c['id'])) for c in s['cells'])
        mapping = ' · '.join(str(c['slot'])+': '+c['id'] for c in s['cells'])
        cards.append('<article data-search="'+html.escape(s['batch']+' '+names+' '+mapping, quote=True)+'"><h2>'+html.escape(s['batch'])+'</h2><a href="'+file+'"><img loading="lazy" src="'+file+'"></a><p>'+html.escape(names)+'</p><small>'+html.escape(mapping)+'</small></article>')
    page = '''<!doctype html><meta charset="utf-8"><title>RoadDash · all prop concepts</title><style>body{background:#15191f;color:#eee;font:16px system-ui;margin:24px}header{position:sticky;top:0;background:#15191f;padding:12px;z-index:1}input{padding:12px;width:min(600px,90%)}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(440px,1fr));gap:24px}article{background:#242a32;padding:16px}article[hidden]{display:none}img{width:100%}h2{font-size:18px}p,small{overflow-wrap:anywhere}a{color:#9dd}</style><header><h1>RoadDash · all prop concepts</h1><p>COVERAGE source groups · SHEETS sheets. Generated bitmap drafts, not 3D meshes or approved production art. Source banks may be alternate views/colors of the same object.</p><input id="search" placeholder="Search category, source name or PROP ID"><b id="count"></b></header><main>CARDS</main><script>const q=document.querySelector('#search');function filter(){let n=0;for(const a of document.querySelectorAll('article')){a.hidden=!a.dataset.search.toLowerCase().includes(q.value.toLowerCase());n+=!a.hidden}document.querySelector('#count').textContent=' '+n+' sheets'}q.oninput=filter;filter();</script>'''.replace('COVERAGE',str(len(coverage))).replace('SHEETS',str(len(sheets))).replace('CARDS','\n'.join(cards))
    (OUT/'ALL_CONCEPTS.html').write_text(page)
    assets = ROOT/'docs/third_party/ASSETS.csv'
    with assets.open(newline='') as f:
        reader=csv.DictReader(f); fields=reader.fieldnames; rows=list(reader)
    known={r['path'] for r in rows}
    for s in remaining['sheets']:
        if s['status'] != 'draft_not_approved' or s['file'] in known: continue
        rows.append(dict(zip(fields,[
            'prop-concepts-'+Path(s['file']).stem,s['file'],'RoadDash art direction / OpenAI image generation',
            'project-generated:art/props/concept_3d/ALL_REMAINING.json','UNVERIFIED-reference-derived-visual-design',
            'docs/DECISIONS.md',s['sha256'],'Source-mapped bitmap concept; no mesh or runtime asset',
            'Source IDs; exact prompt and style/source hashes in ALL_REMAINING.json',
            'local-development-only; art/release review pending','false'])))
    with assets.open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=fields);w.writeheader();w.writerows(rows)
    local=Path.home()/'Downloads/RoadDash_Prop_References/concepts';local.mkdir(exist_ok=True)
    files={Path(s['file']).name for s in sheets}|{'ALL_CONCEPTS.html','ALL_REMAINING.json','MANIFEST.json','BATCH_02.json','PROMPTS_BATCH_02.json'}
    for file in files: shutil.copy2(OUT/file,local/file)
    archive=Path.home()/'Downloads/RoadDash_All_Prop_Concepts.zip'
    with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
        for file in sorted(files):z.write(OUT/file,file)
    with zipfile.ZipFile(archive) as z:
        assert z.testzip() is None
    receipt=dict(covered_ids=len(coverage),expected_ids=len(expected),missing_ids=sorted(expected-coverage),
                 new_sheets=sum(s['status']=='draft_not_approved' for s in remaining['sheets']),
                 total_sheets=len(sheets),archive=str(archive),archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                 output_visually_reviewed=sum(s.get('output_reviewed',False) for s in remaining['sheets']))
    (ROOT/'docs/testing/prop-catalog/all-concepts.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt))
if __name__=='__main__':package()
