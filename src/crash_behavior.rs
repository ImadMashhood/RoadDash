//! Common A/B loss-of-balance/ejection lifecycle and comparison controls.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{BodyKind, OwnedBody, Recovery, RecoveryStage},
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct DormantChassis(pub Collider);
#[derive(Component)]
pub struct ReleaseGrace {
    pub wreck: Entity,
    pub remaining: f32,
}
#[derive(SystemParam)]
pub struct ReleaseHooks<'w, 's> {
    bodies: Query<'w, 's, &'static OwnedBody>,
    grace: Query<'w, 's, &'static ReleaseGrace>,
}
impl BevyPhysicsHooks for ReleaseHooks<'_, '_> {
    fn filter_contact_pair(
        &self,
        pair: PairFilterContextView,
    ) -> Option<bevy_rapier3d::rapier::prelude::SolverFlags> {
        use bevy_rapier3d::rapier::prelude::SolverFlags;
        let a = pair.collider1();
        let b = pair.collider2();
        if let (Ok(a_body), Ok(b_body)) = (self.bodies.get(a), self.bodies.get(b))
            && a_body.owner == b_body.owner
            && a_body.kind != b_body.kind
        {
            let wreck = if a_body.kind == BodyKind::Wreck { a } else { b };
            if self
                .grace
                .get(a_body.owner)
                .is_ok_and(|g| g.remaining > 0.0 && g.wreck == wreck)
            {
                return None;
            }
        }
        Some(SolverFlags::COMPUTE_RIGID_IMPULSES)
    }
}
pub fn prepare(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    settings: Res<crate::biomechanics::Settings>,
    mut riders: Query<(
        Entity,
        &mut Recovery,
        Option<&mut ReleaseGrace>,
        Option<&DormantChassis>,
    )>,
    shapes: Query<(&Transform, &Collider, &OwnedBody)>,
    context: ReadRapierContext,
) {
    for (entity, mut rec, grace, dormant) in &mut riders {
        if rec.stage == RecoveryStage::Riding
            && let Some(dormant) = dormant
        {
            commands
                .entity(entity)
                .insert(dormant.0.clone())
                .remove::<DormantChassis>();
        }
        rec.active_crash_behavior = settings.enabled;
        if let Some(mut grace) = grace {
            if race.phase == Phase::Racing {
                grace.remaining -= time.delta_secs();
            }
            let mut overlapping = false;
            let mut checked = false;
            if grace.remaining < 0.60
                && let Ok(context) = context.single()
            {
                let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| e == grace.wreck;
                for (pose, collider, body) in &shapes {
                    if body.owner == entity && body.kind == BodyKind::Ragdoll {
                        checked = true;
                        context.intersect_shape(
                            pose.translation,
                            pose.rotation,
                            collider.raw.as_ref(),
                            QueryFilter::default().predicate(&predicate),
                            |_, _| {
                                overlapping = true;
                                false
                            },
                        );
                    }
                }
            }
            if grace.remaining <= 0.0
                || (checked && !overlapping)
                || rec.stage == RecoveryStage::Riding
                || !settings.enabled
            {
                commands.entity(entity).remove::<ReleaseGrace>();
            }
        }
        if !settings.enabled || rec.stage != RecoveryStage::Riding {
            rec.stagger = None;
        }
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn stagger(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    art: Res<crate::world::Art>,
    context: ReadRapierContext,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        &Transform,
        &mut Visibility,
        Option<&Velocity>,
    )>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    for (entity, mut rider, mut recovery, t, mut visible, velocity) in &mut riders {
        if recovery.stage != RecoveryStage::Riding {
            continue;
        }
        let Some(ref mut stagger) = recovery.stagger else {
            continue;
        };
        if stagger.origin.is_none() {
            stagger.origin = Some(t.translation.to_array());
            stagger.right = (t.rotation * Vec3::Z)
                .cross(Vec3::Y)
                .normalize_or_zero()
                .to_array();
        }
        let right = Vec3::from_array(stagger.right);
        let error = (t.translation - Vec3::from_array(stagger.origin.unwrap())).dot(right);
        let lateral = velocity
            .map_or(rider.motion_velocity, |v| v.linear)
            .dot(right);
        let (steer, release) = stagger.advance(time.delta_secs(), error, lateral);
        rider.drive_intent.steer = steer;
        rider.raw_steer = steer;
        if release {
            crate::recovery::crash(
                &mut commands,
                &art,
                entity,
                &mut rider,
                &mut recovery,
                t.translation,
                t.rotation,
                Vec3::ZERO,
                &context,
            );
            *visible = Visibility::Hidden;
        }
    }
}
#[derive(Resource, Default)]
struct Compare {
    next_speed: usize,
    pending: u8,
    speed_edge: bool,
}
#[derive(Component)]
pub(crate) struct CompareLabel;
pub const METHOD: &str = "VELOCITY MUSCLES + SUPPORT GUIDE";
pub fn install_comparison(app: &mut App) {
    if !std::env::args().any(|s| s == "--crash-comparison") {
        return;
    }
    app.insert_resource(crate::game::Persistence(false))
        .init_resource::<Compare>()
        .add_systems(Startup, comparison_label)
        .add_systems(PreUpdate, capture.after(bevy::input::InputSystems))
        .add_systems(
            FixedUpdate,
            comparison.before(crate::runtime::SimSet::Gameplay),
        );
}
fn comparison_label(mut commands: Commands) {
    commands.spawn((
        CompareLabel,
        Text::new(format!(
            "{METHOD}\nF8 combat stagger | F9 frontal release | F10 set 20/60/120/200 km/h"
        )),
        TextFont {
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.25)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.0),
            bottom: Val::Px(95.0),
            ..default()
        },
    ));
}
fn capture(keys: Res<ButtonInput<KeyCode>>, mut c: ResMut<Compare>) {
    if keys.just_pressed(KeyCode::F8) {
        c.pending = 8;
    }
    if keys.just_pressed(KeyCode::F9) {
        c.pending = 9;
    }
    c.speed_edge |= keys.just_pressed(KeyCode::F10);
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn comparison(
    mut commands: Commands,
    mut c: ResMut<Compare>,
    mut race: ResMut<Race>,
    art: Res<crate::world::Art>,
    context: ReadRapierContext,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        &Transform,
        &mut Velocity,
        &mut Visibility,
    )>,
    mut labels: Query<&mut Text, With<CompareLabel>>,
) {
    let action = std::mem::take(&mut c.pending);
    let speed_edge = std::mem::take(&mut c.speed_edge);
    if race.phase != Phase::Racing {
        return;
    }
    let Some((entity, mut rider, mut recovery, t, mut velocity, mut visible)) =
        riders.iter_mut().find(|(_, r, ..)| r.id == 0)
    else {
        return;
    };
    if recovery.stage != RecoveryStage::Riding {
        return;
    }
    if speed_edge {
        let kmh = [20.0, 60.0, 120.0, 200.0][c.next_speed];
        c.next_speed = (c.next_speed + 1) % 4;
        let forward = (t.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
        let v = forward * (kmh / 3.6);
        velocity.linear = v;
        rider.speed = kmh / 3.6;
        rider.motion_velocity = v;
        race.message = format!("{METHOD} / {kmh:.0} KM/H");
        race.message_time = 3.0;
        for mut label in &mut labels {
            label.0 = format!(
                "{METHOD} / {kmh:.0} KM/H\nF8 combat stagger | F9 frontal release | F10 next speed"
            );
        }
    }
    if action == 8 && !recovery.active_crash_behavior {
        crate::recovery::crash(
            &mut commands,
            &art,
            entity,
            &mut rider,
            &mut recovery,
            t.translation,
            t.rotation,
            Vec3::ZERO,
            &context,
        );
        *visible = Visibility::Hidden;
    } else if action == 8 && recovery.stagger.is_none() {
        recovery.stagger = Some(rr_core::crash_behavior::Stagger::new(1.0));
        race.message = format!("{METHOD} / COMBAT STAGGER");
        race.message_time = 2.0;
    } else if action == 9 {
        commands
            .entity(entity)
            .insert(crate::biomechanics::reactions::FrontalRelease);
        crate::recovery::crash(
            &mut commands,
            &art,
            entity,
            &mut rider,
            &mut recovery,
            t.translation,
            t.rotation,
            Vec3::ZERO,
            &context,
        );
        *visible = Visibility::Hidden;
    }
}

/// Restore the exact pre-crash chassis shape; the solid wreck is removed separately.
pub fn restore_chassis(commands: &mut Commands, owner: Entity) {
    commands.queue(move |world: &mut World| {
        if let Some(shape) = world.entity_mut(owner).take::<DormantChassis>() {
            world.entity_mut(owner).insert(shape.0);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn continuous_crash_has_no_ghost_chassis_and_reset_restores_exact_shape() {
        let mut app = crate::game::tests::simulation();
        app.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = true;
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let original = app
            .world()
            .get::<Collider>(owner)
            .unwrap()
            .raw
            .compute_local_aabb();
        app.world_mut().get_mut::<Rider>(owner).unwrap().health = 0.0;
        app.update();
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Falling
        );
        assert!(app.world().get::<Collider>(owner).is_none());
        assert!(app.world().get::<DormantChassis>(owner).is_some());
        let grace = app.world().get::<ReleaseGrace>(owner).unwrap().remaining;
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(
            app.world().get::<ReleaseGrace>(owner).unwrap().remaining,
            grace
        );
        app.world_mut()
            .resource_mut::<crate::controls::Controls>()
            .restart = true;
        app.update();
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        assert!(app.world().get::<DormantChassis>(owner).is_none());
        assert!(app.world().get::<ReleaseGrace>(owner).is_none());
        assert_eq!(
            app.world()
                .get::<Collider>(owner)
                .unwrap()
                .raw
                .compute_local_aabb(),
            original
        );
    }
}
