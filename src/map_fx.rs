//! Bounded, presentation-only map atmosphere. All meshes are allocated at world setup.
pub mod survey;
use crate::{
    game::{ChaseCamera, Phase, Race},
    track::Track,
};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

const SAND_COUNT: usize = 48;
const HEAT_COUNT: usize = 16;
const STEAM_COUNT: usize = 16;
const POLLEN_COUNT: usize = 16;
const SAND_STATIONS: [f32; 12] = [
    6500., 7000., 7500., 8000., 8500., 9000., 9500., 10000., 10500., 11000., 11500., 12000.,
];
const HEAT_STATIONS: [f32; 8] = [5900., 6500., 7100., 7900., 8900., 9900., 10900., 11800.];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Sand,
    Heat,
    Steam,
    Pollen,
}

#[derive(Component)]
pub struct MapFxSprite {
    kind: Kind,
    anchor: Vec3,
    road_rotation: Quat,
    phase: f32,
}

#[derive(Resource, Default)]
pub struct MapFxState;

/// The environment variable is deliberately read only during construction.
/// `0` disables all optional overdraw; `low` halves the default sprite count.
pub fn quality() -> usize {
    match std::env::var("ROADDASH_MAP_FX").as_deref() {
        Ok("0") => 0,
        Ok("low") => 1,
        _ => 2,
    }
}

fn soft_sprite() -> Image {
    let n = 64usize;
    let mut rgba = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let p = Vec2::new((x as f32 + 0.5) / n as f32, (y as f32 + 0.5) / n as f32) * 2.
                - Vec2::ONE;
            let radial = (1. - p.length_squared()).max(0.);
            let grain =
                0.75 + 0.15 * (p.x * 18. + p.y * 11.).sin() + 0.1 * (p.y * 27. - p.x * 9.).cos();
            rgba.extend([255, 255, 255, (radial.powi(2) * grain * 255.) as u8]);
        }
    }
    Image::new(
        Extent3d {
            width: n as u32,
            height: n as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

fn heat_sprite() -> Image {
    let n = 64usize;
    let mut rgba = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let u = (x as f32 + 0.5) / n as f32 * 2. - 1.;
            let v = (y as f32 + 0.5) / n as f32 * 2. - 1.;
            // Thin, uneven horizontal air bands rise past one another. This is
            // a light-scattering cue, not a screen-colour refraction pass.
            let ripple = (v * 20. + (u * 11. + v * 5.).sin() * 1.1)
                .sin()
                .abs()
                .powi(12);
            let envelope = (1. - u * u).max(0.).powi(2) * (1. - v * v).max(0.).powi(2);
            rgba.extend([255, 250, 226, (ripple * envelope * 255.) as u8]);
        }
    }
    Image::new(
        Extent3d {
            width: n as u32,
            height: n as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

fn budget(kind: Kind, quality: usize) -> usize {
    let full = match kind {
        Kind::Sand => SAND_COUNT,
        Kind::Heat => HEAT_COUNT,
        Kind::Steam => STEAM_COUNT,
        Kind::Pollen => POLLEN_COUNT,
    };
    full * quality / 2
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: Option<&mut Assets<Image>>,
    track: &Track,
) {
    let level = quality();
    if level == 0 {
        return;
    }
    let Some(images) = images else { return };
    let is_hills = track.is_hills();
    // Metropolis and the authored legacy city share one restrained vent-steam cue.
    let city = track.metropolis.is_some() || track.city_surface.is_some();
    let texture = images.add(soft_sprite());
    let heat_texture = if is_hills {
        Some(images.add(heat_sprite()))
    } else {
        None
    };
    let quad = meshes.add(Rectangle::new(1., 1.));
    let kinds: &[Kind] = if is_hills {
        &[Kind::Sand, Kind::Heat]
    } else if city {
        &[Kind::Steam]
    } else {
        &[Kind::Pollen]
    };
    for &kind in kinds {
        let color = match kind {
            Kind::Sand => Color::srgba(0.87, 0.72, 0.48, 0.16),
            Kind::Heat => Color::srgba(0.97, 0.82, 0.59, 0.17),
            Kind::Steam => Color::srgba(0.72, 0.78, 0.82, 0.13),
            Kind::Pollen => Color::srgba(0.78, 0.83, 0.61, 0.13),
        };
        let image = if kind == Kind::Heat {
            heat_texture.clone().unwrap()
        } else {
            texture.clone()
        };
        // One immutable asset per cue avoids a render-asset upload every frame.
        let material = materials.add(StandardMaterial {
            base_color: color,
            base_color_texture: Some(image),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            double_sided: true,
            cull_mode: None,
            ..default()
        });
        for i in 0..budget(kind, level) {
            let station = match kind {
                Kind::Sand => SAND_STATIONS[i % 12] + (i / 12) as f32 * 20.,
                Kind::Heat => HEAT_STATIONS[i % 8] + (i / 8) as f32 * 12.,
                Kind::Steam if track.metropolis.is_some() => {
                    8300. + (i % 8) as f32 * 330. + (i / 8) as f32 * 12.
                }
                Kind::Steam => {
                    track.race_interval().start
                        + 240.
                        + (i % 8) as f32 * 320.
                        + (i / 8) as f32 * 12.
                }
                Kind::Pollen => track.length * (i as f32 + 0.5) / budget(kind, level) as f32,
            }
            .min(track.length - 10.);
            let route = if is_hills && i % 4 == 0 { 1 } else { 0 };
            let (_, rotation) = track.sample_route(route, station, 0.);
            let side = if i % 2 == 0 { 1. } else { -1. };
            let offset = match kind {
                Kind::Sand => 14. + (i % 3) as f32 * 6.,
                Kind::Heat => 16. + (i % 3) as f32 * 7.,
                Kind::Steam => 9. + (i % 3) as f32 * 2.,
                Kind::Pollen => 13. + (i % 3) as f32 * 5.,
            };
            let (mut anchor, _) = track.sample_route(route, station, -side * offset);
            anchor.y += 0.5;
            commands.spawn((
                Name::new(match kind {
                    Kind::Sand => "Blowing sand",
                    Kind::Heat => "Ground heat cue",
                    Kind::Steam => "City vent steam",
                    Kind::Pollen => "Valley air motes",
                }),
                MapFxSprite {
                    kind,
                    anchor,
                    road_rotation: rotation,
                    phase: i as f32 * 2.399_963_1,
                },
                Mesh3d(quad.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(anchor),
                Visibility::Hidden,
                bevy::light::NotShadowCaster,
                bevy::light::NotShadowReceiver,
            ));
        }
    }
    commands.insert_resource(MapFxState::default());
}

fn visual(kind: Kind, time: f32, phase: f32) -> (Vec3, Vec3, f32) {
    let wave = time + phase;
    match kind {
        Kind::Sand => {
            // A fixed envelope slides across the shoulder; no sand contact or surface changes.
            let travel = (wave * 0.32).fract();
            let fade = (travel * (1. - travel) * 4.).powi(2);
            (
                Vec3::new(
                    (wave * 0.8).sin() * 0.6,
                    0.25 + (wave * 1.9).sin().abs() * 0.4,
                    (travel - 0.5) * 20.,
                ),
                Vec3::new(3.8, 1.1, 1.),
                0.18 * fade,
            )
        }
        Kind::Heat => (
            Vec3::new((wave * 1.3).sin() * 0.35, (wave * 0.13).fract() * 0.9, 0.),
            Vec3::new(5., 1.35, 1.),
            0.13 + 0.05 * (wave * 3.1).sin().abs(),
        ),
        Kind::Steam => {
            let cycle = (wave * 0.17).fract();
            (
                Vec3::new(
                    (wave * 0.5).sin() * 0.8,
                    cycle * 4.,
                    (wave * 0.3).cos() * 0.5,
                ),
                Vec3::splat(1.2 + cycle * 2.),
                0.13 * (1. - cycle).powi(2),
            )
        }
        Kind::Pollen => (
            Vec3::new(
                (wave * 0.3).sin() * 2.,
                1. + (wave * 0.8).sin() * 0.6,
                (wave * 0.22).cos() * 2.,
            ),
            Vec3::splat(0.8),
            0.12 + 0.03 * (wave * 2.).sin().abs(),
        ),
    }
}

pub fn animate(
    race: Res<Race>,
    camera: Query<&Transform, With<ChaseCamera>>,
    mut sprites: Query<(&MapFxSprite, &mut Transform, &mut Visibility)>,
) {
    let Ok(camera) = camera.single() else { return };
    // Fixed-loop race time gives the same air pose across variable render rates.
    let elapsed = race.elapsed.max(0.);
    for (sprite, mut transform, mut visibility) in &mut sprites {
        let range = match sprite.kind {
            Kind::Sand => 125.,
            Kind::Heat => 115.,
            Kind::Steam => 100.,
            Kind::Pollen => 90.,
        };
        if sprite.anchor.distance_squared(camera.translation) > range * range
            || race.phase == Phase::Garage
        {
            *visibility = Visibility::Hidden;
            continue;
        }
        let (offset, size, alpha) = visual(sprite.kind, elapsed, sprite.phase);
        transform.translation = sprite.anchor
            + sprite.road_rotation * Vec3::new(offset.x, 0., offset.z)
            + Vec3::Y * offset.y;
        transform.rotation = camera.rotation;
        // Scale the soft sprite at the gust/plume envelope instead of changing
        // materials, keeping the render asset immutable after world setup.
        let peak = match sprite.kind {
            Kind::Sand => 0.18,
            Kind::Heat => 0.18,
            Kind::Steam => 0.13,
            Kind::Pollen => 0.15,
        };
        let envelope = (alpha / peak).clamp(0., 1.).sqrt();
        transform.scale = size * envelope;
        *visibility = if envelope > 0.08 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_budgets_and_temporal_envelopes() {
        assert_eq!(budget(Kind::Sand, 2), 48);
        assert_eq!(budget(Kind::Heat, 1), 8);
        assert_eq!(budget(Kind::Steam, 0), 0);
        assert_eq!(budget(Kind::Pollen, 2), 16);
        for kind in [Kind::Sand, Kind::Heat, Kind::Steam, Kind::Pollen] {
            for tick in 0..1000 {
                let (offset, scale, alpha) = visual(kind, tick as f32 / 60., 1.4);
                assert!(offset.is_finite() && scale.is_finite());
                assert!((0. ..=0.2).contains(&alpha));
            }
        }
        assert_eq!(soft_sprite().data.unwrap().len(), 64 * 64 * 4);
        assert_eq!(heat_sprite().data.unwrap().len(), 64 * 64 * 4);
    }
}
