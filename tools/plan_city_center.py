#!/usr/bin/env python3
"""Draw a 2D city-center / two-fork race proposal. No game content is written."""
import argparse
import hashlib
import html
import json
import math
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image, ImageDraw
from plan_dense_city import fonts

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'docs/planning/city-center'
BASELINE = '58eeabb'
ROAD_WIDTH, SIDEWALK, SETBACK = 8.0, 2.0, 0.2
CLEAR = ROAD_WIDTH / 2 + SIDEWALK + SETBACK
COLOURS = {'retail': '#cba77c', 'residential': '#b8b4a1',
           'workshops': '#879ba2', 'civic': '#91b8b0'}


def distance_point_segment(p, a, b):
    delta = [b[k] - a[k] for k in [0, 1]]
    d2 = sum(v * v for v in delta)
    t = max(0.0, min(1.0, sum((p[k] - a[k]) * delta[k] for k in [0, 1]) / d2))
    return math.dist(p, [a[k] + t * delta[k] for k in [0, 1]])


def rect_points(rect):
    x0, z0, x1, z1 = rect
    return [[x0, z0], [x1, z0], [x1, z1], [x0, z1]]


def segment_near_rect(a, b, rect, clearance):
    # Slab intersection against the actual rectangle, then Euclidean edge distance.
    low, high = 0.0, 1.0
    for k in [0, 1]:
        delta = b[k] - a[k]
        if abs(delta) < 1e-10:
            if not rect[k] <= a[k] <= rect[k + 2]:
                low, high = 1.0, 0.0
                break
        else:
            t0, t1 = sorted(((rect[k] - a[k]) / delta, (rect[k + 2] - a[k]) / delta))
            low, high = max(low, t0), min(high, t1)
    if low <= high:
        return True
    corners = rect_points(rect)
    distances = [distance_point_segment(p, a, b) for p in corners]
    for p in [a, b]:
        nearest = [max(rect[k], min(rect[k + 2], p[k])) for k in [0, 1]]
        distances.append(math.dist(p, nearest))
    return min(distances) < clearance - 1e-6


def race_graph():
    nodes = {'entry': [0, -960], 'split_1': [0, -600],
             'center_merge': [0, 0], 'split_2': [0, 300],
             'merge_2': [0, 660], 'exit': [0, 960]}
    specs = [
        ('approach', 'entry', 'split_1', [], 'common'),
        ('market_street', 'split_1', 'center_merge', [[-240, -480], [-240, -240]], 'left'),
        ('station_street', 'split_1', 'center_merge', [[240, -480], [240, -240]], 'right'),
        ('central_avenue', 'center_merge', 'split_2', [], 'common'),
        ('old_town', 'split_2', 'merge_2', [[-240, 420], [-240, 540]], 'left'),
        ('civic_avenue', 'split_2', 'merge_2', [[240, 420], [240, 540]], 'right'),
        ('departure', 'merge_2', 'exit', [], 'common'),
    ]
    edges = []
    for ident, source, target, via, kind in specs:
        points = [nodes[source], *via, nodes[target]]
        edges.append(dict(id=ident, source=source, target=target, points=points,
                          kind=kind, width_m=ROAD_WIDTH,
                          length_m=sum(math.dist(a, b) for a, b in zip(points, points[1:]))))
    return nodes, edges


def district(x, z):
    if abs(x) < 270 and abs(z) < 260:
        return 'retail'
    if x > 250 and z < -120:
        return 'workshops'
    if x > 100 and z > 120:
        return 'civic'
    return 'residential'


def design():
    nodes, race = race_graph()
    race_segments = [(a, b) for e in race for a, b in zip(e['points'], e['points'][1:])]
    blocks, plots, streets = [], [], {}
    for ix in range(-5, 5):
        for iz in range(-6, 6):
            x0, z0 = ix * 120.0, iz * 120.0
            cx, cz = x0 + 60.0, z0 + 60.0
            if (cx / 660.0) ** 2 + (cz / 840.0) ** 2 > 1.0:
                continue
            ident = f'CENTER-B{ix + 5:02}-{iz + 6:02}'
            kind = district(cx, cz)
            site = [x0 + CLEAR, z0 + CLEAR, x0 + 120 - CLEAR, z0 + 120 - CLEAR]
            role = 'perimeter_block'
            if (ix, iz) == (0, 0):
                role = 'central_square'
            elif (ix, iz) == (1, 0):
                role = 'city_hall'
            elif (ix, iz) == (-3, 2):
                role = 'neighborhood_park'
            elif (ix, iz) == (-1, -2):
                role = 'market_square'
            blocks.append(dict(id=ident, site=site, district=kind, role=role))
            corners = [[x0, z0], [x0 + 120, z0], [x0 + 120, z0 + 120], [x0, z0 + 120]]
            for a, b in zip(corners, corners[1:] + corners[:1]):
                key = tuple(sorted((tuple(a), tuple(b))))
                # No straight-through race shortcut on the central spine inside forks.
                pedestrian = a[0] == b[0] == 0 and (
                    (-600 < max(a[1], b[1]) and min(a[1], b[1]) < 0) or (300 < max(a[1], b[1]) and min(a[1], b[1]) < 660))
                streets[key] = dict(a=a, b=b, role='pedestrian_link' if pedestrian else 'street', width_m=8.0)
            if role != 'perimeter_block':
                continue
            a, b, c, d = site
            strips = [[a, b, c, b + 18], [a, d - 18, c, d],
                      [a, b + 18, a + 18, d - 18], [c - 18, b + 18, c, d - 18]]
            for side, strip in enumerate(strips):
                axis = 0 if side < 2 else 1
                count = max(1, round((strip[axis + 2] - strip[axis]) / 12))
                for bay in range(count):
                    rect = strip.copy()
                    rect[axis] = strip[axis] + (strip[axis + 2] - strip[axis]) * bay / count
                    rect[axis + 2] = strip[axis] + (strip[axis + 2] - strip[axis]) * (bay + 1) / count
                    if any(segment_near_rect(p, q, rect, CLEAR) for p, q in race_segments):
                        continue
                    plots.append(dict(id=f'{ident}-S{side}-P{bay + 1:02}', block_id=ident,
                                      district=kind, footprint=rect,
                                      owner_id=f'PROPOSED-{ident}-S{side}-P{bay + 1:02}'))
    split_sections = [dict(id='choice_1', split='split_1', merge='center_merge',
                           alternatives=['market_street', 'station_street']),
                      dict(id='choice_2', split='split_2', merge='merge_2',
                           alternatives=['old_town', 'civic_avenue'])]
    return dict(
        status='2D city/route proposal only; not playable and no 3D changes',
        coordinate_system='local planning metres, north up; not a terrain-aligned runtime bake',
        design_focus='compact city with a civic center, connected neighborhoods and two racing choices',
        dimensions_m=dict(carriageway=ROAD_WIDTH, sidewalk=SIDEWALK, setback=SETBACK),
        nodes=nodes, race_edges=race, split_sections=split_sections,
        streets=list(streets.values()), blocks=blocks, proposed_plots=plots,
        route_contract=dict(
            legal_paths=4,
            common_gates=['entry', 'center_merge', 'merge_2', 'exit'],
            branch_gates='Require an ordered midpoint gate on the selected branch; no straight-through shortcut credit.',
            progress='Project arc length onto the active edge, then map it into a shared split-to-merge progress interval.',
            merges='Reserve the merge corner and keep both approaches free of plots; lane smoothing, visibility and traffic priority remain implementation work.',
            nonrace_streets='Connected city/traffic network, not additional race shortcuts. Closure/signage design and branch validation are future runtime work.',
        ),
        limits=['Race integration, corner radii and merge traffic are not implemented.',
                'City massing is a plan; proposed plots do not replace the canonical lot IDs/quotas.',
                'Current models, heights, textures, physics and playable race remain unchanged.',
                'Diagonal-cut blocks need dedicated corner frontage design before construction.'],
    )


def validate(plan):
    nodes = plan['nodes']
    edges = {e['id']: e for e in plan['race_edges']}
    assert len(edges) == len(plan['race_edges'])
    for edge in edges.values():
        assert edge['points'][0] == nodes[edge['source']]
        assert edge['points'][-1] == nodes[edge['target']]
        measured = sum(math.dist(a, b) for a, b in zip(edge['points'], edge['points'][1:]))
        assert abs(edge['length_m'] - measured) < 1e-7 and measured > 0
    paths = []
    def visit(node, path, seen):
        assert node not in seen, 'cycle in race graph'
        if node == 'exit':
            paths.append(path)
            return
        outgoing = [e for e in edges.values() if e['source'] == node]
        assert outgoing, f'dead end at {node}'
        for edge in outgoing:
            visit(edge['target'], path + [edge['id']], seen | {node})
    visit('entry', [], set())
    assert len(paths) == 4
    city_links = {}
    for street in plan['streets']:
        if street['role'] != 'street':
            continue
        a, b = tuple(street['a']), tuple(street['b'])
        city_links.setdefault(a, set()).add(b)
        city_links.setdefault(b, set()).add(a)
    frontier, reached = [next(iter(city_links))], set()
    while frontier:
        node = frontier.pop()
        if node in reached:
            continue
        reached.add(node)
        frontier.extend(city_links[node] - reached)
    assert reached == set(city_links), 'disconnected city street network'
    gaps = []
    for section in plan['split_sections']:
        pair = [edges[e] for e in section['alternatives']]
        assert all(e['source'] == section['split'] and e['target'] == section['merge'] for e in pair)
        lengths = [e['length_m'] for e in pair]
        difference = abs(lengths[0] - lengths[1]) / max(lengths)
        assert difference < 0.02
        gaps.append(difference)
    segments = [(a, b) for e in edges.values() for a, b in zip(e['points'], e['points'][1:])]
    assert len({p['id'] for p in plan['proposed_plots']}) == len(plan['proposed_plots'])
    for plot in plan['proposed_plots']:
        assert not any(segment_near_rect(a, b, plot['footprint'], CLEAR) for a, b in segments)
    return dict(legal_paths=paths, branch_length_difference_fraction=gaps,
                path_lengths_m=[sum(edges[e]['length_m'] for e in path) for path in paths],
                connected_entry_exit=True, connected_city_street_network=True, cycles=False, plot_race_clearance_m=CLEAR,
                blocks=len(plan['blocks']), proposed_plots=len(plan['proposed_plots']))


def draw(plan, folder, route_only=False, close=False):
    w, h = (1700, 1250) if close else (1600, 1960)
    bounds = [-420, -360, 420, 360] if close else [-760, -1070, 760, 1070]
    scale = min((w - 100) / (bounds[2] - bounds[0]), (h - 300) / (bounds[3] - bounds[1]))
    left = (w - (bounds[2] - bounds[0]) * scale) / 2
    def xy(p):
        return [left + (p[0] - bounds[0]) * scale, 180 + (bounds[3] - p[1]) * scale]
    im = Image.new('RGB', (w, h), '#14242c')
    d = ImageDraw.Draw(im)
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}"><rect width="100%" height="100%" fill="#14242c"/><defs><clipPath id="map"><rect x="35" y="165" width="{w-70}" height="{h-280}"/></clipPath></defs><g clip-path="url(#map)">']
    layer = Image.new('RGB', (w, h), '#14242c')
    ld = ImageDraw.Draw(layer)
    def rect(rectangle, color, label='', outline=None):
        a = xy([rectangle[0], rectangle[3]])
        b = xy([rectangle[2], rectangle[1]])
        ld.rectangle((*a, *b), fill=color, outline=outline)
        svg.append(f'<rect x="{a[0]:.2f}" y="{a[1]:.2f}" width="{b[0]-a[0]:.2f}" height="{b[1]-a[1]:.2f}" fill="{color}" stroke="{outline or color}"><title>{html.escape(label)}</title></rect>')
    def line(points, color, width):
        pix = [xy(p) for p in points]
        ld.line(pix, fill=color, width=max(1, round(width * scale)), joint='curve')
        svg.append('<polyline points="'+' '.join(f'{x:.2f},{y:.2f}' for x, y in pix)+f'" fill="none" stroke="{color}" stroke-width="{width*scale:.2f}" stroke-linejoin="round"/>')
    for b in plan['blocks']:
        colour = '#2a3d38' if b['role'] == 'perimeter_block' else '#56796b'
        rect(b['site'], colour, b['id'] + ' | ' + b['role'])
    for street in plan['streets']:
        line([street['a'], street['b']], '#536a68' if street['role']=='pedestrian_link' else '#3f515c', street['width_m'])
    if not route_only:
        for p in plan['proposed_plots']:
            rect(p['footprint'], COLOURS[p['district']], p['id'], '#283b3f')
    for b in plan['blocks']:
        if b['role']=='city_hall':
            a,bz,c,dz=b['site'];rect([a+16,bz+15,c-16,dz-15], '#a4c5c0', 'City hall footprint concept')
    # Keep race carriageway readable over the full grid.
    for e in plan['race_edges']:
        line(e['points'], '#9eada5', 12)
        line(e['points'], '#374954', 8)
        line(e['points'], {'common':'#f1e6c9','left':'#f0b45d','right':'#63cfdb'}[e['kind']], 3.8)
    labels=[('entry','CITY ENTRY'),('split_1','S1  SPLIT'),('center_merge','R1  CITY CENTER'),
            ('split_2','S2  SPLIT'),('merge_2','R2  REJOIN'),('exit','CITY EXIT')]
    for ident, label in labels:
        p=xy(plan['nodes'][ident]);radius=7 if close else 6
        color='#a3e7b0' if ident.startswith(('center','merge')) else '#f0e5c9'
        ld.ellipse((p[0]-radius,p[1]-radius,p[0]+radius,p[1]+radius),fill=color,outline='#14242c',width=2)
        font=fonts(21 if close else 17)
        tx,ty=p[0]+14,p[1]-10
        box=ld.textbbox((tx,ty),label,font=font)
        ld.rectangle((box[0]-5,box[1]-3,box[2]+5,box[3]+3),fill='#14242c')
        ld.text((tx,ty),label,font=font,fill=color)
    # District labels outside the racing line, rather than another long strip.
    for p,label in [([-470,-245],'WORKING / MARKET QUARTER'),([400,-300],'WORKSHOPS'),
                    ([-410,355],'OLD TOWN'),([390,360],'CIVIC QUARTER'),([76,60],'SQUARE'),([-60,-180],'MARKET'),([180,60],'CITY HALL')]:
        pix=xy(p);f=fonts(14 if label=='SQUARE' else 16)
        ld.text(pix,label,font=f,fill='#e4e6d6',anchor='mm',stroke_width=2,stroke_fill='#14242c')
    im.paste(layer.crop((35,165,w-35,h-115)),(35,165))
    d.text((55,28),'ROAD / DASH — A CITY WITH A CENTER',font=fonts(35),fill='#f2e9d7')
    d.text((55,82),'2D layout only • two split / rejoin sections • connected neighborhoods',font=fonts(22),fill='#dfc296')
    d.text((55,122),'First rejoin at downtown; second rejoin before exit. Four route combinations.',font=fonts(19),fill='#b6c8c8')
    d.text((55,h-88),'GOLD: left choice   CYAN: right choice   IVORY: shared route   GREEN: squares / courts',font=fonts(18),fill='#dfdfce')
    d.text((55,h-53),'Concept streets and parcels — not yet playable. No 3D, terrain or game-code changes.',font=fonts(18),fill='#b6c8c8')
    svg.append('</g></svg>')
    name='center-detail' if close else 'race-routes' if route_only else 'city-masterplan'
    im.save(folder/f'{name}.png')
    (folder/f'{name}.svg').write_text(''.join(svg))


def verify_outputs(folder):
    original = design()
    negative_checks = []
    for name in ['dead_end', 'cycle', 'endpoint', 'plot_in_road']:
        candidate = json.loads(json.dumps(original))
        if name == 'dead_end':
            candidate['race_edges'] = [e for e in candidate['race_edges'] if e['id'] != 'central_avenue']
        elif name == 'cycle':
            candidate['race_edges'].append(dict(id='bad_cycle', source='center_merge', target='split_1',
                points=[[0, 0], [0, -600]], kind='left', width_m=8, length_m=600))
        elif name == 'endpoint':
            candidate['race_edges'][0]['points'][0] = [99, 99]
        else:
            candidate['proposed_plots'].append(dict(id='bad_plot', footprint=[-1, -1, 1, 1]))
        try:
            validate(candidate)
        except (AssertionError, KeyError):
            negative_checks.append(name)
        else:
            raise AssertionError(f'Invalid {name} accepted')
    outputs = ['LAYOUT.json', 'city-masterplan.png', 'city-masterplan.svg',
               'race-routes.png', 'race-routes.svg', 'center-detail.png', 'center-detail.svg', 'index.html']
    with tempfile.TemporaryDirectory() as temp:
        subprocess.run([sys.executable, str(Path(__file__).resolve()), '--output', temp],
                       cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
        hashes = {p: hashlib.sha256((folder / p).read_bytes()).hexdigest() for p in outputs}
        assert all(hashlib.sha256((Path(temp) / p).read_bytes()).hexdigest() == digest
                   for p, digest in hashes.items())
    changed = subprocess.check_output(['git', 'diff', '--name-only', BASELINE], cwd=ROOT, text=True).splitlines()
    changed += subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard'], cwd=ROOT, text=True).splitlines()
    runtime = [p for p in changed if p.startswith(('src/', 'crates/', 'content/', 'art/', 'assets/', 'Cargo.'))]
    assert not runtime, runtime
    receipt = dict(graph=validate(original), rejected_invalid_graphs=negative_checks,
                   deterministic_output_sha256=hashes, runtime_differences_from_58eeabb=runtime,
                   limits='Plan-only graph and plot checks. Not a playable map, complete junction/terrain audit or measured racing-time balance.')
    (folder / 'VALIDATION.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print('Verified graph, negative cases, deterministic exports and unchanged runtime.')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,default=OUTPUT)
    parser.add_argument('--verify',action='store_true')
    args=parser.parse_args()
    plan=design();plan['validation']=validate(plan)
    args.output.mkdir(parents=True,exist_ok=True)
    (args.output/'LAYOUT.json').write_text(json.dumps(plan,indent=2)+'\n')
    draw(plan,args.output)
    draw(plan,args.output,route_only=True)
    draw(plan,args.output,close=True)
    (args.output/'index.html').write_text('''<!doctype html><meta charset="utf-8"><title>RoadDash city center layout</title><style>body{background:#14242c;color:#eee;font:18px system-ui;margin:24px}img{max-width:100%}a{color:#63cfdb}</style><h1>City center and two racing choices</h1><p>2D only. Choose left/right at S1, rejoin downtown at R1. Choose again at S2, rejoin at R2. Four route combinations.</p><img src="city-masterplan.png"><h2>Route map</h2><img src="race-routes.png"><h2>Downtown</h2><img src="center-detail.png"><p><a href="city-masterplan.svg">Zoomable parcel map</a> · <a href="LAYOUT.json">Graph and layout data</a></p>''')
    print(json.dumps(plan['validation'],indent=2))
    if args.verify:
        verify_outputs(args.output)


if __name__=='__main__':
    main()
