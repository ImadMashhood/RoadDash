//! Fixed-tick direct melee. Geometry and consecutive-hit rules are shared by all targets.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{self, Recovery, RecoveryStage},
    sound::{AudioCue, Cue},
    world::Art,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Message)]
pub struct AttackAttempt(pub Entity, pub rr_core::AttackSide);
#[derive(Message)]
pub struct NpcHit(pub Entity, pub Vec3, pub bool);
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn melee(
    mut attempts: MessageReader<AttackAttempt>,
    mut commands: Commands,
    art: Res<Art>,
    mut race: ResMut<Race>,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        &Transform,
        &mut Visibility,
    )>,
    npcs: Query<(Entity, &crate::npc::Npc, &Transform)>,
    mut velocities: Query<&mut Velocity>,
    context: ReadRapierContext,
    mut npc_hits: MessageWriter<NpcHit>,
    mut audio: MessageWriter<AudioCue>,
) {
    if race.phase != Phase::Racing {
        attempts.clear();
        return;
    }
    for (_, mut rider, recovery, _, _) in &mut riders {
        if recovery.stage == RecoveryStage::Riding && rider.hits.expire(race.elapsed) {
            rider.health = 100.0;
        }
    }
    for &AttackAttempt(attacker, action) in attempts.read() {
        let side = action.sign();
        let Ok((_, rider, state, t, _)) = riders.get(attacker) else {
            continue;
        };
        if state.stage != RecoveryStage::Riding || state.stagger.is_some() {
            continue;
        }
        let kicking = matches!(action, rr_core::AttackSide::Kick);
        let player = rider.id == 0;
        let attacker_id = rider.id;
        let kick_strength = if player {
            8.0
        } else {
            8.0 * rider.ai_profile.strength / 7.0
        };
        let origin = t.translation + Vec3::Y * 1.45;
        let forward = t.rotation * Vec3::Z;
        let right = forward.cross(Vec3::Y).normalize_or_zero();
        let attacker_velocity = rider.motion_velocity;
        let mut nearest: Option<(Entity, f32, Vec3, bool)> = None;
        let mut consider = |entity: Entity, p: Vec3, is_npc: bool| {
            let delta = p - origin;
            let lateral = delta.dot(right);
            let longitudinal = delta.dot(forward);
            if !rr_core::direct_strike_geometry(
                lateral,
                longitudinal,
                delta.y,
                if kicking { lateral.signum() } else { side },
            ) {
                return;
            }
            let clear = context.single().is_ok_and(|ctx| {
                let predicate = |e| e != entity;
                ctx.cast_ray(
                    origin,
                    delta.normalize_or_zero(),
                    delta.length(),
                    true,
                    QueryFilter::default()
                        .exclude_rigid_body(attacker)
                        .exclude_sensors()
                        .predicate(&predicate),
                )
                .is_none()
            });
            if clear && nearest.is_none_or(|(_, distance, _, _)| delta.length_squared() < distance)
            {
                nearest = Some((entity, delta.length_squared(), p, is_npc));
            }
        };
        for (e, r, s, t, _) in &riders {
            if e != attacker
                && s.stage == RecoveryStage::Riding
                && s.stagger.is_none()
                && r.immunity <= 0.0
                && (t.rotation * Vec3::Z).dot(forward) > 0.7
            {
                consider(e, t.translation + Vec3::Y * 1.45, false);
            }
        }
        for (e, n, t) in &npcs {
            if n.active() {
                consider(
                    e,
                    t.translation
                        + Vec3::Y
                            * if n.kind == crate::npc::NpcKind::Cow {
                                1.0
                            } else {
                                1.3
                            },
                    true,
                );
            }
        }
        let Some((target, _, p, is_npc)) = nearest else {
            continue;
        };
        if kicking && let Ok((_, mut rider, _, _, _)) = riders.get_mut(attacker) {
            rider.attack_side = (p - origin).dot(right).signum();
        }
        audio.write(AudioCue::new(Cue::Punch, p));
        if is_npc {
            npc_hits.write(NpcHit(target, attacker_velocity, true));
            continue;
        }
        if let Ok((_, mut r, mut state, t, mut visible)) = riders.get_mut(target) {
            let column = rr_core::pursuit::personality(attacker_id);
            r.relationships[column] =
                (r.relationships[column] + r.ai_profile.hit_reaction).clamp(-1024.0, 1024.0);
            if kicking {
                let push = right * (p - origin).dot(right).signum() * kick_strength;
                if let Ok(mut velocity) = velocities.get_mut(target) {
                    velocity.linvel += push;
                }
                r.side_velocity += (p - origin).dot(right).signum() * kick_strength;
                r.motion_velocity += push;
                if player {
                    race.message = "KICK".into();
                    race.message_time = 1.0;
                }
                continue;
            }
            let knocked_out = r.hits.direct_hit(race.elapsed);
            r.health = 100.0 * (3 - r.hits.count) as f32 / 3.0;
            if player {
                race.message = format!("DIRECT HIT / {} OF 3", r.hits.count);
                race.message_time = 1.0;
            }
            if knocked_out {
                if state.active_crash_behavior {
                    if state.stagger.is_none() {
                        state.stagger = Some(rr_core::crash_behavior::Stagger::new(
                            (p - origin).dot(right),
                        ));
                    }
                } else {
                    recovery::crash(
                        &mut commands,
                        &art,
                        target,
                        &mut r,
                        &mut state,
                        t.translation,
                        t.rotation,
                        Vec3::ZERO,
                        &context,
                    );
                    *visible = Visibility::Hidden;
                }
                audio.write(AudioCue::new(Cue::Crash, t.translation));
                if player {
                    race.takedowns += 1;
                    race.message = "TAKEDOWN / +100 CREDITS".into();
                    race.message_time = 2.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    #[test]
    fn kick_selects_nearest_side_pushes_without_damage_and_animates_that_leg() {
        for side in [-1.0, 1.0] {
            let mut app = crate::game::tests::simulation();
            for _ in 0..3 {
                app.update();
            }
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let t = *app.world().get::<Transform>(owner).unwrap();
            let right = (t.rotation * Vec3::Z).cross(Vec3::Y).normalize();
            let mut targets = Vec::new();
            for (id, distance) in [(1, side * 1.4), (2, -side * 2.0)] {
                let mut rider = Rider::new(id);
                rider.immunity = 0.0;
                targets.push(
                    app.world_mut()
                        .spawn((
                            rider,
                            Transform::from_translation(t.translation + right * distance)
                                .with_rotation(t.rotation),
                            Visibility::Visible,
                            Velocity::zero(),
                        ))
                        .id(),
                );
            }
            app.world_mut()
                .write_message(AttackAttempt(owner, rr_core::AttackSide::Kick));
            app.world_mut().run_system_once(melee).unwrap();
            assert!(
                app.world()
                    .get::<Velocity>(targets[0])
                    .unwrap()
                    .linvel
                    .dot(right)
                    * side
                    > 7.9
            );
            assert_eq!(
                app.world().get::<Velocity>(targets[1]).unwrap().linvel,
                Vec3::ZERO
            );
            assert_eq!(app.world().get::<Rider>(targets[0]).unwrap().health, 100.0);
            assert_eq!(app.world().get::<Rider>(owner).unwrap().attack_side, side);
        }
    }
    #[test]
    fn decisive_punch_stays_mounted_through_both_swerve_phases_and_counts_once() {
        let mut app = crate::game::tests::simulation();
        app.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = true;
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..3 {
            app.update();
        }
        let target = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let pose = *app.world().get::<Transform>(target).unwrap();
        let right = (pose.rotation * Vec3::Z).cross(Vec3::Y).normalize();
        app.world_mut().get_mut::<Rider>(target).unwrap().immunity = 0.0;
        let attacker = app
            .world_mut()
            .spawn((
                Rider::new(1),
                Transform::from_translation(pose.translation - right * 1.4)
                    .with_rotation(pose.rotation),
                Visibility::Visible,
                Velocity::zero(),
            ))
            .id();
        for _ in 0..3 {
            app.world_mut().resource_mut::<Race>().elapsed += 0.30;
            app.world_mut()
                .write_message(AttackAttempt(attacker, rr_core::AttackSide::Right));
            app.world_mut().run_system_once(melee).unwrap();
        }
        assert_eq!(app.world().get::<Rider>(target).unwrap().health, 0.0);
        assert!(
            app.world()
                .get::<Recovery>(target)
                .unwrap()
                .stagger
                .is_some()
        );
        app.world_mut().resource_mut::<Race>().elapsed += 0.30;
        app.world_mut()
            .write_message(AttackAttempt(attacker, rr_core::AttackSide::Right));
        app.world_mut().run_system_once(melee).unwrap();
        assert_eq!(
            app.world().get::<Recovery>(target).unwrap().stage,
            RecoveryStage::Riding
        );
        app.world_mut().despawn(attacker);
        let mut away = false;
        let mut counter = false;
        for tick in 0..85 {
            app.update();
            let rec = app.world().get::<Recovery>(target).unwrap();
            let r = app.world().get::<Rider>(target).unwrap();
            if tick < 30 {
                assert_eq!(
                    rec.stage,
                    RecoveryStage::Riding,
                    "early release tick={tick}"
                );
            }
            away |= r.drive_intent.steer > 0.2;
            counter |= r.drive_intent.steer < -0.2;
        }
        assert!(away && counter);
        assert_eq!(
            app.world().get::<Recovery>(target).unwrap().stage,
            RecoveryStage::Falling
        );
    }
}
