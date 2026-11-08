#!/usr/bin/env python3
"""Redraw the entire farm-to-long-city race as a curved, graded 2D planning atlas."""
import argparse
import bisect
import hashlib
import html
import json
import math
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image, ImageDraw
from plan_dense_city import Route, fonts
from street_first_layout import make_fabric, audit_fabric
from city_massing import apply_massing, audit_massing, height_colour, DETAIL_COLOURS
from race_first_layout import audit_race_first

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'docs/planning/farms-long-city'
BASELINE = 'fb46a0a'
CITY_START, FARM_END, FINISH = 3150.0, 2700.0, 12000.0
CITY_LENGTH = FINISH - CITY_START
SPLIT = 200.0
MERGE = CITY_LENGTH * 0.25
COLOURS = ['#bfae91', '#caa77b', '#96aeaa', '#a7a898', '#b8ad9b']


def smooth(x):
    x = min(1.0, max(0.0, x))
    return x * x * (3 - 2 * x)


class Trace:
    def __init__(self, points):
        self.points = points
        self.distances = [0.0]
        for a, b in zip(points, points[1:]):
            self.distances.append(self.distances[-1] + math.dist(a, b))

    def at(self, distance):
        d = min(self.distances[-1], max(0.0, distance))
        i = min(bisect.bisect_right(self.distances, d) - 1, len(self.points) - 2)
        t = (d - self.distances[i]) / (self.distances[i + 1] - self.distances[i])
        return [self.points[i][k] + t * (self.points[i + 1][k] - self.points[i][k]) for k in [0, 1]]

    def frame(self, distance):
        p = self.at(distance)
        a, b = self.at(distance - 5), self.at(distance + 5)
        dx, dz = b[0] - a[0], b[1] - a[1]
        length = math.hypot(dx, dz)
        return p, [dz / length, -dx / length]


def raw_city(t):
    return [350 * math.sin(3 * math.pi * t) ** 2 + 230 * math.sin(2 * math.pi * t) ** 2, 7200 * t]


class World:
    def __init__(self, report):
        old = Route(report['centerline'])
        anchor = old.at(CITY_START)
        a, b = old.at(CITY_START - 2), old.at(CITY_START + 2)
        length = math.dist(a, b)
        forward = [(b[k] - a[k]) / length for k in [0, 1]]
        right = [forward[1], -forward[0]]
        self.projection = dict(anchor=anchor, forward=forward, right=right)
        self.farm_samples = []
        for i in range(631):
            d = CITY_START * i / 630
            p = old.at(d)
            delta = [p[k] - anchor[k] for k in [0, 1]]
            self.farm_samples.append((d, [sum(delta[k] * right[k] for k in [0, 1]),
                                           sum(delta[k] * forward[k] for k in [0, 1])]))
        self.farm = Trace([p for _, p in self.farm_samples])
        raw = [raw_city(i / 1200) for i in range(1201)]
        raw_length = sum(math.dist(a, b) for a, b in zip(raw, raw[1:]))
        self.city_scale = CITY_LENGTH / raw_length
        self.city = Trace([[x * self.city_scale, z * self.city_scale] for x, z in raw])
        self.height_z = self.city.points[-1][1]

    def center_x(self, z):
        t = z / self.height_z
        return raw_city(min(1.0, max(0.0, t)))[0] * self.city_scale

    def site_point(self, u, v):
        # Coordinates remain rigid; road boundaries determine each site polygon.
        return [u, v]

    def height(self, p):
        x, z = p
        farm = 12 + 2.0 * math.sin(z / 950) + 1.3 * math.sin(x / 650)
        city = (12 + 0.0042 * z + 8 * math.sin(z / 1050)
                + 4 * math.sin(x / 700 + z / 1300) + 6 * math.sin(z / 450))
        blend = smooth((z + 200) / 400)
        return farm * (1 - blend) + city * blend

    def xyz(self, p):
        return [p[0], self.height(p), p[1]]

    def farm_at(self, station, offset=0):
        p, normal = self.farm.frame(self.farm.distances[-1] * station / CITY_START)
        return [p[k] + normal[k] * offset for k in [0, 1]]

    def city_at(self, s, offset=0):
        p, normal = self.city.frame(s)
        return [p[k] + normal[k] * offset for k in [0, 1]]


def branch(world, side, width):
    points = []
    for i in range(241):
        t = i / 240
        s = SPLIT + t * (MERGE - SPLIT)
        offset = side * width * math.sin(math.pi * t) ** 2
        points.append(world.city_at(s, offset))
    # Exactly shared graph anchors, not almost-equal endpoints.
    points[0], points[-1] = world.city_at(SPLIT), world.city_at(MERGE)
    return points


def length(points):
    return sum(math.dist(a, b) for a, b in zip(points, points[1:]))


def plan(report):
    world = World(report)
    # Similar nominal distances, but deliberately different city streets/grades.
    candidates = {side: [(width, branch(world, side, width)) for width in range(240, 441, 10)] for side in [-1, 1]}
    pair = min(((a, b) for a in candidates[-1] for b in candidates[1]),
               key=lambda pair: abs(length(pair[0][1]) - length(pair[1][1])) + 0.01 * abs(pair[0][0] - 340) + 0.01 * abs(pair[1][0] - 340))
    widths, choices = [p[0] for p in pair], [p[1] for p in pair]
    approach = [p for _, p in world.farm_samples] + [world.city_at(SPLIT * i / 30) for i in range(1, 31)]
    tail = [world.city_at(MERGE + (CITY_LENGTH - MERGE) * i / 700) for i in range(701)]
    nodes = dict(start=world.xyz(approach[0]), split=world.xyz(choices[0][0]),
                 rejoin=world.xyz(choices[0][-1]), finish=world.xyz(tail[-1]))
    edges = []
    for ident, source, target, points, interval in [
        ('farm_approach', 'start', 'split', approach, [0, CITY_START + SPLIT]),
        ('old_market', 'split', 'rejoin', choices[0], [CITY_START + SPLIT, CITY_START + MERGE]),
        ('station_avenue', 'split', 'rejoin', choices[1], [CITY_START + SPLIT, CITY_START + MERGE]),
        ('long_city', 'rejoin', 'finish', tail, [CITY_START + MERGE, FINISH]),
    ]:
        xyz = [world.xyz(p) for p in points]
        edges.append(dict(id=ident, source=source, target=target, points=xyz,
                          logical_chainage_m=interval, measured_planar_m=length(points),
                          measured_3d_m=length(xyz)))
    fields=[]
    for field in report['layout']['fields']:
        a,b=field['start_m'],min(field['end_m'],CITY_START)
        if a>=b:continue
        side=field['side'];outer=250+(int(a/150)%4)*60
        stations=[a+(b-a)*i/8 for i in range(9)]
        polygon=[world.farm_at(d,side*16) for d in stations]+[world.farm_at(d,side*outer) for d in reversed(stations)]
        fields.append(dict(id=field['id'],polygon=polygon,crop=['pasture','grain','fallow'][int(a/150)%3]))
    landmarks=[]
    for ident,name,s,side,offset in [
        ('city_entry','CITY ENTRY',0,-1,70),('city_hall','CITY HALL / SPLIT',SPLIT,-1,90),
        ('market','OLD MARKET',950,-1,480),('station','EAST STATION',1150,1,480),
        ('rejoin','REJOIN — CITY QUARTER',MERGE,1,90),
        ('courts','MARKET QUARTER',3400,-1,600),('workshops','WORKSHOPS / HOUSING',4800,1,650),
        ('terraces','TERRACED CITY STREETS',6300,-1,650),('north','NORTH CITY',7700,1,600),
        ('finish','FINISH — INSIDE THE CITY',CITY_LENGTH,-1,100)]:
        landmarks.append(dict(id=ident,name=name,point=world.city_at(s,side*offset),race_point=world.city_at(s),city_chainage_m=s))
    public_sites=[]
    for landmark in landmarks:
        if landmark['id'] in ['city_hall','station','market','finish']:
            x,z=landmark['point'];r=52 if landmark['id']=='finish' else 38
            public_sites.append(dict(id=landmark['id'],name=landmark['name'],bounds=[x-r,z-r,x+r,z+r]))
    race_rules=json.loads((OUTPUT/'RACE_LAYOUT_RULES.json').read_text())
    fabric=make_fabric(world,edges,public_sites,nodes,race_rules)
    result=dict(status='2D layout/elevation proposal only; current game and all 3D content unchanged',
        design='farms → urban transition → long city with curved graded streets defining polygonal blocks and straight-edged corner buildings → urban finish; one split/rejoin',
        planning_chainage_m=dict(farms_end=FARM_END,city_start=CITY_START,split=CITY_START+SPLIT,rejoin=CITY_START+MERGE,finish=FINISH),
        distance_note='12 km is the draft spine chainage. Measured paths include the branch detour and are reported separately.',
        farm_source_projection=world.projection,source_geometry_fingerprint=report['geometry_fingerprint'],
        nodes=nodes,edges=edges,public_sites=public_sites,fields=fields,landmarks=landmarks,**fabric,
        branch_half_widths_m=widths,
        phases=['farms','urban_transition','city'],
        elevation_note='A continuous proposed street/terrain height field, not a new hills district or applied physics surface.',
        route_contract='One split, two alternatives, one rejoin. Common ordered gates at split/rejoin; edge-relative progress and branch recovery require future runtime integration.',
        construction_limits='Local planning coordinates. Street-first polygon sites, not accepted building architecture. Junction controls, complete lot access/terrain support, bridges and runtime IDs/quotas need later construction validation.')
    rules=json.loads((OUTPUT/'MASSING_RULES.json').read_text())
    apply_massing(world,result,rules)
    return world,result


def validate(world, data):
    assert data['phases']==['farms','urban_transition','city']
    outgoing={n:[] for n in data['nodes']};edges={e['id']:e for e in data['edges']}
    for edge in edges.values():
        assert edge['points'][0]==data['nodes'][edge['source']]
        assert edge['points'][-1]==data['nodes'][edge['target']]
        assert abs(length(edge['points'])-edge['measured_3d_m'])<1e-6
        outgoing[edge['source']].append(edge)
    assert sum(len(e)>1 for e in outgoing.values())==1
    paths=[]
    def walk(n,path,seen):
        assert n not in seen
        if n=='finish':paths.append(path);return
        assert outgoing[n]
        for e in outgoing[n]:walk(e['target'],path+[e['id']],seen|{n})
    walk('start',[],set());assert len(paths)==2
    grades=[];heights=[]
    for e in edges.values():
        for a,b in zip(e['points'],e['points'][1:]):
            planar=math.hypot(b[0]-a[0],b[2]-a[2]);assert planar>0
            grades.append(abs(b[1]-a[1])/planar);heights.append(a[1])
    assert max(grades)<0.06
    assert max(heights)-min(heights)>25
    assert abs(data['planning_chainage_m']['rejoin']-CITY_START-CITY_LENGTH/4)<1e-6
    assert len({b['id'] for b in data['blocks']})==len(data['blocks'])
    fabric_audit=audit_fabric(data)
    massing_audit=audit_massing(data)
    race_audit=audit_race_first(data)
    lengths=[sum(edges[e]['measured_3d_m'] for e in p) for p in paths]
    early=[(edges['farm_approach']['measured_3d_m']-CITY_START+edges[e]['measured_3d_m'])/(lengths[i]-CITY_START) for i,e in enumerate(['old_market','station_avenue'])]
    assert all(.22<x<.30 for x in early)
    finish = [data['nodes']['finish'][0], data['nodes']['finish'][2]]
    nearby = [b['center'] for b in data['blocks'] if math.dist(b['center'], finish) < 350]
    assert any(p[0] < finish[0] - 30 for p in nearby) and any(p[0] > finish[0] + 30 for p in nearby)
    assert any(p[1] < finish[1] - 30 for p in nearby) and any(p[1] > finish[1] + 30 for p in nearby)
    return dict(legal_paths=paths,measured_route_lengths_m=lengths,merge_fraction_of_actual_city_paths=early,
                exactly_one_split=True,exactly_one_rejoin=True,finish_surrounded_by_city=True,
                city_spine_length_m=CITY_LENGTH,city_share_of_spine=CITY_LENGTH/FINISH,
                proposed_elevation_range_m=[min(heights),max(heights)],max_sampled_grade_percent=max(grades)*100,
                city_blocks=len(data['blocks']),source_farm_fields=len(data['fields']),
                fabric=fabric_audit,massing=massing_audit,race_first=race_audit,all_building_edges_straight=True)


def profile(world,data,out):
    w,h=2100,760;im=Image.new('RGB',(w,h),'#14272d');d=ImageDraw.Draw(im)
    d.text((65,30),'ELEVATION STUDY — FARMS TO CITY FINISH',font=fonts(34),fill='#f2ebdb')
    d.text((65,84),'Urban rises, dips and crests remain. The separate hills district is removed from this proposal.',font=fonts(22),fill='#d7c094')
    lo,hi=0,70;top,bottom=160,580
    def xy(station,height):return(90+station/FINISH*(w-180),bottom-(height-lo)/(hi-lo)*(bottom-top))
    for level in range(0,71,10):
        yy=xy(0,level)[1];d.line((90,yy,w-90,yy),fill='#354c50',width=1);d.text((40,yy),str(level)+' m',font=fonts(16),fill='#afc4c5',anchor='lm')
    for edge in data['edges']:
        a,b=edge['logical_chainage_m'];pts=[]
        for i,p in enumerate(edge['points']):pts.append(xy(a+(b-a)*i/(len(edge['points'])-1),p[1]))
        d.line(pts,fill={'old_market':'#efb667','station_avenue':'#6cd4df'}.get(edge['id'],'#eee4ce'),width=5,joint='curve')
    for station,label in [(0,'START'),(FARM_END,'FARM EDGE'),(CITY_START,'CITY'),(CITY_START+MERGE,'REJOIN'),(FINISH,'FINISH')]:
        x=xy(station,0)[0];d.line((x,top,x,bottom),fill='#597270');d.text((x,bottom+24),f'{station/1000:.2f} km',font=fonts(18),fill='#e8e2cd',anchor='mm')
    d.text((90,650),'ONE split / rejoin • draft ~12 km spine • no separate hills zone • city continues through the finish',font=fonts(22),fill='#e6d7b6')
    d.text((90,700),f"Maximum sampled proposal grade: {data['validation']['max_sampled_grade_percent']:.2f}% • elevations are planning data, not game terrain",font=fonts(20),fill='#b2c9c9')
    im.save(out/'elevation-profile.png')


def contours(world, bounds, draw_line, step=110):
    xmin,zmin,xmax,zmax=bounds
    for x in range(math.floor(xmin/step)*step,math.ceil(xmax/step)*step,step):
        for z in range(math.floor(zmin/step)*step,math.ceil(zmax/step)*step,step):
            corners=[[x,z],[x+step,z],[x+step,z+step],[x,z+step]]
            values=[world.height(p) for p in corners]
            for level in range(math.ceil(min(values)/5)*5,math.floor(max(values)/5)*5+1,5):
                hits=[]
                for i,j in [(0,1),(1,2),(2,3),(3,0)]:
                    if (values[i]<level)!=(values[j]<level):
                        t=(level-values[i])/(values[j]-values[i]);hits.append([corners[i][k]+t*(corners[j][k]-corners[i][k]) for k in [0,1]])
                if len(hits)==2:draw_line(hits,'#567365',1)
                elif len(hits)==4:
                    draw_line(hits[:2],'#567365',1);draw_line(hits[2:],'#567365',1)


def draw(world,data,out,detail=False,corner=False,view='layout',downtown=False,junction=False):
    if junction:
        core=data['massing']['core_point']
        candidates=[j for j in data['race_first']['primary_junctions'] if j['avenue_id']=='long_city']
        selected=min(candidates,key=lambda j:abs(j['angle_degrees']-60)+abs(j['point'][1]-core[1])*.001)
        x,z=selected['point'];bounds=[x-300,z-310,x+300,z+310]
        w,h=1900,2150
    elif downtown:
        x,z=data['massing']['core_point'];bounds=[x-680,z-680,x+680,z+680]
        w,h=2100,2250
    elif corner:
        example=next(b for b in data['blocks'] if b['id']==data['corner_example_id'])
        x,z=example['center'];bounds=[x-290,z-300,x+290,z+300]
        w,h=1800,2050
    elif detail:
        p=[p for e in data['edges'] if e['id'] in ['old_market','station_avenue'] for p in e['points']]
        bounds=[min(q[0] for q in p)-550,min(q[2] for q in p)-220,max(q[0] for q in p)+550,max(q[2] for q in p)+220]
        w,h=2000,2250
    else:
        pts=[p for b in data['blocks'] for p in b['polygon']]+[p for f in data['fields'] for p in f['polygon']]
        bounds=[min(p[0] for p in pts)-200,min(p[1] for p in pts)-250,max(p[0] for p in pts)+250,max(p[1] for p in pts)+250]
        w,h=2200,3600
    xmin,zmin,xmax,zmax=bounds;pad,top,bottom=80,190,190
    scale=min((w-2*pad)/(xmax-xmin),(h-top-bottom)/(zmax-zmin))
    left=(w-(xmax-xmin)*scale)/2
    def xy(p):return(left+(p[0]-xmin)*scale,top+(zmax-p[1])*scale)
    im=Image.new('RGB',(w,h),'#263c35');d=ImageDraw.Draw(im)
    layer=Image.new('RGB',(w,h),'#263c35');ld=ImageDraw.Draw(layer)
    svg=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}"><rect width="100%" height="100%" fill="#263c35"/><defs><clipPath id="map"><rect x="40" y="170" width="{w-80}" height="{h-340}"/></clipPath></defs><g clip-path="url(#map)">']
    def polygon(points,color,label='',outline=None):
        pp=[xy(p) for p in points];ld.polygon(pp,fill=color,outline=outline)
        svg.append('<polygon points="'+' '.join(f'{x:.2f},{y:.2f}' for x,y in pp)+f'" fill="{color}" stroke="{outline or color}"><title>{html.escape(label)}</title></polygon>')
    def line(points,color,width):
        pp=[xy(p) for p in points]
        pixels=max(4 if color in ['#f06d58','#f3ead1','#f0b257','#6ed5df'] else 1,width*scale)
        ld.line(pp,fill=color,width=round(pixels),joint='curve')
        svg.append('<polyline points="'+' '.join(f'{x:.2f},{y:.2f}' for x,y in pp)+f'" fill="none" stroke="{color}" stroke-width="{pixels:.2f}"/>')
    def shape(records,color,hole_color):
        for record in records:
            polygon(record['shell'],color)
            for hole in record['holes']:polygon(hole,hole_color)
    contours(world,bounds,line,80 if detail else 110)
    for f in data['fields']:
        polygon(f['polygon'],{'pasture':'#65815b','grain':'#b5a36c','fallow':'#8d8763'}[f['crop']],f['id']+' | '+f['crop'],'#405a42')
    # Draw actual road/sidewalk polygons first, then the complementary land blocks.
    shape(data['road_reservation'],'#b5b9ac','#263c35')
    shape(data['road_surface'],'#34474d','#b5b9ac')
    for b in data['blocks']:
        polygon(b['polygon'],'#42514a',b['id']+' | '+b['district'],'#5c6a60')
        if b['role'] in ['park','traffic_island']:
            polygon(b['polygon'],'#647f64',b['id']+' | '+b['role'])
            continue
        if corner:
            for parcel in b['parcels']:polygon(parcel,'#42514a',b['id']+' | ownership boundary','#667267')
        for building in b['buildings']:
            face=b['faces'][building['footprint_index']]
            colour=(height_colour(building['floors']) if view=='height' else
                    DETAIL_COLOURS[building['detail_tier']] if view=='detail' else
                    COLOURS[['old_city','market','mixed','terraces','north_city'].index(b['district'])])
            outline='#f4e6c8' if building['skyline_anchor'] else '#626450'
            polygon(face,colour,building['id']+f" | {building['floors']} floors / {building['height_m']} m | {building['detail_tier']}",outline)
            for hole in b['face_holes'][building['footprint_index']]:polygon(hole,'#42514a')
            if downtown and building['skyline_anchor']:
                center=[sum(p[k] for p in face)/len(face) for k in [0,1]]
                pp=xy(center)
                ld.text(pp,str(building['floors']),font=fonts(17),anchor='mm',fill='#fff4df')
                svg.append(f'<text x="{pp[0]:.2f}" y="{pp[1]+6:.2f}" text-anchor="middle" fill="#fff4df" font-size="17">{building["floors"]}</text>')
        for site in b['infill_sites']:
            shape(site['access_easement'],'#afbaa4','#42514a')
        shape(b['public_land'],'#b5bda5','#42514a')
    if corner:
        polygon(example['polygon'],'#ecb464',example['id']+' | road-cut corner building','#f8deab')
    # Original farmyards tied to specific approach stations, rather than scattered props.
    for station,side in [(420,-1),(980,1),(1560,-1),(2220,1),(2810,-1)]:
        p=world.farm_at(station,side*65);size=22
        polygon([[p[0]-size,p[1]-10],[p[0]+size,p[1]-10],[p[0]+size,p[1]+10],[p[0]-size,p[1]+10]],'#c0ab87','Farmyard / barn footprint')
        line([world.farm_at(station,side*8),p],'#b0a38a',4)
    for e in data['edges']:
        points=[[p[0],p[2]] for p in e['points']]
        if e['id']=='farm_approach':
            line(points,'#b5b9ac',12.8);line(points,'#34474d',8)
        line(points,{'old_market':'#f0b257','station_avenue':'#6ed5df'}.get(e['id'],'#f06d58'),1.5 if detail or corner or downtown or junction else 3)
    def label(point,text,color='#efe9d6',size=22,dx=14,dy=-8):
        p=xy(point);f=fonts(size);pos=(p[0]+dx,p[1]+dy)
        box=ld.textbbox(pos,text,font=f);ld.rectangle((box[0]-5,box[1]-4,box[2]+5,box[3]+4),fill='#20342e')
        ld.text(pos,text,font=f,fill=color)
        svg.append(f'<text x="{pos[0]:.2f}" y="{pos[1]+size:.2f}" fill="{color}" font-size="{size}" font-family="sans-serif">{html.escape(text)}</text>')
    if corner:
        label(example['center'],example['id']+' — ANGLED CORNER SITE',size=24,dx=20,dy=-50)
    for mark in data['landmarks']:
        p=mark['race_point'] if mark['id'] in ['city_entry','city_hall','rejoin','finish'] else mark['point']
        pp=xy(p)
        if not 180<pp[1]<h-190:continue
        ld.ellipse((pp[0]-6,pp[1]-6,pp[0]+6,pp[1]+6),fill='#efe6c6',outline='#20342e',width=2)
        if mark['id'] in ['city_entry','city_hall','rejoin','finish']:
            label(p,mark['name']+f"  {(CITY_START+mark['city_chainage_m'])/1000:.2f} km",size=23 if detail else 21)
        else:label(p,mark['name'],size=18 if detail else 20)
    if junction:
        label(selected['point'],'RACE AVENUE / PRIORITY CROSSING',size=23,dx=18,dy=-55)
        label([selected['point'][0]-180,selected['point'][1]-155],f"Crossing angle: {selected['angle_degrees']:.0f} degrees",size=22,dx=0,dy=0)
    if view!='layout':
        label(data['massing']['core_point'],'DOWNTOWN HEIGHT CORE',size=25,dx=20,dy=25)
    if not detail and not corner and not downtown and not junction:
        for station in [6000, 8000, 10000]:
            point = world.city_at(station - CITY_START)
            label(point, f'{station/1000:.0f} km / {world.height(point):.0f} m elev.', size=21, dx=-180, dy=-16)
        label(world.farm_at(0),'START — FARMS  0 km',size=24)
        label(world.farm_at(1300,-350),'OPEN FIELDS / FARMYARDS',size=23)
        label(world.farm_at(2850,-250),'FARMS TO URBAN TRANSITION',size=22)
    # Metric scale and north direction in the actual map projection.
    xx,yy=90,h-225
    ld.line((xx,yy,xx+100*scale,yy),fill='#efe9d6',width=4)
    ld.text((xx,yy-28),'100 m',font=fonts(20),fill='#efe9d6')
    ld.text((w-100,190),'N ^',font=fonts(26),fill='#efe9d6')
    im.paste(layer.crop((40,170,w-40,h-170)),(40,170))
    d.rectangle((0,0,w,165),fill='#14272d')
    title='RACE-FIRST CITY JUNCTION' if junction else 'DOWNTOWN / TOWER SITES' if downtown else 'BUILDING HEIGHT PLAN' if view=='height' else 'BUILDING DETAIL ALLOCATION' if view=='detail' else 'STREETS DEFINE THE BLOCKS' if corner else 'ROAD / DASH — CITY AROUND THE RACE AVENUE'
    d.text((65,28),title,font=fonts(39),fill='#f4ecda')
    subtitle='Taller central streets • scattered taller neighbors / interior sites • six-metre service access' if view=='height' else 'Distance from both legal race routes • skyline anchors retain their silhouette' if view=='detail' else 'Road boundaries -> blocks -> fitted parcels / buildings' if corner else 'Primary avenue first • balanced city on both sides • straight-edged blocks / buildings'
    d.text((65,86),subtitle,font=fonts(23),fill='#d8c393')
    d.text((65,128),'2D planning draft — no separate hills district; current game / 3D content unchanged',font=fonts(21),fill='#b2c9c7')
    d.rectangle((0,h-165,w,h),fill='#14272d')
    if view=='height':
        legend=['TAN: 2–4 floors   OCHRE: 5–7   ORANGE: 8–12   ROSE: 13–20   PURPLE: 21–32',
                'Floor height: 3.2 m • outlined sites: interior towers • pale green: clear service access',
                'Downtown is tallest; frontage accents and interior towers interrupt the surrounding street walls.']
    elif view=='detail':
        legend=['PEACH: hero <70 m   GOLD: standard 70–220 m   BLUE: skyline 220–550 m   GREY: massing >550 m',
                'Interior towers retain at least skyline detail • distance measured from nearest footprint to either race path',
                'Every tier retains closed walls, roofs, height and visual identity. Runtime LOD is future work.']
    else:
        legend=['RED: common race avenue   GOLD: Old Market branch   CYAN: Station Avenue branch',
                'Dark: streets • grey: sidewalks • tan: footprints • green: courts • pale access lanes lead to towers',
                'Draft spine: 0–3.15 km farms / transition; 3.15–12 km city. Height/detail maps are in the atlas.']
    for i,text in enumerate(legend):d.text((65,h-140+i*42),text,font=fonts(21 if i==0 else 20),fill='#e6dcc3' if i==0 else '#b5c8c1')
    svg.append(f'</g><rect width="{w}" height="165" fill="#14272d"/><rect y="{h-165}" width="{w}" height="165" fill="#14272d"/>')
    for text,y,size in [(title,65,39),(subtitle,112,23),('2D plan only; no runtime / 3D changes',153,21)]+[(text,h-117+i*42,21) for i,text in enumerate(legend)]:
        svg.append(f'<text x="65" y="{y}" fill="#e6dcc3" font-family="sans-serif" font-size="{size}">{html.escape(text)}</text>')
    svg.append('</svg>')
    name='race-junction-detail' if junction else 'downtown-plan' if downtown else 'height-plan' if view=='height' else 'detail-plan' if view=='detail' else 'corner-block-detail' if corner else 'city-quarter-detail' if detail else 'whole-route'
    im.save(out/f'{name}.png');(out/f'{name}.svg').write_text(''.join(svg))


def verify(world,data,out,report_path):
    rejected=[]
    for case in ["second_split","hills_phase","grade_jump","outside_city_finish","road_intrusion","overlapping_parcels","short_block","unnoded_crossing","invalid_height","wrong_detail","missing_tower_access","blocked_service_access","false_centering","off_track_junction","omitted_street_restored","disconnected_city_fragment"]:
        candidate=json.loads(json.dumps(data))
        if case=="second_split":
            extra=json.loads(json.dumps(candidate["edges"][-1]));extra["id"]="extra_exit";candidate["edges"].append(extra)
        elif case=="hills_phase":candidate["phases"].append("hills")
        elif case=="grade_jump":
            edge=candidate["edges"][-1];edge["points"][100][1]+=100;edge["measured_3d_m"]=length(edge["points"])
        elif case=="invalid_height":
            next(b for b in candidate['blocks'] if b['buildings'])['buildings'][0]['floors']=99
        elif case=="wrong_detail":
            building=next(x for b in candidate['blocks'] for x in b['buildings'] if x['detail_tier']=='massing')
            building['detail_tier']='hero'
        elif case=="missing_tower_access":
            next(b for b in candidate['blocks'] if b['infill_sites'])['infill_sites']=[]
        elif case=="blocked_service_access":
            block=next(b for b in candidate['blocks'] if b['infill_sites'])
            block['infill_sites'][0]['access_easement']=[dict(shell=block['faces'][0],holes=[])]
        elif case=="false_centering":candidate['race_first']['centering_rows'][8]['route_x_range_m'][0]+=400
        elif case=="off_track_junction":candidate['race_first']['primary_junctions'][0]['point'][0]+=40
        elif case=="omitted_street_restored":
            missing=candidate['race_first']['omitted_secondary_segments'][0].copy();missing.pop('reason')
            candidate['streets'].append(missing)
        elif case=="disconnected_city_fragment":candidate['street_graph_segments'].append([[90000,90000],[90100,90000]])
        elif case=="short_block":candidate['block_pitch_m']=[180,180]
        elif case=="unnoded_crossing":
            candidate['street_graph_segments'] += [[[9000,0],[9100,100]],[[9000,100],[9100,0]]]
        elif case=="overlapping_parcels":
            block=next(b for b in candidate['blocks'] if b['parcels']);block['parcels'].append(block['parcels'][0])
        elif case=="road_intrusion":
            first=next(b for b in candidate['blocks'] if b['faces'])
            first['faces'][0]=[[-5000,0],[-4990,0],[-4990,10],[-5000,10]]
        else:
            edge=candidate["edges"][-1];edge["points"][-1][2]+=2000;candidate["nodes"]["finish"]=edge["points"][-1];edge["measured_3d_m"]=length(edge["points"])
        try:validate(world,candidate)
        except AssertionError:rejected.append(case)
        else:raise AssertionError(f"Invalid {case} was accepted")
    names=['LAYOUT.json','whole-route.png','whole-route.svg','city-quarter-detail.png','city-quarter-detail.svg','elevation-profile.png','corner-block-detail.png','corner-block-detail.svg','height-plan.png','height-plan.svg','detail-plan.png','detail-plan.svg','downtown-plan.png','downtown-plan.svg','race-junction-detail.png','race-junction-detail.svg','index.html']
    with tempfile.TemporaryDirectory() as folder:
        subprocess.run([sys.executable,str(Path(__file__).resolve()),str(report_path.resolve()),'--output',folder],check=True,stdout=subprocess.DEVNULL,cwd=ROOT)
        hashes={name:hashlib.sha256((out/name).read_bytes()).hexdigest() for name in names}
        assert all(hashlib.sha256((Path(folder)/name).read_bytes()).hexdigest()==digest for name,digest in hashes.items())
    changed=subprocess.check_output(['git','diff','--name-only',BASELINE],cwd=ROOT,text=True).splitlines()
    changed+=subprocess.check_output(['git','ls-files','--others','--exclude-standard'],cwd=ROOT,text=True).splitlines()
    runtime=[p for p in changed if p.startswith(('src/','content/','art/','assets/','crates/','Cargo.'))]
    assert not runtime,runtime
    previous=json.loads(subprocess.check_output(['git','show','de20a70:docs/planning/farms-long-city/LAYOUT.json'],cwd=ROOT,text=True))
    for name in ['nodes','edges','fields','planning_chainage_m','farm_source_projection']:
        assert previous[name]==data[name], f'Race/farm alignment changed: {name}'
    receipt=dict(preserved_race_and_farms_from_de20a70=True,route=data['validation'],rejected_invalid_plans=rejected,deterministic_outputs=hashes,runtime_differences_from_fb46a0a=runtime,
                 limits='2D concept/height study only. Not a playable route bake, full lot/traffic/terrain validation or measured performance.')
    (out/'VALIDATION.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('Verified one-fork graph, urban finish, street-derived polygon blocks/parcels, elevation/grade bounds, repeatable outputs and unchanged runtime.')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report',type=Path)
    parser.add_argument('--output',type=Path,default=OUTPUT)
    parser.add_argument('--verify',action='store_true');args=parser.parse_args()
    report=json.loads(args.report.read_text());world,data=plan(report)
    data['source_report_sha256']=hashlib.sha256(args.report.read_bytes()).hexdigest()
    data['validation']=validate(world,data)
    args.output.mkdir(parents=True,exist_ok=True)
    (args.output/'LAYOUT.json').write_text(json.dumps(data,indent=2)+'\n')
    draw(world,data,args.output);draw(world,data,args.output,True);draw(world,data,args.output,corner=True);draw(world,data,args.output,view='height');draw(world,data,args.output,view='detail');draw(world,data,args.output,view='height',downtown=True);draw(world,data,args.output,junction=True);profile(world,data,args.output)
    (args.output/'index.html').write_text('''<!doctype html><meta charset="utf-8"><title>RoadDash farms to long city</title><style>body{background:#14272d;color:#eee;font:18px system-ui;margin:24px}img{max-width:100%}a{color:#6ed5df}</style><h1>Whole route: farms → long city → urban finish</h1><p>One split, two alternatives, rejoin near the first city quarter. Curved roads and proposed street elevation. No hills district. Layout only.</p><img src="whole-route.png"><h2>Race-first junction detail</h2><img src="race-junction-detail.png"><h2>First city quarter / split</h2><img src="city-quarter-detail.png"><h2>Street-cut corner site / actual block from this map</h2><img src="corner-block-detail.png"><h2>Building height plan</h2><img src="height-plan.png"><h2>Authored detail by race-road distance</h2><img src="detail-plan.png"><h2>Downtown towers / access</h2><img src="downtown-plan.png"><h2>Elevation profile</h2><img src="elevation-profile.png"><p><a href="whole-route.svg">Zoomable vector map</a> · <a href="LAYOUT.json">Layout / height / route data</a></p>''')
    print(json.dumps(data['validation'],indent=2))
    if args.verify:verify(world,data,args.output,args.report)


if __name__=='__main__':main()
