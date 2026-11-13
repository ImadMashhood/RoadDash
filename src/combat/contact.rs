use super::*;
use bevy_rapier3d::parry::{
    query::intersection_test,
    shape::{Capsule, Cuboid, Shape},
};
use bevy_rapier3d::rapier::math::{Pose3, Rot3};

pub(super) struct Sweep {
    samples: [(f32, Transform, [Vec3; 3]); 5],
    radius: f32,
    hand: bool,
}

pub(super) fn previous(history: &crate::runtime::PoseHistory, pose: Transform) -> Transform {
    if history
        .previous
        .translation
        .distance_squared(pose.translation)
        <= 25.
    {
        history.previous
    } else {
        pose
    }
}

fn lerp(a: Transform, b: Transform, t: f32) -> Transform {
    Transform::from_translation(a.translation.lerp(b.translation, t))
        .with_rotation(a.rotation.slerp(b.rotation, t))
}

fn iso(t: Transform) -> Pose3 {
    let q = t.rotation;
    Pose3::from_parts(
        t.translation.to_array().into(),
        Rot3::from_xyzw(q.x, q.y, q.z, q.w),
    )
}

impl Sweep {
    pub(super) fn new(
        library: &crate::animation::Library,
        clock: rr_core::motion::StrikeClock,
        lean: f32,
        before: Transform,
        now: Transform,
        dt: f32,
    ) -> Self {
        let first = match (clock.action == rr_core::AttackSide::Kick, clock.side > 0.) {
            (true, true) => 17,
            (true, false) => 14,
            (false, true) => 11,
            (false, false) => 7,
        };
        let (start, end) = clock.contact_window();
        let samples = std::array::from_fn(|i| {
            let begin = (clock.age - dt).max(start);
            let finish = clock.age.min(end);
            let age = begin + (finish - begin) * i as f32 / 4.;
            let fraction = ((age - (clock.age - dt)) / dt.max(f32::EPSILON)).clamp(0., 1.);
            let frames = library.strike_frames(clock, age, lean);
            let root = lerp(before, now, fraction);
            let mut points =
                std::array::from_fn(|j| root.transform_point(frames[first + j].translation));
            if let Some(weapon) = clock.weapon {
                let direction = (points[2] - points[1]).normalize_or_zero();
                points[2] += direction * (weapon.lateral_reach - 1.55).clamp(0., 1.15);
            }
            (fraction, root, points)
        });
        Self {
            samples,
            radius: if first >= 14 { 0.19 } else { 0.15 },
            hand: first < 14 && clock.weapon.is_none(),
        }
    }

    pub(super) fn hit(
        &self,
        before: Transform,
        now: Transform,
        body: &[Transform; 20],
    ) -> Option<(Vec3, bool)> {
        let bike = Cuboid::new(Vec3::new(0.38, 0.59, 1.12).to_array().into());
        let body_parts = [
            (1, 3, 0.29),
            (3, 5, 0.20),
            (7, 8, 0.12),
            (8, 9, 0.12),
            (11, 12, 0.12),
            (12, 13, 0.12),
            (14, 15, 0.15),
            (15, 16, 0.14),
            (17, 18, 0.15),
            (18, 19, 0.14),
        ]
        .map(|(a, b, r)| {
            Capsule::new(
                body[a].translation.to_array().into(),
                body[b].translation.to_array().into(),
                r,
            )
        });
        for (fraction, root, points) in &self.samples {
            let target = lerp(before, now, *fraction);
            if root.translation.distance_squared(target.translation) > 30. {
                continue;
            }
            let target_iso = iso(target);
            let bike_iso = iso(target.mul_transform(Transform::from_xyz(0., 0.64, 0.)));
            for (index, pair) in points.windows(2).enumerate().rev() {
                let distal = if self.hand && index == 1 {
                    // The rig joint sits at the wrist. Include the armored
                    // glove beyond that joint in the contact shape.
                    pair[1] + (pair[1] - pair[0]).normalize_or_zero() * 0.16
                } else {
                    pair[1]
                };
                let limb = Capsule::new(
                    pair[0].to_array().into(),
                    distal.to_array().into(),
                    self.radius,
                );
                let touches = |pose: &Pose3, shape: &dyn Shape| {
                    intersection_test(&Pose3::IDENTITY, &limb, pose, shape)
                        .is_ok_and(|hit| hit.intersecting)
                };
                let point = pair[0].lerp(pair[1], 0.5);
                if body_parts.iter().any(|part| touches(&target_iso, part)) {
                    return Some((point, false));
                }
                if touches(&bike_iso, &bike) {
                    return Some((point, true));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bike_can_move_into_an_extended_kick_after_the_authored_contact_key() {
        let library = crate::animation::Library::default();
        let dt = 1. / 60.;
        let mut clock = rr_core::motion::StrikeClock::new(rr_core::AttackSide::Kick, 1., 0.24);
        clock.age = clock.contact + 3. * dt;
        let pose = Transform::IDENTITY;
        let sweep = Sweep::new(&library, clock, 0., pose, pose, dt);
        let foot = sweep.samples[4].2[2];
        let bike_at = Transform::from_translation(foot - Vec3::Y * 0.64);
        let out = Transform::from_translation(bike_at.translation + Vec3::Z * 4.);
        let no_body = [Transform::from_xyz(0., 30., 0.); 20];
        assert!(sweep.hit(out, out, &no_body).is_none());
        let (_, bike_only) = sweep.hit(out, bike_at, &no_body).unwrap();
        assert!(bike_only);
        let overlapping = [Transform::from_xyz(0., 0.64, 0.); 20];
        assert!(!sweep.hit(bike_at, bike_at, &overlapping).unwrap().1);
    }
}
