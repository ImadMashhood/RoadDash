//! Offline NPC interest rules. Future multiplayer authority must own this decision.
pub const OFFSCREEN_RECOVERY_TICKS: u16 = 5 * crate::SIMULATION_HZ as u16;
/// Abstract only distant rear crashes that are also behind the camera plane.
pub fn defer_rear_crash(
    player: [f32; 3],
    forward: [f32; 3],
    camera: [f32; 3],
    view: [f32; 3],
    point: [f32; 3],
) -> bool {
    let relative = std::array::from_fn::<_, 3, _>(|i| point[i] - player[i]);
    let dot = |a: [f32; 3], b: [f32; 3]| a.into_iter().zip(b).map(|(a, b)| a * b).sum::<f32>();
    dot(relative, forward) < -15.0
        && dot(relative, relative) > 25.0 * 25.0
        && dot(std::array::from_fn(|i| point[i] - camera[i]), view) < -3.0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rear_crash_interest_has_strict_near_and_camera_guards() {
        let origin = [0.0; 3];
        let forward = [0.0, 0.0, 1.0];
        let camera = [0.0, 3.0, -6.0];
        assert!(defer_rear_crash(
            origin,
            forward,
            camera,
            forward,
            [0.0, 0.0, -60.0]
        ));
        assert!(!defer_rear_crash(
            origin,
            forward,
            camera,
            forward,
            [0.0, 0.0, -25.0]
        ));
        assert!(!defer_rear_crash(
            origin,
            forward,
            camera,
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -60.0]
        ));
        assert!(!defer_rear_crash(
            origin,
            forward,
            camera,
            forward,
            [0.0, 0.0, 60.0]
        ));
        assert_eq!(OFFSCREEN_RECOVERY_TICKS, 300);
    }
}
