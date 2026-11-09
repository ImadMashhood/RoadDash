//! One resident, non-racing officer using the ordinary rider/vehicle/recovery path.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    track::Track,
    world::{self, Art},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_core::{
    police::{Mind, Observation, State},
    pursuit::Target,
};
pub mod survey;
pub const OFFICER_ID: usize = 15;
#[derive(Resource)]
pub struct Police {
    pub rules: rr_core::police::Rules,
    pub mind: Mind,
    pub target: Option<(Target, Vec3)>,
    pub foot_goal: Option<Vec3>,
    tick: u64,
    pub perceptions: u64,
    pub contact_checks: u64,
    pub evaluations: u64,
    pub micros: u64,
    pub max_micros: u64,
    last_elapsed: f32,
    pub escapes: u32,
    pub assault: Option<f32>,
    pub intentional_foot: bool,
    pub contact_block_until: f32,
}
impl Default for Police {
    fn default() -> Self {
        Self {
            rules: rr_content::police::builtin(),
            mind: default(),
            target: None,
            foot_goal: None,
            tick: 0,
            perceptions: 0,
            contact_checks: 0,
            evaluations: 0,
            micros: 0,
            max_micros: 0,
            last_elapsed: 0.,
            escapes: 0,
            assault: None,
            intentional_foot: false,
            contact_block_until: 0.,
        }
    }
}
impl Police {
    pub fn hud(&self) -> String {
        match self.mind.state {
            State::Patrol | State::Return => {
                format!("POLICE / LIMIT {:.0} KM/H", self.rules.speed_limit * 3.6)
            }
            State::Warning => "POLICE WARNING / SLOW DOWN & STOP FIGHTING".into(),
            State::Search => format!(
                "POLICE SEARCH / STAY OUT OF SIGHT {:.1}s",
                (self.rules.search_seconds - self.mind.search).max(0.)
            ),
            State::Arrested => "BUSTED / ENTER OR R TO RETRY".into(),
            _ => {
                if self.mind.arrest > 0. {
                    format!(
                        "ARREST {:.0}% / MOVE TO ESCAPE",
                        100. * self.mind.arrest / self.rules.arrest_seconds
                    )
                } else {
                    "POLICE PURSUIT / BREAK LINE OF SIGHT".into()
                }
            }
        }
    }
    pub fn drive_goal(&self, rider: &Rider, track: &Track, top: f32) -> (f32, f32) {
        if self.mind.state.wanted()
            && let Some((t, _)) = self.target
        {
            let gap = t.progress - rider.progress;
            let speed = rr_core::police::approach_speed(t.speed, gap, t.downed, top, self.rules);
            return (
                if (t.downed && gap.abs() <= rr_core::police::FOOT_APPROACH_RANGE)
                    || (t.speed <= self.rules.arrest_speed && gap.abs() < 12.)
                {
                    rider.lane
                } else {
                    t.lane
                },
                speed,
            );
        }
        (
            track.street_offset(rider.route, rider.progress) + 2.,
            if rider.progress >= track.length - 8. {
                0.
            } else {
                self.rules.patrol_speed
            },
        )
    }
}
pub fn setup(mut commands: Commands, art: Res<Art>, track: Res<Track>) {
    let mut r = Rider::new(OFFICER_ID);
    r.palette_index = 6;
    r.progress = track.length * rr_content::police::builtin().patrol_fraction;
    r.route = 0;
    r.lane = 2.;
    // The officer only fights the observed player during active pursuit.
    r.combat_mind.struck_by(0);
    let (p, q) = track.sample_route(r.route, r.progress, r.lane);
    let root = world::bike(&mut commands, &art, r.palette_index, true);
    commands.entity(root).insert((
        Name::new("Resident police motorcycle"),
        r,
        Transform::from_translation(p).with_rotation(q),
        crate::recovery::bike_body(),
    ));
    // Original high-contrast blue/white marker. Existing shared GPU geometry only.
    for (x, mat) in [(-0.22, art.colors[3].clone()), (0.22, art.white.clone())] {
        let part = world::part(
            &mut commands,
            art.cube.clone(),
            mat,
            Vec3::new(x, 1.25, -0.85),
            Vec3::new(0.20, 0.18, 0.22),
            Quat::IDENTITY,
        );
        commands.entity(root).add_child(part);
    }
}
fn location(r: &Recovery, t: &Transform, poses: &Query<&Transform, Without<Rider>>) -> Vec3 {
    r.runner
        .or(r.torso)
        .and_then(|e| poses.get(e).ok())
        .map_or(t.translation, |t| t.translation - Vec3::Y * 0.95)
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn think(
    mut police: ResMut<Police>,
    mut race: ResMut<Race>,
    riders: Query<(Entity, &Rider, &Recovery, &Transform)>,
    poses: Query<&Transform, Without<Rider>>,
    owned: Query<&crate::recovery::OwnedBody>,
    context: ReadRapierContext,
    track: Res<Track>,
    mut audio: MessageWriter<crate::sound::AudioCue>,
) {
    if race.phase == Phase::Countdown || race.elapsed < police.last_elapsed {
        *police = Police::default();
    }
    police.last_elapsed = race.elapsed;
    if race.phase != Phase::Racing {
        police.mind.interrupt_arrest();
        return;
    }
    let Some((player, pr, prec, pt)) = riders.iter().find(|(_, r, _, _)| r.id == 0) else {
        return;
    };
    let Some((officer, cr, crec, ct)) = riders.iter().find(|(_, r, _, _)| r.police) else {
        return;
    };
    if !matches!(crec.stage, RecoveryStage::Running | RecoveryStage::Mounting) {
        police.intentional_foot = false;
    }
    let pp = location(prec, pt, &poses);
    let cp = location(crec, ct, &poses);
    let pv = if prec.stage == RecoveryStage::Riding {
        pr.motion_velocity
    } else {
        prec.walk_velocity
    };
    let cv = if crec.stage == RecoveryStage::Riding {
        cr.motion_velocity
    } else {
        crec.walk_velocity
    };
    let ps = pv.length().max(if prec.stage == RecoveryStage::Riding {
        pr.speed.abs()
    } else {
        0.
    });
    let cs = cv.length().max(if crec.stage == RecoveryStage::Riding {
        cr.speed.abs()
    } else {
        0.
    });
    let relative_speed = (pv - cv).length();
    let target_ready = matches!(prec.stage, RecoveryStage::Riding | RecoveryStage::Running)
        && prec.stagger.is_none()
        && (prec.stage != RecoveryStage::Riding || pr.immunity <= 0.)
        && (prec.stage != RecoveryStage::Running || prec.elapsed >= 0.5)
        && pr.hop_height <= 0.15
        && (prec.stage != RecoveryStage::Running || crec.stage == RecoveryStage::Running);
    let officer_ready = (crec.stage == RecoveryStage::Riding
        || (crec.stage == RecoveryStage::Running && police.intentional_foot))
        && crec.stagger.is_none()
        && cr.immunity <= 0.
        && race.elapsed >= police.contact_block_until;
    let distance = pp.distance(cp);
    let rules = police.rules;
    let contact_candidate = police.mind.state.wanted()
        && police.mind.state != State::Search
        && target_ready
        && officer_ready
        && distance <= rules.arrest_radius
        && (pp.y - cp.y).abs() <= 0.6
        && ps <= rules.arrest_speed
        && relative_speed <= rules.arrest_speed;
    if !contact_candidate {
        police.mind.interrupt_arrest();
    }
    police.tick += 1;
    let perception = police.tick.is_multiple_of(6);
    if !perception && !contact_candidate {
        return;
    }
    let start = std::time::Instant::now();
    let direction = pp - cp;
    let predicate = |e| {
        e != officer
            && e != player
            && !owned
                .get(e)
                .is_ok_and(|b| b.owner == officer || b.owner == player)
    };
    let (progress, lane, _) = if prec.stage == RecoveryStage::Riding {
        (pr.progress, pr.lane, 0.)
    } else {
        track.project_route(pr.route, pp, pr.progress)
    };
    let visible = distance <= rules.detection_radius
        && track.same_corridor(cr.route, cr.progress, pr.route, progress)
        && context.single().ok().is_some_and(|ctx| {
            ctx.cast_ray(
                cp + Vec3::Y * 1.2,
                direction.normalize_or_zero(),
                distance,
                true,
                QueryFilter::default()
                    .exclude_sensors()
                    .predicate(&predicate),
            )
            .is_none()
        });
    if perception && visible {
        police.target = Some((
            Target {
                id: 0,
                progress,
                lane,
                speed: ps,
                downed: prec.stage != RecoveryStage::Riding,
            },
            pp,
        ));
    }
    let supported = |p: Vec3| {
        context.single().ok().is_some_and(|ctx| {
            ctx.cast_ray_and_get_normal(
                p + Vec3::Y * 0.3,
                -Vec3::Y,
                1.,
                true,
                QueryFilter::default()
                    .exclude_sensors()
                    .predicate(&predicate),
            )
            .is_some_and(|(_, hit)| hit.normal.y > 0.5)
        })
    };
    let support = contact_candidate && supported(pp) && supported(cp);
    let old = police.mind.state;
    let o = Observation {
        elapsed: race.elapsed,
        visible,
        violation: pr.speed > rules.speed_limit
            || police.assault.is_some_and(|t| race.elapsed - t <= 2.),
        target_ahead: progress - cr.progress,
        officer_ready,
        target_ready: target_ready && support,
        distance,
        target_speed: ps,
        relative_speed,
        at_patrol: crec.stage == RecoveryStage::Riding && cs <= rules.patrol_speed + 1.,
    };
    if perception {
        police.mind.step(o, 0.1, rules);
        police.perceptions += 1;
    }
    if contact_candidate {
        police.mind.contact(o, 1. / 60., rules);
        police.contact_checks += 1;
    }
    police.foot_goal = if police.mind.state.wanted() {
        police.target.map(|t| t.1)
    } else {
        None
    };
    if old != police.mind.state {
        race.message = match police.mind.state {
            State::Warning => "POLICE / SLOW DOWN",
            State::Pursue | State::Intercept => "POLICE PURSUIT",
            State::Search => "POLICE LOST SIGHT / KEEP MOVING",
            State::Return => "ESCAPED / PATROL RESUMES",
            State::Arrested => "BUSTED / NO PAYOUT",
            _ => "POLICE WARNING CLEARED",
        }
        .into();
        race.message_time = 3.;
        if police.mind.state == State::Return {
            police.escapes += 1;
            police.target = None;
        }
        audio.write(crate::sound::AudioCue::new(crate::sound::Cue::Siren, cp));
    }
    if police.mind.state == State::Arrested {
        race.arrested = true;
        race.earned = 0;
        race.phase = Phase::Finished;
    }
    let micros = start.elapsed().as_micros() as u64;
    police.evaluations += 1;
    police.micros += micros;
    police.max_micros = police.max_micros.max(micros);
}

/// A stopped motorcycle can park and create the ordinary solid runner. No
/// falling-body impulses, teleport to target, or animation-owned transforms.
#[allow(clippy::too_many_arguments)]
pub fn dismount(
    mut commands: Commands,
    mut police: ResMut<Police>,
    race: Res<Race>,
    art: Res<Art>,
    context: ReadRapierContext,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        &Transform,
        &mut Visibility,
    )>,
) {
    if race.phase != Phase::Racing
        || !police.mind.state.wanted()
        || !police
            .target
            .is_some_and(|(t, _)| t.downed || t.speed <= police.rules.arrest_speed)
    {
        return;
    }
    let Some(goal) = police.foot_goal else {
        return;
    };
    for (owner, mut r, mut rec, t, mut visibility) in &mut riders {
        if !r.police
            || rec.stage != RecoveryStage::Riding
            || rec.stagger.is_some()
            || r.immunity > 0.
            || r.hop_height > 0.15
            || r.speed.abs() > 0.8
            || r.motion_velocity.length() > 0.8
            || t.translation.distance(goal)
                > if police.target.is_some_and(|(t, _)| t.downed) {
                    rr_core::police::FOOT_APPROACH_RANGE
                } else {
                    12.
                }
        {
            continue;
        }
        let Ok(ctx) = context.single() else {
            continue;
        };
        let filter = QueryFilter::default()
            .exclude_rigid_body(owner)
            .exclude_sensors();
        let shape = Collider::capsule_y(0.65, 0.28);
        let mut safe = None;
        for sign in [1., -1.] {
            let side = t.translation + t.rotation * Vec3::X * (1.35 * sign);
            let Some((_, height)) =
                ctx.cast_ray(side + Vec3::Y * 0.45, -Vec3::Y, 0.9, true, filter)
            else {
                continue;
            };
            let floor = side.y + 0.45 - height;
            if (floor - t.translation.y).abs() > 0.35 {
                continue;
            }
            let point = Vec3::new(side.x, floor + 0.95, side.z);
            let from = t.translation + Vec3::Y * 0.95;
            let options = bevy_rapier3d::parry::query::ShapeCastOptions {
                max_time_of_impact: 1.,
                target_distance: 0.01,
                stop_at_penetration: true,
                ..default()
            };
            if ctx
                .cast_shape(
                    from,
                    Quat::IDENTITY,
                    point - from,
                    shape.raw.as_ref(),
                    options,
                    filter,
                )
                .is_some()
            {
                continue;
            }
            let mut blocked = false;
            ctx.intersect_shape(point, Quat::IDENTITY, shape.raw.as_ref(), filter, |_| {
                blocked = true;
                false
            });
            if !blocked {
                safe = Some(point);
                break;
            }
        }
        let Some(point) = safe else {
            continue;
        };
        let wreck = world::bike(&mut commands, &art, r.palette_index, false);
        commands.entity(wreck).insert((
            *t,
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::new(0., 0.85, 0.),
                Quat::IDENTITY,
                Collider::cuboid(0.22, 0.65, 1.12),
            )]),
            Velocity::zero(),
            ColliderMassProperties::Mass(r.bike_mass),
            Ccd::enabled(),
            Restitution::coefficient(0.25),
            Friction {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Min,
            },
            Sleeping::disabled(),
            Damping {
                linear_damping: 0.25,
                angular_damping: 0.8,
            },
            crate::game::Debris(f32::INFINITY),
            crate::recovery::OwnedBody {
                owner,
                kind: crate::recovery::BodyKind::Wreck,
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
        let (runner, pose) = world::runner(&mut commands, &art, r.palette_index);
        commands.entity(runner).insert((
            Transform::from_translation(point).with_rotation(t.rotation),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.65, 0.28),
            KinematicCharacterController {
                translation: Some(Vec3::ZERO),
                custom_mass: Some(70.),
                apply_impulse_to_dynamic_bodies: false,
                offset: CharacterLength::Absolute(0.02),
                snap_to_ground: Some(CharacterLength::Absolute(0.3)),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.3),
                    min_width: CharacterLength::Absolute(0.15),
                    include_dynamic_bodies: false,
                }),
                ..default()
            },
            crate::recovery::OwnedBody {
                owner,
                kind: crate::recovery::BodyKind::Runner,
            },
            crate::game::Debris(f32::INFINITY),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ));
        police.intentional_foot = true;
        rec.stage = RecoveryStage::Running;
        rec.runner = Some(runner);
        rec.pose = Some(pose);
        rec.wreck = Some(wreck);
        rec.previous_foot = Some(point);
        rec.elapsed = 0.;
        r.speed = 0.;
        r.drive_intent = default();
        r.crashed = 1.;
        r.strike = None;
        *visibility = Visibility::Hidden;
        commands
            .entity(owner)
            .insert((ColliderDisabled, RigidBodyDisabled));
        commands.queue(move |world: &mut World| {
            if let Some(c) = world.entity_mut(owner).take::<Collider>() {
                world
                    .entity_mut(owner)
                    .insert(crate::crash_behavior::DormantChassis(c));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    fn fixture() -> (App, Entity, Entity) {
        let mut app = crate::game::tests::simulation();
        let player = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        let mut cop = Rider::new(OFFICER_ID);
        cop.palette_index = 6;
        cop.progress = -8.;
        cop.lane = -0.5;
        cop.immunity = 0.;
        cop.combat_mind.struck_by(0);
        let track = app.world().resource::<Track>();
        let (p, q) = track.sample_route(0, -8., -0.5);
        let officer = app
            .world_mut()
            .spawn((
                cop,
                Transform::from_translation(p).with_rotation(q),
                Visibility::Visible,
                crate::recovery::bike_body(),
            ))
            .id();
        let floor = app
            .world()
            .resource::<Track>()
            .sample_route(0, -8., -2.)
            .0
            .y;
        for mut t in app
            .world_mut()
            .query_filtered::<&mut Transform, With<crate::world::Ground>>()
            .iter_mut(app.world_mut())
        {
            t.translation.y = floor - 0.5;
        }
        app.update();
        app.update();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        app.world_mut().get_mut::<Rider>(player).unwrap().immunity = 0.;
        (app, player, officer)
    }
    #[test]
    fn integrated_arrest_has_no_payout_or_best_time_and_retry_preserves_noncompetitor() {
        let (mut app, player, officer) = fixture();
        // Full shared controller approaches from20m behind a stopped player,
        // parks, walks to live contact and completes a safe dwell.
        fn relocate(
            mut commands: Commands,
            track: Res<Track>,
            mut riders: Query<(Entity, &mut Rider)>,
        ) {
            for (e, mut r) in &mut riders {
                r.progress = if r.police { 80. } else { 100. };
                r.lane = if r.police { 0. } else { -2. };
                r.immunity = 0.;
                r.speed = 0.;
                r.motion_velocity = Vec3::ZERO;
                let (p, q) = track.sample_route(0, r.progress, r.lane);
                crate::vehicle::remount_at(
                    &mut commands,
                    e,
                    Transform::from_translation(p).with_rotation(q),
                );
            }
        }
        app.world_mut().run_system_once(relocate).unwrap();
        app.world_mut().resource_mut::<Police>().mind.state = State::Pursue;
        let original = app.world().resource::<crate::game::Profile>().credits;
        for frame in 0..1800 {
            app.update();
            if frame % 300 == 0 {
                let c = app.world().get::<Rider>(officer).unwrap();
                let p = app.world().get::<Rider>(player).unwrap();
                eprintln!(
                    "police fixture f={frame} player={}/{}/{} cop={}/{}/{} vel={:?} target={:?} mind={:?}",
                    p.progress,
                    p.lane,
                    p.speed,
                    c.progress,
                    c.lane,
                    c.speed,
                    c.motion_velocity,
                    app.world()
                        .resource::<Police>()
                        .target
                        .map(|(t, p)| (t.progress, t.speed, p)),
                    app.world().resource::<Police>().mind
                );
                let rec = app.world().get::<Recovery>(officer).unwrap();
                eprintln!(
                    "stage={:?} runner={:?} walk={:?} footgoal={:?}",
                    rec.stage,
                    rec.runner
                        .and_then(|e| app.world().get::<Transform>(e))
                        .map(|t| t.translation),
                    rec.walk_velocity,
                    app.world().resource::<Police>().foot_goal
                );
            }
            if app.world().resource::<Race>().arrested {
                break;
            }
        }
        assert!(
            app.world().resource::<Race>().arrested,
            "mind={:?}, cop={:?}, intentional={}, position={:?}",
            app.world().resource::<Police>().mind,
            app.world().get::<Recovery>(officer).unwrap().stage,
            app.world().resource::<Police>().intentional_foot,
            app.world().get::<Transform>(officer).unwrap().translation
        );
        assert_eq!(app.world().resource::<Race>().phase, Phase::Finished);
        let profile = app.world().resource::<crate::game::Profile>();
        assert_eq!(profile.credits, original);
        assert_eq!(profile.races, 0);
        assert_eq!(profile.best_seconds, None);
        let before = app.world().resource::<Race>().elapsed;
        for _ in 0..20 {
            app.update();
        }
        assert_eq!(app.world().resource::<Race>().elapsed, before);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();
        assert!(!app.world().resource::<Race>().arrested);
        assert_eq!(app.world().resource::<Police>().mind.state, State::Patrol);
        assert!(app.world().get::<Rider>(officer).unwrap().police);
        assert_eq!(app.world().get::<Rider>(officer).unwrap().palette_index, 6);
    }
    #[test]
    fn integrated_los_and_falling_prevent_arrest_and_loss_allows_escape() {
        let (mut app, player, officer) = fixture();
        app.world_mut().resource_mut::<Police>().mind.state = State::Pursue;
        // A tall solid wall separates the two actual root/eye positions.
        let a = app.world().get::<Transform>(player).unwrap().translation;
        let b = app.world().get::<Transform>(officer).unwrap().translation;
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(0.1, 3., 5.),
            Transform::from_translation((a + b) * 0.5 + Vec3::Y)
                .with_rotation(Quat::from_rotation_arc(Vec3::X, (b - a).normalize())),
        ));
        for _ in 0..530 {
            for e in [player, officer] {
                let mut r = app.world_mut().get_mut::<Rider>(e).unwrap();
                r.speed = 0.;
                r.motion_velocity = Vec3::ZERO;
            }
            app.update();
        }
        assert!(!app.world().resource::<Race>().arrested);
        assert!(app.world().resource::<Police>().escapes >= 1);
    }
    #[test]
    fn opposite_motion_and_one_tick_wall_or_support_break_reset_arrest() {
        let (mut app, player, officer) = fixture();
        app.world_mut().resource_mut::<Police>().mind.state = State::Pursue;
        app.world_mut().resource_mut::<Police>().mind.arrest = 0.5;
        app.world_mut()
            .get_mut::<Rider>(player)
            .unwrap()
            .motion_velocity = Vec3::X;
        app.world_mut()
            .get_mut::<Rider>(officer)
            .unwrap()
            .motion_velocity = Vec3::NEG_X;
        app.world_mut().run_system_once(think).unwrap();
        assert_eq!(
            app.world().resource::<Police>().mind.arrest,
            0.,
            "opposite equal speed was treated as stationary relative motion"
        );
        for e in [player, officer] {
            app.world_mut().get_mut::<Rider>(e).unwrap().motion_velocity = Vec3::ZERO;
        }
        let a = app.world().get::<Transform>(player).unwrap().translation;
        let b = app.world().get::<Transform>(officer).unwrap().translation;
        let wall = app
            .world_mut()
            .spawn((
                RigidBody::Fixed,
                Collider::cuboid(0.1, 3., 5.),
                Transform::from_translation((a + b) * 0.5 + Vec3::Y)
                    .with_rotation(Quat::from_rotation_arc(Vec3::X, (b - a).normalize())),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        app.world_mut().run_schedule(FixedUpdate);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Police>().mind.arrest = 0.5;
        app.world_mut().resource_mut::<Police>().tick = 0;
        app.world_mut().run_system_once(think).unwrap();
        assert_eq!(
            app.world().resource::<Police>().mind.arrest,
            0.,
            "one tick occlusion survived between perceptions"
        );
        app.world_mut().despawn(wall);
        let ground = app
            .world_mut()
            .query_filtered::<Entity, With<crate::world::Ground>>()
            .iter(app.world())
            .collect::<Vec<_>>();
        for e in ground {
            app.world_mut().despawn(e);
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        app.world_mut().run_schedule(FixedUpdate);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Police>().mind.arrest = 0.5;
        app.world_mut().resource_mut::<Police>().tick = 0;
        app.world_mut().run_system_once(think).unwrap();
        assert_eq!(
            app.world().resource::<Police>().mind.arrest,
            0.,
            "one tick unsupported contact survived between perceptions"
        );
    }
    #[test]
    fn real_pause_restart_and_valid_finish_take_priority_over_threshold_arrest() {
        for key in [Some(KeyCode::Escape), Some(KeyCode::KeyR), None] {
            let (mut app, player, _) = fixture();
            {
                let mut police = app.world_mut().resource_mut::<Police>();
                police.mind.state = State::Pursue;
                police.mind.arrest = police.rules.arrest_seconds - 0.001;
            }
            if let Some(key) = key {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(key);
            } else {
                let mut r = app.world_mut().get_mut::<Rider>(player).unwrap();
                r.gates.completed_laps = crate::track::LAPS;
            }
            app.update();
            assert!(!app.world().resource::<Race>().arrested);
            assert_eq!(app.world().resource::<Police>().mind.arrest, 0.);
            let profile = app.world().resource::<crate::game::Profile>();
            assert_eq!(profile.races, u32::from(key.is_none()));
            if key == Some(KeyCode::Escape) {
                assert_eq!(app.world().resource::<Race>().phase, Phase::Paused);
            }
            if key == Some(KeyCode::KeyR) {
                assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
            }
        }
    }
    #[test]
    fn same_tick_third_contact_interrupts_officer_arrest_authority() {
        let (mut app, player, officer) = fixture();
        {
            let mut police = app.world_mut().resource_mut::<Police>();
            police.mind.state = State::Pursue;
            police.mind.arrest = police.rules.arrest_seconds - 0.001;
        }
        {
            let mut r = app.world_mut().get_mut::<Rider>(officer).unwrap();
            r.hits.direct_hit(8.);
            r.hits.direct_hit(9.);
        }
        app.world_mut().write_message(crate::combat::AttackAttempt(
            player,
            rr_core::AttackSide::Right,
        ));
        app.update();
        assert!(!app.world().resource::<Race>().arrested);
        assert_eq!(app.world().resource::<Police>().mind.arrest, 0.);
        let rec = app.world().get::<Recovery>(officer).unwrap();
        assert!(
            rec.stage == RecoveryStage::Falling || rec.stagger.is_some(),
            "stage={:?}, hits={}, health={}, block={}",
            rec.stage,
            app.world().get::<Rider>(officer).unwrap().hits.count,
            app.world().get::<Rider>(officer).unwrap().health,
            app.world().resource::<Police>().contact_block_until
        );
        assert!(!app.world().resource::<Police>().intentional_foot);
    }
    #[test]
    fn buffered_club_hits_live_foot_officer_interrupt_arrest_and_third_hit_knocks_down() {
        let (mut app, player, officer) = fixture();
        let root = *app.world().get::<Transform>(player).unwrap();
        let right = (root.rotation * Vec3::Z).cross(Vec3::Y);
        let point = root.translation + right * 1.65 + Vec3::Y * 0.95;
        let runner = app
            .world_mut()
            .spawn((
                Transform::from_translation(point).with_rotation(root.rotation),
                RigidBody::KinematicPositionBased,
                Collider::capsule_y(0.65, 0.28),
                KinematicCharacterController::default(),
                crate::recovery::OwnedBody {
                    owner: officer,
                    kind: crate::recovery::BodyKind::Runner,
                },
            ))
            .id();
        let wreck = app
            .world_mut()
            .spawn((
                Transform::from_translation(root.translation - root.rotation * Vec3::Z * 12.),
                RigidBody::KinematicPositionBased,
                Collider::cuboid(0.22, 0.65, 1.12),
                ColliderMassProperties::Mass(170.),
                Velocity::zero(),
                crate::recovery::OwnedBody {
                    owner: officer,
                    kind: crate::recovery::BodyKind::Wreck,
                },
            ))
            .id();
        {
            let mut rec = app.world_mut().get_mut::<Recovery>(officer).unwrap();
            rec.stage = RecoveryStage::Running;
            rec.runner = Some(runner);
            rec.wreck = Some(wreck);
            rec.elapsed = 1.;
            rec.previous_foot = Some(point);
        }
        app.world_mut()
            .entity_mut(officer)
            .insert((ColliderDisabled, RigidBodyDisabled))
            .remove::<Collider>();
        {
            let mut police = app.world_mut().resource_mut::<Police>();
            police.intentional_foot = true;
            police.mind.state = State::Pursue;
            police.foot_goal = Some(root.translation);
            police.target = Some((
                Target {
                    id: 0,
                    progress: -8.,
                    lane: -2.,
                    speed: 0.,
                    downed: false,
                },
                root.translation,
            ));
        }
        app.world_mut().get_mut::<Rider>(player).unwrap().weapon =
            Some(rr_core::weapons::Weapon::Club);
        for expected in 1..=3 {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyK);
            app.update();
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .release(KeyCode::KeyK);
            for _ in 0..60 {
                app.update();
                assert!(!app.world().resource::<Race>().arrested);
            }
            assert_eq!(
                app.world().get::<Rider>(officer).unwrap().hits.count,
                expected
            );
        }
        assert_eq!(
            app.world().get::<Recovery>(officer).unwrap().stage,
            RecoveryStage::Falling
        );
        assert!(!app.world().resource::<Police>().intentional_foot);
        assert!(app.world().get_entity(runner).is_err());
    }
    #[test]
    fn police_cannot_finish_or_enter_fifteen_racer_payout() {
        let mut r = Rider::new(OFFICER_ID);
        r.gates.completed_laps = crate::track::LAPS;
        let mut race = Race::default();
        let mut profile = crate::game::Profile::default();
        assert!(!crate::game::finish_rider(
            &mut r,
            &mut race,
            &mut profile,
            false,
            1,
            false
        ));
        assert!(r.finished.is_none());
        assert_eq!(profile.races, 0);
        race.arrested = true;
        let mut player = Rider::new(0);
        player.gates.completed_laps = crate::track::LAPS;
        assert!(!crate::game::finish_rider(
            &mut player,
            &mut race,
            &mut profile,
            false,
            1,
            false
        ));
    }
}

#[cfg(test)]
mod fairness_tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    #[test]
    fn confirmed_assault_must_be_witnessed_at_contact_not_after_arrival() {
        for obscured in [false, true] {
            let mut app = crate::game::tests::simulation();
            app.update();
            app.update();
            let (owner, root) = app
                .world_mut()
                .query::<(Entity, &Rider, &Transform)>()
                .iter(app.world())
                .find(|(_, r, _)| r.id == 0)
                .map(|(e, _, t)| (e, *t))
                .unwrap();
            let track = app.world().resource::<Track>();
            let mut cop = Rider::new(OFFICER_ID);
            cop.palette_index = 6;
            cop.progress = -18.;
            cop.lane = -2.;
            cop.immunity = 0.;
            let (p, q) = track.sample_route(0, cop.progress, cop.lane);
            let officer = app
                .world_mut()
                .spawn((
                    cop,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::Visible,
                    crate::recovery::bike_body(),
                ))
                .id();
            let right = (root.rotation * Vec3::Z).cross(Vec3::Y);
            let target = Transform::from_translation(root.translation + right * 1.7);
            app.world_mut().spawn((
                crate::npc::Npc::new(crate::npc::NpcKind::Pedestrian, target),
                target,
                Visibility::Visible,
            ));
            let wall = if obscured {
                Some(
                    app.world_mut()
                        .spawn((
                            RigidBody::Fixed,
                            Collider::cuboid(4., 3., 0.1),
                            Transform::from_translation((p + root.translation) * 0.5 + Vec3::Y)
                                .with_rotation(q),
                        ))
                        .id(),
                )
            } else {
                None
            };
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.update();
            app.update();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Race>().elapsed = 10.;
            app.world_mut().write_message(crate::combat::AttackAttempt(
                owner,
                rr_core::AttackSide::Right,
            ));
            app.world_mut()
                .run_system_once(crate::combat::melee)
                .unwrap();
            assert_eq!(
                app.world().resource::<Police>().assault.is_some(),
                !obscured
            );
            if let Some(wall) = wall {
                app.world_mut().despawn(wall);
                app.update();
                assert!(
                    app.world().resource::<Police>().assault.is_none(),
                    "unseen hit was charged after wall removal"
                );
            }
            app.world_mut().resource_mut::<Police>().assault = None;
            app.world_mut()
                .resource_mut::<Messages<crate::combat::AttackAttempt>>()
                .clear();
            app.world_mut().write_message(crate::combat::AttackAttempt(
                officer,
                rr_core::AttackSide::Left,
            ));
            app.world_mut()
                .run_system_once(crate::combat::melee)
                .unwrap();
            assert!(
                app.world().resource::<Police>().assault.is_none(),
                "non-player or whiff recorded a player crime"
            );
        }
    }
}
