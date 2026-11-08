//! Fixed-clock visual poses. No rigid bodies, collision state or root motion writes.
use crate::{
    game::{Race, Rider},
    recovery::{BodyKind, OwnedBody, Recovery, RecoveryStage},
};
use bevy::prelude::*;
use rr_core::motion::{self, RidePose};
mod grounding;
mod locomotion;
pub mod preview;
#[derive(Clone, Copy)]
pub(crate) struct Pose {
    pub root: Vec3,
    pub held: u8,
    pub lean: f32,
    pub tuck: f32,
    planted: [Option<Vec3>; 2],
    normals: [Vec3; 2],
    pub rotations: [Quat; 20],
}
impl Default for Pose {
    fn default() -> Self {
        Self {
            root: Vec3::ZERO,
            held: 0,
            lean: 0.0,
            tuck: 0.0,
            planted: [None; 2],
            normals: [Vec3::Y; 2],
            rotations: [Quat::IDENTITY; 20],
        }
    }
}
impl Pose {
    pub(crate) fn blend(self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            root: self.root.lerp(b.root, t),
            lean: self.lean.lerp(b.lean, t),
            tuck: self.tuck.lerp(b.tuck, t),
            planted: std::array::from_fn(|i| match (self.planted[i], b.planted[i]) {
                (Some(a), Some(b)) => Some(a.lerp(b, t)),
                (a, b) => {
                    if t < 0.5 {
                        a
                    } else {
                        b
                    }
                }
            }),
            normals: std::array::from_fn(|i| {
                self.normals[i].lerp(b.normals[i], t).normalize_or_zero()
            }),
            held: if t <= 0.0 {
                self.held
            } else if t >= 1.0 {
                b.held
            } else {
                self.held & b.held
            },
            rotations: std::array::from_fn(|i| {
                self.rotations[i].slerp(b.rotations[i], t).normalize()
            }),
        }
    }
}
#[derive(Resource)]
pub(crate) struct Library {
    data: rr_content::motion::Library,
    keys: std::collections::BTreeMap<u16, Pose>,
    sole_local: [Vec3; 2],
}
impl Default for Library {
    fn default() -> Self {
        let data =
            rr_content::motion::Library::from_json(include_str!("../assets/models/motion.json"))
                .expect("validated motion library");
        let keys = data
            .keys
            .iter()
            .map(|k| {
                (
                    k.id,
                    Pose {
                        root: Vec3::from_array(k.root),
                        held: k.held,
                        lean: 0.0,
                        tuck: if k.id == 2030 { 1.0 } else { 0.0 },
                        planted: [None; 2],
                        normals: [Vec3::Y; 2],
                        rotations: k.rotations.map(Quat::from_array),
                    },
                )
            })
            .collect();
        let mut lib = Self {
            data,
            keys,
            sole_local: [Vec3::Y; 2],
        };
        let standing = lib.world_pose(lib.key(2364));
        for limb in &lib.data.limbs {
            if limb.bit >= 2 {
                lib.sole_local[(limb.bit - 2) as usize] =
                    standing[limb.end].rotation.inverse() * Vec3::Y;
            }
        }
        lib
    }
}
impl Library {
    pub(crate) fn key(&self, id: u16) -> Pose {
        self.keys[&id]
    }
    fn clip(&self, name: &str) -> &rr_content::motion::Clip {
        self.data
            .clips
            .iter()
            .find(|c| c.name == name)
            .expect("required visual clip")
    }
    fn duration(&self, name: &str) -> f32 {
        self.clip(name).durations.iter().sum()
    }
    pub(crate) fn sample(&self, name: &str, t: f32) -> Pose {
        let c = self.clip(name);
        let p = motion::sample_phase(&c.durations, t);
        self.key(c.keys[p.from])
            .blend(self.key(c.keys[p.to]), p.blend)
    }
    pub(crate) fn frames(&self, pose: Pose) -> [Transform; 11] {
        let world = self.world_pose(pose);
        self.data.render_joints.map(|i| world[i])
    }
    fn landing_pose(&self, frames: [Transform; 16], anchor: Transform) -> Pose {
        let inverse = anchor.to_matrix().inverse();
        let frames = frames.map(|f| Transform::from_matrix(inverse * f.to_matrix()));
        let mut pose = self.key(2364);
        pose.held = 0;
        pose.lean = 0.0;
        pose.tuck = 0.0;
        pose.root = frames[2].translation - Vec3::from_array(self.data.offsets[1]);
        pose.rotations[0] = Quat::IDENTITY;
        let mut world = [Transform::IDENTITY; 20];
        world[0].translation = pose.root;
        for i in 1..20 {
            let parent = self.data.parents[i] as usize;
            let desired = self
                .data
                .render_joints
                .iter()
                .position(|j| *j == i)
                .map(|j| frames[j].rotation)
                .or_else(|| {
                    [(2, 11), (13, 12), (9, 13), (19, 14), (16, 15)]
                        .iter()
                        .find(|(j, _)| *j == i)
                        .map(|(_, j)| frames[*j].rotation)
                })
                .unwrap_or(world[parent].rotation * pose.rotations[i]);
            pose.rotations[i] = (world[parent].rotation.inverse() * desired).normalize();
            world[i] = world[parent].mul_transform(
                Transform::from_translation(Vec3::from_array(self.data.offsets[i]))
                    .with_rotation(pose.rotations[i]),
            );
        }
        pose
    }
    pub(crate) fn skin_frames(
        &self,
        pose: Pose,
        bind: [Transform; 16],
        seated: bool,
    ) -> [Transform; 16] {
        let all = self.world_pose(pose);
        let reference = self.world_pose(self.key(if seated { 2020 } else { 2364 }));
        let mut out = [Transform::IDENTITY; 16];
        for (i, j) in self.data.render_joints.iter().enumerate() {
            out[i] = all[*j];
        }
        for (i, j, parent, parent_node) in [
            (11, 2, 0, 3),
            (12, 13, 4, 12),
            (13, 9, 6, 8),
            (14, 19, 8, 18),
            (15, 16, 10, 15),
        ] {
            // Distal rest frames must inherit the animated parent, not the old authoring pose.
            let rest =
                Transform::from_matrix(bind[parent].to_matrix().inverse() * bind[i].to_matrix());
            let current_local = all[parent_node].rotation.inverse() * all[j].rotation;
            let reference_local = reference[parent_node].rotation.inverse() * reference[j].rotation;
            out[i] = out[parent].mul_transform(rest);
            out[i].rotation =
                (out[parent].rotation * current_local * reference_local.inverse() * rest.rotation)
                    .normalize();
        }
        out
    }
    fn world_pose(&self, pose: Pose) -> [Transform; 20] {
        let mut world = [Transform::IDENTITY; 20];
        for i in 0..20 {
            let parent = self.data.parents[i];
            let local = Transform::from_translation(if i == 0 {
                pose.root
            } else {
                Vec3::from_array(self.data.offsets[i])
            })
            .with_rotation(pose.rotations[i]);
            world[i] = if parent < 0 {
                local
            } else {
                world[parent as usize].mul_transform(local)
            };
        }
        // Render-only two-link contact fitting. Preserve each anatomical length;
        // never translate a joint independently of its parent chain.
        for limb in &self.data.limbs {
            let planted = if limb.bit >= 2 {
                pose.planted[(limb.bit - 2) as usize]
            } else {
                None
            };
            if limb.bit < 2 && pose.held & (1 << limb.bit) == 0 && planted.is_none() {
                continue;
            }
            let a = world[limb.upper];
            let b = world[limb.lower];
            let mut end = world[limb.end];
            if planted.is_some() {
                let i = (limb.bit - 2) as usize;
                end.rotation =
                    (Quat::from_rotation_arc(end.rotation * self.sole_local[i], pose.normals[i])
                        * end.rotation)
                        .normalize();
                world[limb.end].rotation = end.rotation;
            }
            let l1 = a.translation.distance(b.translation);
            let l2 = b.translation.distance(end.translation);
            let held = pose.held & (1 << limb.bit) != 0;
            let mut target = if planted.is_some() || held {
                planted.unwrap_or(Vec3::from_array(limb.target))
                    - end.rotation * Vec3::from_array(limb.offset)
            } else {
                end.translation
            };
            if limb.bit >= 2 && !held {
                // Keep the ankle on its own side of the pelvis during pickup
                // and locomotion. The knee is solved as one chain below.
                let pelvis = world[1];
                let mut local = pelvis.rotation.inverse() * (target - pelvis.translation);
                let side = Vec3::from_array(self.data.offsets[limb.upper]).x.signum();
                local.x = side * (side * local.x).max(0.08);
                target = pelvis.translation + pelvis.rotation * local;
            }
            let delta = target - a.translation;
            let axis = delta.normalize_or_zero();
            if axis.length_squared() < 0.5 {
                continue;
            }
            let minimum = (l1 * l1 + l2 * l2 + 2.0 * l1 * l2 * limb.max_flex.cos()).sqrt() + 0.0001;
            let d = delta.length().clamp(minimum, l1 + l2 - 0.0001);
            let wrist = a.translation + axis * d;
            let along = (l1 * l1 - l2 * l2 + d * d) / (2.0 * d);
            let inward = if limb.bit < 2 {
                -a.translation.x.signum() * (pose.lean.abs() * 0.10 + pose.tuck * 0.12).min(0.18)
            } else {
                0.0
            };
            let mut pole = if limb.bit >= 2 {
                let side = Vec3::from_array(self.data.offsets[limb.upper]).x.signum();
                world[1].rotation
                    * (Vec3::Z
                        + if held && planted.is_none() {
                            Vec3::X * side * 0.28
                        } else {
                            Vec3::ZERO
                        })
            } else {
                b.translation - a.translation + Vec3::X * inward
            };
            if limb.bit < 2 {
                pole.y = pole.y.min(-0.18) - pose.tuck * 0.18;
            }
            let perp = (pole - axis * pole.dot(axis))
                .try_normalize()
                .unwrap_or(Vec3::Y);
            let elbow =
                a.translation + axis * along + perp * (l1 * l1 - along * along).max(0.0).sqrt();
            world[limb.upper].rotation = (Quat::from_rotation_arc(
                (b.translation - a.translation).normalize(),
                (elbow - a.translation).normalize(),
            ) * a.rotation)
                .normalize();
            world[limb.lower].translation = elbow;
            world[limb.lower].rotation = (Quat::from_rotation_arc(
                (end.translation - b.translation).normalize(),
                (wrist - elbow).normalize(),
            ) * b.rotation)
                .normalize();
            world[limb.end].translation = wrist;
        }
        world
    }

    fn turn(&self, mut pose: Pose, turn: f32) -> Pose {
        pose.lean = turn.clamp(-1.0, 1.0);
        let neutral = self.key(2020);
        let target = self.key(if turn < 0.0 { 2900 } else { 2901 });
        for i in 0..20 {
            let delta = neutral.rotations[i].inverse() * target.rotations[i];
            pose.rotations[i] =
                (pose.rotations[i] * Quat::IDENTITY.slerp(delta, turn.abs().min(1.0))).normalize();
        }
        pose
    }
}
#[derive(Clone)]
struct Attack {
    name: &'static str,
    age: f32,
    from: Pose,
}
#[derive(Component)]
pub struct Animator {
    pub(crate) previous: Pose,
    pub(crate) current: Pose,
    pub(crate) ready: bool,
    tuck: f32,
    accel: f32,
    serial: u32,
    attack: Option<Attack>,
    stage: RecoveryStage,
    return_age: f32,
    return_from: Pose,
    last_race_time: f32,
    pub(crate) bike_bank: f32,
    previous_bank: f32,
    pickup_mirrored: bool,
    bank_from: f32,
    anchor: Transform,
    landed: Option<Pose>,
    plants: [Option<(Vec3, Vec3)>; 2],
    plant_weight: [f32; 2],
}
impl Default for Animator {
    fn default() -> Self {
        Self {
            previous: Pose::default(),
            current: Pose::default(),
            ready: false,
            tuck: 0.0,
            accel: 0.0,
            serial: 0,
            attack: None,
            stage: RecoveryStage::Riding,
            return_age: 1.0,
            return_from: Pose::default(),
            last_race_time: 0.0,
            bike_bank: 0.0,
            previous_bank: 0.0,
            pickup_mirrored: false,
            bank_from: 0.0,
            anchor: Transform::IDENTITY,
            landed: None,
            plants: [None; 2],
            plant_weight: [0.0; 2],
        }
    }
}
#[derive(Component, Clone, Copy)]
pub(crate) enum Binding {
    Riding(Entity),
    Runner(Entity),
}
#[allow(clippy::too_many_arguments)]
fn update(
    time: Res<Time<Fixed>>,
    race: Option<Res<Race>>,
    lib: Res<Library>,
    mut query: Query<(&Rider, &Recovery, &mut Animator, Option<&Transform>)>,
    bodies: Query<&Transform, Without<Rider>>,
    context: bevy_rapier3d::prelude::ReadRapierContext,
    art: Option<Res<crate::world::Art>>,
    recovery_tuning: Option<Res<crate::recovery::RecoveryTuning>>,
) {
    let Some(race) = race else { return };
    if !race.phase.simulating() {
        for (_, _, mut a, _) in &mut query {
            a.previous = a.current;
            a.previous_bank = a.bike_bank;
        }
        return;
    }
    let dt = time.delta_secs();
    for (r, rec, mut a, physical) in &mut query {
        if !a.ready || race.elapsed < a.last_race_time {
            *a = Animator::default();
            a.current = lib.key(2020);
            a.previous = a.current;
            a.ready = true;
        }
        a.last_race_time = race.elapsed;
        a.previous = a.current;
        a.previous_bank = a.bike_bank;
        let entered = rec.stage != a.stage;
        if entered {
            a.plants = [None; 2];
            a.plant_weight = [0.0; 2];
            if rec.stage == RecoveryStage::GettingUp {
                a.landed = rec
                    .landing
                    .as_ref()
                    .and_then(|land| art.as_ref()?.models.as_ref()?.physical_frames(land))
                    .zip(rec.runner.and_then(|e| bodies.get(e).ok()))
                    .map(|(frames, root)| {
                        lib.landing_pose(
                            frames,
                            root.mul_transform(Transform::from_xyz(0.0, -0.95, 0.0)),
                        )
                    });
                if let Some(landed) = a.landed {
                    a.current = landed;
                    a.previous = landed;
                }
            }
            if rec.stage == RecoveryStage::Mounting {
                if let Some((runner, wreck)) = rec
                    .runner
                    .and_then(|e| bodies.get(e).ok())
                    .zip(rec.wreck.and_then(|e| bodies.get(e).ok()))
                {
                    let anchor = bike_anchor(*wreck);
                    let space = anchor.to_matrix().inverse()
                        * runner.to_matrix()
                        * Mat4::from_translation(Vec3::new(0.0, -0.95, 0.0));
                    a.current = change_space(a.current, space);
                    a.previous = a.current;
                    a.anchor = anchor;
                    let up = (anchor.rotation.inverse() * wreck.rotation) * Vec3::Y;
                    a.bank_from = (-up.x).atan2(up.y);
                    a.previous_bank = a.bank_from;
                    a.pickup_mirrored = a.bank_from > 0.0;
                }
            } else if rec.stage == RecoveryStage::Riding
                && a.stage == RecoveryStage::Mounting
                && let Some(physical) = physical
            {
                a.current = change_space(
                    a.current,
                    physical.to_matrix().inverse() * a.anchor.to_matrix(),
                );
                a.previous = a.current;
            }
            a.return_from = a.current;
            a.return_age = 0.0;
            a.attack = None;
        }
        a.stage = rec.stage;
        if rec.stage != RecoveryStage::Riding {
            a.tuck = motion::approach(a.tuck, 0.0, dt, 0.18);
            a.accel = motion::approach(a.accel, 0.0, dt, 0.12);
        }
        match rec.stage {
            RecoveryStage::Riding => {
                // Original fixed-point selector, not an arbitrary speed threshold. The
                // stock flag branch is enabled; nitro does not rewrite physics or speed.
                let mode = motion::reference_ride_pose(
                    motion::reference_velocity(r.speed),
                    (r.drive_intent.throttle_strength() * 100.0) as i32,
                    (r.side_velocity * 256.0) as i32,
                    0,
                    100,
                );
                a.tuck = motion::approach(
                    a.tuck,
                    if mode == RidePose::Tuck { 1.0 } else { 0.0 },
                    dt,
                    0.18,
                );
                a.accel = motion::approach(
                    a.accel,
                    if mode == RidePose::Acceleration {
                        1.0
                    } else {
                        0.0
                    },
                    dt,
                    0.12,
                );
                let base = lib.turn(
                    lib.key(2020)
                        .blend(lib.key(2030), a.tuck)
                        .blend(lib.key(2040), a.accel),
                    r.lean / 0.65,
                );
                if r.attack_serial != a.serial {
                    a.serial = r.attack_serial;
                    if r.attack > 0.0 {
                        // Gameplay right is forward cross up (-X); authored left is +X.
                        // Both punching and kicking use the resolved gameplay side.
                        let positive = r.attack_side > 0.0;
                        let name = match (r.kicking, positive) {
                            (false, true) => "punch_right",
                            (false, false) => "punch_left",
                            (true, true) => "kick_right",
                            (true, false) => "kick_left",
                        };
                        a.attack = Some(Attack {
                            name,
                            age: 0.0,
                            from: a.current,
                        });
                    }
                }
                let mut pose = base;
                if let Some(attack) = a.attack.as_mut() {
                    attack.age += dt;
                    let playback = 1.8;
                    let source_duration = lib.duration(attack.name);
                    let duration = source_duration / playback;
                    let enter = 0.055;
                    let exit = 0.08;
                    pose = if attack.age < enter {
                        attack.from.blend(
                            lib.turn(lib.sample(attack.name, 0.0), r.lean / 0.65),
                            motion::smoothstep(attack.age / enter),
                        )
                    } else if attack.age < enter + duration {
                        lib.turn(
                            lib.sample(attack.name, (attack.age - enter) * playback),
                            r.lean / 0.65,
                        )
                    } else {
                        lib.turn(lib.sample(attack.name, source_duration), r.lean / 0.65)
                            .blend(
                                base,
                                motion::smoothstep((attack.age - enter - duration) / exit),
                            )
                    };
                    if attack.age >= enter + duration + exit {
                        a.attack = None;
                    }
                }
                if a.return_age < 0.16 {
                    a.return_age += dt;
                    pose = a
                        .return_from
                        .blend(pose, motion::smoothstep(a.return_age / 0.16));
                }
                a.current = pose;
                a.bike_bank = 0.0;
            }
            RecoveryStage::Falling => {
                a.current = lib.sample("getup", 0.0);
                a.attack = None;
                a.serial = r.attack_serial;
            }
            RecoveryStage::GettingUp => {
                let target = lib.sample(
                    "getup",
                    (rec.elapsed / 0.9).clamp(0.0, 1.0) * lib.duration("getup"),
                );
                a.current = a.landed.map_or(target, |from| {
                    from.blend(target, motion::smoothstep(rec.elapsed / 0.45))
                });
            }
            RecoveryStage::Running => {
                let stride = recovery_tuning.as_ref().map_or(3.2, |t| t.0.stride_length);
                let phase = (rec.walk_distance / stride).rem_euclid(1.0);
                let speed = rec.walk_velocity.with_y(0.0).length();
                let target = lib.locomotion(phase, speed);
                a.return_age += dt;
                a.current = a
                    .return_from
                    .blend(target, motion::smoothstep(a.return_age / 0.12));
            }
            RecoveryStage::Mounting => {
                let fraction = (rec.elapsed / 0.75).clamp(0.0, 1.0);
                let name = if a.pickup_mirrored {
                    "pickup_mirrored"
                } else {
                    "pickup"
                };
                let progress = ((fraction - 0.16) / 0.69).clamp(0.0, 1.0);
                let target = lib.sample(name, progress * lib.duration(name));
                a.current = if fraction < 0.16 {
                    a.return_from
                        .blend(target, motion::smoothstep(fraction / 0.16))
                } else {
                    target
                };
                if fraction > 0.85 {
                    a.current = a.current.blend(
                        lib.key(2020).blend(lib.key(2030), a.tuck),
                        motion::smoothstep((fraction - 0.85) / 0.15),
                    );
                }
                let sign = if a.pickup_mirrored { -1.0 } else { 1.0 };
                let angles = [-48.0f32, -28.0, -12.0, -5.0, 0.0];
                let f = progress * 4.0;
                let i = f.floor().min(3.0) as usize;
                let bank = sign
                    * (angles[i] + (angles[i + 1] - angles[i]) * motion::smoothstep(f - i as f32))
                        .to_radians();
                a.bike_bank = if fraction < 0.16 {
                    a.bank_from + (bank - a.bank_from) * motion::smoothstep(fraction / 0.16)
                } else {
                    bank
                };
                if let Some(wreck) = rec.wreck.and_then(|e| bodies.get(e).ok()) {
                    a.anchor = bike_anchor(*wreck);
                }
            }
        }
        if matches!(rec.stage, RecoveryStage::Running | RecoveryStage::GettingUp)
            && let Some(runner) = rec.runner
            && let Ok(body) = bodies.get(runner)
        {
            grounding::apply(
                &lib,
                &mut a,
                rec,
                body,
                &context,
                dt,
                recovery_tuning.as_ref().map_or(3.2, |t| t.0.stride_length),
            );
        }
    }
}
fn change_space(mut pose: Pose, space: Mat4) -> Pose {
    let (_, rotation, translation) = space.to_scale_rotation_translation();
    pose.root = rotation * pose.root + translation;
    pose.rotations[0] = (rotation * pose.rotations[0]).normalize();
    pose
}
/// Unbanked presentation anchor shared by lifting rider and lifting bike mesh.
pub(crate) fn bike_anchor(t: Transform) -> Transform {
    let f = (t.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
    let q = if f.length_squared() > 0.5 {
        Quat::from_rotation_arc(Vec3::Z, f)
    } else {
        Quat::IDENTITY
    };
    Transform::from_translation(t.translation).with_rotation(q)
}
fn lift_visuals(
    owners: Query<&OwnedBody>,
    riders: Query<(&Animator, &Recovery)>,
    bodies: Query<&Transform, Without<crate::runtime::VisualRoot>>,
    mut visuals: Query<(&crate::runtime::VisualRoot, &mut Transform)>,
    fixed: Res<Time<Fixed>>,
) {
    for (v, mut local) in &mut visuals {
        let Ok(owner) = owners.get(v.owner) else {
            continue;
        };
        if owner.kind != BodyKind::Wreck {
            continue;
        }
        let Ok((a, rec)) = riders.get(owner.owner) else {
            continue;
        };
        if !a.ready || rec.stage != RecoveryStage::Mounting {
            continue;
        }
        let Ok(physical) = bodies.get(v.owner) else {
            continue;
        };
        let anchor = bike_anchor(*physical);
        local.rotation = physical.rotation.inverse()
            * anchor.rotation
            * Quat::from_rotation_z(a.previous_bank).slerp(
                Quat::from_rotation_z(a.bike_bank),
                fixed.overstep_fraction(),
            );
    }
}
pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Library>()
            .add_systems(
                FixedUpdate,
                update.after(crate::runtime::SimSet::AfterPhysics),
            )
            .add_systems(Update, lift_visuals.after(crate::vehicle::visuals));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interpolation_preserves_every_render_chain_length_and_unit_rotations() {
        let lib = Library::default();
        for clip in [
            "punch_left",
            "punch_right",
            "kick_left",
            "kick_right",
            "pickup",
            "getup",
        ] {
            for step in 0..100 {
                let p = lib.sample(clip, lib.duration(clip) * step as f32 / 99.0);
                assert!(
                    p.rotations
                        .iter()
                        .all(|q| q.is_finite() && (q.length() - 1.0).abs() < 1e-5)
                );
                let frames = lib.frames(p);
                for (i, &parent) in rr_content::riding_model::RENDER_PARENTS.iter().enumerate() {
                    if parent >= 0 {
                        assert!(
                            frames[i]
                                .translation
                                .distance(frames[parent as usize].translation)
                                .is_finite()
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn normal_tuck_endpoints_are_distinct_and_shortest_arc_is_finite() {
        let lib = Library::default();
        let n = lib.key(2020);
        let t = lib.key(2030);
        assert!(lib.frames(n)[1].translation.y - lib.frames(t)[1].translation.y > 0.15);
        let mid = n.blend(t, 0.5);
        assert!(mid.rotations.iter().all(|q| q.is_finite()));
        assert_eq!(n.blend(t, 0.0).root, n.root);
        assert_eq!(n.blend(t, 1.0).root, t.root);
    }
    fn state_app() -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Library>()
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(Race {
                phase: crate::game::Phase::Racing,
                ..default()
            })
            .add_systems(FixedUpdate, update);
        let mut rider = Rider::new(0);
        rider.speed = 65.0;
        let e = app.world_mut().spawn((rider, Transform::IDENTITY)).id();
        (app, e)
    }
    fn tick(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
        app.world_mut().resource_mut::<Race>().elapsed += 1.0 / 60.0;
        app.world_mut().run_schedule(FixedUpdate);
    }
    #[test]
    fn resolved_gameplay_side_selects_the_matching_punch_and_kick() {
        for (kicking, side, expected) in [
            (false, -1.0, "punch_left"),
            (false, 1.0, "punch_right"),
            (true, -1.0, "kick_left"),
            (true, 1.0, "kick_right"),
        ] {
            let (mut app, e) = state_app();
            tick(&mut app);
            {
                let mut r = app.world_mut().get_mut::<Rider>(e).unwrap();
                r.attack_serial = 1;
                r.attack = 0.65;
                r.attack_side = side;
                r.kicking = kicking;
            }
            tick(&mut app);
            assert_eq!(
                app.world()
                    .get::<Animator>(e)
                    .unwrap()
                    .attack
                    .as_ref()
                    .unwrap()
                    .name,
                expected
            );
        }
    }
    #[test]
    fn attacks_enter_from_actual_tuck_and_return_to_the_latest_base_without_retrigger() {
        let (mut app, e) = state_app();
        for _ in 0..120 {
            tick(&mut app);
        }
        let from = app.world().get::<Animator>(e).unwrap().current;
        assert!(app.world().get::<Animator>(e).unwrap().tuck > 0.999);
        {
            let mut r = app.world_mut().get_mut::<Rider>(e).unwrap();
            r.attack_serial = 1;
            r.attack = 0.65;
            r.attack_side = 1.0;
        }
        tick(&mut app);
        let a = app.world().get::<Animator>(e).unwrap();
        let target = app.world().resource::<Library>().sample("punch_right", 0.0);
        let expected = from.blend(target, motion::smoothstep((1.0 / 60.0) / 0.055));
        assert!(a.current.root.distance(expected.root) < 1e-6);
        for (a, b) in a.current.rotations.iter().zip(expected.rotations) {
            assert!(a.dot(b).abs() > 0.99999);
        }
        app.world_mut().get_mut::<Rider>(e).unwrap().speed = 10.0;
        for _ in 0..100 {
            tick(&mut app);
        }
        let a = app.world().get::<Animator>(e).unwrap();
        assert!(a.attack.is_none());
        assert!(a.tuck < 0.001);
        let base = app.world().resource::<Library>().key(2020);
        assert!(a.current.root.distance(base.root) < 0.001);
    }
    #[test]
    fn pause_freezes_pose_clock_and_crash_interrupts_only_visual_state() {
        let (mut app, e) = state_app();
        for _ in 0..20 {
            tick(&mut app);
        }
        let before = app.world().get::<Animator>(e).unwrap().current;
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Paused;
        for _ in 0..20 {
            tick(&mut app);
        }
        let a = app.world().get::<Animator>(e).unwrap();
        assert_eq!(a.current.root, before.root);
        assert_eq!(a.current.rotations, before.rotations);
        assert_eq!(a.previous.root, a.current.root);
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Racing;
        app.world_mut().get_mut::<Recovery>(e).unwrap().stage = RecoveryStage::Falling;
        let physical = *app.world().get::<Transform>(e).unwrap();
        tick(&mut app);
        assert!(app.world().get::<Animator>(e).unwrap().attack.is_none());
        assert_eq!(*app.world().get::<Transform>(e).unwrap(), physical);
    }
    #[test]
    fn held_contacts_and_anatomical_lengths_survive_between_keyframes() {
        let lib = Library::default();
        let mut maximum = 0.0f32;
        for name in [
            "normal",
            "tuck",
            "punch_left",
            "punch_right",
            "kick_left",
            "kick_right",
        ] {
            for step in 0..100 {
                let p = lib.sample(name, lib.duration(name) * step as f32 / 99.0);
                let world = lib.world_pose(p);
                for l in &lib.data.limbs {
                    let upper = world[l.upper].translation;
                    let lower = world[l.lower].translation;
                    let end = world[l.end].translation;
                    assert!(
                        (upper.distance(lower)
                            - Vec3::from_array(lib.data.offsets[l.lower]).length())
                        .abs()
                            < 0.0001
                    );
                    assert!(
                        (lower.distance(end) - Vec3::from_array(lib.data.offsets[l.end]).length())
                            .abs()
                            < 0.0001
                    );
                    if p.held & (1 << l.bit) != 0 {
                        let tip = end + world[l.end].rotation * Vec3::from_array(l.offset);
                        maximum = maximum.max(tip.distance(Vec3::from_array(l.target)));
                    }
                }
            }
        }
        assert!(maximum < 0.006, "held contact drift {maximum} m");
        for step in 0..100 {
            let p = lib.key(2020).blend(lib.key(2030), step as f32 / 99.0);
            let world = lib.world_pose(p);
            for l in &lib.data.limbs {
                let tip =
                    world[l.end].translation + world[l.end].rotation * Vec3::from_array(l.offset);
                assert!(tip.distance(Vec3::from_array(l.target)) < 0.006);
            }
        }
    }
}

#[cfg(test)]
mod correction_tests {
    use super::*;
    #[test]
    fn seated_knee_bend_plane_points_slightly_outward_and_feet_stay_on_pegs() {
        let lib = Library::default();
        for key in [2020, 2030, 2040, 2900, 2901] {
            let p = lib.key(key);
            let w = lib.world_pose(p);
            let pelvis = w[1];
            for l in lib.data.limbs.iter().filter(|l| l.bit >= 2) {
                let side = Vec3::from_array(lib.data.offsets[l.upper]).x.signum();
                let axis = (w[l.end].translation - w[l.upper].translation).normalize();
                let knee = w[l.lower].translation - w[l.upper].translation;
                let bend = knee - axis * knee.dot(axis);
                let out = pelvis.rotation * (Vec3::Z + Vec3::X * side * 0.28);
                assert!(
                    bend.normalize()
                        .dot((out - axis * out.dot(axis)).normalize())
                        > 0.999
                );
                let tip = w[l.end].translation + w[l.end].rotation * Vec3::from_array(l.offset);
                assert!(tip.distance(Vec3::from_array(l.target)) < 0.006);
            }
        }
    }
    #[test]
    fn pickup_and_running_keep_ankles_on_their_own_side_and_knees_in_a_bending_plane() {
        let lib = Library::default();
        for clip in ["pickup", "pickup_mirrored", "run", "getup"] {
            for step in 0..100 {
                let p = lib.sample(clip, lib.duration(clip) * step as f32 / 99.0);
                let w = lib.world_pose(p);
                let pelvis = w[1];
                for limb in lib.data.limbs.iter().filter(|l| l.bit >= 2) {
                    let side = Vec3::from_array(lib.data.offsets[limb.upper]).x.signum();
                    let ankle =
                        pelvis.rotation.inverse() * (w[limb.end].translation - pelvis.translation);
                    assert!(ankle.x * side > 0.04, "{clip} crossed ankle {ankle:?}");
                    let axis = (w[limb.end].translation - w[limb.upper].translation).normalize();
                    let knee = w[limb.lower].translation - w[limb.upper].translation;
                    let perpendicular = knee - axis * knee.dot(axis);
                    let forward = pelvis.rotation * Vec3::Z;
                    let pole = (forward - axis * forward.dot(axis)).normalize();
                    assert!(
                        perpendicular.normalize().dot(pole) > 0.999,
                        "{clip} sideways knee"
                    );
                }
            }
        }
    }
    #[test]
    fn tuck_and_steering_draw_both_elbows_in_without_releasing_grips() {
        let lib = Library::default();
        for id in [2020, 2030] {
            for lean in [-1.0, 0.0, 1.0] {
                let corrected = lib.turn(lib.key(id), lean);
                let mut old = corrected;
                old.lean = 0.0;
                old.tuck = 0.0;
                let a = lib.world_pose(old);
                let b = lib.world_pose(corrected);
                for (elbow, hand) in [(8, 9), (12, 13)] {
                    assert!(b[elbow].translation.x.abs() <= a[elbow].translation.x.abs() + 0.0001);
                    assert!(a[hand].translation.distance(b[hand].translation) < 0.001);
                }
                if id == 2030 || lean != 0.0 {
                    assert!(a[8].translation.distance(b[8].translation) > 0.005);
                }
            }
        }
    }
    #[test]
    fn held_elbows_point_down_in_normal_and_fast_tuck() {
        let lib = Library::default();
        for id in [2020, 2030] {
            let w = lib.world_pose(lib.key(id));
            for (shoulder, elbow) in [(7, 8), (11, 12)] {
                assert!(
                    w[elbow].translation.y < w[shoulder].translation.y - 0.02,
                    "raised elbow in {id}"
                );
            }
        }
    }
    #[test]
    fn planted_feet_fit_a_sloped_surface_without_stretching_the_leg() {
        let lib = Library::default();
        let mut pose = lib.key(2364);
        pose.held = 0;
        let raw = lib.world_pose(pose);
        for limb in &lib.data.limbs {
            if limb.bit >= 2 {
                let i = (limb.bit - 2) as usize;
                pose.planted[i] = Some(
                    raw[limb.end].translation
                        + raw[limb.end].rotation * Vec3::from_array(limb.offset)
                        + Vec3::Y * 0.08,
                );
                pose.normals[i] = Vec3::new(0.0, 1.0, 0.15).normalize();
            }
        }
        let fitted = lib.world_pose(pose);
        for limb in &lib.data.limbs {
            if limb.bit >= 2 {
                assert!(
                    (raw[limb.upper]
                        .translation
                        .distance(raw[limb.lower].translation)
                        - fitted[limb.upper]
                            .translation
                            .distance(fitted[limb.lower].translation))
                    .abs()
                        < 0.001
                );
                assert!(
                    (raw[limb.lower]
                        .translation
                        .distance(raw[limb.end].translation)
                        - fitted[limb.lower]
                            .translation
                            .distance(fitted[limb.end].translation))
                    .abs()
                        < 0.001
                );
                assert!(
                    (fitted[limb.end].rotation * lib.sole_local[(limb.bit - 2) as usize])
                        .dot(pose.normals[(limb.bit - 2) as usize])
                        > 0.999
                );
            }
        }
    }
}
