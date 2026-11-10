"""Compare the elevation-only bake to its immutable integration baseline."""
import copy
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools'))
from metropolis_elevation import height

BASE = 'b3f14b48'
def baseline(path):
    return json.loads(subprocess.check_output(['git', 'show', f'{BASE}:{path}'], cwd=ROOT))
def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()
def planar(manifest):
    m = copy.deepcopy(manifest)
    del m['elevation_profile'], m['source_layout_sha256']
    for path in m['paths'] + m['traffic_paths']:
        for p in path['points']: p[1] = 0
    for cp in m['track']['control_points']: cp['position'][1] = 0
    return m

def metrics(m):
    profile = m['elevation_profile']
    knots = profile['knots']
    report = dict(knot_min_m=min(y for _,y in knots), knot_max_m=max(y for _,y in knots),
                  analytic_max_longitudinal_grade_percent=max(187.5*abs(b[1]-a[1])/(b[0]-a[0]) for a,b in zip(knots,knots[1:])),
                  analytic_max_longitudinal_curvature_per_m=max(10/math.sqrt(3)*abs(b[1]-a[1])/(b[0]-a[0])**2 for a,b in zip(knots,knots[1:])),
                  crossfall_amplitude_m=profile['crossfall_amplitude'], routes=[])
    for path in m['paths']:
        p=path['points']
        window=[v for v,s in zip(p,path['progress']) if 500<=s<=11500]
        grades=[abs(b[1]-a[1])/math.hypot(b[0]-a[0],b[2]-a[2])*100 for a,b in zip(p,p[1:])]
        report['routes'].append(dict(min_m=min(v[1] for v in p),max_m=max(v[1] for v in p),
                                    playable_sample_min_m=min(v[1] for v in window),playable_sample_max_m=max(v[1] for v in window),
                                    max_sampled_grade_percent=max(grades),samples=len(p),progress_endpoints=[path['progress'][0],path['progress'][-1]]))
    report['support']={}
    for name, shapes in [('pavement',[s['shape'] for s in m['surfaces']]),
                         ('fields',[s for f in m['fields'] for s in f['patches']])]:
        worst=0.;samples=0
        for shape in shapes:
            vertices=[p for ring in shape['rings'] for p in ring]
            for triangle in shape['triangles']:
                ps=[vertices[i] for i in triangle]
                for weights in [(1/3,1/3,1/3),(.5,.5,0),(.5,0,.5),(0,.5,.5)]:
                    x,z=[sum(w*p[k] for w,p in zip(weights,ps)) for k in [0,1]]
                    y=sum(w*height(*p,profile) for w,p in zip(weights,ps))
                    worst=max(worst,abs(y-height(x,z,profile)));samples+=1
        report['support'][name]=dict(samples=samples,max_sampled_interpolation_error_m=worst)
    return report

old=baseline('content/world/metropolis.json')
new=json.loads((ROOT/'content/world/metropolis.json').read_text())
assert planar(old)==planar(new)
assert new['source_layout_sha256']==hashlib.sha256((ROOT/'docs/planning/farms-long-city/LAYOUT.json').read_bytes()).hexdigest()
report=dict(baseline_commit=BASE, before=metrics(old),after=metrics(new),
            preservation=dict(exact_planar_manifest=True,normalized_sha256=digest(planar(new)),
                              source_layout_sha256=new['source_layout_sha256'],
                              counts={k:len(new[k]) for k in ['paths','traffic_paths','buildings','fields','surfaces','civic_sites','parked_cars','pedestrians','street_props']}))
(ROOT/'docs/testing/next-elevation/measurements.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
