"""Original civic context and pavement-connected public garden authoring."""
import math
from shapely import Point, LineString, Polygon, box
from shapely.ops import unary_union, nearest_points
from metropolis_streetscape import footprint
from street_first_layout import polygons

def tiles(shape, triangulate):
    result=[]
    if shape.is_empty:return result
    a,b,c,d=shape.bounds
    for x in range(math.floor(a/16)*16,math.ceil(c/16)*16,16):
        for z in range(math.floor(b/16)*16,math.ceil(d/16)*16,16):
            for p in polygons(shape.intersection(box(x,z,x+16,z+16))):
                if p.area>.001:result.append(triangulate(p))
    return result

def furnish(site, boundary, walk, road, records, triangulate):
    x,z=site['position'];h=site['heading'];sn,cs=math.sin(h),math.cos(h)
    world=lambda a,b:(x+cs*a+sn*b,z-sn*a+cs*b)
    buildings=[];entries=[]
    if site['kind']=='city_hall':
        original=(x,z)
        # Bring the entire frontage toward the driving street, including stairs.
        for distance in range(24,-1,-1):
            candidate=(original[0]+sn*distance,original[1]+cs*distance)
            body=footprint(candidate,h,174,130)
            stairs=footprint((candidate[0]+sn*70,candidate[1]+cs*70),h,74,16)
            if boundary.buffer(-3).covers(body.union(stairs)):
                x,z=candidate;break
        site['position']=[x,z]
        body=footprint((x,z),h,174,130)
        forecourt=footprint(world(0,77),h,90,24)
        entrance=nearest_points(Point(world(0,90)),walk)[1]
        entries=[list(entrance.coords[0])]
        paving=body.buffer(3).union(forecourt).union(LineString([world(0,80),entrance.coords[0]]).buffer(7))
        forbidden=body.buffer(12).union(paving.buffer(5))
        # Low-rise shops/civic offices frame the back and ends, retaining all old IDs.
        xmin,zmin,xmax,zmax=boundary.bounds
        for gz in range(math.ceil((zmin+18)/28)*28,int(zmax-12),28):
            for gx in range(math.ceil((xmin+14)/26)*26,int(xmax-12),26):
                lot=footprint((gx,gz),0,18,20)
                if not boundary.buffer(-5).covers(lot) or lot.intersects(forbidden):continue
                if lot.intersects(walk) or lot.intersects(road):continue
                ident=f'civic-context-{len(buildings):02}'
                buildings.append(lot)
                records.append(dict(id=ident,shape=triangulate(lot),height=9.6 if len(buildings)%3 else 12.8,floors=3 if len(buildings)%3 else 4,tier=1,style=[8,9,0,10][len(buildings)%4]))
                forbidden=forbidden.union(lot.buffer(4))
        assert len(buildings)>=4, len(buildings)
        allowed=boundary.difference(walk).difference(road).difference(unary_union(buildings))
        trees=[]
        for gx in range(math.ceil(xmin/12)*12,int(xmax),12):
            for gz in range(math.ceil(zmin/12)*12,int(zmax),12):
                p=Point(gx,gz)
                if allowed.covers(p.buffer(3)) and not paving.buffer(3).intersects(p.buffer(3)):trees.append([gx,gz])
        site['trees']=trees
    else:
        # Six real sidewalk connections, extending both north/south promenades
        # and the east/west path to the irregular public curb boundary.
        lines=[]
        for start,direction in [(world(0,0),(0,1)),(world(0,0),(0,-1)),(world(65,0),(0,1)),(world(65,0),(0,-1)),(world(0,0),(1,0)),(world(0,0),(-1,0))]:
            ray=LineString([start,(start[0]+direction[0]*500,start[1]+direction[1]*500)])
            clipped=ray.intersection(boundary)
            endpoint=max(list(clipped.coords),key=lambda p:Point(p).distance(Point(start)))
            sidewalk=nearest_points(Point(endpoint),walk)[1]
            entries.append(list(sidewalk.coords[0]))
            lines.append(LineString([start,sidewalk.coords[0]]))
        paving=unary_union(lines).buffer(2.5,cap_style='square').union(Point(site['position']).buffer(12)).difference(Point(site['position']).buffer(8.7))
        allowed=boundary.difference(road).difference(walk)
    site['grounds']=tiles(allowed,triangulate)
    site['paving']=tiles(paving.intersection(boundary.union(walk)).difference(road),triangulate)
    site['entrances']=entries
    site['context_buildings']=len(buildings)
    return dict(kind=site['kind'],position=site['position'],context_buildings=len(buildings),trees=len(site.get('trees',[])),entrances=entries,grass_area=allowed.difference(paving).area,paving_area=paving.intersection(allowed).area)
