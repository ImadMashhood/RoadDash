//! Original four-stroke, inline-four exhaust voice driven by crank speed.
mod twin;
pub(crate) use twin::TwinEngine;
pub const RATE: u32 = 48_000;

pub struct EngineSignal {
    phase: f32,
    rpm: f32,
    load: f32,
    gear: usize,
    shift: f32,
    exhaust: f32,
    dc: f32,
}

impl Default for EngineSignal {
    fn default() -> Self {
        Self {
            phase: 0.,
            rpm: 1200.,
            load: 0.,
            gear: 0,
            shift: 0.,
            exhaust: 0.,
            dc: 0.,
        }
    }
}

impl EngineSignal {
    pub fn sample(&mut self, rpm: f32, throttle: f32, gear: usize) -> f32 {
        if gear > self.gear {
            self.shift = 1.;
        }
        self.gear = gear;
        self.shift *= 0.9995;
        self.rpm += (rpm.clamp(1200., 10500.) - self.rpm) * 0.0016;
        let load = throttle.clamp(0., 1.) * (1. - self.shift * 0.85);
        self.load += (load - self.load) * 0.0009;

        // Four cylinders fire twice per crank revolution. Preserve phase when RPM changes.
        self.phase = (self.phase + self.rpm * 2. / (60. * RATE as f32)).fract();
        let angle = std::f32::consts::TAU * self.phase;
        let (sine, cosine) = angle.sin_cos();
        let second = 2. * sine * cosine;
        let third = sine * (3. - 4. * sine * sine);
        let fourth = 2. * second * (2. * cosine * cosine - 1.);
        let pulse = (sine * 0.55 + second * 0.24 + third * 0.13 + fourth * 0.08)
            * (0.68 + self.load * 0.32);
        let pressure = (pulse * (1.1 + self.load * 2.5)).tanh();

        // Closed throttle reduces brightness as well as loudness.
        self.exhaust += (pressure - self.exhaust) * (0.11 + 0.42 * self.load);
        self.dc += (self.exhaust - self.dc) * 0.002;
        (self.exhaust - self.dc) * (0.25 + self.load * 0.45)
    }
}

const PULL_LOW: &[u8] = include_bytes!("../../assets/audio/demo/street-low.pcm");
const PULL_HIGH: &[u8] = include_bytes!("../../assets/audio/demo/street-high.pcm");
const COAST: &[u8] = include_bytes!("../../assets/audio/demo/street-coast.pcm");

pub struct StreetEngine {
    phases: [f32; 3],
    rpm: f32,
    load: f32,
    gear: usize,
    shift: f32,
    exhaust: f32,
    dc: f32,
}

impl Default for StreetEngine {
    fn default() -> Self {
        Self {
            phases: [0.; 3],
            rpm: 1200.,
            load: 0.,
            gear: 0,
            shift: 0.,
            exhaust: 0.,
            dc: 0.,
        }
    }
}

impl StreetEngine {
    pub fn sample(&mut self, rpm: f32, throttle: f32, gear: usize) -> f32 {
        if gear > self.gear {
            self.shift = 1.;
        }
        self.gear = gear;
        self.shift *= 0.99955;
        self.rpm += (rpm.clamp(1200., 10500.) - self.rpm) * 0.0016;
        let load = throttle.clamp(0., 1.) * (1. - self.shift);
        self.load += (load - self.load) * 0.0017;

        // The roadside recording has no tachometer data. These are playback
        // tuning anchors, with bounded pitch at the ends of the gearbox range.
        let [low, high, coast] = std::array::from_fn(|index| {
            let (pcm, anchor) = [(PULL_LOW, 4000.), (PULL_HIGH, 6400.), (COAST, 4600.)][index];
            recording_sample(
                pcm,
                &mut self.phases[index],
                (self.rpm / anchor).clamp(0.47, 1.48),
            )
        });
        let band = ((self.rpm - 4200.) / 2600.).clamp(0., 1.);
        let pull = low * (1. - band) + high * band;
        let pressure = coast * (1. - self.load) + pull * self.load;
        self.exhaust += (pressure - self.exhaust) * (0.08 + 0.43 * self.load);
        self.dc += (self.exhaust - self.dc) * 0.004;
        ((self.exhaust - self.dc) * (1.1 + 1.6 * self.load)).tanh()
    }
}

fn recording_sample(pcm: &[u8], phase: &mut f32, pitch: f32) -> f32 {
    let frames = pcm.len() / 2;
    let index = *phase as usize;
    let read = |index: usize| i16::from_le_bytes([pcm[2 * index], pcm[2 * index + 1]]) as f32;
    let a = read(index);
    let b = read((index + 1) % frames);
    let sample = a + (b - a) * phase.fract();
    *phase = (*phase + pitch * (24_000. / RATE as f32)).rem_euclid(frames as f32);
    sample / 32768.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycles(rpm: f32, throttle: f32) -> (usize, f32) {
        let mut signal = EngineSignal::default();
        for _ in 0..RATE {
            signal.sample(rpm, throttle, 1);
        }
        let mut previous = 0.;
        let mut crossings = 0;
        let mut energy = 0.;
        for _ in 0..RATE {
            let sample = signal.sample(rpm, throttle, 1);
            assert!(sample.is_finite() && sample.abs() <= 1.);
            crossings += usize::from(previous <= 0. && sample > 0.);
            energy += sample * sample;
            previous = sample;
        }
        (crossings, energy)
    }

    #[test]
    fn rpm_sets_firing_rate_and_load_changes_the_signal() {
        for rpm in [1200., 4000., 7600.] {
            let (loaded, energy) = cycles(rpm, 1.);
            let (coasting, quiet) = cycles(rpm, 0.);
            let expected = (rpm / 30.).round() as usize;
            assert!(loaded.abs_diff(expected) <= 1);
            assert!(coasting.abs_diff(expected) <= 1);
            assert!(quiet < energy * 0.4);
        }
    }
}
