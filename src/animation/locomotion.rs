//! Distance-matched visual gait. Physics owns displacement; these states only pose skins.
use super::*;
#[derive(Default)]
pub(super) struct Gait {
    pub phase: f32,
    pub speed: f32,
    distance: Option<f32>,
    pub direction: Vec3,
}
impl Gait {
    pub fn advance(&mut self, distance: f32, velocity: Vec3, body: Quat, dt: f32) {
        let speed = velocity.with_y(0.).length();
        self.speed = motion::approach(
            self.speed,
            speed,
            dt,
            if speed > self.speed { 0.08 } else { 0.12 },
        );
        let traveled = self
            .distance
            .map_or(0., |last| (distance - last).clamp(0., 0.5));
        self.distance = Some(distance);
        self.phase = (self.phase + traveled / cycle(self.speed)).rem_euclid(1.);
        let local = (body.inverse() * velocity).with_y(0.);
        let target = if local.length_squared() > 0.01 {
            local.normalize()
        } else {
            Vec3::Z
        };
        let previous = self.direction.x.atan2(self.direction.z);
        let desired = target.x.atan2(target.z);
        let delta = (desired - previous).sin().atan2((desired - previous).cos());
        let angle = previous + delta * (1. - (-dt / 0.08).exp());
        self.direction = Vec3::new(angle.sin(), 0., angle.cos());
    }
}
fn running(speed: f32) -> f32 {
    motion::smoothstep(((speed - 2.3) / 2.2).clamp(0., 1.))
}
fn amount(speed: f32) -> f32 {
    (speed / 1.5).clamp(0., 1.)
}
pub(super) fn cycle(speed: f32) -> f32 {
    (1.5 + 1.25 * running(speed)) * amount(speed).max(0.15)
}
pub(super) fn stance(speed: f32) -> f32 {
    0.825 / (1.5 + 1.25 * running(speed))
}
pub(super) fn contact(phase: f32, speed: f32, side: usize) -> bool {
    speed < 0.15 || (phase + side as f32 * 0.5).rem_euclid(1.) < stance(speed)
}
fn foot(phase: f32, speed: f32) -> (f32, f32) {
    let contact = stance(speed);
    let length = cycle(speed);
    let reach = length * contact * 0.5;
    if phase < contact {
        (reach - length * phase, 0.)
    } else {
        let u = (phase - contact) / (1. - contact);
        // Hermite endpoint tangents match the stance velocity at lift/landing.
        let tangent = -length * (1. - contact);
        let u2 = u * u;
        let u3 = u2 * u;
        let forward = (2. * u3 - 3. * u2 + 1.) * (-reach)
            + (u3 - 2. * u2 + u) * tangent
            + (-2. * u3 + 3. * u2) * reach
            + (u3 - u2) * tangent;
        let lift = (0.08 + 0.14 * running(speed))
            * amount(speed)
            * (std::f32::consts::PI * u).sin().powi(2);
        (forward, lift)
    }
}
impl Library {
    pub(super) fn standing(&self) -> Pose {
        let mut p = self.key(2364);
        // The authored standing sole sits 9.27cm above its anchor. Seat the sole
        // at the same 1.5cm support clearance as ground IK, without moving bodies.
        p.root.y -= 0.0777;
        p.rotations[9] = Quat::IDENTITY;
        p.rotations[13] = Quat::IDENTITY;
        p
    }
    #[cfg(test)]
    pub(crate) fn locomotion(&self, phase: f32, speed: f32) -> Pose {
        self.directional_locomotion(phase, speed, Vec3::Z)
    }
    pub(super) fn directional_locomotion(&self, phase: f32, speed: f32, direction: Vec3) -> Pose {
        let mut p = self.standing();
        let weight = amount(speed);
        if weight <= 0.0001 {
            return p;
        }
        let theta = phase * std::f32::consts::TAU;
        let run = running(speed);
        let standing = self.world_pose(p);
        let bob = (theta * 2. - stance(speed) * std::f32::consts::TAU).cos();
        // Walk over the supporting leg instead of remaining in a fixed squat.
        // Running compresses during support and rises through its aerial phase.
        let walk_drop = 0.045 - 0.030 * bob;
        let run_drop = 0.095 + 0.025 * bob;
        p.root.y -= weight * walk_drop.lerp(run_drop, run);
        p.hands = [[0., 0.4 * run * weight]; 2];
        p.root.x += weight * 0.012 * theta.sin();
        let yaw = direction.x.atan2(direction.z).clamp(-0.55, 0.55);
        // A small forward trunk inclination and bent opposing arms distinguish a
        // run from accelerated walking. Neck counter-pitch keeps the gaze level.
        for (bone, angle, axis) in [
            (1, 0.045 * theta.sin() + yaw * 0.25, Vec3::Y),
            (3, -0.065 * theta.sin() - yaw * 0.10, Vec3::Y),
            (3, 0.07 + 0.12 * run, Vec3::X),
            (4, -0.05 - 0.10 * run, Vec3::X),
            (7, (0.30 + 0.38 * run) * theta.cos(), Vec3::X),
            (11, -(0.30 + 0.38 * run) * theta.cos(), Vec3::X),
            (8, -0.25 - 0.85 * run - 0.12 * theta.sin(), Vec3::X),
            (12, -0.25 - 0.85 * run + 0.12 * theta.sin(), Vec3::X),
        ] {
            let parent = self.data.parents[bone] as usize;
            let q = standing[parent].rotation;
            p.rotations[bone] =
                (q.inverse() * Quat::from_axis_angle(axis, angle * weight) * q * p.rotations[bone])
                    .normalize();
        }
        let direction = if direction.length_squared() > 0.5 {
            direction
        } else {
            Vec3::Z
        };
        for limb in &self.data.limbs {
            if limb.bit < 2 {
                continue;
            }
            let side = (limb.bit - 2) as usize;
            let t = (phase + side as f32 * 0.5).rem_euclid(1.);
            let (forward, lift) = foot(t, speed);
            let sole = standing[limb.end].transform_point(Vec3::from_array(limb.offset));
            // Travel direction is relative to actual body heading during turns.
            // Lateral reach stays bounded; anatomical IK retains knee/ankle sides.
            p.planted[side] = Some(
                sole + Vec3::new(
                    (direction.x * forward).clamp(-0.18, 0.18),
                    lift,
                    direction.z * forward,
                ),
            );
            let swing = ((t - stance(speed)) / (1. - stance(speed))).clamp(0., 1.);
            let pitch = (0.12 + 0.35 * run) * (swing * std::f32::consts::TAU).sin() * weight;
            p.normals[side] = Quat::from_rotation_x(pitch) * Vec3::Y;
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
        let arms = lib.world_pose(a);
        assert!(
            arms[13].translation.z > arms[9].translation.z,
            "left arm must oppose the leading right leg"
        );
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

#[cfg(test)]
mod trajectory_tests {
    use super::*;
    #[test]
    fn stance_velocity_matches_displacement_and_run_has_aerial_intervals() {
        for speed in [0.5, 1.5, 2., 3., 4.5, 6.] {
            let dt = 0.0001;
            for phase in [0.05, 0.1, 0.2] {
                let (a, _) = foot(phase, speed);
                let (b, _) = foot(phase + speed * dt / cycle(speed), speed);
                assert!(
                    ((b - a) / dt + speed).abs() < 0.01,
                    "stance must counter actual travel speed"
                );
            }
        }
        assert!(
            contact(0., 2., 0) && contact(0., 2., 1),
            "walk has double support"
        );
        assert!(
            !contact(0.4, 6., 0) && !contact(0.4, 6., 1),
            "run has flight"
        );
        let mut gait = Gait::default();
        let mut distance = 0.;
        let mut previous = 0.;
        for frame in 0..600 {
            let speed = if frame < 120 {
                0.
            } else if frame < 360 {
                6.
            } else {
                0.
            };
            distance += speed / 60.;
            gait.advance(distance, Vec3::Z * speed, Quat::IDENTITY, 1. / 60.);
            let delta = (gait.phase - previous).rem_euclid(1.);
            assert!(delta < 0.5, "speed change must not rephase the cycle");
            previous = gait.phase;
        }
        assert!(gait.speed < 0.001);
    }
    #[test]
    fn turning_changes_swing_direction_without_changing_leg_lengths() {
        let lib = Library::default();
        for direction in [Vec3::Z, Vec3::new(0.6, 0., 0.8), Vec3::new(-0.6, 0., 0.8)] {
            for i in 0..120 {
                let pose = lib.directional_locomotion(i as f32 / 120., 6., direction);
                let frames = lib.world_pose(pose);
                for limb in lib.data.limbs.iter().filter(|l| l.bit >= 2) {
                    let side = Vec3::from_array(lib.data.offsets[limb.upper]).x.signum();
                    let local = frames[1].rotation.inverse()
                        * (frames[limb.end].translation - frames[1].translation);
                    assert!(local.x * side > 0.03);
                    for (a, b) in [(limb.upper, limb.lower), (limb.lower, limb.end)] {
                        assert!(
                            (frames[a].translation.distance(frames[b].translation)
                                - Vec3::from_array(lib.data.offsets[b]).length())
                            .abs()
                                < 1e-4
                        );
                    }
                }
            }
        }
    }
}
