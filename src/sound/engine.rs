use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use bevy::{
    audio::{ChannelCount, Decodable, SampleRate, Source},
    prelude::*,
};

use super::engine_signal::{EngineSignal, RATE, StreetEngine, TwinEngine};

#[derive(Clone, Default)]
pub(crate) struct EngineControl(Arc<AtomicU32>);

impl EngineControl {
    pub(crate) fn update(&self, rpm: f32, throttle: f32, gear: usize) {
        let rpm = rpm.round().clamp(1200., 10500.) as u32;
        let load = (throttle.clamp(0., 1.) * 255.).round() as u32;
        self.0.store(
            rpm | (load << 14) | ((gear.min(5) as u32) << 22),
            Ordering::Relaxed,
        );
    }
}

#[derive(Asset, TypePath)]
pub(crate) struct EngineSource(pub(crate) EngineControl);

enum Voice {
    Twin(TwinEngine),
    Street(StreetEngine),
    Synth(EngineSignal),
}

pub(crate) struct Decoder {
    control: EngineControl,
    voice: Voice,
}

impl Decodable for EngineSource {
    type Decoder = Decoder;

    fn decoder(&self) -> Decoder {
        Decoder {
            control: self.0.clone(),
            voice: if std::env::var_os("ROADDASH_ENGINE_SYNTH").is_some() {
                Voice::Synth(EngineSignal::default())
            } else if std::env::var_os("ROADDASH_ENGINE_STREET").is_some() {
                Voice::Street(StreetEngine::default())
            } else {
                Voice::Twin(TwinEngine::default())
            },
        }
    }
}

impl Iterator for Decoder {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let state = self.control.0.load(Ordering::Relaxed);
        let rpm = (state & 0x3fff) as f32;
        let throttle = ((state >> 14) & 255) as f32 / 255.;
        let gear = (state >> 22) as usize;
        Some(match &mut self.voice {
            Voice::Twin(voice) => voice.sample(rpm, throttle, gear),
            Voice::Street(voice) => voice.sample(rpm, throttle, gear),
            Voice::Synth(voice) => voice.sample(rpm, throttle, gear),
        })
    }
}

impl Source for Decoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> ChannelCount {
        ChannelCount::MIN
    }
    fn sample_rate(&self) -> SampleRate {
        SampleRate::new(RATE).expect("nonzero engine sample rate")
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
