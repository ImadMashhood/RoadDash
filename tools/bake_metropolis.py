"""Compile the approved layout into runtime polygons/triangles and original facade tiles."""
import hashlib,json,math
from pathlib import Path
import shapely
from shapely import Polygon, box
from shapely.geometry.polygon import orient
from PIL import Image
from street_first_layout import decode,polygons
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'content/world/metropolis.json'

def triangulate(poly):
    poly=shapely.make_valid(poly)
    clean=shapely.set_precision(poly,1e-6).simplify(1e-6,preserve_topology=True)
    assert abs(clean.area-poly.area)<.01
    rings=[];vertices=[];triangles=[]
    for part in polygons(clean):
        part=orient(part,sign=1)
        rr=[list(part.exterior.coords)[:-1]]+[list(r.coords)[:-1] for r in part.interiors]
        rings.extend(rr);vertices.extend(p for r in rr for p in r)
        lookup={p:i for i,p in enumerate(vertices)}
        for t in shapely.constrained_delaunay_triangles(part).geoms:
            assert part.covers(t.representative_point())
            triangles.append([lookup[p] for p in list(orient(t,sign=1).exterior.coords)[:3]])
    assert triangles
    assert abs(sum(Polygon([vertices[i] for i in t]).area for t in triangles)-clean.area)<1e-5
    return dict(rings=rings,triangles=triangles)

def run():
    source=ROOT/'docs/planning/farms-long-city/LAYOUT.json';d=json.loads(source.read_text())
    from metropolis_street_life import varied_floors, parked_cars, traffic_paths
    life=json.loads((ROOT/'content/world/metropolis_street_life.json').read_text())
    records=[]
    for b in d['blocks']:
        for building in b['buildings']:
            i=building['footprint_index'];p=Polygon(b['faces'][i],b['face_holes'][i])
            floors=varied_floors(building,b,life)
            records.append(dict(id=building['id'],shape=triangulate(p),height=round(floors*life['floor_height'],2),
                                floors=floors,tier=['hero','standard','skyline','massing'].index(building['detail_tier']),
                                style=int.from_bytes(hashlib.blake2b(building['id'].encode(),digest_size=4).digest(),'little')%12))
    edges={e['id']:e for e in d['edges']};paths=[]
    for choice in ['old_market','station_avenue']:
        points=[];progress=[]
        for ident in ['farm_approach',choice,'long_city']:
            e=edges[ident];ds=[0]
            for a,b in zip(e['points'],e['points'][1:]):ds.append(ds[-1]+math.dist(a,b))
            lo,hi=e['logical_chainage_m']
            for j,p in enumerate(e['points']):
                if points and j==0:continue
                points.append(p);progress.append(lo+(hi-lo)*ds[j]/ds[-1])
        paths.append(dict(points=points,progress=progress))
    cp=[]
    for i in range(0,len(paths[0]['points']),max(1,len(paths[0]['points'])//125)):
        p=paths[0]['points'][i];s=paths[0]['progress'][i]
        if cp and math.dist(p,cp[-1]['position'])<20:continue
        cp.append(dict(position=p,half_width=5,bank_degrees=0,sector='farms' if s<3150 else 'city',surface='asphalt'))
    if math.dist(cp[-1]['position'],paths[0]['points'][-1])<20:cp.pop()
    cp.append(dict(position=paths[0]['points'][-1],half_width=5,bank_degrees=0,sector='city',surface='asphalt'))
    gates=sorted(set([0,1000,2200,3150,3350,3900,4500,5000,5362.5,6500,8000,9500,11000,12000]))
    track=dict(schema_version=1,id='metro-run-01',name='Metropolis — Farms to Downtown / two routes',closed=False,scenery_seed=67201,
               control_points=cp,checkpoints=[dict(id=f'metro-gate-{i:02}',fraction=s/12000) for i,s in enumerate(gates)],
               spawns=[dict(id=f'metro-grid-{i:02}',distance=-8-(14-i)//2*20,lane=-2 if i%2 else 2) for i in range(15)],props=[],verge_width=2.0)
    ground_bounds=decode(d['city_envelope']).bounds
    field_bounds=[Polygon(f['polygon']).bounds for f in d['fields']]
    bounds=[min(x[0] for x in [ground_bounds]+field_bounds)-100,min(x[1] for x in [ground_bounds]+field_bounds)-180,
            max(x[2] for x in [ground_bounds]+field_bounds)+100,max(x[3] for x in [ground_bounds]+field_bounds)+100]
    from metropolis_streetscape import sidewalks, furniture, audit
    road, walk, gained = sidewalks(d)
    street_props = furniture(d, paths, walk)
    sidewalk_report = audit(d, road, walk, gained, street_props)
    parked=parked_cars(d,paths,road,life)
    surfaces=[]
    # Clip to short tiles before triangulation: pavement follows the graded height field.
    courts=[]
    from shapely.ops import unary_union
    for b in d['blocks']:
        if b['role'] in ['park','traffic_island']:continue
        land=Polygon(b['polygon'],b['holes'])
        built=unary_union([Polygon(p,h) for p,h in zip(b['faces'],b['face_holes'])])
        courts.extend(polygons(land.difference(built)))
    for kind,name in [(0,'road_surface'),(1,'sidewalks'),(2,None)]:
        geometry=unary_union(courts) if name is None else road if kind==0 else walk;xmin,zmin,xmax,zmax=geometry.bounds
        geometry=shapely.set_precision(geometry,1e-6)
        shapely.prepare(geometry)
        for x in range(math.floor(xmin/32)*32,math.ceil(xmax/32)*32,32):
            for z in range(math.floor(zmin/32)*32,math.ceil(zmax/32)*32,32):
                if not geometry.intersects(box(x,z,x+32,z+32)):continue
                for p in polygons(shapely.clip_by_rect(geometry,x,z,x+32,z+32)):
                    if p.area>.0001:
                        if not p.is_valid:p=shapely.make_valid(p)
                        for part in polygons(p):surfaces.append(dict(kind=kind,shape=triangulate(part)))
    # Exact farm pavement includes the complete starting grid, not a city-only preview.
    farm=edges['farm_approach']['points']
    from shapely import LineString
    farm_road=LineString([(p[0],p[2]) for p in farm]).buffer(5,cap_style='square',join_style='mitre')
    farm_road=farm_road.difference(decode(d['road_surface']))
    xmin,zmin,xmax,zmax=farm_road.bounds
    for x in range(math.floor(xmin/32)*32,math.ceil(xmax/32)*32,32):
        for z in range(math.floor(zmin/32)*32,math.ceil(zmax/32)*32,32):
            for p in polygons(shapely.clip_by_rect(farm_road,x,z,x+32,z+32)):
                if p.area>.001:surfaces.append(dict(kind=0,shape=triangulate(p)))
    # Grid apron continues the first source segment backwards.
    a,b=paths[0]['points'][:2];dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz)
    apron=LineString([(a[0]-dx/n*180,a[2]-dz/n*180),(a[0],a[2])]).buffer(5,cap_style='square')
    for p in polygons(apron):surfaces.append(dict(kind=0,shape=triangulate(p)))
    manifest=dict(schema_version=1,source_layout_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),track=track,paths=paths,
                  bounds=bounds,buildings=records,surfaces=surfaces,streets=d['streets'],
                  junctions=[dict(point=j['point'],street_id=j['secondary_street_ids'][0]) for j in d['race_first']['primary_junctions']],
                  traffic_paths=traffic_paths(paths,d),street_life=life,parked_cars=parked,street_props=street_props,fields=[dict(id=f['id'],shape=triangulate(Polygon(f['polygon'])),crop=f['crop']) for f in d['fields']])
    (ROOT/'docs/testing/metropolis-sidewalks').mkdir(exist_ok=True)
    (ROOT/'docs/testing/metropolis-sidewalks/authoring.json').write_text(json.dumps(sidewalk_report,indent=2)+'\n')
    OUT.write_text(json.dumps(manifest,separators=(',',':'))+'\n')
    atlas_root=ROOT/'art/buildings/mesh_assets/far_facades';atlas=json.loads((atlas_root/'MANIFEST.json').read_text())
    designs=['BUSS1','BUSS4','CHINA12','CLSTGA11','OFFC1','OFFC2','OFFC10','RUNDWN11','VICT11','VICT12','WHSE11','WRHS1']
    data=bytearray();sources=[]
    for design in designs:
        for band in [0,1]:
            source_design='CLSTGA11' if design=='BUSS4' and band==1 else design
            tile=next(t for t in atlas['records'] if t['design']==source_design and t['band']==band)
            for level in range(5):
                image=Image.open(atlas_root/f'albedo-mip{level}.png').convert('RGBA');w,h=image.size
                lo=tile['uv_min'];side=96>>level
                x,y=round(lo[0]*w-.5),round(lo[1]*h-.5)
                crop=image.crop((x,y,x+side,y+side))
                back=Image.new('RGBA',crop.size,(135,126,108,255));back.alpha_composite(crop);data.extend(back.tobytes())
            sources.append(dict(design=design,source_design=source_design,band=band,source_sha256=tile['source_sha256']))
    dest=ROOT/'assets/metropolis';dest.mkdir(exist_ok=True);(dest/'facades.rgba').write_bytes(data)
    (dest/'PROVENANCE.json').write_text(json.dumps(dict(source_manifest_sha256=hashlib.sha256((atlas_root/'MANIFEST.json').read_bytes()).hexdigest(),
       sources=sources,modifications='Crop original model-derived storey albedo/mips; flatten transparent gaps to opaque masonry; no original game media.',sha256=hashlib.sha256(data).hexdigest()),indent=2)+'\n')
    print(json.dumps(dict(buildings=len(records),surface_patches=len(surfaces),manifest_bytes=OUT.stat().st_size,facade_bytes=len(data)),indent=2))
if __name__=='__main__':run()
