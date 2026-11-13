//! Fixed-tick direct melee. Geometry and consecutive-hit rules are shared by all targets.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{self, Recovery, RecoveryStage},
    sound::{AudioCue, Cue},
    world::Art,
};
mod contact;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
/// Only intentional police foot chase extends the existing mounted target set.
fn target_pose(
    r: &Rider,
    rec: &Recovery,
    t: &Transform,
    poses: &Query<&Transform, Without<Rider>>,
    police_foot: bool,
) -> Option<Transform> {
    if rec.stage == RecoveryStage::Riding {
        Some(*t)
    } else if r.police && police_foot && rec.stage == RecoveryStage::Running {
        rec.runner
            .and_then(|e| poses.get(e).ok())
            .map(|p| Transform {
                translation: p.translation - Vec3::Y * 0.95,
                ..*p
            })
    } else {
        None
    }
}
#[derive(Message)]
pub struct AttackAttempt(pub Entity, pub rr_core::AttackSide);
/// Input/AI intent. Only the fixed scheduler turns this into a contact attempt.
#[derive(Message)]
pub struct AttackIntent(pub Entity, pub rr_core::AttackSide);
#[derive(Message)]
pub struct StrikeContact(
    pub(crate) Entity,
    pub(crate) rr_core::AttackSide,
    pub(crate) f32,
);

#[derive(Resource)]
pub struct StrikeTimings([f32; 2]);
impl StrikeTimings {
    pub fn contact_after(&self, kick: bool, weapon: Option<rr_core::weapons::Ability>) -> f32 {
        rr_core::motion::StrikeClock::new(
            if kick {
                rr_core::AttackSide::Kick
            } else {
                rr_core::AttackSide::Right
            },
            1.,
            weapon.map_or(self.0[usize::from(kick)], |w| w.source_contact),
        )
        .contact
    }
}
impl Default for StrikeTimings {
    fn default() -> Self {
        let lib =
            rr_content::motion::Library::from_json(include_str!("../assets/models/motion.json"))
                .expect("validated strike clips");
        Self([
            lib.strike_contact(false).unwrap(),
            lib.strike_contact(true).unwrap(),
        ])
    }
}

/// Resolve the side at windup and evaluate contact until the strike hits or expires.
/// Rider-owned clocks reset with the rider; interrupted/recovered riders cannot retain hits.
#[allow(clippy::too_many_arguments)]
pub fn schedule(
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    timings: Res<StrikeTimings>,
    weapons: Res<crate::weapons::WeaponRules>,
    mut intents: MessageReader<AttackIntent>,
    mut riders: Query<(Entity, &mut Rider, &Recovery, &Transform)>,
    npcs: Query<(Entity, &crate::npc::Npc, &Transform)>,
    mut contacts: MessageWriter<StrikeContact>,
    positions: Query<&Transform, Without<Rider>>,
    police: Option<Res<crate::police::Police>>,
    mut audio: MessageWriter<AudioCue>,
) {
    if race.phase == Phase::Paused {
        intents.clear();
        return;
    }
    if race.phase != Phase::Racing {
        intents.clear();
        for (_, mut r, _, _) in &mut riders {
            r.strike = None;
            r.hit_reaction = None;
        }
        return;
    }
    let police_foot = police.is_some_and(|p| p.intentional_foot);
    for &AttackIntent(owner, action) in intents.read() {
        let Ok((_, r, rec, t)) = riders.get(owner) else {
            continue;
        };
        if rec.stage != RecoveryStage::Riding || rec.stagger.is_some() || r.finished.is_some() {
            continue;
        }
        if r.strike.is_some_and(|clock| clock.pending()) {
            continue;
        }
        let mut side = action.sign();
        if action == rr_core::AttackSide::Kick {
            let origin = t.translation;
            let right = (t.rotation * Vec3::Z).cross(Vec3::Y).normalize_or_zero();
            let eligible = riders
                .iter()
                .filter(|(e, other, state, pose)| {
                    *e != owner
                        && (r.human_controlled()
                            || (r.can_fight(other.id)
                                && r.combat_mind.target_id().is_none_or(|id| id == other.id)))
                        && target_pose(other, state, pose, &positions, police_foot).is_some()
                        && state.stagger.is_none()
                        && other.immunity <= 0.0
                        && other.finished.is_none()
                })
                .map(|(e, r, rec, t)| {
                    (
                        e,
                        target_pose(r, rec, t, &positions, police_foot)
                            .unwrap()
                            .translation,
                    )
                })
                .chain(
                    npcs.iter()
                        .filter(|(_, n, _)| {
                            n.active() && (r.human_controlled() || r.can_fight(usize::MAX))
                        })
                        .map(|(e, _, t)| (e, t.translation)),
                )
                .filter_map(|(e, p)| {
                    let delta = p - origin;
                    let lateral = delta.dot(right);
                    (delta.length_squared() < 144.0 && lateral.abs() > 0.05).then_some((
                        delta.length_squared(),
                        e,
                        lateral.signum(),
                    ))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
            if let Some((_, _, chosen)) = eligible {
                side = chosen;
            }
        }
        let weapon = r
            .weapon
            .filter(|_| action != rr_core::AttackSide::Kick)
            .map(|w| weapons.get(w));
        let source_contact = weapon.map_or(
            timings.0[usize::from(action == rr_core::AttackSide::Kick)],
            |a| a.source_contact,
        );
        if let Ok((_, mut r, _, _)) = riders.get_mut(owner) {
            r.attack_side = side;
            r.strike = Some(rr_core::motion::StrikeClock::new(
                action,
                side,
                source_contact,
            ));
            r.strike.as_mut().unwrap().weapon = weapon;
            if weapon.is_some() {
                r.weapon_hand = usize::from(side > 0.0);
            }
        }
    }
    for (e, mut r, rec, pose) in &mut riders {
        if rec.stage != RecoveryStage::Riding
            || rec.stagger.is_some()
            || r.finished.is_some()
            || r.hit_reaction.is_some_and(|hit| !hit.active(race.elapsed))
        {
            r.hit_reaction = None;
        }
        if rec.stage != RecoveryStage::Riding || rec.stagger.is_some() || r.finished.is_some() {
            r.strike = None;
            continue;
        }
        if let Some(mut clock) = r.strike {
            let dt = time.delta_secs();
            clock.step(dt);
            let (start, end) = clock.contact_window();
            if clock.pending() && clock.age >= start && clock.age - dt <= end {
                contacts.write(StrikeContact(e, clock.action, clock.side));
            } else if clock.pending() && clock.age > end {
                clock.consume();
                audio.write(AudioCue::new(Cue::Whiff, pose.translation + Vec3::Y));
            }
            if !r.human_controlled()
                && !r.police
                && clock.age >= end + 0.08
                && r.combat_mind.target_id().is_some()
            {
                r.combat_mind.finish_exchange();
            }
            r.strike = Some(clock);
        }
    }
}
#[derive(Message)]
pub struct NpcHit(pub Entity, pub Vec3, pub bool);
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn melee(
    mut attempts: MessageReader<AttackAttempt>,
    mut contacts: MessageReader<StrikeContact>,
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
    mut police: Option<ResMut<crate::police::Police>>,
    positions: Query<&Transform, Without<Rider>>,
    owned: Query<&crate::recovery::OwnedBody>,
    mut audio: MessageWriter<AudioCue>,
    (library, histories, animators, fixed): (
        Res<crate::animation::Library>,
        Query<&crate::runtime::PoseHistory>,
        Query<&crate::animation::Animator>,
        Res<Time<Fixed>>,
    ),
) {
    if race.phase != Phase::Racing {
        attempts.clear();
        contacts.clear();
        return;
    }
    let police_foot = police.as_ref().is_some_and(|p| p.intentional_foot);
    for (_, mut rider, recovery, _, _) in &mut riders {
        if (recovery.stage == RecoveryStage::Riding
            || (rider.police && police_foot && recovery.stage == RecoveryStage::Running))
            && rider.hits.expire(race.elapsed)
        {
            rider.health = 100.0;
        }
    }
    let attempts = attempts
        .read()
        .map(|&AttackAttempt(e, a)| (e, a, None))
        .chain(
            contacts
                .read()
                .map(|&StrikeContact(e, a, side)| (e, a, Some(side))),
        );
    for (attacker, action, chosen_side) in attempts {
        let side = chosen_side.unwrap_or(action.sign());
        let Ok((_, rider, state, t, _)) = riders.get(attacker) else {
            continue;
        };
        if state.stage != RecoveryStage::Riding
            || state.stagger.is_some()
            || rider.finished.is_some()
        {
            continue;
        }
        if chosen_side.is_some() && rider.strike.is_none_or(|clock| !clock.pending()) {
            continue;
        }
        let strike = rider.strike;
        let lean = rider.lean;
        let attacker_pose = *t;
        let weapon = chosen_side.and_then(|_| rider.strike.and_then(|s| s.weapon));
        let kicking = matches!(action, rr_core::AttackSide::Kick);
        let player = rider.human_controlled();
        let networked = rider.networked;
        let attacker_id = rider.id;
        let mind = rider.combat_mind.clone();
        let police_attacker = rider.police;
        let kick_strength = if player {
            8.0
        } else {
            8.0 * rider.ai_profile.strength / 7.0
        };
        let origin = t.translation + Vec3::Y * 1.45;
        let forward = t.rotation * Vec3::Z;
        let right = forward.cross(Vec3::Y).normalize_or_zero();
        let attacker_velocity = rider.motion_velocity;
        if kicking && chosen_side.is_none() {
            let direction = riders
                .iter()
                .filter(|(e, r, state, t, _)| {
                    (player
                        || (if police_attacker {
                            r.id == 0
                        } else {
                            mind.can_fight(r.id)
                        }))
                        && *e != attacker
                        && target_pose(r, state, t, &positions, police_foot)
                            .is_some_and(|p| p.translation.distance_squared(origin) < 144.)
                })
                .min_by(|a, b| {
                    a.3.translation
                        .distance_squared(origin)
                        .total_cmp(&b.3.translation.distance_squared(origin))
                })
                .map(|(_, _, _, t, _)| (t.translation - origin).dot(right))
                .filter(|side| side.abs() > 0.05);
            if let Some(direction) = direction
                && let Ok((_, mut rider, _, _, _)) = riders.get_mut(attacker)
            {
                rider.attack_side = direction.signum();
            }
        }
        let sweep = chosen_side.and(strike).map(|clock| {
            contact::Sweep::new(
                &library,
                clock,
                lean,
                histories
                    .get(attacker)
                    .map_or(attacker_pose, |h| contact::previous(h, attacker_pose)),
                attacker_pose,
                fixed.delta_secs(),
            )
        });
        let mut nearest: Option<(Entity, f32, Vec3, bool, bool)> = None;
        let mut consider =
            |entity: Entity, p: Vec3, is_npc: bool, model_hit: Option<(Vec3, bool)>| {
                let (p, bike_hit) = model_hit.unwrap_or((p, false));
                let delta = p - origin;
                let lateral = delta.dot(right);
                let longitudinal = delta.dot(forward);
                if model_hit.is_none()
                    && !weapon.map_or_else(
                        || {
                            rr_core::direct_strike_geometry(
                                lateral,
                                longitudinal,
                                delta.y,
                                if kicking && chosen_side.is_none() {
                                    lateral.signum()
                                } else {
                                    side
                                },
                            )
                        },
                        |a| a.reaches(lateral, longitudinal, delta.y, side),
                    )
                {
                    return;
                }
                let clear = context.single().is_ok_and(|ctx| {
                    let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                        e != entity
                            && !owned.get(e).is_ok_and(|b| {
                                b.owner == entity && b.kind == recovery::BodyKind::Runner
                            })
                    };
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
                if clear
                    && nearest
                        .is_none_or(|(_, distance, _, _, _)| delta.length_squared() < distance)
                {
                    nearest = Some((entity, delta.length_squared(), p, is_npc, bike_hit));
                }
            };
        for (e, r, s, t, _) in &riders {
            if e != attacker
                && (player
                    || (if police_attacker {
                        r.id == 0
                    } else {
                        mind.can_fight(r.id)
                    }))
                && target_pose(r, s, t, &positions, police_foot).is_some()
                && s.stagger.is_none()
                && r.immunity <= 0.0
                && r.finished.is_none()
                && (chosen_side.is_some()
                    || (r.police && police_foot && s.stage == RecoveryStage::Running)
                    || (t.rotation * Vec3::Z).dot(forward) > 0.7)
            {
                let model_hit = if let Some(sweep) = sweep.as_ref()
                    && s.stage == RecoveryStage::Riding
                {
                    let frames = animators.get(e).ok().filter(|a| a.ready).map_or_else(
                        || library.seated_frames(r.lean),
                        |a| library.world_pose(a.current),
                    );
                    let hit = sweep.hit(
                        histories.get(e).map_or(*t, |h| contact::previous(h, *t)),
                        *t,
                        &frames,
                    );
                    if hit.is_none()
                        && !strike.is_some_and(|clock| {
                            clock.weapon.is_some() && clock.age >= clock.contact
                        })
                    {
                        continue;
                    }
                    hit
                } else {
                    None
                };
                consider(
                    e,
                    target_pose(r, s, t, &positions, police_foot)
                        .unwrap()
                        .translation
                        + Vec3::Y * 1.45,
                    false,
                    model_hit,
                );
            }
        }
        for (e, n, t) in &npcs {
            if n.active() && player && strike.is_none_or(|s| s.age >= s.contact) {
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
                    None,
                );
            }
        }
        let Some((target, _, p, is_npc, bike_hit)) = nearest else {
            continue;
        };
        if chosen_side.is_some()
            && let Ok((_, mut rider, _, _, _)) = riders.get_mut(attacker)
            && let Some(clock) = rider.strike.as_mut()
        {
            clock.consume();
        }
        if player
            && let Some(ref mut police) = police
            && race.elapsed >= police.rules.launch_grace
            && let Some((observer, _, rec, t, _)) = riders.iter().find(|(_, r, _, _, _)| r.police)
        {
            let eye = rec
                .runner
                .and_then(|e| positions.get(e).ok())
                .map_or(t.translation + Vec3::Y * 1.2, |p| {
                    p.translation + Vec3::Y * 0.25
                });
            let delta = p - eye;
            let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                e != observer
                    && e != attacker
                    && e != target
                    && !owned.get(e).is_ok_and(|b| b.owner == observer)
            };
            let witnessed = delta.length() <= police.rules.detection_radius
                && context.single().ok().is_some_and(|ctx| {
                    ctx.cast_ray(
                        eye,
                        delta.normalize_or_zero(),
                        delta.length(),
                        true,
                        QueryFilter::default()
                            .exclude_sensors()
                            .predicate(&predicate),
                    )
                    .is_none()
                });
            if witnessed {
                police.assault = Some(race.elapsed);
            }
        }
        if kicking
            && chosen_side.is_none()
            && let Ok((_, mut rider, _, _, _)) = riders.get_mut(attacker)
        {
            rider.attack_side = (p - origin).dot(right).signum();
        }
        audio.write(AudioCue::new(
            if bike_hit {
                Cue::BikeStrike
            } else if kicking {
                Cue::Kick
            } else {
                Cue::Punch
            },
            p,
        ));
        if player || riders.get(target).is_ok_and(|(_, r, _, _, _)| r.id == 0) {
            race.shake = race.shake.max(if kicking { 0.16 } else { 0.10 });
        }
        if is_npc {
            npc_hits.write(NpcHit(target, attacker_velocity, true));
            continue;
        }
        if let Ok((_, mut r, mut state, t, mut visible)) = riders.get_mut(target) {
            let foot = r.police && police_foot && state.stage == RecoveryStage::Running;
            let hit_pose = target_pose(&r, &state, t, &positions, police_foot).unwrap_or(*t);
            if r.police
                && let Some(ref mut police) = police
            {
                police.mind.interrupt_arrest();
                police.contact_block_until = race.elapsed + 0.25;
            }
            if bike_hit {
                let direction = (t.translation - origin).dot(right).signum();
                let push = right * direction * if kicking { kick_strength } else { 1.5 };
                if let Ok(mut velocity) = velocities.get_mut(target) {
                    velocity.linear += push;
                }
                r.side_velocity += direction * push.length();
                r.motion_velocity += push;
                r.combat_mind.struck_by(attacker_id);
                if player {
                    race.message = "BIKE HIT".into();
                    race.message_time = 0.75;
                }
                continue;
            }
            if r.human_controlled() {
                race.message = if kicking { "KICKED!" } else { "HIT!" }.into();
                race.message_time = 0.85;
            }
            r.combat_mind.struck_by(attacker_id);
            let column = rr_core::pursuit::personality(attacker_id);
            r.relationships[column] =
                (r.relationships[column] + r.ai_profile.hit_reaction).clamp(-1024.0, 1024.0);
            // Confirmed contact only. Replaces the old pulse; never adds force or locks input.
            if !foot {
                let away = hit_pose.rotation.inverse() * (p - origin);
                r.hit_reaction = Some(rr_core::motion::HitReaction::new(
                    race.elapsed,
                    away.x,
                    kicking,
                ));
            }
            if kicking && foot {
                continue;
            }
            if kicking {
                let push = right * (p - origin).dot(right).signum() * kick_strength;
                if let Ok(mut velocity) = velocities.get_mut(target) {
                    velocity.linear += push;
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
                r.hit_reaction = None;
                if state.active_crash_behavior && !foot {
                    if state.stagger.is_none() {
                        state.stagger = Some(rr_core::crash_behavior::Stagger::new(
                            (p - origin).dot(right),
                        ));
                    }
                } else {
                    if foot {
                        r.motion_velocity = state.walk_velocity;
                        r.motion_angular_velocity = Vec3::ZERO;
                    }
                    recovery::crash(
                        &mut commands,
                        &art,
                        target,
                        &mut r,
                        &mut state,
                        hit_pose.translation,
                        hit_pose.rotation,
                        Vec3::ZERO,
                        &context,
                    );
                    *visible = Visibility::Hidden;
                }
                audio.write(AudioCue::new(Cue::Crash, t.translation));
                if player {
                    commands.queue(move |world: &mut World| {
                        if let Some(mut r) = world.get_mut::<Rider>(attacker) {
                            r.online_takedowns += 1;
                        }
                    });
                    if attacker_id == 0 {
                        race.takedowns += 1;
                    }
                    race.message = if networked {
                        "TAKEDOWN"
                    } else {
                        "TAKEDOWN / +100 CREDITS"
                    }
                    .into();
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
    fn confirmed_reactions_replace_not_stack_and_misses_pause_recovery_reset_are_safe() {
        for kicking in [false, true] {
            for side in [-1.0, 1.0] {
                let mut app = crate::game::tests::simulation();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                for _ in 0..3 {
                    app.update();
                }
                let owner = app
                    .world_mut()
                    .query_filtered::<Entity, With<Rider>>()
                    .single(app.world())
                    .unwrap();
                let pose = *app.world().get::<Transform>(owner).unwrap();
                let right = (pose.rotation * Vec3::Z).cross(Vec3::Y).normalize();
                let mut victim = Rider::new(1);
                victim.immunity = 0.0;
                let target = app
                    .world_mut()
                    .spawn((
                        victim,
                        Transform::from_translation(pose.translation + right * side * 1.4)
                            .with_rotation(pose.rotation),
                        Visibility::Visible,
                        Velocity::zero(),
                    ))
                    .id();
                let action = if kicking {
                    rr_core::AttackSide::Kick
                } else if side < 0.0 {
                    rr_core::AttackSide::Left
                } else {
                    rr_core::AttackSide::Right
                };
                app.world_mut().write_message(AttackAttempt(owner, action));
                app.world_mut().run_system_once(melee).unwrap();
                let hit = app
                    .world()
                    .get::<Rider>(target)
                    .unwrap()
                    .hit_reaction
                    .unwrap();
                assert_eq!(hit.direction, -side);
                assert_eq!(hit.kicking, kicking);
                let victim = app.world().get::<Rider>(target).unwrap();
                assert_eq!(victim.hits.count, if kicking { 0 } else { 1 });
                assert!(
                    (app.world().get::<Velocity>(target).unwrap().linear
                        - if kicking {
                            right * side * 8.0
                        } else {
                            Vec3::ZERO
                        })
                    .length()
                        < 1e-6
                );
                assert_eq!(
                    app.world().get::<Recovery>(target).unwrap().stage,
                    RecoveryStage::Riding
                );
                assert_eq!(
                    app.world().get::<Transform>(target).unwrap().rotation,
                    pose.rotation
                );
                app.world_mut().resource_mut::<Race>().elapsed += 0.1;
                app.world_mut().write_message(AttackAttempt(owner, action));
                app.world_mut().run_system_once(melee).unwrap();
                let replaced = app
                    .world()
                    .get::<Rider>(target)
                    .unwrap()
                    .hit_reaction
                    .unwrap();
                assert!(replaced.at > hit.at);
                assert_eq!(replaced.weight(replaced.at), 0.25);
                // Out-of-range misses do not retrigger or play impact audio.
                app.world_mut()
                    .get_mut::<Transform>(target)
                    .unwrap()
                    .translation += Vec3::Y * 10.0;
                app.world_mut().resource_mut::<Messages<AudioCue>>().clear();
                app.world_mut().resource_mut::<Race>().shake = 0.0;
                app.world_mut().write_message(AttackAttempt(owner, action));
                app.world_mut().run_system_once(melee).unwrap();
                assert_eq!(
                    app.world()
                        .get::<Rider>(target)
                        .unwrap()
                        .hit_reaction
                        .unwrap()
                        .at,
                    replaced.at
                );
                assert!(app.world().resource::<Messages<AudioCue>>().is_empty());
                assert_eq!(app.world().resource::<Race>().shake, 0.0);
                app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
                app.world_mut().run_system_once(schedule).unwrap();
                assert_eq!(
                    app.world()
                        .get::<Rider>(target)
                        .unwrap()
                        .hit_reaction
                        .unwrap()
                        .at,
                    replaced.at
                );
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                app.world_mut().get_mut::<Recovery>(target).unwrap().stage = RecoveryStage::Falling;
                app.world_mut().run_system_once(schedule).unwrap();
                assert!(
                    app.world()
                        .get::<Rider>(target)
                        .unwrap()
                        .hit_reaction
                        .is_none()
                );
                assert!(Rider::new(1).hit_reaction.is_none());
            }
        }
    }
    /// Real buffered input -> drive -> scheduler -> LOS/contact -> hit chain.
    /// Render frequency includes zero-tick frames; no resolved contact is injected.
    #[test]
    fn buffered_strikes_wait_for_contact_and_resolve_once_at_all_render_rates() {
        for hz in [30, 60, 120, 144] {
            for (key, name, side, source_contact) in [
                (KeyCode::KeyJ, "punch_left", -1.0, 0.26),
                (KeyCode::KeyK, "punch_right", 1.0, 0.26),
                (KeyCode::Space, "kick_left", -1.0, 0.24),
                (KeyCode::Space, "kick_right", 1.0, 0.24),
            ] {
                let mut app = crate::game::tests::simulation();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                for _ in 0..3 {
                    app.update();
                }
                app.add_plugins(crate::animation::MotionPlugin);
                app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                    std::time::Duration::from_secs_f64(1.0 / hz as f64),
                ));
                let (owner, pose) = app
                    .world_mut()
                    .query::<(Entity, &Rider, &Transform)>()
                    .iter(app.world())
                    .find(|(_, r, _)| r.id == 0)
                    .map(|(e, _, t)| (e, *t))
                    .unwrap();
                let right = (pose.rotation * Vec3::Z).cross(Vec3::Y).normalize();
                let target_pose =
                    Transform::from_translation(pose.translation + right * side * 1.7);
                let target = app
                    .world_mut()
                    .spawn((
                        crate::npc::Npc::new(crate::npc::NpcKind::Pedestrian, target_pose),
                        target_pose,
                        Visibility::default(),
                    ))
                    .id();
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(key);
                let mut observed_contact = false;
                let mut samples = Vec::new();
                for _ in 0..hz {
                    app.update();
                    let clock = app
                        .world_mut()
                        .query::<&Rider>()
                        .iter(app.world())
                        .find(|r| r.id == 0)
                        .unwrap()
                        .strike;
                    let hits = app
                        .world()
                        .get::<crate::npc::Npc>(target)
                        .unwrap()
                        .hits
                        .count;
                    if let Some(clock) = clock {
                        if clock.age < clock.contact {
                            assert_eq!(hits, 0, "early hit at {hz}Hz");
                        } else {
                            if !observed_contact {
                                let pose = app
                                    .world()
                                    .get::<crate::animation::Animator>(owner)
                                    .unwrap()
                                    .current;
                                let expected = app
                                    .world()
                                    .resource::<crate::animation::Library>()
                                    .sample(name, source_contact);
                                for (actual, expected) in
                                    pose.rotations.into_iter().zip(expected.rotations)
                                {
                                    assert!(
                                        actual.dot(expected).abs() > 0.99999,
                                        "contact pose differs at {hz}Hz"
                                    );
                                }
                            }
                            observed_contact = true;
                            assert_eq!(
                                hits, 1,
                                "duplicate/missing hit at {hz}Hz age {} contact {}",
                                clock.age, clock.contact
                            );
                        }
                        samples.push((clock.age, hits));
                    } else {
                        assert_eq!(hits, 0);
                    }
                }
                assert!(observed_contact);
                let first = samples.iter().find(|(_, h)| *h == 1).unwrap().0;
                assert!((0.188..0.234).contains(&first), "{hz}Hz: {first}");
                println!(
                    "render_hz={hz}, clip={name}, source_contact={source_contact}s, first_observed_contact_age={first:.6}s, hits=1"
                );
            }
        }
    }
    #[test]
    fn windup_pauses_then_cancels_on_recovery_and_kick_side_stays_chosen() {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..3 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        let mut rival = Rider::new(1);
        rival.lane = -3.7;
        rival.progress = -8.0;
        rival.immunity = 0.0;
        let target = app
            .world_mut()
            .spawn((rival, Transform::default(), Visibility::default()))
            .id();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);
        for _ in 0..3 {
            app.update();
        }
        let clock = app
            .world()
            .get::<Rider>(owner)
            .unwrap()
            .strike
            .expect("kick input starts windup");
        app.world_mut()
            .write_message(AttackIntent(owner, rr_core::AttackSide::Left));
        app.world_mut().run_system_once(schedule).unwrap();
        let unchanged = app.world().get::<Rider>(owner).unwrap().strike.unwrap();
        assert_eq!(unchanged.action, rr_core::AttackSide::Kick);
        assert_eq!(unchanged.side, clock.side);
        assert!(
            unchanged.age > clock.age,
            "duplicate intent must not restart windup"
        );
        let clock = unchanged;
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(
            app.world().get::<Rider>(owner).unwrap().strike.unwrap().age,
            clock.age
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        // Move the opponent through the attacker during windup. The chosen leg
        // cannot switch or hit a newly occupied opposite-side contact volume.
        app.world_mut().get_mut::<Rider>(target).unwrap().lane = 0.0;
        for _ in 0..20 {
            app.update();
        }
        let r = app.world().get::<Rider>(owner).unwrap();
        assert_eq!(r.attack_side, clock.side);
        assert_eq!(r.strike.unwrap().side, clock.side);
        app.world_mut().get_mut::<Rider>(owner).unwrap().strike = Some(
            rr_core::motion::StrikeClock::new(rr_core::AttackSide::Right, 1.0, 0.26),
        );
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stagger =
            Some(rr_core::crash_behavior::Stagger::new(1.0));
        app.world_mut().run_system_once(schedule).unwrap();
        assert!(app.world().get::<Rider>(owner).unwrap().strike.is_none());
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stagger = None;
        // Recovery entry must discard even an unconsumed contact clock.
        app.world_mut().get_mut::<Rider>(owner).unwrap().strike = Some(
            rr_core::motion::StrikeClock::new(rr_core::AttackSide::Right, 1.0, 0.26),
        );
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::Falling;
        app.update();
        assert!(app.world().get::<Rider>(owner).unwrap().strike.is_none());
    }
    #[test]
    fn nearby_role_free_rider_can_strike_without_prior_grudge() {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..3 {
            app.update();
        }
        let player = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let pose = *app.world().get::<Transform>(player).unwrap();
        let right = (pose.rotation * Vec3::Z).cross(Vec3::Y).normalize();
        app.world_mut().get_mut::<Rider>(player).unwrap().immunity = 0.;
        let mut r = Rider::new(1);
        r.immunity = 0.;
        let racer = app
            .world_mut()
            .spawn((
                r,
                Transform::from_translation(pose.translation - right * 1.4)
                    .with_rotation(pose.rotation),
                Visibility::Visible,
            ))
            .id();
        app.world_mut()
            .write_message(AttackAttempt(racer, rr_core::AttackSide::Right));
        app.world_mut().run_system_once(melee).unwrap();
        assert!(app.world().get::<Rider>(player).unwrap().health < 100.);
        assert!(
            app.world()
                .get::<Rider>(racer)
                .unwrap()
                .combat_mind
                .can_fight(0)
        );
    }
    #[test]
    fn kick_faces_nearest_out_of_reach_opponent_without_a_remote_hit() {
        for side in [-1.0, 1.0] {
            let mut app = crate::game::tests::simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let t = *app.world().get::<Transform>(owner).unwrap();
            let right = (t.rotation * Vec3::Z).cross(Vec3::Y).normalize();
            let target = app
                .world_mut()
                .spawn((
                    Rider::new(1),
                    Transform::from_translation(t.translation + right * side * 6.0)
                        .with_rotation(t.rotation),
                    Visibility::Visible,
                    Velocity::zero(),
                ))
                .id();
            app.world_mut()
                .write_message(AttackAttempt(owner, rr_core::AttackSide::Kick));
            app.world_mut().run_system_once(melee).unwrap();
            assert_eq!(app.world().get::<Rider>(owner).unwrap().attack_side, side);
            assert_eq!(
                app.world().get::<Velocity>(target).unwrap().linear,
                Vec3::ZERO
            );
            assert_eq!(app.world().get::<Rider>(target).unwrap().health, 100.0);
        }
    }
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
                    .linear
                    .dot(right)
                    * side
                    > 7.9
            );
            assert_eq!(
                app.world().get::<Velocity>(targets[1]).unwrap().linear,
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
                .get::<Rider>(target)
                .unwrap()
                .hit_reaction
                .is_none()
        );
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
