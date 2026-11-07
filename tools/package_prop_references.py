#!/usr/bin/env python3
"""Make labeled 3x2 local reference sheets + searchable offline review gallery."""
import argparse
from collections import Counter
import hashlib
import html
import json
from pathlib import Path
import zipfile
from PIL import Image, ImageDraw, ImageFont

FONT = '/usr/share/fonts/google-noto/NotoSans-Regular.ttf'


def fit(image, size):
    scale = min(size[0] / image.width, size[1] / image.height)
    return image.resize((max(1, round(image.width * scale)), max(1, round(image.height * scale))), Image.Resampling.NEAREST)


def package(folder):
    data = json.loads((folder / 'catalog.json').read_text())
    concept_path = folder / 'concepts/MANIFEST.json'
    if concept_path.exists():
        manifest = json.loads(concept_path.read_text())
        concepts = {c['id']: c for c in manifest['cells']}
        for record in data['records']:
            if record['id'] in concepts:
                record['concept_status'] = 'draft_not_approved'
                record['concept'] = dict(file='art/props/concept_3d/' + manifest['file'], slot=concepts[record['id']]['slot'])
    batch_path = folder / 'concepts/BATCH_02.json'
    batches = json.loads(batch_path.read_text()) if batch_path.exists() else []
    remaining_path = folder / 'concepts/ALL_REMAINING.json'
    if remaining_path.exists():
        batches.extend(dict(s, file=Path(s.get('file', '')).name) for s in json.loads(remaining_path.read_text())['sheets'] if s['status'] == 'draft_not_approved')
    for batch in batches:
        if batch['status'] == 'draft_not_approved':
            concept_ids = {c['id']: c for c in batch['cells']}
            for record in data['records']:
                if record['id'] in concept_ids:
                    record['concept_status'] = 'draft_not_approved'
                    record['concept'] = dict(file='art/props/concept_3d/' + batch['file'], slot=concept_ids[record['id']]['slot'])
    sheets = folder / 'sheets'; sheets.mkdir(exist_ok=True)
    thumbs = folder / 'thumbnails'; thumbs.mkdir(exist_ok=True)
    font = ImageFont.truetype(FONT, 20); small = ImageFont.truetype(FONT, 14)
    cards = {}; groups = {}; pages = []
    for number, r in enumerate(data['records'], 1):
        r['review_number'] = number
        group = r['group']; groups.setdefault(group, []).append(r)
        card = Image.new('RGB', (600, 470), (30, 34, 40)); draw = ImageDraw.Draw(card)
        draw.text((18, 12), f"{number:04}  {r['id']}", font=font, fill='white')
        label = ', '.join(r['names'])
        draw.text((18, 43), label[:68] + ('…' if len(label) > 68 else ''), font=small, fill=(130, 220, 218))
        # Three largest distinct images are review views, not inferred action labels.
        selected = r['cells'][:3]
        for index, c in enumerate(selected):
            im = Image.open(folder / 'individual' / (c['pixel_sha256'] + '.png')).convert('RGBA')
            width = 564 // len(selected); tile = fit(im, (width - 12, 310))
            x = 18 + width * index + (width - tile.width) // 2
            card.paste(tile, (x, 80 + (310 - tile.height) // 2), tile)
        draw.text((18, 402), f"{group} | {len(r['cells'])} unique images | {len(r['aliases'])} source aliases", font=small, fill=(210, 215, 225))
        draw.text((18, 427), 'REFERENCE SPRITES — new textures / 3D meshes not yet authored', font=small, fill=(255, 188, 112))
        path = thumbs / (r['id'] + '.png'); card.save(path); cards[r['id']] = card
        r['thumbnail'] = 'thumbnails/' + path.name
    for group, entries in sorted(groups.items()):
        for start in range(0, len(entries), 6):
            sheet = Image.new('RGB', (1800, 940), (18, 21, 25)); chunk = entries[start:start + 6]
            name = f'{group}_{start // 6 + 1:03}.png'
            for slot, record in enumerate(chunk):
                sheet.paste(cards[record['id']], ((slot % 3) * 600, (slot // 3) * 470))
                record['sheet'] = 'sheets/' + name; record['sheet_slot'] = slot + 1
            sheet.save(sheets / name); pages.append(dict(file='sheets/' + name, group=group, ids=[r['id'] for r in chunk]))
    # Overview uses one representative from each category; full scope lives in gallery.
    summary = Image.new('RGB', (1800, ((len(groups) + 2) // 3) * 470), (18, 21, 25))
    for i, (group, entries) in enumerate(sorted(groups.items())):
        tile = cards[entries[0]['id']].copy(); d = ImageDraw.Draw(tile)
        d.rectangle((0, 0, 600, 38), fill=(30, 60, 65)); d.text((14, 8), f'{group}: {len(entries)} entries', font=font, fill='white')
        summary.paste(tile, (i % 3 * 600, i // 3 * 470))
    summary.save(folder / 'overview.png')
    data['sheets'] = pages
    (folder / 'catalog.json').write_text(json.dumps(data, indent=2) + '\n')
    tiles = []
    for r in data['records']:
        views = ''.join(f'<figure><img loading="lazy" src="individual/{c["pixel_sha256"]}.png"><figcaption>{html.escape(c["name"])} · {c["width"]}×{c["height"]}</figcaption></figure>' for c in r['cells'])
        sources = html.escape(json.dumps(r['aliases'], indent=2))
        names = html.escape(', '.join(r['names']))
        tiles.append(f'''<article data-group="{r['group']}" data-search="{html.escape(r['id']+' '+names+' '+r['group'], quote=True)}">
<img class="card" loading="lazy" src="{r['thumbnail']}"><h2>{r['review_number']:04} · {r['id']}</h2>
<p>{names}</p><p>{r['group']} · course family matches: {', '.join(r['course_family_load_matches']) or 'none established'}</p>
<a href="{r['sheet']}">3×2 sheet, slot {r['sheet_slot']}</a>
<details><summary>All {len(r['cells'])} stored images / views</summary><div class="views">{views}</div></details>
<details><summary>Source paths, offsets and frame references</summary><pre>{sources}</pre></details></article>''')
    options = ''.join(f'<option value="{g}">{g} ({len(v)})</option>' for g, v in sorted(groups.items()))
    page = '''<!doctype html><meta charset="utf-8"><title>RoadDash · full prop reference review</title>
<style>body{background:#15191f;color:#eee;font:16px system-ui;margin:24px}header{position:sticky;top:0;background:#15191f;padding:12px;z-index:1}input,select{padding:10px;max-width:100%}main{display:grid;grid-template-columns:repeat(auto-fill,minmax(420px,1fr));gap:20px}article{background:#242a32;padding:12px;overflow:hidden}article[hidden]{display:none}.card{width:100%}h2{font-size:18px}a{color:#89e0db}pre{white-space:pre-wrap;font-size:12px}figure{margin:8px}.views{display:flex;flex-wrap:wrap}.views img{image-rendering:pixelated;width:190px;height:230px;object-fit:contain}p{overflow-wrap:anywhere}summary{padding:12px;cursor:pointer}</style>
<header><h1>RoadDash · world prop reference catalog</h1><p>Local original sprites, not new models. All five courses; source aliases and views retained. Unclassified entries remain visible. Family loading ≠ actual placement.</p>
<input id="search" placeholder="Search ID / original name"><select id="group"><option value="">All groups</option>OPTIONS</select><b id="count"></b></header><main>TILES</main>
<script>const search=document.querySelector('#search'),group=document.querySelector('#group');function filter(){let n=0;for(const a of document.querySelectorAll('article')){a.hidden=Boolean(group.value&&a.dataset.group!==group.value)||!a.dataset.search.toLowerCase().includes(search.value.toLowerCase());n+=!a.hidden}document.querySelector('#count').textContent=' '+n+' entries'}search.oninput=group.onchange=filter;filter();</script>'''.replace('OPTIONS', options).replace('TILES', '\n'.join(tiles))
    if (folder / 'concepts/props_01.png').exists():
        page = page.replace('<input id="search"', '<p><a href="concepts/props_01.png">New six-item concept sheet 01 — draft, not 3D meshes</a></p><input id="search"')
    if batches:
        links = ' · '.join('<a href="concepts/' + b['file'] + '">' + b['batch'] + '</a>' for b in batches if b['status'] == 'draft_not_approved')
        page = page.replace('<input id="search"', '<p>New generated concepts (not meshes): ' + links + '</p><input id="search"')
    (folder / 'index.html').write_text(page)
    (folder / 'README.txt').write_text('Open index.html in a browser. Sheets are 3x2, grouped by provisional category. Individual indexed images are local research references. No new textures, models, animation labels or game integration is claimed. All source aliases/frame tables remain in catalog.json. Do not bundle retail reference images in production.\n')
    archive = folder.parent / (folder.name + '.zip')
    expected = [p for p in folder.rglob('*') if p.is_file()]
    with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED, compresslevel=3) as z:
        for p in sorted(expected):
            z.write(p, p.relative_to(folder))
    with zipfile.ZipFile(archive) as z:
        if z.testzip() is not None or len(z.namelist()) != len(expected):
            raise ValueError('invalid reference archive')
    return dict(assets=data['asset_count'], sheets=len(pages), groups=data['groups'], archive=str(archive),
                sha256=hashlib.sha256(archive.read_bytes()).hexdigest(), files=len(expected))


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('folder', type=Path); args = ap.parse_args()
    print(json.dumps(package(args.folder)))
