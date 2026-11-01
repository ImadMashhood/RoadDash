use super::*;
#[test]
fn bundle_has_shared_standing_seated_and_bike_geometry_and_rejects_bad_data() {
    let records = rr_content::riding_model::decode(BUNDLE).unwrap();
    for group in [
        "standing",
        "seated",
        "motorcycle",
        "front-wheel",
        "rear-wheel",
    ] {
        assert!(records.iter().any(|r| r.group == group));
    }
    let count = |group: &str| {
        records
            .iter()
            .filter(|r| r.group == group)
            .map(|r| r.indices.len() / 3)
            .sum::<usize>()
    };
    assert_eq!(count("standing"), 13_896);
    assert_eq!(count("standing"), count("seated"));
    assert_eq!(
        count("motorcycle") + count("front-wheel") + count("rear-wheel"),
        44_871
    );
    assert!(rr_content::riding_model::decode(&BUNDLE[..BUNDLE.len() - 1]).is_err());
    let mut corrupt = BUNDLE.to_vec();
    corrupt[0] = 0;
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
}

fn authored(app: &mut App) {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let mut binds = Assets::<SkinnedMeshInverseBindposes>::default();
    let models = AuthoredModels::load(&mut meshes, &mut materials, &mut binds, None);
    app.world_mut().resource_mut::<world::Art>().models = Some(models);
    app.insert_resource(meshes)
        .insert_resource(materials)
        .insert_resource(binds);
}
#[test]
fn all_four_human_states_use_the_same_skin_and_render_entities_have_no_physics() {
    use bevy::ecs::system::RunSystemOnce;
    use bevy_rapier3d::prelude::*;
    let mut app = crate::game::tests::simulation();
    authored(&mut app);
    app.world_mut()
        .run_system_once(|mut commands: Commands, art: Res<world::Art>| {
            world::bike(&mut commands, &art, 4, true);
            world::runner(&mut commands, &art, 4);
            let owner = commands.spawn_empty().id();
            crate::recovery::humanoid(
                &mut commands,
                &art,
                owner,
                4,
                Vec3::new(5.0, 3.0, 0.0),
                Quat::IDENTITY,
                Vec3::ZERO,
            );
            crate::environment::spawn_pedestrian(
                &mut commands,
                &art,
                Vec3::new(-5.0, 0.0, 0.0),
                Quat::IDENTITY,
                1.0,
            );
        })
        .unwrap();
    app.update();
    let w = app.world_mut();
    let skins:Vec<_>=w.query_filtered::<(Entity,&SkinnedMesh,&MeshMaterial3d<StandardMaterial>),With<AuthoredHuman>>().iter(w).map(|(e,s,m)|(e,s.joints.clone(),m.0.clone())).collect();
    assert_eq!(
        skins.len(),
        16,
        "four material regions in each of four human states"
    );
    assert_eq!(w.query::<&human::HumanSegment>().iter(w).count(), 44);
    for (e, joints, _) in &skins {
        assert_eq!(joints.len(), 11);
        for joint in joints {
            assert!(w.get::<human::HumanSegment>(*joint).is_some());
            assert_eq!(w.get::<Transform>(*joint).unwrap().scale, Vec3::ONE);
        }
        assert!(w.get::<RigidBody>(*e).is_none());
        assert!(w.get::<Collider>(*e).is_none());
        assert!(w.get::<ImpulseJoint>(*e).is_none());
    }
    for (e, _) in w
        .query_filtered::<(Entity, &Mesh3d), With<AuthoredMotorcycle>>()
        .iter(w)
    {
        assert!(w.get::<Collider>(e).is_none());
        assert!(w.get::<RigidBody>(e).is_none());
    }
    // Three standing states share mesh/material handles; seated uses the same atlas materials.
    let mut skin_material_counts = std::collections::HashMap::new();
    for (_, _, m) in &skins {
        *skin_material_counts.entry(m.id()).or_insert(0) += 1;
    }
    // Region handles may differ per bind mesh, but standing appearances must be shared.
    assert!(skin_material_counts.values().filter(|c| **c == 3).count() == 4);
}
#[test]
fn authored_skin_does_not_change_any_ragdoll_body_joint_or_180_tick_trajectory() {
    use bevy::ecs::system::RunSystemOnce;
    use bevy_rapier3d::prelude::*;
    let mut old = crate::game::tests::simulation();
    let mut new = crate::game::tests::simulation();
    authored(&mut new);
    fn spawn(app: &mut App) -> Vec<Entity> {
        app.world_mut()
            .run_system_once(|mut commands: Commands, art: Res<world::Art>| {
                let owner = commands.spawn_empty().id();
                crate::recovery::humanoid(
                    &mut commands,
                    &art,
                    owner,
                    4,
                    Vec3::new(0.0, 3.0, 8.0),
                    Quat::from_rotation_y(0.3),
                    Vec3::new(3.0, -2.0, 18.0),
                )
            })
            .unwrap()
    }
    let a = spawn(&mut old);
    let b = spawn(&mut new);
    for (a, b) in a.iter().zip(&b) {
        let a = old.world().entity(*a);
        let b = new.world().entity(*b);
        for component in [
            "body",
            "collider",
            "mass",
            "damping",
            "friction",
            "restitution",
            "ccd",
            "joint",
        ] {
            let signature = |e: bevy::ecs::world::EntityRef<'_>| match component {
                "body" => format!("{:?}", e.get::<RigidBody>()),
                "collider" => format!(
                    "{:?}",
                    e.get::<Collider>()
                        .unwrap()
                        .as_cuboid()
                        .unwrap()
                        .half_extents()
                ),
                "mass" => format!("{:?}", e.get::<ColliderMassProperties>()),
                "damping" => format!("{:?}", e.get::<Damping>()),
                "friction" => format!("{:?}", e.get::<Friction>()),
                "restitution" => format!("{:?}", e.get::<Restitution>()),
                "ccd" => format!("{:?}", e.get::<Ccd>()),
                _ => format!("{:?}", e.get::<ImpulseJoint>().map(|j| &j.data)),
            };
            assert_eq!(
                signature(a),
                signature(b),
                "physics component changed: {component}"
            );
        }
    }
    let mut max_position_error = 0.0_f32;
    for tick in 0..180 {
        old.update();
        new.update();
        for (a, b) in a.iter().zip(&b) {
            let ta = old.world().get::<Transform>(*a).unwrap();
            let tb = new.world().get::<Transform>(*b).unwrap();
            let va = old.world().get::<Velocity>(*a).unwrap();
            let vb = new.world().get::<Velocity>(*b).unwrap();
            max_position_error = max_position_error.max(ta.translation.distance(tb.translation));
            assert!(
                ta.translation.distance(tb.translation) < 0.0001,
                "body trajectory changed at tick {tick}: {ta:?} vs {tb:?}"
            );
            assert!(
                ta.rotation.angle_between(tb.rotation) < 0.001,
                "body rotation changed at tick {tick}"
            );
            assert!(
                va.linvel.distance(vb.linvel) < 0.0001 && va.angvel.distance(vb.angvel) < 0.0001,
                "momentum changed at {tick}"
            );
            assert_eq!(ta.scale, tb.scale);
        }
    }
    println!("MODEL_PHYSICS_PARITY ticks=180 bodies=11 max_position_error_m={max_position_error}");
}
#[test]
fn visual_replacement_preserves_full_crash_run_remount_and_cleans_up_skin() {
    use crate::{
        game::{Phase, Race, Rider},
        recovery::{Recovery, RecoveryStage},
    };
    let mut old = crate::game::tests::simulation();
    let mut new = crate::game::tests::simulation();
    authored(&mut new);
    for app in [&mut old, &mut new] {
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let w = app.world_mut();
        let mut q = w.query::<&mut Rider>();
        let mut r = q.single_mut(w).unwrap();
        r.speed = 8.0;
        r.motion_velocity = Vec3::new(3.0, 0.0, 8.0);
        r.health = 0.0;
    }
    let mut visited = [false; 4];
    let mut remounted = false;
    for tick in 0..1800 {
        old.update();
        new.update();
        let state = |app: &mut App| {
            let w = app.world_mut();
            let mut q = w.query::<(&Rider, &Recovery, &Transform)>();
            let (r, c, t) = q.single(w).unwrap();
            (c.stage, r.health, r.speed, *t)
        };
        let a = state(&mut old);
        let b = state(&mut new);
        assert_eq!(a.0, b.0, "recovery timing changed at {tick}");
        assert!(
            (a.1 - b.1).abs() < 0.0001
                && (a.2 - b.2).abs() < 0.0001
                && a.3.translation.distance(b.3.translation) < 0.0001,
            "racer state changed at {tick}"
        );
        match a.0 {
            RecoveryStage::Falling => visited[0] = true,
            RecoveryStage::GettingUp => visited[1] = true,
            RecoveryStage::Running => visited[2] = true,
            RecoveryStage::Mounting => visited[3] = true,
            RecoveryStage::Riding if visited[0] => {
                remounted = true;
                break;
            }
            _ => {}
        }
    }
    assert!(
        remounted && visited.iter().all(|s| *s),
        "full recovery not observed: {visited:?}"
    );
    let w = new.world_mut();
    assert_eq!(
        w.query::<&AuthoredHuman>().iter(w).count(),
        0,
        "detached skin leaked after remount"
    );
}

#[test]
fn visual_intake_rejects_nonfinite_vertices_bad_joint_ids_and_physics_payloads() {
    let size = u32::from_le_bytes(BUNDLE[8..12].try_into().unwrap()) as usize;
    let start = 12 + size;
    let mut corrupt = BUNDLE.to_vec();
    corrupt[start..start + 4].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
    let mut corrupt = BUNDLE.to_vec();
    corrupt[start + 48..start + 50].copy_from_slice(&11_u16.to_le_bytes());
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
    let mut header: serde_json::Value = serde_json::from_slice(&BUNDLE[12..start]).unwrap();
    header["physics_data"] = serde_json::json!(true);
    let json = serde_json::to_vec(&header).unwrap();
    let mut corrupt = b"RDMESH01".to_vec();
    corrupt.extend_from_slice(&(json.len() as u32).to_le_bytes());
    corrupt.extend_from_slice(&json);
    corrupt.extend_from_slice(&BUNDLE[start..]);
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
}
