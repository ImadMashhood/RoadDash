"""Deterministic per-building height changes and parked street activity; no footprint edits."""
import math,hashlib
from shapely import Point
from metropolis_streetscape import footprint

def _raw_floors(building,block,cfg):
    if building['role']=='interior_tower':return building['floors']
    h=int.from_bytes(hashlib.blake2b(building['id'].encode(),digest_size=8).digest(),'little')
    r=h%100;total=0
    for band in cfg['height_bands']:
        total+=band['weight']
        if r<total:
            high=band['max']+int(block['core_score']*band['downtown_extra'])
            return band['min']+(h//100)%(high-band['min']+1)
    raise AssertionError('height weights')

def varied_floors(building,block,cfg):
    floors=_raw_floors(building,block,cfg)
    if 'center' not in block:return floors
    # The approved city grows northward from the farm boundary. Keep every lot,
    # including interior towers, low at the edge; grow the skyline over1.8km.
    t=max(0.,min(1.,(block['center'][1]+200.)/1800.))
    cap=2+int(30*t*t*(3-2*t))
    return min(floors,cap)

def parked_cars(data,paths,road,cfg):
    import shapely
    # Parking bays may reclaim the curb overlay inside the original road ROW.
    from street_first_layout import decode
    if "road_surface" in data: road=decode(data["road_surface"])
    shapely.prepare(road)
    junctions=[Point(j['point']) for j in data['race_first']['primary_junctions']]
    result=[]
    for route,path in enumerate(paths):
        for d in range(3260,11800,42):
            if route==1 and not 3500<d<5320:continue
            section=next(s for s in cfg['lane_sections'] if s['route']==route and s['start']<=d<s['end'])
            if section['end']<=3350 or section['outbound']+section['inbound']!=2 or d<section['start']+110 or d>section['end']-110:continue
            i=next(i for i in range(len(path['progress'])-1) if path['progress'][i]<=d<path['progress'][i+1]);a,b=path['points'][i:i+2];t=(d-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
            for side in [-1,1]:
                p=[a[0]+(b[0]-a[0])*t+dz*side*3.95,a[2]+(b[2]-a[2])*t-dx*side*3.95];yaw=math.atan2(dx,dz)+(math.pi if side<0 else 0)
                shape=footprint(p,yaw,1.8,3.8).buffer(.1)
                if not road.covers(shape) or any(shape.distance(j)<25 for j in junctions):continue
                result.append(dict(id=f'metro-parked-{len(result):03}',position=p,heading=yaw))
    # No curb parking in the new live 4/3-lane avenue. Keep original parked
    # vehicles on wide secondary streets with a full two-lane centre remaining.
    from shapely import LineString
    primary=shapely.union_all([LineString([(p[0],p[2]) for p in path['points']]) for path in paths])
    candidates=[]
    for street in data['streets']:
        if street['width_m']<8:continue
        line=LineString(street['points']);a,b=street['points'][0],street['points'][-1]
        dx,dz=b[0]-a[0],b[1]-a[1];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
        for along in range(32,int(n)-31,42):
            q=line.interpolate(along)
            if q.distance(primary)<22 or q.distance(primary)>140:continue
            # Eight-metre side streets retain one five-metre clear channel;
            # never park inside a live cross-traffic sweep near the race junction.
            narrow=street['width_m']<12
            if narrow and any(q.distance(j)<82 for j in junctions):continue
            for side in ([1] if narrow else [-1,1]):
                offset=min(street['width_m']/2-1.05,3.95)
                p=[q.x+dz*side*offset,q.y-dx*side*offset];yaw=math.atan2(dx,dz)+(math.pi if side<0 else 0)
                shape=footprint(p,yaw,1.8,3.8).buffer(.1)
                if not road.covers(shape) or any(shape.distance(j)<25 for j in junctions):continue
                candidates.append(dict(position=p,heading=yaw))
    from metropolis_population import balanced
    for c in balanced(candidates,paths,lambda c:c['position']):
        if len(result)>=128:break
        shape=footprint(c['position'],c['heading'],1.8,3.8).buffer(.1)
        if any(shape.distance(footprint(v['position'],v['heading'],1.8,3.8))<1 for v in result):continue
        result.append(dict(id=f'metro-parked-{len(result):03}',**c))
    return result

def traffic_paths(paths,data=None):
    from shapely import LineString
    result=[]
    for route,path in enumerate(paths):
        line=LineString([(p[0],p[2]) for p in path['points']]).simplify(2.0)
        points=[];progress=[]
        for p,s in zip(path['points'],path['progress']):
            if data is not None:
                name='farm_approach' if s<=3350 else ('old_market' if route==0 else 'station_avenue') if s<=5362.5 else 'long_city'
                e=next(e for e in data['edges'] if e['id']==name)
                line=LineString([(v[0],v[2]) for v in e['points']]).simplify(2.0)
            # Join the reserved street centre before the narrow urban entry, while
            # still on the wide farm pavement. Avoid a lateral snap at3150m.
            original=Point(p[0],p[2]);projected=line.interpolate(line.project(original))
            t=max(0.,min(1.,(s-2800.)/100.));t=t*t*(3.-2.*t)
            q=Point(original.x+(projected.x-original.x)*t,original.y+(projected.y-original.y)*t)
            from metropolis_elevation import height
            v=[q.x,height(q.x,q.y),q.y]
            if points and math.dist(v,points[-1])<.001:continue
            points.append(v);progress.append(s)
        assert progress[0]==0 and progress[-1]==12000
        result.append(dict(points=points,progress=progress))
    return result
