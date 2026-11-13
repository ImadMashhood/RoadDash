//! Bounded live speech capture and playback. Audio callbacks never wait for the game thread.
use super::*;
use cpal::{
    FromSample, SampleFormat, SizedSample,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

const SAMPLE_RATE: u32 = 8_000;
pub(super) const SAMPLES: usize = 320;
const BUFFER: usize = SAMPLES * 4;
const STREAM_TIMEOUT: Option<Duration> = Some(Duration::from_millis(300));

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Activation,
    PushToTalk,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Preferences {
    pub enabled: bool,
    pub microphone: bool,
    pub mode: Mode,
    pub threshold: f32,
    pub volume: f32,
    pub push_key: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            enabled: true,
            microphone: false,
            mode: Mode::Activation,
            threshold: 0.035,
            volume: 1.,
            push_key: "KeyV".into(),
        }
    }
}

impl Preferences {
    pub fn validate(&self) -> bool {
        (0.005..=0.25).contains(&self.threshold)
            && (0. ..=1.).contains(&self.volume)
            && matches!(self.push_key.as_bytes(), [b'K', b'e', b'y', b'A'..=b'Z'])
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct Packet {
    speech: String,
}

impl Packet {
    pub(super) fn from_samples(samples: [f32; SAMPLES]) -> Self {
        let mut speech = String::with_capacity(SAMPLES * 2);
        for sample in samples {
            // Linear 8-bit PCM has bounded work and no decoder state.
            let pcm = (sample.clamp(-1., 1.) * 127.).round() as i8 as u8;
            const HEX: &[u8; 16] = b"0123456789abcdef";
            speech.push(HEX[(pcm >> 4) as usize] as char);
            speech.push(HEX[(pcm & 15) as usize] as char);
        }
        Self { speech }
    }

    pub(super) fn samples(&self) -> Option<[f32; SAMPLES]> {
        if self.speech.len() != SAMPLES * 2 {
            return None;
        }
        let mut samples = [0.; SAMPLES];
        let digit = |n| match n {
            b'0'..=b'9' => Some(n - b'0'),
            b'a'..=b'f' => Some(n - b'a' + 10),
            _ => None,
        };
        for (pair, sample) in self.speech.as_bytes().chunks_exact(2).zip(&mut samples) {
            let pcm = digit(pair[0])? * 16 + digit(pair[1])?;
            *sample = (pcm as i8 as f32 / 127.).clamp(-1., 1.);
        }
        Some(samples)
    }
}

struct Stream {
    _device: cpal::Stream,
    failed: Arc<AtomicBool>,
}

struct Playback {
    samples: [VecDeque<f32>; RACERS],
    gain: f32,
}
impl Default for Playback {
    fn default() -> Self {
        Self {
            samples: std::array::from_fn(|_| VecDeque::with_capacity(BUFFER)),
            gain: 0.,
        }
    }
}

#[derive(Default)]
pub(super) struct Voice {
    input: Option<Stream>,
    output: Option<Stream>,
    capture: Arc<Mutex<VecDeque<f32>>>,
    capture_gate: Arc<AtomicBool>,
    playback: Arc<Mutex<Playback>>,
    input_error: Option<String>,
    output_error: Option<String>,
    last_loud: Option<Instant>,
    last_sent: Option<Instant>,
    last_heard: Option<(usize, Instant)>,
}

impl Voice {
    pub(super) fn configure(&mut self, transmitting: bool, listening: bool, playback_gain: f32) {
        if !transmitting {
            self.capture_gate.store(false, Ordering::Relaxed);
            self.input = None;
            self.input_error = None;
            self.last_loud = None;
            self.last_sent = None;
        } else if self.input.as_ref().is_some_and(Stream::failed) {
            self.input = None;
            self.input_error =
                Some("Microphone disconnected. Toggle the microphone to retry.".into());
        } else if self.input.is_none() && self.input_error.is_none() {
            self.capture = Arc::new(Mutex::new(VecDeque::with_capacity(BUFFER)));
            match open_input(self.capture.clone(), self.capture_gate.clone()) {
                Ok(input) => self.input = Some(input),
                Err(error) => self.input_error = Some(format!("Microphone unavailable: {error}")),
            }
        }
        if !listening {
            self.output = None;
            self.output_error = None;
            self.last_heard = None;
        } else if self.output.as_ref().is_some_and(Stream::failed) {
            self.output = None;
            self.output_error =
                Some("Voice output disconnected. Toggle voice chat to retry.".into());
        }
        let mut playback = self.playback.lock().unwrap_or_else(|e| e.into_inner());
        playback.gain = playback_gain;
        if !listening || playback_gain == 0. {
            for samples in &mut playback.samples {
                samples.clear();
            }
        }
    }

    pub(super) fn outgoing(&mut self, settings: &Preferences, push: bool) -> Vec<Packet> {
        if self.input.is_none() {
            return Vec::new();
        }
        let capture_enabled = settings.mode == Mode::Activation || push;
        let was_enabled = self.capture_gate.swap(capture_enabled, Ordering::Relaxed);
        let Ok(mut capture) = self.capture.try_lock() else {
            return Vec::new();
        };
        if !capture_enabled || !was_enabled {
            capture.clear();
            return Vec::new();
        }
        let mut packets = Vec::new();
        while capture.len() >= SAMPLES {
            let mut samples = [0.; SAMPLES];
            for (sample, captured) in samples.iter_mut().zip(capture.drain(..SAMPLES)) {
                *sample = captured;
            }
            let rms = (samples.iter().map(|x| x * x).sum::<f32>() / SAMPLES as f32).sqrt();
            if rms >= settings.threshold {
                self.last_loud = Some(Instant::now());
            }
            if settings.mode == Mode::PushToTalk
                || self
                    .last_loud
                    .is_some_and(|t| t.elapsed() < Duration::from_millis(160))
            {
                packets.push(Packet::from_samples(samples));
                self.last_sent = Some(Instant::now());
            }
        }
        packets
    }

    pub(super) fn hear(&mut self, slot: usize, gain: u8, packet: &Packet) {
        let Some(samples) = packet.samples() else {
            return;
        };
        if slot >= RACERS || gain == 0 {
            return;
        }
        {
            let mut playback = self.playback.lock().unwrap_or_else(|e| e.into_inner());
            if playback.gain == 0. {
                return;
            }
            let queue = &mut playback.samples[slot];
            if queue.is_empty() {
                // Give the next packet one network frame to arrive before an underrun.
                queue.resize(SAMPLES, 0.);
            }
            append_samples(
                queue,
                samples.into_iter().map(|s| s * f32::from(gain) / 255.),
            );
        }
        if self.output.is_none() && self.output_error.is_none() {
            match open_output(self.playback.clone()) {
                Ok(output) => self.output = Some(output),
                Err(error) => {
                    self.output_error = Some(format!("Voice output unavailable: {error}"))
                }
            }
        }
        if self.output.is_some() {
            self.last_heard = Some((slot, Instant::now()));
        }
    }

    pub(super) fn clear_slot(&mut self, slot: usize) {
        if let Some(queue) = self
            .playback
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .samples
            .get_mut(slot)
        {
            queue.clear();
        }
        if self.last_heard.is_some_and(|(speaker, _)| speaker == slot) {
            self.last_heard = None;
        }
    }

    pub(super) fn status(&self, transmitting: bool, lobby: Option<&LobbyView>) -> String {
        if let Some(error) = self.input_error.as_ref().or(self.output_error.as_ref()) {
            return error.clone();
        }
        if transmitting
            && self
                .last_sent
                .is_some_and(|t| t.elapsed() < Duration::from_millis(250))
        {
            return "Microphone transmitting".into();
        }
        if let Some((slot, time)) = self.last_heard
            && time.elapsed() < Duration::from_millis(400)
            && let Some(player) =
                lobby.and_then(|l| l.players.iter().find(|p| p.slot == slot && !p.bot))
        {
            return format!("Speaking · {}", player.name);
        }
        if transmitting && self.input.is_some() {
            return "Microphone ready".into();
        }
        String::new()
    }
}

impl Stream {
    fn failed(&self) -> bool {
        self.failed.load(Ordering::Relaxed)
    }
}

fn append_samples(queue: &mut VecDeque<f32>, samples: impl ExactSizeIterator<Item = f32>) {
    let drop = (queue.len() + samples.len()).saturating_sub(BUFFER);
    queue.drain(..drop);
    queue.extend(samples);
}

fn open_input(queue: Arc<Mutex<VecDeque<f32>>>, gate: Arc<AtomicBool>) -> Result<Stream, String> {
    let device = cpal::default_host()
        .default_input_device()
        .ok_or("no default input device")?;
    let config = device.default_input_config().map_err(|e| e.to_string())?;
    match config.sample_format() {
        SampleFormat::F32 => input::<f32>(&device, &config.config(), queue, gate),
        SampleFormat::I16 => input::<i16>(&device, &config.config(), queue, gate),
        SampleFormat::U16 => input::<u16>(&device, &config.config(), queue, gate),
        SampleFormat::I32 => input::<i32>(&device, &config.config(), queue, gate),
        format => Err(format!("unsupported input format {format}")),
    }
}

fn input<T: SizedSample>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    queue: Arc<Mutex<VecDeque<f32>>>,
    gate: Arc<AtomicBool>,
) -> Result<Stream, String>
where
    f32: FromSample<T>,
{
    let failed = Arc::new(AtomicBool::new(false));
    let flag = failed.clone();
    let (channels, rate) = (usize::from(config.channels), config.sample_rate);
    let (mut phase, mut sum, mut count) = (0, 0., 0);
    let stream = device
        .build_input_stream::<T, _, _>(
            config,
            move |data, _| {
                let Ok(mut queue) = queue.try_lock() else {
                    phase = 0;
                    sum = 0.;
                    count = 0;
                    return;
                };
                if !gate.load(Ordering::Relaxed) {
                    phase = 0;
                    sum = 0.;
                    count = 0;
                    queue.clear();
                    return;
                }
                for frame in data.chunks_exact(channels) {
                    let value = frame.iter().map(|s| s.to_sample::<f32>()).sum::<f32>();
                    sum += if value.is_finite() {
                        value / channels as f32
                    } else {
                        0.
                    };
                    count += 1;
                    phase += SAMPLE_RATE;
                    if phase >= rate {
                        let sample = sum / count as f32;
                        while phase >= rate {
                            append_samples(&mut queue, std::iter::once(sample));
                            phase -= rate;
                        }
                        sum = 0.;
                        count = 0;
                    }
                }
            },
            move |error| {
                if !matches!(error, cpal::StreamError::BufferUnderrun) {
                    flag.store(true, Ordering::Relaxed);
                }
            },
            STREAM_TIMEOUT,
        )
        .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Stream {
        _device: stream,
        failed,
    })
}

fn open_output(playback: Arc<Mutex<Playback>>) -> Result<Stream, String> {
    let device = cpal::default_host()
        .default_output_device()
        .ok_or("no default output device")?;
    let config = device.default_output_config().map_err(|e| e.to_string())?;
    match config.sample_format() {
        SampleFormat::F32 => output::<f32>(&device, &config.config(), playback),
        SampleFormat::I16 => output::<i16>(&device, &config.config(), playback),
        SampleFormat::U16 => output::<u16>(&device, &config.config(), playback),
        SampleFormat::I32 => output::<i32>(&device, &config.config(), playback),
        format => Err(format!("unsupported output format {format}")),
    }
}

fn output<T: SizedSample + FromSample<f32>>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    playback: Arc<Mutex<Playback>>,
) -> Result<Stream, String> {
    let failed = Arc::new(AtomicBool::new(false));
    let flag = failed.clone();
    let (channels, rate) = (usize::from(config.channels), config.sample_rate);
    let mut phase = rate;
    let mut sample: f32 = 0.;
    let stream = device
        .build_output_stream::<T, _, _>(
            config,
            move |data, _| {
                let Ok(mut playback) = playback.try_lock() else {
                    data.fill(T::EQUILIBRIUM);
                    return;
                };
                for frame in data.chunks_exact_mut(channels) {
                    while phase >= rate {
                        phase -= rate;
                        sample = playback
                            .samples
                            .iter_mut()
                            .filter_map(VecDeque::pop_front)
                            .sum();
                    }
                    frame.fill(T::from_sample((sample * playback.gain).clamp(-1., 1.)));
                    phase += SAMPLE_RATE;
                }
            },
            move |error| {
                if !matches!(error, cpal::StreamError::BufferUnderrun) {
                    flag.store(true, Ordering::Relaxed);
                }
            },
            STREAM_TIMEOUT,
        )
        .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Stream {
        _device: stream,
        failed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speech_frame_is_bounded_and_proximity_fades() {
        let mut input = [0.; SAMPLES];
        input[0] = -1.;
        input[1] = 1.;
        input[2] = 0.42;
        let packet = Packet::from_samples(input);
        let samples = packet.samples().unwrap();
        assert_eq!(&samples[..2], &input[..2]);
        assert!((samples[2] - input[2]).abs() < 0.01);
        assert!(
            Packet {
                speech: "a".repeat(SAMPLES * 2 + 1)
            }
            .samples()
            .is_none()
        );
        assert!(
            Packet {
                speech: "g".repeat(SAMPLES * 2)
            }
            .samples()
            .is_none()
        );
        use rr_core::online::proximity_voice_gain;
        assert_eq!(proximity_voice_gain(0.), 255);
        assert_eq!(proximity_voice_gain(27.5), 128);
        assert_eq!(proximity_voice_gain(55.), 0);
        assert_eq!(proximity_voice_gain(f32::NAN), 0);
    }
}
