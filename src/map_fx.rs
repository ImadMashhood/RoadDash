//! Bounded, presentation-only map atmosphere. All meshes are allocated at world setup.
pub(crate) mod braking;
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

use rr_core::surface::Surface;

pub(crate) fn wheel_spray(
    surface: Surface,
    speed: f32,
    load: f32,
    bike: Option<&crate::vehicle::BikeState>,
) -> f32 {
    if !matches!(
        surface,
        Surface::Sand | Surface::Dirt | Surface::Gravel | Surface::Grass
    ) || bike.is_some_and(|bike| !bike.grounded[0])
    {
        return 0.;
    }
    let slip = bike.map_or(0., |bike| bike.slip.abs().max(bike.slip_angle.abs()));
    ((speed.abs() - 2.) / 16.).clamp(0., 1.) * (0.23 + 0.27 * load + 0.5 * slip.clamp(0., 1.))
}

const SAND_COUNT: usize = 48;
const STEAM_COUNT: usize = 16;
const POLLEN_COUNT: usize = 16;
const SAND_START: f32 = 6500.;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Sand,
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

#[derive(Resource)]
pub struct MapFxState;

const SAND_ALPHA_LEVELS: usize = 16;

#[derive(Resource)]
pub struct SandPlumeMaterials([Handle<StandardMaterial>; SAND_ALPHA_LEVELS]);

/// The environment variable is deliberately read only during construction.
/// `0` disables all optional overdraw; `low` halves the default sprite count.
pub fn quality() -> usize {
    match std::env::var("ROADDASH_MAP_FX").as_deref() {
        Ok("0") => 0,
        Ok("low") => 1,
        _ => 2,
    }
}

pub(crate) fn soft_sprite() -> Image {
    let n = 64usize;
    let mut rgba = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let p = Vec2::new((x as f32 + 0.5) / n as f32, (y as f32 + 0.5) / n as f32) * 2.
                - Vec2::ONE;
            let radial = (1. - p.length_squared()).max(0.);
            // Low-frequency variation survives filtering as a soft plume, not a grid.
            let grain =
                0.9 + 0.07 * (p.x * 5. + p.y * 3.).sin() + 0.03 * (p.y * 7. - p.x * 2.).cos();
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

fn dust_sprite() -> Image {
    let n = 64usize;
    let mut rgba = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let p = Vec2::new((x as f32 + 0.5) / n as f32, (y as f32 + 0.5) / n as f32) * 2.
                - Vec2::ONE;
            let edge = (1. - p.length_squared()).max(0.).powf(0.8);
            let lobe = |center: Vec2, spread: f32| (-(p - center).length_squared() / spread).exp();
            // An asymmetric, fully feathered cloud mask. No external particle art.
            let billows = 0.57
                + 0.42 * lobe(Vec2::new(-0.32, -0.05), 0.28)
                + 0.38 * lobe(Vec2::new(0.24, 0.18), 0.23)
                + 0.33 * lobe(Vec2::new(0.05, -0.28), 0.22);
            rgba.extend([255, 255, 255, (edge * billows.min(1.) * 255.) as u8]);
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

fn steam_sprite() -> Image {
    let n = 64usize;
    let mut rgba = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let p = Vec2::new((x as f32 + 0.5) / n as f32, (y as f32 + 0.5) / n as f32) * 2.
                - Vec2::ONE;
            let sway = 0.12 * (p.y * 3.4).sin();
            let width = 0.38 + 0.12 * (p.y + 1.);
            let core = (1. - ((p.x - sway) / width).powi(2)).max(0.).powf(1.2);
            let ends = (1. - p.y * p.y).max(0.).powf(0.8);
            let soft_lobes = 0.72 + 0.16 * (p.y * 4.1 + p.x * 2.).sin();
            rgba.extend([255, 255, 255, (core * ends * soft_lobes * 255.) as u8]);
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
    braking::spawn(commands, meshes, materials, images, level);
    let is_hills = track.is_hills();
    // Metropolis and the authored legacy city share one restrained vent-steam cue.
    let city = track.metropolis.is_some() || track.city_surface.is_some();
    let texture = images.add(if city { steam_sprite() } else { soft_sprite() });
    let dust_texture = if is_hills {
        Some(images.add(dust_sprite()))
    } else {
        None
    };
    let quad = meshes.add(Rectangle::new(1., 1.));
    let kinds: &[Kind] = if is_hills {
        &[Kind::Sand]
    } else if city {
        &[Kind::Steam]
    } else {
        &[Kind::Pollen]
    };
    for &kind in kinds {
        let color = match kind {
            Kind::Sand => Color::srgba(0.76, 0.57, 0.35, 0.0),
            Kind::Steam => Color::srgba(0.72, 0.76, 0.78, 0.48),
            Kind::Pollen => Color::srgba(0.78, 0.83, 0.61, 0.35),
        };
        let image = match kind {
            Kind::Sand => dust_texture.clone().unwrap(),
            Kind::Steam | Kind::Pollen => texture.clone(),
        };
        let template = StandardMaterial {
            base_color: color,
            base_color_texture: Some(image),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            double_sided: true,
            cull_mode: None,
            ..default()
        };
        let material = if kind == Kind::Sand {
            let levels = std::array::from_fn(|i| {
                materials.add(StandardMaterial {
                    base_color: color.with_alpha(0.70 * i as f32 / (SAND_ALPHA_LEVELS - 1) as f32),
                    ..template.clone()
                })
            });
            let first = levels[0].clone();
            commands.insert_resource(SandPlumeMaterials(levels));
            first
        } else {
            materials.add(template)
        };
        for i in 0..budget(kind, level) {
            let station = match kind {
                Kind::Sand => {
                    SAND_START
                        + (i as f32 + 0.5) * (track.length - SAND_START)
                            / budget(kind, level) as f32
                }
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
                Kind::Sand => 7.2 + (i % 3) as f32 * 0.5,
                Kind::Steam => track.half_width(station) + 0.7,
                Kind::Pollen => 13. + (i % 3) as f32 * 5.,
            };
            let (mut anchor, _) = track.sample_route(route, station, -side * offset);
            anchor.y = if is_hills {
                crate::world_gen::hills::rendered_ground_height(track, anchor.x, anchor.z)
            } else {
                track.terrain_height(anchor.x, anchor.z)
            } + match kind {
                Kind::Sand => 2.7,
                Kind::Steam => 0.75,
                Kind::Pollen => 0.5,
            };
            commands.spawn((
                Name::new(match kind {
                    Kind::Sand => "Blowing sand",
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
    commands.insert_resource(MapFxState);
}

fn visual(kind: Kind, time: f32, phase: f32) -> (Vec3, Vec3, f32) {
    let wave = time + phase;
    match kind {
        Kind::Sand => {
            let cycle = (time * 0.19 + phase * 0.16).fract();
            let smooth = |t: f32| {
                let t = t.clamp(0., 1.);
                t * t * (3. - 2. * t)
            };
            let fade = smooth(cycle / 0.23) * smooth((1. - cycle) / 0.35);
            (
                Vec3::new((cycle - 0.5) * 13., 0.35 + cycle * 0.55, (cycle - 0.5) * 4.),
                Vec3::new(12.5 + cycle * 2., 4.3 + cycle, 1.),
                0.18 * fade,
            )
        }
        Kind::Steam => {
            let cycle = (wave * 0.17).fract();
            (
                Vec3::new(
                    (wave * 0.5).sin() * 0.8,
                    0.9 + cycle * 2.3,
                    (wave * 0.3).cos() * 0.5,
                ),
                Vec3::new(2.8 + cycle * 1.2, 4.5 + cycle * 2., 1.),
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
    sand: Option<Res<SandPlumeMaterials>>,
    mut sprites: Query<
        (
            &MapFxSprite,
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<ChaseCamera>,
    >,
) {
    let Ok(camera) = camera.single() else { return };
    // Fixed-loop race time gives the same air pose across variable render rates.
    let elapsed = race.elapsed.max(0.);
    for (sprite, mut transform, mut visibility, mut material) in &mut sprites {
        let range = match sprite.kind {
            Kind::Sand => 125.,
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
        if let Some(sand) = sand.as_ref().filter(|_| sprite.kind == Kind::Sand) {
            let level =
                ((alpha / 0.18).clamp(0., 1.) * (SAND_ALPHA_LEVELS - 1) as f32).round() as usize;
            transform.scale = size;
            material.set_if_neq(MeshMaterial3d(sand.0[level].clone()));
            visibility.set_if_neq(if level == 0 {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            });
            continue;
        }
        let peak = match sprite.kind {
            Kind::Sand => 0.18,
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
    fn sand_plume_fades_at_fixed_footprint_while_crossing_the_verge() {
        let mut last_width = 0.;
        let mut last_x = f32::NEG_INFINITY;
        for cycle in 0..100 {
            let (position, size, alpha) = visual(Kind::Sand, cycle as f32 * 0.01 / 0.19, 0.);
            assert!(size.x >= last_width && size.x >= 12.5);
            assert!(position.x >= last_x && position.y < 1.);
            if cycle == 0 || cycle == 99 {
                assert!(alpha < 0.002);
            } else if cycle == 50 {
                assert!(alpha > 0.17);
            }
            last_width = size.x;
            last_x = position.x;
        }
    }

    #[test]
    fn rear_wheel_spray_respects_the_surface_and_ground_contact() {
        let mut bike = crate::vehicle::BikeState {
            slip: 0.8,
            grounded: [true, false],
            ..default()
        };
        assert!(wheel_spray(Surface::Sand, 16., 1., Some(&bike)) > 0.5);
        assert!(wheel_spray(Surface::Dirt, -16., 1., Some(&bike)) > 0.5);
        assert_eq!(wheel_spray(Surface::Asphalt, 30., 1., Some(&bike)), 0.);
        bike.grounded = [false, true];
        assert_eq!(wheel_spray(Surface::Sand, 30., 1., Some(&bike)), 0.);
    }

    #[test]
    fn fixed_budgets_and_temporal_envelopes() {
        assert_eq!(budget(Kind::Sand, 2), 48);
        assert_eq!(budget(Kind::Steam, 0), 0);
        assert_eq!(budget(Kind::Pollen, 2), 16);
        for kind in [Kind::Sand, Kind::Steam, Kind::Pollen] {
            for tick in 0..1000 {
                let (offset, scale, alpha) = visual(kind, tick as f32 / 60., 1.4);
                assert!(offset.is_finite() && scale.is_finite());
                assert!((0. ..=0.2).contains(&alpha));
            }
        }
        assert_eq!(soft_sprite().data.unwrap().len(), 64 * 64 * 4);
        assert_eq!(dust_sprite().data.unwrap().len(), 64 * 64 * 4);
        assert_eq!(steam_sprite().data.unwrap().len(), 64 * 64 * 4);
    }
}
