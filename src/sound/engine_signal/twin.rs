use super::RATE;

const ANCHORS: [f32; 6] = [1500., 3000., 4500., 6000., 7500., 9000.];
const FRAMES_PER_CYCLE: f32 = 256.;
const LOOP_FRAMES: usize = 4096;

type Clip = &'static [u8; LOOP_FRAMES * 2];

macro_rules! clip {
    ($name:literal) => {
        include_bytes!(concat!("../../../assets/audio/twin/", $name, ".pcm"))
    };
}

const PULL: [Clip; 6] = [
    clip!("1500-pull"),
    clip!("3000-pull"),
    clip!("4500-pull"),
    clip!("6000-pull"),
    clip!("7500-pull"),
    clip!("9000-pull"),
];
const COAST: [Clip; 5] = [
    clip!("1500-coast"),
    clip!("3000-coast"),
    clip!("4500-coast"),
    clip!("6000-coast"),
    clip!("7500-coast"),
];

pub(crate) struct TwinEngine {
    phase: f32,
    rpm: f32,
    load: f32,
    gear: usize,
    shift: f32,
    closed_throttle: f32,
    sustained_note: f32,
}

impl Default for TwinEngine {
    fn default() -> Self {
        Self {
            phase: 0.,
            rpm: 1200.,
            load: 0.,
            gear: 0,
            shift: 0.,
            closed_throttle: 0.,
            sustained_note: 0.,
        }
    }
}

impl TwinEngine {
    pub(crate) fn sample(&mut self, rpm: f32, throttle: f32, gear: usize) -> f32 {
        if gear > self.gear {
            self.shift = 1.;
        }
        self.gear = gear;
        self.shift *= 0.99945;
        self.rpm += (rpm.clamp(1200., 10500.) - self.rpm) * 0.0023;
        let load = throttle.clamp(0., 1.) * (1. - self.shift);
        let response = if self.shift > 0.15 || load > self.load {
            0.0019
        } else {
            0.00045
        };
        self.load += (load - self.load) * response;

        // Both load layers follow the same 720-degree crank phase.
        let step = self.rpm * FRAMES_PER_CYCLE / (120. * RATE as f32);
        self.phase = (self.phase + step) % LOOP_FRAMES as f32;
        let index = self.phase as usize;
        let next = (index + 1) % LOOP_FRAMES;
        let fraction = self.phase.fract();
        let read = |clip: Clip| {
            let decode = |frame: usize| {
                i16::from_le_bytes([clip[frame * 2], clip[frame * 2 + 1]]) as f32 / 32768.
            };
            let sample = decode(index);
            sample + (decode(next) - sample) * fraction
        };

        let band = ((self.rpm - ANCHORS[0]) / (ANCHORS[1] - ANCHORS[0]))
            .clamp(0., (ANCHORS.len() - 1) as f32);
        let lower = (band as usize).min(ANCHORS.len() - 2);
        let blend = band - lower as f32;
        let pull = read(PULL[lower]) * (1. - blend) + read(PULL[lower + 1]) * blend;
        let coast_lower = lower.min(COAST.len() - 1);
        let coast_upper = (lower + 1).min(COAST.len() - 1);
        let coast = read(COAST[coast_lower]) * (1. - blend) + read(COAST[coast_upper]) * blend;
        self.closed_throttle += (coast - self.closed_throttle) * 0.27;
        // Preserve some of the same exhaust note when the rider releases the
        // throttle at speed. Fade that layer out as the crank approaches idle.
        let rolling = ((self.rpm - 1500.) / 1800.).clamp(0., 1.);
        self.sustained_note += (pull - self.sustained_note) * 0.16;
        let quiet = (self.closed_throttle * (0.55 - rolling * 0.23)
            + self.sustained_note * rolling * 0.36)
            * (1. - self.shift * 0.45);
        quiet * (1. - self.load) + pull * self.load
    }
}
