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
    app.add_plugins(RoadModelPlugin);
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
        assert_eq!(joints.len(), 16);
        for joint in joints {
            assert!(w.get::<SkinJoint>(*joint).is_some());
            assert!((w.get::<Transform>(*joint).unwrap().scale - Vec3::ONE).length() < 0.00001);
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
    for enabled in [false, true] {
        let mut old = crate::game::tests::simulation();
        let mut new = crate::game::tests::simulation();
        authored(&mut new);
        old.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = enabled;
        new.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = enabled;
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
            if enabled {
                fn segments(
                    app: &mut App,
                ) -> std::collections::BTreeMap<usize, (Transform, Velocity)> {
                    app.world_mut()
                        .query::<(&crate::biomechanics::Segment, &Transform, &Velocity)>()
                        .iter(app.world())
                        .map(|(s, t, v)| (s.part, (*t, *v)))
                        .collect()
                }
                let physical_a = segments(&mut old);
                let physical_b = segments(&mut new);
                assert_eq!(physical_a.len(), 16);
                assert_eq!(physical_b.len(), 16);
                for (part, (ta, va)) in physical_a {
                    let (tb, vb) = physical_b[&part];
                    assert!(
                        ta.translation.distance(tb.translation) < 0.0001,
                        "distal position {part} at {tick}"
                    );
                    assert!(
                        ta.rotation.angle_between(tb.rotation) < 0.001,
                        "distal rotation {part} at {tick}"
                    );
                    assert!(
                        va.linvel.distance(vb.linvel) < 0.0001
                            && va.angvel.distance(vb.angvel) < 0.0001,
                        "distal velocity {part} at {tick}"
                    );
                }
            }
            for (a, b) in a.iter().zip(&b) {
                let ta = old.world().get::<Transform>(*a).unwrap();
                let tb = new.world().get::<Transform>(*b).unwrap();
                let va = old.world().get::<Velocity>(*a).unwrap();
                let vb = new.world().get::<Velocity>(*b).unwrap();
                max_position_error =
                    max_position_error.max(ta.translation.distance(tb.translation));
                assert!(
                    ta.translation.distance(tb.translation) < 0.0001,
                    "body trajectory changed at tick {tick}: {ta:?} vs {tb:?}"
                );
                assert!(
                    ta.rotation.angle_between(tb.rotation) < 0.001,
                    "body rotation changed at tick {tick}"
                );
                assert!(
                    va.linvel.distance(vb.linvel) < 0.0001
                        && va.angvel.distance(vb.angvel) < 0.0001,
                    "momentum changed at {tick}"
                );
                assert_eq!(ta.scale, tb.scale);
            }
        }
        println!(
            "MODEL_PHYSICS_PARITY enabled={enabled} ticks=180 source_bodies=11 max_position_error_m={max_position_error}"
        );
    }
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
                // Rebuilding the visual hierarchy changes entity ordering. Allow
                // 2 cm for contact-order/remount projection after gravity-led
                // wreck toppling; stages/velocities remain strict. Isolated
                // eleven-body render parity is tested separately above.
                && a.3.translation.distance(b.3.translation) < 0.02,
            "racer state changed at {tick}: old={a:?} new={b:?}"
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

#[test]
fn visual_chains_keep_anatomical_endpoints_and_hinges_connected_in_every_pose() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    for (seated, frames) in [(false, data.standing), (true, data.seated)] {
        let source = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let legacy = std::array::from_fn(|i| {
            if seated {
                human::seated(i)
            } else {
                Transform::from_translation(human::SPECS[i].0)
            }
        });
        let rest = connected_pose(legacy, source, legacy);
        for i in 0..11 {
            assert!(rest[i].translation.distance(source[i].translation) < 0.0001);
            assert!(rest[i].rotation.angle_between(source[i].rotation) < 0.001);
        }
        for tick in 0..120 {
            let input = std::array::from_fn(|i| {
                Transform::from_translation(legacy[i].translation + Vec3::new(10.0, 2.0, -5.0))
                    .with_rotation(
                        Quat::from_euler(
                            EulerRot::XYZ,
                            (tick as f32 * 0.07 + i as f32).sin() * 3.0,
                            (i as f32 * 0.71).sin(),
                            (tick as f32 * 0.05).cos(),
                        ) * legacy[i].rotation,
                    )
            });
            let pose = connected_pose(input, source, legacy);
            for i in RIG_ORDER {
                let parent = rr_content::riding_model::RENDER_PARENTS[i];
                if parent < 0 {
                    continue;
                }
                let p = parent as usize;
                let expected = pose[p].translation
                    + pose[p].rotation
                        * (source[p].rotation.inverse()
                            * (source[i].translation - source[p].translation));
                assert!(
                    expected.distance(pose[i].translation) < 0.0001,
                    "detached joint {i}"
                );
                assert!(
                    (pose[i].translation.distance(pose[p].translation)
                        - source[i].translation.distance(source[p].translation))
                    .abs()
                        < 0.0001,
                    "stretched limb {i}"
                );
                if matches!(i, 4 | 6 | 8 | 10) {
                    let bend =
                        (pose[p].rotation * Vec3::Y).angle_between(pose[i].rotation * Vec3::Y);
                    assert!(
                        bend <= if i < 7 { 2.351 } else { 2.451 },
                        "impossible hinge {i}: {bend}"
                    );
                }
                assert!(pose[i].translation.is_finite() && pose[i].rotation.is_finite());
            }
        }
    }
}

#[test]
fn distal_skin_weights_are_normalized_and_all_five_joints_are_used() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let mut used = [0usize; 16];
    for mut mesh in rr_content::riding_model::decode(BUNDLE).unwrap() {
        let frames = match mesh.group.as_str() {
            "standing" => data.standing,
            "seated" => data.seated,
            _ => continue,
        };
        let base = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        articulation::weights(&mut mesh, &bind);
        for (joints, weights) in mesh.joints.iter().zip(&mesh.weights) {
            assert!((weights.iter().sum::<f32>() - 1.0).abs() < 1e-5);
            for (&j, &w) in joints.iter().zip(weights) {
                assert!(j < 16 && w.is_finite() && w >= 0.0);
                if w > 0.01 {
                    used[j as usize] += 1;
                }
            }
        }
        let mut physical = [Transform::IDENTITY; 16];
        for frame in physical.iter_mut().skip(11) {
            frame.rotation = Quat::from_rotation_x(0.4);
        }
        let pose = articulation::extend(base, bind, Some(&physical));
        for i in 11..16 {
            assert!(pose[i].rotation.angle_between(bind[i].rotation) > 0.39);
            assert!(pose[i].translation.distance(bind[i].translation) < 1e-5);
        }
    }
    for (i, n) in used.into_iter().enumerate().skip(11) {
        assert!(n > 10, "unused distal joint {i}: {n}");
    }
}

#[test]
fn neutral_distal_skin_joints_remain_attached_and_wrist_axes_follow_forearms() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let lib = crate::animation::Library::default();
    for (seated, frames, id) in [(false, data.standing, 2364), (true, data.seated, 2020)] {
        let base = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        let out = lib.skin_frames(lib.key(id), bind, seated);
        for (i, parent) in [(12, 4), (13, 6)] {
            let rest =
                Transform::from_matrix(bind[parent].to_matrix().inverse() * bind[i].to_matrix());
            let expected = out[parent].mul_transform(rest);
            assert!(
                out[i].rotation.angle_between(expected.rotation) < 0.001,
                "neutral wrist kink {i}"
            );
            assert!(
                out[i].translation.distance(expected.translation) < 1e-5,
                "detached wrist {i}"
            );
        }
    }
}
