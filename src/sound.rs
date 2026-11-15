//! Recorded gameplay audio, loops and world-space cue playback.
mod bike_audio;
mod engine;
mod engine_signal;
mod traffic_audio;
pub(crate) use bike_audio::{BikeEmitter, update_bikes};
pub(crate) use traffic_audio::{PickupEngine, update_traffic};

use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
};
use bevy::{audio::Volume, prelude::*};
use std::sync::Arc;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Cue {
    Crash,
    Break,
    Punch,
    Kick,
    Honk,
    Land,
    Footstep,
    Siren,
    Whiff,
    BikeStrike,
    PassBy,
    AirBrake,
    GravelStep,
    Steam,
    Runover,
    RunoverBike,
    TownShot,
    TownPunch,
    RiverSplash,
}
#[derive(Message, Clone, Copy)]
pub struct AudioCue {
    pub kind: Cue,
    pub position: Vec3,
}
impl AudioCue {
    pub fn new(kind: Cue, position: Vec3) -> Self {
        Self { kind, position }
    }
}
#[derive(Resource)]
pub struct Sounds {
    siren: Handle<AudioSource>,
    crash: Handle<AudioSource>,
    river_splash: Handle<AudioSource>,
    breaking: Handle<AudioSource>,
    punch: Handle<AudioSource>,
    kick: Handle<AudioSource>,
    honk: Handle<AudioSource>,
    land: Handle<AudioSource>,
    footstep: Handle<AudioSource>,
    beep: Handle<AudioSource>,
    finish: Handle<AudioSource>,
    whiff: Handle<AudioSource>,
    bike_strike: Handle<AudioSource>,
    runover: Handle<AudioSource>,
    runover_bike: Handle<AudioSource>,
    town_shot: Handle<AudioSource>,
    pass_by: Handle<AudioSource>,
    air_brake: Handle<AudioSource>,
    gravel_step: [Handle<AudioSource>; 2],
    steam: Handle<AudioSource>,
}
#[derive(Resource, Default)]
pub struct Muted(pub bool);
pub(crate) fn mix_gain(base: f32, master: f32, muted: bool) -> f32 {
    if muted { 0. } else { base * master }
}
#[derive(Component)]
pub struct CueGain(pub f32);
#[derive(Component, Clone, Copy)]
pub enum LoopSound {
    Engine(usize),
    Wind,
    Nitro,
    Brake,
    LooseSurface,
    Ambience(usize),
}

fn recording(bytes: &'static [u8]) -> AudioSource {
    AudioSource {
        bytes: Arc::from(bytes),
    }
}

// Recordings are fixed mono PCM at 24 kHz. Read the data chunk rather than
// assuming the encoder omitted metadata. Crossfade the tail into the head.
fn loop_recording(bytes: &'static [u8], offset: f32, length: f32) -> AudioSource {
    let mut cursor = 12;
    while cursor + 8 <= bytes.len() {
        let size = u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        let start = cursor + 8;
        if &bytes[cursor..cursor + 4] == b"data" {
            let begin = start + ((offset * 24000.) as usize) * 2;
            let end = (begin + ((length * 24000.) as usize) * 2).min(start + size);
            let pcm = &bytes[begin..end];
            let mut output = bytes[..start].to_vec();
            let fade = 240;
            let trimmed = pcm.len() - fade * 2;
            for i in 0..trimmed / 2 {
                let sample = |j| i16::from_le_bytes([pcm[j * 2], pcm[j * 2 + 1]]) as f32;
                let x = if i < fade {
                    let blend = i as f32 / fade as f32;
                    sample(trimmed / 2 + i) * (1. - blend) + sample(i) * blend
                } else {
                    sample(i)
                };
                output.extend_from_slice(&(x as i16).to_le_bytes());
            }
            output[cursor + 4..start].copy_from_slice(&(trimmed as u32).to_le_bytes());
            let riff = (output.len() - 8) as u32;
            output[4..8].copy_from_slice(&riff.to_le_bytes());
            return AudioSource {
                bytes: output.into(),
            };
        }
        cursor = start + size + (size % 2);
    }
    recording(bytes)
}

pub(crate) fn register_engine(app: &mut App) {
    use bevy::audio::AddAudioSource;
    app.add_audio_source::<engine::EngineSource>();
}

pub(crate) fn setup(
    mut commands: Commands,
    mut sources: ResMut<Assets<AudioSource>>,
    asset_server: Res<AssetServer>,
) {
    let rev = include_bytes!("../assets/audio/demo/rev.wav");
    if std::env::var_os("ROADDASH_ENGINE_LEGACY").is_some() {
        let idle = include_bytes!("../assets/audio/demo/idle.wav");
        for (index, clip) in [
            loop_recording(idle, 0.1, 2.7),
            loop_recording(rev, 0.7, 0.65),
            loop_recording(rev, 1.35, 0.65),
            loop_recording(rev, 2., 0.55),
        ]
        .into_iter()
        .enumerate()
        {
            commands.spawn((
                AudioPlayer::new(sources.add(clip)),
                PlaybackSettings::LOOP.with_volume(Volume::SILENT),
                LoopSound::Engine(index),
            ));
        }
    }
    for (source, marker) in [
        (
            loop_recording(include_bytes!("../assets/audio/recorded/wind.wav"), 0., 7.),
            LoopSound::Wind,
        ),
        (loop_recording(rev, 1.35, 0.65), LoopSound::Nitro),
        (
            recording(include_bytes!("../assets/audio/demo/skid.wav")),
            LoopSound::Brake,
        ),
        (
            loop_recording(include_bytes!("../assets/audio/demo/gravel.wav"), 0.2, 1.8),
            LoopSound::LooseSurface,
        ),
        (
            loop_recording(include_bytes!("../assets/audio/recorded/city.wav"), 0., 12.),
            LoopSound::Ambience(0),
        ),
        (
            loop_recording(include_bytes!("../assets/audio/recorded/farm.wav"), 0., 12.),
            LoopSound::Ambience(1),
        ),
        (
            loop_recording(
                include_bytes!("../assets/audio/recorded/desert.wav"),
                0.,
                12.,
            ),
            LoopSound::Ambience(2),
        ),
    ] {
        commands.spawn((
            AudioPlayer::new(sources.add(source)),
            PlaybackSettings::LOOP.with_volume(Volume::SILENT),
            marker,
        ));
    }
    commands.insert_resource(Sounds {
        siren: asset_server.load("audio/recorded/siren.wav"),
        crash: asset_server.load("audio/impacts/crash.wav"),
        river_splash: asset_server.load("audio/impacts/river-splash.wav"),
        breaking: asset_server.load("audio/impacts/weapon.wav"),
        punch: asset_server.load("audio/impacts/punch.wav"),
        kick: asset_server.load("audio/impacts/kick.wav"),
        honk: asset_server.load("audio/recorded/horn.wav"),
        land: asset_server.load("audio/impacts/land.wav"),
        footstep: asset_server.load("audio/impacts/footstep.wav"),
        beep: asset_server.load("audio/recorded/countdown.wav"),
        finish: asset_server.load("audio/recorded/finish.wav"),
        whiff: asset_server.load("audio/impacts/whiff.wav"),
        bike_strike: asset_server.load("audio/impacts/weapon.wav"),
        runover: asset_server.load("audio/impacts/runover.wav"),
        runover_bike: asset_server.load("audio/impacts/runover_bike.wav"),
        town_shot: asset_server.load("audio/impacts/town-shot.wav"),
        pass_by: sources.add(recording(include_bytes!("../assets/audio/demo/rev.wav"))),
        air_brake: sources.add(recording(include_bytes!(
            "../assets/audio/demo/air_brake.wav"
        ))),
        gravel_step: [0.1, 1.0].map(|offset| {
            sources.add(loop_recording(
                include_bytes!("../assets/audio/demo/gravel.wav"),
                offset,
                0.24,
            ))
        }),
        steam: asset_server.load("audio/recorded/steam.wav"),
    });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update(
    mut commands: Commands,
    controls: Res<crate::controls::Controls>,
    race: Res<Race>,
    sounds: Res<Sounds>,
    muted: Res<Muted>,
    preferences: Option<Res<crate::settings::Preferences>>,
    riders: Query<(&Rider, &Recovery, &Transform)>,
    bike_states: Query<(&Rider, &crate::vehicle::BikeState)>,
    mut cues: MessageReader<AudioCue>,
    mut sinks: Query<(&mut AudioSink, Option<&LoopSound>, Option<&CueGain>)>,
    mut spatial: Query<(&mut bevy::audio::SpatialAudioSink, &CueGain)>,
    mut last_count: Local<i32>,
    mut last_phase: Local<Option<Phase>>,
    online: Option<Res<crate::online::NetState>>,
    track: Res<crate::track::Track>,
    listener: Query<&Transform, With<bevy::audio::SpatialListener>>,
) {
    let Some((player, recovery, player_transform)) = riders
        .iter()
        .find(|(r, _, _)| r.id == online.as_ref().map_or(0, |n| n.local_slot))
    else {
        return;
    };
    let riding = race.phase == Phase::Racing && recovery.stage == RecoveryStage::Riding && !muted.0;
    let master = preferences.as_ref().map_or(1., |p| p.file.master_volume);
    let mix = preferences.as_ref().map(|p| &p.file.mix);
    let effects = master * mix.map_or(1., |m| m.effects);
    let bike = bike_states
        .iter()
        .find(|(r, _)| r.id == player.id)
        .map(|(_, bike)| bike);
    let rpm = bike.map_or(1500. + player.speed * 145., |b| b.rpm);
    let load = if online.as_ref().is_some_and(|state| state.local_slot != 0) {
        controls.buffer.held.throttle_strength()
    } else {
        player.drive_intent.throttle_strength()
    };
    let surface = track.surface_route(player.route, player.progress, player.lane);
    let loose = crate::map_fx::wheel_spray(surface, player.speed, load, bike);
    let centers = [1450., 3400., 6100., 9100.];
    let lower = centers
        .partition_point(|center| *center <= rpm)
        .saturating_sub(1)
        .min(2);
    let band = lower as f32
        + ((rpm - centers[lower]) / (centers[lower + 1] - centers[lower])).clamp(0., 1.);
    for (mut sink, loop_sound, gain) in &mut sinks {
        if race.phase == Phase::Paused {
            sink.pause();
        } else {
            sink.play();
        }
        let Some(loop_sound) = loop_sound else {
            if muted.0 {
                sink.set_volume(Volume::SILENT);
            } else if let Some(g) = gain {
                sink.set_volume(Volume::Linear(mix_gain(g.0, effects, muted.0)));
            }
            continue;
        };
        let boost = controls.buffer.held.nitro && controls.buffer.held.throttle;
        let volume = if let LoopSound::Ambience(zone) = loop_sound {
            let urban = if track.city_surface.is_some() {
                1.
            } else if track.metropolis.is_some() {
                ((player.progress - 8650.) / 850.).clamp(0., 1.)
            } else {
                0.
            };
            let hills = if track.is_hills() {
                ((player.progress - 2600.) / 600.).clamp(0., 1.)
            } else {
                0.
            };
            if race.phase == Phase::Racing {
                [1. - urban.max(hills), hills, urban][*zone] * 0.065
            } else {
                0.
            }
        } else if !riding {
            0.0
        } else {
            match loop_sound {
                LoopSound::Engine(i) => {
                    (1. - (band - *i as f32).abs()).max(0.) * (0.16 + 0.19 * load)
                }
                LoopSound::LooseSurface => loose * 0.24,
                LoopSound::Ambience(_) => 0.,
                LoopSound::Wind => (player.speed / 60.0).clamp(0.0, 1.0).powi(2) * 0.18,
                LoopSound::Nitro => {
                    if boost && player.nitro > 0.0 && player.speed > 2.0 {
                        0.20
                    } else {
                        0.0
                    }
                }
                LoopSound::Brake => {
                    if surface == rr_core::surface::Surface::Asphalt
                        && player.speed > 12.0
                        && controls.buffer.held.brake
                    {
                        0.18
                    } else {
                        0.0
                    }
                }
            }
        };
        let bus = mix.map_or(1., |m| match loop_sound {
            LoopSound::Engine(_) => m.engine,
            LoopSound::Wind | LoopSound::Ambience(_) => m.ambience,
            LoopSound::Nitro | LoopSound::Brake | LoopSound::LooseSurface => m.effects,
        });
        sink.set_volume(Volume::Linear(mix_gain(volume, master * bus, muted.0)));
        if let LoopSound::Engine(index) = loop_sound {
            sink.set_speed((rpm / centers[*index]).clamp(0.65, 1.55));
        }
    }
    for (mut sink, gain) in &mut spatial {
        sink.set_volume(Volume::Linear(mix_gain(gain.0, effects, muted.0)));
        if race.phase == Phase::Paused {
            sink.pause();
        } else {
            sink.play();
        }
    }
    let listener_position = listener
        .single()
        .map_or(player_transform.translation, |transform| {
            transform.translation
        });
    for cue in cues.read() {
        // Keep distant AI impacts quiet instead of filling the mix with eight riders.
        let distance = cue.position.distance(listener_position);
        let gain = (1.0 / (1.0 + distance * 0.09)).clamp(0.0, 1.0);
        let handle = match cue.kind {
            Cue::Siren => &sounds.siren,
            Cue::Crash => &sounds.crash,
            Cue::RiverSplash => &sounds.river_splash,
            Cue::Break => &sounds.breaking,
            Cue::Punch | Cue::TownPunch => &sounds.punch,
            Cue::Kick => &sounds.kick,
            Cue::Honk => &sounds.honk,
            Cue::Land => &sounds.land,
            Cue::Footstep => &sounds.footstep,
            Cue::Whiff => &sounds.whiff,
            Cue::BikeStrike => &sounds.bike_strike,
            Cue::PassBy => &sounds.pass_by,
            Cue::AirBrake => &sounds.air_brake,
            Cue::GravelStep => &sounds.gravel_step[(race.elapsed * 9.) as usize % 2],
            Cue::Steam => &sounds.steam,
            Cue::Runover => &sounds.runover,
            Cue::RunoverBike => &sounds.runover_bike,
            Cue::TownShot => &sounds.town_shot,
        };
        let (base_gain, scale) = match cue.kind {
            Cue::Crash | Cue::RunoverBike | Cue::RiverSplash => (0.95, 0.12),
            Cue::TownShot => (0.75, 0.035),
            Cue::Kick | Cue::Runover => (0.75, 0.12),
            Cue::Punch | Cue::TownPunch | Cue::BikeStrike | Cue::Break => (0.65, 0.12),
            _ => (0.55, 0.25),
        };
        let gain = gain * base_gain;
        if !muted.0 && race.phase == Phase::Racing && distance < 120. {
            commands.spawn((
                AudioPlayer::new(handle.clone()),
                PlaybackSettings::DESPAWN
                    .with_volume(Volume::Linear(mix_gain(gain, effects, muted.0)))
                    .with_spatial(true)
                    .with_spatial_scale(bevy::audio::SpatialScale(Vec3::splat(scale))),
                Transform::from_translation(cue.position),
                CueGain(gain),
            ));
        }
    }
    let count = if race.phase == Phase::Countdown {
        race.countdown.ceil() as i32
    } else {
        0
    };
    if count > 0 && count != *last_count && !muted.0 {
        commands.spawn((
            AudioPlayer::new(sounds.beep.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(mix_gain(0.35, effects, muted.0))),
            CueGain(0.35),
        ));
    }
    if race.phase == Phase::Finished
        && *last_phase != Some(Phase::Finished)
        && race.player_wasted.is_none()
        && !race.continued
        && !muted.0
    {
        commands.spawn((
            AudioPlayer::new(if race.arrested {
                sounds.siren.clone()
            } else {
                sounds.finish.clone()
            }),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(mix_gain(0.5, effects, muted.0))),
            CueGain(0.5),
        ));
    }
    *last_count = count;
    *last_phase = Some(race.phase);
}

#[derive(Default)]
pub(crate) struct RoadsideAudio {
    next: f32,
    last: f32,
    previous: std::collections::BTreeMap<Entity, f32>,
}

pub(crate) fn roadside(
    race: Res<Race>,
    track: Res<crate::track::Track>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Transform)>,
    trucks: Query<(
        Entity,
        &crate::game::Traffic,
        &Transform,
        &crate::heavy_traffic::HeavyTraffic,
    )>,
    mut state: Local<RoadsideAudio>,
    mut audio: MessageWriter<AudioCue>,
) {
    if race.phase != Phase::Racing || race.elapsed < state.last {
        *state = default();
        return;
    }
    state.last = race.elapsed;
    let slot = online.as_ref().map_or(0, |n| n.local_slot);
    let Some((player, pose)) = riders.iter().find(|(r, _)| r.id == slot) else {
        return;
    };
    if race.elapsed < state.next {
        return;
    }
    let mut source = None;
    for (entity, traffic, t, heavy) in &trucks {
        let old = state.previous.insert(entity, traffic.speed);
        if heavy.kind == crate::heavy_traffic::Kind::Semi
            && t.translation.distance_squared(pose.translation) < 45. * 45.
            && old.is_some_and(|v| v - traffic.speed > 0.7)
        {
            source = Some(AudioCue::new(Cue::AirBrake, t.translation));
        }
    }
    if source.is_none()
        && track.metropolis.is_some()
        && player.progress >= 9200.
        && player.progress < 10500.
    {
        let (p, _) = track.street_sample(player.route, player.progress + 10., -12.);
        source = Some(AudioCue::new(Cue::Steam, p));
    }
    if let Some(cue) = source {
        audio.write(cue);
        state.next = race.elapsed + 7.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn master_gain_is_absolute_and_preserves_default_mix() {
        for base in [0., 0.1, 0.35, 0.55, 1.] {
            assert_eq!(mix_gain(base, 1., false), base);
            assert_eq!(mix_gain(base, 0., false), 0.);
            assert_eq!(mix_gain(base, 1., true), 0.);
            assert_eq!(mix_gain(base, 0.5, false), base * 0.5);
        }
    }
    #[test]
    fn recorded_wind_loop_decodes_with_headroom() {
        use bevy::audio::Decodable;
        let source = loop_recording(include_bytes!("../assets/audio/recorded/wind.wav"), 0., 7.);
        let peak = source.decoder().map(f32::abs).fold(0., f32::max);
        assert!((0.01..0.40).contains(&peak), "wind peak is {peak}");
    }
}
