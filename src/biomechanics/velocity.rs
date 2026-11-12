//! Selected velocity muscle controller with constrained passive anatomy.
//! No body position or linear velocity is authored by this controller.
use super::*;
use rr_core::velocity_muscles::{Phase, Sense};
use std::collections::HashMap;
#[derive(Resource)]
pub struct Tuning(pub rr_content::velocity_muscles::Config);
impl Default for Tuning {
    fn default() -> Self {
        Self(
            rr_content::velocity_muscles::Config::from_json(include_str!(
                "../../content/velocity_muscles.json"
            ))
            .expect("validated velocity muscles"),
        )
    }
}
#[derive(Component, Default)]
pub struct Pilot {
    pub state: rr_core::velocity_muscles::Recovery,
    pub impulses: u64,
    pub supported: bool,
    pub supine: f32,
    pub guide_impulses: u64,
    pub guide_blocked: bool,
}
#[derive(Clone, Copy)]
struct Body {
    entity: Entity,
    pose: Transform,
    omega: Vec3,
    inverse: Mat3,
    target: Quat,
}
#[derive(Default)]
pub(super) struct Rig {
    parts: [Option<Body>; 16],
    mass: f32,
    momentum: Vec3,
    support: Vec3,
    any_support: Vec3,
    support_surfaces: Vec<Entity>,
}
fn tensor(m: Mat3) -> rr_core::velocity_muscles::Tensor {
    std::array::from_fn(|i| std::array::from_fn(|j| m.col(j)[i] as f64))
}
fn as_vec(v: rr_core::velocity_muscles::Vector) -> Vec3 {
    Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32)
}
fn array(v: Vec3) -> rr_core::velocity_muscles::Vector {
    v.to_array().map(f64::from)
}
fn error(q: Quat) -> Vec3 {
    let q = if q.w < 0.0 { -q } else { q };
    let angle = 2.0 * q.w.clamp(-1.0, 1.0).acos();
    q.xyz().normalize_or_zero() * angle
}
#[allow(clippy::too_many_arguments)]
fn desired(
    part: usize,
    default: Quat,
    parent: Quat,
    root: Transform,
    normal: Vec3,
    velocity: Vec3,
    phase: Phase,
    direction: f32,
) -> Quat {
    let forward = root.rotation * Vec3::Z;
    let side = root.rotation * Vec3::X;
    match phase {
        Phase::Brace if matches!(part, 3 | 5) => {
            let fall = velocity.with_y(0.0).normalize_or_zero();
            let arm_side = if part == 3 { -1.0 } else { 1.0 };
            let reach = (fall * 0.65 - normal * 0.85 + side * arm_side * 0.18).normalize_or_zero();
            parent.inverse() * Quat::from_rotation_arc(Vec3::NEG_Y, reach)
        }
        Phase::Brace if matches!(part, 4 | 6) => rotation([-0.35, 0.0, 0.0]),
        Phase::Brace if part == 1 => rotation([
            if forward.dot(velocity) > 0.0 {
                -0.2
            } else {
                0.35
            },
            0.0,
            0.0,
        ]),
        Phase::Roll if matches!(part, 3 | 5) => {
            // Clear the supporting arm footprint before the body rolls. Arms
            // reaching sideways into the ground otherwise prop a prone body up.
            rotation([1.1, 0.0, if part == 3 { 0.3 } else { -0.3 }])
        }
        Phase::Roll if matches!(part, 2 | 11) => rotation([0.0, direction * 0.08, 0.0]),
        Phase::Roll if matches!(part, 7 | 9) => {
            rotation([0.02, 0.0, if part == 7 { -0.03 } else { 0.03 }])
        }
        Phase::Roll if matches!(part, 8 | 10) => rotation([0.08, 0.0, 0.0]),
        Phase::Roll if matches!(part, 4 | 6) => rotation([-0.3, 0.0, 0.0]),
        Phase::Slide if part == 1 => rotation([0.3, 0.0, 0.0]),
        Phase::Slide if matches!(part, 8 | 10) => rotation([0.45, 0.0, 0.0]),
        Phase::Slide if matches!(part, 3 | 5) => {
            rotation([0.15, 0.0, if part == 3 { -0.1 } else { 0.1 }])
        }
        Phase::Slide if matches!(part, 4 | 6) => rotation([-0.5, 0.0, 0.0]),
        _ => default,
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(super) fn update(
    settings: Res<Settings>,
    tuning: Res<Tuning>,
    time: Res<Time<Fixed>>,
    race: Option<Res<crate::game::Race>>,
    context: ReadRapierContext,
    snapshots: Query<(
        Entity,
        &OwnedBody,
        &Transform,
        &ReadMassProperties,
        &Segment,
        Option<&Muscle>,
    )>,
    mut velocities: Query<&mut Velocity>,
    fixed: Query<&RigidBody>,
    mut brains: Query<(&OwnedBody, &mut Brain, &mut Pilot)>,
    mut scratch: Local<HashMap<Entity, Rig>>,
) {
    if !settings.enabled || race.is_some_and(|r| !r.phase.simulating()) {
        return;
    }
    scratch.clear();
    let context = context.single().ok();
    let dt = time.delta_secs();
    for (entity, owner, pose, mass, segment, muscle) in &snapshots {
        if mass.mass <= 0.0 || mass.principal_inertia.min_element() <= 1e-8 {
            continue;
        }
        let Ok(v) = velocities.get(entity) else {
            continue;
        };
        let frame = Mat3::from_quat(pose.rotation * mass.principal_inertia_local_frame);
        let inverse =
            frame * Mat3::from_diagonal(mass.principal_inertia.recip()) * frame.transpose();
        let rig = scratch.entry(owner.owner).or_default();
        rig.mass += mass.mass;
        rig.momentum += v.linear * mass.mass;
        rig.parts[segment.part] = Some(Body {
            entity,
            pose: *pose,
            omega: v.angular,
            inverse,
            target: muscle.map_or(Quat::IDENTITY, |m| m.target),
        });
        if let Some(context) = &context {
            for pair in context.contact_pairs_with(entity) {
                let first = pair.collider1() == Some(entity);
                let other = if first {
                    pair.collider2()
                } else {
                    pair.collider1()
                };
                if !pair.has_any_active_contact()
                    || !other.is_some_and(|e| fixed.get(e).is_ok_and(|b| *b == RigidBody::Fixed))
                {
                    continue;
                }
                for manifold in pair.manifolds() {
                    let normal = if first {
                        -manifold.normal()
                    } else {
                        manifold.normal()
                    };
                    if manifold.num_solver_contacts() > 0 && normal.y >= tuning.0.support_normal_y {
                        rig.any_support += normal;
                        if matches!(segment.part, 0 | 2 | 11) {
                            rig.support += normal;
                        }
                        if let Some(other) = other
                            && !rig.support_surfaces.contains(&other)
                        {
                            rig.support_surfaces.push(other);
                        }
                    }
                }
            }
        }
    }
    for (owner, mut brain, mut pilot) in &mut brains {
        let Some(rig) = scratch.get_mut(&owner.owner) else {
            continue;
        };
        let Some(root) = rig.parts[0] else {
            continue;
        };
        if rig.mass <= 1.0 {
            continue;
        }
        let velocity = rig.momentum / rig.mass;
        // Coarse capsules/boots can hold the real torso proxy a few cm above
        // the same plane. Require actual upward limb contact plus a core probe
        // within8cm of its oriented extent, never an airborne ray-only guess.
        if rig.support.length_squared() < 0.1
            && rig.any_support.length_squared() > 0.1
            && let Some(context) = &context
        {
            for part in [0, 2, 11] {
                let Some(body) = rig.parts[part] else {
                    continue;
                };
                let half = specs()[part].size * 0.5;
                let extent = (body.pose.rotation * Vec3::X).y.abs() * half.x
                    + (body.pose.rotation * Vec3::Y).y.abs() * half.y
                    + (body.pose.rotation * Vec3::Z).y.abs() * half.z;
                if let Some((entity, hit)) = context.cast_ray_and_get_normal(
                    body.pose.translation,
                    Vec3::NEG_Y,
                    extent + 0.08,
                    true,
                    QueryFilter::only_fixed().exclude_sensors(),
                ) && hit.normal.y >= tuning.0.support_normal_y
                {
                    rig.support += hit.normal;
                    if !rig.support_surfaces.contains(&entity) {
                        rig.support_surfaces.push(entity);
                    }
                }
            }
        }
        let supported = rig.support.length_squared() > 0.1;
        let normal = if supported {
            rig.support.normalize()
        } else {
            Vec3::Y
        };
        let supine = (root.pose.rotation * Vec3::Z).dot(normal);
        let limb_spin = rig
            .parts
            .iter()
            .flatten()
            .map(|b| b.omega.length())
            .fold(0.0f32, f32::max);
        let spin = [0, 2, 11]
            .into_iter()
            .filter_map(|i| rig.parts[i])
            .map(|b| b.omega.length())
            .fold(0.0f32, f32::max);
        let sense = Sense {
            supported,
            speed: velocity.with_y(0.0).length(),
            spin,
            limb_spin,
            supine,
            roll_side: (root.pose.rotation * Vec3::X).dot(normal),
        };
        let phase = pilot.state.advance(dt, sense, &tuning.0.policy);
        pilot.supported = supported;
        pilot.supine = supine;
        brain.supported = supported;
        let strength = match phase {
            Phase::Brace => 0.65,
            Phase::Roll => 0.9,
            Phase::Slide => 0.18,
            _ => 0.0,
        };
        if strength <= 0.0 {
            continue;
        }
        let mut budget = ((tuning.0.policy.max_work_j - pilot.state.work).max(0.0))
            .min(tuning.0.policy.power_w * f64::from(dt));
        // Reference-style support-reaction guide: only the fixed supporting
        // world supplies the omitted opposite reaction. No linear motion or
        // transforms are written; exact work shares the same finite ledger.
        pilot.guide_blocked = false;
        let guide_strength = pilot.state.roll_strength();
        if phase == Phase::Roll && supported && tuning.0.guide_torque > 0.0 && guide_strength > 0.0
        {
            let axis = (root.pose.rotation * Vec3::Y)
                .reject_from_normalized(normal)
                .normalize_or_zero();
            let front = root.pose.rotation * Vec3::Z;
            if axis.length_squared() > 0.5 {
                let a = front.reject_from_normalized(axis).normalize_or_zero();
                let b = normal.reject_from_normalized(axis).normalize_or_zero();
                let mut angle = axis.dot(a.cross(b)).atan2(a.dot(b));
                if angle.abs() > 2.6 {
                    angle = angle.abs() * pilot.state.direction;
                }
                let desired_rate = (angle * 2.5).clamp(-tuning.0.guide_rate, tuning.0.guide_rate);
                let next_rotation =
                    Quat::from_axis_angle(axis, desired_rate * dt) * root.pose.rotation;
                let mut blocked = false;
                if let Some(context) = &context {
                    let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                        !rig.parts.iter().flatten().any(|b| b.entity == e)
                            && !rig.support_surfaces.contains(&e)
                    };
                    context.intersect_shape(
                        root.pose.translation,
                        next_rotation,
                        shape(specs()[0]).raw.as_ref(),
                        QueryFilter::default()
                            .exclude_sensors()
                            .predicate(&predicate),
                        |_, _| {
                            blocked = true;
                            false
                        },
                    );
                }
                pilot.guide_blocked = blocked;
                if !blocked {
                    // Coherent angular-only guide across the articulated rig.
                    // Each share uses its world inertia about the roll axis;
                    // summed torque and positive work stay globally bounded.
                    let weights: [f32; 16] = std::array::from_fn(|i| {
                        rig.parts[i]
                            .map_or(0.0, |body| axis.dot(body.inverse.inverse() * axis).max(0.0))
                    });
                    let total = weights.iter().sum::<f32>();
                    if total > 1e-6 {
                        let guide_budget = budget * 0.7 * f64::from(guide_strength);
                        for (part, weight) in weights.into_iter().enumerate() {
                            let Some(body) = rig.parts[part] else {
                                continue;
                            };
                            let share = f64::from(weight / total);
                            let want = body.omega + axis * (desired_rate - body.omega.dot(axis));
                            if let Some(result) = rr_core::velocity_muscles::support_drive(
                                array(body.omega),
                                tensor(body.inverse),
                                array(want),
                                f64::from(tuning.0.guide_torque * dt * guide_strength) * share,
                                guide_budget * share,
                            ) {
                                let delta = as_vec(result.child_delta);
                                if delta.length_squared() > 1e-10
                                    && let Ok(mut v) = velocities.get_mut(body.entity)
                                {
                                    v.angular += delta;
                                    if let Some(record) = rig.parts[part].as_mut() {
                                        record.omega = v.angular;
                                    }
                                    budget = (budget - result.positive_work).max(0.0);
                                    pilot.state.work = (pilot.state.work + result.positive_work)
                                        .min(tuning.0.policy.max_work_j);
                                    pilot.guide_impulses += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if pilot.guide_blocked {
            pilot.state.obstructed(tuning.0.policy.retry_seconds);
            continue;
        }
        for part in 1..16 {
            let Some(child) = rig.parts[part] else {
                continue;
            };
            let Some(parent_index) = specs()[part].parent else {
                continue;
            };
            let Some(parent) = rig.parts[parent_index] else {
                continue;
            };
            let c = &settings.config.joints[part];
            let target = limited_rotation(
                desired(
                    part,
                    child.target,
                    parent.pose.rotation,
                    root.pose,
                    normal,
                    velocity,
                    phase,
                    pilot.state.direction,
                ),
                c,
            );
            let world_target = parent.pose.rotation * target;
            let target_rate = (error(world_target * child.pose.rotation.inverse())
                * tuning.0.frequency
                * strength)
                .clamp_length_max(tuning.0.max_rate);
            let relative = child.omega - parent.omega;
            let correction = ((target_rate - relative)
                * (1.0 - (-tuning.0.frequency * tuning.0.damping * dt * 6.0).exp()))
            .clamp_length_max(tuning.0.max_delta_rate);
            // Active torque works against retained passive tissue resistance.
            // A bounded feed-forward term compensates part of that load rather
            // than silently disabling native passive springs/damping.
            let current = parent.pose.rotation.inverse() * child.pose.rotation;
            let local_speed = parent.pose.rotation.inverse() * target_rate;
            let mut tissue = (coordinates(current) - coordinates(rotation(c.rest))) * c.passive_k
                + local_speed * c.passive_d;
            if c.hinge {
                tissue.y = 0.0;
                tissue.z = 0.0;
            }
            tissue = tissue.clamp_length_max(c.passive_torque) * strength;
            if relative.length() > target_rate.length() + 1.0 {
                tissue = Vec3::ZERO;
            }
            let feed = parent.pose.rotation * tissue * dt;
            let wanted = relative + correction + (child.inverse + parent.inverse) * feed;
            let max_impulse =
                f64::from((tuning.0.max_torque.min(c.active_torque) * strength * dt).max(0.0));
            let Some(result) = rr_core::velocity_muscles::drive(
                array(child.omega),
                array(parent.omega),
                tensor(child.inverse),
                tensor(parent.inverse),
                array(wanted),
                max_impulse,
                budget,
            ) else {
                continue;
            };
            let dc = as_vec(result.child_delta);
            let dp = as_vec(result.parent_delta);
            if dc.length_squared() + dp.length_squared() < 1e-10 {
                continue;
            }
            // Exact reciprocal angular impulses. Linear COM velocities are untouched.
            if let Ok([mut cv, mut pv]) = velocities.get_many_mut([child.entity, parent.entity]) {
                cv.angular += dc;
                pv.angular += dp;
                if let Some(b) = rig.parts[part].as_mut() {
                    b.omega = cv.angular;
                }
                if let Some(b) = rig.parts[parent_index].as_mut() {
                    b.omega = pv.angular;
                }
                budget = (budget - result.positive_work).max(0.0);
                pilot.state.work =
                    (pilot.state.work + result.positive_work).min(tuning.0.policy.max_work_j);
                pilot.impulses += 1;
            }
        }
    }
}

/// Comparison HUD exposes measured work and retry phase, not a claim of success.
pub fn hud(
    tuning: Res<Tuning>,
    riders: Query<&crate::game::Rider>,
    brains: Query<(&OwnedBody, &Pilot)>,
    mut texts: Query<&mut Text>,
) {
    let status = brains
        .iter()
        .find(|(o, _)| riders.get(o.owner).is_ok_and(|r| r.id == 0))
        .map_or("RIDING".into(), |(_, p)| {
            format!(
                "{:?} / work {:.1}/{:.0} J / attempts {} / supported {} / guide {} / blocked {}",
                p.state.phase,
                p.state.work,
                tuning.0.policy.max_work_j,
                p.state.attempts,
                p.supported,
                p.guide_impulses,
                p.guide_blocked
            )
        });
    for mut text in &mut texts {
        if text.0.starts_with(crate::crash_behavior::METHOD) {
            let top = text.0.lines().take(2).collect::<Vec<_>>().join("\n");
            let next = format!("{top}\n{status}");
            if text.0 != next {
                text.0 = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    fn capture(app: &mut App) -> (HashMap<Entity, Vec3>, Vec3, f64, u64) {
        let mut linear = HashMap::new();
        let mut momentum = Vec3::ZERO;
        let mut energy = 0.0;
        for (e, t, m, v) in app
            .world_mut()
            .query_filtered::<(Entity, &Transform, &ReadMassProperties, &Velocity), With<Segment>>()
            .iter(app.world())
        {
            linear.insert(e, v.linear);
            let frame = Mat3::from_quat(t.rotation * m.principal_inertia_local_frame);
            let inertia = frame * Mat3::from_diagonal(m.principal_inertia) * frame.transpose();
            momentum += inertia * v.angular;
            energy += f64::from(0.5 * v.angular.dot(inertia * v.angular));
        }
        let impulses = app
            .world_mut()
            .query::<&Pilot>()
            .iter(app.world())
            .map(|p| p.impulses)
            .sum();
        (linear, momentum, energy, impulses)
    }
    #[test]
    fn real_backend_drive_changes_angular_motion_without_authored_linear_motion() {
        let mut app = lab::fixture(true, lab::Scenario::Drop);
        for _ in 0..5 {
            app.update();
        }
        let before = capture(&mut app);
        let work_before: f64 = app
            .world_mut()
            .query::<&Pilot>()
            .iter(app.world())
            .map(|p| p.state.work)
            .sum();
        app.world_mut().run_system_once(update).unwrap();
        let after = capture(&mut app);
        let work_after: f64 = app
            .world_mut()
            .query::<&Pilot>()
            .iter(app.world())
            .map(|p| p.state.work)
            .sum();
        assert_eq!(before.0, after.0, "muscles authored linear velocity");
        assert!(
            before.1.distance(after.1) < 0.0002,
            "angular momentum changed: {:?} -> {:?}",
            before.1,
            after.1
        );
        assert!(after.3 > before.3, "controller was inactive");
        assert!(after.2 - before.2 <= work_after - work_before + 0.0002);
        assert!(
            work_after - work_before
                <= app.world().resource::<Tuning>().0.policy.power_w / 60.0 + 0.00001
        );
        for p in app.world_mut().query::<&Pilot>().iter(app.world()) {
            assert!(p.state.work <= 180.0);
        }
    }
    #[test]
    fn wall_only_contacts_never_enable_supine_support() {
        let mut app = lab::fixture(true, lab::Scenario::Wall);
        let floor: Vec<_> = app
            .world_mut()
            .query_filtered::<(Entity, &Transform), With<Collider>>()
            .iter(app.world())
            .filter(|(_, t)| t.translation.y < 0.0)
            .map(|(e, _)| e)
            .collect();
        for e in floor {
            app.world_mut().despawn(e);
        }
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(20.0, 20.0, 0.5),
            Transform::from_xyz(0.0, 10.0, 3.0),
        ));
        let mut saw_contact = false;
        for _ in 0..45 {
            app.update();
            saw_contact |= app
                .world_mut()
                .run_system_once(
                    |context: ReadRapierContext, segments: Query<(Entity, &Segment)>| {
                        let Ok(c) = context.single() else {
                            return false;
                        };
                        segments
                            .iter()
                            .filter(|(_, s)| matches!(s.part, 0 | 2 | 11))
                            .any(|(e, _)| {
                                c.contact_pairs_with(e).any(|p| p.has_any_active_contact())
                            })
                    },
                )
                .unwrap();
            assert!(
                app.world_mut()
                    .query::<&Pilot>()
                    .iter(app.world())
                    .all(|p| !p.supported)
            );
        }
        assert!(saw_contact, "wall fixture never made real contact");
    }
    #[test]
    fn supported_prone_slide_runs_a_bounded_recovery_attempt() {
        let mut app = lab::fixture(true, lab::Scenario::Drop);
        for _ in 0..5 {
            app.update();
        }
        let root = app
            .world_mut()
            .query::<(&Transform, &Segment)>()
            .iter(app.world())
            .find(|(_, s)| s.part == 0)
            .unwrap()
            .0
            .translation;
        let turn = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        let mut query = app
            .world_mut()
            .query_filtered::<(&mut Transform, &mut Velocity), With<Segment>>();
        for (mut pose, mut velocity) in query.iter_mut(app.world_mut()) {
            pose.translation = Vec3::new(0.0, 0.4, 0.0) + turn * (pose.translation - root);
            pose.rotation = turn * pose.rotation;
            *velocity = Velocity::linear(Vec3::Z * 8.0);
        }
        for mut pilot in app
            .world_mut()
            .query::<&mut Pilot>()
            .iter_mut(app.world_mut())
        {
            *pilot = Pilot::default();
        }
        let mut rolled = false;
        let mut observed_support = false;
        let mut best = -1.0f32;
        let mut guided = false;
        let mut phases = std::collections::BTreeMap::new();
        let mut max_guides = 0;
        let mut max_work = 0.0f64;
        for _ in 0..240 {
            app.update();
            for p in app.world_mut().query::<&Pilot>().iter(app.world()) {
                rolled |= p.state.attempts > 0;
                guided |= p.guide_impulses > 0;
                *phases
                    .entry(format!("{:?}", p.state.phase))
                    .or_insert(0usize) += 1;
                max_guides = max_guides.max(p.guide_impulses);
                max_work = max_work.max(p.state.work);
                observed_support |= p.supported;
                best = best.max(p.supine);
                assert!(p.state.work <= 180.00001 && p.state.attempts <= 2);
            }
            for v in app.world_mut().query::<&Velocity>().iter(app.world()) {
                assert!(v.linear.is_finite() && v.angular.is_finite());
            }
        }
        println!(
            "METHOD_B_PRONE observed_support={observed_support} attempts_started={rolled} best_supine={best} guide_active={guided} guide_count={max_guides} work={max_work} phases={phases:?}"
        );
        assert!(
            observed_support && rolled && guided,
            "controller did not attempt on actual ground"
        );
    }
    #[test]
    fn baseline_rig_has_no_velocity_pilot_or_new_work() {
        let mut app = lab::fixture(false, lab::Scenario::Drop);
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(
            app.world_mut().query::<&Pilot>().iter(app.world()).count(),
            0
        );
    }
}
