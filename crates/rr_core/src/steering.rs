//! Independent normalized implementation of the reference's growing/relaxing
//! steering-force mechanism. Tuning is authored RoadDash data, not original SPECs.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SteeringTuning {
    pub rise_seconds: f32,
    pub release_seconds: f32,
    pub full_speed_mps: f32,
}
impl Default for SteeringTuning {
    fn default() -> Self {
        Self {
            rise_seconds: 0.5,
            release_seconds: 0.3,
            full_speed_mps: 12.0,
        }
    }
}
impl SteeringTuning {
    pub fn validate(self) -> Result<(), String> {
        if !self.rise_seconds.is_finite()
            || !(0.1..=4.0).contains(&self.rise_seconds)
            || !self.release_seconds.is_finite()
            || !(0.1..=4.0).contains(&self.release_seconds)
            || !self.full_speed_mps.is_finite()
            || !(1.0..=40.0).contains(&self.full_speed_mps)
        {
            return Err("invalid steering response tuning".into());
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct SteeringMemory {
    pub force: f32,
}
impl SteeringMemory {
    pub fn step(&mut self, input: f32, speed: f32, dt: f32, tuning: SteeringTuning) -> f32 {
        if !input.is_finite() || !speed.is_finite() || !dt.is_finite() || speed <= 0.0 {
            self.force = 0.0;
            return 0.0;
        }
        if dt <= 0.0 {
            return self.force;
        }
        // Digital full lock requests a 45-degree steering angle in the reference.
        let target = ((input.clamp(-1.0, 1.0) * std::f32::consts::FRAC_PI_4).sin()
            / std::f32::consts::FRAC_1_SQRT_2
            * speed
            / tuning.full_speed_mps)
            .clamp(-1.0, 1.0);
        let seconds = if target == 0.0 || target.abs() <= self.force.abs() {
            tuning.release_seconds
        } else {
            tuning.rise_seconds
        };
        let limit = dt / seconds;
        self.force += (target - self.force).clamp(-limit, limit);
        self.force
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn holding_builds_force_and_release_or_reversal_unwinds_it() {
        let tuning = SteeringTuning {
            rise_seconds: 1.0,
            release_seconds: 0.6,
            ..Default::default()
        };
        let mut s = SteeringMemory::default();
        for _ in 0..6 {
            s.step(1.0, 30.0, 1.0 / 60.0, tuning);
        }
        let tap = s.force;
        for _ in 0..24 {
            s.step(1.0, 30.0, 1.0 / 60.0, tuning);
        }
        assert!(s.force > tap * 4.9 && s.force < 0.51);
        let held = s.force;
        s.step(-1.0, 30.0, 1.0 / 60.0, tuning);
        assert!(s.force > 0.0 && s.force < held);
        for _ in 0..60 {
            s.step(1.0, 30.0, 1.0 / 60.0, tuning);
        }
        assert!((s.force - 1.0).abs() < 0.001);
        for _ in 0..36 {
            s.step(0.0, 30.0, 1.0 / 60.0, tuning);
        }
        assert!(s.force.abs() < 0.001);
        s.step(1.0, 0.0, 1.0 / 60.0, tuning);
        assert_eq!(s.force, 0.0);
    }
    #[test]
    fn response_matches_across_fixed_step_sizes_and_remains_speed_dependent() {
        let mut results = vec![];
        for hz in [30, 60, 120] {
            let mut s = SteeringMemory::default();
            for _ in 0..hz / 2 {
                s.step(
                    1.0,
                    30.0,
                    1.0 / hz as f32,
                    SteeringTuning {
                        rise_seconds: 1.0,
                        release_seconds: 0.6,
                        ..Default::default()
                    },
                );
            }
            results.push(s.force);
        }
        assert!(results.iter().all(|v| (*v - 0.5).abs() < 0.001));
        let mut slow = SteeringMemory::default();
        for _ in 0..120 {
            slow.step(1.0, 3.0, 1.0 / 60.0, SteeringTuning::default());
        }
        assert!((slow.force - 0.25).abs() < 0.001);
    }
    #[test]
    fn requested_double_response_is_symmetric_and_keeps_the_same_limit() {
        for direction in [-1.0, 1.0] {
            let mut ticks: Vec<i32> = vec![];
            for tune in [
                SteeringTuning {
                    rise_seconds: 1.0,
                    release_seconds: 0.6,
                    ..Default::default()
                },
                SteeringTuning::default(),
            ] {
                let mut s = SteeringMemory::default();
                let mut count = 0;
                while s.force.abs() < 0.999 && count < 300 {
                    s.step(direction, 30.0, 1.0 / 120.0, tune);
                    count += 1;
                }
                assert!((s.force - direction).abs() < 0.002);
                ticks.push(count);
            }
            assert!(
                (ticks[0] - ticks[1] * 2).abs() <= 2,
                "not twice as fast: {ticks:?}"
            );
        }
    }
}
