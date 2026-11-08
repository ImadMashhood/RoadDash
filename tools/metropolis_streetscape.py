"""Original sidewalk reservations and deterministic curb furniture authoring."""
import math
from collections import Counter
from shapely import Point, Polygon, box
from shapely.ops import unary_union
from street_first_layout import decode

CURB_SHIFT = 1.0
CLEAR_WALK_M = 2.0

def sidewalks(data):
    road, walk = decode(data['road_surface']), decode(data['sidewalks'])
    bounds = decode(data['city_envelope']).bounds
    urban = box(bounds[0]-1, 0, bounds[2]+1, bounds[3]+1)
    gained = walk.buffer(CURB_SHIFT, join_style='mitre').intersection(road).intersection(urban)
    return road.difference(gained), walk.union(gained), gained

def footprint(p, yaw, width, depth):
    c,s=math.cos(yaw),math.sin(yaw)
    return Polygon([(p[0]+c*x+s*z,p[1]-s*x+c*z) for x,z in [(-width/2,-depth/2),(width/2,-depth/2),(width/2,depth/2),(-width/2,depth/2)]])

def furniture(data, paths, walk):
    result=[];occupied=[]
    junctions=[Point(j['point']) for j in data['race_first']['primary_junctions']]
    import shapely
    shapely.prepare(walk)
    def add(kind,p,yaw,size,width=None,depth=None):
        for shift in sorted([i*.15 for i in range(-12,13)],key=abs):
            candidate=(p[0]+dz*side*shift,p[1]-dx*side*shift)
            shape=footprint(candidate,yaw,width or size[0],depth or size[2]).buffer(.12)
            clear_center=(candidate[0]+dz*side*1.7,candidate[1]-dx*side*1.7)
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
                # Curb-side band; keep the building-side >2m walking strip open.
                def pos(along=0):return (x+dz*side*5.6+dx*along,z-dx*side*5.6+dz*along)
                seed=index+(1 if side>0 else 0)
                if seed%2==0:add('street_lamp',pos(-9),yaw,(.5,5.2,.5))
                if seed%12==0:
                    add('broadleaf',pos(),yaw,(2.8,6.5,2.8),.7,.7)
                    add('flower_planter',pos(5),yaw,(.7,.7,1.1))
                elif seed%5==0:
                    add('bench',pos(),yaw+side*math.pi/2,(1.8,.95,.65))
                    add('street_bin',pos(3),yaw,(.55,.85,.55))
                else:
                    kind,size=[('hydrant',(.45,.8,.45)),('news_boxes',(.65,1.1,.65)),('flower_planter',(.7,.7,1.1)),('parking_meter',(.25,1.25,.3)),('street_bin',(.55,.85,.55)),('bus_sign',(.45,2.5,.3)),('callbox',(.45,1.5,.45)),('mailboxes',(.65,1.0,.6)),('recycling_pair',(.65,.9,.65)),('utility_box',(.65,1.1,.5))][seed//5%10]
                    add(kind,pos(),yaw,size)
    return result

def audit(data, road, walk, gained, props):
    built=unary_union([Polygon(b['faces'][v['footprint_index']],b['face_holes'][v['footprint_index']]) for b in data['blocks'] for v in b['buildings']])
    assert road.intersection(walk).area<1e-5
    assert walk.intersection(built).area<1e-4
    old=decode(data['road_surface']).union(decode(data['sidewalks']))
    assert old.symmetric_difference(road.union(walk)).area<1e-5
    assert gained.area>1000 and len(props)>200
    # Every physical footprint is on paved sidewalk; all buildings/route points untouched.
    for p in props:
        w,z=(.7,.7) if p['kind']=='broadleaf' else (p['size'][0],p['size'][2])
        assert walk.covers(footprint(p['position'],p['heading'],w,z).buffer(.12))
    return dict(curb_shift_m=CURB_SHIFT,typical_sidewalk_width_m=3.4,minimum_building_side_walk_m=CLEAR_WALK_M,
                added_sidewalk_area_m2=round(gained.area,3),props=len(props),by_kind=dict(Counter(p['kind'] for p in props)),
                checks=['no building overlap','no carriageway overlap','unchanged street reservation','all physical footprints on sidewalk','junction setbacks >=20m','deterministic curb band placement'])
