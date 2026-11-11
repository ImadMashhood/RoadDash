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
            # Narrow landward sidewalk clips can leave a sub-millimetre edge
            # triangle whose representative point rounds onto the boundary.
            # Check actual exterior area; the total-area assertion below still
            # rejects missing or duplicate building geometry.
            assert t.difference(part).area < 1e-6
            triangles.append([lookup[p] for p in list(orient(t,sign=1).exterior.coords)[:3]])
    assert triangles
    assert abs(sum(Polygon([vertices[i] for i in t]).area for t in triangles)-clean.area)<1e-5
    return dict(rings=rings,triangles=triangles)

def tiled_shapes(geometry):
    geometry=shapely.set_precision(shapely.make_valid(geometry),1e-6)
    xmin,zmin,xmax,zmax=geometry.bounds
    for x in range(math.floor(xmin/32)*32,math.ceil(xmax/32)*32,32):
        for z in range(math.floor(zmin/32)*32,math.ceil(zmax/32)*32,32):
            for p in polygons(shapely.clip_by_rect(geometry,x,z,x+32,z+32)):
                if p.area>.0001:yield triangulate(p)

def validate_palette(palette):
    families = palette['families']
    assert len(families) == 24 and len(set(families)) == len(families)
    catalog = json.loads((ROOT/'art/buildings/mesh_assets/storey_kit/MANIFEST.json').read_text())
    records = {r['id']: r for r in catalog['records']}
    assert all(x in records and records[x]['asset_type'] == 'facade' for x in families)
    assert set(palette['pools']) == {'old_city', 'market', 'mixed', 'terraces', 'north_city', 'tower'}
    assert set(x for pool in palette['pools'].values() for x in pool) == set(families)
    assert all(pool and len(set(pool)) == len(pool) for pool in palette['pools'].values())
    assert all(x.startswith('OFFC') for x in palette['pools']['tower'])
    assert palette['tower_min_floors'] == 13
    approach = palette['approach']
    assert approach['max_lowrise_floors'] == 5
    assert set(approach['pools']) == {'roadside', 'workshop', 'neighborhood'}
    for pool in approach['pools'].values():
        assert len(pool) >= 3 and len(pool) == len(set(pool))
        assert set(pool) <= set(families)
        assert not any(x.startswith('OFFC') for x in pool)
    stations = approach['stations']
    assert len(stations) >= 3 and stations[0]['z'] == -180 and stations[-1]['z'] == 1800
    assert all(a['z'] < b['z'] for a,b in zip(stations,stations[1:]))
    for station in stations:
        assert set(station['weights']) <= set(approach['pools']) | {'district'}
        assert all(0 <= w <= 1 for w in station['weights'].values())
        assert abs(sum(station['weights'].values())-1) < 1e-9
    assert stations[-1]['weights'] == {'district': 1}



def facade_role(building, block, floors, palette):
    """Blend low-rise use along the approved northward approach, not across ROW.

    Authored three-index parcel groups share the use lottery; individual original
    families still vary. Only low-rise approach source towers lose office priority.
    Geometry, height caps, parcel setbacks and existing vegetation stay authored.
    """
    if floors >= palette['tower_min_floors'] or (
        building['role'] == 'interior_tower' and (
            floors > palette['approach']['max_lowrise_floors']
            or block.get('center', [0, 1800])[1] >= 1800
        )
    ):
        return 'tower'
    approach = palette['approach']
    if floors > approach['max_lowrise_floors'] or 'center' not in block:
        return block['district']
    z = block['center'][1]
    stations = approach['stations']
    if z >= stations[-1]['z']:
        return block['district']
    left = stations[0]
    right = stations[0]
    for a, b in zip(stations, stations[1:]):
        if a['z'] <= z < b['z']:
            left, right = a, b
            break
    t = 0 if left == right else (z-left['z'])/(right['z']-left['z'])
    group = f"{block['id']}:{building['footprint_index']//3}"
    pick = int.from_bytes(hashlib.blake2b(group.encode(), digest_size=4).digest(), 'little') / 2**32
    cumulative = 0
    for role in ['roadside', 'workshop', 'neighborhood', 'district']:
        cumulative += (1-t)*left['weights'].get(role,0) + t*right['weights'].get(role,0)
        if pick < cumulative:
            return block['district'] if role == 'district' else role
    raise AssertionError('approach weights must sum to one')


def facade_style(building, block, floors, palette):
    role = facade_role(building, block, floors, palette)
    pool = palette['pools'].get(role) or palette['approach']['pools'][role]
    seed = int.from_bytes(hashlib.blake2b(building["id"].encode(), digest_size=4).digest(), "little")
    return palette["families"].index(pool[seed % len(pool)])

def run():
    source=ROOT/'docs/planning/farms-long-city/LAYOUT.json';d=json.loads(source.read_text())
    from metropolis_street_life import varied_floors, parked_cars, traffic_paths
    life=json.loads((ROOT/'content/world/metropolis_street_life.json').read_text())
    from living_city import CONFIG
    assert 64 <= CONFIG['main_traffic'] <= 144
    life['main_traffic']=CONFIG['main_traffic']
    palette=json.loads((ROOT/'content/world/metropolis_facades.json').read_text())
    validate_palette(palette)
    records=[]
    for b in d['blocks']:
        for building in b['buildings']:
            i=building['footprint_index'];p=Polygon(b['faces'][i],b['face_holes'][i])
            floors=varied_floors(building,b,life)
            records.append(dict(id=building['id'],shape=triangulate(p),height=round(floors*life['floor_height'],2),
                                floors=floors,tier=['hero','standard','skyline','massing'].index(building['detail_tier']),
                                style=facade_style(building,b,floors,palette)))
    edges={e['id']:e for e in d['edges']};paths=[]
    for choice in ['old_market','station_avenue']:
        points=[];progress=[]
        for ident in ['farm_approach',choice,'long_city']:
            e=edges[ident];ds=[0]
            for a,b in zip(e['points'],e['points'][1:]):ds.append(ds[-1]+math.dist(a,b))
            ds=e.get('progress_fractions',ds)
            lo,hi=e['logical_chainage_m']
            for j,p in enumerate(e['points']):
                if points and j==0:continue
                points.append(p);progress.append(lo+(hi-lo)*ds[j]/ds[-1])
        paths.append(dict(points=points,progress=progress))
    cp=[]
    for i in range(0,len(paths[0]['points']),max(1,len(paths[0]['points'])//125)):
        p=paths[0]['points'][i];s=paths[0]['progress'][i]
        if cp and math.dist(p,cp[-1]['position'])<20:continue
        cp.append(dict(position=p,half_width=6,bank_degrees=0,sector='farms' if s<3150 else 'city',surface='asphalt'))
    if math.dist(cp[-1]['position'],paths[0]['points'][-1])<20:cp.pop()
    cp.append(dict(position=paths[0]['points'][-1],half_width=6,bank_degrees=0,sector='city',surface='asphalt'))
    gates=sorted(set([0,1000,2200,3150,3350,3900,4500,5000,5362.5,6500,8000,9500,11000,12000]))
    track=dict(schema_version=1,id='metro-run-01',name='Metropolis — Farms to Downtown / two routes',closed=False,scenery_seed=67201,
               control_points=cp,checkpoints=[dict(id=f'metro-gate-{i:02}',fraction=s/12000) for i,s in enumerate(gates)],
               spawns=[dict(id=f'metro-grid-{i:02}',distance=-8-(14-i)*20,lane=-1.5 if i%2 else -4.5) for i in range(15)],props=[],verge_width=2.0)
    ground_bounds=decode(d['city_envelope']).bounds
    field_bounds=[Polygon(f['polygon']).bounds for f in d['fields']]
    bounds=[min(x[0] for x in [ground_bounds]+field_bounds)-100,min(x[1] for x in [ground_bounds]+field_bounds)-180,
            max(x[2] for x in [ground_bounds]+field_bounds)+100,max(x[3] for x in [ground_bounds]+field_bounds)+100]
    from metropolis_streetscape import sidewalks, furniture, audit
    road, walk, gained = sidewalks(d)
    from living_city import augment, shape_polygon
    walk,civic_sites=augment(d,paths,records,road,walk,triangulate)
    parked=parked_cars(d,paths,road,life)
    # Small side-street curb bays reuse original pavement reservation. Preserve
    # the main race carriageway and retain >=1.5m of outer sidewalk per bay.
    from metropolis_streetscape import footprint
    from shapely.ops import unary_union
    parked=[c for c in parked if walk.covers(footprint([c['position'][0]+math.cos(c['heading'])*1.9,c['position'][1]-math.sin(c['heading'])*1.9],c['heading'],1.5,6.4))]
    bays=unary_union([footprint(c['position'],c['heading'],2.2,4.4) for c in parked]).intersection(walk)
    road=road.union(bays);walk=walk.difference(bays)
    street_props = furniture(d, paths, walk)
    cafe_shapes=[footprint(s['position'],s['heading'],s['size'][0],s['size'][1]).buffer(.2) for s in civic_sites if s['kind']=='cafe']
    street_props=[p for p in street_props if not any(footprint(p['position'],p['heading'],p['size'][0],p['size'][2]).intersects(c) for c in cafe_shapes)]
    sidewalk_report = audit(d, road, walk, gained, street_props, built_override=shapely.union_all([shape_polygon(r['shape']) for r in records]))
    sidewalk_report['parking_bay_area_m2']=round(bays.area,3)
    from metropolis_population import pedestrians, crossing_routes
    walker_props=street_props+[dict(position=s['position'],heading=s['heading'],size=[s['size'][0],3.,s['size'][1]],kind='cafe') for s in civic_sites if s['kind']=='cafe']
    walkers=pedestrians(d,paths,walk,walker_props,budget=CONFIG['pedestrians'])
    selected_crossings=crossing_routes(walkers,road,walk,walker_props,CONFIG['pedestrian_crossings'])
    assert selected_crossings, 'no clear sidewalk-to-sidewalk crossings'
    assert len(walkers)==CONFIG['pedestrians']
    surfaces=[]
    # Clip to short tiles before triangulation: pavement follows the graded height field.
    courts=[]
    from shapely.ops import unary_union
    for b in d['blocks']:
        if b['role'] in ['park','traffic_island']:continue
        land=Polygon(b['polygon'],b['holes'])
        built=unary_union([Polygon(p,h) for p,h in zip(b['faces'],b['face_holes'])])
        courts.extend(polygons(land.difference(built).difference(walk).difference(road)))
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
    from shapely import LineString
    from metropolis_streetscape import farm_pavement
    farm_road=farm_pavement(d)
    xmin,zmin,xmax,zmax=farm_road.bounds
    for x in range(math.floor(xmin/32)*32,math.ceil(xmax/32)*32,32):
        for z in range(math.floor(zmin/32)*32,math.ceil(zmax/32)*32,32):
            for p in polygons(shapely.clip_by_rect(farm_road,x,z,x+32,z+32)):
                if p.area>.001:surfaces.append(dict(kind=0,shape=triangulate(p)))
    # Grid apron continues the first source segment backwards.
    a,b=paths[0]['points'][:2];dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz)
    apron=LineString([(a[0]-dx/n*340,a[2]-dz/n*340),(a[0],a[2])]).buffer(6.8,cap_style='square')
    xmin,zmin,xmax,zmax=apron.bounds
    for x in range(math.floor(xmin/32)*32,math.ceil(xmax/32)*32,32):
        for z in range(math.floor(zmin/32)*32,math.ceil(zmax/32)*32,32):
            for p in polygons(shapely.clip_by_rect(apron,x,z,x+32,z+32)):
                if p.area>.001:surfaces.append(dict(kind=0,shape=triangulate(p)))
    from metropolis_elevation import PROFILE
    assert d['elevation_profile'] == PROFILE
    manifest=dict(schema_version=1,elevation_profile=PROFILE,source_layout_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),track=track,paths=paths,
                  bounds=bounds,civic_sites=civic_sites,buildings=records,surfaces=surfaces,streets=d['streets'],
                  junctions=[dict(point=j['point'],street_id=j['secondary_street_ids'][0]) for j in d['race_first']['primary_junctions']],
                  traffic_paths=traffic_paths(paths,d),street_life=life,parked_cars=parked,pedestrians=walkers,street_props=street_props,fields=[dict(id=f['id'],shape=triangulate(Polygon(f['polygon'])),patches=list(tiled_shapes(Polygon(f['polygon']))),crop=f['crop']) for f in d['fields']])
    (ROOT/'docs/testing/civic-greenery').mkdir(exist_ok=True)
    (ROOT/'docs/testing/civic-greenery/sidewalk-authoring.json').write_text(json.dumps(sidewalk_report,indent=2)+'\n')
    OUT.write_text(json.dumps(manifest,separators=(',',':'))+'\n')
    bake_facades(palette)
    print(json.dumps(dict(buildings=len(records),surface_patches=len(surfaces),manifest_bytes=OUT.stat().st_size),indent=2))

def reheight_manifest(manifest, previous, updated):
    """Rebake only elevation after proving the approved planar source is identical.

    Keep repaired pavement, lane profiles, civic grounds and every resident object
    verbatim. Full layout changes must use the normal geometry bake instead.
    """
    import copy
    from metropolis_elevation import PROFILE, height

    def without_elevation(layout):
        layout = copy.deepcopy(layout)
        layout.pop('elevation_profile', None)
        layout.pop('elevation_note', None)
        for point in layout['nodes'].values():
            point[1] = 0
        for edge in layout['edges']:
            edge.pop('measured_3d_m', None)
            for point in edge['points']:
                point[1] = 0
        for name in ['proposed_elevation_range_m', 'max_sampled_grade_percent']:
            layout['validation'].pop(name, None)
        return layout

    if without_elevation(previous) != without_elevation(updated):
        raise ValueError('elevation-only bake rejects planar/source metadata changes')
    if updated['elevation_profile'] != PROFILE:
        raise ValueError('updated source must match the authored elevation profile')
    if manifest['elevation_profile'] != previous['elevation_profile']:
        raise ValueError('previous source profile does not match baked world')
    for point in list(updated['nodes'].values()) + [
        p for edge in updated['edges'] for p in edge['points']
    ]:
        if abs(point[1] - height(point[0], point[2])) > 1e-7:
            raise ValueError('updated source has stale route heights')
    result = copy.deepcopy(manifest)
    result['elevation_profile'] = copy.deepcopy(PROFILE)
    for path in result['paths'] + result['traffic_paths']:
        for point in path['points']:
            point[1] = height(point[0], point[2])
    for control in result['track']['control_points']:
        point = control['position']
        point[1] = height(point[0], point[2])
    return result


def bake_elevation(previous_layout):
    source = ROOT/'docs/planning/farms-long-city/LAYOUT.json'
    previous_bytes = Path(previous_layout).read_bytes()
    manifest = json.loads(OUT.read_text())
    if hashlib.sha256(previous_bytes).hexdigest() != manifest['source_layout_sha256']:
        raise ValueError('previous layout hash must match the currently baked source')
    result = reheight_manifest(manifest, json.loads(previous_bytes), json.loads(source.read_text()))
    result['source_layout_sha256'] = hashlib.sha256(source.read_bytes()).hexdigest()
    OUT.write_text(json.dumps(result,separators=(',',':'))+'\n')
    print('Rebaked elevation only; all planar geometry, IDs, lane profiles and counts preserved.')


def bake_facades(palette):
    atlas_root=ROOT/'art/buildings/mesh_assets/far_facades';atlas=json.loads((atlas_root/'MANIFEST.json').read_text())
    designs=palette['families']
    data=bytearray();packed_data=bytearray();sources=[]
    for design in designs:
        for band in [0,1,2]:
            source_design='CLSTGA11' if design=='BUSS4' and band==1 else design
            tile=next(t for t in atlas['records'] if t['design']==source_design and t['band']==band)
            for level in range(5):
                image=Image.open(atlas_root/f'albedo-mip{level}.png').convert('RGBA');w,h=image.size
                lo=tile['uv_min'];side=96>>level
                x,y=round(lo[0]*w-.5),round(lo[1]*h-.5)
                crop=image.crop((x,y,x+side,y+side))
                back=Image.new('RGBA',crop.size,(135,126,108,255));back.alpha_composite(crop);data.extend(back.tobytes())
                packed=Image.open(atlas_root/f'packed-mip{level}.png').convert('RGBA').crop((x,y,x+side,y+side))
                # Opaque resident envelope behind holes: rough dielectric masonry.
                packed_back=Image.new('RGBA',crop.size,(255,230,0,255));packed_back.alpha_composite(packed);packed_data.extend(packed_back.tobytes())
            sources.append(dict(design=design,source_design=source_design,band=band,source_sha256=tile['source_sha256']))
    dest=ROOT/'assets/metropolis';dest.mkdir(exist_ok=True);(dest/'facades.rgba').write_bytes(data);(dest/'facades-packed.rgba').write_bytes(packed_data)
    (dest/'PROVENANCE.json').write_text(json.dumps(dict(source_manifest_sha256=hashlib.sha256((atlas_root/'MANIFEST.json').read_bytes()).hexdigest(),
       packed_sha256=hashlib.sha256(packed_data).hexdigest(),packed_channels='linear R=unused G=perceptual roughness B=metallic A=opaque',palette_sha256=hashlib.sha256((ROOT/'content/world/metropolis_facades.json').read_bytes()).hexdigest(),atlas_columns=8,atlas_rows=9,bands_per_family=3,mip_levels=5,sources=sources,modifications='Crop original model-derived storey albedo/mips; flatten transparent gaps to opaque masonry; no original game media.',sha256=hashlib.sha256(data).hexdigest()),indent=2)+'\n')

if __name__=='__main__':
    import sys
    if '--elevation-only' in sys.argv:
        import argparse
        parser = argparse.ArgumentParser(description=__doc__)
        parser.add_argument('--elevation-only', action='store_true')
        parser.add_argument('--previous-layout', required=True, type=Path)
        args = parser.parse_args()
        bake_elevation(args.previous_layout)
    elif '--facades-only' in sys.argv:
        bake_facades(json.loads((ROOT/'content/world/metropolis_facades.json').read_text()))
    else:run()
