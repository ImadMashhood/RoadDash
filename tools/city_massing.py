"""Seeded, offline building height/detail and interior-site authoring."""
import hashlib
import math
from collections import Counter

from shapely import LineString, Point, Polygon, box
from shapely.ops import nearest_points, unary_union

from street_first_layout import encode, decode, polygons, ring

HEIGHT_COLOURS = [(4, '#c8b991'), (7, '#bfa16a'), (12, '#cf895b'),
                  (20, '#be6573'), (100, '#a285ce')]
DETAIL_COLOURS = dict(hero='#efaa72', standard='#d5c67f', skyline='#76b5c6', massing='#7d9186')


def height_colour(floors):
    return next(colour for maximum, colour in HEIGHT_COLOURS if floors <= maximum)


def pick(seed, key):
    digest = hashlib.blake2b(f'{seed}:{key}'.encode(), digest_size=8).digest()
    return int.from_bytes(digest, 'big') / 2**64


def integer(seed, key, limits):
    return limits[0] + int(pick(seed, key) * (limits[1] - limits[0] + 1))


def core_score(point, core, rules):
    rx, rz = rules['core_falloff_radii_m']
    return math.exp(-.5 * (((point[0]-core[0])/rx)**2 + ((point[1]-core[1])/rz)**2))


def zone_for(score, rules):
    return next(z for z in reversed(rules['zones']) if score >= z['minimum_core_score'])


def tier_for(distance, anchor, rules):
    index = sum(distance >= limit for limit in rules['detail_distance_edges_m'])
    return rules['detail_tiers'][min(index, 2) if anchor else index]


def tower_site(block, faces, reserved, seed, key, rules):
    # Actual residual court land, not a tower stamped over another building.
    free = block.difference(unary_union(faces).union(reserved)).buffer(-rules['tower_frontage_clearance_m'], join_style='mitre')
    if free.is_empty:
        return None
    sites = []
    for part in polygons(free):
        xmin, ymin, xmax, ymax = part.bounds
        sites.append(part.representative_point())
        for x in range(math.ceil(xmin/24)*24, math.floor(xmax/24)*24+1, 24):
            for y in range(math.ceil(ymin/24)*24, math.floor(ymax/24)*24+1, 24):
                if part.contains(Point(x,y)):
                    sites.append(Point(x,y))
    sites.sort(key=lambda p:pick(seed, f'{key}:site:{p.x:.5f}:{p.y:.5f}'))
    for w, h in rules['tower_size_options_m']:
        for point in sites:
            pad = box(point.x-w/2, point.y-h/2, point.x+w/2, point.y+h/2)
            if not free.covers(pad):
                continue
            entry = nearest_points(block.exterior, point)[0]
            route = LineString([entry, point])
            access = route.buffer(rules['service_access_width_m']/2, cap_style='square', join_style='mitre').intersection(block).difference(pad)
            if access.intersection(reserved).area > 1e-6:
                continue
            if access.boundary.intersection(block.boundary.buffer(1e-7)).length < rules['service_access_width_m']-.1:
                continue
            return dict(pad=pad, access=access, route=route, entry=entry)
    return None


def apply_massing(world, data, rules):
    assert isinstance(rules['seed'],int)
    assert rules['floor_height_m']>0 and all(v>0 for v in rules['core_falloff_radii_m'])
    assert rules['detail_distance_edges_m']==sorted(set(rules['detail_distance_edges_m']))
    assert len(rules['detail_tiers'])==len(rules['detail_distance_edges_m'])+1
    for zone in rules['zones']:
        for kind in ['base_floors','accent_floors','tower_floors']:
            lo,hi=zone[kind];assert isinstance(lo,int) and isinstance(hi,int) and 1<=lo<=hi<=100
        assert zone['accent_floors'][0]>zone['base_floors'][1]
        assert zone['tower_floors'][0]>zone['base_floors'][1]
    seed = rules['seed']
    core = next(l['race_point'] for l in data['landmarks'] if l['id']==rules['core_landmark'])
    race = unary_union([LineString([(p[0],p[2]) for p in edge['points']]) for edge in data['edges']])
    for b in data['blocks']:
        score = core_score(b['center'], core, rules)
        zone = zone_for(score, rules)
        b.update(core_score=score, height_zone=zone['id'], buildings=[], infill_sites=[],
                 parcel_ids=[f'{b["id"]}-P{i:03}' for i in range(len(b['parcels']))])
        if not b['faces']:
            continue
        original = [Polygon(p,h) for p,h in zip(b['faces'],b['face_holes'])]
        reserved = decode(b['public_land'])
        block = Polygon(b['polygon'], b['holes'])
        base = integer(seed, b['id']+':base', zone['base_floors'])
        accent = None
        chance = rules['frontage_accent_probability']
        if pick(seed,b['id']+':accent') < chance[0]+score*(chance[1]-chance[0]):
            accent = integer(seed,b['id']+':accent-index',[0,len(original)-1])
        site = None
        chance = rules['interior_tower_probability']
        if b['role']=='perimeter_block' and pick(seed,b['id']+':tower') < chance[0]+score*(chance[1]-chance[0]):
            site = tower_site(block, original, reserved, seed, b['id'], rules)
        entries = []
        for i, polygon in enumerate(original):
            # Cut a real at-grade service opening; do not imply access through solid walls.
            pieces = polygons(polygon.difference(site['access'])) if site else [polygon]
            floors = (integer(seed,f'{b["id"]}:{i}:accent',zone['accent_floors']) if i==accent
                      else max(zone['base_floors'][0], min(zone['base_floors'][1], base+integer(seed,f'{b["id"]}:{i}:variation',[-1,1]))))
            for j, piece in enumerate(pieces):
                if piece.area >= rules['minimum_footprint_area_m2']:
                    entries.append((f'{b["id"]}-F{i:03}-{j}',piece,floors,'frontage_accent' if i==accent else 'street_wall'))
        if site:
            entries.append((b['id']+'-TOWER',site['pad'],integer(seed,b['id']+':tower-floors',zone['tower_floors']),'interior_tower'))
            affected = [f'{b["id"]}-P{i:03}' for i,p in enumerate(b['parcels'])
                        if Polygon(p).intersection(site['pad'].union(site['access'])).area>1e-6]
            points = list(map(list,site['route'].coords))
            heights = [world.height(p) for p in points]
            grade = abs(heights[1]-heights[0])/site['route'].length
            b['infill_sites']=[dict(id=b['id']+'-SITE',tower_building_id=b['id']+'-TOWER',
                                   access_easement=encode(site['access']), access_centerline=points,
                                   street_entry=list(site['entry'].coords[0]), access_width_m=rules['service_access_width_m'],
                                   affected_parcel_ids=affected, access_grade_percent=100*grade,
                                   ownership_note='Proposed joint site assembly and recorded access easement; not a legal parcel transfer.')]
        b['faces']=[ring(p) for _,p,_,_ in entries]
        b['face_holes']=[[list(map(list,r.coords[:-1])) for r in p.interiors] for _,p,_,_ in entries]
        for index,(ident,polygon,floors,role) in enumerate(entries):
            distance=polygon.distance(race)
            height=round(floors*rules['floor_height_m'],2)
            anchor=role=='interior_tower'
            center=polygon.centroid
            b['buildings'].append(dict(id=ident,footprint_index=index,role=role,floors=floors,height_m=height,
                                       base_elevation_m=world.height([center.x,center.y]),
                                       roof_elevation_m=round(world.height([center.x,center.y])+height,3),
                                       min_race_distance_m=distance,detail_tier=tier_for(distance,anchor,rules),
                                       skyline_anchor=anchor))
    data['massing']=dict(rules=rules,core_point=core,
                         scope='Offline height/site/detail authoring only; no 3D geometry or runtime LOD implemented.')


def audit_massing(data):
    rules=data['massing']['rules'];core=data['massing']['core_point']
    race=unary_union([LineString([(p[0],p[2]) for p in e['points']]) for e in data['edges']])
    all_buildings=[];towers=0;accents=0;tiers=Counter();central=[];edge_heights=[]
    identifiers=[]
    for b in data['blocks']:
        zone=zone_for(core_score(b['center'],core,rules),rules)
        assert b['height_zone']==zone['id']
        assert len(b['faces'])==len(b['buildings']), 'Footprint/height association missing'
        for building in b['buildings']:
            identifiers.append(building['id'])
            face=Polygon(b['faces'][building['footprint_index']],b['face_holes'][building['footprint_index']])
            role=building['role']
            limits=zone['tower_floors'] if role=='interior_tower' else zone['accent_floors'] if role=='frontage_accent' else zone['base_floors']
            assert limits[0]<=building['floors']<=limits[1], 'Height outside district band'
            assert abs(building['height_m']-building['floors']*rules['floor_height_m'])<1e-6
            assert abs(building['roof_elevation_m']-building['base_elevation_m']-building['height_m'])<.001
            distance=face.distance(race)
            assert abs(distance-building['min_race_distance_m'])<1e-6
            assert building['detail_tier']==tier_for(distance,building['skyline_anchor'],rules), 'Wrong authored detail tier'
            assert building['skyline_anchor']==(role=='interior_tower')
            assert face.intersection(decode(b['public_land'])).area<1e-5, 'Building invades civic land'
            tiers[building['detail_tier']]+=1
            towers+=role=='interior_tower';accents+=role=='frontage_accent'
            all_buildings.append(building)
            if role=='street_wall':
                if zone['id']=='downtown':central.append(building['height_m'])
                elif zone['id']=='outskirts':edge_heights.append(building['height_m'])
        tower_ids={x['id'] for x in b['buildings'] if x['role']=='interior_tower'}
        assert len(tower_ids)==len(b['infill_sites']), 'Missing interior tower access'
        for site in b['infill_sites']:
            assert site['tower_building_id'] in tower_ids and site['affected_parcel_ids']
            assert set(site['affected_parcel_ids']).issubset(b['parcel_ids']), 'Unknown service/site parcel'
            access=decode(site['access_easement'])
            assert access.is_valid and access.area>0
            tower=next(x for x in b['buildings'] if x['id']==site['tower_building_id'])
            pad=Polygon(b['faces'][tower['footprint_index']],b['face_holes'][tower['footprint_index']])
            approach=LineString(site['access_centerline']).difference(pad)
            assert access.buffer(1e-7).covers(approach), 'Disconnected service route'
            assert site['access_width_m']>=6 and site['access_grade_percent']<8
            assert access.difference(Polygon(b['polygon'],b['holes'])).area<1e-5
            assert all(access.intersection(Polygon(face,holes)).area<1e-5 for face,holes in zip(b['faces'],b['face_holes'])), 'Service access blocked by building'
            assert access.boundary.intersection(Polygon(b['polygon']).boundary.buffer(1e-7)).length>5.9
    assert len(set(identifiers))==len(identifiers)
    assert towers>10 and accents>10 and set(tiers)==set(rules['detail_tiers'])
    assert sum(central)/len(central)>sum(edge_heights)/len(edge_heights)+8, 'No downtown height hierarchy'
    return dict(buildings=len(all_buildings),interior_towers=towers,frontage_accents=accents,detail_tiers=dict(tiers),
                downtown_mean_streetwall_height_m=sum(central)/len(central),
                outskirts_mean_streetwall_height_m=sum(edge_heights)/len(edge_heights),
                height_range_m=[min(x['height_m'] for x in all_buildings),max(x['height_m'] for x in all_buildings)],
                service_access_width_m=rules['service_access_width_m'],core_point=core,
                note='Authored detail allocation, not measured runtime LOD/performance.')
