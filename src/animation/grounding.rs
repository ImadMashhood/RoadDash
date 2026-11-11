//! Renderer-only stance locking and sole alignment against existing solid supports.
use super::*;
use bevy_rapier3d::prelude::*;
#[derive(Clone, Copy)]
pub(super) struct Plant {
    support: Entity,
    local_point: Vec3,
    local_normal: Vec3,
}
impl Plant {
    fn new(support: Entity, frame: Transform, point: Vec3, normal: Vec3) -> Self {
        Self {
            support,
            local_point: frame.to_matrix().inverse().transform_point3(point),
            local_normal: frame.rotation.inverse() * normal,
        }
    }
    fn world(self, frame: Transform) -> (Vec3, Vec3) {
        (
            frame.transform_point(self.local_point),
            frame.rotation * self.local_normal,
        )
    }
}
/// Use the physics collider frame, including parented/kinematic supports. Render
/// GlobalTransform propagation may lag the fixed step and is not a support frame.
fn support_frame(ctx: &RapierContext<'_>, entity: Entity) -> Option<Transform> {
    let handle = ctx.colliders.entity2collider().get(&entity)?;
    let p = ctx.colliders.colliders.get(*handle)?.position();
    let q = p.rotation.quaternion();
    Some(
        Transform::from_translation(Vec3::new(p.translation.x, p.translation.y, p.translation.z))
            .with_rotation(Quat::from_xyzw(q.i, q.j, q.k, q.w)),
    )
}
#[allow(clippy::too_many_arguments)]
pub(super) fn apply(
    lib: &Library,
    a: &mut Animator,
    rec: &Recovery,
    body: &Transform,
    context: &ReadRapierContext,
    dt: f32,
) {
    let Some(runner) = rec.runner else { return };
    let Ok(ctx) = context.single() else { return };
    let anchor = body.mul_transform(Transform::from_xyz(0.0, -0.95, 0.0));
    let inverse = anchor.to_matrix().inverse();
    let frames = lib.world_pose(a.current);
    let phase = a.gait.phase;
    for limb in &lib.data.limbs {
        if limb.bit < 2 {
            continue;
        }
        let i = (limb.bit - 2) as usize;
        let stance = rec.stage == RecoveryStage::GettingUp
            || if rec.walk_velocity.with_y(0.).length() < 0.2 {
                // Let the settling pose bring feet under the hips before taking
                // idle locks; otherwise a stopped mid-swing stride stays spread.
                a.gait.speed < 0.2
            } else {
                locomotion::contact(phase, a.gait.speed, i)
            };
        let mut sole =
            anchor.transform_point(frames[limb.end].transform_point(Vec3::from_array(limb.offset)));
        // A new stance always captures a new contact, even if exponential release
        // from the previous stride has not reached exact zero.
        if stance && !a.plant_stance[i] {
            a.plants[i] = None;
            a.plant_weight[i] = 0.0;
        }
        a.plant_stance[i] = stance;
        let filter = || {
            QueryFilter::exclude_dynamic()
                .exclude_sensors()
                .exclude_collider(runner)
        };
        // Swing also samples the surface: an uphill/crest foot must clear the
        // ground before touchdown, not wait for its next stance lock. One bounded
        // ray per swing foot; physical runner/terrain transforms are read-only.
        if !stance && rec.stage == RecoveryStage::Running {
            let lift = locomotion::swing_lift(phase, a.gait.speed, i);
            let hit = ctx
                .cast_ray_and_get_normal(sole + Vec3::Y * 0.6, -Vec3::Y, 1.3, true, filter())
                .filter(|(_, h)| h.normal.y > 0.4);
            let required = hit.map_or(0., |(_, h)| {
                (0.615 - h.time_of_impact + lift).clamp(-0.35, 0.35)
            });
            // Rising support wins immediately; downward changes settle smoothly.
            a.swing_ground[i] =
                required.max(motion::approach(a.swing_ground[i], required, dt, 0.04));
            sole.y += a.swing_ground[i];
            a.current.planted[i] = Some(inverse.transform_point3(sole));
            if let Some((_, h)) = hit {
                a.current.normals[i] =
                    Quat::from_rotation_arc(Vec3::Y, anchor.rotation.inverse() * h.normal)
                        * a.current.normals[i];
            }
        } else {
            a.swing_ground[i] = 0.;
        }
        if let Some(plant) = a.plants[i] {
            let supported = support_frame(&ctx, plant.support)
                .map(|frame| plant.world(frame))
                .filter(|(point, normal)| normal.y > 0.4 && point.distance(sole) <= 0.9)
                .filter(|(point, _)| {
                    ctx.cast_ray_and_get_normal(
                        *point + Vec3::Y * 0.1,
                        -Vec3::Y,
                        0.23,
                        true,
                        filter(),
                    )
                    .is_some_and(|(entity, h)| entity == plant.support && h.normal.y > 0.4)
                });
            if supported.is_none() {
                // Despawned, unreachable or lost support cannot pin a foot in space.
                a.plants[i] = None;
                a.plant_weight[i] = 0.0;
            }
        }
        if stance && a.plants[i].is_none() {
            a.plants[i] = ctx
                .cast_ray_and_get_normal(sole + Vec3::Y * 0.45, -Vec3::Y, 1.0, true, filter())
                .filter(|(_, h)| h.normal.y > 0.4)
                .and_then(|(entity, h)| {
                    Some(Plant::new(
                        entity,
                        support_frame(&ctx, entity)?,
                        sole + Vec3::Y * (0.465 - h.time_of_impact),
                        h.normal,
                    ))
                });
        }
        a.plant_weight[i] = motion::approach(
            a.plant_weight[i],
            if stance && a.plants[i].is_some() {
                1.0
            } else {
                0.0
            },
            dt,
            if stance { 0.015 } else { 0.035 },
        );
        // Exponential decay never reaches zero. Finite release prevents stale
        // locks surviving a swing; the next stance independently reacquires.
        if !stance && a.plant_weight[i] < 0.02 {
            a.plants[i] = None;
            a.plant_weight[i] = 0.0;
        }
        if let Some((point, normal)) =
            a.plants[i].and_then(|p| support_frame(&ctx, p.support).map(|f| p.world(f)))
        {
            a.current.planted[i] =
                Some(inverse.transform_point3(sole.lerp(point, a.plant_weight[i])));
            a.current.normals[i] = a.current.normals[i]
                .lerp(anchor.rotation.inverse() * normal, a.plant_weight[i])
                .normalize_or_zero();
        }
    }
    // Reach-aware pelvis height is visual only. A back foot on an uphill support
    // must not be pulled off the ground by the capsule moving up the slope.
    // Lower only as far as required by the existing anatomical chain; recover
    // height smoothly after lift-off so contacts cannot produce a pelvis pop.
    let fitted = lib.world_pose(a.current);
    let mut required = 0f32;
    for limb in lib.data.limbs.iter().filter(|l| l.bit >= 2) {
        let side = (limb.bit - 2) as usize;
        if !a.plant_stance[side] || a.plant_weight[side] < 0.5 {
            continue;
        }
        if let Some(sole) = a.current.planted[side] {
            let ankle = sole - fitted[limb.end].rotation * Vec3::from_array(limb.offset);
            let hip = fitted[limb.upper].translation;
            let reach = Vec3::from_array(lib.data.offsets[limb.lower]).length()
                + Vec3::from_array(lib.data.offsets[limb.end]).length()
                - 0.008;
            let vertical = (reach * reach - (ankle - hip).xz().length_squared())
                .max(0.)
                .sqrt();
            required = required.min(ankle.y + vertical - hip.y);
        }
    }
    a.ground_drop = required
        .min(a.ground_drop * (-dt / 0.10).exp())
        .clamp(-0.18, 0.);
    a.current.root.y += a.ground_drop;
}
#[cfg(test)]
mod tests {
    use super::*;
    fn probe(
        lib: Res<Library>,
        mut q: Query<(&mut Animator, &Recovery)>,
        bodies: Query<&Transform>,
        context: ReadRapierContext,
    ) {
        for (mut a, rec) in &mut q {
            a.current = lib.standing();
            a.gait.phase = (rec.walk_distance / 3.2).rem_euclid(1.);
            a.gait.speed = 6.;
            apply(
                &lib,
                &mut a,
                rec,
                bodies.get(rec.runner.unwrap()).unwrap(),
                &context,
                1.0 / 60.0,
            );
        }
    }
    #[test]
    fn swing_replants_and_physical_support_motion_and_loss_are_observed() {
        use bevy::ecs::system::RunSystemOnce;
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .init_resource::<Library>()
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
        let support = app
            .world_mut()
            .spawn((
                RigidBody::KinematicPositionBased,
                Collider::cuboid(5.0, 0.1, 5.0),
                Transform::from_xyz(0.0, -0.1, 0.0),
            ))
            .id();
        let runner = app
            .world_mut()
            .spawn(Transform::from_xyz(0.0, 0.95, 0.0))
            .id();
        let rec = Recovery {
            runner: Some(runner),
            stage: RecoveryStage::Running,
            walk_velocity: Vec3::Z,
            ..default()
        };
        let owner = app.world_mut().spawn((Animator::default(), rec)).id();
        for _ in 0..3 {
            app.update();
        }
        for _ in 0..30 {
            app.world_mut().run_system_once(probe).unwrap();
        }
        let first = app.world().get::<Animator>(owner).unwrap().plants[0].expect("stance plants");
        app.world_mut()
            .get_mut::<Recovery>(owner)
            .unwrap()
            .walk_distance = 3.2 * 0.6;
        for _ in 0..30 {
            app.world_mut().run_system_once(probe).unwrap();
        }
        assert!(
            app.world().get::<Animator>(owner).unwrap().plants[0].is_none(),
            "swing must finitely release"
        );
        app.world_mut()
            .get_mut::<Transform>(runner)
            .unwrap()
            .translation
            .x += 0.3;
        app.world_mut()
            .get_mut::<Recovery>(owner)
            .unwrap()
            .walk_distance = 3.2;
        for _ in 0..30 {
            app.world_mut().run_system_once(probe).unwrap();
        }
        let second = app.world().get::<Animator>(owner).unwrap().plants[0].unwrap();
        assert!(
            (second.local_point.x - first.local_point.x - 0.3).abs() < 1e-5,
            "new stance must acquire current sole"
        );
        app.world_mut()
            .get_mut::<Transform>(support)
            .unwrap()
            .translation += Vec3::new(0.2, 0.1, 0.0);
        app.update();
        app.world_mut().run_system_once(probe).unwrap();
        let a = app.world().get::<Animator>(owner).unwrap();
        assert!(
            a.plants[0]
                .unwrap()
                .local_point
                .distance(second.local_point)
                < 1e-6,
            "moving support retains local lock"
        );
        let point = a.current.planted[0].unwrap();
        assert!(point.y > 0.05, "kinematic lift is followed: {point:?}");
        // A one-tick swing deliberately retains substantial release weight.
        // Reentry must still use a fresh stance, rather than the old lock.
        app.world_mut()
            .get_mut::<Recovery>(owner)
            .unwrap()
            .walk_distance = 3.2 * 1.6;
        app.world_mut().run_system_once(probe).unwrap();
        assert!(app.world().get::<Animator>(owner).unwrap().plant_weight[0] > 0.5);
        app.world_mut()
            .get_mut::<Transform>(runner)
            .unwrap()
            .translation
            .x += 0.4;
        app.world_mut()
            .get_mut::<Recovery>(owner)
            .unwrap()
            .walk_distance = 6.4;
        app.world_mut().run_system_once(probe).unwrap();
        let fresh = app.world().get::<Animator>(owner).unwrap().plants[0].unwrap();
        assert!(fresh.local_point.distance(second.local_point) > 0.05);
        // Idle/getup supports both soles; beginning locomotion releases the swing foot.
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::GettingUp;
        for _ in 0..30 {
            app.world_mut().run_system_once(probe).unwrap();
        }
        assert!(
            app.world()
                .get::<Animator>(owner)
                .unwrap()
                .plants
                .iter()
                .all(Option::is_some)
        );
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::Running;
        for _ in 0..30 {
            app.world_mut().run_system_once(probe).unwrap();
        }
        assert!(app.world().get::<Animator>(owner).unwrap().plants[0].is_some());
        assert!(app.world().get::<Animator>(owner).unwrap().plants[1].is_none());
        app.world_mut().despawn(support);
        app.update();
        app.world_mut().run_system_once(probe).unwrap();
        let a = app.world().get::<Animator>(owner).unwrap();
        assert!(a.plants[0].is_none(), "support loss clears airborne lock");
        assert_eq!(a.plant_weight[0], 0.0);
        // A non-walkable plane cannot recreate the lost stance contact.
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(5.0, 0.01, 5.0),
            Transform::from_rotation(Quat::from_rotation_z(1.3)),
        ));
        app.update();
        app.world_mut().run_system_once(probe).unwrap();
        assert!(app.world().get::<Animator>(owner).unwrap().plants[0].is_none());
    }
    #[test]
    fn plant_tracks_translated_and_rotated_support_without_world_drift() {
        let entity = Entity::from_bits(1);
        let frame = Transform::from_xyz(1.0, 0.5, -2.0).with_rotation(Quat::from_rotation_y(0.4));
        let local = Vec3::new(0.2, 0.1, -0.3);
        let plant = Plant::new(entity, frame, frame.transform_point(local), Vec3::Y);
        for n in 0..120 {
            let moving = Transform::from_xyz(n as f32 * 0.02, 0.8, 1.0)
                .with_rotation(Quat::from_rotation_y(n as f32 * 0.01));
            let (point, normal) = plant.world(moving);
            assert!(point.distance(moving.transform_point(local)) < 1e-6);
            assert!(normal.distance(Vec3::Y) < 1e-6);
        }
    }
}
