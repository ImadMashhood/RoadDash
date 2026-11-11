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
// Two original palette rows: sand and meadow. U is cross-shoulder distance,
// so the gravel edge ends within3.5m; UVs change only, not mesh or collision.
fn shoulder_material(images: Option<&mut Assets<Image>>) -> StandardMaterial {
    use bevy::{
        asset::RenderAssetUsages,
        image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    let Some(images) = images else {
        return StandardMaterial {
            base_color: Color::srgb(0.32, 0.37, 0.19),
            ..default()
        };
    };
    let sand = Vec3::new(0.69, 0.51, 0.29);
    let grass = Vec3::new(0.32, 0.37, 0.19);
    let mut pixels = Vec::new();
    for row in 0..2 {
        for x in 0..128 {
            let weight = ((x as f32 / 127. - 0.035) / 0.11).clamp(0., 1.);
            let c = if row == 0 {
                sand
            } else {
                sand.lerp(grass, weight)
            };
            pixels.extend([
                (c.x * 255.) as u8,
                (c.y * 255.) as u8,
                (c.z * 255.) as u8,
                255,
            ]);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: 128,
            height: 2,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        ..ImageSamplerDescriptor::linear()
    });
    StandardMaterial {
        base_color_texture: Some(images.add(image)),
        perceptual_roughness: 1.,
        ..default()
    }
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
    let shoulder = materials.add(shoulder_material(images));
    let terrain_material = materials.add(StandardMaterial {
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
            let local: Vec<_> = v.iter().map(|p| *p - origin).collect();
            let mut render = crate::environment::mesh(&local, &ix);
            let rows = v.len() / 4;
            render.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                (0..rows)
                    .flat_map(|i| {
                        let station = a + (b - a) * i as f32 / (rows - 1) as f32;
                        let green = ((5500. - station) / 300.).clamp(0., 1.);
                        [1., 0., 0., 1.].map(|u| [u, 0.25 + green * 0.5])
                    })
                    .collect::<Vec<_>>(),
            );
            surface(
                commands,
                meshes,
                shoulder.clone(),
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
            let center = origin + Vec2::splat(tile / 2.);
            let (station, _) = track.nearby_station(center.x, center.y);
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
            let base = if station < 5300. {
                Vec3::new(0.32, 0.37, 0.19)
            } else if station < 8500. {
                Vec3::new(0.69, 0.51, 0.29)
            } else {
                Vec3::new(0.76, 0.61, 0.38)
            };
            let origin = v[v.len() / 2];
            let local: Vec<_> = v.iter().map(|p| *p - origin).collect();
            let mut render = crate::environment::mesh(&local, &ix);
            render.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                v.iter()
                    .map(|p| {
                        let strata = (p.y * 0.42).sin() * 0.045;
                        let rock = Vec3::new(0.57 + strata, 0.36 + strata, 0.20 + strata);
                        let c = base.lerp(rock, track.hills_canyon_weight(p.x, p.z));
                        let linear = Color::srgb(c.x, c.y, c.z).to_linear();
                        [linear.red, linear.green, linear.blue, linear.alpha]
                    })
                    .collect::<Vec<_>>(),
            );
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
