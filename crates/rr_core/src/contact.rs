//! Original-style contact classes, independent of physics/rendering.
//! Geometry/SI thresholds are explicit modern adapters, not native position units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Low,
    Bike,
    Tall,
    Glance,
}
/// Matches the original low -> containment -> bike/tall routing order.
pub fn classify(low: bool, contained: bool, tall: bool) -> Class {
    if low {
        Class::Low
    } else if !contained {
        Class::Glance
    } else if tall {
        Class::Tall
    } else {
        Class::Bike
    }
}
/// Conservative modern low-obstacle calibration. Fallen rider/bike state is
/// classified separately; only actually low geometry is suspension support.
pub const LOW_HEIGHT_M: f32 = 0.8;
/// An arcade ride-over hop, explicitly requested for rear/downed-bike contacts.
/// No launch for parking-speed touches; cap the unobstructed rise below 0.85 m.
pub fn ride_over_velocity(closing: f32) -> Option<f32> {
    (closing.is_finite() && closing >= 3.0).then(|| (2.8 + closing * 0.035).min(4.0))
}
/// User-selected upright rear-hit knockoff threshold (SI units).
pub const REAR_KNOCKOFF_MPS: f32 = 60.0 / 3.6;
/// Perfectly inelastic normal-speed exchange: preserves pair momentum, dissipates energy.
pub fn shared_normal_speed(a: f32, b: f32, mass_a: f32, mass_b: f32) -> f32 {
    (a * mass_a + b * mass_b) / (mass_a + mass_b)
}
pub fn low_geometry(bottom: f32, top: f32, road: f32) -> bool {
    bottom.is_finite()
        && top.is_finite()
        && road.is_finite()
        && top >= bottom
        && top - bottom <= LOW_HEIGHT_M
        && top <= road + LOW_HEIGHT_M
        && bottom >= road - 0.25
}
/// Keep the accepted 50 km/h direct impact gate; low bodies and side glances
/// never enter that gate. Exact native damage/6001-unit parity stays separate.
pub fn wipeout(class: Class, closing: f32) -> bool {
    matches!(class, Class::Bike | Class::Tall) && closing.is_finite() && closing >= 50.0 / 3.6
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ride_over_is_bounded_and_requires_closing_motion() {
        for v in [f32::NAN, -20.0, 0.0, 2.9] {
            assert!(ride_over_velocity(v).is_none());
        }
        for v in [3.0, 15.0, 75.0, 200.0] {
            let up = ride_over_velocity(v).unwrap();
            assert!((0.4..0.85).contains(&(up * up / (2.0 * 9.81))));
        }
    }
    #[test]
    fn low_precedes_containment_and_cannot_force_a_wipeout() {
        for contained in [false, true] {
            for tall in [false, true] {
                for speed in [0.0, 1.0, 15.0, 60.0] {
                    let c = classify(true, contained, tall);
                    assert_eq!(c, Class::Low);
                    assert!(!wipeout(c, speed));
                }
            }
        }
    }
    #[test]
    fn glances_push_and_direct_hits_use_closing_speed() {
        assert_eq!(classify(false, false, true), Class::Glance);
        assert!(!wipeout(Class::Glance, 60.0));
        assert!(!wipeout(Class::Bike, 0.0));
        assert!(!wipeout(Class::Tall, 3.0));
        assert!(wipeout(Class::Bike, 50.0 / 3.6));
        assert!(!wipeout(Class::Tall, -30.0));
    }
    #[test]
    fn airborne_or_tall_shapes_are_not_low_support() {
        assert!(low_geometry(0.0, 0.4, 0.0));
        assert!(!low_geometry(2.0, 2.4, 0.0));
        assert!(!low_geometry(0.0, 1.6, 0.0));
        assert!(!low_geometry(0.0, f32::NAN, 0.0));
    }
}

#[cfg(test)]
mod rear_tests {
    use super::*;
    #[test]
    fn rear_exchange_conserves_momentum_and_dissipates_energy() {
        for (a, b, ma, mb) in [
            (30.0, 20.0, 250.0, 300.0),
            (20.0, 20.0, 280.0, 280.0),
            (60.0, 5.0, 250.0, 270.0),
        ] {
            let v = shared_normal_speed(a, b, ma, mb);
            assert!(((ma + mb) * v - (ma * a + mb * b)).abs() < 0.002);
            assert!((ma + mb) * v * v <= ma * a * a + mb * b * b + 0.002);
            assert!((b..=a).contains(&v));
        }
        assert!((REAR_KNOCKOFF_MPS * 3.6 - 60.0).abs() < 1e-4);
    }
}
