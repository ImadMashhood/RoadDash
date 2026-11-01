//! Explicit arcade collision policy. Units are metres/second, not displayed km/h.
//! This speed-loss rule is a design choice, not a two-body Newtonian solution.
pub fn retained_speed(rider_speed: f32, object_speed: f32) -> f32 {
    if !rider_speed.is_finite() || !object_speed.is_finite() {
        return 0.0;
    }
    (rider_speed.max(0.0) - object_speed.abs()).max(0.0)
}

/// Stationary civilian shares the incoming velocity without creating kinetic energy.
pub fn shared_speed_fraction(source_mass: f32, target_mass: f32) -> f32 {
    if !source_mass.is_finite() || !target_mass.is_finite() || source_mass <= 0.0 {
        return 0.0;
    }
    source_mass / (source_mass + target_mass.max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requested_120_40_cases_retain_80_without_reversing() {
        for other in [-40.0_f32, 40.0] {
            assert!((retained_speed(120.0 / 3.6, other / 3.6) * 3.6 - 80.0).abs() < 0.001);
        }
        assert_eq!(retained_speed(10.0, 20.0), 0.0);
        assert_eq!(retained_speed(10.0, 0.0), 10.0);
        assert_eq!(retained_speed(f32::NAN, 0.0), 0.0);
    }
    #[test]
    fn civilian_transfer_dissipates_instead_of_duplicating_energy() {
        for source in [200.0, 300.0] {
            for target in [56.5, 363.0] {
                let ratio = shared_speed_fraction(source, target);
                assert!((source + target) * ratio * ratio <= source);
            }
        }
    }
}
