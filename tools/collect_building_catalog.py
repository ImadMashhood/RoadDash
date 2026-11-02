#!/usr/bin/env python3
"""Reconcile exact reference coverage with actual mesh/source/render deliverables."""
import argparse
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/buildings/mesh_assets'


def collect(partial=False):
    plan=json.loads((OUT/'catalog_designs.json').read_text());expected={d['id']:d for d in plan['designs']}
    first=json.loads((OUT/'MANIFEST.json').read_text());by_id={}
    for record in first['records']:
        design=expected[record['id']];r=dict(record,asset_type='facade',builder='accepted_first_sheet',reference_sheet='facades_01.png',
            body_dimensions_m=[design['width'],design['depth'],design['height']],source_blend='RoadDash_Building_Models.blend',
            render='renders/'+record['id']+'.png')
        by_id[r['id']]=r
    for path in sorted((OUT/'catalog_manifests').glob('*.json')):
        batch=json.loads(path.read_text());blend=OUT/batch['source_blend']
        if hashlib.sha256(blend.read_bytes()).hexdigest()!=batch['source_sha256']:raise ValueError('Edited source blend after export: '+str(blend))
        for record in batch['records']:
            if record['id'] in by_id:raise ValueError('Duplicate catalog ID '+record['id'])
            if record['id'] not in expected:raise ValueError('Unknown catalog ID '+record['id'])
            by_id[record['id']]=dict(record,source_blend=batch['source_blend'],render='catalog_renders/'+record['id']+'.png')
    missing=sorted(set(expected)-set(by_id))
    if missing and not partial:raise ValueError('Missing models: '+','.join(missing))
    for identity,r in by_id.items():
        if r['asset_type']!=expected[identity]['asset_type']:raise ValueError('Changed asset classification')
        if len(r['lods'])!=3:raise ValueError('Missing LOD')
        for lod in r['lods']:
            path=OUT/'models'/lod['file']
            if hashlib.sha256(path.read_bytes()).hexdigest()!=lod['sha256']:raise ValueError('GLB hash mismatch '+str(path))
        if not(OUT/r['render']).is_file():raise ValueError('Missing actual render '+identity)
    counts={k:sum(r['asset_type']==k for r in by_id.values())for k in ['facade','structure','surface_module']}
    result=dict(schema_version=2,expected_ids=146,exported_ids=len(by_id),missing_ids=missing,asset_counts=counts,
        glb_files=sum(len(r['lods'])for r in by_id.values()),source_sheets=25,
        records=[by_id[d['id']]for d in plan['designs']if d['id'] in by_id],
        scope='Actual modeled draft coverage; individual art/fidelity approval, final material polish and game integration remain separate gates',
        first_six_preserved=True,reference_metadata='../concept_3d/MANIFEST.json')
    (OUT/'CATALOG_MANIFEST.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--partial',action='store_true');a=ap.parse_args();r=collect(a.partial)
    print(json.dumps({k:r[k]for k in ['expected_ids','exported_ids','asset_counts','glb_files','missing_ids']}))
