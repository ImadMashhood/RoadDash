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
    shapely.prepare(road)
    junctions=[Point(j['point']) for j in data['race_first']['primary_junctions']]
    result=[]
    for route,path in enumerate(paths):
        for d in range(3260,11800,42):
            if route==1 and not 3500<d<5320:continue
            section=next(s for s in cfg['lane_sections'] if s['route']==route and s['start']<=d<s['end'])
            if section['outbound']+section['inbound']!=2 or d<section['start']+110 or d>section['end']-110:continue
            i=next(i for i in range(len(path['progress'])-1) if path['progress'][i]<=d<path['progress'][i+1]);a,b=path['points'][i:i+2];t=(d-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
            for side in [-1,1]:
                p=[a[0]+(b[0]-a[0])*t+dz*side*3.95,a[2]+(b[2]-a[2])*t-dx*side*3.95];yaw=math.atan2(dx,dz)+(math.pi if side<0 else 0)
                shape=footprint(p,yaw,1.8,3.8).buffer(.1)
                if not road.covers(shape) or any(shape.distance(j)<25 for j in junctions):continue
                result.append(dict(id=f'metro-parked-{len(result):03}',position=p,heading=yaw))
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
            q=line.interpolate(line.project(Point(p[0],p[2]))) if s>=3150 else Point(p[0],p[2])
            v=[q.x,p[1],q.y]
            if points and math.dist(v,points[-1])<.001:continue
            points.append(v);progress.append(s)
        assert progress[0]==0 and progress[-1]==12000
        result.append(dict(points=points,progress=progress))
    return result
