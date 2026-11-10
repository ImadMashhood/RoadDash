use super::*;

#[test]
fn compressed_city_car_queue_drains_in_both_directions() {
    for direction in [-1., 1.] {
        let mut app = tests::simulation();
        app.insert_resource(Track::metropolis().unwrap());
        let player = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        // Keep the observer outside the lane; a legitimate downed-person stop
        // has its own retained regression and must not be timed out or ignored.
        app.world_mut().get_mut::<Rider>(player).unwrap().finished = Some(0.);
        let mut cars: Vec<_> = (0..3)
            .map(|_| {
                app.world_mut()
                    .spawn((
                        Traffic {
                            route: 0,
                            progress: 2900.,
                            lane: 0.,
                            speed: 0.,
                        },
                        TrafficYield {
                            cruise: Some(direction * 18.),
                            lane_slot: 0,
                            ..default()
                        },
                        Transform::default(),
                        RigidBody::KinematicPositionBased,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    ))
                    .id()
            })
            .collect();
        cars.sort();
        for (i, &entity) in cars.iter().enumerate() {
            app.world_mut().get_mut::<Traffic>(entity).unwrap().progress =
                2900. + direction * i as f32 * 0.04;
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..1200 {
            app.update();
        }
        let results: Vec<_> = cars
            .iter()
            .map(|e| {
                let c = app.world().get::<Traffic>(*e).unwrap();
                (c.progress, c.speed)
            })
            .collect();
        for &(progress, speed) in &results {
            assert!(
                (progress - 2900.) * direction > 20. && speed * direction > 1.,
                "stationary circular queue, direction={direction}: {results:?}"
            );
        }
        for pair in results.windows(2) {
            assert!(
                (pair[0].0 - pair[1].0).abs() >= 7.5,
                "queue spacing: {results:?}"
            );
        }
    }
}

#[test]
fn production_city_fleet_keeps_moving_on_an_unobstructed_course() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = tests::simulation();
    app.insert_resource(Track::metropolis().unwrap());
    app.world_mut().run_system_once(setup_race).unwrap();
    // Use the production fleet and lane/recycle configuration, with no riders
    // or crashed people so a sustained halt cannot be legitimate yielding.
    let riders: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .iter(app.world())
        .collect();
    for e in riders {
        app.world_mut().despawn(e);
    }
    let count = app
        .world_mut()
        .query::<&Traffic>()
        .iter(app.world())
        .count();
    assert_eq!(count, 144);
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let mut stopped = std::collections::BTreeMap::<Entity, u32>::new();
    for _ in 0..7200 {
        app.update();
        let mut query = app.world_mut().query::<(Entity, &Traffic)>();
        for (e, car) in query.iter(app.world()) {
            let streak = stopped.entry(e).or_default();
            *streak = if car.speed.abs() < 0.1 {
                *streak + 1
            } else {
                0
            };
            assert!(
                *streak < 600,
                "unobstructed car stopped for ten seconds: {e:?}, station={}, lane={}",
                car.progress,
                car.lane
            );
        }
    }
}
