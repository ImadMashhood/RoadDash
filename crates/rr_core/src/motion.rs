//! Presentation timing and original riding-pose selection; no physics or renderer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RidePose {
    LowSpeed,
    Acceleration,
    Normal,
    Tuck,
}
/// Original selector (actor_animation.cpp), using original fixed-point units.
pub fn reference_ride_pose(
    speed: i32,
    throttle: i32,
    lateral: i32,
    flags: u8,
    boost: i32,
) -> RidePose {
    if speed < 300 {
        RidePose::LowSpeed
    } else if speed < 0x708 && throttle > 30 {
        RidePose::Acceleration
    } else if (flags & 0x20 == 0 || boost > 99) && (lateral < -0x1000 || speed >= 0x251d) {
        RidePose::Tuck
    } else {
        RidePose::Normal
    }
}
/// Inverse of the original HUD's unrounded mph coefficients: 41 / 4096.
/// Rendering policy only; this conversion never changes motorcycle speed.
pub fn reference_velocity(mps: f32) -> i32 {
    (mps.max(0.0) / 0.44704 * 4096.0 / 41.0).round() as i32
}
pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
pub fn approach(current: f32, target: f32, dt: f32, seconds: f32) -> f32 {
    current + (target - current) * (1.0 - (-dt.max(0.0) / seconds).exp())
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseSample {
    pub from: usize,
    pub to: usize,
    pub blend: f32,
}
/// Sample time between source phases; a repeated image is still a timed phase.
pub fn sample_phase(durations: &[f32], seconds: f32) -> PhaseSample {
    assert!(!durations.is_empty());
    let mut t = seconds.max(0.0);
    for (i, &d) in durations.iter().enumerate() {
        if t < d || i + 1 == durations.len() {
            return PhaseSample {
                from: i,
                to: (i + 1).min(durations.len() - 1),
                blend: smoothstep(t / d),
            };
        }
        t -= d;
    }
    unreachable!()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_selector_boundaries_and_boost_gate() {
        assert_eq!(reference_ride_pose(299, 100, 0, 0, 0), RidePose::LowSpeed);
        assert_eq!(
            reference_ride_pose(300, 31, 0, 0, 0),
            RidePose::Acceleration
        );
        assert_eq!(reference_ride_pose(1799, 30, 0, 0, 0), RidePose::Normal);
        assert_eq!(reference_ride_pose(9500, 100, 0, 0, 0), RidePose::Normal);
        assert_eq!(reference_ride_pose(9501, 100, 0, 0, 0), RidePose::Tuck);
        assert_eq!(
            reference_ride_pose(9501, 100, 0, 0x20, 99),
            RidePose::Normal
        );
        assert_eq!(reference_ride_pose(9501, 100, 0, 0x20, 100), RidePose::Tuck);
        assert_eq!(reference_ride_pose(2000, 0, -4097, 0, 0), RidePose::Tuck);
    }
    #[test]
    fn phase_order_repeats_and_end_are_preserved() {
        let ds = [0.14, 0.12, 0.14];
        assert_eq!(sample_phase(&ds, 0.0).from, 0);
        assert_eq!(sample_phase(&ds, 0.14).from, 1);
        assert_eq!(sample_phase(&ds, 99.0).from, 2);
        assert_eq!(sample_phase(&ds, 99.0).to, 2);
    }
    #[test]
    fn smoothing_is_frame_rate_independent() {
        let mut results = vec![];
        for hz in [30, 60, 120] {
            let mut x = 0.0;
            for _ in 0..hz {
                x = approach(x, 1.0, 1.0 / hz as f32, 0.18);
            }
            results.push(x);
        }
        assert!((results[0] - results[2]).abs() < 1e-6);
        assert!((reference_velocity(42.51) - 9501).abs() < 5);
    }
}
