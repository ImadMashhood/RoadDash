//! The final kilometre of the existing Metropolis race, translated without
//! changing its authored buildings, facade families or street surface triangles.
use super::*;
use crate::world_gen::metropolis::{
    self, geometry::grass_material::GrassMaterial, material::FacadeMaterial,
};
use rr_content::metropolis::{Metropolis, Shape};

#[derive(Resource, Default, serde::Serialize)]
pub struct CitySlice {
    pub source_start: f32,
    pub source_finish: f32,
    pub buildings: usize,
    pub surfaces: usize,
    pub street_props: usize,
    pub pedestrians: usize,
    pub source_building_ids: Vec<String>,
}

fn shift(p: [f32; 2]) -> [f32; 2] {
    let origin = Track::hills_city_source_origin();
    [p[0] - origin.x, p[1] - origin.z]
}
fn translated(shape: &mut Shape) {
    for p in shape.rings.iter_mut().flatten() {
        *p = shift(*p);
    }
}
fn clip_polygon(mut polygon: Vec<Vec2>, lo: Vec2, hi: Vec2) -> Vec<Vec2> {
    for (axis, edge, sign) in [(0, lo.x, 1.), (0, hi.x, -1.), (1, lo.y, 1.), (1, hi.y, -1.)] {
        let input = std::mem::take(&mut polygon);
        if input.is_empty() {
            break;
        }
        let mut previous = *input.last().unwrap();
        for current in input {
            let a = (previous[axis] - edge) * sign;
            let b = (current[axis] - edge) * sign;
            if (a >= 0.) != (b >= 0.) {
                polygon.push(previous.lerp(current, a / (a - b)));
            }
            if b >= 0. {
                polygon.push(current);
            }
            previous = current;
        }
    }
    polygon
}
// Clip surface triangles, never building footprints. Their winding and authored
// interior holes survive because we retain each original triangulated piece.
fn clipped(shape: &Shape, lo: Vec2, hi: Vec2) -> Option<Shape> {
    let vertices: Vec<_> = shape
        .vertices()
        .map(|p| Vec2::from_array(shift(p)))
        .collect();
    let mut result = Shape {
        rings: vec![],
        triangles: vec![],
    };
    let mut offset = 0;
    for triangle in &shape.triangles {
        let polygon = clip_polygon(
            triangle.iter().map(|i| vertices[*i as usize]).collect(),
            lo,
            hi,
        );
        if polygon.len() < 3 {
            continue;
        }
        for i in 1..polygon.len() - 1 {
            if (polygon[i] - polygon[0])
                .perp_dot(polygon[i + 1] - polygon[0])
                .abs()
                > 0.0001
            {
                result
                    .triangles
                    .push([offset, offset + i as u32, offset + i as u32 + 1]);
            }
        }
        offset += polygon.len() as u32;
        result
            .rings
            .push(polygon.into_iter().map(|p| p.to_array()).collect());
    }
    (!result.triangles.is_empty()).then_some(result)
}

fn triangle_contains([a, b, c]: [Vec2; 3], point: Vec2) -> bool {
    let signs = [
        (b - a).perp_dot(point - a),
        (c - b).perp_dot(point - b),
        (a - c).perp_dot(point - c),
    ];
    signs.iter().all(|s| *s >= -0.001) || signs.iter().all(|s| *s <= 0.001)
}

fn content() -> Metropolis {
    let _load_span = crate::diagnostics::loading::Span::new("hills_city_source");
    let source = Track::metropolis().expect("approved Metropolis source");
    let mut m = (**source.metropolis.as_ref().unwrap()).clone();
    let spine: Vec<_> = (10200..=11600)
        .step_by(20)
        .map(|s| Track::hills_city_to_local(source.street_sample(0, s as f32, 0.).0).xz())
        .collect();
    let lo = spine
        .iter()
        .copied()
        .fold(Vec2::splat(f32::INFINITY), Vec2::min)
        - Vec2::new(190., 0.);
    let hi = spine
        .iter()
        .copied()
        .fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max)
        + Vec2::new(190., 0.);
    let near = |p: [f32; 2], radius: f32| {
        let p = Vec2::from_array(shift(p));
        p.cmpge(lo).all()
            && p.cmple(hi).all()
            && spine.windows(2).any(|edge| {
                let d = edge[1] - edge[0];
                let t = ((p - edge[0]).dot(d) / d.length_squared()).clamp(0., 1.);
                p.distance(edge[0] + d * t) < radius
            })
    };
    m.buildings.retain(|b| {
        let v: Vec<_> = b.shape.vertices().collect();
        let p = v.iter().fold(Vec2::ZERO, |a, p| a + Vec2::from_array(*p)) / v.len() as f32;
        near(p.to_array(), 180.)
    });
    for b in &mut m.buildings {
        translated(&mut b.shape);
    }
    m.surfaces = m
        .surfaces
        .into_iter()
        .filter_map(|mut s| {
            s.shape = clipped(&s.shape, lo, hi)?;
            Some(s)
        })
        .collect();
    m.street_props.retain(|p| near(p.position, 150.));
    for p in &mut m.street_props {
        p.position = shift(p.position);
    }
    let sidewalks: Vec<[Vec2; 3]> = m
        .surfaces
        .iter()
        .filter(|surface| surface.kind == 1)
        .flat_map(|surface| {
            let vertices: Vec<_> = surface.shape.vertices().map(Vec2::from_array).collect();
            surface
                .shape
                .triangles
                .iter()
                .map(move |triangle| triangle.map(|i| vertices[i as usize]))
                .collect::<Vec<_>>()
        })
        .collect();
    // Reuse the source curb furnishing sites, then add small original catalog
    // props only where the translated sidewalk polygon still supports them.
    let anchors = m.street_props.clone();
    for (index, anchor) in anchors.iter().enumerate().step_by(5) {
        if !matches!(anchor.kind.as_str(), "street_lamp" | "broadleaf" | "bench") {
            continue;
        }
        let forward = Vec2::new(anchor.heading.sin(), anchor.heading.cos());
        let candidate = Vec2::from_array(anchor.position) + forward * 3.6;
        if !sidewalks
            .iter()
            .any(|triangle| triangle_contains(*triangle, candidate))
            || m.street_props
                .iter()
                .any(|prop| Vec2::from_array(prop.position).distance(candidate) < 2.6)
            || m.pedestrians
                .iter()
                .any(|walker| Vec2::from_array(shift(walker.position)).distance(candidate) < 1.5)
        {
            continue;
        }
        let (kind, size) = if index.is_multiple_of(2) {
            ("flower_planter", [1.15, 0.7, 0.65])
        } else {
            ("street_bin", [0.5, 1.0, 0.5])
        };
        m.street_props.push(rr_content::metropolis::StreetProp {
            id: format!("hills-city-curb-{index}"),
            kind: kind.into(),
            position: candidate.to_array(),
            heading: anchor.heading,
            size,
        });
    }
    m.parked_cars.retain(|p| near(p.position, 150.));
    for p in &mut m.parked_cars {
        p.position = shift(p.position);
    }
    m.junctions.retain(|p| near(p.point, 40.));
    for p in &mut m.junctions {
        p.point = shift(p.point);
    }
    m.fields.clear();
    m.civic_sites.clear();
    m.pedestrians
        .retain(|p| near(p.position, 125.) && (10500. ..=11750.).contains(&p.station));
    // The imported source corridor is intentionally calmer than Metropolis:
    // retain exactly alternating authored walkers, including their crossings.
    let mut walker_index = 0;
    m.pedestrians.retain(|_| {
        let keep = walker_index % 2 == 0;
        walker_index += 1;
        keep
    });
    for walker in &mut m.pedestrians {
        walker.position = shift(walker.position);
        walker.station -= 10500.;
        walker.crossing_end = walker
            .crossing_end
            .filter(|end| near(*end, 125.))
            .map(shift);
    }
    m.streets.clear();
    for section in &mut m.street_life.lane_sections {
        section.start -= 10500.;
        section.end -= 10500.;
    }
    m.bounds = [lo.x, lo.y, hi.x, hi.y];
    m
}

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: Option<&mut Assets<Image>>,
    facades: Option<&mut Assets<FacadeMaterial>>,
    grass: Option<&mut Assets<GrassMaterial>>,
    server: Option<&AssetServer>,
    art: &Art,
    track: &Track,
) {
    let m = content();
    let mut local_images = Assets::default();
    let mut local_facades = Assets::default();
    let mut local_grass = Assets::default();
    metropolis::geometry::spawn(
        commands,
        meshes,
        materials,
        images.unwrap_or(&mut local_images),
        facades.unwrap_or(&mut local_facades),
        grass.unwrap_or(&mut local_grass),
        &m,
        track,
    );
    if let Some(server) = server {
        commands.insert_resource(metropolis::facades::Kit::load_buildings(
            server,
            &m.buildings,
            track,
        ));
    }
    let props = metropolis::streetscape::spawn_props(commands, art, track, &m.street_props);
    if art.props.is_some() {
        for (index, walker) in m.pedestrians.iter().enumerate() {
            let p = Vec3::new(
                walker.position[0],
                track.terrain_height(walker.position[0], walker.position[1]) + 0.15,
                walker.position[1],
            );
            let entity = crate::environment::spawn_pedestrian_patrol(
                commands,
                art,
                p,
                Quat::from_rotation_y(walker.heading),
                walker.station,
                walker.extent,
            );
            commands
                .entity(entity)
                .insert(Name::new(format!("Hills city {}", walker.id)));
            if let Some(end) = walker.crossing_end {
                commands.entity(entity).insert((
                    metropolis::pedestrians::CrossingWalker::new(
                        p,
                        Vec3::new(end[0], 0., end[1]),
                        index,
                    ),
                    bevy_rapier3d::prelude::Velocity::zero(),
                ));
            }
        }
    }
    if let Some(catalog) = &art.props {
        use bevy_rapier3d::prelude::*;
        for (index, car) in m.parked_cars.iter().enumerate() {
            let p = Vec3::new(
                car.position[0],
                track.terrain_height(car.position[0], car.position[1]),
                car.position[1],
            );
            let root = commands
                .spawn((
                    Name::new(format!("Hills imported {}", car.id)),
                    Transform::from_translation(p)
                        .with_rotation(Quat::from_rotation_y(car.heading)),
                    Visibility::default(),
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                    crate::world::Obstacle,
                    HillsSolid,
                ))
                .id();
            let visual = catalog.vehicle(commands, index);
            crate::runtime::attach_visuals(commands, root, &[visual]);
        }
    }
    commands.insert_resource(CitySlice {
        source_start: 10500.,
        source_finish: 11500.,
        buildings: m.buildings.len(),
        surfaces: m.surfaces.len(),
        street_props: props,
        pedestrians: if art.props.is_some() {
            m.pedestrians.len()
        } else {
            0
        },
        source_building_ids: m.buildings.into_iter().map(|b| b.id).collect(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn city_is_an_actual_source_slice_with_unchanged_building_shapes_and_styles() {
        let source = Track::metropolis().unwrap();
        let m = content();
        assert!(m.buildings.len() > 100 && m.buildings.len() < 2000);
        assert!(m.surfaces.len() > 100 && !m.street_props.is_empty());
        assert!(
            m.pedestrians.len() >= 8,
            "imported Hills city remains populated"
        );
        assert!(
            m.pedestrians
                .iter()
                .all(|p| (0. ..=1250.).contains(&p.station))
        );
        let origin = Track::hills_city_source_origin();
        for b in &m.buildings {
            let original = source
                .metropolis
                .as_ref()
                .unwrap()
                .buildings
                .iter()
                .find(|v| v.id == b.id)
                .unwrap();
            assert_eq!(b.style, original.style);
            assert_eq!(b.height, original.height);
            assert_eq!(b.shape.triangles, original.shape.triangles);
            for (p, q) in b.shape.vertices().zip(original.shape.vertices()) {
                assert!(
                    Vec2::new(p[0] + origin.x, p[1] + origin.z).distance(Vec2::from_array(q))
                        < 0.002
                );
            }
        }
        assert!(m.fields.is_empty());
    }
    #[test]
    fn clipping_preserves_surface_area_and_open_holes() {
        let p = clip_polygon(
            vec![
                Vec2::new(-2., -2.),
                Vec2::new(2., -2.),
                Vec2::new(2., 2.),
                Vec2::new(-2., 2.),
            ],
            Vec2::splat(-1.),
            Vec2::ONE,
        );
        assert_eq!(p.len(), 4);
        assert!(p.iter().all(|p| p.abs().cmple(Vec2::ONE).all()));
        let area: f32 = (0..p.len())
            .map(|i| p[i].perp_dot(p[(i + 1) % p.len()]))
            .sum::<f32>()
            * 0.5;
        assert!((area - 4.).abs() < 0.0001);
    }
    fn contains_triangle(p: Vec2, [a, b, c]: [Vec2; 3]) -> bool {
        let cross = |x: Vec2, y: Vec2, z: Vec2| (y - x).perp_dot(z - x);
        let sides = [cross(a, b, p), cross(b, c, p), cross(c, a, p)];
        sides.iter().all(|v| *v >= -0.002) || sides.iter().all(|v| *v <= 0.002)
    }
    #[test]
    fn clipped_source_road_covers_both_lane_edges_for_the_entire_imported_kilometre() {
        let hills = Track::hills().unwrap();
        let m = content();
        let triangles: Vec<_> = m
            .surfaces
            .iter()
            .filter(|s| s.kind == 0)
            .flat_map(|s| {
                let v: Vec<_> = s.shape.vertices().map(Vec2::from_array).collect();
                s.shape
                    .triangles
                    .iter()
                    .map(move |t| t.map(|i| v[i as usize]))
            })
            .collect();
        assert!(triangles.len() > 100);
        for station in (0..=1000).step_by(10) {
            for lane in [-5.5, 0., 5.5] {
                let p = hills.street_sample(0, station as f32, lane).0.xz();
                assert!(
                    triangles.iter().any(|t| contains_triangle(p, *t)),
                    "imported road hole at {station}/{lane}: {p:?}"
                );
            }
        }
    }
    #[test]
    fn starting_grid_and_launch_corridor_follow_clear_source_street() {
        let hills = Track::hills().unwrap();
        let m = content();
        let roads: Vec<_> = m
            .surfaces
            .iter()
            .filter(|s| s.kind == 0)
            .flat_map(|s| {
                let v: Vec<_> = s.shape.vertices().map(Vec2::from_array).collect();
                s.shape
                    .triangles
                    .iter()
                    .map(move |t| t.map(|i| v[i as usize]))
            })
            .collect();
        for spawn in &hills.definition.spawns {
            for route in 0..2 {
                for ahead in (0..=180).step_by(3) {
                    let d = hills.grid_station(spawn.distance) + ahead as f32;
                    let (p, _) = hills.sample_route(route, d, spawn.lane);
                    let (projected, lane, _) = hills.project_route(route, p, d);
                    assert!(
                        (projected - d).abs() < 0.5 && (lane - spawn.lane).abs() < 0.1,
                        "projection route{route} station{d} got{projected} lane{} got{lane}",
                        spawn.lane
                    );
                    for dx in [-0.9, 0., 0.9] {
                        for dz in [-1.4, 0., 1.4] {
                            let point = p.xz() + Vec2::new(dx, dz);
                            assert!(
                                roads.iter().any(|t| contains_triangle(point, *t)),
                                "off street: {}/{d}: {point:?}",
                                spawn.id.as_str()
                            );
                            for b in &m.buildings {
                                let v: Vec<_> = b.shape.vertices().map(Vec2::from_array).collect();
                                assert!(
                                    !b.shape.triangles.iter().any(|t| contains_triangle(
                                        point,
                                        t.map(|i| v[i as usize])
                                    )),
                                    "building overlap: {}/{d}/{}",
                                    spawn.id.as_str(),
                                    b.id
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn clipping_original_triangulated_holes_does_not_fill_them() {
        let origin = Track::hills_city_source_origin().xz();
        let local = [
            [-2., -2.],
            [2., -2.],
            [2., 2.],
            [-2., 2.],
            [-1., -1.],
            [1., -1.],
            [1., 1.],
            [-1., 1.],
        ];
        let points: Vec<_> = local
            .map(|p| (Vec2::from_array(p) + origin).to_array())
            .to_vec();
        let shape = Shape {
            rings: vec![points[..4].to_vec(), points[4..].to_vec()],
            triangles: vec![
                [0, 1, 5],
                [0, 5, 4],
                [1, 2, 6],
                [1, 6, 5],
                [2, 3, 7],
                [2, 7, 6],
                [3, 0, 4],
                [3, 4, 7],
            ],
        };
        let result = clipped(&shape, Vec2::splat(-1.5), Vec2::splat(1.5)).unwrap();
        result.validate().unwrap();
        let vertices: Vec<_> = result.vertices().map(Vec2::from_array).collect();
        let mut area = 0.;
        for t in result.triangles {
            let [a, b, c] = t.map(|i| vertices[i as usize]);
            assert!(
                !contains_triangle(Vec2::ZERO, [a, b, c]),
                "clipping filled source courtyard"
            );
            area += (b - a).perp_dot(c - a).abs() * 0.5;
        }
        assert!(
            (area - 5.).abs() < 0.005,
            "source hole area changed: {area}"
        );
    }
}
