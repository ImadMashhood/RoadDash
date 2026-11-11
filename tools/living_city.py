"""Original civic sites, gradual rural infill and widened sidewalk authoring.
Derived from approved farm-to-city source; source route and legacy maps retained.
"""
import math,json
from pathlib import Path
from shapely import Polygon,Point,LineString,box,make_valid,set_precision
from shapely.ops import unary_union
from shapely.affinity import scale
ROOT=Path(__file__).resolve().parents[1]
CONFIG=json.loads((ROOT/'content/world/living_city.json').read_text())

def shape_polygon(shape):
    result=Polygon()
    for ring in shape['rings']: result=result.symmetric_difference(make_valid(Polygon(ring)))
    return result

def widen(road,walk,obstacles):
    # Add on the land side, never steal a driving lane. A uniform offset grows
    # corridor width; area target handles curved junctions and irregular blocks.
    lo,hi=0.,3.
    target=walk.area*CONFIG['sidewalk_width_scale']
    for _ in range(24):
        mid=(lo+hi)/2
        candidate=walk.buffer(mid,join_style='mitre').difference(road).difference(obstacles)
        if candidate.area<target:lo=mid
        else:hi=mid
    result=walk.buffer(hi,join_style='mitre').difference(road).difference(obstacles)
    return result,hi

def augment(data,paths,records,road,walk,triangulate):
    from street_first_layout import polygons
    from metropolis_streetscape import footprint
    from metropolis_population import station,route_lines
    from metropolis_elevation import height
    def polygon(shape):
        result=Polygon()
        for ring in shape['rings']: result=result.symmetric_difference(make_valid(Polygon(ring)))
        return result
    originals=[polygon(r['shape']) for r in records]
    # Reserve central 81% footprint islands so no building identity disappears.
    protected=[]
    for p in originals:
        core=p.intersection(scale(p,.9,.9,origin='centroid'))
        protected.append(core if core.area>=p.area*.8 else p)
    cores=unary_union(protected)
    report={};old_area=walk.area;walk,offset=widen(road,walk,cores)
    walk=set_precision(walk,1e-6)
    # Avoid disconnected building fragments by retaining the component containing
    # the original representative point and returning tiny scraps to the walk.
    updated=[]
    for r,poly in zip(records,originals):
        clipped=poly.difference(walk)
        parts=list(polygons(clipped))
        assert parts,r['id']
        assert clipped.area>poly.area*.65,(r['id'],clipped.area/poly.area)
        try:r['shape']=triangulate(clipped)
        except AssertionError as exc:raise AssertionError(f"failed sidewalk frontage triangulation: {r['id']}") from exc
        updated.append(clipped)
    report.update(sidewalk_area_before=old_area,sidewalk_area_after=walk.area,landward_offset_m=offset,width_area_ratio=walk.area/old_area,setback_buildings=sum(a.area-b.area>.001 for a,b in zip(originals,updated)))
    obstacles=unary_union(updated)
    sites=[]
    for ident,kind in [(CONFIG['city_hall_site'],'city_hall'),(CONFIG['park_site'],'park')]:
        site=next(s for s in data['blocks'] if s['id']==ident)
        assert site['role']=='park'
        boundary=Polygon(site['polygon'],site['holes'])
        p=[270.,3960.] if kind=='city_hall' else [425.,6480.]
        heading=0.
        if kind=='city_hall':
            from metropolis_street_life import traffic_paths
            race=LineString([(v[0],v[2]) for v in traffic_paths(paths,data)[0]['points']])
            front=race.interpolate(race.project(Point(p)))
            heading=math.atan2(front.x-p[0],front.y-p[1])
        size=[300.,145.] if kind=='city_hall' else [100.,310.]
        if kind=='city_hall':
            assert boundary.covers(footprint(p,heading,62*2.8,46*2.8))
        grounds=boundary.difference(walk).difference(road)
        patches=[]
        x0,z0,x1,z1=grounds.bounds
        for gx in range(math.floor(x0/32)*32,math.ceil(x1/32)*32,32):
            for gz in range(math.floor(z0/32)*32,math.ceil(z1/32)*32,32):
                for tile in polygons(grounds.intersection(box(gx,gz,gx+32,gz+32))):
                    if tile.area>.001:patches.append(triangulate(tile))
        sites.append(dict(id='civic-'+kind,kind=kind,position=p,heading=heading,size=size,grounds=patches))

    from civic_neighborhood import furnish
    report['civic_context']=[]
    for civic, ident in zip(sites,[CONFIG['city_hall_site'],CONFIG['park_site']]):
        block=next(b for b in data['blocks'] if b['id']==ident)
        report['civic_context'].append(furnish(civic,Polygon(block['polygon'],block['holes']),walk,road,records,triangulate))

    # Increasing density: sparse homes -> small workshops -> near-continuous edge.
    path=paths[0];line=LineString([(p[0],p[2]) for p in path['points']]);lines=route_lines(paths)
    additions=[]
    for n,s in enumerate([2100,2350,2530,2660,2760,2840,2910,2970,3020,3070,3120]):
        i=next(i for i in range(len(path['progress'])-1) if path['progress'][i]<=s<path['progress'][i+1]);a,b=path['points'][i:i+2];t=(s-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];mag=math.hypot(dx,dz);dx,dz=dx/mag,dz/mag
        for side in ([-1,1] if n>4 else [(-1)**n]):
            p=[a[0]+(b[0]-a[0])*t+dz*side*32,a[2]+(b[2]-a[2])*t-dx*side*32]
            shape=footprint(p,math.atan2(dx,dz),14 if n<6 else 19,12)
            if shape.intersects(road.buffer(3)) or shape.intersects(walk) or shape.intersects(obstacles):continue
            record=dict(id=f'approach-{n:02}-{side}',shape=triangulate(shape),height=6.4 if n<7 else 9.6,floors=2 if n<7 else 3,tier=1,style=[8,9,0,10][n%4])
            additions.append(record)
    records.extend(additions);report['approach_buildings']=len(additions)
    # Cafe patios attach to real building fronts. Keep a parallel 1.5m clear
    # public walking strip; furniture/roof use only the private inset frontage.
    for r in records:
        if len([s for s in sites if s['kind']=='cafe'])>=CONFIG['cafe_count']:break
        if r['height']>22:continue
        ring=r['shape']['rings'][0]
        for a,b in zip(ring,ring[1:]+ring[:1]):
            dx,dz=b[0]-a[0],b[1]-a[1];length=math.hypot(dx,dz)
            if length<10:continue
            dx,dz=dx/length,dz/length;mid=[(a[0]+b[0])/2,(a[1]+b[1])/2]
            for sign in [-1,1]:
                p=[mid[0]+dz*sign*0.8,mid[1]-dx*sign*0.8];yaw=math.atan2(dx,dz)
                patio=footprint(p,yaw,1.3,7.)
                if not walk.covers(patio) or line.distance(Point(p))>35:continue
                if any(math.dist(p,s['position'])<140 for s in sites):continue
                # Leave a continuous strip outward from the patio.
                clear=footprint([p[0]+dz*sign*1.45,p[1]-dx*sign*1.45],yaw,1.6,8.)
                if not walk.covers(clear):continue
                sites.append(dict(id=f'cafe-{len(sites):02}',kind='cafe',position=p,heading=yaw+(math.pi if sign<0 else 0),size=[1.3,7.]));break
            else:continue
            break
    report['sites']=sites
    (ROOT/'docs/testing/civic-greenery/authoring.json').write_text(json.dumps(report,indent=2)+'\n')
    return walk,sites
