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
