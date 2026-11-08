"""Original race-first city envelope and secondary-street authoring; offline only."""
import math
from collections import defaultdict

from shapely import LineString, Point
from shapely.ops import unary_union


def points_of(geometry):
    if geometry.is_empty:
        return []
    if geometry.geom_type=='Point':
        return [geometry]
    if geometry.geom_type in ['LineString','LinearRing']:
        return [Point(geometry.coords[0]),Point(geometry.coords[-1])]
    return [p for part in geometry.geoms for p in points_of(part)]


def primary_lines(edges):
    return [(edge['id'],LineString([(p[0],p[2]) for p in edge['points']]).simplify(2.0)) for edge in edges]


def row_extent(world, primary, z, half_width, pitch):
    cut=LineString([(-100000,z),(100000,z)])
    xs=[point.x for _,line in primary for point in points_of(line.intersection(cut))]
    # City continues beyond the finish; extend its endpoint's development axis.
    if not xs:
        xs=[world.city.points[0 if z<0 else -1][0]]
    left,right=min(xs),max(xs)
    start=round((left-half_width)/pitch)
    end=round((right+half_width)/pitch)
    return start,end,dict(northing_m=z,route_x_range_m=[left,right],
                          envelope_x_range_m=[start*pitch,end*pitch],
                          center_error_m=abs((start+end)*pitch/2-(left+right)/2))


def crossing_angle(street, avenue, point):
    station=avenue.project(point)
    a,b=avenue.interpolate(max(0,station-1)),avenue.interpolate(min(avenue.length,station+1))
    u=[b.x-a.x,b.y-a.y]
    a,b=street.coords[0],street.coords[-1];v=[b[k]-a[k] for k in [0,1]]
    dot=abs(sum(u[k]*v[k] for k in [0,1]))/(math.hypot(*u)*math.hypot(*v))
    return math.degrees(math.acos(min(1,max(0,dot))))


def parallel_run_m(line, primary, rules):
    """Measure a sustained close, similarly aligned run, rather than one crossing."""
    distance=rules['parallel_competition_distance_m']
    if min(avenue.distance(line) for _,avenue in primary)>distance:
        return 0.0
    count=max(1,math.ceil(line.length/rules['parallel_sample_step_m']))
    step=line.length/count;current=maximum=0.0
    for i in range(count):
        point=line.interpolate((i+.5)*step)
        avenue=min(primary,key=lambda item:item[1].distance(point))[1]
        near=(avenue.distance(point)<distance and
              crossing_angle(line,avenue,point)<rules['parallel_max_angle_degrees'])
        current=current+step if near else 0.0
        maximum=max(maximum,current)
    return maximum


def select_streets(streets, primary, nodes, rules):
    throats=[Point(nodes[n][0],nodes[n][2]) for n in ['split','rejoin']]
    candidates=[];omitted=[]
    for street in streets:
        line=LineString(street['points']);crossings=[]
        for ident,avenue in primary:
            for point in points_of(line.intersection(avenue)):
                crossings.append(dict(avenue_id=ident,point=list(point.coords[0]),
                                      station_m=avenue.project(point),angle_degrees=crossing_angle(line,avenue,point)))
        parallel=parallel_run_m(line,primary,rules)>=rules['parallel_min_run_m']
        reason=('parallel_competition' if parallel else 'fork_throat' if any(
                Point(c['point']).distance(throat)<rules['fork_throat_clearance_m']
                for c in crossings for throat in throats) else None)
        if reason:
            omitted.append(dict(**street,reason=reason))
        else:
            # Larger crossing angle gets the junction first; ties are stable by ID.
            priority=max((c['angle_degrees'] for c in crossings),default=-1)
            candidates.append((-priority,street['id'],street,crossings))
    accepted=defaultdict(list);kept=[];junctions={}
    minimum=rules['minimum_distinct_race_junction_spacing_m']
    for _,_,street,crossings in sorted(candidates):
        crowded=any(1e-6<abs(c['station_m']-station)<minimum
                    for c in crossings for station in accepted[c['avenue_id']])
        if crowded:
            omitted.append(dict(**street,reason='redundant_close_junction'))
            continue
        kept.append(street)
        for crossing in crossings:
            accepted[crossing['avenue_id']].append(crossing['station_m'])
            key=(crossing['avenue_id'],round(crossing['station_m'],6))
            if key not in junctions:
                junctions[key]=dict(**crossing,secondary_street_ids=[],priority='race_avenue',control='Secondary traffic gives way; final signals/markings deferred')
            junctions[key]['secondary_street_ids'].append(street['id'])
    return sorted(kept,key=lambda s:s['id']),dict(rules=rules,centering_rows=[],
                 primary_junctions=list(junctions.values()),omitted_secondary_segments=omitted,
                 route_contract='Race line remains the primary avenue. Both legal alternatives determine the city envelope.')


def audit_race_first(data):
    from street_first_layout import decode
    rules=data['race_first']['rules'];primary=dict(primary_lines(data['edges']))
    envelope=decode(data['city_envelope']);maximum=0
    for row in data['race_first']['centering_rows']:
        z=row['northing_m']
        actual=envelope.intersection(LineString([(-100000,z),(100000,z)]))
        assert not actual.is_empty
        lo,_,hi,_=actual.bounds
        assert abs(lo-row['envelope_x_range_m'][0])<1e-6 and abs(hi-row['envelope_x_range_m'][1])<1e-6
        cut=LineString([(-100000,z),(100000,z)])
        xs=[p.x for line in primary.values() for p in points_of(line.intersection(cut))]
        if xs:
            assert abs(min(xs)-row['route_x_range_m'][0])<1e-6 and abs(max(xs)-row['route_x_range_m'][1])<1e-6
        left,right=row['route_x_range_m'];error=abs((lo+hi)/2-(left+right)/2)
        assert abs(error-row['center_error_m'])<1e-6
        assert error<=data['block_pitch_m'][0]/2+1e-6, 'City no longer centered on race'
        assert lo<left-500 and hi>right+500, 'Insufficient development on both sides'
        maximum=max(maximum,error)
    streets={s['id']:LineString(s['points']) for s in data['streets']}
    for ident,line in streets.items():
        assert parallel_run_m(line,list(primary.items()),rules)<rules['parallel_min_run_m'], f'Competing parallel street retained: {ident}'
    stations=defaultdict(list)
    for junction in data['race_first']['primary_junctions']:
        point=Point(junction['point']);avenue=primary[junction['avenue_id']]
        assert point.distance(avenue)<1e-6, 'Race junction off track'
        assert abs(avenue.project(point)-junction['station_m'])<1e-6
        for ident in junction['secondary_street_ids']:
            assert ident in streets and streets[ident].distance(point)<1e-6, 'Missing secondary junction connection'
        for name in ['split','rejoin']:
            p=data['nodes'][name]
            assert point.distance(Point(p[0],p[2]))>=rules['fork_throat_clearance_m']-1e-6
        stations[junction['avenue_id']].append(junction['station_m'])
    gaps=[b-a for values in stations.values() for a,b in zip(sorted(values),sorted(values)[1:])]
    assert gaps and min(gaps)>=rules['minimum_distinct_race_junction_spacing_m']-1e-5
    # Every actual secondary/race crossing has a primary-junction record.
    recorded={(j['avenue_id'],ident,round(j['station_m'],5))
              for j in data['race_first']['primary_junctions'] for ident in j['secondary_street_ids']}
    for ident,line in streets.items():
        for avenue_id,avenue in primary.items():
            for point in points_of(line.intersection(avenue)):
                assert (avenue_id,ident,round(avenue.project(point),5)) in recorded, 'Unplanned race crossing'
    adjacency=defaultdict(set)
    for segment in data['street_graph_segments']:
        a,b=tuple(segment[0]),tuple(segment[-1]);adjacency[a].add(b);adjacency[b].add(a)
    seen=set();pending=[next(iter(adjacency))]
    while pending:
        node=pending.pop()
        if node not in seen:
            seen.add(node);pending.extend(adjacency[node]-seen)
    assert len(seen)==len(adjacency), 'Disconnected street network'
    reasons=defaultdict(int)
    for omitted in data['race_first']['omitted_secondary_segments']:
        assert omitted['id'] not in streets
        reasons[omitted['reason']]+=1
    return dict(centered_rows=len(data['race_first']['centering_rows']),max_center_error_m=maximum,
                grid_quantization_limit_m=data['block_pitch_m'][0]/2,
                primary_junctions=len(data['race_first']['primary_junctions']),
                minimum_primary_junction_spacing_m=min(gaps),omitted_segments_by_reason=dict(reasons),
                street_network_connected=True,remaining_competing_parallel_segments=0,
                parallel_clearance_m=rules['parallel_competition_distance_m'],
                parallel_minimum_run_m=rules['parallel_min_run_m'])
