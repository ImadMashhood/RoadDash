//! Immutable scenery needs a solid collider, but no fixed-body integration state.
//! Keep every collider on its original entity with unchanged shape and roles.
use super::HillsSolid;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default, serde::Serialize)]
pub struct StaticCollision {
    pub bodyless_solids: usize,
    pub fixed_body_control: bool,
}
fn convert(world: &mut World, control: bool) {
    let entities: Vec<_> = world
        .query_filtered::<(Entity, &RigidBody), (With<HillsSolid>, With<Collider>)>()
        .iter(world)
        .filter_map(|(e, b)| (*b == RigidBody::Fixed).then_some(e))
        .collect();
    if !control {
        for entity in &entities {
            world.entity_mut(*entity).remove::<RigidBody>();
        }
    }
    world.insert_resource(StaticCollision {
        bodyless_solids: if control { 0 } else { entities.len() },
        fixed_body_control: control,
    });
}
pub fn queue(commands: &mut Commands) {
    let control = std::env::var("ROADDASH_HILLS_FIXED_BODIES").as_deref() == Ok("1");
    commands.queue(move |world: &mut World| convert(world, control));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion_preserves_entities_shapes_roles_and_moving_bodies() {
        let mut world = World::new();
        let pose = Transform::from_xyz(7., 8., 9.).with_scale(Vec3::new(2., 3., 4.));
        let solid = world
            .spawn((
                HillsSolid,
                RigidBody::Fixed,
                Collider::cuboid(1., 2., 3.),
                pose,
                crate::world::Ground,
                Friction::coefficient(0.85),
            ))
            .id();
        let moving = world
            .spawn((
                HillsSolid,
                RigidBody::KinematicPositionBased,
                Collider::ball(1.),
            ))
            .id();
        let other = world.spawn((RigidBody::Fixed, Collider::ball(1.))).id();
        convert(&mut world, false);
        assert!(world.get::<RigidBody>(solid).is_none());
        assert!(world.get::<crate::world::Ground>(solid).is_some());
        assert_eq!(world.get::<Transform>(solid).unwrap(), &pose);
        assert_eq!(
            world
                .get::<Collider>(solid)
                .unwrap()
                .as_cuboid()
                .unwrap()
                .half_extents(),
            Vec3::new(1., 2., 3.)
        );
        assert_eq!(world.get::<Friction>(solid).unwrap().coefficient, 0.85);
        assert_eq!(
            world.get::<RigidBody>(moving),
            Some(&RigidBody::KinematicPositionBased)
        );
        assert_eq!(world.get::<RigidBody>(other), Some(&RigidBody::Fixed));
        assert_eq!(world.resource::<StaticCollision>().bodyless_solids, 1);
    }
}

#[cfg(test)]
mod physical_equivalence {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    fn physical_fixture(control: bool) -> (Vec3, Vec3, f32, bool) {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ));
        let floor = app
            .world_mut()
            .spawn((
                HillsSolid,
                RigidBody::Fixed,
                Collider::cuboid(12., 0.5, 12.),
                Transform::from_xyz(0., -0.5, 0.),
                crate::world::Ground,
                Friction::coefficient(0.85),
            ))
            .id();
        let wall = app
            .world_mut()
            .spawn((
                HillsSolid,
                RigidBody::Fixed,
                Collider::cuboid(0.5, 0.5, 0.5),
                Transform::from_xyz(4., 2., 0.).with_scale(Vec3::new(0.4, 4., 8.)),
                crate::world::Obstacle,
                Friction::coefficient(0.8),
                ActiveEvents::COLLISION_EVENTS,
            ))
            .id();
        let resting = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                Collider::ball(0.4),
                Transform::from_xyz(-2., 4., 0.),
                Velocity::zero(),
                Friction::coefficient(0.85),
            ))
            .id();
        let moving = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                Collider::cuboid(0.35, 0.35, 0.35),
                Transform::from_xyz(-3., 1., 2.),
                Velocity::linear(Vec3::X * 12.),
                Ccd::enabled(),
                Friction::coefficient(0.3),
            ))
            .id();
        convert(app.world_mut(), control);
        for _ in 0..240 {
            app.update();
        }
        let resting = app.world().get::<Transform>(resting).unwrap().translation;
        let moving = app.world().get::<Transform>(moving).unwrap().translation;
        let (height, wall_hit) = app
            .world_mut()
            .run_system_once(move |context: ReadRapierContext| {
                let context = context.single().unwrap();
                let filter = |e, _: &bevy_rapier3d::rapier::geometry::Collider| e == floor;
                let distance = context
                    .cast_ray(
                        Vec3::new(0., 4., 0.),
                        Vec3::NEG_Y,
                        10.,
                        true,
                        QueryFilter::default().predicate(&filter),
                    )
                    .unwrap()
                    .1;
                let filter = |e, _: &bevy_rapier3d::rapier::geometry::Collider| e == wall;
                let wall_hit = context
                    .cast_ray(
                        Vec3::new(0., 2., 0.),
                        Vec3::X,
                        10.,
                        true,
                        QueryFilter::default().predicate(&filter),
                    )
                    .is_some();
                (4. - distance, wall_hit)
            })
            .unwrap();
        (resting, moving, height, wall_hit)
    }
    #[test]
    fn standalone_static_shapes_preserve_support_scaled_wall_contact_and_queries() {
        let fixed = physical_fixture(true);
        let standalone = physical_fixture(false);
        for result in [fixed, standalone] {
            assert!(
                (result.0.y - 0.4).abs() < 0.03,
                "dynamic body must settle on actual floor: {result:?}"
            );
            assert!(
                result.1.x < 3.6 && result.1.x > -1.,
                "fast moving body must hit scaled solid wall: {result:?}"
            );
            assert!(result.2.abs() < 0.001 && result.3);
        }
        assert!(fixed.0.distance(standalone.0) < 0.005);
        assert!(fixed.1.distance(standalone.1) < 0.02);
        assert_eq!(fixed.2, standalone.2);
    }
    fn rider_collision(
        control: bool,
        adapter: crate::vehicle::VehiclePhysicsAdapter,
    ) -> (bool, u32, Vec3) {
        use crate::{
            game::{Phase, Race, Rider},
            recovery::{Recovery, RecoveryStage},
        };
        let mut app = crate::game::tests::simulation();
        app.insert_resource(adapter);
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let (p, q) = app
            .world()
            .resource::<crate::track::Track>()
            .sample(-8., -2.);
        *app.world_mut().get_mut::<Transform>(owner).unwrap() =
            Transform::from_translation(p).with_rotation(q);
        app.world_mut().get_mut::<Rider>(owner).unwrap().immunity = 0.;
        app.world_mut().spawn((
            HillsSolid,
            RigidBody::Fixed,
            Collider::cuboid(20., 2.5, 0.3),
            Transform::from_translation(p + q * Vec3::Z * 22. + Vec3::Y * 2.5).with_rotation(q),
            crate::world::Obstacle,
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ));
        convert(app.world_mut(), control);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for tick in 0..360 {
            app.update();
            if app.world().get::<Recovery>(owner).unwrap().stage != RecoveryStage::Riding {
                return (
                    true,
                    tick,
                    app.world().get::<Transform>(owner).unwrap().translation,
                );
            }
        }
        let position = app.world().get::<Transform>(owner).unwrap().translation;
        let advance = (position - p).dot(q * Vec3::Z);
        assert!(
            (17. ..22.).contains(&advance),
            "unchanged Assisted control must actually reach and stop at the wall: advance={advance}, pose={position:?}"
        );
        (false, 360, position)
    }
    #[test]
    fn standalone_solid_wall_preserves_native_knockoff_and_assisted_blocking() {
        for adapter in [
            crate::vehicle::VehiclePhysicsAdapter::Dynamic,
            crate::vehicle::VehiclePhysicsAdapter::Assisted,
        ] {
            let fixed = rider_collision(true, adapter);
            let standalone = rider_collision(false, adapter);
            assert_eq!(
                fixed.0, standalone.0,
                "recovery behavior changed for {adapter:?}"
            );
            if adapter == crate::vehicle::VehiclePhysicsAdapter::Dynamic {
                assert!(fixed.0, "native dynamic bike must knock off");
            }
            assert!(
                fixed.1.abs_diff(standalone.1) <= 1,
                "contact timing changed ({adapter:?}): {fixed:?} {standalone:?}"
            );
            assert!(
                fixed.2.distance(standalone.2) < 0.1,
                "contact placement changed ({adapter:?}): {fixed:?} {standalone:?}"
            );
        }
    }
}
