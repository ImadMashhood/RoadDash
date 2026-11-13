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
/// The same authored sand palette drives terrain vertices and wheel dust.
pub(super) fn sand_color(station: f32, x: f32, z: f32) -> Vec3 {
    let base = if station < 8500. {
        Vec3::new(0.69, 0.51, 0.29)
    } else {
        Vec3::new(0.76, 0.61, 0.38)
    };
    let dune = (x * 0.019 + z * 0.013).sin() * (z * 0.026 - x * 0.009).sin();
    let grit = (x * 0.071 - z * 0.058).sin() * 0.25;
    let tint = dune * 0.055 + grit * 0.022;
    base + Vec3::new(tint, tint * 0.78, tint * 0.52)
}
/// A small original, tileable grit field adds close-up sand detail without
/// additional draw calls or runtime texture uploads.
fn terrain_grit() -> Image {
    use bevy::{
        asset::RenderAssetUsages,
        image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
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
    let mut image = Image::new(
        Extent3d {
            width: SIZE as u32,
            height: SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    image
}
/// Mipped blocks read as coarse ground detail nearby and average out at speed.
fn meadow_detail() -> Image {
    let mut pixels = Vec::with_capacity(64 * 64 * 4);
    for y in 0..64_u32 {
        for x in 0..64_u32 {
            let block = |x: u32, y: u32| {
                let key = x.wrapping_mul(374_761_393) ^ y.wrapping_mul(668_265_263);
                ((key ^ (key >> 13)).wrapping_mul(1_274_126_177) >> 26) as u8
            };
            let tone = 191 + block(x / 3, y / 2) * 3 / 4 + block(x, y) / 4;
            pixels.extend([tone, tone, tone, 255]);
        }
    }
    crate::road_detail::image(pixels, 64, true)
}

fn hill_color(p: Vec3, station: f32, face_up: f32, canyon: f32) -> Vec3 {
    let meadow = rr_core::surface::hills_meadow_weight(station);
    let patches = (p.x * 0.062 + p.z * 0.019).sin() * (p.z * 0.077).sin();
    let flecks = (p.x * 0.29 - p.z * 0.17).sin() * (p.z * 0.21).cos();
    let grass =
        Vec3::new(0.40, 0.49, 0.20) + Vec3::new(0.075, 0.070, 0.028) * (patches + flecks * 0.30);
    let base = sand_color(station, p.x, p.z).lerp(grass, meadow);

    let strata = (p.y * 0.85 + p.x * 0.12 + p.z * 0.05).sin();
    let stone = Vec3::new(0.61, 0.52, 0.44)
        + Vec3::new(0.079, 0.072, 0.068) * (strata * 0.72 + flecks * 0.28);
    let cut = ((0.82 + patches * 0.035 - face_up) / 0.13).clamp(0., 1.);
    base.lerp(stone, (cut * meadow).max(canyon))
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
    mut images: Option<&mut Assets<Image>>,
) {
    let mut rail_ground = super::guardrails::GroundSamples::default();
    let asphalt = materials.add(Color::srgb(0.16, 0.16, 0.155));
    let shoulder = materials.add(shoulder_material(images.as_deref_mut()));
    let terrain_material = materials.add(StandardMaterial {
        base_color_texture: images
            .as_deref_mut()
            .map(|images| images.add(terrain_grit())),
        perceptual_roughness: 1.,
        ..default()
    });
    let meadow_material = materials.add(StandardMaterial {
        base_color_texture: images.map(|images| images.add(meadow_detail())),
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
                        let green = rr_core::surface::hills_meadow_weight(station);
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
            let Some(bevy::mesh::VertexAttributeValues::Float32x3(normals)) =
                render.attribute(Mesh::ATTRIBUTE_NORMAL)
            else {
                unreachable!("terrain mesh has computed normals");
            };
            let colors = v
                .iter()
                .zip(normals)
                .map(|(p, normal)| {
                    let (station, _) = track.nearby_station(p.x, p.z);
                    let color =
                        hill_color(*p, station, normal[1], track.hills_canyon_weight(p.x, p.z));
                    let linear = Color::srgb(color.x, color.y, color.z).to_linear();
                    [linear.red, linear.green, linear.blue, linear.alpha]
                })
                .collect::<Vec<_>>();
            render.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            let (station, _) = track.nearby_station(origin.x, origin.z);
            let material = if station < 5500. {
                &meadow_material
            } else {
                &terrain_material
            };
            surface(
                commands,
                meshes,
                material.clone(),
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
    fn ground_grit_has_bounded_detail_and_sand_palette_covers_town_and_dunes() {
        let image = terrain_grit();
        let bytes = image.data.unwrap();
        assert_eq!(bytes.len(), 64 * 64 * 4);
        let values: Vec<_> = bytes.chunks_exact(4).map(|pixel| pixel[0]).collect();
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
