//! Longitudinal yielding for an occupied lane. Distances/speeds are SI.
/// Return a bounded speed target, leaving five metres for a downed person.
/// `gap` is positive in the car's own travel direction, independent of lane side.
pub fn yielding_speed(cruise: f32, gap: Option<f32>) -> f32 {
    let cruise = cruise.abs();
    gap.map_or(cruise, |gap| {
        (2.0 * 5.0 * (gap - 5.0).max(0.0)).sqrt().min(cruise)
    })
}
/// Cargo/farm traffic has its own dimensions and cruise, without changing bike physics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeavyKind {
    Semi,
    Tractor,
}
impl HeavyKind {
    pub fn cruise(self) -> f32 {
        match self {
            Self::Semi => 12.,
            Self::Tractor => 6.,
        }
    }
    pub fn wheels(self) -> usize {
        match self {
            Self::Semi => 18,
            Self::Tractor => 4,
        }
    }
}
/// Edge-to-edge following, rather than treating a semitrailer as a four-metre car.
pub fn following_speed(cruise: f32, gap: f32, front: f32, leader_rear: f32, leader: f32) -> f32 {
    let free = (gap - front - leader_rear - 5.).max(0.);
    (leader.abs().powi(2) + 2. * 4. * free)
        .sqrt()
        .min(cruise.abs())
}
/// Two metres/second² lateral acceleration envelope for ordinary road traffic.
pub fn curve_speed(cruise: f32, turn_radians: f32, lookahead_m: f32) -> f32 {
    (2. * lookahead_m.max(1.) / turn_radians.abs().max(0.001))
        .sqrt()
        .min(cruise.abs())
}
/// A person alongside the trailer is still inside the vehicle's occupied lane.
pub fn occupied_gap(gap: f32, front: f32, rear: f32) -> Option<f32> {
    (-rear..80.).contains(&gap).then_some((gap - front).max(0.))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cars_slow_queue_and_resume_without_driving_over_the_person() {
        assert_eq!(yielding_speed(18.0, None), 18.0);
        assert_eq!(yielding_speed(-13.0, Some(4.0)), 0.0);
        assert_eq!(yielding_speed(18.0, Some(5.0)), 0.0);
        assert!(yielding_speed(18.0, Some(6.0)) < 4.0);
        assert!(yielding_speed(18.0, Some(20.0)) < 18.0);
        assert_eq!(yielding_speed(18.0, Some(80.0)), 18.0);
    }
    #[test]
    fn long_vehicle_following_leaves_space_behind_the_trailer() {
        assert_eq!(following_speed(18., 16., 2., 12.5, 0.), 0.);
        assert!(following_speed(18., 25., 2., 12.5, 0.) < following_speed(18., 25., 2., 2., 0.));
        assert_eq!(HeavyKind::Semi.wheels(), 18);
        assert_eq!(HeavyKind::Tractor.wheels(), 4);
        assert!(HeavyKind::Tractor.cruise() < HeavyKind::Semi.cruise());
        assert_eq!(curve_speed(12., 0., 12.), 12.);
        assert!(curve_speed(12., 0.8, 12.) < 6.);
        assert_eq!(curve_speed(12., -0.8, 12.), curve_speed(12., 0.8, 12.));
        assert_eq!(occupied_gap(-8., 3.5, 13.4), Some(0.));
        assert_eq!(occupied_gap(2., 3.5, 13.4), Some(0.));
        assert_eq!(occupied_gap(-15., 3.5, 13.4), None);
        assert_eq!(occupied_gap(20., 3.5, 13.4), Some(16.5));
    }
}
