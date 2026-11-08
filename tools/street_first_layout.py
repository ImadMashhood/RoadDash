"""Original, offline street -> block -> parcel authoring; no runtime geometry edits."""
import math
from collections import Counter

import shapely
from shapely import LineString, MultiLineString, MultiPoint, Polygon, box
from shapely.ops import unary_union
from race_first_layout import primary_lines, row_extent, select_streets

PITCH = (180.0, 360.0)
SIDEWALK = 2.4


def polygons(geometry):
    if geometry.is_empty:
        return []
    if geometry.geom_type == 'Polygon':
        return [geometry]
    return [p for g in geometry.geoms for p in polygons(g)] if hasattr(geometry, 'geoms') else []


def ring(poly):
    return [[float(x), float(y)] for x, y in poly.exterior.coords[:-1]]


def encode(geometry):
    return [dict(shell=ring(p), holes=[list(map(list, r.coords[:-1])) for r in p.interiors])
            for p in polygons(geometry)]


def decode(records):
    return unary_union([Polygon(r['shell'], r['holes']) for r in records])


def parcels_for(block):
    """Boundary frontage seeds partition the actual site, never a warped grid."""
    if block.area < 4200:
        return [block], [block], 'corner_building'
    boundary = block.exterior
    count = max(4, round(boundary.length / 18))
    seeds = [boundary.interpolate((i + .5) * boundary.length / count) for i in range(count)]
    cells = shapely.voronoi_polygons(MultiPoint(seeds), extend_to=block.envelope, ordered=True)
    parcels = [p for cell in cells.geoms for p in polygons(cell.intersection(block)) if p.area > 1e-7]
    # Absorb clipped sliver fragments into a neighboring property, preserving land.
    # A frontage Voronoi cell can split at a concave corner of the real block.
    while min(p.area for p in parcels)<30:
        part=min(parcels,key=lambda p:p.area)
        neighbors=sorted((p for p in parcels if p is not part),
                         key=lambda p:part.boundary.intersection(p.boundary).length, reverse=True)
        for neighbor in neighbors:
            merged=part.union(neighbor)
            if merged.geom_type!='Polygon':
                # Resolve nanometre clipping roundoff only; reject any material land change.
                snapped=shapely.union_all([part,neighbor],grid_size=1e-9)
                if snapped.symmetric_difference(merged).area<1e-6:
                    merged=snapped
            if merged.geom_type=='Polygon' and merged.is_valid:
                parcels=[p for p in parcels if p is not part and p is not neighbor]+[merged]
                break
        else:
            raise AssertionError('Unmerged property sliver')
    # All parcels retain ownership to the courtyard; only their street-side portion is built.
    band = block.difference(block.buffer(-23, join_style='mitre'))
    # Fixed-precision overlay avoids near-coincident clipping artifacts on concave blocks.
    faces = [p for parcel in parcels for p in polygons(shapely.intersection(parcel,band,grid_size=1e-9)) if p.area >= 30]
    return parcels, faces, 'perimeter_block'


def make_fabric(world, edges, public_sites, nodes, rules):
    px, pz = PITCH
    cells, streets, rows = [], {}, []
    primary=primary_lines(edges)
    for row in range(math.ceil((world.height_z + 650) / pz)):
        z0 = -180 + row * pz
        t = min(1, max(0, (z0 + pz / 2) / world.height_z))
        opening = min(1, t * 5); opening = opening * opening * (3 - 2 * opening)
        half_width = rules['city_edge_half_width_m'] + rules['city_growth_width_m'] * opening + rules['city_middle_width_m'] * math.sin(t * math.pi)
        start,end,section=row_extent(world,primary,z0+pz/2,half_width,px)
        rows.append(section)
        for col in range(start, end):
            x0 = col * px
            cells.append(box(x0, z0, x0 + px, z0 + pz))
            corners = [(x0,z0),(x0+px,z0),(x0+px,z0+pz),(x0,z0+pz)]
            for a,b in zip(corners,corners[1:]+corners[:1]):
                key = tuple(sorted((a,b)))
                streets[key] = dict(points=[list(a),list(b)], width_m=12 if a[0]==b[0] and round(a[0]/px)%3==0 else 8)
    envelope = unary_union(cells)
    streets = [dict(id=f'ST-{i:04}', **s) for i,s in enumerate(streets.values())]
    streets,race_first=select_streets(streets,primary,nodes,rules)
    race_first['centering_rows']=rows
    # Simplification is applied to road centerlines, not to derived land/buildings.
    paths = [(LineString(s['points']),s['width_m']) for s in streets]
    for e in edges:
        line = LineString([(p[0],p[2]) for p in e['points']]).simplify(2.0)
        paths.append((line, 8 if e['id']=='farm_approach' else 12))
    carriageway = unary_union([line.buffer(width/2, cap_style='square', join_style='mitre') for line,width in paths])
    reservation = unary_union([line.buffer(width/2+SIDEWALK, cap_style='square', join_style='mitre') for line,width in paths])
    land = envelope.difference(reservation)
    raw_blocks = sorted(polygons(land), key=lambda p:(p.centroid.y,p.centroid.x))
    public = unary_union([box(*s['bounds']) for s in public_sites])
    blocks = []
    for i, poly in enumerate(raw_blocks):
        # Unbuildable slivers are explicit landscape/traffic islands, never a distorted building.
        p = poly.centroid; t = max(0,min(1,p.y/world.height_z))
        district = 'old_city' if t<.25 else 'market' if t<.43 else 'mixed' if t<.64 else 'terraces' if t<.82 else 'north_city'
        role='traffic_island' if poly.area < 250 or poly.buffer(-4).is_empty else 'perimeter_block'
        if i % 47 == 11 and poly.area > 25000:
            role='park'
        parcels, faces = [], []
        if role=='perimeter_block':
            parcels,faces,role=parcels_for(poly)
        # Civic reservations are clipped to actual land; never cover a road or sidewalk.
        reserved=poly.intersection(public)
        faces=[part for f in faces for part in polygons(f.difference(reserved)) if part.area >= 30]
        blocks.append(dict(id=f'LC-S{i:04}',district=district,polygon=ring(poly),holes=[list(map(list,r.coords[:-1])) for r in poly.interiors],
                           faces=[ring(f) for f in faces],face_holes=[[list(map(list,r.coords[:-1])) for r in f.interiors] for f in faces],parcels=[ring(p) for p in parcels],role=role,
                           center=[p.x,p.y],base_elevation_m=world.height([p.x,p.y]),area_m2=poly.area,
                           public_land=encode(reserved),courtyard_use='private_courts'))
    noded=shapely.node(MultiLineString([line.coords for line,_ in paths]))
    counts=Counter(tuple(p) for line in noded.geoms for p in [line.coords[0],line.coords[-1]])
    junctions=[dict(point=list(p),degree=d) for p,d in sorted(counts.items()) if d>=3]
    # Pick a real small, pointed block cut by the race road, not a fabricated demo triangle.
    race=unary_union([line for line,_ in paths[len(streets):]])
    candidates=[b for b in blocks if b['role']=='corner_building' and Polygon(b['polygon']).distance(race)<15 and b['center'][1]>200]
    assert candidates, 'The road network should create usable corner sites'
    example=max(candidates,key=lambda b:b['area_m2'])
    return dict(blocks=blocks,streets=streets,junctions=junctions,block_pitch_m=list(PITCH),race_first=race_first,
                city_envelope=encode(envelope),road_surface=encode(carriageway.intersection(envelope)),
                road_reservation=encode(reservation.intersection(envelope)),
                sidewalks=encode(reservation.difference(carriageway).intersection(envelope)),
                street_graph_segments=[list(map(list,line.coords)) for line in noded.geoms],
                corner_example_id=example['id'],
                geometry_backend=dict(shapely=shapely.__version__,geos=shapely.geos_version_string))


def audit_fabric(data):
    envelope=decode(data['city_envelope']); roads=decode(data['road_reservation'])
    surface=decode(data['road_surface']);sidewalks=decode(data['sidewalks'])
    assert surface.intersection(sidewalks).area<1e-5, 'Sidewalk overlaps carriageway'
    assert surface.union(sidewalks).symmetric_difference(roads).area<1e-5, 'Incomplete street reservation'
    block_polys=[Polygon(b['polygon'],b['holes']) for b in data['blocks']]
    all_land=unary_union(block_polys)
    assert all(p.is_valid and p.area>0 for p in block_polys), 'Invalid block polygon'
    assert roads.intersection(all_land).area < 1e-5, 'Road/building land intrusion'
    assert envelope.difference(roads.union(all_land)).area<1e-5, 'Unaccounted land'
    assert abs(sum(p.area for p in block_polys)-all_land.area)<1e-5, 'Overlapping blocks'
    count=0;nonrectangular=0;max_parcel_error=0
    for b,block in zip(data['blocks'],block_polys):
        faces=[Polygon(f,h) for f,h in zip(b['faces'],b['face_holes'])]
        assert len(b['face_holes'])==len(b['faces'])
        parcels=[Polygon(p) for p in b['parcels']]
        assert all(p.is_valid and p.area>=30-1e-6 for p in faces), 'Invalid footprint'
        assert all(p.difference(block).area<1e-6 for p in faces+parcels), 'Footprint/parcel outside block'
        if parcels:
            assert all(p.boundary.intersection(block.boundary.buffer(1e-7)).length>.1 for p in parcels), 'Landlocked parcel'
            union=unary_union(parcels)
            error=block.symmetric_difference(union).area
            max_parcel_error=max(error,max_parcel_error)
            assert error<1e-5, 'Ownership does not cover block'
            assert abs(sum(p.area for p in parcels)-union.area)<1e-5, 'Ownership overlaps'
        if faces:
            assert abs(sum(p.area for p in faces)-unary_union(faces).area)<1e-5, 'Building overlap'
        count+=len(faces)
        nonrectangular+=sum(abs(p.area-p.minimum_rotated_rectangle.area)>1 for p in faces)
    assert data['block_pitch_m']==[180,360]
    example=next(b for b in data['blocks'] if b['id']==data['corner_example_id'])
    assert example['role']=='corner_building' and example['faces']
    shape=Polygon(example['polygon'])
    assert shape.area < .9*shape.minimum_rotated_rectangle.area, 'Corner example must be an angled road-cut site'
    # Every street crossing must be a graph endpoint; no lines cross in their interiors.
    lines=[LineString(s) for s in data['street_graph_segments']]
    again=shapely.node(MultiLineString([s.coords for s in lines]))
    assert len(again.geoms)==len(lines), 'Unnoded street crossing'
    return dict(blocks=len(block_polys),footprints=count,nonrectangular_footprints=nonrectangular,
                street_junctions=len(data['junctions']),noded_street_segments=len(lines),
                road_land_overlap_m2=roads.intersection(all_land).area,
                land_coverage_error_m2=envelope.difference(roads.union(all_land)).area,
                max_parcel_coverage_error_m2=max_parcel_error,corner_example_id=example['id'],
                corner_example_area_m2=shape.area,nominal_grid_pitch_m=data['block_pitch_m'])
