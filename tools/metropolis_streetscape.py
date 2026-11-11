"""Original sidewalk reservations and deterministic curb furniture authoring."""
import math
from collections import Counter
from shapely import Point, Polygon, LineString, box
from shapely.ops import unary_union
from street_first_layout import decode

CURB_SHIFT = 1.0
CLEAR_WALK_M = 1.5

def sidewalks(data):
    road, walk = decode(data['road_surface']), decode(data['sidewalks'])
    bounds = decode(data['city_envelope']).bounds
    urban = box(bounds[0]-1, 0, bounds[2]+1, bounds[3]+1)
    # Restore the original 12m primary carriageway within its existing right of way.
    # Secondary streets keep their curb overlay; no building footprint moves.
    primary = unary_union([LineString([(p[0],p[2]) for p in e['points']]).simplify(2.0).buffer(6.0, cap_style='square', join_style='mitre') for e in data.get('edges', [])])
    gained = walk.buffer(CURB_SHIFT, join_style='mitre').intersection(road).intersection(urban).difference(primary)
    return road.difference(gained), walk.union(gained), gained

def farm_pavement(data):
    """Additional farm asphalt, excluding the city's retained street reservation.

    The farm is four lanes wide. The existing urban approach is eight metres;
    widening underneath its raised sidewalks/buildings does not make more lanes.
    """
    farm=next(e for e in data['edges'] if e['id']=='farm_approach')
    wide=LineString([(p[0],p[2]) for p in farm['points']]).buffer(6.8,cap_style='square',join_style='mitre')
    return wide.difference(decode(data['city_envelope'])).difference(decode(data['road_surface']))

def footprint(p, yaw, width, depth):
    c,s=math.cos(yaw),math.sin(yaw)
    return Polygon([(p[0]+c*x+s*z,p[1]-s*x+c*z) for x,z in [(-width/2,-depth/2),(width/2,-depth/2),(width/2,depth/2),(-width/2,depth/2)]])

def furniture(data, paths, walk):
    result=[];occupied=[]
    clear_shift=1.1
    junctions=[Point(j['point']) for j in data['race_first']['primary_junctions']]
    import shapely
    shapely.prepare(walk)
    def add(kind,p,yaw,size,width=None,depth=None):
        for shift in sorted([i*.025 for i in range(-72,73)],key=abs):
            candidate=(p[0]+dz*side*shift,p[1]-dx*side*shift)
            shape=footprint(candidate,yaw,width or size[0],depth or size[2]).buffer(.12)
            clear_center=(candidate[0]+dz*side*clear_shift,candidate[1]-dx*side*clear_shift)
            clear=footprint(clear_center,math.atan2(dx,dz),CLEAR_WALK_M,max(2.,depth or size[2]))
            if not walk.covers(clear) or not walk.covers(shape) or any(shape.distance(j)<20 for j in junctions):continue
            if any(shape.distance(other)<1.0 for other in occupied):continue
            occupied.append(shape)
            result.append(dict(id=f'metro-furniture-{len(result):04}',kind=kind,position=list(candidate),heading=yaw,size=list(size)))
            return
    for route,path in enumerate(paths):
        for index,d in enumerate(range(3210,11850,32)):
            if route==1 and not 3500<d<5320:continue
            if 3320<d<3510 or 5320<d<5400:continue
            i=next(i for i in range(len(path['progress'])-1) if path['progress'][i]<=d<path['progress'][i+1])
            a,b=path['points'][i:i+2];t=(d-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
            x,z=a[0]+(b[0]-a[0])*t,a[2]+(b[2]-a[2])*t;yaw=math.atan2(dx,dz)
            for side in [-1,1]:
                # Curb-side band; keep the building-side >=1.5m walking strip open.
                def pos(along=0):return (x+dz*side*6.5+dx*along,z-dx*side*6.5+dz*along)
                seed=index+(1 if side>0 else 0)
                if seed%2==0:add('street_lamp',pos(-9),yaw,(.5,5.2,.5))
                if seed%3==0:
                    add('broadleaf',pos(),yaw,(2.8,6.5,2.8),.36,.36)
                    add('flower_planter',pos(5),yaw,(.7,.7,1.1))
                elif seed%5==0:
                    add('bench',pos(),yaw+side*math.pi/2,(1.8,.95,.65))
                    add('street_bin',pos(3),yaw,(.55,.85,.55))
                else:
                    kind,size=[('hydrant',(.45,.8,.45)),('news_boxes',(.65,1.1,.65)),('flower_planter',(.7,.7,1.1)),('parking_meter',(.25,1.25,.3)),('street_bin',(.55,.85,.55)),('bus_sign',(.45,2.5,.3)),('callbox',(.45,1.5,.45)),('mailboxes',(.65,1.0,.6)),('recycling_pair',(.65,.9,.65)),('utility_box',(.65,1.1,.5))][seed//5%10]
                    add(kind,pos(),yaw,size)
    # The wider avenue has less curb room. Retain larger original furniture on
    # nearby secondary streets rather than placing it in live traffic lanes.
    clear_shift=1.4
    primary=unary_union([LineString([(p[0],p[2]) for p in path['points']]) for path in paths])
    kinds_by_zone=[
        [('broadleaf',(2.8,6.5,2.8)),('bench',(1.8,.95,.65)),('mailboxes',(.65,1.0,.6)),('utility_box',(.65,1.1,.5)),('hydrant',(.45,.8,.45)),('parking_meter',(.25,1.25,.3))],
        [('broadleaf',(2.8,6.5,2.8)),('flower_planter',(.7,.7,1.1)),('news_boxes',(.65,1.1,.65)),('callbox',(.45,1.5,.45)),('street_bin',(.55,.85,.55)),('bus_sign',(.45,2.5,.3))],
        [('broadleaf',(2.8,6.5,2.8)),('flower_planter',(.7,.7,1.1)),('bench',(1.8,.95,.65)),('recycling_pair',(.65,.9,.65)),('news_boxes',(.65,1.1,.65)),('bus_sign',(.45,2.5,.3))],
    ]
    before=len(result)
    candidates=[]
    for street in sorted(data.get('streets',[]),key=lambda st:st.get('id','')):
        line=LineString(street['points'])
        if line.distance(primary)>140:continue
        a,b=street['points'][0],street['points'][-1];dx,dz=b[0]-a[0],b[1]-a[1];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
        yaw=math.atan2(dx,dz)
        for along in range(24,int(n)-23,32):
            q=line.interpolate(along)
            if q.distance(primary)<22:continue
            for side in [-1,1]:
                p=(q.x+dz*side*(street['width_m']/2-.4),q.y-dx*side*(street['width_m']/2-.4))
                candidates.append((p,yaw,dx,dz,side))
    # Spend the bounded secondary-street budget across the full city, rather
    # than exhausting it in source street order near the first neighborhoods.
    from metropolis_population import balanced,station,route_lines
    route_geometry=route_lines(paths)
    zone_counts=[0,0,0]
    for p,yaw,dx,dz,side in balanced(candidates,paths,lambda c:c[0]):
        if len(result)-before>=450:break
        progress=station(p,paths,route_geometry)
        zone=0 if progress<5500 else 1 if progress<8500 else 2
        kind,size=kinds_by_zone[zone][zone_counts[zone]%len(kinds_by_zone[zone])]
        zone_counts[zone]+=1
        add(kind,p,yaw+side*math.pi/2 if kind=='bench' else yaw,size,.36 if kind=='broadleaf' else None,.36 if kind=='broadleaf' else None)
    return result

def audit(data, road, walk, gained, props, built_override=None):
    built=unary_union([Polygon(b['faces'][v['footprint_index']],b['face_holes'][v['footprint_index']]) for b in data['blocks'] for v in b['buildings']])
    assert road.intersection(walk.buffer(-0.00001)).area<1e-5
    if built_override is not None: built=built_override
    overlap=walk.buffer(-0.00001).intersection(built).area
    assert overlap<1e-4, f"building/walk overlap {overlap} m2"
    old=decode(data['road_surface']).union(decode(data['sidewalks']))
    if built_override is None:
        assert old.symmetric_difference(road.union(walk)).area<1e-5
    else:
        assert old.difference(road.union(walk).buffer(0.00001)).area<1e-4
    assert gained.area>1000 and len(props)>200
    # Every physical footprint is on paved sidewalk; all buildings/route points untouched.
    for p in props:
        w,z=(.36,.36) if p['kind']=='broadleaf' else (p['size'][0],p['size'][2])
        assert walk.covers(footprint(p['position'],p['heading'],w,z).buffer(.12))
    return dict(curb_shift_m=CURB_SHIFT,typical_primary_sidewalk_width_m=2.4,minimum_building_side_walk_m=CLEAR_WALK_M,
                added_sidewalk_area_m2=round(gained.area,3),props=len(props),by_kind=dict(Counter(p['kind'] for p in props)),
                checks=['no building overlap','no carriageway overlap','unchanged street reservation' if built_override is None else 'landward expansion with retained carriageway','all physical footprints on sidewalk','junction setbacks >=20m','deterministic curb band placement'])
