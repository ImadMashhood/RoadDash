//! Low-RPM, phase-aligned Engine Simulator captures for road traffic.
use super::RATE;

const FRAMES_PER_CYCLE: f32 = 256.;
const FRAMES: usize = 4096;
const FIRST_ANCHOR: f32 = 1000.;
const ANCHOR_STEP: f32 = 500.;
const ANCHORS: usize = 3;
type Clip = &'static [u8; 2 * FRAMES];
type Bank = [[Clip; 2]; ANCHORS];

macro_rules! bank {
    ($model:literal) => {{
        macro_rules! band {
            ($rpm:literal) => {
                [
                    include_bytes!(concat!(
                        "../../../assets/audio/traffic/",
                        $model,
                        "-",
                        $rpm,
                        "-coast.pcm"
                    )),
                    include_bytes!(concat!(
                        "../../../assets/audio/traffic/",
                        $model,
                        "-",
                        $rpm,
                        "-pull.pcm"
                    )),
                ]
            };
        }
        [band!("1000"), band!("1500"), band!("2000")]
    }};
}

#[derive(Clone, Copy)]
pub(crate) enum TrafficEngineKind {
    Car,
    Truck,
    Tractor,
    Semi,
}

pub(crate) struct TrafficEngine {
    bank: Bank,
    phase: f32,
    rpm: f32,
    load: f32,
}

impl TrafficEngine {
    pub(crate) fn new(kind: TrafficEngineKind) -> Self {
        Self {
            bank: match kind {
                TrafficEngineKind::Car => bank!("car"),
                TrafficEngineKind::Truck => bank!("truck"),
                TrafficEngineKind::Tractor => bank!("tractor"),
                TrafficEngineKind::Semi => bank!("semi"),
            },
            phase: 0.,
            rpm: 750.,
            load: 0.,
        }
    }

    pub(crate) fn sample(&mut self, rpm: f32, throttle: f32) -> f32 {
        self.rpm += (rpm.clamp(650., 2250.) - self.rpm) * 0.001;
        self.load += (throttle.clamp(0., 1.) - self.load) * 0.001;
        let step = self.rpm * FRAMES_PER_CYCLE / (120. * RATE as f32);
        self.phase = (self.phase + step) % FRAMES as f32;
        let index = self.phase as usize;
        let next = (index + 1) % FRAMES;
        let fraction = self.phase.fract();
        let read = |clip: Clip| {
            let decode = |frame: usize| {
                i16::from_le_bytes([clip[frame * 2], clip[frame * 2 + 1]]) as f32 / 32768.
            };
            let sample = decode(index);
            sample + (decode(next) - sample) * fraction
        };
        let band = ((self.rpm - FIRST_ANCHOR) / ANCHOR_STEP).clamp(0., (ANCHORS - 1) as f32);
        let lower = (band as usize).min(ANCHORS - 2);
        let fraction = band - lower as f32;
        let loaded = |[coast, pull]: [Clip; 2]| {
            let quiet = read(coast) * 0.72;
            quiet + (read(pull) - quiet) * self.load
        };
        let low = loaded(self.bank[lower]);
        let high = loaded(self.bank[lower + 1]);
        low + (high - low) * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_play_at_idle_and_driving_rpm_with_load_response() {
        let mut fingerprints = Vec::new();
        for kind in [
            TrafficEngineKind::Car,
            TrafficEngineKind::Truck,
            TrafficEngineKind::Tractor,
            TrafficEngineKind::Semi,
        ] {
            let mut engine = TrafficEngine::new(kind);
            let rms = |engine: &mut TrafficEngine, rpm, load| {
                for _ in 0..RATE / 4 {
                    engine.sample(rpm, load);
                }
                let power = (0..RATE)
                    .map(|_| {
                        let sample = engine.sample(rpm, load);
                        assert!(sample.is_finite() && sample.abs() < 0.85);
                        sample * sample
                    })
                    .sum::<f32>();
                (power / RATE as f32).sqrt()
            };
            let idle = rms(&mut engine, 720., 0.);
            let coast = rms(&mut engine, 1700., 0.);
            let pull = rms(&mut engine, 1700., 1.);
            assert!(idle > 0.01 && coast > 0.01);
            assert!(pull > coast * 1.15, "{pull} <= {coast}");
            fingerprints.push(engine.bank[0][1]);
        }
        for (index, capture) in fingerprints.iter().enumerate() {
            assert!(!fingerprints[..index].contains(capture));
        }
    }
}
