//! Continuous gait authored in anatomical space; sole targets are fitted by the shared IK.
use super::*;
impl Library {
    pub(super) fn standing(&self) -> Pose {
        let mut p = self.key(2364);
        // Straight wrists: no bend or twist relative to each forearm.
        p.rotations[9] = Quat::IDENTITY;
        p.rotations[13] = Quat::IDENTITY;
        p
    }
    pub(super) fn locomotion(&self, phase: f32, speed: f32) -> Pose {
        let mut p = self.standing();
        let amount = (speed / 4.0).clamp(0.0, 1.0);
        if amount <= 0.0001 {
            return p;
        }
        let theta = phase * std::f32::consts::TAU;
        let standing = self.world_pose(p);
        p.root.y -= amount * (0.065 + 0.018 * (2.0 * theta).cos());
        p.root.x += amount * 0.018 * theta.sin();
        // Pelvis/shoulders counter-rotate, arms oppose the leading leg.
        for (bone, angle, axis) in [
            (1, 0.045 * theta.sin(), Vec3::Y),
            (3, -0.065 * theta.sin(), Vec3::Y),
            (7, -0.45 * theta.cos(), Vec3::X),
            (11, 0.45 * theta.cos(), Vec3::X),
            (8, -0.45 - 0.12 * theta.sin(), Vec3::X),
            (12, -0.45 + 0.12 * theta.sin(), Vec3::X),
        ] {
            let parent = self.data.parents[bone] as usize;
            let q = standing[parent].rotation;
            p.rotations[bone] =
                (q.inverse() * Quat::from_axis_angle(axis, angle * amount) * q * p.rotations[bone])
                    .normalize();
        }
        for limb in &self.data.limbs {
            if limb.bit < 2 {
                continue;
            }
            let i = (limb.bit - 2) as usize;
            let t = (phase + i as f32 * 0.5).rem_euclid(1.0);
            let (forward, lift) = if t < 0.5 {
                (0.42 - 1.68 * t, 0.0)
            } else {
                let u = (t - 0.5) * 2.0;
                (
                    -0.42 + 0.84 * motion::smoothstep(u),
                    0.19 * (u * std::f32::consts::PI).sin(),
                )
            };
            let sole = standing[limb.end].transform_point(Vec3::from_array(limb.offset));
            p.planted[i] = Some(sole + Vec3::new(0.0, lift, forward) * amount);
            p.normals[i] = Vec3::Y;
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gait_is_continuous_mirrored_and_stops_with_neutral_palms() {
        let lib = Library::default();
        let idle = lib.standing();
        assert!(idle.rotations[9].angle_between(Quat::IDENTITY) < 0.001);
        assert!(idle.rotations[13].angle_between(Quat::IDENTITY) < 0.001);
        let a = lib.locomotion(0.0, 6.0);
        let b = lib.locomotion(0.5, 6.0);
        assert!((a.planted[0].unwrap().z - b.planted[1].unwrap().z).abs() < 0.01);
        let mut previous = lib.world_pose(lib.locomotion(0.0, 6.0));
        for tick in 1..=240 {
            let p = lib.locomotion(tick as f32 / 240.0, 6.0);
            let w = lib.world_pose(p);
            for i in 0..20 {
                assert!(w[i].translation.is_finite() && w[i].rotation.is_finite());
                assert!(
                    w[i].translation.distance(previous[i].translation) < 0.05,
                    "gait pop bone={i} phase={} distance={}",
                    tick as f32 / 240.0,
                    w[i].translation.distance(previous[i].translation)
                );
            }
            for l in &lib.data.limbs {
                assert!(
                    (w[l.upper].translation.distance(w[l.lower].translation)
                        - Vec3::from_array(lib.data.offsets[l.lower]).length())
                    .abs()
                        < 1e-4
                );
            }
            previous = w;
        }
        let stop = lib.locomotion(0.42, 0.0);
        assert_eq!(stop.root, idle.root);
        assert!(stop.planted.into_iter().all(|x| x.is_none()));
    }
}
