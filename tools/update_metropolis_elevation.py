#!/usr/bin/env python3
"""Reauthor only the elevation layer of the approved plan; never regenerate lots."""
import json
from pathlib import Path
from metropolis_elevation import PROFILE, height
from plan_farms_long_city import length, profile
ROOT=Path(__file__).resolve().parents[1]
def run(render_profile=True):
    source=ROOT/'docs/planning/farms-long-city/LAYOUT.json'
    d=json.loads(source.read_text())
    for p in d['nodes'].values():p[1]=height(p[0],p[2])
    grades=[];heights=[]
    for edge in d['edges']:
        if 'progress_fractions' not in edge:
            distances=[0.]
            for a,b in zip(edge['points'],edge['points'][1:]):
                distances.append(distances[-1]+length([a,b]))
            edge['progress_fractions']=[v/distances[-1] for v in distances]
        for p in edge['points']:p[1]=height(p[0],p[2]);heights.append(p[1])
        edge['measured_3d_m']=length(edge['points'])
        for a,b in zip(edge['points'],edge['points'][1:]):
            grades.append(abs(b[1]-a[1])/length([[a[0],a[2]],[b[0],b[2]]]))
    assert max(grades)<.04
    d['elevation_profile']=PROFILE
    d['elevation_note']='Authored C2 land/street profile shared with baked Metropolis; no separate hills district.'
    d['validation']['proposed_elevation_range_m']=[min(heights),max(heights)]
    d['validation']['max_sampled_grade_percent']=max(grades)*100
    source.write_text(json.dumps(d,indent=2)+'\n')
    if render_profile:
        profile(None,d,source.parent)
if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--no-chart', action='store_true')
    args=parser.parse_args()
    run(render_profile=not args.no_chart)
