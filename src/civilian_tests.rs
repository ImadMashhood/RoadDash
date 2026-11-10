use super::*;

#[test]
fn civilian_passes_slow_vehicle_in_both_course_directions_with_ground_support() {
    for hills in [false, true] {
        for direction in [-1., 1.] {
            for (heavy_speed, rear) in [(6., 3.), (12., 13.4)] {
                let mut app = tests::simulation();
                app.insert_resource(if hills {
                    Track::hills().unwrap()
                } else {
                    Track::metropolis().unwrap()
                });
                let riders: Vec<_> = app
                    .world_mut()
                    .query_filtered::<Entity, With<Rider>>()
                    .iter(app.world())
                    .collect();
                for e in riders {
                    app.world_mut().despawn(e);
                }
                let station = if hills { 5900. } else { 1400. };
                let ids: Vec<_> = [
                    (station, 18., 1.95),
                    (station + direction * 30., heavy_speed, rear),
                ]
                .into_iter()
                .map(|(progress, speed, rear)| {
                    let track = app.world().resource::<Track>();
                    let ratio = 3.1 * direction / track.half_width(progress);
                    let (home, _, _) = civilian_lanes(track, 0, progress, direction, 0, ratio);
                    let (p, q) = track.street_sample(0, progress, home);
                    app.world_mut()
                        .spawn((
                            Traffic {
                                route: 0,
                                progress,
                                lane: home,
                                speed: speed * direction,
                            },
                            TrafficYield {
                                cruise: Some(speed * direction),
                                lane_ratio: Some(ratio),
                                start_progress: Some(progress),
                                ..default()
                            },
                            Transform::from_translation(p).with_rotation(q),
                            RigidBody::KinematicPositionBased,
                            Collider::compound(vec![(
                                Vec3::new(0., 0.95, (1.95 - rear) * 0.5),
                                Quat::IDENTITY,
                                Collider::cuboid(0.95, 0.9, (1.95 + rear) * 0.5),
                            )]),
                        ))
                        .id()
                })
                .collect();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                let mut passed = false;
                let mut max_blend = 0_f32;
                for _ in 0..3000 {
                    app.update();
                    let car = app.world().get::<Traffic>(ids[0]).unwrap();
                    let leader = app.world().get::<Traffic>(ids[1]).unwrap();
                    let memory = &app.world().get::<TrafficYield>(ids[0]).unwrap().passing;
                    max_blend = max_blend.max(memory.blend);
                    passed |= (car.progress - leader.progress) * direction > 15.;
                    let track = app.world().resource::<Track>();
                    let ratio = app
                        .world()
                        .get::<TrafficYield>(ids[0])
                        .unwrap()
                        .lane_ratio
                        .unwrap();
                    let (home, opposite, _) =
                        civilian_lanes(track, 0, car.progress, direction, 0, ratio);
                    let expected = track
                        .street_sample(0, car.progress, home + (opposite - home) * memory.blend)
                        .0;
                    let actual = app.world().get::<Transform>(ids[0]).unwrap().translation;
                    assert!(
                        actual.distance(expected) < 0.02,
                        "traffic lost authored support"
                    );
                    let (_, q) = track.sample_route(0, car.progress, 0.);
                    let separation =
                        app.world().get::<Transform>(ids[1]).unwrap().translation - actual;
                    assert!(
                        separation.dot(q * Vec3::X).abs() > 1.9
                            || (leader.progress - car.progress).abs() > rear + 1.95,
                        "traffic overlap hills={hills} direction={direction}"
                    );
                }
                assert!(
                    passed && max_blend > 0.95,
                    "never overtook hills={hills} direction={direction} heavy={heavy_speed} blend={max_blend}"
                );
                assert!(app.world().get::<Collider>(ids[0]).is_some());
                app.world_mut().resource_mut::<Race>().elapsed = 0.;
                app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
                app.update();
                assert_eq!(
                    app.world()
                        .get::<TrafficYield>(ids[0])
                        .unwrap()
                        .passing
                        .phase,
                    rr_core::civilian::Phase::Follow
                );
            }
        }
    }
}

#[test]
fn upright_oncoming_biker_does_not_veto_civilian_pass_or_remove_collision() {
    let run = |with_biker| {
        let mut app = tests::simulation();
        app.insert_resource(Track::metropolis().unwrap());
        let biker = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        if with_biker {
            let track = app.world().resource::<Track>();
            let (lane, _, _) = civilian_lanes(track, 0, 1600., -1., 0, 0.);
            let (p, q) = track.street_sample(0, 1600., lane);
            let mut r = app.world_mut().get_mut::<Rider>(biker).unwrap();
            r.progress = 1600.;
            r.lane = lane;
            r.speed = 18.;
            r.motion_velocity = q * Vec3::NEG_Z * 18.;
            r.finished = Some(0.);
            *app.world_mut().get_mut::<Transform>(biker).unwrap() = Transform::from_translation(p)
                .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI));
        } else {
            app.world_mut().despawn(biker);
        }
        let mut ids = vec![];
        for (progress, speed) in [(1400., 18.), (1430., 6.)] {
            let track = app.world().resource::<Track>();
            let (home, _, _) = civilian_lanes(track, 0, progress, 1., 0, 0.);
            let (p, q) = track.street_sample(0, progress, home);
            ids.push(
                app.world_mut()
                    .spawn((
                        Traffic {
                            route: 0,
                            progress,
                            lane: home,
                            speed,
                        },
                        TrafficYield {
                            cruise: Some(speed),
                            ..default()
                        },
                        Transform::from_translation(p).with_rotation(q),
                        RigidBody::KinematicPositionBased,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    ))
                    .id(),
            );
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..180 {
            app.update();
        }
        assert!(app.world().get::<Collider>(ids[0]).is_some());
        if with_biker {
            assert!(app.world().get::<Collider>(biker).is_some());
        }
        let y = app.world().get::<TrafficYield>(ids[0]).unwrap();
        assert_eq!(y.passing.phase, rr_core::civilian::Phase::Pass);
        (
            y.passing.blend,
            app.world().get::<Traffic>(ids[0]).unwrap().progress,
        )
    };
    assert_eq!(run(false), run(true));
}

#[test]
fn civilian_refuses_static_departure_obstruction_then_resumes_after_clearance() {
    let mut app = tests::simulation();
    app.insert_resource(Track::metropolis().unwrap());
    let riders: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .iter(app.world())
        .collect();
    for e in riders {
        app.world_mut().despawn(e);
    }
    let mut ids = vec![];
    for (progress, speed) in [(1400., 18.), (1430., 6.)] {
        let track = app.world().resource::<Track>();
        let (home, _, _) = civilian_lanes(track, 0, progress, 1., 0, 0.);
        let (p, q) = track.street_sample(0, progress, home);
        ids.push(
            app.world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress,
                        lane: home,
                        speed,
                    },
                    TrafficYield {
                        cruise: Some(speed),
                        ..default()
                    },
                    Transform::from_translation(p).with_rotation(q),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(0.95, 0.9, 1.95),
                ))
                .id(),
        );
    }
    let track = app.world().resource::<Track>();
    let (_, opposite, _) = civilian_lanes(track, 0, 1450., 1., 0, 0.);
    let (p, q) = track.street_sample(0, 1450., opposite);
    let obstruction = app
        .world_mut()
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(1.2, 2., 80.),
            Transform::from_translation(p + Vec3::Y).with_rotation(q),
        ))
        .id();
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    for _ in 0..300 {
        app.update();
        assert_eq!(
            app.world()
                .get::<TrafficYield>(ids[0])
                .unwrap()
                .passing
                .blend,
            0.
        );
    }
    app.world_mut().despawn(obstruction);
    let mut started = false;
    for _ in 0..240 {
        app.update();
        started |= app
            .world()
            .get::<TrafficYield>(ids[0])
            .unwrap()
            .passing
            .blend
            > 0.5;
    }
    assert!(started, "cleared static obstruction must release the pass");
}

#[test]
fn asymmetric_metropolis_pavement_is_centered_on_sampler_not_lane_divider() {
    let track = Track::metropolis().unwrap();
    let sections = &track.metropolis.as_ref().unwrap().street_life.lane_sections;
    let station = 4000.;
    let (home, opposite, divider) = civilian_lanes(&track, 0, station, 1., 1, 0.);
    let width = rr_core::streets::markings(sections, 0, station).2;
    assert!(divider.abs() > 1.);
    assert!(home.abs() + 0.95 < width && opposite.abs() + 0.95 < width);
    // Subtracting the asymmetric divider from a physical pavement bound would
    // falsely reject the perfectly valid outer outbound lane.
    assert!((home - divider).abs() + 0.95 > width);
    let (center, q) = track.street_sample(0, station, 0.);
    let (point, _) = track.street_sample(0, station, home);
    assert!(((point - center).dot(q * Vec3::NEG_X) - home).abs() < 0.02);
}

#[test]
fn civilian_lane_transition_stays_on_hills_bridge_deck() {
    let track = Track::hills().unwrap();
    let bridge = crate::world_gen::hills::bridge::CENTER;
    for direction in [-1., 1.] {
        for step in 0..=60 {
            let station = bridge - 90. + step as f32 * 3.;
            let (home, opposite, _) = civilian_lanes(
                &track,
                0,
                station,
                direction,
                0,
                3.1 * direction / track.half_width(station),
            );
            let blend = step as f32 / 60.;
            let lane = home + (opposite - home) * blend;
            let (actual, _) = track.street_sample(0, station, lane);
            let (deck, _) = crate::world_gen::hills::bridge::deck_sample(&track, station, lane);
            assert!(
                actual.distance(deck) < 0.01,
                "lane transition must retain actual deck elevation"
            );
            assert!(
                lane.abs() + 1.5 < crate::world_gen::hills::bridge::HALF_DECK,
                "full vehicle width leaves bridge support"
            );
        }
    }
}

#[test]
fn civilian_eventually_passes_a_persistently_disabled_same_class_car() {
    let mut app = tests::simulation();
    app.insert_resource(Track::metropolis().unwrap());
    let riders: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .iter(app.world())
        .collect();
    for e in riders {
        app.world_mut().despawn(e);
    }
    let mut ids = vec![];
    for progress in [1400., 1435.] {
        let track = app.world().resource::<Track>();
        let (home, _, _) = civilian_lanes(track, 0, progress, 1., 0, 0.);
        let (p, q) = track.street_sample(0, progress, home);
        ids.push(
            app.world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress,
                        lane: home,
                        speed: 0.,
                    },
                    TrafficYield {
                        cruise: Some(18.),
                        ..default()
                    },
                    Transform::from_translation(p).with_rotation(q),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(0.95, 0.9, 1.95),
                ))
                .id(),
        );
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let mut max_blend = 0_f32;
    for _ in 0..2400 {
        // Model an actual disabled motor without changing its authored speed
        // class or removing its collider. This fixture alone holds the leader.
        let mut leader = app.world_mut().get_mut::<Traffic>(ids[1]).unwrap();
        leader.speed = 0.;
        leader.progress = 1435.;
        app.update();
        let follower = app.world().get::<Traffic>(ids[0]).unwrap();
        let leader = app.world().get::<Traffic>(ids[1]).unwrap();
        let y = app.world().get::<TrafficYield>(ids[0]).unwrap();
        max_blend = max_blend.max(y.passing.blend);
        let track = app.world().resource::<Track>();
        let lateral = (follower.lane - track.street_offset(0, follower.progress))
            - (leader.lane - track.street_offset(0, leader.progress));
        assert!(
            lateral.abs() >= 1.9 || (follower.progress - leader.progress).abs() > 3.9,
            "disabled car was overlapped"
        );
    }
    assert!(max_blend > 0.95 && app.world().get::<Traffic>(ids[0]).unwrap().progress > 1455.);
    let y = app.world().get::<TrafficYield>(ids[0]).unwrap();
    assert_eq!(y.passing.phase, rr_core::civilian::Phase::Follow);
    assert_eq!(y.passing.blend, 0.);
    assert!(app.world().get::<Collider>(ids[1]).is_some());
}
