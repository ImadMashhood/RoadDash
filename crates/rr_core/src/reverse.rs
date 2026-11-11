//! Low-speed parking reverse, using brake only after forward motion stops.
pub const MAX_SPEED: f32 = 2.5;
pub const ACCELERATION: f32 = 2.0;

/// Signed acceleration along the bike's forward axis. None leaves normal driving intact.
pub fn acceleration(
    speed: f32,
    throttle: bool,
    brake: bool,
    strength: f32,
    dt: f32,
) -> Option<f32> {
    if !throttle && brake && speed <= 0.15 {
        Some(
            ((-MAX_SPEED - speed) / dt.max(0.001)).clamp(-ACCELERATION, ACCELERATION)
                * strength.clamp(0.0, 1.0),
        )
    } else {
        None
    }
}

/// Heading follows signed travel. The existing speed-bounded steering demand
/// vanishes at standstill. Lateral
/// tire demand keeps its wheel-side convention; only heading uses this factor.
pub fn heading_factor(signed_speed: f32) -> f32 {
    if signed_speed < 0.0 { -1.0 } else { 1.0 }
}

/// The Assisted adapter stays lane-constrained; reverse adds a bounded heading
/// offset. Forward orientation is unchanged and zero-speed crossings are continuous.
pub fn assisted_heading(steer: f32, signed_speed: f32) -> f32 {
    steer * (-signed_speed).clamp(0.0, 1.0) * 0.28
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brake_stops_forward_motion_before_reverse_and_reverse_is_bounded() {
        assert_eq!(acceleration(10.0, false, true, 1.0, 1.0 / 60.0), None);
        assert_eq!(acceleration(0.0, true, true, 1.0, 1.0 / 60.0), None);
        for hz in [30, 60, 120] {
            let dt = 1.0 / hz as f32;
            let mut speed = 0.0;
            for _ in 0..hz * 5 {
                speed += acceleration(speed, false, true, 1.0, dt).unwrap() * dt;
            }
            assert!((speed + MAX_SPEED).abs() < 0.001);
            assert_eq!(acceleration(speed, false, false, 1.0, dt), None);
        }
    }
}

#[cfg(test)]
mod heading_tests {
    use super::*;
    #[test]
    fn signed_heading_preserves_forward_and_has_continuous_bounded_zero_crossing() {
        for steer in [-1.0f32, -0.2, 0., 0.2, 1.] {
            for speed in [-2.5f32, -0.1, -0.0001, 0., 0.0001, 0.1, 2.5, 60.] {
                let demand = (steer * 2.).clamp(-speed.abs() * 0.675, speed.abs() * 0.675);
                let signed = demand * heading_factor(speed);
                assert_eq!(signed, if speed < 0. { -demand } else { demand });
                if speed.abs() < 0.001 {
                    assert!(signed.abs() < 0.001);
                }
                assert_eq!(
                    assisted_heading(steer, speed),
                    -assisted_heading(-steer, speed)
                );
                if speed >= 0. {
                    assert_eq!(assisted_heading(steer, speed), 0.);
                }
                assert!(assisted_heading(steer, speed).abs() <= 0.28);
            }
        }
    }
}
