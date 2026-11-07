//! Opt-in bridge between original integer contact rules and modern 3D contacts.
//! It does not replace Rapier, animation, recovery, or the accepted passive rig.
use bevy::prelude::*;
use rr_core::original_crash::{self as rules, Contact, ContactClass, Motion, Response};

#[derive(Resource, Default, Clone, Copy)]
pub struct OriginalCrashes(pub bool);

/// Comparison condition, deliberately independent of existing three-hit combat.
/// These starting values are adapter tuning, not per-racer retail statistics.
pub fn initial_condition() -> rules::Condition {
    rules::Condition {
        base_stamina: 2500,
        max_stamina: 2500,
        stamina: 2500,
        bike: 1000,
    }
}
fn native(value: f32, scale: f32) -> i32 {
    if value.is_finite() {
        (value / scale).round() as i32
    } else {
        0
    }
}
pub fn motion(velocity: Vec3, frame: Quat) -> Motion {
    let local = frame.inverse() * velocity;
    Motion {
        forward: native(local.z, rules::FORWARD_MPS_PER_UNIT),
        lateral: native(local.x, rules::VERTICAL_MPS_PER_UNIT),
        vertical: native(local.y, rules::VERTICAL_MPS_PER_UNIT),
    }
}
pub fn velocity(motion: Motion, frame: Quat) -> Vec3 {
    frame
        * Vec3::new(
            motion.lateral as f32 * rules::VERTICAL_MPS_PER_UNIT,
            motion.vertical as f32 * rules::VERTICAL_MPS_PER_UNIT,
            motion.forward as f32 * rules::FORWARD_MPS_PER_UNIT,
        )
}

/// Geometry adaptation: project into an upright rider frame. Rapier supplies
/// contact detection; native route boxes are not transplanted into the 3D map.
/// The first longitudinal actor gets the mass-weighted delta, the second its
/// negative; reversing pair order is observably different in the original.
#[allow(clippy::too_many_arguments)]
pub fn plan(
    incoming: Vec3,
    partner: Vec3,
    rotation: Quat,
    offset: Vec3,
    car: bool,
    condition: rules::Condition,
) -> (Response, Quat) {
    let (contact, frame) = contact_plan(incoming, partner, rotation, offset, car, condition);
    (rules::contact(contact), frame)
}

#[allow(clippy::too_many_arguments)]
pub fn contact_plan(
    incoming: Vec3,
    partner: Vec3,
    rotation: Quat,
    offset: Vec3,
    car: bool,
    condition: rules::Condition,
) -> (Contact, Quat) {
    let forward = (rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
    let frame = Quat::from_rotation_y(forward.x.atan2(forward.z));
    let a = motion(incoming, frame);
    let b = motion(partner, frame);
    let local_offset = frame.inverse() * offset;
    let partner_mass = if car { 1800 } else { 256 };
    let rider_first = local_offset.z >= 0.0;
    let impulse = |v1, v2| {
        if rider_first {
            rules::separation_delta(v1, v2, 256, partner_mass, false).unwrap_or(0)
        } else {
            rules::separation_delta(v2, v1, partner_mass, 256, false)
                .unwrap_or(0)
                .wrapping_neg()
        }
    };
    let aligned = local_offset.z.abs() > local_offset.x.abs();
    let raw_delta = Motion {
        forward: if aligned {
            impulse(a.forward, b.forward)
        } else {
            0
        },
        lateral: if aligned {
            0
        } else {
            impulse(a.lateral, b.lateral)
        },
        vertical: 0,
    };
    let half_width = if car { 1.05 } else { 0.45 };
    let contained = aligned && local_offset.x.abs() + 0.225 < half_width;
    let class = if !contained {
        ContactClass::Glance
    } else if car {
        ContactClass::Tall
    } else {
        ContactClass::Bike
    };
    (
        Contact {
            incoming: a,
            raw_delta,
            // Retail 0x404ed9 subtracts other+0x108 (SIGNED forward motion).
            // The reconstruction's contact_car instead uses car.speed; do not
            // inherit that scalar/directional discrepancy for oncoming traffic.
            partner_gap_speed: b.forward,
            elasticity_q8: 192,
            class,
            car,
            partner_on_left: local_offset.x < 0.0,
            condition,
        },
        frame,
    )
}

pub fn report() -> serde_json::Value {
    let scenarios: Vec<_> = [
        ("rear_120_40", 40.0),
        ("oncoming_120_40", -40.0),
        ("stopped_120_0", 0.0),
        ("matched_40_40", 40.0),
    ]
    .into_iter()
    .map(|(name, other)| {
        let speed = if name == "matched_40_40" { 40.0 } else { 120.0 };
        let incoming = Vec3::Z * (speed / 3.6);
        let (response, frame) = plan(
            incoming,
            Vec3::Z * (other / 3.6),
            Quat::IDENTITY,
            Vec3::Z * 2.0,
            true,
            initial_condition(),
        );
        serde_json::json!({"name":name,"input_kmh":speed,"other_kmh":other,
            "native":response,"bike_mps":velocity(response.bike,frame).to_array(),
            "rider_mps":response.rider.map(|m|velocity(m,frame).to_array())})
    })
    .collect();
    let drag_cases: Vec<_> = [-12000,-10,0,10,12000,30000].into_iter().flat_map(|speed| {
        [false,true].into_iter().map(move |grounded| {
            let (next,acceleration)=rules::drag_step(speed,1,grounded,0,130,70,0,0);
            serde_json::json!({"speed":speed,"grounded":grounded,"next":next,"acceleration":acceleration})
        })
    }).collect();
    let scale_cases: Vec<_> = [8, -9]
        .into_iter()
        .flat_map(|value| {
            [2, 3].into_iter().map(move |reason| {
                let mut c = initial_condition();
                c.damage(value, value, reason);
                serde_json::json!({"value":value,"reason":reason,"scaled":1000-c.bike})
            })
        })
        .collect();
    serde_json::json!({"reference_commit":"aa9cb6d3181e20736af71022cd4d6aac0e2788d1",
        "scope":"original integer contact/entry math; modern geometry and SI adaptation; not complete executable parity",
        "forward_mps_per_unit":rules::FORWARD_MPS_PER_UNIT,
        "vertical_mps_per_unit":rules::VERTICAL_MPS_PER_UNIT,
        "comparison_condition":initial_condition(), "scenarios":scenarios,
        "native_verification":{"drag_cases":drag_cases,"scale_cases":scale_cases,
            "damage_pair":rules::Entry::Damage.impulses(),"punch_pair":rules::Entry::Punch.impulses(),
            "speed_gap_pair":rules::Entry::SpeedGap.impulses()}})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opposing_traffic_uses_signed_separation_not_absolute_subtraction() {
        let (rear, _) = plan(
            Vec3::Z * 33.333,
            Vec3::Z * 11.111,
            Quat::IDENTITY,
            Vec3::Z * 2.0,
            true,
            initial_condition(),
        );
        let (head, _) = plan(
            Vec3::Z * 33.333,
            -Vec3::Z * 11.111,
            Quat::IDENTITY,
            Vec3::Z * 2.0,
            true,
            initial_condition(),
        );
        assert!(head.scaled_delta.forward < rear.scaled_delta.forward);
        assert!(head.rider.is_some());
        assert_eq!(head.entry, Some(rules::Entry::SpeedGap));
        assert_eq!(head.rider.unwrap().forward, head.bike.forward);
        let mut depleted = initial_condition();
        depleted.stamina = 1;
        let (damage, _) = plan(
            Vec3::Z * 33.333,
            -Vec3::Z * 11.111,
            Quat::IDENTITY,
            Vec3::Z * 2.0,
            true,
            depleted,
        );
        assert_eq!(damage.entry, Some(rules::Entry::Damage));
        assert!(damage.rider.unwrap().forward > damage.bike.forward);
    }
    #[test]
    fn matched_traffic_has_no_added_speed_and_frame_rotates_consistently() {
        let frame = Quat::from_rotation_y(0.6);
        let v = frame * Vec3::Z * 12.0;
        let (r, q) = plan(
            v,
            v,
            frame,
            frame * Vec3::Z * 2.0,
            true,
            initial_condition(),
        );
        assert!(r.entry.is_none());
        assert!((velocity(r.bike, q) - v).length() < 0.01);
    }
    #[test]
    fn forward_and_vertical_calibrations_are_separate() {
        let v = Vec3::new(1.0, 3.0, 33.0);
        assert!((velocity(motion(v, Quat::IDENTITY), Quat::IDENTITY) - v).length() < 0.01);
        assert_ne!(
            motion(Vec3::ONE, Quat::IDENTITY).forward,
            motion(Vec3::ONE, Quat::IDENTITY).vertical
        );
    }
}
