use super::*;
use bevy_rapier3d::prelude::*;
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
    .insert_resource(Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap())
    .init_resource::<Assets<Mesh>>()
    .init_resource::<Assets<StandardMaterial>>()
    .add_systems(Startup, crate::world::setup_world);
    for _ in 0..3 {
        app.update();
    }
    app
}
#[test]
fn every_foundation_is_supported_and_roads_junctions_and_chunk_seams_are_solid() {
    let mut app = scene();
    let world = app.world_mut();
    let layout = world.resource::<layout::WorldLayout>().clone();
    assert_eq!(
        world
            .query::<&buildings::GeneratedBuilding>()
            .iter(world)
            .count(),
        247
    );
    assert_eq!(
        world
            .query::<&crate::environment::LegacyBuilding>()
            .iter(world)
            .count(),
        0
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
    let ground = |e| world.get::<crate::world::Ground>(e).is_some();
    let ray = |p: Vec3| {
        context
            .cast_ray(
                p + Vec3::Y * 4.0,
                -Vec3::Y,
                10.0,
                true,
                QueryFilter::default().predicate(&ground),
            )
            .map(|(_, t)| p.y + 4.0 - t)
            .unwrap_or_else(|| panic!("ground hole at {p:?}"))
    };
    let track = world.resource::<Track>();
    for b in &layout.buildings {
        for p in b.footprint.expanded(0.2).corners() {
            let point = Vec3::new(p.x, b.foundation_m, p.y);
            let height = ray(point);
            assert!(
                (height - b.foundation_m).abs() < 0.035,
                "{}: unsupported/floating footing {} vs {}",
                b.id,
                height,
                b.foundation_m
            );
        }
    }
    for j in &layout.settings.junctions {
        let (p, q) = layout::horizontal_frame(track, j.station_m);
        for lane in [-95.0, -60.0, -15.0, -7.0, 0.0, 7.0, 15.0, 60.0, 95.0] {
            let v = p - q * Vec3::X * lane;
            assert!(ray(v).is_finite());
        }
    }
    for i in 0..150 {
        let d = track.length * i as f32 / 150.0;
        for lane in [-5.0, 0.0, 5.0] {
            let p = track.sample(d, lane).0;
            assert!(
                (ray(p) - p.y).abs() < 0.1,
                "road support/terrain intrusion at {d}"
            );
        }
    }
    // Same sampled seam from either tile; physics surface has no gap.
    let (min, max) = track.bounds;
    for x in
        (1..((max.x - min.x + 400.0) / 200.0) as usize).map(|i| min.x - 200.0 + i as f32 * 200.0)
    {
        for z in (1..((max.z - min.z + 400.0) / 200.0) as usize)
            .map(|i| min.z - 200.0 + i as f32 * 200.0)
        {
            let p = Vec3::new(x, track.terrain_height(x, z), z);
            let left = ray(p + Vec3::X * 0.001);
            let right = ray(p - Vec3::X * 0.001);
            assert!(
                (left - right).abs() < 0.05,
                "terrain seam at {p:?}: {left} vs {right}"
            );
        }
    }
    // Fade pairs must use the same local origin. World-space mesh vertices with
    // an identity transform caused the ground to disappear on most sections.
    let mut terrain_query = world.query::<(
        &Transform,
        &Mesh3d,
        &crate::environment::TerrainLod,
        &bevy::camera::visibility::VisibilityRange,
    )>();
    let assets = world.resource::<Assets<Mesh>>();
    let mut pairs = std::collections::HashMap::<(u32, u32), [bool; 2]>::new();
    for (t, m, lod, range) in terrain_query.iter(world) {
        assert!(!range.use_aabb);
        if let Some(bevy::mesh::VertexAttributeValues::Float32x3(positions)) = assets
            .get(&m.0)
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
        {
            assert!(
                positions
                    .iter()
                    .all(|p| p[0].abs() < 100.01 && p[2].abs() < 100.01),
                "terrain mesh must be local to its tile"
            );
        }
        pairs
            .entry((t.translation.x.to_bits(), t.translation.z.to_bits()))
            .or_default()[usize::from(!lod.0)] = true;
    }
    assert_eq!(pairs.len(), 160);
    assert!(pairs.values().all(|p| *p == [true, true]));
    // Both LODs stay under one root; physics remains at every distance.
    for (building, children, collider) in world
        .query::<(&buildings::GeneratedBuilding, &Children, &Collider)>()
        .iter(world)
    {
        let _ = (building, collider);
        assert!(
            children
                .iter()
                .all(|e| world.get::<Transform>(e).unwrap().translation == Vec3::ZERO),
            "building fade pair origins differ"
        );
        assert!(children.iter().any(|c| {
            world
                .get::<buildings::BuildingLod>(c)
                .is_some_and(|l| l.near)
        }));
        assert!(children.iter().any(|c| {
            world
                .get::<buildings::BuildingLod>(c)
                .is_some_and(|l| !l.near)
        }));
    }
}
#[derive(Component)]
struct Probe {
    path: Vec<Vec3>,
    index: usize,
    goal: Vec3,
    done: bool,
}
fn walk(mut probes: Query<(&Transform, &mut Probe, &mut KinematicCharacterController)>) {
    for (t, mut p, mut c) in &mut probes {
        if p.done {
            c.translation = Some(-Vec3::Y * 0.05);
            continue;
        }
        if p.index < p.path.len() && p.path[p.index].xz().distance(t.translation.xz()) < 0.8 {
            p.index += 1;
        }
        if p.index == p.path.len() {
            p.done = t.translation.xz().distance(p.goal.xz()) < 1.2;
            c.translation = Some(-Vec3::Y * 0.05);
            continue;
        }
        c.translation = Some(
            ((p.path[p.index] - t.translation)
                .with_y(0.0)
                .normalize_or_zero()
                * 6.0
                - Vec3::Y * 5.0)
                / 60.0,
        );
    }
}
#[test]
fn physical_capsules_walk_farm_city_alley_and_hill_routes() {
    let mut app = scene();
    let layout = app.world().resource::<layout::WorldLayout>().clone();
    let cases = [
        (
            "farm",
            layout
                .buildings
                .iter()
                .find(|b| b.section_index == 3)
                .unwrap(),
        ),
        (
            "city_alley",
            layout
                .buildings
                .iter()
                .find(|b| b.section_index == 9 && b.row == rr_content::world::Row::Secondary)
                .unwrap(),
        ),
        (
            "hill",
            layout
                .buildings
                .iter()
                .find(|b| b.section_index == 12)
                .unwrap(),
        ),
    ];
    let mut roots = Vec::new();
    for (name, b) in cases {
        let track = app.world().resource::<Track>();
        let start = Vec3::from_array(b.access) + Vec3::Y * 1.0;
        let goal = track.sample(b.station_m, b.lane_m.signum() * 5.0).0 + Vec3::Y * 0.95;
        let path = navigation::route(&layout, track, start, goal).expect("foot route");
        let root = app
            .world_mut()
            .spawn((
                Name::new(name),
                Transform::from_translation(start),
                RigidBody::KinematicPositionBased,
                Collider::capsule_y(0.65, 0.28),
                KinematicCharacterController {
                    translation: Some(Vec3::ZERO),
                    offset: CharacterLength::Absolute(0.02),
                    snap_to_ground: Some(CharacterLength::Absolute(0.3)),
                    autostep: Some(CharacterAutostep {
                        max_height: CharacterLength::Absolute(0.3),
                        min_width: CharacterLength::Absolute(0.15),
                        include_dynamic_bodies: false,
                    }),
                    ..default()
                },
                Probe {
                    path,
                    index: 0,
                    goal,
                    done: false,
                },
            ))
            .id();
        roots.push(root);
    }
    app.add_systems(FixedUpdate, walk.before(PhysicsSet::SyncBackend));
    for _ in 0..2400 {
        app.update();
        if roots
            .iter()
            .all(|e| app.world().get::<Probe>(*e).unwrap().done)
        {
            break;
        }
    }
    for root in roots {
        let p = app.world().get::<Probe>(root).unwrap();
        let t = app.world().get::<Transform>(root).unwrap();
        assert!(
            p.done,
            "{} stalled at {:?}, goal {:?}, waypoint {}/{}",
            app.world().get::<Name>(root).unwrap(),
            t.translation,
            p.goal,
            p.index,
            p.path.len()
        );
    }
}
#[test]
fn construction_rejects_bad_junctions_and_generated_layout_rejects_corruption() {
    let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
    let plan = WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &track).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(layout::CONSTRUCTION_JSON).unwrap();
    json["junctions"][1]["station_m"] = json["junctions"][0]["station_m"].clone();
    assert!(layout::Construction::from_json(&json.to_string(), &track).is_err());
    let mut layout = layout::WorldLayout::generate(
        &plan,
        &track,
        layout::Construction::from_json(layout::CONSTRUCTION_JSON, &track).unwrap(),
    )
    .unwrap();
    layout.buildings[1].lot = layout.buildings[0].lot.clone();
    assert!(layout.validate(&plan, &track).is_err());
}
#[test]
fn exceptional_recovery_rescue_clears_motion_and_applies_one_visible_penalty() {
    let mut app = crate::game::tests::simulation();
    let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
    let plan = WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &track).unwrap();
    let layout = layout::WorldLayout::generate(
        &plan,
        &track,
        layout::Construction::from_json(layout::CONSTRUCTION_JSON, &track).unwrap(),
    )
    .unwrap();
    let (p, q) = track.sample(500.0, 0.0);
    app.insert_resource(track).insert_resource(layout);
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<crate::game::Rider>>()
        .single(app.world())
        .unwrap();
    let wreck = app
        .world_mut()
        .spawn((
            Transform::from_translation(p + Vec3::Y * 12.0),
            RigidBody::KinematicPositionBased,
            Velocity::linear(Vec3::X * 8.0),
            Collider::cuboid(0.38, 0.8, 1.12),
        ))
        .id();
    let runner = app
        .world_mut()
        .spawn((
            Transform::from_translation(p + Vec3::Y),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.65, 0.28),
            KinematicCharacterController {
                translation: Some(Vec3::X),
                ..default()
            },
        ))
        .id();
    {
        let mut r = app
            .world_mut()
            .get_mut::<crate::recovery::Recovery>(owner)
            .unwrap();
        r.stage = crate::recovery::RecoveryStage::Running;
        r.elapsed = 121.0;
        r.wreck = Some(wreck);
        r.runner = Some(runner);
    }
    app.world_mut().resource_mut::<crate::game::Race>().phase = crate::game::Phase::Racing;
    app.update();
    let race = app.world().resource::<crate::game::Race>();
    assert!((5.0..5.1).contains(&race.elapsed));
    assert!(race.message.contains("RESCUE"));
    let t = app.world().get::<Transform>(runner).unwrap();
    assert!(
        t.translation
            .distance(p + Vec3::Y * 0.98 - q * Vec3::X * 2.0)
            < 0.1,
        "stale walking motion survived rescue"
    );
    assert!(app.world().get::<Velocity>(wreck).unwrap().linvel.length() < 0.1);
    for _ in 0..3 {
        app.update();
    }
    assert!(
        app.world().resource::<crate::game::Race>().elapsed < 5.2,
        "rescue penalty repeated"
    );
}
