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
pub struct AttackAttempt(pub Entity, pub f32);
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
    for &AttackAttempt(attacker, side) in attempts.read() {
        let Ok((_, rider, state, t, _)) = riders.get(attacker) else {
            continue;
        };
        if state.stage != RecoveryStage::Riding {
            continue;
        }
        let player = rider.id == 0;
        let origin = t.translation + Vec3::Y * 1.45;
        let forward = t.rotation * Vec3::Z;
        let right = forward.cross(Vec3::Y).normalize_or_zero();
        let attacker_velocity = rider.motion_velocity;
        let mut nearest: Option<(Entity, f32, Vec3, bool)> = None;
        let mut consider = |entity: Entity, p: Vec3, is_npc: bool| {
            let delta = p - origin;
            let lateral = delta.dot(right);
            let longitudinal = delta.dot(forward);
            if !rr_core::direct_strike_geometry(lateral, longitudinal, delta.y, side) {
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
        audio.write(AudioCue::new(Cue::Punch, p));
        if is_npc {
            npc_hits.write(NpcHit(target, attacker_velocity, true));
            continue;
        }
        if let Ok((_, mut r, mut state, t, mut visible)) = riders.get_mut(target) {
            let knocked_out = r.hits.direct_hit(race.elapsed);
            r.health = 100.0 * (3 - r.hits.count) as f32 / 3.0;
            if player {
                race.message = format!("DIRECT HIT / {} OF 3", r.hits.count);
                race.message_time = 1.0;
            }
            if knocked_out {
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
