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
/// Vertical-only arcade lift on a standing civilian or recovering rider.
/// No launch for parking-speed touches; cap the unobstructed rise below 0.85 m.
pub fn ride_over_velocity(closing: f32) -> Option<f32> {
    (closing.is_finite() && closing >= 3.0).then(|| (2.8 + closing * 0.035).min(4.0))
}
/// Downed bodies and wrecks get twice the standing target's hop height
/// in an unobstructed arc (v squared / 2g).
pub fn downed_ride_over_velocity(closing: f32) -> Option<f32> {
    ride_over_velocity(closing).map(|v| v * std::f32::consts::SQRT_2)
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
/// Low-speed traffic touches remain solid contacts, not scripted rider ejections.
/// Vehicle impact threshold18km/h; wall and upright-rider thresholds stay separate.
pub fn vehicle_wipeout(closing: f32) -> bool {
    closing.is_finite() && closing >= 5.
}
/// Measured approach, independent of crash policy and retained ground speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleApproach {
    Rear,
    Oncoming,
    Side,
    Grazing,
    Stopped,
}

/// The normal points from the rider into the contacted surface. SI world
/// velocities must be sampled before solver response, never from scalar speed.
pub fn vehicle_approach(
    incoming: [f32; 3],
    object: [f32; 3],
    normal: [f32; 3],
) -> (VehicleApproach, f32) {
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[2] * b[2];
    let relative = [incoming[0] - object[0], 0., incoming[2] - object[2]];
    let closing = dot(relative, normal).max(0.);
    let relative_speed = dot(relative, relative).sqrt();
    let own_speed = dot(incoming, incoming).sqrt();
    let other_speed = dot(object, object).sqrt();
    let kind = if closing < relative_speed * 0.25 || closing <= 0. {
        VehicleApproach::Grazing
    } else if dot(incoming, normal).abs() < own_speed * 0.7 {
        VehicleApproach::Side
    } else if other_speed < 0.25 {
        VehicleApproach::Stopped
    } else {
        let alignment = dot(incoming, object) / (own_speed * other_speed).max(0.001);
        if alignment > 0.7 {
            VehicleApproach::Rear
        } else if alignment < -0.7 {
            VehicleApproach::Oncoming
        } else {
            VehicleApproach::Side
        }
    };
    (kind, closing)
}

/// Once balance has been lost, a substantial solid contact interrupts the
/// mounted sequence. This is not a new crash threshold for ordinary riding.
pub fn interrupts_stagger(closing: f32) -> bool {
    closing.is_finite() && closing >= 1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vehicle_touches_require_meaningful_closing_speed() {
        for v in [0., 0.5 / 3.6, 2., -8., f32::NAN] {
            assert!(!vehicle_wipeout(v));
        }
        assert!(vehicle_wipeout(5.));
        assert!(vehicle_wipeout(30.));
    }
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

#[cfg(test)]
mod measured_vehicle_tests {
    use super::*;
    #[test]
    fn world_velocity_closing_and_approach_are_distinct() {
        for (other, normal, expected, closing_kmh) in [
            (
                [0., 0., 60. / 3.6],
                [0., 0., 1.],
                VehicleApproach::Rear,
                40.,
            ),
            (
                [0., 0., -60. / 3.6],
                [0., 0., 1.],
                VehicleApproach::Oncoming,
                160.,
            ),
            ([0.; 3], [0., 0., 1.], VehicleApproach::Stopped, 100.),
            ([-10., 0., 0.], [1., 0., 0.], VehicleApproach::Side, 36.),
            ([0.; 3], [1., 0., 0.], VehicleApproach::Grazing, 0.),
        ] {
            let (kind, closing) = vehicle_approach([0., 0., 100. / 3.6], other, normal);
            assert_eq!(kind, expected);
            assert!((closing * 3.6 - closing_kmh).abs() < 0.001);
        }
    }
    #[test]
    fn parallel_side_door_contact_is_side_even_with_aligned_world_velocities() {
        let (kind, closing) = vehicle_approach([5., 0., 30.], [0., 0., 30.], [1., 0., 0.]);
        assert_eq!(kind, VehicleApproach::Side);
        assert_eq!(closing, 5.);
    }
    #[test]
    fn loss_of_balance_contact_gate_rejects_rest_and_invalid_samples() {
        for closing in [0., 0.99, -2., f32::NAN, f32::INFINITY] {
            assert!(!interrupts_stagger(closing));
        }
        assert!(interrupts_stagger(1.));
        assert!(!vehicle_wipeout(1.));
        assert!(!wipeout(Class::Tall, 1.));
    }
}

#[cfg(test)]
mod polish_tests {
    use super::*;
    #[test]
    fn downed_hop_doubles_height_with_a_bounded_vertical_response() {
        for speed in [3.0, 8.0, 20.0, 60.0] {
            let upright = ride_over_velocity(speed).unwrap();
            let downed = downed_ride_over_velocity(speed).unwrap();
            assert!((downed.powi(2) / upright.powi(2) - 2.).abs() < 1e-5);
        }
        assert!(downed_ride_over_velocity(2.9).is_none());
    }
}

/// Classify an actual upright-bike contact using signed height and relative descent.
/// The return value identifies whether the second rider is the descending upper one.
pub fn rider_landing(height: f32, first_vertical: f32, second_vertical: f32) -> Option<bool> {
    let second_higher = height > 0.;
    let (upper, lower) = if second_higher {
        (second_vertical, first_vertical)
    } else {
        (first_vertical, second_vertical)
    };
    (height.abs() >= 0.42 && upper < -0.8 && upper - lower <= -1.3).then_some(second_higher)
}

#[cfg(test)]
mod rider_landing_tests {
    use super::rider_landing;
    #[test]
    fn upper_descending_contact_is_distinct_from_side_and_rising_contact() {
        assert_eq!(rider_landing(0.56, 0., -3.), Some(true));
        assert_eq!(rider_landing(-0.9, -3., 0.), Some(false));
        assert_eq!(rider_landing(0.25, 0., -3.), None);
        assert_eq!(rider_landing(0.9, 0., 3.), None);
        assert_eq!(rider_landing(0.9, 3., 0.), None);
    }
}
