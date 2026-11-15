use super::*;
use crate::world::Ground;
use bevy_rapier3d::prelude::*;

pub(super) fn surface(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    vertices: Vec<Vec3>,
    indices: Vec<[u32; 3]>,
    road: bool,
    render: Option<Mesh>,
) {
    let origin = vertices[vertices.len() / 2];
    let local: Vec<_> = vertices.iter().map(|p| *p - origin).collect();
    let entity = commands
        .spawn((
            Mesh3d(
                meshes.add(render.unwrap_or_else(|| crate::environment::mesh(&local, &indices))),
            ),
            MeshMaterial3d(material),
            Transform::from_translation(origin),
            RigidBody::Fixed,
            Collider::trimesh(local, indices).expect("Hills surface"),
            Ground,
            HillsSolid,
            Friction::coefficient(0.85),
        ))
        .id();
    if road {
        commands
            .entity(entity)
            .insert(crate::environment::RoadChunk);
    } else {
        commands.entity(entity).insert(crate::environment::Terrain);
    }
}
pub(super) fn ribbon(
    track: &Track,
    route: usize,
    a: f32,
    b: f32,
    half: f32,
    offset: f32,
) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let count = ((b - a) / 1.25).ceil() as usize;
    let mut v = Vec::new();
    let mut ix = Vec::new();
    for i in 0..=count {
        let s = a + (b - a) * i as f32 / count as f32;
        for side in [-1., 1.] {
            let (p, q) = track.sample_route(route, s, side * half);
            v.push(p + q * Vec3::Y * offset);
        }
        if i < count {
            let k = 2 * i as u32;
            ix.extend([[k, k + 1, k + 2], [k + 1, k + 3, k + 2]]);
        }
    }
    (v, ix)
}
fn shoulders(track: &Track, route: usize, a: f32, b: f32) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let count = ((b - a) / 1.25).ceil() as usize;
    let mut v = Vec::new();
    let mut ix = Vec::new();
    for i in 0..=count {
        let s = a + (b - a) * i as f32 / count as f32;
        let half = track.half_width(s);
        for lane in [-30., -half, half, 30.] {
            let (p, q) = track.sample_route(route, s, lane);
            v.push(p - q * Vec3::Y * 0.08);
        }
        if i < count {
            let k = i as u32 * 4;
            ix.extend([
                [k, k + 1, k + 4],
                [k + 1, k + 5, k + 4],
                [k + 2, k + 3, k + 6],
                [k + 3, k + 7, k + 6],
            ]);
        }
    }
    (v, ix)
}

/// Close the visible cut face where each shoulder meets the canyon. The road
/// ribbon and shoulder collider still use their original physical topology.
fn cap_bridge_shoulders(
    track: &Track,
    edge: &[Vec3],
    entry: bool,
    vertices: &mut Vec<Vec3>,
    triangles: &mut Vec<[u32; 3]>,
) {
    const SEGMENTS: u32 = 12;
    for (outer, inner) in [(edge[0], edge[1]), (edge[2], edge[3])] {
        let first = vertices.len() as u32;
        for step in 0..=SEGMENTS {
            let top = outer.lerp(inner, step as f32 / SEGMENTS as f32);
            let terrain = ground_height(track, top.x, top.z);
            vertices.extend([top, top.with_y(top.y.min(terrain) - 0.15)]);
            if step < SEGMENTS {
                let i = first + step * 2;
                let mut strip = [[i, i + 1, i + 2], [i + 2, i + 1, i + 3]];
                if entry {
                    for triangle in &mut strip {
                        triangle.swap(1, 2);
                    }
                }
                triangles.extend(strip);
            }
        }
    }
}

fn terrain_vertex(track: &Track, x: f32, z: f32) -> Vec3 {
    super::terrain_cache::vertex(track, x, z, || terrain_vertex_uncached(track, x, z))
}
fn terrain_vertex_uncached(track: &Track, x: f32, z: f32) -> Vec3 {
    let mut p = Vec3::new(x, track.terrain_height(x, z), z);
    let (s, l) = track.nearby_station(p.x, p.z);
    if l.abs() < 18. {
        let weight = ((18. - l.abs()) / 6.).clamp(0., 1.);
        p.y =
            p.y.lerp(p.y.min(track.sample_route(0, s, 0.).0.y - 1.5), weight);
    }
    if (5300.0..7600.0).contains(&s) {
        let (s, _, dist) = track.project_route(1, p, s);
        if dist < 18. {
            let weight = ((18. - dist) / 6.).clamp(0., 1.);
            p.y =
                p.y.lerp(p.y.min(track.sample_route(1, s, 0.).0.y - 1.5), weight);
        }
    }
    p
}
/// Barycentric height of the exact resident eight-metre terrain triangles.
pub(super) fn ground_height(track: &Track, x: f32, z: f32) -> f32 {
    let origin = track.bounds.0.xz() - Vec2::splat(320.);
    let cell = ((Vec2::new(x, z) - origin) / 8.).floor();
    let base = origin + cell * 8.;
    let t = (Vec2::new(x, z) - base) / 8.;
    let a = terrain_vertex(track, base.x, base.y).y;
    let b = terrain_vertex(track, base.x + 8., base.y).y;
    let c = terrain_vertex(track, base.x, base.y + 8.).y;
    let d = terrain_vertex(track, base.x + 8., base.y + 8.).y;
    if t.x + t.y <= 1. {
        a + (b - a) * t.x + (c - a) * t.y
    } else {
        d + (c - d) * (1. - t.x) + (b - d) * (1. - t.y)
    }
}
pub(super) fn crossing_support(track: &Track, route: usize, p: Vec3, near: f32) -> f32 {
    let ground = ground_height(track, p.x, p.z);
    ground.max(crossing_road_support(track, route, p, near))
}

pub(super) fn crossing_road_support(track: &Track, route: usize, p: Vec3, near: f32) -> f32 {
    let (station, _, distance) = track.project_route(route, p, near);
    if distance >= 30. || (route == 0 && bridge::on_deck(station)) {
        return f32::NEG_INFINITY;
    }
    let (center, frame) = track.sample_route(route, station, 0.);
    let side = ((center - p).dot((frame * Vec3::X).with_y(0.))).signum();
    let half = track.half_width(station);
    let (inner, iq) = track.sample_route(route, station, side * half);
    let width = center.xz().distance(inner.xz());
    if distance <= width {
        let (road, q) = track.sample_route(route, station, side * half * distance / width);
        (road + q * Vec3::Y * 0.025).y
    } else {
        let (outer, oq) = track.sample_route(route, station, side * 30.);
        let outer_width = center.xz().distance(outer.xz());
        let weight = ((distance - width) / (outer_width - width)).clamp(0., 1.);
        (inner - iq * Vec3::Y * 0.08)
            .y
            .lerp((outer - oq * Vec3::Y * 0.08).y, weight)
    }
}

/// The same authored sand palette drives terrain vertices and wheel dust.
pub(super) fn sand_color(station: f32, x: f32, z: f32) -> Vec3 {
    let t = ((station - 8250.) / 500.).clamp(0., 1.);
    let base = Vec3::new(0.69, 0.51, 0.29).lerp(Vec3::new(0.76, 0.61, 0.38), t * t * (3. - 2. * t));
    let dune = (x * 0.019 + z * 0.013).sin() * (z * 0.026 - x * 0.009).sin();
    let grit = (x * 0.071 - z * 0.058).sin() * 0.25;
    let tint = dune * 0.055 + grit * 0.022;
    base + Vec3::new(tint, tint * 0.78, tint * 0.52)
}
/// A small original, tileable grit field adds close-up sand detail without
/// additional draw calls or runtime texture uploads.
fn terrain_grit() -> Image {
    const SIZE: usize = 64;
    let mut pixels = Vec::with_capacity(SIZE * SIZE * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let a = std::f32::consts::TAU * x as f32 / SIZE as f32;
            let b = std::f32::consts::TAU * y as f32 / SIZE as f32;
            let value = (0.94
                + 0.035 * (a * 2.).sin() * (b * 3.).cos()
                + 0.018 * (a * 9. + b * 7.).sin()
                + 0.012 * (a * 17. - b * 15.).cos())
            .clamp(0.86, 1.0);
            let channel = (value * 255.) as u8;
            pixels.extend([channel, channel, channel, 255]);
        }
    }
    crate::road_detail::image(pixels, SIZE, true)
}
// A visual-only ramp picks up grass in patches past the imported urban slice.
pub(super) fn city_exit_meadow(station: f32, x: f32, z: f32) -> f32 {
    let progress = ((station - 1000.) / 640.).clamp(0., 1.);
    if progress == 0. || progress == 1. {
        return progress;
    }
    let footprint = (x * 0.061 + z * 0.039).sin() * (z * 0.094 - x * 0.018).sin();
    let t = (progress + footprint * progress * (1. - progress) * 0.65).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

fn fringe_soil(station: f32, p: Vec3, meadow: f32, patches: f32) -> Vec3 {
    let sand = sand_color(station, p.x, p.z);
    let fringe = Vec3::new(0.42, 0.40, 0.34) + Vec3::splat(patches * 0.023);
    fringe.lerp(sand, meadow)
}

fn hill_color(p: Vec3, station: f32, face_up: f32, canyon: f32) -> Vec3 {
    let exit_meadow = city_exit_meadow(station, p.x, p.z);
    let meadow = rr_core::surface::hills_meadow_weight(station) * exit_meadow;
    let patches = (p.x * 0.062 + p.z * 0.019).sin() * (p.z * 0.077).sin();
    let flecks = (p.x * 0.29 - p.z * 0.17).sin() * (p.z * 0.21).cos();
    let grass =
        Vec3::new(0.40, 0.49, 0.20) + Vec3::new(0.075, 0.070, 0.028) * (patches + flecks * 0.30);
    let soil = fringe_soil(station, p, exit_meadow, patches);
    let base = soil.lerp(grass, meadow);

    let strata = (p.y * 0.85 + p.x * 0.12 + p.z * 0.05).sin();
    let stone = Vec3::new(0.61, 0.52, 0.44)
        + Vec3::new(0.079, 0.072, 0.068) * (strata * 0.72 + flecks * 0.28);
    let cut = ((0.82 + patches * 0.035 - face_up) / 0.13).clamp(0., 1.);
    base.lerp(stone, (cut * meadow).max(canyon))
}

fn terrain_color(track: &Track, p: Vec3, normal: [f32; 3]) -> [f32; 4] {
    let (station, _) = track.nearby_station(p.x, p.z);
    let color = hill_color(p, station, normal[1], track.hills_canyon_weight(p.x, p.z));
    Color::srgb(color.x, color.y, color.z)
        .to_linear()
        .to_f32_array()
}
fn terrain_normal(track: &Track, p: Vec3) -> [f32; 3] {
    const STEP: f32 = 8.;
    let dx = terrain_vertex(track, p.x + STEP, p.z).y - terrain_vertex(track, p.x - STEP, p.z).y;
    let dz = terrain_vertex(track, p.x, p.z + STEP).y - terrain_vertex(track, p.x, p.z - STEP).y;
    Vec3::new(-dx, 2. * STEP, -dz).normalize().to_array()
}
pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
    census: &mut Census,
    images: Option<&mut Assets<Image>>,
) {
    let mut rail_ground = super::guardrails::GroundSamples::default();
    let asphalt = materials.add(Color::srgb(0.16, 0.16, 0.155));
    let terrain_material = materials.add(StandardMaterial {
        base_color_texture: images.map(|images| images.add(terrain_grit())),
        perceptual_roughness: 1.,
        ..default()
    });
    for route in 0..2 {
        let (start, end) = if route == 0 {
            (-300., track.length + 60.)
        } else {
            (5500., 7400.)
        };
        let mut a = start;
        while a < end {
            let mut b = (a + 80.).min(end);
            if route == 0 {
                for boundary in [0., 1000., bridge::START, bridge::END] {
                    if a < boundary {
                        b = b.min(boundary);
                    }
                }
            }
            if route == 0 && a >= -300. && b <= 1000. {
                // The imported Metropolis slice owns its exact asphalt/sidewalks.
                a = b;
                continue;
            }
            let (v, ix) = ribbon(track, route, a, b, track.half_width(a), 0.025);
            surface(commands, meshes, asphalt.clone(), v, ix, true, None);
            census.road_chunks += 1;
            if route == 0 && bridge::on_deck((a + b) * 0.5) {
                // The canyon has no hidden thirty-metre shoulder shelf.
                a = b;
                continue;
            }
            let (v, ix) = shoulders(track, route, a, b);
            if route == 0 && b >= 1580. && a <= 5380. {
                rail_ground.add(&v, &ix);
            }
            let origin = v[v.len() / 2];
            let mut render_vertices = std::borrow::Cow::Borrowed(v.as_slice());
            let mut render_triangles = std::borrow::Cow::Borrowed(ix.as_slice());
            let mut cap_uv_direction = Vec2::ZERO;
            if route == 0 && (a == bridge::END || b == bridge::START) {
                let edge = if a == bridge::END {
                    &v[..4]
                } else {
                    &v[v.len() - 4..]
                };
                cap_uv_direction = (edge[3] - edge[0]).xz().perp().normalize_or_zero();
                cap_bridge_shoulders(
                    track,
                    edge,
                    b == bridge::START,
                    render_vertices.to_mut(),
                    render_triangles.to_mut(),
                );
            }
            let local: Vec<_> = render_vertices.iter().map(|p| *p - origin).collect();
            let mut render = crate::environment::mesh(&local, &render_triangles);
            render.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                render_vertices
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        // Carry the shoulder UV over the lip, then unwrap down the cut face.
                        let drop = if i >= v.len() && !(i - v.len()).is_multiple_of(2) {
                            p.y - render_vertices[i - 1].y
                        } else {
                            0.
                        };
                        ((p.xz() + cap_uv_direction * drop) * 0.18).to_array()
                    })
                    .collect::<Vec<_>>(),
            );
            render.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                render_vertices
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| {
                        if i >= v.len() || matches!(i % 4, 0 | 3) {
                            terrain_color(
                                track,
                                p.with_y(ground_height(track, p.x, p.z)),
                                terrain_normal(track, p),
                            )
                        } else {
                            let (station, _) = track.nearby_station(p.x, p.z);
                            let meadow = city_exit_meadow(station, p.x, p.z);
                            let c = fringe_soil(station, p, meadow, 0.) * 0.92;
                            Color::srgb(c.x, c.y, c.z).to_linear().to_f32_array()
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            if let Some(bevy::mesh::VertexAttributeValues::Float32x3(normals)) =
                render.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
            {
                for (i, (normal, &p)) in normals.iter_mut().zip(&v).enumerate() {
                    if matches!(i % 4, 0 | 3) {
                        *normal = terrain_normal(track, p);
                    }
                }
            }
            surface(
                commands,
                meshes,
                terrain_material.clone(),
                v,
                ix,
                false,
                Some(render),
            );
            a = b;
        }
        let mut d = start;
        while d < end {
            let (p, q) = track.sample_route(route, d, 0.);
            // Dashed centre lines keep the split legible without overpainting the junction.
            if !(0. ..1000.).contains(&d)
                && !(5470.0..5540.0).contains(&d)
                && !(7360.0..7440.0).contains(&d)
            {
                crate::world::part(
                    commands,
                    art.cube.clone(),
                    art.white.clone(),
                    p + q * Vec3::Y * 0.06,
                    Vec3::new(0.14, 0.015, 5.),
                    q,
                );
            }
            d += 16.;
        }
    }
    // Resident heightfield with shared render/collision topology; near-road surface
    // is recessed under the finely tessellated shoulder, so crests cannot bury lanes.
    let terrain_span = crate::diagnostics::loading::Span::new("hills_terrain");
    let (min, max) = track.bounds;
    let tile = 160.;
    let step = 8.;
    let nx = ((max.x - min.x + 640.) / tile).ceil() as usize;
    let nz = ((max.z - min.z + 640.) / tile).ceil() as usize;
    for x in 0..nx {
        for z in 0..nz {
            let origin = Vec2::new(
                min.x - 320. + x as f32 * tile,
                min.z - 320. + z as f32 * tile,
            );
            let (v, ix) = if std::env::var("ROADDASH_TERRAIN_CACHE").as_deref() == Ok("0") {
                let (mut v, ix) = crate::world_gen::ground::terrain_grid(track, origin, tile, step);
                for p in &mut v {
                    *p = terrain_vertex_uncached(track, p.x, p.z);
                }
                (v, ix)
            } else {
                crate::world_gen::ground::terrain_grid_with_height(origin, tile, step, |p| {
                    terrain_vertex_uncached(track, p.x, p.y).y
                })
            };
            let origin = v[v.len() / 2];
            let local: Vec<_> = v.iter().map(|p| *p - origin).collect();
            let mut render = crate::environment::mesh(&local, &ix);
            render.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                v.iter()
                    .map(|p| [p.x * 0.18, p.z * 0.18])
                    .collect::<Vec<_>>(),
            );
            // Sample both sides of the shared edge to keep tile lighting continuous.
            let normals: Vec<_> = v.iter().map(|&p| terrain_normal(track, p)).collect();
            let colors: Vec<_> = v
                .iter()
                .zip(&normals)
                .map(|(&p, &normal)| terrain_color(track, p, normal))
                .collect();
            render.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            render.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            surface(
                commands,
                meshes,
                terrain_material.clone(),
                v,
                ix,
                false,
                Some(render),
            );
            census.terrain_chunks += 1;
        }
    }
    drop(terrain_span);
    super::guardrails::spawn(commands, art, track, census, &rail_ground);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridge_retaining_faces_join_the_exact_shoulder_lips_and_face_the_span() {
        let track = Track::hills().unwrap();
        for (station, entry) in [(bridge::START, true), (bridge::END, false)] {
            let (physical, _) = if entry {
                shoulders(&track, 0, station - 1.25, station)
            } else {
                shoulders(&track, 0, station, station + 1.25)
            };
            let edge = if entry {
                &physical[4..]
            } else {
                &physical[..4]
            };
            let mut vertices = Vec::new();
            let mut triangles = Vec::new();
            cap_bridge_shoulders(&track, edge, entry, &mut vertices, &mut triangles);
            assert_eq!(triangles.len(), 48);
            assert_eq!(
                [vertices[0], vertices[24], vertices[26], vertices[50]].as_slice(),
                edge
            );
            let forward = track.sample(station, 0.).1 * Vec3::Z;
            for triangle in triangles {
                let [a, b, c] = triangle.map(|i| vertices[i as usize]);
                let towards_span = if entry { forward } else { -forward };
                assert!((b - a).cross(c - a).dot(towards_span) > 0.);
            }
            for pair in vertices.chunks_exact(2) {
                assert!(pair[1].y <= ground_height(&track, pair[0].x, pair[0].z) - 0.149);
            }
        }
    }

    #[test]
    fn city_exit_is_neutral_at_the_boundary_and_greens_gradually() {
        let mut previous = 0.;
        for station in (1000..=1640).step_by(8) {
            let cover = city_exit_meadow(station as f32, 320., 130.);
            assert!(cover >= previous);
            assert!(cover - previous < 0.03);
            previous = cover;
        }
        assert_eq!(city_exit_meadow(1000., 320., 130.), 0.);
        assert_eq!(city_exit_meadow(1640., 320., 130.), 1.);
        let neutral = hill_color(Vec3::ZERO, 1000., 1., 0.);
        let meadow = hill_color(Vec3::ZERO, 1640., 1., 0.);
        assert!(neutral.x > neutral.y && meadow.y > meadow.x);
    }

    #[test]
    fn mounted_crossing_support_clears_recessed_shoulders() {
        let track = Track::hills().unwrap();
        for (route, station) in [(0, 6060.), (1, 6820.), (0, 7740.)] {
            let (vertices, indices) = shoulders(&track, route, station - 2., station + 2.);
            let mut shoulders = super::super::guardrails::GroundSamples::default();
            shoulders.add(&vertices, &indices);
            for lane in [-13., -10., 10., 13.] {
                let (p, _) = track.sample_route(route, station, lane);
                let expected = shoulders.height(&track, p);
                let support = crossing_support(&track, route, p, station);
                assert!(
                    (support - expected).abs() < 0.12,
                    "{route} {station} {lane}"
                );
            }
        }
    }

    #[test]
    fn ground_grit_has_bounded_detail_and_sand_palette_covers_town_and_dunes() {
        let image = terrain_grit();
        let bytes = image.data.unwrap();
        assert_eq!(image.texture_descriptor.mip_level_count, 7);
        let values: Vec<_> = bytes[..64 * 64 * 4]
            .chunks_exact(4)
            .map(|pixel| pixel[0])
            .collect();
        assert!(values.iter().min().is_some_and(|v| *v >= 219));
        assert!(values.iter().max().is_some_and(|v| *v >= 250));
        assert!(*values.iter().max().unwrap() - *values.iter().min().unwrap() > 18);
        let town = sand_color(7000., 125., 90.);
        let dunes = sand_color(10000., 125., 90.);
        assert!(dunes.x > town.x && dunes.y > town.y && dunes.z > town.z);
    }
    #[test]
    fn direct_grid_sampling_preserves_resident_mesh_vertices_and_triangles() {
        let track = Track::hills().unwrap();
        for station in [0., 1600., 2800., 4740., 6800., 9200., 12400.] {
            let p = track.sample(station, 0.).0;
            let origin = p.xz() - Vec2::splat(80.);
            let (mut old, indices) =
                crate::world_gen::ground::terrain_grid(&track, origin, 160., 8.);
            for vertex in &mut old {
                *vertex = terrain_vertex_uncached(&track, vertex.x, vertex.z);
            }
            let (new, direct_indices) =
                crate::world_gen::ground::terrain_grid_with_height(origin, 160., 8., |p| {
                    terrain_vertex_uncached(&track, p.x, p.y).y
                });
            assert_eq!(old, new);
            assert_eq!(indices, direct_indices);
        }
    }
    #[test]
    fn hills_roads_and_shoulders_face_up_without_spanning_live_lanes() {
        let track = Track::hills().unwrap();
        for route in 0..2 {
            for s in [0., 1500., 2600., 4000., 5500., 6200., 7400., 9800., 12400.] {
                let (v, ix) = ribbon(&track, route, s, s + 20., 6.5, 0.025);
                for tri in ix {
                    let [a, b, c] = tri.map(|i| v[i as usize]);
                    assert!((b - a).cross(c - a).y > 0.);
                }
                let (v, ix) = shoulders(&track, route, s, s + 20.);
                for tri in ix {
                    let [a, b, c] = tri.map(|i| v[i as usize]);
                    assert!((b - a).cross(c - a).y > 0.);
                    let p = (a + b + c) / 3.;
                    let (_, lane, _) = track.project_route(route, p, s);
                    assert!(lane.abs() > 6.4, "shoulder crosses lane at {s}: {lane}");
                }
            }
        }
    }
}
