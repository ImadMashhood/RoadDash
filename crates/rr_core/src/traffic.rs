//! Longitudinal yielding for an occupied lane. Distances/speeds are SI.
/// Return a bounded speed target, leaving five metres for a downed person.
/// `gap` is positive in the car's own travel direction, independent of lane side.
pub fn yielding_speed(cruise: f32, gap: Option<f32>) -> f32 {
    let cruise = cruise.abs();
    gap.map_or(cruise, |gap| {
        (2.0 * 5.0 * (gap - 5.0).max(0.0)).sqrt().min(cruise)
    })
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
}
