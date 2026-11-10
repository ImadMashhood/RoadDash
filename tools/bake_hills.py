"""Original Hills authoring with the approved Metropolis final-kilometre transplant."""
import bisect
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
metro = json.loads((ROOT / 'content/world/metropolis.json').read_text())
path = metro['traffic_paths'][0]
elevation = metro['elevation_profile']


def source(s):
    i = min(len(path['points']) - 2, bisect.bisect_right(path['progress'], s) - 1)
    t = (s - path['progress'][i]) / (path['progress'][i + 1] - path['progress'][i])
    p = [a + (b - a) * t for a, b in zip(path['points'][i], path['points'][i + 1])]
    for (a, lo), (b, hi) in zip(elevation['knots'], elevation['knots'][1:]):
        if a <= p[2] <= b:
            u = (p[2] - a) / (b - a)
            p[1] = lo + (hi - lo) * u**3 * (10 + u * (-15 + 6*u))
            break
    p[1] += elevation['crossfall_amplitude'] * math.sin(p[0] / elevation['crossfall_wavelength']) + .025
    return p


origin = source(10500)
end = source(11500)
previous = source(11499)
end_heading = math.atan2(end[0] - previous[0], end[2] - previous[2])
end_y = end[1] - origin[1] + 8
end_grade = end[1] - previous[1]


def smooth(t):
    t = min(1, max(0, t))
    return t*t*(3-2*t)


def h(s):
    if s <= 1000:
        return source(10500+s)[1] - origin[1] + 8
    if s < 1500:
        t = (s-1000)/500
        return (2*t**3-3*t*t+1)*end_y + (t**3-2*t*t+t)*500*end_grade + (-2*t**3+3*t*t)*8
    if s < 5500:
        t = (s-1500)/4000
        return 8+62*smooth(t)+145*math.sin(math.pi*t)**2+42*math.sin(4*math.pi*t)**2
    if s < 8500:
        return 70
    t = (s-8500)/4000
    return 70+35*smooth(t)+28*math.sin(4*math.pi*t)**2


def mountain_heading(s):
    t = (s-1500)/4000
    return 1.34*math.sin(10*math.pi*t)*math.sin(math.pi*t)**.5


def heading(s):
    if s < 1500:
        return end_heading*(1-smooth((s-1000)/500))
    if s < 5500:
        a = mountain_heading(s)
        # A real suspension crossing stays straight; sharper bends flank it.
        blend = smooth((s-4470)/150)*(1-smooth((s-4860)/150))
        return a*(1-blend)+mountain_heading(4740)*blend
    if s < 8500:
        return 0
    t = (s-8500)/4000
    return .78*math.sin(5*math.pi*t)*math.sin(math.pi*t)


N = 5000
ds = 2.5
pts = []
x = z = 0
for i in range(N+1):
    s = i*ds
    if s <= 1000:
        p = source(10500+s)
        x, z = p[0]-origin[0], p[2]-origin[2]
    else:
        mid = s-ds/2
        dy = (h(s)-h(s-ds))/ds
        step = ds*math.sqrt(max(0, 1-dy*dy))
        a = heading(mid)
        x += math.sin(a)*step
        z += math.cos(a)*step
    pts.append([x, h(s), z])
base = json.loads((ROOT/'content/tracks/valley.json').read_text())
for route in range(2):
    points = []
    for i, p in enumerate(pts):
        s = i*ds
        offset = 0
        if 5550 < s < 7350:
            offset = (1 if route else -1)*65*math.sin(math.pi*(s-5550)/1800)**2
        points.append([round(p[0]+offset, 5), round(p[1], 5), round(p[2], 5)])
    doc = {k: base[k] for k in ['schema_version', 'closed', 'spawns']}
    doc.update(id='hills-01' if route == 0 else 'hills-east-01', name='Hills', scenery_seed=7319, verge_width=3, props=[])
    doc['control_points'] = [dict(position=points[i], half_width=6+.5*smooth((i*ds-1000)/500), bank_degrees=0,
        sector='city' if i*ds < 1500 or 5500 <= i*ds < 8500 else 'hills', surface='asphalt') for i in range(0, N+1, 20)]
    doc['checkpoints'] = [dict(id=f'hills-gate-{i:02}', fraction=s/12500) for i, s in enumerate([0,1500,2750,4000,5500,6200,7000,7800,8500,10000,11250,12500])]
    (ROOT/f'content/tracks/hills{"-east" if route else ""}.json').write_text(json.dumps(doc, indent=2)+'\n')
