use super::*;
use std::time::Duration;
fn scene() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
    ))
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(TimestepMode::Fixed {
        dt: 1.0 / 60.0,
        substeps: 1,
    })
    .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_secs_f64(1.0 / 60.0),
    ))
    .insert_resource(
        Track::from_json(include_str!("../../../content/tracks/city5300.json")).unwrap(),
    )
    .init_resource::<Assets<Mesh>>()
    .init_resource::<Assets<StandardMaterial>>()
    .add_systems(Startup, crate::world::setup_world);
    for _ in 0..3 {
        app.update();
    }
    app
}
#[test]
fn every_building_and_start_grid_has_real_collision_and_recovery_clearance() {
    let mut app = scene();
    let world = app.world_mut();
    let city = world.resource::<GeneratedCity>();
    let sites = city.plan.lots.clone();
    let blockers = city.blockers.clone();
    assert_eq!(world.query::<&CityBuilding>().iter(world).count(), 977);
    assert_eq!(
        world
            .query::<&crate::environment::CrossTraffic>()
            .iter(world)
            .count(),
        20
    );
    let mut query = world.query::<(
        &RapierContextSimulation,
        &RapierContextColliders,
        &RapierContextJoints,
        &RapierRigidBodySet,
    )>();
    let (sim, colliders, joints, bodies) = query.single(world).unwrap();
    let context = RapierContext {
        simulation: sim,
        colliders,
        rigidbody_set: bodies,
        joints,
    };
    for site in &sites {
        let hit = context
            .cast_ray(
                Vec3::new(
                    site.center_x,
                    site.height_m + 3.0,
                    (site.z_start + site.z_end) / 2.0,
                ),
                Vec3::NEG_Y,
                4.0,
                true,
                QueryFilter::default(),
            )
            .expect("solid building roof");
        assert!(
            world.get::<CityBuilding>(hit.0).is_some(),
            "{} roof is not physical",
            site.id
        );
    }
    let floor = |e| world.get::<Ground>(e).is_some();
    for station in [
        -240.0, -164.0, -20.0, 0.0, 50.0, 100.0, 1200.0, 3500.0, 5290.0, 5300.0, 5350.0,
    ] {
        for x in [-2.0, 2.0, 8.8, 68.0, 200.0] {
            let hit = context
                .cast_ray(
                    Vec3::new(x, 3.0, station),
                    Vec3::NEG_Y,
                    4.0,
                    true,
                    QueryFilter::default().predicate(&floor),
                )
                .expect("supported road/grid");
            let y = 3.0 - hit.1;
            assert!((-0.04..=0.17).contains(&y));
        }
    }
    let track = world.resource::<Track>();
    assert!((track.length - 5300.0).abs() < 0.02);
    assert!(!track.definition.closed);
    for station in [50.0, 1050.0, 2250.0, 3550.0, 5250.0] {
        let from = Vec3::new(29.4, 1.0, station);
        let to = Vec3::new(8.8, 1.0, station);
        let path = super::super::navigation::route_rects(&blockers, track, from, to)
            .expect("back-to-street foot route");
        let mut before = from;
        for p in path {
            assert!(
                !blockers
                    .iter()
                    .any(|b| b.blocks_segment(before.xz(), p.xz(), 0.4))
            );
            before = p;
        }
    }
}
#[test]
fn city_gates_cover_the_whole_open_course_and_preserve_last_place_grid() {
    let track = Track::from_json(include_str!("../../../content/tracks/city5300.json")).unwrap();
    let mut gates = crate::track::GateProgress::default();
    for gate in track.definition.checkpoints.iter().skip(1) {
        let d = gate.fraction * track.length;
        gates.observe(
            &track,
            track.sample(d - 1.0, 0.0).0,
            track.sample(d + 1.0, 0.0).0,
            d - 1.0,
            d + 1.0,
        );
    }
    assert_eq!(gates.completed_laps, 1);
    assert_eq!(track.definition.spawns.len(), 15);
    assert_eq!(track.definition.spawns[0].distance, -176.0);
    assert!(
        track
            .definition
            .spawns
            .iter()
            .skip(1)
            .all(|s| s.distance > track.definition.spawns[0].distance)
    );
}
