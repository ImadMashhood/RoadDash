#!/usr/bin/env python3
"""Independently verify complete concept coverage, bytes and portable gallery."""
import hashlib
import json
from pathlib import Path
import zipfile
from PIL import Image
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/props/concept_3d'

def verify():
    data=json.loads((OUT/'ALL_REMAINING.json').read_text())
    queue=json.loads((ROOT/'docs/research/props/rebuild-queue.json').read_text())
    ids=set(data['already_covered_ids']);seen=set()
    for s in data['sheets']:
        assert s['status']=='draft_not_approved', s['batch']
        assert s['source_reviewed'] and s['output_reviewed'], s['batch']
        assert [c['slot'] for c in s['cells']]==list(range(1,len(s['cells'])+1))
        assert 1<=len(s['cells'])<=6
        path=ROOT/s['file']
        assert hashlib.sha256(path.read_bytes()).hexdigest()==s['sha256']
        with Image.open(path) as im:
            assert im.width>im.height and im.width>=1000
            im.verify()
        for c in s['cells']:
            assert c['id'] not in seen and c['id'] not in ids
            seen.add(c['id'])
    assert seen==set(data['remaining_ids'])
    assert ids|seen=={j['id'] for j in queue['jobs']}
    for j in queue['jobs']:
        assert j['concept_status']=='draft_not_approved'
        assert not j['approved'] and j['model_status'] in {'not_started','draft_volumetric_mesh_not_approved'}
        assert (ROOT/j['concept']['file']).is_file()
    receipt=json.loads((ROOT/'docs/testing/prop-catalog/all-concepts.json').read_text())
    assert not receipt['missing_ids'] and receipt['covered_ids']==len(ids|seen)
    page=(OUT/'ALL_CONCEPTS.html').read_text()
    from html.parser import HTMLParser
    class Links(HTMLParser):
        def handle_starttag(self, tag, attrs):
            for key,value in attrs:
                if key in {'href','src'}:
                    assert (OUT/value).is_file(), value
    Links().feed(page)
    assert page.count('<article ')==receipt['total_sheets']
    archive=Path(receipt['archive'])
    assert hashlib.sha256(archive.read_bytes()).hexdigest()==receipt['archive_sha256']
    with zipfile.ZipFile(archive) as z:
        assert z.testzip() is None
        assert 'ALL_CONCEPTS.html' in z.namelist()
        for s in data['sheets']:
            file=Path(s['file']).name
            assert hashlib.sha256(z.read(file)).hexdigest()==s['sha256']
    return dict(covered_ids=len(ids|seen),new_sheets=len(data['sheets']),missing=0,
                source_reviewed=True,output_reviewed=True,all_png_hashes_and_zip_crc_valid=True)
if __name__=='__main__':print(json.dumps(verify()))
