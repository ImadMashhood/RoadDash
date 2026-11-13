//! Persistent race-local prop damage. Cars, racers and buildings are never damage targets.
use crate::{
    game::{Phase, Race, Rider, Traffic},
    recovery::{OwnedBody, PreviousMotion, SweptContact},
    runtime::SimSet,
    sound::{AudioCue, Cue},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_content::destruction::{Config, Profile, Style};
use rr_core::destruction::{Damage, impact_energy};
use std::collections::HashMap;
pub mod survey;
#[cfg(test)]
mod tests;
mod visuals;
#[derive(Resource)]
pub struct Tuning(pub Config);
impl Default for Tuning {
    fn default() -> Self {
        Self(
            Config::from_json(include_str!("../../content/world/destruction.json"))
                .expect("validated prop destruction tuning"),
        )
    }
}
#[derive(Component)]
pub struct Breakable {
    pub state: Damage,
    pub profile: Profile,
    pub original: Transform,
    pub shape: Collider,
    pub size: Vec3,
    pub slot: Option<usize>,
    pub visual: Entity,
}
#[derive(Component)]
pub struct Loose {
    age: f32,
    pub soft: bool,
}
#[derive(Component, Default)]
struct Before(Vec3);
#[derive(Component)]
pub struct FrozenVisual;
#[derive(Message)]
pub struct Reset;
#[derive(Resource, Default)]
struct Contacts {
    time: f32,
    pairs: HashMap<(Entity, Entity), f32>,
}
#[derive(Resource, Default)]
pub struct Stats {
    pub breaks: usize,
    pub peak_dynamic: usize,
}
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PropDamageVisuals;
pub struct PropDestructionPlugin;
impl Plugin for PropDestructionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tuning>()
            .init_resource::<Contacts>()
            .init_resource::<Stats>()
            .add_message::<Reset>()
            .add_systems(FixedPreUpdate, remember)
            .add_systems(
                FixedUpdate,
                (reset, impacts)
                    .chain()
                    .after(crate::game::drive)
                    .after(crate::combat::schedule)
                    .after(crate::combat::melee)
                    .in_set(SimSet::Gameplay),
            )
            .add_systems(
                FixedUpdate,
                (settle, sync_navigation)
                    .chain()
                    .in_set(SimSet::AfterPhysics),
            )
            .add_systems(
                Update,
                visuals::update
                    .after(crate::props::PropLods)
                    .in_set(PropDamageVisuals),
            );
    }
}
/// Optional queue hook keeps existing minimal race tests independent of this plugin.
pub fn request_reset(world: &mut World) {
    if let Some(mut events) = world.get_resource_mut::<Messages<Reset>>() {
        events.write(Reset);
    }
}
#[allow(clippy::too_many_arguments)]
pub fn register(
    commands: &mut Commands,
    props: &crate::props::PropArt,
    kind: &str,
    seed: usize,
    id: &str,
    pose: Transform,
    size: Vec3,
    slot: Option<usize>,
) -> Entity {
    static CONFIG: std::sync::OnceLock<Tuning> = std::sync::OnceLock::new();
    let profile = CONFIG.get_or_init(Tuning::default).0.profiles[kind].clone();
    let part = if kind == "broadleaf" {
        Collider::cylinder(size.y * 0.5, 0.18)
    } else {
        Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
    };
    let shape = Collider::compound(vec![(Vec3::Y * size.y * 0.5, Quat::IDENTITY, part)]);
    let visual = props.fitted(
        commands,
        props.choose(kind, seed),
        Vec3::ZERO,
        Quat::IDENTITY,
        size,
        "metropolis furniture",
    );
    // Each source has a matching offline variant; share the same physical owner.
    // Authoring bakes height to 38%, so only the fitted base offset is compressed.
    let source = props.choose(kind, seed);
    let damaged = (profile.style == Style::Crush).then(|| {
        let scale = size / Vec3::from_array(source.size);
        let offset = Vec3::new(
            -source.center[0],
            (source.size[1] * 0.5 - source.center[1]) * 0.38,
            -source.center[2],
        ) * scale;
        let scene = props
            .server
            .as_ref()
            .map(|server| server.load(format!("props://broken/{}.glb#Scene0", source.id)))
            .unwrap_or_default();
        commands
            .spawn((
                Name::new(format!("broken prop {}", source.id)),
                WorldAssetRoot(scene),
                Transform::from_translation(offset).with_scale(scale),
                Visibility::Hidden,
                visuals::Damaged,
            ))
            .observe(visuals::ready)
            .id()
    });
    let e = commands
        .spawn((
            Name::new(id.to_owned()),
            RigidBody::Fixed,
            shape.clone(),
            pose,
            crate::world::Obstacle,
            ActiveEvents::COLLISION_EVENTS,
            Breakable {
                state: Damage::default(),
                profile,
                original: pose,
                shape,
                size,
                slot,
                visual,
            },
        ))
        .id();
    commands.entity(e).add_child(visual);
    if let Some(damaged) = damaged {
        commands.entity(e).add_child(damaged);
    }
    commands.entity(visual).insert(visuals::Visual {
        owner: e,
        original: None,
        damaged,
        prepared: false,
    });
    e
}
fn remember(mut commands: Commands, bodies: Query<(Entity, &Velocity, Option<&mut Before>)>) {
    for (e, v, before) in bodies {
        if let Some(mut b) = before {
            b.0 = v.linear;
        } else {
            commands.entity(e).insert(Before(v.linear));
        }
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn impacts(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    tuning: Res<Tuning>,
    mut contacts: ResMut<Contacts>,
    mut stats: ResMut<Stats>,
    mut events: MessageReader<CollisionEvent>,
    mut swept: MessageReader<SweptContact>,
    context: ReadRapierContext,
    mut props: Query<(Entity, &Transform, &mut Breakable)>,
    sources: Query<
        (
            Entity,
            &Transform,
            &Collider,
            Option<&Velocity>,
            Option<&Before>,
            Option<&PreviousMotion>,
            Option<&Rider>,
            Option<&Traffic>,
            Option<&OwnedBody>,
        ),
        (
            Or<(With<Rider>, With<Traffic>, With<OwnedBody>, With<Velocity>)>,
            Without<ColliderDisabled>,
            Without<RigidBodyDisabled>,
        ),
    >,
    ground: Query<(), With<crate::world::Ground>>,
    loose: Query<(), With<Loose>>,
    mut audio: MessageWriter<AudioCue>,
    masses: Query<&AdditionalMassProperties>,
    strikes: (
        MessageReader<crate::combat::AttackAttempt>,
        MessageReader<crate::combat::StrikeContact>,
        Query<&crate::recovery::Recovery>,
        Res<Time<Fixed>>,
    ),
) {
    let (mut attacks, mut contacts_at_key, recoveries, fixed) = strikes;
    let mut pairs: Vec<_> = events
        .read()
        .filter_map(|e| {
            if let CollisionEvent::Started(a, b, _) = e {
                Some((*a, *b))
            } else {
                None
            }
        })
        .collect();
    pairs.extend(swept.read().map(|c| (c.0, c.1)));
    if race.phase != Phase::Racing {
        attacks.clear();
        contacts_at_key.clear();
        return;
    }
    contacts.time += time.delta_secs();
    let now = contacts.time;
    contacts.pairs.retain(|_, t| now - *t < 2.);
    let velocity = |v: Option<&Velocity>,
                    b: Option<&Before>,
                    p: Option<&PreviousMotion>,
                    r: Option<&Rider>,
                    car: Option<&Traffic>,
                    t: &Transform| {
        if let Some(r) = r {
            r.motion_velocity
        } else if let Some(c) = car {
            t.rotation * Vec3::Z * c.speed.abs()
        } else {
            b.map(|v| v.0)
                .or_else(|| p.map(|v| v.0))
                .or_else(|| v.map(|v| v.linear))
                .unwrap_or_default()
        }
    };
    // Sweep one fixed step against all intervening solid geometry, never through a wall.
    // Releasing the prop before Rapier's solve transfers contact momentum without a launch impulse.
    if let Ok(ctx) = context.single() {
        for (e, t, c, v, b, p, r, car, owned) in &sources {
            if r.is_none() && car.is_none() && owned.is_none() && v.is_none() {
                continue;
            }
            let speed = velocity(v, b, p, r, car, t);
            if speed.length() < tuning.0.min_speed_mps {
                continue;
            }
            let (lo, hi) = crate::recovery::collider_bounds(&Transform::IDENTITY, c);
            let center = (lo + hi) * 0.5;
            let half = ((hi - lo) * 0.5 - Vec3::splat(0.015)).max(Vec3::splat(0.02));
            let hull = Collider::cuboid(half.x, half.y, half.z);
            let pred = |other, _: &bevy_rapier3d::rapier::geometry::Collider| {
                !ground.contains(other)
                    && !sources.get(other).is_ok_and(|s| {
                        owned.is_some_and(|o| {
                            s.8.is_some_and(|x| x.owner == o.owner) || other == o.owner
                        })
                    })
            };
            if let Some((hit, _)) = ctx.cast_shape(
                t.translation + t.rotation * center,
                t.rotation,
                speed * time.delta_secs(),
                hull.raw.as_ref(),
                bevy_rapier3d::parry::query::ShapeCastOptions {
                    max_time_of_impact: 1.,
                    target_distance: 0.01,
                    stop_at_penetration: false,
                    ..default()
                },
                QueryFilter::default()
                    .exclude_rigid_body(e)
                    .exclude_sensors()
                    .predicate(&pred),
            ) && props.contains(hit)
            {
                pairs.push((e, hit));
            }
        }
    }
    let mut strikes = HashMap::new();
    let attacks = attacks
        .read()
        .map(|&crate::combat::AttackAttempt(e, action)| (e, action, None))
        .chain(
            contacts_at_key
                .read()
                .map(|&crate::combat::StrikeContact(e, action, side)| (e, action, Some(side))),
        );
    for (attacker, action, chosen_side) in attacks {
        if recoveries.get(attacker).is_ok_and(|state| {
            state.stage != crate::recovery::RecoveryStage::Riding || state.stagger.is_some()
        }) {
            continue;
        }
        let Ok((_, t, _, _, _, _, Some(rider), _, _)) = sources.get(attacker) else {
            continue;
        };
        if rider.finished.is_some()
            || (chosen_side.is_some()
                && rider.strike.is_none_or(|clock| {
                    !clock.pending()
                        || clock.age < clock.contact
                        || clock.age - fixed.delta_secs() >= clock.contact
                }))
        {
            continue;
        }
        let origin = t.translation + Vec3::Y * 1.45;
        let forward = t.rotation * Vec3::Z;
        let right = forward.cross(Vec3::Y).normalize_or_zero();
        let weapon = chosen_side.and_then(|_| rider.strike.and_then(|s| s.weapon));
        let kicking = matches!(action, rr_core::AttackSide::Kick);
        let target = props
            .iter()
            .filter(|(_, _, p)| !p.state.broken)
            .filter_map(|(e, t, p)| {
                let point = t.translation + Vec3::Y * (p.size.y * 0.5).min(1.1);
                let delta = point - origin;
                if !weapon.map_or_else(
                    || {
                        rr_core::direct_strike_geometry(
                            delta.dot(right),
                            delta.dot(forward),
                            delta.y,
                            if kicking {
                                chosen_side.unwrap_or_else(|| delta.dot(right).signum())
                            } else {
                                action.sign()
                            },
                        )
                    },
                    |a| {
                        a.reaches(
                            delta.dot(right),
                            delta.dot(forward),
                            delta.y,
                            chosen_side.unwrap_or(action.sign()),
                        )
                    },
                ) {
                    return None;
                }
                let clear = context.single().is_ok_and(|ctx| {
                    ctx.cast_ray(
                        origin,
                        delta.normalize_or_zero(),
                        delta.length() + 0.05,
                        true,
                        QueryFilter::default()
                            .exclude_rigid_body(attacker)
                            .exclude_sensors(),
                    )
                    .is_some_and(|(hit, _)| hit == e)
                });
                clear.then_some((e, delta.length_squared()))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((target, _)) = target {
            if chosen_side.is_some() {
                commands.queue(move |world: &mut World| {
                    if let Some(mut rider) = world.get_mut::<crate::game::Rider>(attacker)
                        && let Some(clock) = rider.strike.as_mut()
                    {
                        clock.consume();
                    }
                });
            }
            strikes.insert((attacker, target), if kicking { 350. } else { 140. });
            pairs.push((attacker, target));
        }
    }
    pairs.sort_unstable_by_key(|(a, b)| (a.to_bits(), b.to_bits()));
    pairs.dedup();
    let mut active = loose.iter().count();
    for (a, b) in pairs {
        for (source, target) in [(a, b), (b, a)] {
            let Ok((_, t, _, v, before, previous, r, car, owned)) = sources.get(source) else {
                continue;
            };
            if r.is_none() && car.is_none() && owned.is_none() && v.is_none() {
                continue;
            }
            let Ok((e, pose, mut prop)) = props.get_mut(target) else {
                continue;
            };
            if prop.state.broken {
                continue;
            }
            let key = (source, target);
            if contacts
                .pairs
                .get(&key)
                .is_some_and(|last| now - *last < tuning.0.cooldown_s)
            {
                continue;
            }
            let speed = velocity(v, before, previous, r, car, t);
            let delta = (pose.translation - t.translation)
                .with_y(0.)
                .normalize_or_zero();
            let closing = if delta == Vec3::ZERO {
                speed.length()
            } else {
                speed.dot(delta).max(0.)
            };
            let mass = if let Ok(AdditionalMassProperties::Mass(m)) = masses.get(source) {
                *m
            } else if car.is_some() {
                1100.
            } else if owned.is_some_and(|o| o.kind == crate::recovery::BodyKind::Ragdoll) {
                15.
            } else if r.is_some() || owned.is_some() {
                180.
            } else {
                20.
            };
            let energy = strikes
                .get(&key)
                .copied()
                .unwrap_or_else(|| impact_energy(mass, closing, tuning.0.min_speed_mps));
            if energy <= 0. {
                continue;
            }
            contacts.pairs.insert(key, now);
            let capacity = prop.profile.durability_j;
            if !prop.state.hit(energy, capacity) {
                continue;
            }
            stats.breaks += 1;
            commands.entity(prop.visual).insert(FrozenVisual);
            let mut shape = prop.shape.clone();
            if prop.profile.style == Style::Crush {
                let s = prop.size * Vec3::new(1., 0.38, 1.);
                shape = Collider::compound(vec![(
                    Vec3::Y * s.y * 0.5,
                    Quat::IDENTITY,
                    Collider::cuboid(s.x * 0.5, s.y * 0.5, s.z * 0.5),
                )]);
            }
            if active < tuning.0.max_dynamic_bodies {
                commands.entity(e).insert((
                    RigidBody::Dynamic,
                    shape,
                    Velocity::zero(),
                    Before::default(),
                    AdditionalMassProperties::Mass(prop.profile.mass_kg),
                    Damping {
                        linear_damping: 0.6,
                        angular_damping: 1.4,
                    },
                    Ccd::enabled(),
                    Loose {
                        age: 0.,
                        soft: prop.profile.soft,
                    },
                ));
                active += 1;
            } else {
                // Supported static wreckage once the live-debris budget is full; never despawn damage.
                let mut fallen = *pose;
                if prop.profile.style == Style::Topple {
                    fallen.rotation *= Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
                    let (lo, _) = crate::recovery::collider_bounds(&fallen, &shape);
                    fallen.translation.y += pose.translation.y - lo.y + 0.01;
                }
                commands.entity(e).insert((shape, fallen));
            }
            audio.write(AudioCue::new(Cue::Break, pose.translation));
        }
    }
    stats.peak_dynamic = stats.peak_dynamic.max(active);
}
fn reset(
    mut commands: Commands,
    mut events: MessageReader<Reset>,
    mut props: Query<(Entity, &mut Breakable)>,
    mut contacts: ResMut<Contacts>,
    mut stats: ResMut<Stats>,
    mut body_sets: Query<&mut RapierRigidBodySet>,
    mut nav: Option<ResMut<crate::world_gen::metropolis::navigation::Navigation>>,
) {
    if events.read().count() == 0 {
        return;
    }
    contacts.pairs.clear();
    *stats = Stats::default();
    if let Some(nav) = nav.as_deref_mut() {
        nav.reset_props();
    }
    for (e, mut p) in &mut props {
        p.state.reset();
        for mut set in &mut body_sets {
            if let Some(handle) = set.entity2body().get(&e).copied()
                && let Some(body) = set.bodies.get_mut(handle)
            {
                let location = bevy_rapier3d::rapier::math::Pose3::from_parts(
                    p.original.translation.to_array().into(),
                    bevy_rapier3d::rapier::math::Rot3::from_xyzw(
                        p.original.rotation.x,
                        p.original.rotation.y,
                        p.original.rotation.z,
                        p.original.rotation.w,
                    ),
                );
                body.set_body_type(bevy_rapier3d::rapier::dynamics::RigidBodyType::Fixed, true);
                body.set_position(location, true);
                body.set_next_kinematic_position(location);
                body.set_linvel(Vec3::ZERO, true);
                body.set_angvel(Vec3::ZERO, true);
                body.reset_forces(true);
                body.reset_torques(true);
            }
        }

        commands
            .entity(e)
            .insert((
                p.original,
                p.shape.clone(),
                RigidBody::Fixed,
                Velocity::zero(),
            ))
            .remove::<(Loose, Before, AdditionalMassProperties, Damping, Ccd)>();
        commands.entity(p.visual).remove::<FrozenVisual>();
    }
}
#[allow(clippy::too_many_arguments)]
fn settle(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    tuning: Res<Tuning>,
    context: ReadRapierContext,
    mut loose: Query<(Entity, &Velocity, &mut Loose)>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    for (e, v, mut loose) in &mut loose {
        loose.age += time.delta_secs();
        if loose.age >= tuning.0.settle_after_s
            && v.linear.length_squared() < 0.16
            && v.angular.length_squared() < 0.04
            && context.single().is_ok_and(|ctx| {
                ctx.contact_pairs_with(e)
                    .any(|p| p.has_any_active_contact())
            })
        {
            commands
                .entity(e)
                .insert((RigidBody::Fixed, Velocity::zero()))
                .remove::<Loose>();
        }
    }
}

fn sync_navigation(
    props: Query<(&Transform, &Collider, &Breakable), Changed<Transform>>,
    mut nav: Option<ResMut<crate::world_gen::metropolis::navigation::Navigation>>,
) {
    let Some(nav) = nav.as_deref_mut() else {
        return;
    };
    for (t, c, p) in &props {
        if p.state.broken
            && let Some(slot) = p.slot
        {
            let (lo, hi) = crate::recovery::collider_bounds(t, c);
            nav.move_prop(slot, lo.xz(), hi.xz());
        }
    }
}
