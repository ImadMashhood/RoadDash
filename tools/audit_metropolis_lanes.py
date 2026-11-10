"""Whole-body/paint checks against FINAL visible road, sidewalk and building polygons.

Run with the Shapely authoring venv. Unlike the historical carriageway audit,
road containment cannot conceal raised sidewalks or buildings on top of asphalt.
"""
import argparse
import bisect
import json
import math
from pathlib import Path

from shapely import Polygon, STRtree, union_all
from metropolis_streetscape import footprint

ROOT = Path(__file__).resolve().parents[1]


def polygon(shape):
    result = Polygon()
    for ring in shape['rings']:
        result = result.symmetric_difference(Polygon(ring))
    return result


def blend(sections, route, station):
    ss = [s for s in sections if s['route'] == route]
    k = next((i for i, s in enumerate(ss) if s['start'] <= station < s['end']), len(ss)-1)
    b, a = ss[k], ss[max(0, k-1)]
    t = max(0., min(1., (station-b['start'])/100.))
    return a, b, t*t*(3.-2.*t)


def lane_center(sections, route, station, outgoing, slot):
    a, b, t = blend(sections, route, station)
    def center(s):
        count = s['outbound' if outgoing else 'inbound']
        return (s['outbound']-s['inbound'])*s['lane_width']/2 + (-1 if outgoing else 1)*(min(slot, count-1)+.5)*s['lane_width']
    return center(a)+(center(b)-center(a))*t


def marks(sections, route, station):
    a, b, t = blend(sections, route, station)
    def values(s):
        return ((s['outbound']-s['inbound'])*s['lane_width']/2,
                s['lane_width'], (s['outbound']+s['inbound'])*s['lane_width']/2)
    return tuple(x+(y-x)*t for x, y in zip(values(a), values(b)))


def sample(path, station, lane=0.):
    i = max(0, min(len(path['points'])-2, bisect.bisect_right(path['progress'], station)-1))
    a, b = path['points'][i:i+2]
    t = (station-path['progress'][i])/(path['progress'][i+1]-path['progress'][i])
    dx, dz = b[0]-a[0], b[2]-a[2]
    n = math.hypot(dx, dz)
    dx, dz = dx/n, dz/n
    return (a[0]+(b[0]-a[0])*t-dz*lane, a[2]+(b[2]-a[2])*t+dx*lane), math.atan2(dx, dz)


class Surfaces:
    def __init__(self, shapes):
        self.shapes = shapes
        self.tree = STRtree(shapes)

    def local(self, shape):
        return union_all([self.shapes[i] for i in self.tree.query(shape)])

    def overlap(self, shape):
        return sum(shape.intersection(self.shapes[i]).area for i in self.tree.query(shape))


def audit(m):
    road = Surfaces([polygon(s['shape']) for s in m['surfaces'] if s['kind'] == 0])
    walk = Surfaces([polygon(s['shape']) for s in m['surfaces'] if s['kind'] == 1])
    buildings = Surfaces([polygon(b['shape']) for b in m['buildings']])
    # Fine samples span the farm/entry centre alignment and every lane taper.
    stations = sorted(set(range(0, 12000, 4)) | set(range(2700, 3501)) | set(range(5260, 5481)))
    failures, body_count, paint_count = [], 0, 0
    sections = m['street_life']['lane_sections']
    for route, path in enumerate(m['traffic_paths']):
        for station in stations:
            for outgoing in [True, False]:
                for slot in range(2):
                    lane = lane_center(sections, route, station, outgoing, slot)
                    p, yaw = sample(path, station, lane)
                    body = footprint(p, yaw, 1.9, 3.9)
                    body_count += 1
                    outside = body.difference(road.local(body).buffer(.025)).area
                    raised, solid = walk.overlap(body), buildings.overlap(body)
                    if max(outside, raised, solid) > .001:
                        failures.append(dict(kind='body', route=route, station=station, outgoing=outgoing, slot=slot,
                                             outside_road_m2=outside, sidewalk_overlap_m2=raised, building_overlap_m2=solid))
            # Match runtime four-metre paint quads, including their actual width.
            if station % 4 or (route == 1 and not 3350 <= station < 5362.5):
                continue
            divider, width, half = marks(sections, route, station)
            _, profile, _ = blend(sections, route, station)
            stripes = [(lane,4) for lane in [divider-.11, divider+.11, -half+.25, half-.25]]
            if station % 12 < 6:
                stripes += [(divider-i*width,min(4,6-station%12)) for i in range(1, profile['outbound'])]
                stripes += [(divider+i*width,min(4,6-station%12)) for i in range(1, profile['inbound'])]
            for lane,length in stripes:
                (x, z), yaw = sample(path, station, lane)
                end_divider, end_width, end_half = marks(sections, route, station+length)
                end_lane = math.copysign(end_half-.25,lane) if abs(abs(lane)-(half-.25)) < .001 else end_divider+(lane-divider)/width*end_width
                (xx, zz), end_yaw = sample(path, station+length, end_lane)
                ex, ez = math.cos(end_yaw)*.07, -math.sin(end_yaw)*.07
                dx, dz = math.cos(yaw)*.07, -math.sin(yaw)*.07
                shape = Polygon([(x-dx,z-dz),(xx-ex,zz-ez),(xx+ex,zz+ez),(x+dx,z+dz)])
                paint_count += 1
                outside = shape.difference(road.local(shape).buffer(.025)).area
                raised = walk.overlap(shape)
                if max(outside, raised) > .001:
                    failures.append(dict(kind='paint',route=route,station=station,lane=lane,
                                         outside_road_m2=outside,sidewalk_overlap_m2=raised))
    overlaps = sum(walk.overlap(shape) for shape in road.shapes)
    solid_overlap = sum(buildings.overlap(shape) for shape in road.shapes)
    return dict(body_samples=body_count, paint_samples=paint_count, body_dimensions_m=[1.9,3.9],
                fine_sample_step_m=1, road_tolerance_m=.025, visible_road_sidewalk_overlap_m2=overlaps, visible_road_building_overlap_m2=solid_overlap,
                failures=failures)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', type=Path, default=ROOT/'content/world/metropolis.json')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    report = audit(json.loads(args.input.read_text()))
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k != 'failures'}, indent=2))
    print('failures:',len(report['failures']),report['failures'][:8])
    assert not report['failures'] and report['visible_road_sidewalk_overlap_m2'] < .01 and report['visible_road_building_overlap_m2'] < .01


if __name__ == '__main__':
    main()
