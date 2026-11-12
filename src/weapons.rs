//! Bounded original weapon inventory and presentation. No weapon rigid bodies.
use crate::{
    controls::Controls,
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    world::Art,
};
use bevy::prelude::*;
#[derive(Resource)]
pub struct GridWeapons {
    pub seed: u64,
    pub race_number: u64,
    pub armed: Vec<usize>,
    pub fraction: f32,
}
impl Default for GridWeapons {
    fn default() -> Self {
        use std::hash::{BuildHasher, Hasher};
        let seed = std::env::var("ROADDASH_RACE_SEED")
            .map(|v| {
                v.parse()
                    .expect("ROADDASH_RACE_SEED must be an unsigned 64-bit integer")
            })
            .unwrap_or_else(|_| {
                std::collections::hash_map::RandomState::new()
                    .build_hasher()
                    .finish()
            });
        Self {
            seed,
            race_number: 0,
            armed: Vec::new(),
            fraction: rr_content::weapons::Weapons::builtin().starting_armed_fraction,
        }
    }
}
impl GridWeapons {
    pub fn roll(&mut self, ids: impl IntoIterator<Item = usize>) {
        let mut ids: Vec<_> = ids.into_iter().filter(|id| *id != 0).collect();
        ids.sort_unstable();
        ids.dedup();
        self.armed = ids
            .into_iter()
            .filter(|id| rr_core::cast::character(*id).is_some_and(|c| c.weapon.is_some()))
            .collect();
        self.race_number = self.race_number.wrapping_add(1);
        info!(
            "Cast starting weapons: legacy-seed={} race={} riders={:?}",
            self.seed, self.race_number, self.armed
        );
    }
}
pub fn ability() -> rr_core::weapons::Ability {
    static DATA: std::sync::OnceLock<rr_core::weapons::Ability> = std::sync::OnceLock::new();
    *DATA.get_or_init(|| rr_content::weapons::Weapons::builtin().club)
}
#[derive(Resource)]
pub struct WeaponRules(
    pub rr_core::weapons::Ability,
    pub rr_content::weapons::Weapons,
);
impl WeaponRules {
    pub fn get(&self, weapon: rr_core::weapons::Weapon) -> rr_core::weapons::Ability {
        if weapon == rr_core::weapons::Weapon::Club {
            self.0
        } else {
            self.1.ability(weapon)
        }
    }
}
impl Default for WeaponRules {
    fn default() -> Self {
        Self(ability(), rr_content::weapons::Weapons::builtin())
    }
}
#[derive(Component)]
pub struct Pickup;
#[derive(Component)]
pub(crate) struct Held {
    owner: Entity,
    side: usize,
}
/// Three original authored parts, +Y handle axis, shared existing primitive GPU assets.
#[derive(Component)]
pub(crate) struct ClubPart;
fn club(commands: &mut Commands, art: &Art, parent: Entity) {
    commands
        .entity(parent)
        .insert(crate::weapon_models::ModelRoot);
    for (position, scale, metal) in [
        (
            Vec3::new(0., 0.10, 0.),
            Vec3::new(0.055, 0.28, 0.055),
            false,
        ),
        (Vec3::new(0., 0.39, 0.), Vec3::new(0.11, 0.34, 0.10), false),
        (
            Vec3::new(0., 0.57, 0.),
            Vec3::new(0.115, 0.035, 0.105),
            true,
        ),
    ] {
        let child = crate::world::part(
            commands,
            art.cube.clone(),
            if metal {
                art.metal.clone()
            } else {
                art.dark.clone()
            },
            position,
            scale,
            Quat::IDENTITY,
        );
        commands.entity(child).insert(ClubPart);
        commands.entity(parent).add_child(child);
    }
}
pub(crate) fn pickup(commands: &mut Commands, art: &Art, position: Vec3) {
    pickup_kind(commands, art, position, rr_core::weapons::Weapon::Club);
}
#[derive(Component, Clone, Copy)]
pub struct Kind(pub rr_core::weapons::Weapon);
pub(crate) fn pickup_kind(
    commands: &mut Commands,
    art: &Art,
    position: Vec3,
    weapon: rr_core::weapons::Weapon,
) {
    let root = commands
        .spawn((
            Name::new("Original club pickup"),
            Pickup,
            Kind(weapon),
            Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            Visibility::Visible,
        ))
        .id();
    club(commands, art, root);
}
fn supported(
    position: Vec3,
    fallback_y: f32,
    context: &bevy_rapier3d::prelude::ReadRapierContext,
) -> Vec3 {
    let origin = position;
    let y = context
        .single()
        .ok()
        .and_then(|ctx| {
            ctx.cast_ray(
                origin,
                Vec3::NEG_Y,
                12.0,
                true,
                bevy_rapier3d::prelude::QueryFilter::only_fixed().exclude_sensors(),
            )
        })
        .map_or(fallback_y.min(position.y), |(_, distance)| {
            origin.y - distance
        });
    Vec3::new(position.x, y + 0.075, position.z)
}
/// Fixed before drive/scheduler: OR-buffered input is consumed once, nearest available
/// pickup wins in stable rider order; a windup may never swap its loadout.
#[allow(clippy::too_many_arguments)]
pub fn inventory(
    mut commands: Commands,
    art: Res<Art>,
    rules: Res<WeaponRules>,
    race: Res<Race>,
    track: Res<crate::track::Track>,
    mut controls: ResMut<Controls>,
    mut riders: Query<(Entity, &mut Rider, &Recovery, &Transform)>,
    pickups: Query<(Entity, &Transform, Option<&Kind>), With<Pickup>>,
    mut seeded: Local<bool>,
    network: Option<Res<crate::online::AuthorityInputs>>,
    context: bevy_rapier3d::prelude::ReadRapierContext,
) {
    if race.phase == Phase::Garage || race.phase == Phase::Countdown {
        if !*seeded {
            for (e, _, _) in &pickups {
                commands.entity(e).despawn();
            }
            for (index, (distance, lane)) in [
                (25., -1.7),
                (25., 1.7),
                (160., -2.8),
                (160., 2.8),
                (420., -2.8),
                (420., 2.8),
            ]
            .into_iter()
            .enumerate()
            {
                let (p, _) = track.sample_route(0, distance, lane);
                pickup_kind(
                    &mut commands,
                    &art,
                    supported(p, track.terrain_height(p.x, p.z), &context),
                    rr_core::weapons::Weapon::ALL[index % 5],
                );
            }
            *seeded = true;
        }
        controls.pickup = false;
        controls.drop = false;
        return;
    }
    if race.phase != Phase::Racing {
        return;
    }
    *seeded = false;
    let take = std::mem::take(&mut controls.pickup);
    let drop = std::mem::take(&mut controls.drop);
    let mut claimed = std::collections::HashSet::new();
    let mut count = pickups.iter().count();
    let mut entities: Vec<_> = riders.iter().map(|(e, r, _, _)| (r.id, e)).collect();
    entities.sort();
    for (_, e) in entities {
        let Ok((_, mut rider, rec, t)) = riders.get_mut(e) else {
            continue;
        };
        let remote = network.as_ref().and_then(|n| n.get(rider.id));
        let take = remote.map_or(take, |i| i.pickup);
        let drop = remote.map_or(drop, |i| i.drop);
        let interrupted =
            rec.stage != RecoveryStage::Riding || rec.stagger.is_some() || rider.finished.is_some();
        if rider.weapon.is_some() && (interrupted || (rider.human_controlled() && drop)) {
            // Manual drop waits for completed contact; crash cancels the pending clock.
            if !interrupted && rider.strike.is_some_and(|s| s.pending()) {
                if rider.id == 0 {
                    controls.drop = true;
                }
                continue;
            }
            let dropped = rider.weapon.take().unwrap();
            if interrupted {
                rider.strike = None;
            }
            if count < 12 {
                pickup_kind(
                    &mut commands,
                    &art,
                    supported(
                        t.translation,
                        track.terrain_height(t.translation.x, t.translation.z),
                        &context,
                    ),
                    dropped,
                );
                count += 1;
            }
            continue;
        }
        if interrupted
            || rider.weapon.is_some()
            || rider.police
            || rider.strike.is_some_and(|s| s.pending())
            || (rider.human_controlled() && !take)
        {
            continue;
        }
        if let Some((item, _, weapon)) = pickups
            .iter()
            .filter(|(item, _, _)| !claimed.contains(item))
            .filter_map(|(item, p, kind)| {
                let d = t.translation.distance_squared(p.translation);
                let origin = t.translation + Vec3::Y * 0.5;
                let delta = p.translation + Vec3::Y * 0.15 - origin;
                let clear = context.single().is_ok_and(|ctx| {
                    ctx.cast_ray(
                        origin,
                        delta.normalize_or_zero(),
                        delta.length(),
                        true,
                        bevy_rapier3d::prelude::QueryFilter::default()
                            .exclude_rigid_body(e)
                            .exclude_sensors(),
                    )
                    .is_none()
                });
                (d <= rules
                    .get(kind.map_or(rr_core::weapons::Weapon::Club, |k| k.0))
                    .pickup_radius
                    .powi(2)
                    && clear)
                    .then_some((
                        item,
                        d,
                        kind.map_or(rr_core::weapons::Weapon::Club, |k| k.0),
                    ))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
        {
            claimed.insert(item);
            commands.entity(item).despawn();
            rider.weapon = Some(weapon);
        }
    }
}
/// Instantiate before transform propagation, never in PostUpdate. No async loading:
/// the same resident shared GPU primitives back both held and ground clubs.
pub fn attach(
    mut commands: Commands,
    art: Res<Art>,
    sockets: Query<
        (Entity, &crate::models::hands::HandSocket),
        Added<crate::models::hands::HandSocket>,
    >,
    parents: Query<&ChildOf>,
    riders: Query<&Rider>,
) {
    for (socket, s) in &sockets {
        if !s.weapon || !s.seated {
            continue;
        }
        let mut ancestor = socket;
        for _ in 0..10 {
            if riders.get(ancestor).is_ok_and(|r| !r.police) {
                let root = commands
                    .spawn((
                        Name::new("Original held club"),
                        Held {
                            owner: ancestor,
                            side: s.side,
                        },
                        Kind(rr_core::weapons::Weapon::Club),
                        Transform::default(),
                        Visibility::Hidden,
                    ))
                    .id();
                club(&mut commands, &art, root);
                commands.entity(socket).add_child(root);
                break;
            }
            let Ok(p) = parents.get(ancestor) else { break };
            ancestor = p.parent();
        }
    }
}
fn visuals(
    riders: Query<(&Rider, &Recovery)>,
    mut held: Query<(&Held, &mut Visibility, Option<&mut Kind>)>,
) {
    for (h, mut visible, kind) in &mut held {
        if let Some(mut kind) = kind
            && let Ok((r, _)) = riders.get(h.owner)
        {
            kind.0 = r.weapon.unwrap_or(rr_core::weapons::Weapon::Club);
        }
        *visible = if riders.get(h.owner).is_ok_and(|(r, rec)| {
            r.weapon.is_some()
                && rec.stage == RecoveryStage::Riding
                && rec.stagger.is_none()
                && h.side == r.weapon_hand
        }) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
pub struct WeaponVisualPlugin;
impl Plugin for WeaponVisualPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::weapon_models::Selection>()
            .add_systems(Startup, crate::weapon_models::load)
            .add_systems(
                Update,
                (attach, visuals, crate::weapon_models::apply).chain(),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fresh_grid_and_retry_restore_cast_loadouts_and_pause_does_not_rearm() {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<GridWeapons>().seed = 1234;
        for id in 1..=crate::police::OFFICER_ID {
            app.world_mut().spawn((
                Rider::new(id),
                Recovery::default(),
                Transform::default(),
                Visibility::Visible,
            ));
        }
        run(&mut app, 4);
        press(&mut app, KeyCode::Enter);
        run(&mut app, 3);
        let first = app.world().resource::<GridWeapons>().armed.clone();
        assert_eq!(first.len(), 8);
        assert_eq!(app.world().resource::<GridWeapons>().race_number, 1);
        for r in app.world_mut().query::<&Rider>().iter(app.world()) {
            assert_eq!(
                r.weapon,
                rr_core::cast::character(r.id).and_then(|c| c.weapon)
            );
            if r.id == 0 || r.police {
                assert!(r.weapon.is_none());
            }
        }
        let armed = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.weapon.is_some())
            .unwrap()
            .0;
        app.world_mut().get_mut::<Rider>(armed).unwrap().weapon = None;
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        run(&mut app, 3);
        app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
        run(&mut app, 3);
        assert!(app.world().get::<Rider>(armed).unwrap().weapon.is_none());
        assert_eq!(app.world().resource::<GridWeapons>().race_number, 1);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        press(&mut app, KeyCode::KeyR);
        run(&mut app, 3);
        let next = &app.world().resource::<GridWeapons>().armed;
        assert_eq!(next.len(), 8);
        assert_eq!(&first, next);
        assert_eq!(app.world().resource::<GridWeapons>().race_number, 2);
    }
    #[test]
    fn every_family_keeps_identity_through_pickup_strike_and_drop() {
        for weapon in rr_core::weapons::Weapon::ALL {
            for hz in [30, 60, 120, 144] {
                for distance in [
                    2.0,
                    rr_content::weapons::Weapons::builtin()
                        .ability(weapon)
                        .lateral_reach
                        + 0.5,
                ] {
                    let (mut app, owner, target) = fixture(hz, 1., distance);
                    let items: Vec<_> = app
                        .world_mut()
                        .query_filtered::<Entity, With<Pickup>>()
                        .iter(app.world())
                        .collect();
                    for item in items {
                        app.world_mut().entity_mut(item).insert(Kind(weapon));
                    }
                    press(&mut app, KeyCode::KeyE);
                    run(&mut app, 5);
                    assert_eq!(
                        app.world().get::<Rider>(owner).unwrap().weapon,
                        Some(weapon)
                    );
                    press(&mut app, KeyCode::KeyK);
                    for _ in 0..5 {
                        app.update();
                        if app.world().get::<Rider>(owner).unwrap().strike.is_some() {
                            break;
                        }
                    }
                    let clock = app
                        .world()
                        .get::<Rider>(owner)
                        .unwrap()
                        .strike
                        .expect("buffered family attack must start within five render frames");
                    let expected = app.world().resource::<WeaponRules>().get(weapon);
                    assert_eq!(clock.weapon.unwrap().lateral_reach, expected.lateral_reach);
                    assert!(
                        app.world().get::<Rider>(owner).unwrap().attack > expected.cooldown - 0.1
                    );
                    run(&mut app, hz / 2);
                    assert_eq!(
                        app.world().get::<Rider>(target).unwrap().hits.count,
                        u8::from(expected.lateral_reach >= distance)
                    );
                    // Separate the rival before drop so a coarse render frame
                    // cannot also give it the newly dropped pickup on a second tick.
                    app.world_mut().resource_mut::<Match>().distance = 6.;
                    run(&mut app, 3);
                    press(&mut app, KeyCode::KeyQ);
                    for _ in 0..5 {
                        app.update();
                        if app.world().get::<Rider>(owner).unwrap().weapon.is_none() {
                            break;
                        }
                    }
                    assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_none());
                    let dropped: Vec<_> = app
                        .world_mut()
                        .query_filtered::<&Kind, With<Pickup>>()
                        .iter(app.world())
                        .map(|k| k.0)
                        .collect();
                    assert_eq!(dropped, vec![weapon]);
                }
            }
        }
    }
    #[derive(Resource)]
    struct Match {
        side: f32,
        distance: f32,
    }
    fn align(mut riders: Query<(&mut Rider, &mut Transform)>, fixture: Res<Match>) {
        let (p, s, v) = riders
            .iter()
            .find(|(r, _)| r.id == 0)
            .map(|(r, t)| (*t, r.speed, r.motion_velocity))
            .unwrap();
        for (mut r, mut t) in &mut riders {
            if r.id != 0 {
                *t = p;
                t.translation +=
                    (p.rotation * Vec3::Z).cross(Vec3::Y) * fixture.side * fixture.distance;
                r.speed = s;
                r.motion_velocity = v;
            }
        }
    }
    fn fixture(hz: u32, side: f32, distance: f32) -> (App, Entity, Entity) {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..3 {
            app.update();
        }
        app.add_plugins(crate::animation::MotionPlugin);
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / hz as f64),
        ));
        app.insert_resource(Match { side, distance });
        app.add_systems(
            FixedUpdate,
            align
                .after(crate::game::drive)
                .before(crate::combat::schedule),
        );
        let owner = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        app.world_mut().get_mut::<Rider>(owner).unwrap().speed = 15.;
        let p = *app.world().get::<Transform>(owner).unwrap();
        app.world_mut().spawn((Pickup, p, Visibility::Visible));
        let mut rival = Rider::new(1);
        rival.immunity = 0.;
        let target = app.world_mut().spawn((rival, p, Visibility::Visible)).id();
        (app, owner, target)
    }
    fn press(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
    }
    fn run(app: &mut App, n: u32) {
        for _ in 0..n {
            app.update();
        }
    }
    #[test]
    fn real_buffered_pickup_club_pose_contact_and_matched_speed_whiffs() {
        for hz in [30, 60, 120, 144] {
            for side in [-1., 1.] {
                for distance in [2.4, 3.05] {
                    let (mut app, owner, target) = fixture(hz, side, distance);
                    press(&mut app, KeyCode::KeyE);
                    run(&mut app, hz / 8);
                    assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_some());
                    let stamina = app.world().get::<Rider>(owner).unwrap().fight_stamina;
                    press(
                        &mut app,
                        if side < 0. {
                            KeyCode::KeyJ
                        } else {
                            KeyCode::KeyK
                        },
                    );
                    let mut observed = false;
                    for _ in 0..hz / 2 {
                        app.update();
                        let r = app.world().get::<Rider>(owner).unwrap();
                        if let Some(clock) = r.strike {
                            assert!(clock.weapon.is_some());
                            assert!(
                                (clock.contact
                                    - (rr_core::motion::ATTACK_ENTER
                                        + ability().source_contact
                                            / rr_core::motion::ATTACK_PLAYBACK))
                                    .abs()
                                    < 1e-6
                            );
                            let hits = app.world().get::<Rider>(target).unwrap().hits.count;
                            if clock.age < clock.contact {
                                assert_eq!(hits, 0);
                            } else {
                                assert_eq!(hits, u8::from(distance < 2.55));
                                if !observed {
                                    let actual = app
                                        .world()
                                        .get::<crate::animation::Animator>(owner)
                                        .unwrap()
                                        .current;
                                    let expected =
                                        app.world().resource::<crate::animation::Library>().sample(
                                            if side < 0. { "club_left" } else { "club_right" },
                                            ability().source_contact,
                                        );
                                    for (a, b) in
                                        actual.rotations.into_iter().zip(expected.rotations)
                                    {
                                        assert!(
                                            a.dot(b).abs() > 0.99999,
                                            "club contact pose differs"
                                        );
                                    }
                                    assert_eq!(actual.hands, [[1., 0.]; 2]);
                                }
                                observed = true;
                            }
                            if clock.age < 0.1 {
                                assert!(r.fight_stamina < stamina - 270.);
                                assert!(r.attack > 0.7);
                            }
                        }
                    }
                    assert!(observed);
                    let r = app.world().get::<Rider>(owner).unwrap();
                    assert!(r.speed > 10.);
                    assert!(
                        (r.speed - app.world().get::<Rider>(target).unwrap().speed).abs() < 0.01
                    );
                    let pose = app
                        .world()
                        .get::<crate::animation::Animator>(owner)
                        .unwrap()
                        .current;
                    assert!(pose.hands.iter().flatten().all(|x| x.is_finite()));
                    println!(
                        "club input/pickup side={side} target={distance} render_hz={hz} hits={}",
                        app.world().get::<Rider>(target).unwrap().hits.count
                    );
                }
            }
        }
    }
    #[test]
    fn pending_drop_is_deferred_crash_cancels_and_retry_cleans_inventory() {
        let (mut app, owner, target) = fixture(120, 1., 2.4);
        press(&mut app, KeyCode::KeyE);
        run(&mut app, 10);
        press(&mut app, KeyCode::KeyK);
        run(&mut app, 4);
        press(&mut app, KeyCode::KeyQ);
        run(&mut app, 3);
        assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_some());
        assert!(
            app.world()
                .get::<Rider>(owner)
                .unwrap()
                .strike
                .unwrap()
                .weapon
                .is_some()
        );
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::Falling;
        run(&mut app, 40);
        assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_none());
        assert!(app.world().get::<Rider>(owner).unwrap().strike.is_none());
        assert_eq!(app.world().get::<Rider>(target).unwrap().hits.count, 0);
        assert!(
            app.world_mut()
                .query_filtered::<Entity, With<Pickup>>()
                .iter(app.world())
                .count()
                <= 12
        );
        press(&mut app, KeyCode::KeyR);
        run(&mut app, 5);
        assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_none());
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<Pickup>>()
                .iter(app.world())
                .count(),
            6
        );
        let families: std::collections::BTreeSet<_> = app
            .world_mut()
            .query_filtered::<&Kind, With<Pickup>>()
            .iter(app.world())
            .map(|k| k.0.id())
            .collect();
        assert_eq!(
            families.len(),
            5,
            "retry must restore the entire route arsenal"
        );
    }
    #[test]
    fn authored_rule_changes_control_live_start_contact_reach_and_snapshot() {
        let (mut app, owner, target) = fixture(60, 1., 2.4);
        let text = include_str!("../content/weapons.json")
            .replace("0.32", "0.5")
            .replace("0.85", "1.2")
            .replace("300.0", "500.0")
            .replace("2.55", "2.0");
        let data = rr_content::weapons::Weapons::from_json(&text).unwrap();
        app.world_mut().resource_mut::<WeaponRules>().0 = data.club;
        press(&mut app, KeyCode::KeyE);
        run(&mut app, 5);
        let stamina = app.world().get::<Rider>(owner).unwrap().fight_stamina;
        press(&mut app, KeyCode::KeyK);
        run(&mut app, 2);
        let r = app.world().get::<Rider>(owner).unwrap();
        assert!(r.attack > 1.1);
        assert!(r.fight_stamina < stamina - 490.);
        let clock = r.strike.unwrap();
        assert!((clock.contact - (0.055 + 0.5 / 1.8)).abs() < 1e-6);
        app.world_mut().resource_mut::<WeaponRules>().0 = ability(); // in-flight reach stays2m
        run(&mut app, 30);
        assert_eq!(app.world().get::<Rider>(target).unwrap().hits.count, 0);
        app.world_mut().get_mut::<Rider>(owner).unwrap().attack = 0.;
        app.world_mut()
            .get_mut::<Rider>(owner)
            .unwrap()
            .fight_stamina = 0.;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyK);
        app.update();
        press(&mut app, KeyCode::KeyK);
        app.update();
        assert_eq!(app.world().get::<Rider>(owner).unwrap().attack_serial, 1);
    }
    #[test]
    fn club_contact_respects_los_immunity_and_side() {
        for (wall, immune, side) in [(true, false, 1.), (false, true, 1.), (false, false, -1.)] {
            let (mut app, owner, target) = fixture(60, side, 2.4);
            if wall {
                let p = *app.world().get::<Transform>(owner).unwrap();
                let right = (p.rotation * Vec3::Z).cross(Vec3::Y);
                app.world_mut().spawn((
                    bevy_rapier3d::prelude::RigidBody::Fixed,
                    bevy_rapier3d::prelude::Collider::cuboid(0.1, 3., 20.),
                    Transform::from_translation(p.translation + right * 1.2)
                        .with_rotation(p.rotation),
                ));
            }
            if immune {
                app.world_mut().get_mut::<Rider>(target).unwrap().immunity = 2.;
            }
            press(&mut app, KeyCode::KeyE);
            run(&mut app, 5);
            press(&mut app, KeyCode::KeyK);
            run(&mut app, 30);
            assert_eq!(app.world().get::<Rider>(target).unwrap().hits.count, 0);
        }
    }
    #[test]
    fn role_free_rival_obtains_same_club_and_uses_same_tell_in_close_combat() {
        let (mut app, owner, target) = fixture(60, 1., 2.4);
        // Target wins this pickup; player input is deliberately absent.
        let mut p = *app.world().get::<Transform>(owner).unwrap();
        p.translation += (p.rotation * Vec3::Z).cross(Vec3::Y) * 2.4;
        app.world_mut().spawn((Pickup, p, Visibility::Visible));
        run(&mut app, 3);
        assert!(app.world().get::<Rider>(target).unwrap().weapon.is_some());
        app.world_mut().resource_mut::<Race>().elapsed = 9.;
        // A distant pickup does not create a remote strike.
        run(&mut app, 10);
        assert_eq!(app.world().get::<Rider>(target).unwrap().attack_serial, 0);
        {
            let r = app.world().get::<Rider>(owner).unwrap();
            let progress = r.progress;
            let lane = r.lane;
            let mut rival = app.world_mut().get_mut::<Rider>(target).unwrap();
            rival.progress = progress;
            rival.lane = lane + 2.4;
            rival.combat_mind.struck_by(0);
        }
        app.world_mut().get_mut::<Rider>(owner).unwrap().immunity = 0.;
        let mut contact = false;
        for _ in 0..35 {
            // Side-by-side fixture retains equal live route progress for AI perception.
            let progress = app.world().get::<Rider>(owner).unwrap().progress;
            app.world_mut().get_mut::<Rider>(target).unwrap().progress = progress;
            app.update();
            let r = app.world().get::<Rider>(target).unwrap();
            if let Some(clock) = r.strike {
                assert!(clock.weapon.is_some());
                assert!((clock.contact - (0.055 + ability().source_contact / 1.8)).abs() < 1e-6);
                if clock.pending() {
                    assert_eq!(app.world().get::<Rider>(owner).unwrap().hits.count, 0);
                } else {
                    contact = true;
                }
            }
        }
        assert!(contact);
        assert_eq!(app.world().get::<Rider>(owner).unwrap().hits.count, 1);
        assert!(
            app.world()
                .get::<Rider>(target)
                .unwrap()
                .combat_mind
                .can_fight(0)
        );
    }
    #[test]
    fn drops_use_support_below_live_position_under_roofs_and_on_platforms() {
        use bevy::ecs::system::RunSystemOnce;
        use bevy_rapier3d::prelude::*;
        for (platform_y, live_y) in [(0.0, 1.0), (2.0, 3.0)] {
            let mut app = crate::game::tests::simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let position = Vec3::new(7.25, live_y, 4.5);
            for y in [platform_y - 0.1, live_y + 0.4] {
                app.world_mut().spawn((
                    RigidBody::Fixed,
                    Collider::cuboid(1., 0.1, 1.),
                    Transform::from_xyz(position.x, y, position.z),
                ));
            }
            run(&mut app, 5);
            let dropped = app
                .world_mut()
                .run_system_once(move |context: ReadRapierContext| {
                    supported(position, -1., &context)
                })
                .unwrap();
            assert_eq!(dropped.x, position.x);
            assert_eq!(dropped.z, position.z);
            assert!(
                (dropped.y - (platform_y + 0.075)).abs() < 0.002,
                "{dropped:?}"
            );
        }
    }
    #[test]
    fn pickup_is_blocked_by_solid_wall_for_player_and_rival() {
        for id in [0, 1] {
            let mut app = crate::game::tests::simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let owner = app
                .world_mut()
                .query::<(Entity, &Rider)>()
                .iter(app.world())
                .find(|(_, r)| r.id == 0)
                .unwrap()
                .0;
            let p = *app.world().get::<Transform>(owner).unwrap();
            app.world_mut().get_mut::<Rider>(owner).unwrap().id = id;
            app.world_mut().spawn((
                bevy_rapier3d::prelude::RigidBody::Fixed,
                bevy_rapier3d::prelude::Collider::cuboid(3., 3., 0.1),
                Transform::from_translation(p.translation + Vec3::Z * 0.9),
            ));
            run(&mut app, 5);
            app.world_mut().spawn((
                Pickup,
                Transform::from_translation(p.translation + Vec3::Z * 1.8),
                Visibility::Visible,
            ));
            press(&mut app, KeyCode::KeyE);
            run(&mut app, 5);
            assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_none());
        }
    }
    #[test]
    fn held_club_inherits_hidden_owner_and_kicks_keep_weapon_hand() {
        use bevy::ecs::system::RunSystemOnce;
        let (mut app, owner, _) = fixture(60, -1., 6.0);
        press(&mut app, KeyCode::KeyE);
        run(&mut app, 5);
        let hand = app.world().get::<Rider>(owner).unwrap().weapon_hand;
        let child = app
            .world_mut()
            .spawn((Held { owner, side: hand }, Visibility::Hidden))
            .id();
        press(&mut app, KeyCode::Space);
        run(&mut app, 4);
        assert_eq!(app.world().get::<Rider>(owner).unwrap().weapon_hand, hand);
        *app.world_mut().get_mut::<Visibility>(owner).unwrap() = Visibility::Hidden;
        app.world_mut().run_system_once(visuals).unwrap();
        assert_eq!(
            *app.world().get::<Visibility>(child).unwrap(),
            Visibility::Inherited
        );
        app.world_mut().get_mut::<Rider>(owner).unwrap().weapon = None;
        app.world_mut().run_system_once(visuals).unwrap();
        assert_eq!(
            *app.world().get::<Visibility>(child).unwrap(),
            Visibility::Hidden
        );
    }
    #[test]
    fn queued_drop_occurs_once_after_visible_contact_without_changing_hit() {
        let (mut app, owner, target) = fixture(60, 1., 2.4);
        press(&mut app, KeyCode::KeyE);
        run(&mut app, 5);
        press(&mut app, KeyCode::KeyK);
        run(&mut app, 2);
        press(&mut app, KeyCode::KeyQ);
        app.update();
        assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_some());
        assert!(app.world().resource::<Controls>().drop);
        for _ in 0..20 {
            app.update();
            if app.world().get::<Rider>(target).unwrap().hits.count == 1 {
                break;
            }
        }
        assert_eq!(app.world().get::<Rider>(target).unwrap().hits.count, 1);
        app.world_mut().resource_mut::<Match>().distance = 6.;
        app.update();
        assert!(app.world().get::<Rider>(owner).unwrap().weapon.is_none());
        run(&mut app, 10);
        assert!(!app.world().resource::<Controls>().drop);
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<Pickup>>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(app.world().get::<Rider>(target).unwrap().hits.count, 1);
    }
}
