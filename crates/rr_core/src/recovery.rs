//! On-foot authority and neutral-delay policy; independent of presentation/physics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FootMode {
    #[default]
    AutoSeekBike,
    Manual,
    Finished,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FootControl {
    pub mode: FootMode,
    neutral: f32,
}
impl FootControl {
    pub fn step(&mut self, active: bool, assist: bool, dt: f32, delay: f32) -> FootMode {
        if self.mode == FootMode::Finished {
            return self.mode;
        }
        if active {
            self.mode = FootMode::Manual;
            self.neutral = 0.0;
        } else if self.mode == FootMode::Manual {
            self.neutral += dt.max(0.0);
            if assist && self.neutral + 1e-6 >= delay {
                self.mode = FootMode::AutoSeekBike;
            }
        } else if !assist {
            self.mode = FootMode::Manual;
            self.neutral = 0.0;
        }
        self.mode
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manual_wins_and_assist_waits_for_continuous_neutral_input() {
        for hz in [30, 60, 120] {
            let dt = 1.0 / hz as f32;
            let mut c = FootControl::default();
            assert_eq!(c.step(true, true, dt, 0.35), FootMode::Manual);
            for _ in 0..(hz / 4) {
                assert_eq!(c.step(false, true, dt, 0.35), FootMode::Manual);
            }
            c.step(true, true, dt, 0.35);
            for _ in 0..hz {
                c.step(false, true, dt, 0.35);
            }
            assert_eq!(c.mode, FootMode::AutoSeekBike);
            c.step(true, false, dt, 0.35);
            for _ in 0..hz {
                assert_eq!(c.step(false, false, dt, 0.35), FootMode::Manual);
            }
            c.mode = FootMode::Finished;
            assert_eq!(c.step(true, true, dt, 0.35), FootMode::Finished);
        }
    }
}
