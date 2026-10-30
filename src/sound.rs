//! Original synthesized PCM sounds, embedded in memory (no asset downloads).
use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
};
use bevy::{audio::Volume, prelude::*};
use std::{f32::consts::TAU, sync::Arc};

#[derive(Clone, Copy)]
pub enum Cue {
    Crash,
    Punch,
    Land,
    Footstep,
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
    crash: Handle<AudioSource>,
    punch: Handle<AudioSource>,
    land: Handle<AudioSource>,
    footstep: Handle<AudioSource>,
    beep: Handle<AudioSource>,
    finish: Handle<AudioSource>,
}
#[derive(Resource, Default)]
pub struct Muted(pub bool);
#[derive(Component, Clone, Copy)]
pub enum LoopSound {
    Engine,
    Wind,
    Nitro,
    Brake,
}

/// Deterministic 16-bit mono WAV. The first/last sample of loops is zero.
fn wave(duration: f32, kind: u8) -> AudioSource {
    const RATE: u32 = 22050;
    let count = (duration * RATE as f32) as u32;
    let mut bytes = Vec::with_capacity(44 + count as usize * 2);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + count * 2).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&RATE.to_le_bytes());
    bytes.extend_from_slice(&(RATE * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(count * 2).to_le_bytes());
    let mut state = 1234567u32;
    let mut filtered = 0.0;
    for i in 0..count {
        let t = i as f32 / RATE as f32;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = (state >> 8) as f32 / 8388608.0 - 1.0;
        filtered = filtered * 0.92 + noise * 0.08;
        let decay = (-t * if kind == 4 { 5.0 } else { 15.0 }).exp();
        let value = match kind {
            0 => {
                // Engine firing pulses and harmonics at a loop-safe base RPM.
                let phase = TAU * 60.0 * t;
                ((phase.sin() + 0.45 * (phase * 2.0).sin() + 0.22 * (phase * 4.0).sin()) * 0.35
                    + noise * 0.035)
                    * 0.75
            }
            1 => filtered * 1.5,
            2 => noise * 0.16 + (TAU * 430.0 * t).sin() * 0.12,
            3 => {
                ((TAU * (730.0 * t + 35.0 * (TAU * 9.0 * t).sin())).sin() * 0.2 + noise * 0.12)
                    * 0.6
            }
            4 => (noise * 0.7 + (TAU * 67.0 * t).sin() * 0.45) * decay,
            5 => (noise * 0.3 + (TAU * 110.0 * t).sin() * 0.6) * decay,
            6 => (filtered * 2.0 + (TAU * 75.0 * t).sin() * 0.3) * decay,
            7 => filtered * 3.0 * decay,
            8 => (TAU * 880.0 * t).sin() * 0.35,
            _ => {
                let note = [523.25, 659.25, 783.99, 1046.5][((t / 0.17) as usize).min(3)];
                (TAU * note * t).sin() * 0.3 * (-t * 1.8).exp()
            }
        };
        let fade = (t * 300.0).min(1.0) * ((duration - t) * 300.0).min(1.0);
        // Quantize explicitly; WAV data must contain signed 16-bit samples.
        bytes
            .extend_from_slice(&(((value * fade).clamp(-1.0, 1.0) * 30000.0) as i16).to_le_bytes());
    }
    AudioSource {
        bytes: Arc::from(bytes),
    }
}

pub fn setup(mut commands: Commands, mut sources: ResMut<Assets<AudioSource>>) {
    for (kind, marker) in [
        (0, LoopSound::Engine),
        (1, LoopSound::Wind),
        (2, LoopSound::Nitro),
        (3, LoopSound::Brake),
    ] {
        commands.spawn((
            AudioPlayer::new(sources.add(wave(1.0, kind))),
            PlaybackSettings::LOOP.with_volume(Volume::SILENT),
            marker,
        ));
    }
    commands.insert_resource(Sounds {
        crash: sources.add(wave(0.7, 4)),
        punch: sources.add(wave(0.2, 5)),
        land: sources.add(wave(0.3, 6)),
        footstep: sources.add(wave(0.15, 7)),
        beep: sources.add(wave(0.13, 8)),
        finish: sources.add(wave(0.85, 9)),
    });
}

#[allow(clippy::too_many_arguments)]
pub fn update(
    mut commands: Commands,
    controls: Res<crate::controls::Controls>,
    race: Res<Race>,
    sounds: Res<Sounds>,
    muted: Res<Muted>,
    riders: Query<(&Rider, &Recovery, &Transform)>,
    bike_states: Query<(&Rider, &crate::vehicle::BikeState)>,
    mut cues: MessageReader<AudioCue>,
    mut sinks: Query<(&mut AudioSink, Option<&LoopSound>)>,
    mut last_count: Local<i32>,
    mut last_phase: Local<Option<Phase>>,
) {
    let Some((player, recovery, player_transform)) = riders.iter().find(|(r, _, _)| r.id == 0)
    else {
        return;
    };
    let riding = race.phase == Phase::Racing && recovery.stage == RecoveryStage::Riding && !muted.0;
    for (mut sink, loop_sound) in &mut sinks {
        if race.phase == Phase::Paused {
            sink.pause();
        } else {
            sink.play();
        }
        let Some(loop_sound) = loop_sound else {
            if muted.0 {
                sink.set_volume(Volume::SILENT);
            }
            continue;
        };
        let boost = controls.buffer.held.nitro && controls.buffer.held.throttle;
        let volume = if !riding {
            0.0
        } else {
            match loop_sound {
                LoopSound::Engine => 0.20 + player.speed * 0.002,
                LoopSound::Wind => (player.speed / 75.0).powi(2) * 0.35,
                LoopSound::Nitro => {
                    if boost && player.nitro > 0.0 && player.speed > 2.0 {
                        0.20
                    } else {
                        0.0
                    }
                }
                LoopSound::Brake => {
                    if player.speed > 12.0 && controls.buffer.held.brake {
                        0.18
                    } else {
                        0.0
                    }
                }
            }
        };
        sink.set_volume(Volume::Linear(volume));
        if matches!(loop_sound, LoopSound::Engine) {
            sink.set_speed(bike_states.iter().find(|(r, _)| r.id == 0).map_or(
                0.85 + (player.speed % 12.0) * 0.075 + player.speed * 0.012,
                |(_, b)| 0.7 + b.rpm / 6500.0,
            ));
        }
    }
    for cue in cues.read() {
        // Keep distant AI impacts quiet instead of filling the mix with eight riders.
        let distance = cue.position.distance(player_transform.translation);
        let gain = (1.0 / (1.0 + distance * 0.09)).clamp(0.0, 1.0);
        let handle = match cue.kind {
            Cue::Crash => &sounds.crash,
            Cue::Punch => &sounds.punch,
            Cue::Land => &sounds.land,
            Cue::Footstep => &sounds.footstep,
        };
        if !muted.0 && race.phase == Phase::Racing {
            commands.spawn((
                AudioPlayer::new(handle.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.55 * gain)),
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
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.35)),
        ));
    }
    if race.phase == Phase::Finished && *last_phase != Some(Phase::Finished) && !muted.0 {
        commands.spawn((
            AudioPlayer::new(sounds.finish.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.5)),
        ));
    }
    *last_count = count;
    *last_phase = Some(race.phase);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_audio_is_valid_pcm_and_not_silent() {
        for kind in 0..10 {
            let source = wave(0.25, kind);
            assert_eq!(&source.bytes[0..4], b"RIFF");
            assert_eq!(&source.bytes[8..12], b"WAVE");
            assert_eq!(&source.bytes[36..40], b"data");
            assert_eq!(
                u32::from_le_bytes(source.bytes[40..44].try_into().unwrap()) as usize,
                source.bytes.len() - 44
            );
            assert!(source.bytes[44..].iter().any(|v| *v != 0));
        }
    }
}
