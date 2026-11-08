#!/usr/bin/env python3
"""Design a dense 2D street/property layout; never edits playable world content."""
import argparse
import bisect
import hashlib
import html
import json
import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
START, END = 2700.0, 4950.0
ROAD_HALF, WALK, SETBACK = 4.0, 1.8, 0.2
FRONT = ROAD_HALF + WALK + SETBACK
FRONT_DEPTH, COURT_DEPTH, REAR_DEPTH = 18.0, 8.0, 12.0
BACK = FRONT + FRONT_DEPTH + COURT_DEPTH + REAR_DEPTH
CROSS_WIDTH, BLOCK_PITCH = 8.0, 108.0
DEPTH_ROWS = 2
ROW_PITCH = BACK - FRONT + 6.0 + 2.0
OUTER = BACK + ROW_PITCH * (DEPTH_ROWS - 1)


class Route:
    def __init__(self, points):
        self.points = [(p[0], p[2]) for p in points]
        self.tangents = []
        for i in range(len(self.points)):
            a = self.points[max(i - 1, 0)]
            b = self.points[min(i + 1, len(self.points) - 1)]
            length = math.dist(a, b)
            self.tangents.append(((b[0] - a[0]) / length, (b[1] - a[1]) / length))
        self.stations = [0.0]
        for a, b in zip(points, points[1:]):
            self.stations.append(self.stations[-1] + math.dist(a, b))

    def at(self, station, lane=0.0):
        i = min(max(bisect.bisect_right(self.stations, station) - 1, 0), len(self.points) - 2)
        a, b = self.points[i:i + 2]
        t = (station - self.stations[i]) / (self.stations[i + 1] - self.stations[i])
        # Continuous offset normals avoid tiny loops at baked polyline joints.
        ta, tb = self.tangents[i:i + 2]
        dx, dz = (ta[k] + (tb[k] - ta[k]) * t for k in [0, 1])
        length = math.hypot(dx, dz)
        return [a[0] + (b[0] - a[0]) * t + dz / length * lane,
                a[1] + (b[1] - a[1]) * t - dx / length * lane]

    def strip(self, start, end, inner, outer):
        count = max(1, math.ceil((end - start) / 2.0))
        stations = [start + (end - start) * i / count for i in range(count + 1)]
        return ([self.at(d, inner) for d in stations]
                + [self.at(d, outer) for d in reversed(stations)])


def design(report):
    route = Route(report['centerline'])
    crossings = [START + 54.0 + BLOCK_PITCH * i for i in range(math.ceil((END - START) / BLOCK_PITCH))]
    crossings = [d for d in crossings if d < END - CROSS_WIDTH]
    intervals = []
    cursor = START
    for d in crossings:
        intervals.append((cursor, d - CROSS_WIDTH / 2))
        cursor = d + CROSS_WIDTH / 2
    intervals.append((cursor, END))
    parcels, blocks = [], []
    for index, (start, end) in enumerate(intervals):
        count = max(1, round((end - start) / 11.0))
        for side, row in [(side, row) for side in [-1, 1] for row in range(DEPTH_ROWS)]:
            front = FRONT + ROW_PITCH * row
            back = BACK + ROW_PITCH * row
            block_id = f'DENSE-{index + 1:02}-{"L" if side < 0 else "R"}-R{row + 1}'
            blocks.append(dict(id=block_id, side=side, depth_row=row, start_m=start, end_m=end))
            for bay in range(count):
                a = start + (end - start) * bay / count
                b = start + (end - start) * (bay + 1) / count
                ident = f'{block_id}-P{bay + 1:02}'
                parcels.append(dict(
                    id=ident, block_id=block_id, side=side, depth_row=row,
                    start_m=a, end_m=b, frontage_station_m=b - a,
                    owner_id=f'PROPOSED-{ident}',
                    lot=route.strip(a, b, side * front, side * back),
                    front_building=route.strip(a, b, side * front, side * (front + FRONT_DEPTH)),
                    private_court=route.strip(a, b, side * (front + FRONT_DEPTH), side * (front + FRONT_DEPTH + COURT_DEPTH)),
                    rear_wing=route.strip(a, b, side * (back - REAR_DEPTH), side * back),
                    front_door=route.at((a + b) / 2, side * front),
                    rear_access=route.at((a + b) / 2, side * back),
                    boundary_status='concept parcels; independent of existing runtime lot IDs',
                ))
    occupied = sum(b - a for a, b in intervals)
    plan = dict(
        status='2D LAYOUT PROPOSAL ONLY; no runtime, 3D, texture, height or physics changes',
        source_geometry_fingerprint=report['geometry_fingerprint'],
        route_interval_m=[START, END],
        dimensions_m=dict(carriageway=ROAD_HALF * 2, sidewalk_each=WALK,
                          facade_setback=SETBACK, facade_to_facade=FRONT * 2,
                          front_building_depth=FRONT_DEPTH, courtyard_depth=COURT_DEPTH,
                          rear_wing_depth=REAR_DEPTH, back_street_width=6.0, depth_rows_each_side=DEPTH_ROWS,
                          cross_street_width=CROSS_WIDTH, block_pitch=BLOCK_PITCH),
        rules=['Buildings share party walls: zero gaps within a block.',
               'Front doors face the race street; no parking forecourts.',
               'Only cross streets interrupt the primary street wall.',
               'Courtyards and servicing go behind the frontage.',
               'The existing race centerline is reused; this is not a new playable map.',
               'No building heights, models, materials or current lot quotas are changed.'],
        crossing_stations_m=crossings, blocks=blocks, parcels=parcels,
        metrics=dict(proposed_parcels=len(parcels), primary_frontage_parcels=sum(p["depth_row"] == 0 for p in parcels), primary_street_blocks=sum(b["depth_row"] == 0 for b in blocks), total_blocks=len(blocks),
                     frontage_coverage_excluding_crossings=1.0,
                     frontage_coverage_including_crossings=occupied / (END - START),
                     inter_building_gap_m=0.0),
    )
    return route, plan


def area(poly):
    return abs(sum(a[0] * b[1] - b[0] * a[1]
                   for a, b in zip(poly, poly[1:] + poly[:1]))) / 2


def validate(route, plan):
    ids = [p['id'] for p in plan['parcels']]
    assert len(ids) == len(set(ids)), 'duplicate parcel identity'
    assert plan['metrics']['frontage_coverage_including_crossings'] >= 0.92
    for block in plan['blocks']:
        row = [p for p in plan['parcels'] if p['block_id'] == block['id']]
        assert abs(row[0]['start_m'] - block['start_m']) < 1e-8
        assert abs(row[-1]['end_m'] - block['end_m']) < 1e-8
        for a, b in zip(row, row[1:]):
            assert abs(a['end_m'] - b['start_m']) < 1e-8, 'frontage gap'
            offset = ROW_PITCH * block['depth_row']
            for depth in [FRONT + offset, FRONT + FRONT_DEPTH + offset, BACK + offset]:
                assert math.dist(route.at(a['end_m'], a['side'] * depth),
                                 route.at(b['start_m'], b['side'] * depth)) < 1e-8
    for parcel in plan['parcels']:
        assert 7.0 < parcel['frontage_station_m'] < 15.0
        for key in ['front_building', 'private_court', 'rear_wing', 'lot']:
            assert all(math.isfinite(v) for p in parcel[key] for v in p)
            assert area(parcel[key]) > 1.0, 'degenerate footprint'
    return dict(parcel_ids_unique=True, within_block_frontage_gaps=0,
                shared_party_boundaries=True, finite_nonzero_polygons=True,
                runtime_files_written=False,
                limitations='Concept geometry only. Full cross-route overlap, swept vehicle, junction, terrain support and lot migration validation precede future runtime implementation.')


def fonts(size):
    for name in ['DejaVuSans.ttf', 'NotoSans-Regular.ttf']:
        try:
            return ImageFont.truetype(name, size)
        except OSError:
            pass
    return ImageFont.load_default(size=size)


def draw_map(route, plan, out, detail=False):
    start, end = (3420.0, 3650.0) if detail else (START, END)
    selected = [p for p in plan['parcels'] if p['end_m'] > start and p['start_m'] < end]
    extent = [route.at(d, lane) for d in [start + (end - start) * i / 300 for i in range(301)] for lane in [-OUTER-10, OUTER+10]]
    lo = [min(p[i] for p in extent) for i in [0, 1]]
    hi = [max(p[i] for p in extent) for i in [0, 1]]
    w, h = (1700, 1250) if detail else (1400, 1900)
    padding, top, bottom = 70, 180, 120
    scale = min((w - padding * 2) / (hi[0] - lo[0]), (h - top - bottom) / (hi[1] - lo[1]))
    origin_x = (w - (hi[0] - lo[0]) * scale) / 2
    def xy(p):
        return (origin_x + (p[0] - lo[0]) * scale,
                top + (hi[1] - p[1]) * scale)
    im = Image.new('RGB', (w, h), '#16252b')
    d = ImageDraw.Draw(im)
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}"><rect width="100%" height="100%" fill="#16252b"/><defs><clipPath id="map"><rect x="30" y="160" width="{w-60}" height="{h-260}"/></clipPath></defs><g clip-path="url(#map)">']
    layer = Image.new('RGB', (w, h), '#16252b')
    ld = ImageDraw.Draw(layer)
    def polygon(points, color, label='', outline=None):
        pp = [xy(p) for p in points]
        ld.polygon(pp, fill=color, outline=outline)
        svg.append('<polygon points="' + ' '.join(f'{x:.2f},{y:.2f}' for x, y in pp) + f'" fill="{color}" stroke="{outline or color}" stroke-width="0.6"><title>{html.escape(label)}</title></polygon>')
    for side in [-1, 1]:
        polygon(route.strip(start - 10, end + 10, side * ROAD_HALF, side * (ROAD_HALF + WALK)), '#bbc1b6')
        for row in range(1, DEPTH_ROWS):
            front = FRONT + ROW_PITCH * row
            polygon(route.strip(start - 10, end + 10, side * (front - WALK - SETBACK), side * (front - SETBACK)), '#bbc1b6')
        for row in range(DEPTH_ROWS):
            back = BACK + ROW_PITCH * row
            polygon(route.strip(start - 10, end + 10, side * back, side * (back + 6)), '#394c54')
    polygon(route.strip(start - 10, end + 10, -ROAD_HALF, ROAD_HALF), '#62737a')
    for cross in plan['crossing_stations_m']:
        if start - 10 < cross < end + 10:
            polygon(route.strip(cross - CROSS_WIDTH / 2, cross + CROSS_WIDTH / 2, -(OUTER + 6), OUTER + 6), '#62737a')
    for p in selected:
        color = ['#d9b186', '#b78f78', '#dbbd99', '#bd9c87'][int(p['block_id'].split('-')[1]) % 4]
        polygon(p['lot'], '#253c36', p['owner_id'], '#7a8476' if detail else None)
        polygon(p['front_building'], color, p['id'] + ' | touching front building', '#26383b' if detail else None)
        polygon(p['rear_wing'], '#89978c', p['id'] + ' | rear wing', '#26383b' if detail else None)
        if detail:
            door = xy(p['front_door'])
            ld.ellipse((door[0]-2, door[1]-2, door[0]+2, door[1]+2), fill='#ecdfb8')
    # Clip the map area before adding labels; no off-sheet streets over titles.
    im.paste(layer.crop((30, 160, w - 30, h - 100)), (30, 160))
    d.text((55, 25), 'ROAD / DASH — DENSE STREET LAYOUT', font=fonts(34), fill='#f1eadc')
    d.text((55, 78), '2D proposal only • zero gaps between buildings • no 3D changes', font=fonts(22), fill='#e0c08a')
    d.text((55, 115), f'{start/1000:.2f}–{end/1000:.2f} km of the existing race centerline', font=fonts(19), fill='#b7c7c9')
    d.text((55, h - 84), 'TAN: touching street frontage   GREEN: private rear courts   GREY: rear wings / service streets', font=fonts(17), fill='#d5ddda')
    bar = 20 if detail else 100
    d.line([(w-245, h-52), (w-245+bar*scale, h-52)], fill='white', width=3)
    d.text((w-245, h-39), f'{bar} m', font=fonts(16), fill='white')
    svg.append('</g></svg>')
    name = 'block-plan' if detail else 'masterplan'
    im.save(out / f'{name}.png')
    (out / f'{name}.svg').write_text(''.join(svg))


def draw_widths(out):
    im = Image.new('RGB', (1700, 530), '#16252b')
    d = ImageDraw.Draw(im)
    d.text((55, 30), 'THE STREET MUST FEEL ENCLOSED', font=fonts(34), fill='#f1eadc')
    d.text((55, 90), 'Top-down width study. Building fronts only 12 m apart. No forecourts or lawns.', font=fonts(22), fill='#e0c08a')
    scale = 55
    center = 850
    for a, b, col in [(-12, -FRONT, '#d9b186'), (FRONT, 12, '#d9b186'),
                      (-ROAD_HALF, ROAD_HALF, '#62737a'),
                      (-ROAD_HALF-WALK, -ROAD_HALF, '#bbc1b6'),
                      (ROAD_HALF, ROAD_HALF+WALK, '#bbc1b6')]:
        d.rectangle((center+a*scale, 185, center+b*scale, 350), fill=col)
    d.line((center, 200, center, 335), fill='#eadc9b', width=3)
    for x, label in [(center - 9 * scale, 'BUILDING'), (center, '8 m ROAD'),
                     (center + 9 * scale, 'BUILDING')]:
        d.text((x, 265), label, font=fonts(23), fill='#f9f5ea', anchor='mm')
    d.text((center, 400), '1.8 m sidewalk + 0.2 m setback on each side', font=fonts(24), fill='#d5ddda', anchor='mm')
    d.text((center, 455), 'Street-facing buildings touch their neighbours; courtyards and access sit behind them.', font=fonts(20), fill='#d5ddda', anchor='mm')
    im.save(out / 'street-widths.png')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report', type=Path, help='Existing --inspect-world JSON; read only')
    parser.add_argument('--output', type=Path, default=ROOT / 'docs/planning/dense-city-layout')
    args = parser.parse_args()
    report = json.loads(args.report.read_text())
    route, plan = design(report)
    plan['source_report_sha256'] = hashlib.sha256(args.report.read_bytes()).hexdigest()
    plan['validation'] = validate(route, plan)
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / 'LAYOUT.json').write_text(json.dumps(plan, indent=2) + '\n')
    draw_map(route, plan, args.output)
    draw_map(route, plan, args.output, detail=True)
    draw_widths(args.output)
    (args.output / 'index.html').write_text('''<!doctype html><meta charset="utf-8"><title>RoadDash dense city layout</title><style>body{background:#16252b;color:#eee;font:18px system-ui;margin:24px}img{max-width:100%}a{color:#e0c08a}object{width:100%;height:85vh}</style><h1>Dense city — 2D layout only</h1><p>Buildings touch. Narrow sidewalks. 12m facade-to-facade. Courtyards and servicing behind the street wall. No game/3D changes.</p><img src="street-widths.png"><h2>Block plan</h2><img src="block-plan.png"><h2>Full city strip</h2><p>Open the SVG to zoom and hover parcel IDs.</p><a href="masterplan.svg">Full vector masterplan</a><object type="image/svg+xml" data="masterplan.svg"></object>''')
    print(json.dumps(plan['metrics'], indent=2))
    print('Layout-only validation passed. Runtime files untouched.')


if __name__ == '__main__':
    main()
