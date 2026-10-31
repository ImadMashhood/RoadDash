#!/usr/bin/env python3
"""Export the actual generated XZ footprints, road and reservations to SVG."""
import argparse
import html
import json
from pathlib import Path


def export(report, target):
    layout = report['layout']
    points = report['centerline']
    min_x = min(p[0] for p in points) - 140
    max_x = max(p[0] for p in points) + 140
    min_z = min(p[2] for p in points) - 80
    max_z = max(p[2] for p in points) + 80
    scale = min(800 / (max_x - min_x), 1650 / (max_z - min_z))

    def point(x, z):
        return f'{340 + (x-min_x)*scale:.3f},{1780-(z-min_z)*scale:.3f}'

    def corners(rect):
        x, z = rect['center']
        a, b = rect['right']
        hx, hz = rect['half']
        return [point(x + a*hx*s - b*hz*t, z + b*hx*s + a*hz*t)
                for s, t in [(-1, -1), (1, -1), (1, 1), (-1, 1)]]

    colors = {'F': '#cf724f', 'R': '#e1c580', 'C': '#7dabb0', 'U': '#a4a9db', 'H': '#8bc57a'}
    out = ['<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="1850" viewBox="0 0 1200 1850">',
           '<rect width="1200" height="1850" fill="#18232a"/>',
           '<g font-family="sans-serif" fill="#e7efee"><text x="35" y="50" font-size="28">ROAD / DASH — Valley Run</text>',
           '<text x="35" y="84" font-size="16">Generated structural plan / metres / +Y up / XZ view</text>',
           f'<text x="35" y="114" font-size="16">{len(layout["buildings"])} buildings · 15 sections · {len(layout["settings"]["junctions"])} junctions · {len(layout["fields"])} fields</text>',
           f'<text x="35" y="144" font-size="13">{html.escape(report["geometry_fingerprint"])}</text></g>']
    for index, (family, color) in enumerate(colors.items()):
        labels = {'F': 'Farm', 'R': 'Residential', 'C': 'Commercial', 'U': 'Urban', 'H': 'Hillside'}
        y = 82 + index * 22
        out.append(f'<rect x="760" y="{y-12}" width="14" height="14" fill="{color}"/>')
        out.append(f'<text x="783" y="{y}" font-family="sans-serif" font-size="14" fill="#e7efee">{labels[family]}</text>')
    out.append('<g fill="#32434a" stroke="#617178" stroke-width="0.2">')
    for rect in layout['road_reservations']:
        out.append(f'<polygon points="{" ".join(corners(rect))}"/>')
    for rect in layout['junction_rects']:
        out.append(f'<polygon points="{" ".join(corners(rect))}" fill="#56636a"/>')
    out.append('</g><polyline fill="none" stroke="#e7d49b" stroke-width="1" points="' +
               ' '.join(point(p[0], p[2]) for p in points) + '"/>')
    for building in layout['buildings']:
        color = colors[building['archetype_id'][0]]
        label = html.escape(f'{building["id"]} / {building["archetype_id"]} / {building["row"]} / foundation {building["foundation_m"]:.2f} m')
        out.append(f'<polygon points="{" ".join(corners(building["footprint"]))}" fill="{color}" stroke="#0e171d" stroke-width="0.25"><title>{label}</title></polygon>')
    out.append('<g font-family="sans-serif" font-size="14" fill="#e7efee">')
    for section in report['sections']:
        x, y = point(section['start_position'][0], section['start_position'][2]).split(',')
        out.append(f'<text x="35" y="{y}">{html.escape(section["id"]+" / "+section["name"])} · {section["total"]}</text>')
        out.append(f'<line x1="295" y1="{y}" x2="{x}" y2="{y}" stroke="#66767e" stroke-width="0.5"/>')
    out.append('</g></svg>')
    target.write_text('\n'.join(out), encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('report', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    export(json.loads(args.report.read_text(encoding='utf-8')), args.output)
