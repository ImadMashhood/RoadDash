"""Bounded full-route street life and capsule-clear pedestrian patrol authoring."""
import math
from collections import defaultdict
from shapely import LineString, Point
from shapely.ops import unary_union
from metropolis_streetscape import footprint


def route_lines(paths):
    return [LineString([(p[0], p[2]) for p in path['points']]) for path in paths]


def station(point, paths, lines):
    p=Point(point);r=min(range(len(lines)),key=lambda i:lines[i].distance(p))
    # Geometry length is not logical race progress; interpolate the actual source.
    line=lines[r];along=line.project(p);total=0.
    for i,(a,b) in enumerate(zip(paths[r]['points'],paths[r]['points'][1:])):
        length=math.dist((a[0],a[2]),(b[0],b[2]))
        if along<=total+length or i==len(paths[r]['points'])-2:
            t=max(0.,min(1.,(along-total)/max(length,1e-6)))
            return paths[r]['progress'][i]*(1-t)+paths[r]['progress'][i+1]*t
        total+=length
    raise AssertionError('empty path')


def balanced(items, paths, position):
    """Round robin deterministic 500m districts; no first-street budget bias."""
    lines=route_lines(paths);buckets=defaultdict(list)
    for item in items:buckets[int(station(position(item),paths,lines)//500)].append(item)
    for i in range(max((len(b) for b in buckets.values()),default=0)):
        for key in sorted(buckets):
            if i<len(buckets[key]):yield buckets[key][i]


def pedestrians(data,paths,walk,props,budget=384):
    """Bake the whole traversed capsule corridor, not only its starting point."""
    assert 0 <= budget <= 384
    import shapely
    occupied=unary_union([footprint(p['position'],p['heading'],.36 if p['kind']=='broadleaf' else p['size'][0],.36 if p['kind']=='broadleaf' else p['size'][2]).buffer(.2) for p in props])
    safe=walk.buffer(-.42).difference(occupied.buffer(.42))
    shapely.prepare(safe)
    junctions=[Point(j['point']) for j in data['race_first']['primary_junctions']]
    candidates=[]
    for route,path in enumerate(paths):
        for d in range(3240,11850,20):
            if route==1 and not 3500<d<5320:continue
            i=next(i for i in range(len(path['progress'])-1) if path['progress'][i]<=d<path['progress'][i+1])
            a,b=path['points'][i:i+2];t=(d-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
            x,z=a[0]+(b[0]-a[0])*t,a[2]+(b[2]-a[2])*t
            for side in [-1,1]:
                accepted=False
                for offset in [7.4,7.7,8.,8.3,8.6,9.,9.4,10.,10.6,11.2,12.]:
                    p=[x+dz*side*offset,z-dx*side*offset]
                    for extent in [9.,6.,4.]:
                        corridor=LineString([(p[0]-dx*extent,p[1]-dz*extent),(p[0]+dx*extent,p[1]+dz*extent)])
                        if not safe.covers(corridor) or any(corridor.distance(j)<16 for j in junctions):continue
                        candidates.append(dict(position=p,heading=math.atan2(dx,dz),extent=extent,route=route,station=d))
                        accepted=True;break
                    if accepted:break
    result=[]
    for p in balanced(candidates,paths,lambda c:c['position']):
        if len(result)>=budget:break
        if any(math.dist(p['position'],a['position'])<10 for a in result):continue
        result.append(dict(id=f'metro-walker-{len(result):03}',**p))
    return result


def crossing_routes(walkers,road,walk,props,budget=24):
    """Occasional full sidewalk-to-sidewalk capsule corridors, never through props."""
    assert 0 <= budget <= 24
    occupied=unary_union([footprint(p['position'],p['heading'],p['size'][0],p['size'][2]).buffer(.5) for p in props])
    support=road.union(walk)
    endpoints=walk.buffer(-.42)
    picked=[]
    # Use existing authored sidewalk starts; keep the counterpart as a normal walker.
    for a in walkers:
        if len(picked)>=budget:break
        if any(abs(a['station']-s)<240 for s in picked):continue
        for b in walkers:
            if a is b or a['route']!=b['route'] or a['station']!=b['station']:continue
            line=LineString([a['position'],b['position']])
            if not 12<=line.length<=26:continue
            if not endpoints.covers(Point(a['position'])) or not endpoints.covers(Point(b['position'])):continue
            corridor=line.buffer(.42,cap_style='round')
            if not support.covers(corridor) or corridor.intersects(occupied):continue
            if line.intersection(road).length<6:continue
            a['crossing_end']=b['position'][:]
            picked.append(a['station']);break
    return picked
