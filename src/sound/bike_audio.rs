use std::collections::BTreeMap;

use bevy::{
    audio::{AudioSinkPlayback, SpatialAudioSink, SpatialListener, SpatialScale, Volume},
    prelude::*,
};
use bevy_rapier3d::prelude::Velocity;

use crate::{
    game::{Phase, Race, Rider},
    online::{NetState, replication::NetworkPose},
    recovery::{Recovery, RecoveryStage},
    settings::Preferences,
    vehicle::BikeState,
};

use super::{
    Muted,
    engine::{EngineControl, EngineSource},
    mix_gain,
};

const AUDIBLE_RANGE: f32 = 105.;
const RETAIN_RANGE: f32 = 120.;
const IDLE_RPM: f32 = 1200.;

#[derive(Component)]
pub(crate) struct BikeEmitter(pub(crate) Entity);

struct Voice {
    entity: Entity,
    control: EngineControl,
    gain: f32,
    pitch: f32,
    load: f32,
    speed: f32,
}

#[derive(Default)]
pub(crate) struct BikeAudio {
    voices: BTreeMap<Entity, Voice>,
    listener: Option<Vec3>,
    listener_velocity: Vec3,
}

fn spatial_gain(distance: f32) -> f32 {
    let tail = ((AUDIBLE_RANGE - distance) / 30.).clamp(0., 1.);
    tail * tail / (1. + (distance / 13.).powi(2))
}

fn doppler(offset: Vec3, source_velocity: Vec3, listener_velocity: Vec3) -> f32 {
    let direction = offset.normalize_or_zero();
    let radial = |velocity: Vec3| velocity.dot(direction).clamp(-105., 105.);
    ((343. + radial(listener_velocity)) / (343. + radial(source_velocity))).clamp(0.65, 1.5)
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn update_bikes(
    mut commands: Commands,
    race: Res<Race>,
    time: Res<Time<Real>>,
    muted: Res<Muted>,
    preferences: Option<Res<Preferences>>,
    online: Option<Res<NetState>>,
    controls: Res<crate::controls::Controls>,
    mut assets: ResMut<Assets<EngineSource>>,
    listener: Query<&Transform, With<SpatialListener>>,
    riders: Query<(
        Entity,
        &Rider,
        &Recovery,
        &Transform,
        Option<&BikeState>,
        Option<&NetworkPose>,
    )>,
    wrecks: Query<
        (&Transform, Option<&Velocity>, Option<&NetworkPose>),
        (Without<Rider>, Without<BikeEmitter>),
    >,
    mut emitters: Query<
        (&mut Transform, Option<&mut SpatialAudioSink>),
        (With<BikeEmitter>, Without<Rider>, Without<SpatialListener>),
    >,
    mut state: Local<BikeAudio>,
) {
    let active = matches!(race.phase, Phase::Countdown | Phase::Racing | Phase::Paused);
    let Some(listener) = listener.single().ok().filter(|_| active) else {
        for (_, voice) in std::mem::take(&mut state.voices) {
            commands.entity(voice.entity).try_despawn();
        }
        state.listener = None;
        state.listener_velocity = Vec3::ZERO;
        return;
    };
    let dt = time.delta_secs().max(0.001);
    let smoothing = 1. - (-dt * 15.).exp();
    let paused = race.phase == Phase::Paused;
    let previous = state.listener.replace(listener.translation);
    let camera_velocity = previous
        .map(|last| (listener.translation - last) / dt)
        .filter(|v| !paused && v.length_squared() < 120. * 120.)
        .unwrap_or_default();
    state.listener_velocity = state.listener_velocity.lerp(camera_velocity, smoothing);
    let listener_velocity = state.listener_velocity;
    let bus = preferences
        .as_ref()
        .map_or(1., |p| p.file.master_volume * p.file.mix.engine);
    let local_slot = online.as_ref().map_or(0, |n| n.local_slot);
    let legacy = std::env::var_os("ROADDASH_ENGINE_LEGACY").is_some();

    state.voices.retain(|rider, voice| {
        if riders.get(*rider).is_ok() && emitters.get(voice.entity).is_ok() {
            true
        } else {
            commands.entity(voice.entity).try_despawn();
            false
        }
    });
    for (entity, rider, recovery, pose, bike, replicated) in &riders {
        let local = rider.id == local_slot;
        let mounted = recovery.stage == RecoveryStage::Riding;
        let (pose, velocity) = if mounted {
            replicated.map_or((*pose, rider.motion_velocity), |pose| {
                (pose.shown(), pose.presentation_velocity())
            })
        } else if let Some((wreck, velocity, replicated)) =
            recovery.wreck.and_then(|wreck| wrecks.get(wreck).ok())
        {
            replicated.map_or(
                (*wreck, velocity.map_or(Vec3::ZERO, |v| v.linear)),
                |pose| (pose.shown(), pose.presentation_velocity()),
            )
        } else {
            // Replicated recovery can briefly arrive before its wreck entity.
            (*pose, Vec3::ZERO)
        };
        let position = pose.translation + pose.rotation * Vec3::Y * 0.45;
        let offset = position - listener.translation;
        let distance = offset.length();
        let existing = state.voices.contains_key(&entity);
        let limit = if existing {
            RETAIN_RANGE
        } else {
            AUDIBLE_RANGE
        };
        if distance > limit || (legacy && local && mounted) {
            if let Some(voice) = state.voices.remove(&entity) {
                commands.entity(voice.entity).try_despawn();
            }
            continue;
        }
        let (rpm, load, gear) = if mounted && race.phase != Phase::Countdown {
            let load = if local && local_slot != 0 {
                controls.buffer.held.throttle_strength()
            } else if replicated.is_some() {
                // Snapshots carry speed and RPM, so estimate remote load from
                // acceleration and sustain a small amount at cruising speed.
                let previous = state.voices.get(&entity);
                let delta = previous.map_or(0., |voice| rider.speed - voice.speed);
                let held = previous.map_or(0., |voice| voice.load * (-dt * 4.).exp());
                (held + delta * 0.25).clamp((rider.speed / 180.).clamp(0., 0.3), 1.)
            } else {
                rider.drive_intent.throttle_strength()
            };
            let rpm = bike
                .filter(|bike| bike.rpm >= IDLE_RPM)
                .map_or(IDLE_RPM + rider.speed.max(0.) * 145., |bike| bike.rpm);
            (rpm, load, bike.map_or(0, |bike| bike.gear))
        } else {
            (IDLE_RPM, 0., 0)
        };
        let target_gain = if local && mounted {
            0.55
        } else {
            let peak = if local { 0.55 } else { 0.34 };
            peak * spatial_gain(distance)
        };
        let target_pitch = if local && mounted {
            1.
        } else {
            doppler(offset, velocity, listener_velocity)
        };
        let voice = state.voices.entry(entity).or_insert_with(|| {
            let control = EngineControl::default();
            let source = assets.add(EngineSource(control.clone()));
            let emitter = commands
                .spawn((
                    BikeEmitter(entity),
                    Transform::from_translation(position),
                    AudioPlayer(source),
                    PlaybackSettings::ONCE
                        .with_spatial(true)
                        .with_spatial_scale(SpatialScale(Vec3::splat(0.01)))
                        .with_volume(Volume::SILENT),
                ))
                .id();
            Voice {
                entity: emitter,
                control,
                gain: 0.,
                pitch: target_pitch,
                load,
                speed: rider.speed,
            }
        });
        voice.control.update(rpm, load, gear);
        voice.load = load;
        voice.speed = rider.speed;
        voice.gain += (target_gain - voice.gain) * smoothing;
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
    fn moving_sources_shift_pitch_and_fade_with_distance() {
        let ahead = Vec3::Z * 20.;
        assert!(doppler(ahead, -Vec3::Z * 40., Vec3::ZERO) > 1.);
        assert!(doppler(ahead, Vec3::Z * 40., Vec3::ZERO) < 1.);
        assert_eq!(doppler(ahead, Vec3::Z * 40., Vec3::Z * 40.), 1.);
        assert!(spatial_gain(15.) > spatial_gain(50.));
        assert_eq!(spatial_gain(AUDIBLE_RANGE), 0.);
    }
}
