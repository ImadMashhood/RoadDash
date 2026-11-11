//! Active native review of the rebuilt Hills composition and real bridge exit.
use crate::{
    game::{ChaseCamera, Phase, Race, Rider},
    heavy_traffic::{HeavyTraffic, Kind},
    recovery::OwnedBody,
    track::Track,
    vehicle::{BikeState, VehiclePhysicsAdapter},
    world_gen::hills::{
        Census, HillsFarm, bridge, city_slice::CitySlice, static_collision::StaticCollision,
    },
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    captures: usize,
    player: Option<Entity>,
    fall_deck: f32,
    lowest: f32,
    observed_physical_fall: bool,
    lowest_chassis: f32,
    maximum_downward_speed: f32,
    semi_start: Option<Vec3>,
    tractor_start: Option<Vec3>,
    semi_moved: f32,
    tractor_moved: f32,
}
fn captured(_: On<ScreenshotCaptured>, mut state: ResMut<Survey>) {
    state.captures += 1;
}
const NAMES: [&str; 15] = [
    "city-driving",
    "city-overview",
    "city-to-hills",
    "hill-farm",
    "tight-curves",
    "suspension-canyon",
    "bridge-open-sides",
    "western-west",
    "western-east",
    "western-rejoin",
    "moving-eighteen-wheeler",
    "moving-tractor",
    "sand-dunes",
    "western-backlots",
    "farmyard-detail",
];
const DELIVERY_VIEWS: [&str; 9] = [
    "farm-parcel-1",
    "farm-parcel-2",
    "farm-parcel-3",
    "farm-parcel-4",
    "farm-parcel-5",
    "farm-parcel-6",
    "distinct-sun",
    "canyon-river",
    "city-shadow-range",
];
fn route_view(track: &Track, route: usize, station: f32, wide: bool) -> Transform {
    let (p, q) = track.sample_route(route, station, 0.);
    let eye = if wide {
        p + q * Vec3::new(85., 70., -100.)
    } else {
        p - q * Vec3::Z * 13. + Vec3::Y * 5.
    };
    Transform::from_translation(eye).looking_at(p + q * Vec3::Z * 40. + Vec3::Y * 2., Vec3::Y)
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    adapter: Res<VehiclePhysicsAdapter>,
    mut race: ResMut<Race>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut recoveries: Query<&mut crate::recovery::Recovery>,
    mut objects: Query<
        (
            Entity,
            &mut Transform,
            Option<&mut Rider>,
            Option<&mut Velocity>,
            Option<&HeavyTraffic>,
            Option<&ChaseCamera>,
            Option<&HillsFarm>,
            Option<&OwnedBody>,
            Option<&mut BikeState>,
        ),
        Or<(
            With<Rider>,
            With<HeavyTraffic>,
            With<ChaseCamera>,
            With<HillsFarm>,
            With<OwnedBody>,
        )>,
    >,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    (city, census, solids, metrics, clock, bridge_census, clouds, lighting, river, shadows, solar): (
        Res<CitySlice>,
        Res<Census>,
        Res<StaticCollision>,
        Res<crate::diagnostics::Metrics>,
        Res<crate::runtime::SimulationClock>,
        Res<bridge::BridgeCensus>,
        Option<Res<crate::world_gen::hills::clouds::CloudBudget>>,
        Option<Res<crate::sky::HillsLighting>>,
        Option<Res<crate::world_gen::hills::river::RiverCensus>>,
        Res<crate::world::ShadowBudget>,
        Option<Res<crate::sky::SolarBudget>>,
    ),
    physics: ReadRapierContext,
    ground: Query<(), With<crate::world::Ground>>,
    town: Res<crate::world_gen::hills::TownDetailCensus>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    let frame = state.frame;
    let assisted = *adapter == VehiclePhysicsAdapter::Assisted;
    let perf = std::env::var("ROADDASH_HILLS_PERF").as_deref() == Ok("1");
    let delivery = std::env::var("ROADDASH_HILLS_DELIVERY").as_deref() == Ok("1");
    let view_count = NAMES.len() + if delivery { DELIVERY_VIEWS.len() } else { 0 };
    let period = if perf { 300 } else { 150 };
    let fall_start = view_count as u32 * period + 10;
    race.phase = Phase::Racing;
    keys.clear();
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    let mut semi = None;
    let mut tractor = None;
    let mut farm = None;
    let mut farms = Vec::new();
    let mut follow = None;
    let mut semi_count = 0;
    let mut tractor_count = 0;
    for (entity, mut t, rider, mut velocity, heavy, _, field, owned, mut bike) in &mut objects {
        if let Some(mut r) = rider
            && r.id == 0
        {
            state.player = Some(entity);
            if frame == 1 {
                r.finished = Some(0.);
                r.speed = 0.;
                r.health = 100.;
                r.motion_velocity = Vec3::ZERO;
                t.translation = track.sample(100., 0.).0 + Vec3::new(600., 30., 0.);
                commands
                    .entity(entity)
                    .insert((RigidBody::Fixed, ColliderDisabled));
            }
            if frame == fall_start && !perf {
                let station = bridge::GAP_STATIONS[0];
                let lane = bridge::HALF_DECK - 0.8;
                let (p, q) = bridge::deck_sample(&track, station, lane);
                *t = Transform::from_translation(p + Vec3::Y * 0.5).with_rotation(q);
                let v = -(q * Vec3::X) * 12.;
                r.finished = None;
                r.route = 0;
                r.progress = station;
                r.lane = lane;
                r.speed = 0.;
                r.crashed = 0.;
                r.health = 100.;
                r.motion_velocity = v;
                if let Some(velocity) = velocity.as_deref_mut() {
                    *velocity = Velocity::linear(v);
                }
                if let Some(b) = bike.as_deref_mut() {
                    b.position = t.translation;
                    b.velocity = v;
                    b.impact_velocity = v;
                    b.grounded = [false; 2];
                    b.was_riding = true;
                    b.raw_steer = 0.;
                    b.intent = default();
                }
                // Apply riding-kick lateral state without forcing a crash.
                // Assisted detects missing support; Dynamic falls under gravity.
                r.side_velocity = 12.;
                *recoveries.get_mut(entity).unwrap() = default();
                commands
                    .entity(entity)
                    .remove::<ColliderDisabled>()
                    .insert((
                        if assisted {
                            RigidBody::KinematicPositionBased
                        } else {
                            RigidBody::Dynamic
                        },
                        Visibility::Visible,
                    ));
                state.fall_deck = p.y;
                state.lowest = p.y;
                state.lowest_chassis = p.y;
            }
            if frame >= fall_start && !perf {
                let recovery = recoveries.get(entity).unwrap();
                state.observed_physical_fall |= recovery.stage
                    == crate::recovery::RecoveryStage::Falling
                    && recovery.parts.len() >= 11
                    && recovery.wreck.is_some();
                // Readback stores the physical chassis pose before presentation
                // interpolation. A mounted Dynamic fall need not enter recovery
                // until contact, so measure it independently of ragdoll bodies.
                if !assisted && recovery.stage == crate::recovery::RecoveryStage::Riding {
                    let b = bike
                        .as_deref()
                        .expect("Dynamic survey requires physical chassis");
                    state.lowest_chassis = state.lowest_chassis.min(b.position.y);
                    state.lowest = state.lowest.min(b.position.y);
                    state.maximum_downward_speed = state.maximum_downward_speed.max(-b.velocity.y);
                }
                if follow.is_none_or(|p: Vec3| t.translation.y < p.y) {
                    follow = Some(t.translation);
                }
            }
        }
        if let Some(body) = owned
            && Some(body.owner) == state.player
            && frame >= fall_start
            && !perf
        {
            state.lowest = state.lowest.min(t.translation.y);
            if let Some(v) = velocity.as_deref() {
                state.maximum_downward_speed = state.maximum_downward_speed.max(-v.linvel.y);
            }
            if follow.is_none_or(|p| t.translation.y < p.y) {
                follow = Some(t.translation);
            }
        }
        if let Some(h) = heavy {
            match h.kind {
                Kind::Semi => {
                    semi_count += 1;
                    if semi.is_none() {
                        semi = Some(*t);
                    }
                }
                Kind::Tractor => {
                    tractor_count += 1;
                    if tractor.is_none() {
                        tractor = Some(*t);
                    }
                }
            }
        }
        if field.is_some() {
            if farm.is_none() {
                farm = Some(*t);
            }
            if delivery {
                farms.push(*t);
            }
        }
    }
    let semi = semi.expect("moving semitrailer population");
    let tractor = tractor.expect("moving agricultural traffic");
    if let Some(p) = state.semi_start {
        state.semi_moved = state.semi_moved.max(p.distance(semi.translation));
    } else {
        state.semi_start = Some(semi.translation);
    }
    if let Some(p) = state.tractor_start {
        state.tractor_moved = state.tractor_moved.max(p.distance(tractor.translation));
    } else {
        state.tractor_start = Some(tractor.translation);
    }
    farms.sort_by(|a, b| a.translation.z.total_cmp(&b.translation.z));
    let index = ((frame / period) as usize).min(view_count - 1);
    let camera = match index {
        0 => route_view(&track, 0, 250., false),
        1 => route_view(&track, 0, 650., true),
        2 => route_view(&track, 0, 1300., false),
        3 => {
            let f = farm.expect("hill farm foundations");
            let p = f.translation + Vec3::Y * f.scale.y * 0.5;
            Transform::from_translation(p + f.rotation * Vec3::new(-48., 24., -52.))
                .looking_at(p + Vec3::Y * 4., Vec3::Y)
        }
        4 => {
            let (p, q) = track.sample_route(0, 3500., 0.);
            Transform::from_translation(p - q * Vec3::Z * 35. + Vec3::Y * 24.)
                .looking_at(p + q * Vec3::Z * 45., Vec3::Y)
        }
        5 => {
            let (p, q) = bridge::deck_sample(&track, bridge::CENTER, 0.);
            Transform::from_translation(p + q * Vec3::new(100., 62., -100.))
                .looking_at(p - Vec3::Y * 12., Vec3::Y)
        }
        6 => route_view(&track, 0, bridge::GAP_STATIONS[0], false),
        7 => route_view(&track, 0, 6200., false),
        8 => route_view(&track, 1, 6200., false),
        9 => route_view(&track, 0, 7800., false),
        10 => {
            Transform::from_translation(semi.translation + semi.rotation * Vec3::new(12., 6., 12.))
                .looking_at(
                    semi.translation - semi.rotation * Vec3::Z * 5. + Vec3::Y * 1.8,
                    Vec3::Y,
                )
        }
        11 => Transform::from_translation(
            tractor.translation + tractor.rotation * Vec3::new(9., 4., 9.),
        )
        .looking_at(tractor.translation + Vec3::Y * 1.2, Vec3::Y),
        12 => route_view(&track, 0, 10500., false),
        13 => route_view(&track, 0, 6800., true),
        14 => {
            let f = farm.expect("hill farm foundations");
            let p = f.translation + Vec3::Y * f.scale.y * 0.5;
            Transform::from_translation(p + f.rotation * Vec3::new(-26., 10., -30.))
                .looking_at(p + Vec3::Y * 3., Vec3::Y)
        }
        15..=20 => {
            let f = farms[index - 15];
            let p = f.translation + Vec3::Y * f.scale.y * 0.5;
            Transform::from_translation(p + f.rotation * Vec3::new(-95., 65., -130.))
                .looking_at(p + f.rotation * Vec3::new(0., 0., 20.), Vec3::Y)
        }
        21 => {
            let p = track.sample(650., 0.).0 + Vec3::Y * 12.;
            let sun = Vec3::from_array(lighting.as_ref().expect("Hills lighting").sun_direction)
                .normalize();
            Transform::from_translation(p).looking_at(p + sun * 1000., Vec3::Y)
        }
        22 => {
            let (p, q) = bridge::deck_sample(&track, bridge::CENTER, 0.);
            let floor = p.with_y(bridge::canyon_ground_height(&track, p.x, p.z) + 2.);
            let mut eye = p + q * Vec3::new(100., -35., -110.);
            eye.y = eye
                .y
                .max(bridge::canyon_ground_height(&track, eye.x, eye.z) + 25.);
            Transform::from_translation(eye).looking_at(floor, Vec3::Y)
        }
        _ => {
            let (p, q) = track.sample(600., 0.);
            Transform::from_translation(p + q * Vec3::new(40., 50., -90.))
                .looking_at(p + q * Vec3::Z * 180., Vec3::Y)
        }
    };
    let camera = if frame >= fall_start && !perf {
        let p =
            follow.unwrap_or_else(|| bridge::deck_sample(&track, bridge::GAP_STATIONS[0], 10.).0);
        Transform::from_translation(p + Vec3::new(14., 8., -18.)).looking_at(p, Vec3::Y)
    } else {
        camera
    };
    for (_, mut t, _, _, _, camera_tag, _, _, _) in &mut objects {
        if camera_tag.is_some() {
            *t = camera;
        }
    }
    let mut capture = None;
    if !perf && frame < view_count as u32 * period && frame % period == period - 10 {
        capture = Some(if index < NAMES.len() {
            NAMES[index]
        } else {
            DELIVERY_VIEWS[index - NAMES.len()]
        });
    }
    if !perf && frame == fall_start + 60 {
        capture = Some("actual-bike-falling");
    }
    if !perf && frame == fall_start + 260 {
        capture = Some("canyon-landing");
    }
    if let Some(name) = capture {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/revision-{name}.png")))
            .observe(captured);
    }
    let expected = if perf { 0 } else { view_count + 2 };
    let finish = fall_start + if perf { 20 } else { 380 };
    if !perf
        && frame > finish
        && frame < fall_start + 1200
        && recoveries.get(state.player.unwrap()).unwrap().stage
            != crate::recovery::RecoveryStage::Riding
    {
        return;
    }
    if frame > finish && state.captures == expected {
        commands.insert_resource(crate::diagnostics::FixtureComplete);
        let _verification_span =
            crate::diagnostics::loading::Span::new("hills_survey_verification");
        assert_eq!(semi_count, crate::heavy_traffic::SEMIS);
        assert_eq!(tractor_count, crate::heavy_traffic::TRACTORS);
        assert!(state.semi_moved > 20. && state.tractor_moved > 10.);
        assert!(city.buildings > 100 && census.farms >= 6 && census.trees > 800);
        if !perf {
            if assisted {
                assert!(
                    state.observed_physical_fall,
                    "Assisted open bridge edge must initiate production physical recovery"
                );
            } else {
                assert!(
                    state.maximum_downward_speed > 5.,
                    "Dynamic bridge exit must descend under physical gravity: {} m/s",
                    state.maximum_downward_speed
                );
            }
            assert!(
                state.fall_deck - state.lowest > 40.,
                "actual player did not fall into canyon: deck={} lowest={}",
                state.fall_deck,
                state.lowest
            );
        }
        let ctx = physics.single().unwrap();
        let is_ground = |e| ground.contains(e);
        let mut error = 0f32;
        let mut probes = 0;
        for route in 0..2 {
            for station in (0..=12500).step_by(25) {
                for lane in [-4., 0., 4.] {
                    let p = track.sample_route(route, station as f32, lane).0;
                    let (_, toi) = ctx
                        .cast_ray(
                            p + Vec3::Y * 3.,
                            Vec3::NEG_Y,
                            6.,
                            true,
                            QueryFilter::default().predicate(&is_ground),
                        )
                        .unwrap_or_else(|| panic!("missing road at{route}/{station}/{lane}"));
                    error = error.max((3. - toi).abs());
                    probes += 1;
                }
            }
        }
        assert!(error < 0.05, "revised road support error {error}");
        let recovery = recoveries.get(state.player.unwrap()).unwrap();
        if !perf {
            assert_eq!(recovery.stage, crate::recovery::RecoveryStage::Riding);
        }
        let report = serde_json::json!({"solar":solar.as_deref(),"shadows":&*shadows,"river":river.as_deref(),"city":&*city,"clouds":clouds.as_deref(),"bridge":&*bridge_census,"town_detail":&*town,"census":&*census,"static_collision":&*solids,
            "semis":semi_count,"tractors":tractor_count,"semi_displacement_m":state.semi_moved,
            "tractor_displacement_m":state.tractor_moved,"observed_physical_fall":state.observed_physical_fall,"fall_initiation":"lateral exit without forced crash",
            "vehicle_adapter":format!("{:?}", *adapter),
            "chassis_drop_m":if perf || assisted{0.}else{state.fall_deck-state.lowest_chassis},
            "maximum_downward_speed_mps":state.maximum_downward_speed,
            "player_canyon_drop_m":if perf{0.}else{state.fall_deck-state.lowest},
            "recovery_stage":format!("{:?}", recovery.stage),"exceptional_rescues":recovery.rescues,
            "road_probes":probes,"maximum_support_error_m":error,"captures":state.captures,"performance_mode":perf,
            "metrics":metrics.evidence(&clock),"limits":"Scripted scene/fall evidence; not a full human race or GPU benchmark."});
        std::fs::write(
            "screenshots/hills-revision.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        exit.write(AppExit::Success);
    }
    assert!(frame < finish + 600, "revision captures timed out");
}
