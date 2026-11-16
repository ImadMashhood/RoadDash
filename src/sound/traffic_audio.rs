use std::collections::BTreeMap;

use bevy::{
    audio::{AudioSinkPlayback, SpatialAudioSink, SpatialListener, SpatialScale, Volume},
    prelude::*,
};
use bevy_rapier3d::prelude::{ColliderDisabled, Velocity};

use crate::{
    environment::{CrossTraffic, Patrol},
    game::{Phase, Race, Traffic},
    heavy_traffic::{HeavyTraffic, Kind},
    online::replication::NetworkPose,
    settings::Preferences,
};

use super::{
    Muted,
    bike_audio::doppler,
    engine::{EngineControl, EngineSource},
    engine_signal::TrafficEngineKind,
    mix_gain,
};

const MAX_VOICES: usize = 6;
const RANGE: f32 = 72.;
const RETAIN_DISTANCE: f32 = 8.;

#[derive(Component)]
pub(crate) struct PickupEngine;

#[derive(Component)]
pub(crate) struct TrafficEmitter;

struct Voice {
    entity: Entity,
    control: EngineControl,
    speed: f32,
    feed: f32,
    gain: f32,
    pitch: f32,
}

#[derive(Default)]
pub(crate) struct TrafficAudio {
    voices: BTreeMap<Entity, Voice>,
    listener: Option<Vec3>,
    listener_velocity: Vec3,
}

struct Candidate {
    vehicle: Entity,
    kind: TrafficEngineKind,
    offset: Vec3,
    distance: f32,
    priority: f32,
    speed: f32,
    velocity: Vec3,
}

fn operating_point(kind: TrafficEngineKind, speed: f32, acceleration: f32) -> (f32, f32) {
    let (idle, cruise_rpm, cruise_speed) = match kind {
        TrafficEngineKind::Car => (760., 1930., 19.),
        TrafficEngineKind::Truck => (690., 1710., 18.),
        TrafficEngineKind::Tractor => (760., 1770., 8.),
        TrafficEngineKind::Semi => (650., 1420., 21.),
    };
    let fraction = (speed / cruise_speed).clamp(0., 1.2);
    let rpm = idle + (cruise_rpm - idle) * fraction;
    let load = (fraction * 0.42 + acceleration * 0.15).clamp(0., 0.85);
    (rpm, load)
}

fn distance_gain(kind: TrafficEngineKind, distance: f32) -> f32 {
    let gain = match kind {
        TrafficEngineKind::Car => 0.15,
        TrafficEngineKind::Truck => 0.18,
        TrafficEngineKind::Tractor => 0.20,
        TrafficEngineKind::Semi => 0.21,
    };
    let tail = ((RANGE - distance) / 24.).clamp(0., 1.);
    gain * tail * tail / (1. + (distance / 14.).powi(2))
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn update_traffic(
    mut commands: Commands,
    race: Res<Race>,
    time: Res<Time<Real>>,
    muted: Res<Muted>,
    preferences: Option<Res<Preferences>>,
    mut assets: ResMut<Assets<EngineSource>>,
    listener: Query<&Transform, With<SpatialListener>>,
    vehicles: Query<
        (
            Entity,
            &Transform,
            Option<&Traffic>,
            Option<&Patrol>,
            Option<&HeavyTraffic>,
            Option<&NetworkPose>,
            Option<&Velocity>,
            Has<PickupEngine>,
        ),
        (
            Or<(With<Traffic>, (With<CrossTraffic>, With<Patrol>))>,
            Without<ColliderDisabled>,
            Without<TrafficEmitter>,
        ),
    >,
    mut emitters: Query<
        (&mut Transform, Option<&mut SpatialAudioSink>),
        (With<TrafficEmitter>, Without<SpatialListener>),
    >,
    mut state: Local<TrafficAudio>,
) {
    let active = matches!(race.phase, Phase::Countdown | Phase::Racing | Phase::Paused);
    let Some(listener) = listener.single().ok().filter(|_| active) else {
        for voice in std::mem::take(&mut state.voices).into_values() {
            commands.entity(voice.entity).try_despawn();
        }
        state.listener = None;
        state.listener_velocity = Vec3::ZERO;
        return;
    };
    let dt = time.delta_secs().max(0.001);
    let smoothing = 1. - (-dt * 12.).exp();
    let paused = race.phase == Phase::Paused;
    let racing = race.phase == Phase::Racing;
    let previous = state.listener.replace(listener.translation);
    let camera_velocity = previous
        .map(|last| (listener.translation - last) / dt)
        .filter(|v| racing && v.length_squared() < 120. * 120.)
        .unwrap_or_default();
    state.listener_velocity = state.listener_velocity.lerp(camera_velocity, smoothing);
    let listener_velocity = state.listener_velocity;
    let bus = preferences
        .as_ref()
        .map_or(1., |p| p.file.master_volume * p.file.mix.engine);

    let mut candidates = Vec::new();
    for (vehicle, pose, lane, crossing, heavy, replicated, velocity, pickup) in &vehicles {
        let kind = match heavy.map(|heavy| heavy.kind) {
            Some(Kind::Tractor) => TrafficEngineKind::Tractor,
            Some(Kind::Semi) => TrafficEngineKind::Semi,
            None if pickup => TrafficEngineKind::Truck,
            None => TrafficEngineKind::Car,
        };
        let pose = replicated.map_or(*pose, NetworkPose::shown);
        let offset = pose.translation + Vec3::Y * 0.7 - listener.translation;
        let distance = offset.length();
        let existing = state.voices.get(&vehicle);
        let priority = if existing.is_some() {
            distance - RETAIN_DISTANCE
        } else {
            distance
        };
        if priority >= RANGE {
            continue;
        }
        let speed = if paused {
            existing.map_or(0., |voice| voice.speed)
        } else if racing {
            lane.map_or_else(
                || {
                    replicated.map_or_else(
                        || {
                            velocity.map_or_else(
                                || crossing.map_or(0., |path| path.speed),
                                |velocity| velocity.linear.xz().length(),
                            )
                        },
                        |pose| pose.presentation_velocity().xz().length(),
                    )
                },
                |lane| lane.speed.abs(),
            )
        } else {
            0.
        };
        let velocity = replicated.map_or_else(
            || pose.rotation * Vec3::Z * speed,
            NetworkPose::presentation_velocity,
        );
        candidates.push(Candidate {
            vehicle,
            kind,
            offset,
            distance,
            priority,
            speed,
            velocity,
        });
    }
    candidates.sort_unstable_by(|a, b| {
        a.priority
            .total_cmp(&b.priority)
            .then(a.vehicle.cmp(&b.vehicle))
    });
    candidates.truncate(MAX_VOICES);
    state.voices.retain(|vehicle, voice| {
        if candidates.iter().any(|c| c.vehicle == *vehicle) && emitters.get(voice.entity).is_ok() {
            true
        } else {
            commands.entity(voice.entity).try_despawn();
            false
        }
    });
    for candidate in candidates {
        let Candidate {
            vehicle,
            kind,
            offset,
            distance,
            speed,
            velocity,
            ..
        } = candidate;
        let position = listener.translation + offset;
        let target_pitch = doppler(offset, velocity, listener_velocity).clamp(0.85, 1.17);
        let voice = state.voices.entry(vehicle).or_insert_with(|| {
            let control = EngineControl::default();
            let source = assets.add(EngineSource(control.clone(), Some(kind)));
            let entity = commands
                .spawn((
                    TrafficEmitter,
                    Transform::from_translation(position),
                    AudioPlayer(source),
                    PlaybackSettings::ONCE
                        .with_spatial(true)
                        .with_spatial_scale(SpatialScale(Vec3::splat(0.01)))
                        .with_volume(Volume::SILENT),
                ))
                .id();
            Voice {
                entity,
                control,
                speed,
                feed: 0.,
                gain: 0.,
                pitch: target_pitch,
            }
        });
        let acceleration = ((speed - voice.speed) / dt).clamp(-6., 3.);
        voice.speed = speed;
        voice.feed += (acceleration - voice.feed) * (1. - (-dt * 3.).exp());
        let (rpm, load) = operating_point(kind, speed, voice.feed);
        voice.control.update(rpm, load, 0);
        voice.gain += (distance_gain(kind, distance) - voice.gain) * smoothing;
        voice.pitch += (target_pitch - voice.pitch) * smoothing;
        if let Ok((mut transform, sink)) = emitters.get_mut(voice.entity) {
            transform.translation = position;
            if let Some(mut sink) = sink {
                sink.set_volume(Volume::Linear(mix_gain(voice.gain, bus, muted.0)));
                sink.set_speed(voice.pitch);
                if paused {
                    sink.pause();
                } else {
                    sink.play();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traffic_emitter_budget_and_lifecycle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Race>()
            .init_resource::<Muted>()
            .init_resource::<Assets<EngineSource>>()
            .add_systems(Update, update_traffic);
        let world = app.world_mut();
        world.resource_mut::<Race>().phase = Phase::Racing;
        world.spawn((Transform::default(), SpatialListener::new(0.35)));
        for i in 1..=8 {
            world.spawn((
                Transform::from_xyz(i as f32 * 5., 0., 0.),
                Traffic {
                    route: 0,
                    progress: 0.,
                    lane: 0.,
                    speed: 10.,
                },
            ));
        }
        world.spawn((Transform::default(), CrossTraffic));
        let mut emitters = world.query_filtered::<&Transform, With<TrafficEmitter>>();

        app.update();
        let mut positions = emitters
            .iter(app.world())
            .map(|pose| pose.translation.x)
            .collect::<Vec<_>>();
        positions.sort_unstable_by(f32::total_cmp);
        assert_eq!(positions, [5., 10., 15., 20., 25., 30.]);

        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        app.update();
        assert_eq!(emitters.iter(app.world()).count(), 0);
    }

    #[test]
    fn operating_point_respects_low_rpm_and_releases_load_under_braking() {
        for (kind, speed) in [
            (TrafficEngineKind::Car, 19.),
            (TrafficEngineKind::Truck, 18.),
            (TrafficEngineKind::Tractor, 8.),
            (TrafficEngineKind::Semi, 21.),
        ] {
            let (idle, no_load) = operating_point(kind, 0., 0.);
            let (cruise, load) = operating_point(kind, speed, 0.);
            let (_, braking) = operating_point(kind, speed, -6.);
            assert!(idle < 1000. && no_load == 0.);
            assert!((1400. ..2000.).contains(&cruise));
            assert!(load > braking);
            assert!(distance_gain(kind, 10.) > distance_gain(kind, 30.));
            assert_eq!(distance_gain(kind, RANGE), 0.);
        }
    }
}
