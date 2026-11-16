//! Pooled rear-tire marks and smoke driven by braking and wheel contact.
use super::soft_sprite;
use crate::{
    game::{ChaseCamera, Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    track::Track,
    vehicle::BikeState,
};
use bevy::{light::NotShadowCaster, prelude::*};
use rr_core::surface::Surface;

const MARKS: usize = 192;
const PUFFS: usize = 16;
const FADES: usize = 8;
const PUFF_LIFE: f32 = 0.72;
const MARK_SPACING: f32 = 0.65;

#[derive(Component)]
pub(crate) struct SkidMark;

#[derive(Component)]
pub(crate) struct BrakePuff {
    born: f32,
    origin: Vec3,
    drift: Vec3,
    density: f32,
}

#[derive(Resource)]
pub(crate) struct BrakeFx {
    marks: Vec<Entity>,
    puffs: Vec<Entity>,
    mark: usize,
    puff: usize,
    last: Option<Vec3>,
    next_puff: f32,
    smoke: [Handle<StandardMaterial>; FADES],
}

pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    quality: usize,
) {
    let mesh = meshes.add(Rectangle::new(1., 1.));
    let rubber = materials.add(StandardMaterial {
        base_color: Color::srgba(0.046, 0.043, 0.041, 0.68),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let texture = images.add(soft_sprite());
    let smoke = std::array::from_fn(|i| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(0.73, 0.74, 0.75, 0.31 * i as f32 / (FADES - 1) as f32),
            base_color_texture: Some(texture.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    });
    let marks = (0..MARKS * quality / 2)
        .map(|_| {
            commands
                .spawn((
                    SkidMark,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(rubber.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                    NotShadowCaster,
                ))
                .id()
        })
        .collect();
    let puffs = (0..PUFFS * quality / 2)
        .map(|_| {
            commands
                .spawn((
                    BrakePuff {
                        born: f32::NEG_INFINITY,
                        origin: Vec3::ZERO,
                        drift: Vec3::ZERO,
                        density: 0.,
                    },
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(smoke[0].clone()),
                    Transform::default(),
                    Visibility::Hidden,
                    NotShadowCaster,
                ))
                .id()
        })
        .collect();
    commands.insert_resource(BrakeFx {
        marks,
        puffs,
        mark: 0,
        puff: 0,
        last: None,
        next_puff: 0.,
        smoke,
    });
}

fn intensity(brake: f32, forward_speed: f32, slip: f32) -> f32 {
    if brake < 0.85 || forward_speed < 12. {
        return 0.;
    }
    // Fast, full service braking can leave a visible trace even when the
    // arcade tire model has enough grip to avoid saturating its force budget.
    let hard_stop = ((forward_speed - 16.) / 24.).clamp(0., 1.) * ((brake - 0.85) / 0.15);
    (hard_stop.max(slip * 1.8)).clamp(0., 1.)
}

fn segment(start: Vec3, end: Vec3, normal: Vec3) -> Option<Transform> {
    let delta = end - start;
    let length = delta.length();
    if !(MARK_SPACING..=3.).contains(&length) {
        return None;
    }
    let along = delta / length;
    let side = along.cross(normal).normalize_or_zero();
    if side == Vec3::ZERO {
        return None;
    }
    let normal = side.cross(along);
    let rotation = Quat::from_mat3(&Mat3::from_cols(side, along, normal));
    Some(
        Transform::from_translation(start.midpoint(end) + normal * 0.028)
            .with_rotation(rotation)
            .with_scale(Vec3::new(0.19, length + 0.10, 1.)),
    )
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn update(
    mut fx: ResMut<BrakeFx>,
    race: Res<Race>,
    track: Res<Track>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Recovery, &Transform, Option<&BikeState>)>,
    camera: Query<&Transform, With<ChaseCamera>>,
    mut marks: Query<
        (&mut Transform, &mut Visibility),
        (
            With<SkidMark>,
            Without<BrakePuff>,
            Without<Rider>,
            Without<ChaseCamera>,
        ),
    >,
    mut puffs: Query<
        (
            &mut BrakePuff,
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        (Without<SkidMark>, Without<Rider>, Without<ChaseCamera>),
    >,
) {
    if matches!(race.phase, Phase::Paused | Phase::Finished) {
        fx.last = None;
        return;
    }
    if race.phase != Phase::Racing {
        if fx.mark != 0 || fx.puff != 0 {
            for (_, mut visible) in &mut marks {
                visible.set_if_neq(Visibility::Hidden);
            }
            for (mut puff, _, mut visible, _) in &mut puffs {
                puff.born = f32::NEG_INFINITY;
                visible.set_if_neq(Visibility::Hidden);
            }
        }
        fx.mark = 0;
        fx.puff = 0;
        fx.last = None;
        fx.next_puff = 0.;
        return;
    }
    let local = online.map_or(0, |online| online.local_slot);
    let contact = riders
        .iter()
        .find(|(rider, _, _, _)| rider.id == local)
        .and_then(|(rider, recovery, pose, bike)| {
            if recovery.stage != RecoveryStage::Riding
                || rider.wasted.is_some()
                || track.surface_route(rider.route, rider.progress, rider.lane) != Surface::Asphalt
            {
                return None;
            }
            let brake = bike
                .map_or(rider.drive_intent, |bike| bike.intent)
                .brake_strength();
            let forward_speed = rider.motion_velocity.dot(pose.rotation * Vec3::Z);
            let strength = intensity(brake, forward_speed, bike.map_or(0., |bike| bike.slip));
            if strength < 0.12 {
                return None;
            }
            let (point, normal) = if let Some(bike) = bike {
                bike.rear_contact?
            } else {
                if rider.hop_height > 0.05 {
                    return None;
                }
                let rear = pose.transform_point(Vec3::Z * -0.78);
                let (road, rotation) =
                    track.sample_route(rider.route, rider.progress - 0.78, rider.lane);
                (rear.with_y(road.y), rotation * Vec3::Y)
            };
            Some((point, normal, strength, rider.motion_velocity))
        });
    let now = race.elapsed;
    if let Some((point, normal, strength, velocity)) = contact {
        let previous = *fx.last.get_or_insert(point);
        if previous.distance_squared(point) >= MARK_SPACING * MARK_SPACING {
            if let Some(pose) = segment(previous, point, normal)
                && let Ok((mut transform, mut visible)) =
                    marks.get_mut(fx.marks[fx.mark % fx.marks.len()])
            {
                *transform = pose;
                visible.set_if_neq(Visibility::Inherited);
                fx.mark += 1;
            }
            fx.last = Some(point);
        }
        if now >= fx.next_puff
            && let Ok((mut puff, _, _, _)) = puffs.get_mut(fx.puffs[fx.puff % fx.puffs.len()])
        {
            puff.born = now;
            puff.origin = point + normal * 0.14;
            puff.drift = velocity * 0.09 + normal * 0.84;
            puff.density = strength;
            fx.puff += 1;
            fx.next_puff = now + 0.095;
        }
    } else {
        fx.last = None;
    }

    let camera = camera
        .single()
        .map_or(Quat::IDENTITY, |camera| camera.rotation);
    for (puff, mut pose, mut visible, mut material) in &mut puffs {
        let age = now - puff.born;
        if !(0. ..PUFF_LIFE).contains(&age) {
            visible.set_if_neq(Visibility::Hidden);
            continue;
        }
        let t = age / PUFF_LIFE;
        let fade = (1. - t) * (t * 12.).min(1.);
        let level = (fade * (0.45 + 0.55 * puff.density) * (FADES - 1) as f32).round() as usize;
        pose.translation = puff.origin + puff.drift * age;
        pose.rotation = camera;
        pose.scale = Vec3::splat(0.30 + 1.4 * t);
        material.set_if_neq(MeshMaterial3d(fx.smoke[level].clone()));
        visible.set_if_neq(if level == 0 {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_drives_pooled_effects_and_restart_clears_them() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        world.insert_resource(Track::hills().unwrap());
        world.insert_resource(Race {
            phase: Phase::Racing,
            elapsed: 1.,
            ..default()
        });
        world.init_resource::<Assets<Mesh>>();
        world.init_resource::<Assets<StandardMaterial>>();
        world.init_resource::<Assets<Image>>();
        world
            .run_system_once(
                |mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>,
                 mut images: ResMut<Assets<Image>>| {
                    spawn(&mut commands, &mut meshes, &mut materials, &mut images, 2);
                },
            )
            .unwrap();
        let mut rider = Rider::new(0);
        rider.progress = 10480.;
        rider.motion_velocity = Vec3::Z * 38.;
        let entity = world
            .spawn((
                rider,
                Transform::default(),
                BikeState {
                    rear_contact: Some((Vec3::ZERO, Vec3::Y)),
                    intent: rr_core::VehicleIntent {
                        brake: true,
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();
        world.run_system_once(update).unwrap();
        world.resource_mut::<Race>().elapsed = 1.12;
        world.get_mut::<BikeState>(entity).unwrap().rear_contact = Some((Vec3::Z, Vec3::Y));
        world.run_system_once(update).unwrap();
        let mut marks = world.query_filtered::<&Visibility, With<SkidMark>>();
        let mut smoke = world.query_filtered::<&Visibility, With<BrakePuff>>();
        assert_eq!(
            marks
                .iter(&world)
                .filter(|v| **v == Visibility::Inherited)
                .count(),
            1
        );
        assert!(smoke.iter(&world).any(|v| *v == Visibility::Inherited));
        world.get_mut::<BikeState>(entity).unwrap().rear_contact = None;
        world.resource_mut::<Race>().elapsed = 2.;
        world.run_system_once(update).unwrap();
        assert!(smoke.iter(&world).all(|v| *v == Visibility::Hidden));
        assert_eq!(
            marks
                .iter(&world)
                .filter(|v| **v == Visibility::Inherited)
                .count(),
            1
        );
        world.resource_mut::<Race>().phase = Phase::Countdown;
        world.run_system_once(update).unwrap();
        assert!(marks.iter(&world).all(|v| *v == Visibility::Hidden));
    }

    #[test]
    fn skid_requires_hard_braking_at_speed_and_joins_only_local_contacts() {
        assert_eq!(intensity(0.4, 40., 0.7), 0.);
        assert_eq!(intensity(1., -2.5, 1.), 0.);
        assert!(intensity(1., 33., 0.) > 0.6);
        assert!(intensity(0.9, 20., 0.4) > 0.6);
        assert!(segment(Vec3::ZERO, Vec3::Z * 9., Vec3::Y).is_none());
        let mark = segment(Vec3::ZERO, Vec3::Z, Vec3::Y).unwrap();
        assert!((mark.rotation * Vec3::Z).dot(Vec3::Y) > 0.99);
        assert!(mark.translation.y > 0. && mark.translation.y < 0.05);
    }
}
